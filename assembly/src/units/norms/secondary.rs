//! Duties that follow from other duties: instantiating a template, deriving, and voiding when the basis goes.

use super::*;

impl Norms {
    pub(super) fn instantiate(&self, parent: &Record, t: &Template, party: Option<&ActorId>, basis: Basis) -> Result<CommitmentTerms, SkipReason> {
        let p = parent.terms();
        let who = |e: PartyExpr| -> Result<ActorId, SkipReason> {
            match e {
                PartyExpr::Debtor => Ok(p.debtor.clone()),
                PartyExpr::Creditor => Ok(p.creditor.clone()),
                PartyExpr::Party => party.cloned().ok_or(SkipReason::NoParty),
                PartyExpr::RoleHolder(r) => self.roles.get(&r).cloned().ok_or(SkipReason::NoHolderOfRole(r)),
                PartyExpr::Payer => p.terms.as_ref().map(|x| x.payer.clone()).ok_or(SkipReason::NoTerms),
            }
        };
        let debtor = who(t.debtor)?;
        let creditor = who(t.creditor)?;
        let cons = match t.cons {
            ConsExpr::Remediated => Condition::Holds(Prop::Remediated(parent.commitment.id)),
            ConsExpr::Audited => Condition::Holds(Prop::Audited(parent.commitment.id)),
            ConsExpr::RefundOfPrice => {
                // The terms are the debtor's own statement. What is refunded is what the record
                // shows the payer actually paid the debtor, up to the stated price.
                let x = p.terms.as_ref().ok_or(SkipReason::NoTerms)?;
                let actually = self.paid.get(&(x.payer.clone(), p.debtor.clone(), x.resource.clone())).copied().unwrap_or(0).min(x.price.0);
                if actually == 0 {
                    return Err(SkipReason::NothingWasPaid);
                }
                Condition::Paid { from: debtor.clone(), to: x.payer.clone(), resource: x.resource.clone(), at_least: crate::model::Amount(actually) }
            }
        };
        Ok(CommitmentTerms {
            debtor,
            creditor,
            ant: None,
            cons,
            deadline: self.now.plus(t.within),
            tags: t.tags.clone(),
            parent: None,
            terms: None,
            basis: Some(basis),
        })
    }

    /// Ensure the secondary commitment `kind` of `parent` exists and is not void.
    pub(super) fn derive(
        &mut self,
        out: &mut Vec<Event>,
        parent: CommitmentId,
        kind: BasisKind,
        t: &Template,
        party: Option<&ActorId>,
    ) -> Result<(), LeafError> {
        if let Some(child) = self.derived.get(&parent).and_then(|m| m.get(&kind)).copied() {
            if let Some(r) = self.c.get_mut(&child) {
                if r.op == Some(Op::Void) {
                    r.op = None;
                    self.refresh(out, [child])?;
                }
            }
            return Ok(());
        }
        let Some(rec) = self.c.get(&parent) else { return Ok(()) };
        match self.instantiate(rec, t, party, Basis { of: parent, kind: kind.clone() }) {
            Err(reason) => out.push(Event::NormSkipped { of: parent, kind, reason }),
            Ok(terms) => {
                if let Some(id) = self.create(out, terms, Source::Rule, None)? {
                    let _ = self.derived.entry(parent).or_default().insert(kind, id);
                }
            }
        }
        Ok(())
    }

    pub(super) fn void(&mut self, out: &mut Vec<Event>, parent: CommitmentId, keep: impl Fn(&BasisKind) -> bool) -> Result<(), LeafError> {
        let children: Vec<CommitmentId> = self.derived.get(&parent).into_iter().flatten().filter(|(k, _)| !keep(k)).map(|(_, c)| *c).collect();
        for child in children {
            if let Some(r) = self.c.get_mut(&child) {
                // Its basis is gone: whatever was done about it since (even its repudiation) is moot.
                if matches!(r.op, None | Some(Op::Breach)) {
                    r.op = Some(Op::Void);
                    self.refresh(out, [child])?;
                }
            }
        }
        Ok(())
    }

    pub(super) fn on_verdict(&mut self, out: &mut Vec<Event>, id: CommitmentId, findings: &BTreeSet<(Finding, ActorId)>) -> Result<(), LeafError> {
        let Some(tags) = self.c.get(&id).map(|r| r.terms().tags.clone()) else { return Ok(()) };
        let mut want: BTreeMap<BasisKind, (Template, ActorId)> = BTreeMap::new();
        for (i, t) in self.rules.on_verdict.iter().enumerate() {
            for (finding, party) in findings {
                if t.finding == *finding && tags.contains(&t.tag) {
                    let kind = BasisKind::Verdict { template: ix(i), finding: *finding, party: party.clone() };
                    let _ = want.insert(kind, (t.create.clone(), party.clone()));
                }
            }
        }
        self.void(out, id, |k| matches!(k, BasisKind::Violation { .. }) || want.contains_key(k))?;
        for (kind, (t, party)) in want {
            self.derive(out, id, kind, &t, Some(&party))?;
        }
        Ok(())
    }
}

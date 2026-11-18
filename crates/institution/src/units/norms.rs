//! Directed conditional commitments: the single normative primitive.
//!
//! A commitment's [`Status`] is not stored truth but a function of the current facts
//! ([`Norms::evaluate`]). So when a claim is refuted, `Satisfied` falls back to `Detached` or
//! `Violated` by re-evaluation, and everything that was derived from a violation that no longer
//! stands is voided. Violations and verdicts instantiate secondary commitments from rule
//! templates (contrary-to-duty obligations: remediation, refund, audit).

use crate::model::LeafError;
use crate::model::{
    Act, ActorId, Assessment, Basis, BasisKind, Commitment, CommitmentId, CommitmentTerms, Condition, ConsExpr, Event, Finding, Kind, NormError, NormRules,
    PartyExpr, Prop, Role, SkipReason, Source, Status, Template, TemplateIx, Time, Transfer, Version,
};
use rigor_kernel::{digest_of, Envelope, Leaf, Name};
use std::collections::{BTreeMap, BTreeSet};

pub const NORMS: Name = Name::lit("norms");

/// An explicit operation performed on a commitment.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
enum Op {
    Released,
    Cancelled,
    /// Its basis no longer stands.
    Void,
    /// The debtor repudiated it after it had detached.
    Breach,
}

#[derive(Clone, Debug, Hash, serde::Serialize, serde::Deserialize)]
pub struct Record {
    pub commitment: Commitment,
    op: Option<Op>,
    pub status: Option<Status>,
    pub created_at: Time,
    ant_at: Option<Time>,
    cons_at: Option<Time>,
}

impl Record {
    #[must_use]
    pub const fn terms(&self) -> &CommitmentTerms {
        &self.commitment.terms
    }
}

#[derive(Clone, Debug, Hash, serde::Serialize, serde::Deserialize)]
pub struct Norms {
    now: Time,
    version: Version,
    rules: NormRules,
    c: BTreeMap<CommitmentId, Record>,
    /// Per proposition: its assessment, since when it has been accepted, and when this unit learned of it.
    prop: BTreeMap<Prop, (Assessment, Option<Time>, Time)>,
    by_prop: BTreeMap<Prop, BTreeSet<CommitmentId>>,
    watch: BTreeSet<CommitmentId>,
    due: BTreeMap<Time, BTreeSet<CommitmentId>>,
    roles: BTreeMap<Role, ActorId>,
    derived: BTreeMap<CommitmentId, BTreeMap<BasisKind, CommitmentId>>,
    /// What has actually been transferred, by (from, to, resource).
    paid: BTreeMap<(ActorId, ActorId, crate::model::Resource), u64>,
}

impl Default for Norms {
    fn default() -> Self {
        Self {
            now: Time::default(),
            version: Version::default(),
            rules: NormRules { on_violation: Vec::new(), on_verdict: Vec::new(), maintenance_period: 1 },
            c: BTreeMap::new(),
            prop: BTreeMap::new(),
            by_prop: BTreeMap::new(),
            watch: BTreeSet::new(),
            due: BTreeMap::new(),
            roles: BTreeMap::new(),
            derived: BTreeMap::new(),
            paid: BTreeMap::new(),
        }
    }
}

fn ix(i: usize) -> TemplateIx {
    TemplateIx(u32::try_from(i).unwrap_or(u32::MAX))
}

impl Norms {
    #[must_use]
    pub const fn now(&self) -> Time {
        self.now
    }

    pub fn records(&self) -> impl Iterator<Item = &Record> {
        self.c.values()
    }

    #[must_use]
    pub fn record(&self, id: &CommitmentId) -> Option<&Record> {
        self.c.get(id)
    }

    #[must_use]
    pub fn status(&self, id: &CommitmentId) -> Option<Status> {
        self.c.get(id).and_then(|r| r.status)
    }

    /// Commitments whose antecedent or consequent mentions the proposition.
    #[must_use]
    pub fn depending_on(&self, p: &Prop) -> BTreeSet<CommitmentId> {
        self.by_prop.get(p).cloned().unwrap_or_default()
    }

    /// Commitments created by rule because of this one.
    #[must_use]
    pub fn derived_from(&self, id: &CommitmentId) -> BTreeSet<CommitmentId> {
        self.derived.get(id).map(|m| m.values().copied().collect()).unwrap_or_default()
    }

    fn when(&self, r: &Record, cond: Option<&Condition>, occurred: Option<Time>) -> Option<Time> {
        match cond {
            None => Some(r.created_at),
            Some(Condition::Holds(p)) => match self.prop.get(p) {
                Some((Assessment::Accepted, since, _)) => *since,
                Some(_) | None => None,
            },
            // A maintenance condition is never "brought about": see `evaluate`.
            Some(Condition::Unrefuted(_)) => None,
            Some(Condition::Paid { .. }) => occurred,
        }
    }

    /// The status function. First matching line wins.
    #[must_use]
    pub fn evaluate(&self, r: &Record) -> Status {
        let t = r.terms();
        match r.op {
            Some(Op::Released) => return Status::Released,
            Some(Op::Cancelled) => return Status::Cancelled,
            Some(Op::Void) => return Status::Void,
            Some(Op::Breach) | None => {}
        }
        if self.when(r, Some(&t.cons), r.cons_at).is_some_and(|at| at <= t.deadline) {
            return Status::Satisfied;
        }
        if r.op == Some(Op::Breach) {
            return Status::Violated;
        }
        let late = self.now > t.deadline;
        if let Condition::Unrefuted(p) = &t.cons {
            // Maintenance: violated while the proposition stands refuted, if the refutation came
            // within the period; discharged when the period ends without one.
            let detached = self.when(r, t.ant.as_ref(), r.ant_at).is_some_and(|at| at <= t.deadline);
            let refuted_in_time = matches!(self.prop.get(p), Some((Assessment::Refuted, _, learned)) if *learned <= t.deadline);
            return match (detached, refuted_in_time, late) {
                (false, _, true) => Status::Expired,
                (false, _, false) => Status::Conditional,
                (true, true, _) => Status::Violated,
                (true, false, true) => Status::Satisfied,
                (true, false, false) => Status::Detached,
            };
        }
        match self.when(r, t.ant.as_ref(), r.ant_at) {
            Some(at) if at <= t.deadline => {
                if late {
                    Status::Violated
                } else {
                    Status::Detached
                }
            }
            Some(_) | None => {
                if late {
                    Status::Expired
                } else {
                    Status::Conditional
                }
            }
        }
    }

    fn create(&mut self, out: &mut Vec<Event>, terms: CommitmentTerms, source: Source, by: Option<&ActorId>) -> Result<Option<CommitmentId>, LeafError> {
        let reject = |reason| Event::NormRejected { by: by.cloned(), reason };
        if terms.deadline < self.now {
            out.push(reject(NormError::DeadlineInThePast));
            return Ok(None);
        }
        if matches!(terms.ant, Some(Condition::Unrefuted(_))) {
            out.push(reject(NormError::MaintenanceAsAntecedent));
            return Ok(None);
        }
        if let Some(parent) = terms.parent {
            let Some(p) = self.c.get(&parent) else {
                out.push(reject(NormError::UnknownParent));
                return Ok(None);
            };
            // "In service of" is a relation between two parties: someone else takes on part of
            // the parent's debtor's burden, and owes it to that debtor. A debtor cannot hang a
            // commitment of its own making under its own, or under anyone else's.
            if terms.creditor != p.terms().debtor || terms.debtor == p.terms().debtor {
                out.push(reject(NormError::NotASubcontract));
                return Ok(None);
            }
        }
        let id = CommitmentId(digest_of(&terms)?);
        if self.c.contains_key(&id) {
            out.push(reject(NormError::Duplicate(id)));
            return Ok(None);
        }
        let mut watched = false;
        for cond in terms.ant.iter().chain(std::iter::once(&terms.cons)) {
            match cond {
                Condition::Holds(p) | Condition::Unrefuted(p) => {
                    let _ = self.by_prop.entry(p.clone()).or_default().insert(id);
                }
                Condition::Paid { .. } => watched = true,
            }
        }
        if watched {
            let _ = self.watch.insert(id);
        }
        let _ = self.due.entry(terms.deadline).or_default().insert(id);
        let commitment = Commitment { id, terms, source, rules: self.version };
        out.push(Event::NormCreated(commitment.clone()));
        let _ = self.c.insert(id, Record { commitment, op: None, status: None, created_at: self.now, ant_at: None, cons_at: None });
        self.refresh(out, [id])?;
        Ok(Some(id))
    }

    fn instantiate(&self, parent: &Record, t: &Template, party: Option<&ActorId>, basis: Basis) -> Result<CommitmentTerms, SkipReason> {
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
    fn derive(&mut self, out: &mut Vec<Event>, parent: CommitmentId, kind: BasisKind, t: &Template, party: Option<&ActorId>) -> Result<(), LeafError> {
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

    fn void(&mut self, out: &mut Vec<Event>, parent: CommitmentId, keep: impl Fn(&BasisKind) -> bool) -> Result<(), LeafError> {
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

    fn refresh(&mut self, out: &mut Vec<Event>, ids: impl IntoIterator<Item = CommitmentId>) -> Result<(), LeafError> {
        let ids: BTreeSet<CommitmentId> = ids.into_iter().collect();
        for id in ids {
            let Some(r) = self.c.get(&id) else { continue };
            let (new, old) = (self.evaluate(r), r.status);
            if Some(new) == old {
                continue;
            }
            let (debtor, creditor, tags) = (r.terms().debtor.clone(), r.terms().creditor.clone(), r.terms().tags.clone());
            if let Some(r) = self.c.get_mut(&id) {
                r.status = Some(new);
            }
            out.push(Event::NormStatus { id, was: old, status: new, at: self.now, debtor, creditor });
            if new == Status::Violated {
                let fire: Vec<(usize, Template)> =
                    self.rules.on_violation.iter().enumerate().filter(|(_, t)| tags.contains(&t.tag)).map(|(i, t)| (i, t.create.clone())).collect();
                for (i, t) in fire {
                    self.derive(out, id, BasisKind::Violation { template: ix(i) }, &t, None)?;
                }
            } else if old == Some(Status::Violated) {
                // The violation no longer stands: neither do its consequences.
                self.void(out, id, |_| false)?;
            }
        }
        Ok(())
    }

    fn on_verdict(&mut self, out: &mut Vec<Event>, id: CommitmentId, findings: &BTreeSet<(Finding, ActorId)>) -> Result<(), LeafError> {
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

    fn on_transfer(&mut self, out: &mut Vec<Event>, tr: &Transfer) -> Result<(), LeafError> {
        let now = self.now;
        let paid = self.paid.entry((tr.from.clone(), tr.to.clone(), tr.resource.clone())).or_default();
        *paid = paid.saturating_add(tr.amount.0);
        let hits = |cond: Option<&Condition>| match cond {
            Some(Condition::Paid { from, to, resource, at_least }) => tr.from == *from && tr.to == *to && tr.resource == *resource && tr.amount >= *at_least,
            Some(Condition::Holds(_) | Condition::Unrefuted(_)) | None => false,
        };
        // One payment settles one debt. If it says which (a refund names its commitment), that
        // one; otherwise the oldest it could settle.
        let named = match &tr.memo {
            crate::model::Memo::Refund(c) => Some(*c),
            crate::model::Memo::None | crate::model::Memo::Purpose(_) | crate::model::Memo::Payment(_) => None,
        };
        let mut candidates: Vec<(Time, CommitmentId, bool)> = Vec::new();
        for id in &self.watch {
            let Some(r) = self.c.get(id) else { continue };
            if r.status.is_some_and(Status::absorbing) {
                continue;
            }
            if r.cons_at.is_none() && hits(Some(&r.commitment.terms.cons)) {
                candidates.push((r.created_at, *id, true));
            } else if r.ant_at.is_none() && hits(r.commitment.terms.ant.as_ref()) {
                candidates.push((r.created_at, *id, false));
            }
        }
        candidates.sort();
        let chosen = match named {
            Some(c) => candidates.iter().find(|(_, id, _)| *id == c).copied(),
            None => candidates.first().copied(),
        };
        let Some((_, id, is_cons)) = chosen else { return Ok(()) };
        if let Some(r) = self.c.get_mut(&id) {
            if is_cons {
                r.cons_at = Some(now);
            } else {
                r.ant_at = Some(now);
            }
            let pending = |c: Option<&Condition>, at: Option<Time>| matches!(c, Some(Condition::Paid { .. })) && at.is_none();
            if !pending(r.commitment.terms.ant.as_ref(), r.ant_at) && !pending(Some(&r.commitment.terms.cons), r.cons_at) {
                let _ = self.watch.remove(&id);
            }
        }
        self.refresh(out, [id])
    }

    fn operate(&mut self, out: &mut Vec<Event>, by: &ActorId, act: &Act) -> Result<(), LeafError> {
        let (id, op) = match act {
            Act::Release { id } | Act::Cancel { id } | Act::Assign { id, .. } => (*id, act),
            _ => return Ok(()),
        };
        let reject = |reason| Event::NormRejected { by: Some(by.clone()), reason };
        let Some(r) = self.c.get_mut(&id) else {
            out.push(reject(NormError::NoSuchLiveCommitment));
            return Ok(());
        };
        if r.status.is_some_and(Status::absorbing) {
            out.push(reject(NormError::NoSuchLiveCommitment));
            return Ok(());
        }
        match op {
            Act::Release { .. } if *by == r.commitment.terms.creditor => r.op = Some(Op::Released),
            Act::Cancel { .. } if *by == r.commitment.terms.debtor => {
                r.op = Some(if r.status == Some(Status::Conditional) { Op::Cancelled } else { Op::Breach });
            }
            Act::Assign { to, .. } if *by == r.commitment.terms.creditor => {
                r.commitment.terms.creditor = to.clone();
                out.push(Event::NormAssigned { id, creditor: to.clone(), by: by.clone() });
            }
            _ => out.push(reject(NormError::NotWithinTheActorsPower)),
        }
        self.refresh(out, [id])
    }
}

impl Leaf<Event> for Norms {
    fn name(&self) -> Name {
        NORMS
    }
    fn inputs(&self) -> BTreeSet<Kind> {
        [Kind::Member, Kind::Expelled, Kind::Tick, Kind::RulesChanged, Kind::Ok, Kind::Assessed, Kind::Verdict, Kind::Transferred].into_iter().collect()
    }
    fn outputs(&self) -> BTreeSet<Kind> {
        [Kind::NormCreated, Kind::NormStatus, Kind::NormRejected, Kind::NormAssigned, Kind::NormSkipped].into_iter().collect()
    }
    fn on(&mut self, env: &Envelope<'_, Event>) -> Result<Vec<Event>, LeafError> {
        let mut out = Vec::new();
        match env.event {
            Event::Member(m) => {
                // A member holds at most the role last given to them.
                self.roles.retain(|_, who| *who != m.id);
                if let Some(role) = m.role {
                    let _ = self.roles.insert(role, m.id.clone());
                }
            }
            Event::Expelled { member } => self.roles.retain(|_, who| who != member),
            Event::RulesChanged(rc) => {
                self.rules = rc.rules.norms.clone();
                self.version = rc.version;
            }
            Event::Tick(t) => {
                self.now = self.now.max(*t);
                let later = self.due.split_off(&self.now);
                let past = std::mem::replace(&mut self.due, later);
                self.refresh(&mut out, past.into_values().flatten())?;
            }
            Event::Ok { by, act: Act::Commit(o), .. } => {
                let terms = CommitmentTerms {
                    debtor: by.clone(),
                    creditor: o.creditor.clone(),
                    ant: o.ant.clone(),
                    cons: o.cons.clone(),
                    deadline: o.deadline,
                    tags: o.tags.clone(),
                    parent: o.parent,
                    terms: o.terms.clone(),
                    basis: None,
                };
                let _ = self.create(&mut out, terms, Source::Act, Some(by))?;
            }
            Event::Ok { by, act, .. } => self.operate(&mut out, by, act)?,
            Event::Assessed { prop, assessment, since, .. } => {
                // When this unit learned of the CURRENT assessment: unchanged if only its basis changed.
                let learned = match self.prop.get(prop) {
                    Some((was, _, at)) if was == assessment => *at,
                    Some(_) | None => self.now,
                };
                let _ = self.prop.insert(prop.clone(), (*assessment, *since, learned));
                self.refresh(&mut out, self.depending_on(prop))?;
            }
            Event::Verdict(v) => self.on_verdict(&mut out, v.commitment, &v.findings)?,
            Event::Transferred { transfer, .. } => self.on_transfer(&mut out, transfer)?,
            _ => {}
        }
        Ok(out)
    }
}

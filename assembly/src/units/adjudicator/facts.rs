//! The facts of a case: what the judgment unit tells the rules about one family of commitments.

use super::*;

impl Adjudicator {
    pub(super) fn facts(&self, fam: &BTreeSet<CommitmentId>) -> Vec<RFact> {
        let mut f = Vec::new();
        // Every member's attributes: a rule may speak of any member by kind or role, not only of
        // those the family happens to mention. (Cost is paid only when there is something to
        // judge: see `adjudicate`.)
        for m in self.members.values() {
            let a = || Const::Actor(m.id.clone());
            f.push(fact(Pred::Kind, [a(), Const::Kind(m.kind)]));
            if let Some(p) = &m.principal {
                f.push(fact(Pred::Principal, [a(), Const::Actor(p.clone())]));
            }
            if let Some(r) = m.role {
                f.push(fact(Pred::HasRole, [a(), Const::Role(r)]));
            }
        }
        for id in fam {
            let Some(k) = self.c.get(id) else { continue };
            let c = || Const::Commitment(*id);
            f.push(fact(Pred::Debtor, [c(), Const::Actor(k.debtor.clone())]));
            f.push(fact(Pred::Creditor, [c(), Const::Actor(k.creditor.clone())]));
            if let Some(s) = k.status {
                f.push(fact(Pred::Status, [c(), Const::Status(s)]));
            }
            f.extend(k.tags.iter().map(|t| fact(Pred::HasTag, [c(), Const::Tag(*t)])));
            if let Some(p) = k.parent.filter(|p| self.c.contains_key(p)) {
                f.push(fact(Pred::Parent, [c(), Const::Commitment(p)]));
            }
            if let Some(p) = &k.prop {
                let pc = || Const::Prop(p.clone());
                f.push(fact(Pred::Cons, [c(), pc()]));
                let a = self.prop.get(p).copied().unwrap_or(Assessment::Unsupported);
                f.push(fact(Pred::Assessed, [pc(), Const::Assessment(a)]));
                for x in self.claims.get(p).into_iter().flatten().map(|(x, _)| x) {
                    f.push(fact(Pred::Claimed, [Const::Actor(x.clone()), pc()]));
                }
                for (x, heard) in self.notified.get(p).into_iter().flatten() {
                    // A message counts as notice only from a party answerable for the
                    // proposition: the debtor of a commitment it would discharge. The first
                    // such notice stands; nobody else's word replaces it.
                    let entitled = |n: &&Notice| self.by_prop.get(p).into_iter().flatten().filter_map(|c| self.c.get(c)).any(|k| k.debtor == n.from);
                    let Some(Notice { value: v, told, .. }) = heard.iter().find(entitled) else { continue };
                    // Being told counts against someone only if it came BEFORE they made any
                    // claim in this family: what one learns after speaking does not make the
                    // earlier statement a knowing one.
                    let first_claim = fam
                        .iter()
                        .filter_map(|c| self.c.get(c))
                        .filter_map(|k| k.prop.as_ref())
                        .filter_map(|q| self.claims.get(q).and_then(|m| m.get(x)))
                        .min();
                    // `told` is how many claims they had made when told; a claim's ordinal is its
                    // position among their claims. Told before the claim: told < ordinal.
                    if first_claim.is_none_or(|ordinal| told < ordinal) {
                        f.push(fact(Pred::Notified, [Const::Actor(x.clone()), pc(), Const::Bool(*v)]));
                    }
                }
            }
        }
        f
    }
}

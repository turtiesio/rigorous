//! Payments: one transfer settles one debt.

use super::*;

impl Norms {
    pub(super) fn on_transfer(&mut self, out: &mut Vec<Event>, tr: &Transfer) -> Result<(), LeafError> {
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
}

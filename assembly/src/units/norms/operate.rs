//! What an actor may do to a commitment: release, cancel, assign.

use super::*;

impl Norms {
    pub(super) fn operate(&mut self, out: &mut Vec<Event>, by: &ActorId, act: &Act) -> Result<(), LeafError> {
        let (id, op) = match act {
            Act::Release { id } | Act::Cancel { id } | Act::Assign { id, .. } => (*id, act),
            _ => return Ok(()),
        };
        let reject = |reason| Event::NormRejected { by: Some(by.clone()), reason };
        let Some(r) = self.c.get_mut(&id) else {
            out.push(reject(NormError::NoSuchLiveCommitment));
            return Ok(());
        };
        // Nothing more can be done to a commitment that is closed, and one cannot walk away
        // from what has already been done.
        if r.status.is_some_and(Status::absorbing) || r.op == Some(Op::Breach) || (matches!(op, Act::Cancel { .. }) && r.status == Some(Status::Satisfied)) {
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

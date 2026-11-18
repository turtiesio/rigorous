//! Re-evaluating commitments when the facts they depend on move.

use super::*;

impl Norms {
    pub(super) fn refresh(&mut self, out: &mut Vec<Event>, ids: impl IntoIterator<Item = CommitmentId>) -> Result<(), LeafError> {
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
            let closed = new.absorbing() || self.c.get(&id).is_some_and(|r| r.op == Some(Op::Breach));
            out.push(Event::NormStatus { id, was: old, status: new, at: self.now, debtor, creditor, closed });
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
}

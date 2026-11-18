//! Making a commitment, or refusing it with a reason.

use super::*;

impl Norms {
    pub(super) fn create(
        &mut self,
        out: &mut Vec<Event>,
        terms: CommitmentTerms,
        source: Source,
        by: Option<&ActorId>,
    ) -> Result<Option<CommitmentId>, LeafError> {
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
            // the parent's debtor's burden, at that debtor's request, and owes it to that
            // debtor. A debtor cannot hang a commitment of its own making under its own, and
            // nobody can volunteer one under someone else's.
            if terms.creditor != p.terms().debtor || terms.debtor == p.terms().debtor || !self.asked.contains(&(parent, terms.debtor.clone())) {
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
}

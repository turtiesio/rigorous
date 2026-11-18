//! What can be asked of the obligation unit.

use super::*;

impl Norms {
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
}

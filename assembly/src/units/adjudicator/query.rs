//! What can be asked of the judgment unit.

use super::*;

impl Adjudicator {
    /// The findings currently standing against a commitment.
    #[must_use]
    pub fn findings(&self, id: &CommitmentId) -> Findings {
        self.verdicts.get(id).cloned().unwrap_or_default()
    }

    pub fn verdicts(&self) -> impl Iterator<Item = (&CommitmentId, &Findings)> {
        self.verdicts.iter().filter(|(_, f)| !f.is_empty())
    }

    /// The top of the delegation tree a commitment belongs to.
    #[must_use]
    pub fn root_of(&self, id: CommitmentId) -> CommitmentId {
        let mut cur = id;
        while let Some(p) = self.c.get(&cur).and_then(|k| k.parent).filter(|p| self.c.contains_key(p)) {
            cur = p;
        }
        cur
    }

    /// A delegation tree, as a set.
    #[must_use]
    pub fn family(&self, root: CommitmentId) -> BTreeSet<CommitmentId> {
        let mut out = BTreeSet::new();
        let mut stack = vec![root];
        while let Some(x) = stack.pop() {
            if out.insert(x) {
                stack.extend(self.kids.get(&x).into_iter().flatten().copied());
            }
        }
        out
    }
}

//! What an independent reading of the same history concludes, and what the world concluded.

use crate::commit::*;
use rigor_institution::model::{Assessment, ChangeId, Prop, Status};
use rigor_institution::world::{Error, World};
use std::collections::{BTreeMap, BTreeSet};

/// An independent computation of which changes end up refuted, by a different algorithm from
/// the system's (plain recursion on "is this commit still in effect"), for cross-checking.
///
/// A commit is in effect unless some commit that reverts it is itself in effect. A change is
/// refuted exactly when it is not in effect.
#[must_use]
pub fn oracle(commits: &[Commit]) -> BTreeSet<ChangeId> {
    let mut seen: BTreeSet<&ChangeId> = BTreeSet::new();
    let mut reverted_by: BTreeMap<ChangeId, Vec<&ChangeId>> = BTreeMap::new();
    for c in commits {
        for written in &c.reverts {
            let mut hits = seen.iter().filter(|k| k.0.as_str().starts_with(written.as_str()));
            if let (Some(one), None) = (hits.next(), hits.next()) {
                reverted_by.entry((*one).clone()).or_default().push(&c.id);
            }
        }
        let _ = seen.insert(&c.id);
    }
    // Reverts always come later, so one pass from newest to oldest settles everything.
    let mut in_effect: BTreeMap<&ChangeId, bool> = BTreeMap::new();
    for c in commits.iter().rev() {
        let undone = reverted_by.get(&c.id).is_some_and(|rs| rs.iter().any(|r| in_effect.get(r).copied().unwrap_or(true)));
        let _ = in_effect.insert(&c.id, !undone);
    }
    in_effect.into_iter().filter(|(_, ok)| !ok).map(|(c, _)| c.clone()).collect()
}

/// What the system concluded, in aggregate.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Conclusions {
    pub refuted: BTreeSet<ChangeId>,
    pub by_status: BTreeMap<Status, usize>,
    pub commitments_with_findings: usize,
}

/// Read the system's conclusions out of a world.
///
/// # Errors
/// If the world lacks an institutional unit.
pub fn conclusions(w: &World) -> Result<Conclusions, Error> {
    let mut c = Conclusions::default();
    for (p, s) in w.epi()?.standings() {
        if let (Prop::Sound(id), Assessment::Refuted) = (p, s.assessment) {
            let _ = c.refuted.insert(id.clone());
        }
    }
    for r in w.norms()?.records() {
        if let Some(s) = r.status {
            *c.by_status.entry(s).or_default() += 1;
        }
    }
    c.commitments_with_findings = w.adj()?.verdicts().count();
    Ok(c)
}

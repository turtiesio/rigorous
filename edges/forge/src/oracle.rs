//! What an independent reading of the same events concludes, and what the world concluded.

use crate::happening::*;
use rigor_institution::model::{ActorId, Assessment, ChangeId, Prop};
use rigor_institution::world::{Error, World};
use std::collections::BTreeMap;

/// Independent computation of how each change stands: each reviewer's LATEST review is what
/// counts; a dismissal removes it; the author's own approval counts for nothing.
#[must_use]
pub fn oracle(all: &[Happening]) -> BTreeMap<ChangeId, Assessment> {
    let mut author: BTreeMap<&ChangeId, &ActorId> = BTreeMap::new();
    let mut latest: BTreeMap<&ChangeId, BTreeMap<&ActorId, Verdict>> = BTreeMap::new();
    for h in all {
        match h.what {
            What::Opened => {
                let _ = author.entry(&h.change).or_insert(&h.who);
            }
            What::Reviewed(Verdict::Dismissed) => {
                let _ = latest.entry(&h.change).or_default().remove(&h.who);
            }
            What::Reviewed(v) => {
                let _ = latest.entry(&h.change).or_default().insert(&h.who, v);
            }
            What::Merged => {}
        }
    }
    let mut out = BTreeMap::new();
    for (change, reviews) in latest {
        let for_it = reviews.iter().any(|(who, v)| *v == Verdict::Approved && author.get(change) != Some(who));
        let against = reviews.values().any(|v| *v == Verdict::ChangesRequested);
        let standing = match (for_it, against) {
            (true, false) => Assessment::Accepted,
            (false, true) => Assessment::Refuted,
            (true, true) => Assessment::Contested,
            (false, false) => Assessment::Unsupported,
        };
        let _ = out.insert(change.clone(), standing);
    }
    out
}

/// How the system currently assesses every change that was reviewed.
///
/// # Errors
/// If the world lacks an institutional unit.
pub fn conclusions(w: &World, reviewed: &BTreeMap<ChangeId, Assessment>) -> Result<BTreeMap<ChangeId, Assessment>, Error> {
    let epi = w.epi()?;
    Ok(reviewed.keys().map(|c| (c.clone(), epi.assessment(&Prop::Sound(c.clone())))).collect())
}

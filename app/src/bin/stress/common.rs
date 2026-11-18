//! Shared by the stress runs: timing, the self-tuning ingest, ingesting a whole history.

use rigor::gitlog::{Commit, Ingest};
use rigor::world::{Error, World};
use std::time::Instant;

pub(crate) fn secs(t: Instant) -> f64 {
    t.elapsed().as_secs_f64()
}

/// The period comes from the project's rules, and the project approves what its reflector proposes.
pub(crate) fn self_tuning() -> Ingest {
    let mut i = Ingest::by_the_rules();
    i.approve_reflector = true;
    i
}

pub(crate) fn ingest_all(w: &mut World, commits: &[Commit]) -> Result<Ingest, Error> {
    let mut ingest = self_tuning();
    for c in commits {
        ingest.commit(w, c)?;
    }
    ingest.close(w)?;
    Ok(ingest)
}

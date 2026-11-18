//! The steward: the project's record of itself, and the acts that add to it.

use crate::backlog::*;
use crate::cast::*;
use crate::check::*;
use crate::error::*;
use crate::layout;
use rigor_adapter_host::{files_under, migrate, refound, restore, run_check, save, AdapterError, ArtifactWatcher};
use rigor_institution::model::{Act, Assessment, Commitment, Condition, Event, Method, Prop, Requirement, Status, Tag, Time};
use rigor_institution::world::attest;
use rigor_institution::world::World;
use rigor_waist::LogError;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

mod budget;
mod open;
mod owner;
mod record;
mod run;

/// The current day, as whole days since the Unix epoch.
///
/// # Errors
/// If the system clock is before the epoch.
pub fn today() -> Result<u64, StewardError> {
    Ok(SystemTime::now().duration_since(UNIX_EPOCH).map_err(|_| StewardError::Clock)?.as_secs() / 86_400)
}

/// What happened to one check in a run.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    /// Evidence from an earlier run is still about the current files: nothing was run.
    StillEvidenced,
    RanAndPassed,
    RanAndFailed,
    /// The check could not be run at all (for example the tool is missing): nothing is established.
    CouldNotObserve,
    Skipped,
}

/// A steward bound to a repository and its persisted log.
#[derive(Debug)]
pub struct Steward {
    pub world: World,
    root: PathBuf,
    log: PathBuf,
    watcher: ArtifactWatcher,
    /// Set when a stored log existed but could not be used (and so was set aside).
    pub previous_log_rejected: Option<PathBuf>,
    /// Set when the stored log was in an older schema and was migrated (the old schema number).
    pub migrated_from: Option<u32>,
    /// Set when the stored log was founded under an older constitution and was re-founded.
    pub refounded: bool,
    /// Set when the record on disk does not reach, or does not match, the point last anchored.
    pub anchor_problem: Option<AnchorProblem>,
}

/// The last point of the record the steward wrote down outside the record itself
/// (`.rigor-anchor` at the repository root, meant to be kept under version control). A log is
/// self-consistent however much of its end is cut off; the anchor is what a cut is measured against.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Anchor {
    pub events: u64,
    pub head: rigor_waist::Digest,
}

/// How the record fails to meet its anchor.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnchorProblem {
    /// The record has fewer events than were anchored: its end is missing (or all of it).
    Shorter { anchored: u64, found: u64 },
    /// The record has a different event at the anchored position: it was rewritten.
    Different,
}

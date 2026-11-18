//! Changing the system in a controlled manner.
//!
//! A change is a request before it is a fact. Its life:
//!
//! 1. PROPOSED by anyone (a person, or a model): a title, a reason, and the files it adds.
//!    Its identity is the digest of its content, so what is verified and approved is exactly
//!    what is applied.
//! 2. VERIFIED in a sandbox copy of the repository: the project's own checks are run there for
//!    real, and the result is evidence about "this change is sound".
//! 3. REVIEWED by the reviewing model: an acknowledgement or an objection.
//! 4. APPROVED by the owner, and only by the owner.
//! 5. APPLIED only if it is assessed sound AND that assessment rests on both a passing
//!    verification and the owner's approval.
//!
//! A change may add files, or replace the content of an existing file, inside permitted places
//! only (research notes and generated tests; never the code or proofs the system is trusted
//! for). An edit names the content it replaces: it applies to exactly that content or not at
//! all. Deleting files, and editing outside those places, are not built, and the system says
//! so in its account of itself.

use crate::{Check, Steward, StewardError, CI, OWNER, REVIEWER};
use rigor_adapter_host::AdapterError;
use rigor_institution::model::{About, Act, ActorId, Assessment, Attestation, ChangeId, Content, ContentDigest, Method, Prop};
use rigor_institution::world::attest;
use rigor_mind::{Gateway, Model};
use rigor_waist::{digest_of, Name};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Component, Path, PathBuf};
use std::process::Command;

mod apply;
mod propose;
mod review;
mod store;
mod verify;

/// One file a change adds.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NewFile {
    /// Path relative to the repository root.
    pub path: String,
    pub content: String,
}

/// A replacement of an existing file's content.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Edit {
    /// Path relative to the repository root.
    pub path: String,
    /// Digest of the content the edit was written against. The edit applies only to exactly
    /// that content: if the file has changed since, the request is stale and is refused.
    pub was: ContentDigest,
    pub content: String,
}

/// A requested change.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeRequest {
    pub title: String,
    pub reason: String,
    pub proposed_by: ActorId,
    pub files: Vec<NewFile>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub edits: Vec<Edit>,
}

/// Why a request is not acceptable as a request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Refusal {
    Empty,
    /// The path leaves the repository or is absolute.
    PathEscapes(String),
    /// Changes may only add files under `docs/research/` or generated tests.
    OutsidePermittedPlaces(String),
    /// A new file would replace one that exists (that needs an edit, which names what it replaces).
    WouldOverwrite(String),
    /// An edit names a file that is not there.
    NothingToEdit(String),
    /// The file is no longer the content the edit was written against.
    Stale(String),
    /// The edit would discard most of the file. An edit is a revision, not a replacement:
    /// this is refused whatever anyone thinks of the new content.
    DiscardsMostOfTheFile {
        path: String,
        lines_before: usize,
        lines_kept: usize,
    },
}

/// What still stands between a change and its application.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Missing {
    PassingVerification,
    OwnersApproval,
    /// It is not currently assessed sound (for example an objection stands).
    AssessedSound,
    AlreadyApplied,
    /// The repository is no longer what the change was verified against.
    StillFits,
}

/// How big a change is.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Size {
    pub files_added: usize,
    pub files_edited: usize,
    pub lines_removed: usize,
    pub lines_new: usize,
}

/// Where a change is in its life.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Progress {
    pub id: ChangeId,
    pub title: String,
    /// What it would do, in whole lines: files added, and for edits (removed, new).
    pub size: Size,
    pub proposed_by: ActorId,
    pub assessment: Assessment,
    pub missing: Vec<Missing>,
    pub applied: bool,
}

const MAY_ADD: [&str; 2] = crate::layout::OPEN_TO_PROPOSALS;
const MAY_EDIT: [&str; 2] = crate::layout::OPEN_TO_PROPOSALS;

fn inside(path: &str, places: &[&str]) -> Result<(), Refusal> {
    let p = Path::new(path);
    if p.is_absolute() || p.components().any(|c| !matches!(c, Component::Normal(_))) {
        return Err(Refusal::PathEscapes(path.to_owned()));
    }
    if !places.iter().any(|prefix| path.starts_with(prefix)) {
        return Err(Refusal::OutsidePermittedPlaces(path.to_owned()));
    }
    Ok(())
}

fn permitted(root: &Path, f: &NewFile) -> Result<(), Refusal> {
    inside(&f.path, &MAY_ADD)?;
    if root.join(&f.path).exists() {
        return Err(Refusal::WouldOverwrite(f.path.clone()));
    }
    Ok(())
}

fn editable(root: &Path, e: &Edit) -> Result<(), Refusal> {
    inside(&e.path, &MAY_EDIT)?;
    match std::fs::read(root.join(&e.path)) {
        Err(_) => Err(Refusal::NothingToEdit(e.path.clone())),
        Ok(now) if ContentDigest(rigor_waist::digest_bytes(&now)) != e.was => Err(Refusal::Stale(e.path.clone())),
        Ok(now) => {
            let (before, kept, _) = line_change(&String::from_utf8_lossy(&now), &e.content);
            // Fewer than half the original lines survive: not a revision.
            if kept.saturating_mul(2) < before {
                return Err(Refusal::DiscardsMostOfTheFile { path: e.path.clone(), lines_before: before, lines_kept: kept });
            }
            Ok(())
        }
    }
}

/// How an edit changes a text, by whole lines: (lines before, of those how many survive, lines new).
#[must_use]
pub fn line_change(before: &str, after: &str) -> (usize, usize, usize) {
    let mut pool: BTreeMap<&str, usize> = BTreeMap::new();
    for l in after.lines() {
        *pool.entry(l).or_default() += 1;
    }
    let total_after = after.lines().count();
    let mut kept = 0;
    for l in before.lines() {
        if let Some(n) = pool.get_mut(l).filter(|n| **n > 0) {
            *n -= 1;
            kept += 1;
        }
    }
    (before.lines().count(), kept, total_after.saturating_sub(kept))
}

/// A request's identity: the digest of everything about it.
fn identity(request: &ChangeRequest) -> Result<ChangeId, StewardError> {
    let digest = digest_of(request).map_err(|e| StewardError::Adapter(AdapterError::Snapshot(e)))?;
    Ok(ChangeId(Name::new(&format!("{:016x}", digest.0))?))
}

/// Run the project's own checks in a directory. One result per check that was run.
fn run_checks(root: &Path, dir: &Path) -> Result<Vec<(Check, bool)>, StewardError> {
    let mut out = Vec::new();
    for check in Check::ALL {
        let mut command = check.command(dir);
        let _ = command.env("CARGO_TARGET_DIR", root.join("target").join("sandbox"));
        out.push((check, command.output().map_err(AdapterError::Io)?.status.success()));
    }
    Ok(out)
}

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
//! In this version a change may only ADD files inside permitted places. Editing or deleting
//! existing files is not built, and the system says so in its account of itself.

use crate::{Check, Steward, StewardError, CI, OWNER, REVIEWER};
use rigor_adapter_host::AdapterError;
use rigor_institution::model::{About, Act, ActorId, Assessment, Attestation, ChangeId, Content, ContentDigest, Method, Prop};
use rigor_institution::world::attest;
use rigor_kernel::{digest_of, Name};
use rigor_mind::{Gateway, Model};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Component, Path, PathBuf};
use std::process::Command;

/// One file a change adds.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NewFile {
    /// Path relative to the repository root.
    pub path: String,
    pub content: String,
}

/// A requested change.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeRequest {
    pub title: String,
    pub reason: String,
    pub proposed_by: ActorId,
    pub files: Vec<NewFile>,
}

/// Why a request is not acceptable as a request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Refusal {
    Empty,
    /// The path leaves the repository or is absolute.
    PathEscapes(String),
    /// Changes may only add files under `docs/research/` or generated tests.
    OutsidePermittedPlaces(String),
    /// The file exists: this version only adds files.
    WouldOverwrite(String),
}

/// What still stands between a change and its application.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Missing {
    PassingVerification,
    OwnersApproval,
    /// It is not currently assessed sound (for example an objection stands).
    AssessedSound,
    AlreadyApplied,
}

/// Where a change is in its life.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Progress {
    pub id: ChangeId,
    pub title: String,
    pub proposed_by: ActorId,
    pub assessment: Assessment,
    pub missing: Vec<Missing>,
    pub applied: bool,
}

const PERMITTED: [&str; 2] = ["docs/research/", "crates/rigor/tests/generated_"];

fn permitted(root: &Path, f: &NewFile) -> Result<(), Refusal> {
    let p = Path::new(&f.path);
    if p.is_absolute() || p.components().any(|c| !matches!(c, Component::Normal(_))) {
        return Err(Refusal::PathEscapes(f.path.clone()));
    }
    if !PERMITTED.iter().any(|prefix| f.path.starts_with(prefix)) {
        return Err(Refusal::OutsidePermittedPlaces(f.path.clone()));
    }
    if root.join(p).exists() {
        return Err(Refusal::WouldOverwrite(f.path.clone()));
    }
    Ok(())
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

impl Steward {
    fn changes_dir(&self) -> PathBuf {
        self.home().join("changes")
    }

    /// The stored request with this identity.
    ///
    /// # Errors
    /// If it cannot be read.
    pub fn change(&self, id: &ChangeId) -> Result<ChangeRequest, StewardError> {
        let text = std::fs::read_to_string(self.changes_dir().join(format!("{id}.json"))).map_err(AdapterError::Io)?;
        let request: ChangeRequest =
            serde_json::from_str(&text).map_err(|e| StewardError::Adapter(AdapterError::Snapshot(rigor_kernel::EncodingError(e.to_string()))))?;
        // What was verified and approved is identified by its content. A stored request that
        // no longer matches its identity has been altered since, and is not that request.
        if identity(&request)? != *id {
            return Err(StewardError::ChangeAltered(id.clone()));
        }
        Ok(request)
    }

    /// Record a change request. Returns its identity, or why it is not acceptable.
    ///
    /// # Errors
    /// On I/O failure or a kernel fault.
    pub fn propose(&mut self, request: &ChangeRequest) -> Result<Result<ChangeId, Refusal>, StewardError> {
        if request.files.is_empty() {
            return Ok(Err(Refusal::Empty));
        }
        for f in &request.files {
            if let Err(r) = permitted(self.root(), f) {
                return Ok(Err(r));
            }
        }
        let id = identity(request)?;
        std::fs::create_dir_all(self.changes_dir()).map_err(AdapterError::Io)?;
        let text =
            serde_json::to_string_pretty(request).map_err(|e| StewardError::Adapter(AdapterError::Snapshot(rigor_kernel::EncodingError(e.to_string()))))?;
        std::fs::write(self.changes_dir().join(format!("{id}.json")), text).map_err(AdapterError::Io)?;
        let _ = self.world.act(&request.proposed_by, Act::Claim { prop: Prop::Sound(id.clone()) })?;
        self.persist()?;
        Ok(Ok(id))
    }

    /// Every change request on file.
    ///
    /// # Errors
    /// On I/O failure.
    pub fn change_ids(&self) -> Result<Vec<ChangeId>, StewardError> {
        let mut out = Vec::new();
        if let Ok(dir) = std::fs::read_dir(self.changes_dir()) {
            for entry in dir {
                let path = entry.map_err(AdapterError::Io)?.path();
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()).filter(|_| path.extension().is_some_and(|e| e == "json")) {
                    out.push(ChangeId(Name::new(stem)?));
                }
            }
        }
        out.sort();
        Ok(out)
    }

    /// Try the change in a sandbox copy of the repository and attest what the project's own
    /// checks say there. Returns whether they all passed.
    ///
    /// # Errors
    /// On I/O failure or a kernel fault.
    pub fn verify(&mut self, id: &ChangeId) -> Result<bool, StewardError> {
        let root = self.root().to_path_buf();
        self.verify_with(id, |sandbox| run_checks(&root, sandbox))
    }

    /// As [`Self::verify`], with the checks supplied by the caller (they are given the sandbox
    /// directory). Verification counts only if EVERY one of the project's checks was run:
    /// a check that was not run establishes nothing, so nothing is attested.
    ///
    /// # Errors
    /// On I/O failure or a kernel fault.
    pub fn verify_with(&mut self, id: &ChangeId, checks: impl FnOnce(&Path) -> Result<Vec<(Check, bool)>, StewardError>) -> Result<bool, StewardError> {
        let request = self.change(id)?;
        let sandbox = self.home().join("sandbox");
        let _ = std::fs::remove_dir_all(&sandbox);
        std::fs::create_dir_all(&sandbox).map_err(AdapterError::Io)?;
        for entry in ["Cargo.toml", "Cargo.lock", "rustfmt.toml", "crates", "proofs", "docs"] {
            let from = self.root().join(entry);
            if from.exists() {
                let copied = Command::new("cp").arg("-a").arg(&from).arg(&sandbox).status().map_err(AdapterError::Io)?;
                if !copied.success() {
                    return Err(StewardError::Adapter(AdapterError::Io(std::io::Error::other("could not copy the repository into the sandbox"))));
                }
            }
        }
        for f in &request.files {
            let to = sandbox.join(&f.path);
            if let Some(dir) = to.parent() {
                std::fs::create_dir_all(dir).map_err(AdapterError::Io)?;
            }
            std::fs::write(to, &f.content).map_err(AdapterError::Io)?;
        }
        let results = checks(&sandbox)?;
        let _ = std::fs::remove_dir_all(&sandbox);
        let ran: BTreeSet<Check> = results.iter().map(|(c, _)| *c).collect();
        if Check::ALL.iter().any(|c| !ran.contains(c)) {
            return Ok(false);
        }
        let passed = results.iter().all(|(_, ok)| *ok);
        let summary: Vec<String> = results.iter().map(|(c, ok)| format!("{}={}", c.requirement(), if *ok { "passed" } else { "FAILED" })).collect();
        let prop = Prop::Sound(id.clone());
        let about = if passed { About::Supports(prop) } else { About::Rebuts(prop) };
        let _ = self.world.act(&CI, attest(about, Method::Test, &format!("sandbox verification: {}", summary.join(", "))))?;
        self.persist()?;
        Ok(passed)
    }

    /// Have the reviewing model read the change and acknowledge or object. `None` if the budget
    /// could not cover the consultation.
    ///
    /// # Errors
    /// On a gateway failure or a kernel fault.
    pub fn review_change(&mut self, gateway: &Gateway, id: &ChangeId) -> Result<Option<bool>, StewardError> {
        const BRIEF: &str = "You are a strict independent reviewer of a proposed change to a project. You are shown its title, reason and the full content of every file it adds. Decide whether it should be accepted: it does what it says, is correct as far as you can tell, and would not mislead a reader. Your FIRST line must be exactly ACCEPT or REJECT, followed by at most five short bullet points.";
        if self.budget_left()? < 30_000 {
            return Ok(None);
        }
        let request = self.change(id)?;
        let mut shown = format!("TITLE\n{}\n\nREASON\n{}\n", request.title, request.reason);
        for f in &request.files {
            shown.push_str(&format!("\nFILE {}\n{}\n", f.path, f.content.chars().take(12_000).collect::<String>()));
        }
        let reply = gateway.ask(Model::Gpt6Luna, BRIEF, &shown, 600)?;
        if !self.pay(reply.micro_usd.max(1))? {
            return Ok(None);
        }
        let accepted = reply.text.lines().map(str::trim).find(|l| !l.is_empty()).is_some_and(|l| l.starts_with("ACCEPT"));
        std::fs::write(self.changes_dir().join(format!("{id}.review.md")), &reply.text).map_err(AdapterError::Io)?;
        let prop = Prop::Sound(id.clone());
        let a = Attestation {
            about: if accepted { About::Supports(prop) } else { About::Rebuts(prop) },
            method: Method::Ack,
            derived_from: BTreeSet::new(),
            artifacts: BTreeMap::new(),
            content: Content::Document { title: "review of the change".to_owned(), digest: ContentDigest(rigor_kernel::digest_bytes(reply.text.as_bytes())) },
        };
        let _ = self.world.act(&REVIEWER, Act::Attest(a))?;
        self.persist()?;
        Ok(Some(accepted))
    }

    /// The owner approves or rejects a change. Only ever called on the owner's instruction.
    ///
    /// # Errors
    /// On I/O failure or a kernel fault.
    pub fn owner_rules_on(&mut self, id: &ChangeId, approve: bool) -> Result<(), StewardError> {
        let prop = Prop::Sound(id.clone());
        let about = if approve { About::Supports(prop) } else { About::Rebuts(prop) };
        let _ = self.world.act(&OWNER, attest(about, Method::Audit, "the owner's ruling on this change"))?;
        self.persist()
    }

    /// Where a change stands, and what is still missing before it may be applied.
    ///
    /// # Errors
    /// If the request cannot be read or the world lacks a unit.
    pub fn progress(&self, id: &ChangeId) -> Result<Progress, StewardError> {
        let request = self.change(id)?;
        let epi = self.world.epi()?;
        let sound = Prop::Sound(id.clone());
        let standing = epi.standing(&sound);
        let on_it = epi.evidence_on(&sound);
        let evidence: Vec<_> = epi.evidence().filter(|e| on_it.contains(&e.id)).collect();
        let by = |who: &ActorId, method: Method, supports: bool| {
            evidence.iter().any(|e| e.source == *who && e.method == method && matches!(e.about, About::Supports(_)) == supports)
        };
        let applied = epi.assessment(&Prop::Applied(id.clone())) == Assessment::Accepted;
        let mut missing = Vec::new();
        if applied {
            missing.push(Missing::AlreadyApplied);
        }
        if standing.assessment != Assessment::Accepted {
            missing.push(Missing::AssessedSound);
        }
        // A verification that failed is never outweighed: not by a later pass, not by approval.
        if !by(&CI, Method::Test, true) || by(&CI, Method::Test, false) {
            missing.push(Missing::PassingVerification);
        }
        // Nor is a rejection by the owner outweighed by a later approval.
        if !by(&OWNER, Method::Audit, true) || by(&OWNER, Method::Audit, false) {
            missing.push(Missing::OwnersApproval);
        }
        Ok(Progress { id: id.clone(), title: request.title, proposed_by: request.proposed_by, assessment: standing.assessment, missing, applied })
    }

    /// Apply a change: write its files into the repository. Refused, with what is missing,
    /// unless it is verified, approved by the owner, and assessed sound.
    ///
    /// # Errors
    /// On I/O failure or a kernel fault.
    pub fn apply(&mut self, id: &ChangeId) -> Result<Result<(), Vec<Missing>>, StewardError> {
        let progress = self.progress(id)?;
        if !progress.missing.is_empty() {
            return Ok(Err(progress.missing));
        }
        let request = self.change(id)?;
        for f in &request.files {
            // Checked again at the moment of writing: the repository may have changed since the proposal.
            if permitted(self.root(), f).is_err() {
                return Ok(Err(vec![Missing::AssessedSound]));
            }
        }
        for f in &request.files {
            let to = self.root().join(&f.path);
            if let Some(dir) = to.parent() {
                std::fs::create_dir_all(dir).map_err(AdapterError::Io)?;
            }
            std::fs::write(to, &f.content).map_err(AdapterError::Io)?;
        }
        let _ = self.world.act(&CI, attest(About::Supports(Prop::Applied(id.clone())), Method::Probe, "files written to the repository"))?;
        self.persist()?;
        Ok(Ok(()))
    }
}

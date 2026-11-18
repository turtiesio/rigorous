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

/// Where a change may add a file, and where it may edit one. Code and proofs are outside both:
/// nothing the system proposes can alter what it is trusted for.
const MAY_ADD: [&str; 2] = ["docs/research/", "crates/rigor/tests/generated_"];
const MAY_EDIT: [&str; 2] = ["docs/research/", "crates/rigor/tests/generated_"];

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
        Ok(now) if ContentDigest(rigor_kernel::digest_bytes(&now)) != e.was => Err(Refusal::Stale(e.path.clone())),
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
        if request.files.is_empty() && request.edits.is_empty() {
            return Ok(Err(Refusal::Empty));
        }
        for f in &request.files {
            if let Err(r) = permitted(self.root(), f) {
                return Ok(Err(r));
            }
        }
        for e in &request.edits {
            if let Err(r) = editable(self.root(), e) {
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
        for (path, content) in request.files.iter().map(|f| (&f.path, &f.content)).chain(request.edits.iter().map(|e| (&e.path, &e.content))) {
            let to = sandbox.join(path);
            if let Some(dir) = to.parent() {
                std::fs::create_dir_all(dir).map_err(AdapterError::Io)?;
            }
            std::fs::write(to, content).map_err(AdapterError::Io)?;
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

    /// Have the drafting model revise an existing file to an instruction. Returns the change
    /// request (not yet proposed), or `None` if the budget could not cover the consultation.
    ///
    /// # Errors
    /// On a gateway failure, I/O failure or a kernel fault.
    pub fn draft_edit(&mut self, gateway: &Gateway, path: &str, instruction: &str) -> Result<Option<ChangeRequest>, StewardError> {
        const BRIEF: &str = "You revise a document exactly as instructed and change nothing else. Reply with the COMPLETE revised document from its first line to its last, including every part you did not change. Never reply with only the changed part. No preface, no commentary, no code fences.";
        let before = std::fs::read(self.root().join(path)).map_err(AdapterError::Io)?;
        let text = String::from_utf8_lossy(&before).into_owned();
        let Some(revised) = self.consult(gateway, Model::Glm53Flash, BRIEF, &format!("INSTRUCTION\n{instruction}\n\nDOCUMENT\n{text}"))? else {
            return Ok(None);
        };
        let mut content = revised.trim().to_owned();
        content.push('\n');
        Ok(Some(ChangeRequest {
            title: format!("Revise {path}"),
            reason: instruction.to_owned(),
            proposed_by: crate::RESEARCHER,
            files: Vec::new(),
            edits: vec![Edit { path: path.to_owned(), was: ContentDigest(rigor_kernel::digest_bytes(&before)), content }],
        }))
    }

    /// Have the reviewing model read the change and acknowledge or object. `None` if the budget
    /// could not cover the consultation.
    ///
    /// # Errors
    /// On a gateway failure or a kernel fault.
    pub fn review_change(&mut self, gateway: &Gateway, id: &ChangeId) -> Result<Option<crate::research::Verdict>, StewardError> {
        const BRIEF: &str = "You are a strict independent reviewer of a proposed change to a project. You are shown its title, reason and the full content of every file it adds. Decide whether it should be accepted: it does what it says, is correct as far as you can tell, and would not mislead a reader. Your FIRST line must be exactly ACCEPT or REJECT, followed by at most five short bullet points.";
        if self.budget_left()? < 30_000 {
            return Ok(None);
        }
        let request = self.change(id)?;
        let mut shown = format!("TITLE\n{}\n\nREASON\n{}\n", request.title, request.reason);
        for f in &request.files {
            shown.push_str(&format!("\nNEW FILE {}\n{}\n", f.path, f.content.chars().take(12_000).collect::<String>()));
        }
        for e in &request.edits {
            let before = std::fs::read_to_string(self.root().join(&e.path)).unwrap_or_default();
            shown.push_str(&format!(
                "\nEDITED FILE {}\n--- BEFORE ---\n{}\n--- AFTER ---\n{}\n",
                e.path,
                before.chars().take(8_000).collect::<String>(),
                e.content.chars().take(8_000).collect::<String>()
            ));
        }
        let reply = gateway.ask(Model::Gpt6Luna, BRIEF, &shown, 3000)?;
        if !self.pay(reply.micro_usd.max(1))? {
            return Ok(None);
        }
        std::fs::write(self.changes_dir().join(format!("{id}.review.md")), &reply.text).map_err(AdapterError::Io)?;
        let prop = Prop::Sound(id.clone());
        // A new review supersedes the reviewer's earlier one: the earlier evidence is withdrawn.
        let on_it = self.world.epi()?.evidence_on(&prop);
        let earlier: Vec<_> = self.world.epi()?.evidence().filter(|e| on_it.contains(&e.id) && e.source == REVIEWER).map(|e| e.id).collect();
        for e in earlier {
            let _ = self.world.act(&REVIEWER, Act::Retract { evidence: e })?;
        }
        let said = crate::research::verdict(&reply.text);
        let accepted = match said {
            crate::research::Verdict::Accept => true,
            crate::research::Verdict::Reject => false,
            crate::research::Verdict::Unclear => {
                self.persist()?;
                return Ok(Some(said));
            }
        };
        let a = Attestation {
            about: if accepted { About::Supports(prop) } else { About::Rebuts(prop) },
            method: Method::Ack,
            derived_from: BTreeSet::new(),
            artifacts: BTreeMap::new(),
            content: Content::Document { title: "review of the change".to_owned(), digest: ContentDigest(rigor_kernel::digest_bytes(reply.text.as_bytes())) },
        };
        let _ = self.world.act(&REVIEWER, Act::Attest(a))?;
        self.persist()?;
        Ok(Some(said))
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
        let evidence: Vec<_> = epi.standing_evidence().filter(|e| on_it.contains(&e.id)).collect();
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
        let mut size = Size { files_added: request.files.len(), files_edited: request.edits.len(), ..Size::default() };
        size.lines_new = request.files.iter().map(|f| f.content.lines().count()).sum();
        for e in &request.edits {
            let now = std::fs::read_to_string(self.root().join(&e.path)).unwrap_or_default();
            let (before, kept, new) = line_change(&now, &e.content);
            size.lines_removed += before - kept;
            size.lines_new += new;
        }
        Ok(Progress { id: id.clone(), title: request.title, size, proposed_by: request.proposed_by, assessment: standing.assessment, missing, applied })
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
        // Checked again at the moment of writing: the repository may have changed since the
        // proposal. A change verified against content that is no longer there is not applied.
        if request.files.iter().any(|f| permitted(self.root(), f).is_err()) || request.edits.iter().any(|e| editable(self.root(), e).is_err()) {
            return Ok(Err(vec![Missing::StillFits]));
        }
        for (path, content) in request.files.iter().map(|f| (&f.path, &f.content)).chain(request.edits.iter().map(|e| (&e.path, &e.content))) {
            let to = self.root().join(path);
            if let Some(dir) = to.parent() {
                std::fs::create_dir_all(dir).map_err(AdapterError::Io)?;
            }
            std::fs::write(to, content).map_err(AdapterError::Io)?;
        }
        let _ = self.world.act(&CI, attest(About::Supports(Prop::Applied(id.clone())), Method::Probe, "files written to the repository"))?;
        self.persist()?;
        Ok(Ok(()))
    }
}

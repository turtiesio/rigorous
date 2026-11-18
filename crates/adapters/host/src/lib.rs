//! Adapters: where the formal system touches real software.
//!
//! An adapter turns something that happened outside (a file changed, a command exited 0) into
//! an event ABOUT AN OBSERVATION. It never asserts the fact itself: "this command exited 0
//! against these artifact digests" is evidence bound to those digests, and it goes stale the
//! moment an artifact's digest differs.

use rigor_institution::model::{About, Act, ActorId, ArtifactId, Attestation, Content, ContentDigest, Event, Method, Prop};
use rigor_institution::world::{Error, World};
use rigor_kernel::{digest_bytes, read_raw, Log, LogError, Name, Runtime, Snapshot, Unit};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::process::Command;

pub const FILESYSTEM: Name = Name::lit("fs");

/// Why an adapter could not observe.
#[derive(Debug)]
pub enum AdapterError {
    Io(std::io::Error),
    World(Error),
    Log(LogError),
    /// The regenerated log differs from the stored one.
    ReplayDiverged,
    UnknownArtifact(ArtifactId),
    /// A stimulus in an old log has no form in the current vocabulary.
    Unmigratable {
        index: usize,
    },
    Snapshot(rigor_kernel::EncodingError),
    /// The world no longer holds the history a snapshot audit must replay.
    NotAuditable,
    /// Nobody trusted has shown this snapshot to be the state its log produces.
    SnapshotNotAttested,
}

impl From<Error> for AdapterError {
    fn from(e: Error) -> Self {
        Self::World(e)
    }
}

/// Observes real files; reports digest changes as `ArtifactChanged` events.
/// An artifact is a set of files: its digest covers every file's path and content.
#[derive(Debug, Default)]
pub struct ArtifactWatcher {
    paths: BTreeMap<ArtifactId, Vec<PathBuf>>,
    seen: BTreeMap<ArtifactId, ContentDigest>,
}

/// Every file under `dir` (recursively) whose extension is one of `extensions`, sorted.
///
/// # Errors
/// If a directory cannot be read.
pub fn files_under(dir: &Path, extensions: &[&str]) -> Result<Vec<PathBuf>, AdapterError> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        for entry in std::fs::read_dir(&d).map_err(AdapterError::Io)? {
            let path = entry.map_err(AdapterError::Io)?.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().and_then(|e| e.to_str()).is_some_and(|e| extensions.contains(&e)) {
                out.push(path);
            }
        }
    }
    out.sort();
    Ok(out)
}

impl ArtifactWatcher {
    /// One file per artifact.
    #[must_use]
    pub fn new(paths: impl IntoIterator<Item = (ArtifactId, PathBuf)>) -> Self {
        Self::of_sets(paths.into_iter().map(|(a, p)| (a, vec![p])))
    }

    /// A set of files per artifact.
    #[must_use]
    pub fn of_sets(sets: impl IntoIterator<Item = (ArtifactId, Vec<PathBuf>)>) -> Self {
        Self { paths: sets.into_iter().collect(), seen: BTreeMap::new() }
    }

    /// The digest last observed for an artifact.
    #[must_use]
    pub fn seen(&self, a: &ArtifactId) -> Option<ContentDigest> {
        self.seen.get(a).copied()
    }

    /// Digest every watched file; inject an event for each one that changed.
    ///
    /// # Errors
    /// If a file cannot be read, or on a kernel fault.
    pub fn scan(&mut self, w: &mut World) -> Result<Vec<ArtifactId>, AdapterError> {
        let mut changed = Vec::new();
        for (artifact, paths) in &self.paths {
            let mut bytes = Vec::new();
            for path in paths {
                bytes.extend(path.to_string_lossy().as_bytes());
                bytes.push(0);
                bytes.extend(std::fs::read(path).map_err(AdapterError::Io)?);
                bytes.push(0);
            }
            let digest = ContentDigest(digest_bytes(&bytes));
            if self.seen.get(artifact) != Some(&digest) {
                let _ = self.seen.insert(artifact.clone(), digest);
                let _ = w.inject(&FILESYSTEM, Event::ArtifactChanged { artifact: artifact.clone(), digest })?;
                changed.push(artifact.clone());
            }
        }
        Ok(changed)
    }
}

/// Run a real command and attest its outcome as evidence bound to the given artifacts.
/// Exit status 0 supports the proposition; anything else rebuts it.
///
/// # Errors
/// If the command cannot be run, an artifact is not watched, or on a kernel fault.
pub fn run_check(
    w: &mut World,
    watcher: &mut ArtifactWatcher,
    command: &mut Command,
    prop: &Prop,
    artifacts: &[ArtifactId],
    by: &ActorId,
) -> Result<bool, AdapterError> {
    let _ = watcher.scan(w)?;
    let output = command.output().map_err(AdapterError::Io)?;
    let passed = output.status.success();
    let mut bound = BTreeMap::new();
    for a in artifacts {
        let d = watcher.seen.get(a).ok_or_else(|| AdapterError::UnknownArtifact(a.clone()))?;
        let _ = bound.insert(a.clone(), *d);
    }
    let mut bytes = output.stdout;
    bytes.extend(output.stderr);
    let attestation = Attestation {
        about: if passed { About::Supports(prop.clone()) } else { About::Rebuts(prop.clone()) },
        method: Method::Test,
        derived_from: BTreeSet::new(),
        artifacts: bound,
        content: Content::Command {
            program: command.get_program().to_string_lossy().into_owned(),
            exit: output.status.code().unwrap_or(-1),
            output: ContentDigest(digest_bytes(&bytes)),
        },
    };
    let _ = w.act(by, Act::Attest(attestation))?;
    Ok(passed)
}

/// Persist the log, one JSON object per line.
///
/// # Errors
/// On I/O failure.
pub fn save(w: &World, path: &Path) -> Result<(), AdapterError> {
    let f = File::create(path).map_err(AdapterError::Io)?;
    w.rt.log().save(BufWriter::new(f)).map_err(AdapterError::Log)
}

/// Rebuild a world from a persisted log by re-running its external stimuli, and insist that
/// the regenerated log is identical (tamper check and determinism check in one).
///
/// # Errors
/// If the stored chain is broken, or replay does not reproduce it.
pub fn restore(root: Box<dyn Unit<Event>>, path: &Path) -> Result<World, AdapterError> {
    let f = File::open(path).map_err(AdapterError::Io)?;
    let log: Log<Event> = Log::load(BufReader::new(f)).map_err(AdapterError::Log)?;
    let stimuli: Vec<_> = log.entries().iter().filter(|e| e.cause.is_none() && e.event != Event::Settle).map(|e| (e.actor.clone(), e.event.clone())).collect();
    let rt = Runtime::replay(root, &stimuli).map_err(|f| AdapterError::World(Error::Fault(f)))?;
    if rt.log().head() != log.head() {
        return Err(AdapterError::ReplayDiverged);
    }
    Ok(World { rt, scope: Vec::new() })
}

pub const MIGRATOR: Name = Name::lit("migrator");

/// Carry a log written under an older schema into the current one.
///
/// The old file's hash chain is verified from its bytes, without interpreting its events. Its
/// external stimuli are brought up to the current vocabulary and replayed into a fresh world
/// whose first event records the schema and chain head it continues from. Derived events are
/// recomputed by the current units: the new log is what today's rules make of the old inputs.
///
/// # Errors
/// If the old chain is broken, a stimulus cannot be upcast, or on a kernel fault.
pub fn migrate(root: Box<dyn Unit<Event>>, path: &Path) -> Result<World, AdapterError> {
    let f = File::open(path).map_err(AdapterError::Io)?;
    let raw = read_raw(BufReader::new(f)).map_err(AdapterError::Log)?;
    let mut stimuli = Vec::new();
    for (index, e) in raw.entries.iter().enumerate().filter(|(_, e)| e.external) {
        let event: Event =
            serde_json::from_value(rigor_institution::model::upcast(raw.schema, e.event.clone())).map_err(|_| AdapterError::Unmigratable { index })?;
        if event != Event::Settle {
            stimuli.push((e.actor.clone(), event));
        }
    }
    let mut w = World::new(root);
    let count = u64::try_from(stimuli.len()).unwrap_or(u64::MAX);
    let _ = w.inject(&MIGRATOR, Event::Migrated { from_schema: raw.schema, old_head: raw.head, stimuli: count })?;
    for (actor, event) in stimuli {
        let _ = w.rt.inject_as(actor, event).map_err(Error::Fault)?;
    }
    Ok(w)
}

/// Write a snapshot of the world's state to a file.
///
/// # Errors
/// On I/O or encoding failure.
pub fn save_snapshot(w: &World, path: &Path) -> Result<Snapshot, AdapterError> {
    let snap = w.rt.snapshot().map_err(AdapterError::Snapshot)?;
    let bytes = bincode::serialize(&snap).map_err(|e| AdapterError::Snapshot(rigor_kernel::EncodingError(e.to_string())))?;
    std::fs::write(path, bytes).map_err(AdapterError::Io)?;
    Ok(snap)
}

/// Resume a world from a snapshot file: cost independent of how long the history was.
///
/// # Errors
/// If the file is unreadable, corrupt, or does not fit the root.
pub fn resume(root: Box<dyn Unit<Event>>, path: &Path) -> Result<World, AdapterError> {
    let bytes = std::fs::read(path).map_err(AdapterError::Io)?;
    let snap: Snapshot = bincode::deserialize(&bytes).map_err(|e| AdapterError::Snapshot(rigor_kernel::EncodingError(e.to_string())))?;
    Ok(World { rt: Runtime::resume(root, &snap).map_err(AdapterError::Snapshot)?, scope: Vec::new() })
}

/// Audit a snapshot: replay the world's own history up to the snapshot's position into a fresh
/// root and compare. The auditor then ATTESTS the result as evidence about the proposition
/// "this snapshot is faithful", which the institution assesses like any other claim.
/// Returns whether it was found faithful.
///
/// # Errors
/// If the world has compacted away the history, or on a kernel fault.
pub fn audit_snapshot(w: &mut World, root: Box<dyn Unit<Event>>, snap: &Snapshot, auditor: &ActorId) -> Result<bool, AdapterError> {
    let entries = w.rt.log().entries();
    if entries.first().is_none_or(|e| e.seq != 0) {
        return Err(AdapterError::NotAuditable);
    }
    let stimuli: Vec<_> =
        entries.iter().filter(|e| e.seq < snap.seq && e.cause.is_none() && e.event != Event::Settle).map(|e| (e.actor.clone(), e.event.clone())).collect();
    let replayed = Runtime::replay(root, &stimuli).map_err(|f| AdapterError::World(Error::Fault(f)))?;
    let faithful = replayed.snapshot().map_err(AdapterError::Snapshot)? == *snap;
    let prop = Prop::Faithful(ContentDigest(snap.digest));
    let attestation = Attestation {
        about: if faithful { About::Supports(prop) } else { About::Rebuts(prop) },
        method: Method::Audit,
        derived_from: BTreeSet::new(),
        artifacts: BTreeMap::new(),
        content: Content::Note("full replay compared with the snapshot".to_owned()),
    };
    let _ = w.act(auditor, Act::Attest(attestation))?;
    Ok(faithful)
}

/// Resume from a snapshot only if the given world holds it to be faithful.
///
/// # Errors
/// `SnapshotNotAttested` unless the proposition is currently assessed accepted there.
pub fn resume_attested(root: Box<dyn Unit<Event>>, path: &Path, evidence: &World) -> Result<World, AdapterError> {
    let bytes = std::fs::read(path).map_err(AdapterError::Io)?;
    let snap: Snapshot = bincode::deserialize(&bytes).map_err(|e| AdapterError::Snapshot(rigor_kernel::EncodingError(e.to_string())))?;
    if evidence.epi()?.assessment(&Prop::Faithful(ContentDigest(snap.digest))) != rigor_institution::model::Assessment::Accepted {
        return Err(AdapterError::SnapshotNotAttested);
    }
    Ok(World { rt: Runtime::resume(root, &snap).map_err(AdapterError::Snapshot)?, scope: Vec::new() })
}

/// Re-found a world under a new constitution, carrying its history over.
///
/// A constitution is the trust anchor and cannot be amended from inside. When it must change
/// (new founding members, new charters) the change is made in the open: the old log's chain is
/// verified, its founding event is replaced by the new constitution, every later stimulus is
/// replayed under it, and the new log begins by recording the chain head it continues from.
///
/// # Errors
/// If the old chain is broken, a stimulus cannot be read, or on a kernel fault.
pub fn refound(root: Box<dyn Unit<Event>>, path: &Path, constitution: &rigor_institution::model::Constitution) -> Result<World, AdapterError> {
    let f = File::open(path).map_err(AdapterError::Io)?;
    let raw = read_raw(BufReader::new(f)).map_err(AdapterError::Log)?;
    let mut stimuli = Vec::new();
    for (index, e) in raw.entries.iter().enumerate().filter(|(_, e)| e.external) {
        let event: Event =
            serde_json::from_value(rigor_institution::model::upcast(raw.schema, e.event.clone())).map_err(|_| AdapterError::Unmigratable { index })?;
        match event {
            Event::Settle | Event::Migrated { .. } => {}
            Event::Genesis(_) => stimuli.push((e.actor.clone(), Event::Genesis(Box::new(constitution.clone())))),
            other => stimuli.push((e.actor.clone(), other)),
        }
    }
    let mut w = World::new(root);
    let count = u64::try_from(stimuli.len()).unwrap_or(u64::MAX);
    let _ = w.inject(&MIGRATOR, Event::Migrated { from_schema: raw.schema, old_head: raw.head, stimuli: count })?;
    for (actor, event) in stimuli {
        let _ = w.rt.inject_as(actor, event).map_err(Error::Fault)?;
    }
    Ok(w)
}

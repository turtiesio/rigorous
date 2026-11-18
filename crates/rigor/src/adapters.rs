//! Adapters: where the formal system touches real software.
//!
//! An adapter turns something that happened outside (a file changed, a command exited 0) into
//! an event ABOUT AN OBSERVATION. It never asserts the fact itself: "this command exited 0
//! against these artifact digests" is evidence bound to those digests, and it goes stale the
//! moment an artifact's digest differs.

use crate::kernel::{digest_bytes, Log, LogError, Name, Runtime, Unit};
use crate::model::{About, Act, ActorId, ArtifactId, Attestation, Content, ContentDigest, Event, Method, Prop};
use crate::world::{Error, World};
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
}

impl From<Error> for AdapterError {
    fn from(e: Error) -> Self {
        Self::World(e)
    }
}

/// Observes real files; reports digest changes as `ArtifactChanged` events.
#[derive(Debug, Default)]
pub struct ArtifactWatcher {
    paths: BTreeMap<ArtifactId, PathBuf>,
    seen: BTreeMap<ArtifactId, ContentDigest>,
}

impl ArtifactWatcher {
    #[must_use]
    pub fn new(paths: impl IntoIterator<Item = (ArtifactId, PathBuf)>) -> Self {
        Self { paths: paths.into_iter().collect(), seen: BTreeMap::new() }
    }

    /// Digest every watched file; inject an event for each one that changed.
    ///
    /// # Errors
    /// If a file cannot be read, or on a kernel fault.
    pub fn scan(&mut self, w: &mut World) -> Result<Vec<ArtifactId>, AdapterError> {
        let mut changed = Vec::new();
        for (artifact, path) in &self.paths {
            let digest = ContentDigest(digest_bytes(&std::fs::read(path).map_err(AdapterError::Io)?));
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
    Ok(World { rt })
}

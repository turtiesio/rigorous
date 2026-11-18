//! Snapshots on disk: saved, resumed, audited by replay, resumed only when attested.

use crate::error::AdapterError;
use rigor_institution::model::{About, Act, ActorId, Attestation, Content, ContentDigest, Event, Method, Prop};
use rigor_institution::world::{Error, World};
use rigor_waist::{Runtime, Snapshot, Unit};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

/// Write a snapshot of the world's state to a file.
///
/// # Errors
/// On I/O or encoding failure.
pub fn save_snapshot(w: &World, path: &Path) -> Result<Snapshot, AdapterError> {
    let snap = w.rt.snapshot().map_err(AdapterError::Snapshot)?;
    let bytes = bincode::serialize(&snap).map_err(|e| AdapterError::Snapshot(rigor_waist::EncodingError(e.to_string())))?;
    std::fs::write(path, bytes).map_err(AdapterError::Io)?;
    Ok(snap)
}

/// Resume a world from a snapshot file: cost independent of how long the history was.
///
/// # Errors
/// If the file is unreadable, corrupt, or does not fit the root.
pub fn resume(root: Box<dyn Unit<Event>>, path: &Path) -> Result<World, AdapterError> {
    let bytes = std::fs::read(path).map_err(AdapterError::Io)?;
    let snap: Snapshot = bincode::deserialize(&bytes).map_err(|e| AdapterError::Snapshot(rigor_waist::EncodingError(e.to_string())))?;
    Ok(World { rt: Runtime::resume(root, &snap).map_err(AdapterError::Snapshot)?, scope: Vec::new() })
}

/// The proposition "this snapshot is faithful". It names the WHOLE snapshot: its position in
/// the log, the chain head there, and its state. Re-pointing an audited state at another
/// position makes it a different snapshot, which nobody has attested.
///
/// # Errors
/// If the snapshot's identity cannot be encoded.
pub fn faithful(snap: &Snapshot) -> Result<Prop, AdapterError> {
    Ok(Prop::Faithful(ContentDigest(rigor_waist::digest_of(&(snap.seq, snap.head, snap.digest)).map_err(AdapterError::Snapshot)?)))
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
    let prop = faithful(snap)?;
    let faithful = replayed.snapshot().map_err(AdapterError::Snapshot)? == *snap;
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
    let snap: Snapshot = bincode::deserialize(&bytes).map_err(|e| AdapterError::Snapshot(rigor_waist::EncodingError(e.to_string())))?;
    if evidence.epi()?.assessment(&faithful(&snap)?) != rigor_institution::model::Assessment::Accepted {
        return Err(AdapterError::SnapshotNotAttested);
    }
    Ok(World { rt: Runtime::resume(root, &snap).map_err(AdapterError::Snapshot)?, scope: Vec::new() })
}

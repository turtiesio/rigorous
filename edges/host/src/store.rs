//! A log on disk: saved, restored by replay, migrated across schemas.

use crate::error::AdapterError;
use rigor_institution::model::Event;
use rigor_institution::world::{Error, World};
use rigor_waist::{read_raw, Log, Runtime, Unit};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

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

pub use rigor_institution::model::MIGRATOR;

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

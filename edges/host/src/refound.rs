//! Re-founding: a new constitution over an old history, done in the open.

use crate::error::AdapterError;
use rigor_institution::model::Event;
use rigor_institution::world::{Error, World};
use rigor_waist::{read_raw, Unit};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::store::MIGRATOR;

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

//! How a run is cut short: the whole state at one point of the log, and continuing from it.

use crate::chain::Log;
use crate::digest::{digest_bytes, Digest, EncodingError};
use crate::payload::Payload;
use crate::runtime::Runtime;
use crate::unit::Unit;
use serde::{Deserialize, Serialize};

/// The whole state of a runtime at one point of its log. Restoring from it and then replaying
/// the later stimuli gives the same runtime as replaying everything (tested), at a cost that
/// does not depend on how long the history before it was.
///
/// A snapshot is TRUSTED input: its digest shows it was not corrupted, not that it is the state
/// the log up to `head` really produces. Only a full replay shows that.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Snapshot {
    /// Number of events recorded when it was taken.
    pub seq: u64,
    /// Head of the log's hash chain at that point.
    pub head: Digest,
    pub digest: Digest,
    pub state: Vec<u8>,
}

impl<E: Payload> Runtime<E> {
    /// Capture the whole state at the current end of the log.
    ///
    /// # Errors
    /// If a unit's state cannot be encoded.
    pub fn snapshot(&self) -> Result<Snapshot, EncodingError> {
        let state = self.root.save()?;
        Ok(Snapshot { seq: self.log.next_seq(), head: self.log.head(), digest: digest_bytes(&state), state })
    }

    /// A runtime that continues from a snapshot, with an empty retained log.
    /// `root` must be built exactly as the root the snapshot was taken from.
    ///
    /// # Errors
    /// If the snapshot is corrupt or does not fit the root.
    pub fn resume(mut root: Box<dyn Unit<E>>, snapshot: &Snapshot) -> Result<Self, EncodingError> {
        if digest_bytes(&snapshot.state) != snapshot.digest {
            return Err(EncodingError("snapshot digest does not match its state".to_owned()));
        }
        root.load(&snapshot.state)?;
        Ok(Self { inputs: root.inputs(), root, log: Log::starting_at(snapshot.seq, snapshot.head), fuel: 1_000_000 })
    }

    /// Forget the retained entries (they must have been persisted, or summarised by a snapshot).
    /// The chain continues; memory no longer grows with history.
    pub fn compact(&mut self) {
        self.log = Log::starting_at(self.log.next_seq(), self.log.head());
    }
}

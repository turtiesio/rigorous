//! The closed-world executor: one FIFO queue over one root unit, one log.

use super::base::{digest_bytes, state_digest, Digest, EncodingError, Name, Path, StateHasher};
use super::log::{Entry, Log};
use super::unit::{Envelope, Fault, FaultOf, Payload, Unit, Who};
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::collections::VecDeque;
use std::hash::Hasher;

#[derive(Debug)]
pub struct Runtime<E: Payload> {
    root: Box<dyn Unit<E>>,
    inputs: std::collections::BTreeSet<E::Kind>,
    log: Log<E>,
    fuel: u64,
}

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

/// An external stimulus: who did what.
pub type Stimulus<E> = (Name, E);

impl<E: Payload> Runtime<E> {
    #[must_use]
    pub fn new(root: Box<dyn Unit<E>>) -> Self {
        Self { inputs: root.inputs(), root, log: Log::default(), fuel: 1_000_000 }
    }

    #[must_use]
    pub fn with_fuel(mut self, fuel: u64) -> Self {
        self.fuel = fuel;
        self
    }

    /// Let the open world speak once, then run to quiescence. Returns the entries appended.
    ///
    /// # Errors
    /// See [`Fault`]. After a fault the runtime must be discarded.
    pub fn inject(&mut self, actor: &Name, event: E) -> Result<Vec<Entry<E>>, FaultOf<E>> {
        self.inject_as(Path::of(actor), event)
    }

    /// As [`Self::inject`], for a stimulus attributed to an actor inside nested boundaries.
    ///
    /// # Errors
    /// See [`Fault`].
    pub fn inject_as(&mut self, actor: Path, event: E) -> Result<Vec<Entry<E>>, FaultOf<E>> {
        let start = self.log.entries().len();
        let settles = self.inputs.contains(&E::settle_kind());
        let kernel = Path::of(&Name::lit("kernel"));
        let mut q: VecDeque<(Path, Option<Digest>, Path, E)> = VecDeque::new();
        q.push_back((actor.clone(), None, actor, event));
        let (mut steps, mut dirty) = (0u64, false);
        while let Some((actor, cause, src, event)) = q.pop_front() {
            steps += 1;
            if steps > self.fuel {
                return Err(Fault::Divergence { fuel: self.fuel });
            }
            let kind = event.kind();
            dirty = dirty || kind != E::settle_kind();
            let entry = self.log.append(actor, cause, src, event)?.clone();
            if self.inputs.contains(&kind) {
                let env = Envelope { who: Who::Here(entry.actor.clone()), entry: &entry, event: &entry.event };
                let mut outs = self.root.step(&env)?;
                // Stable: each leaf's own order is kept; order across leaves is canonical.
                outs.sort_by(|a, b| a.src.cmp(&b.src));
                q.extend(outs.into_iter().map(|o| (o.actor, Some(entry.id), o.src, o.event)));
            }
            if q.is_empty() && dirty && settles {
                // Quiescence: units that judge whole situations do so now, never on a
                // half-propagated state.
                dirty = false;
                q.push_back((kernel.clone(), None, kernel.clone(), E::settle()));
            }
        }
        Ok(self.log.entries().get(start..).map(<[Entry<E>]>::to_vec).unwrap_or_default())
    }

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

    #[must_use]
    pub fn fork(&self) -> Self {
        Self { root: self.root.fork(), inputs: self.inputs.clone(), log: self.log.clone(), fuel: self.fuel }
    }

    /// In-memory digest of the whole state (not of the log).
    #[must_use]
    pub fn digest(&self) -> Digest {
        let mut h = StateHasher::default();
        self.root.hash_state(&mut h);
        state_digest(&h.finish())
    }

    #[must_use]
    pub const fn log(&self) -> &Log<E> {
        &self.log
    }

    pub fn log_mut(&mut self) -> &mut Log<E> {
        &mut self.log
    }

    /// The external stimuli recorded so far (everything needed to replay).
    #[must_use]
    pub fn stimuli(&self) -> Vec<(Path, E)> {
        self.log.entries().iter().filter(|e| e.cause.is_none() && e.event.kind() != E::settle_kind()).map(|e| (e.actor.clone(), e.event.clone())).collect()
    }

    /// Rebuild by re-running stimuli against a fresh root.
    ///
    /// # Errors
    /// See [`Fault`].
    pub fn replay(root: Box<dyn Unit<E>>, stimuli: &[(Path, E)]) -> Result<Self, FaultOf<E>> {
        let mut rt = Self::new(root);
        for (actor, event) in stimuli {
            let _ = rt.inject_as(actor.clone(), event.clone())?;
        }
        Ok(rt)
    }

    /// Typed read access to a leaf's state.
    #[must_use]
    pub fn leaf<T: Any>(&self, path: &[Name]) -> Option<&T> {
        self.root.find(path).and_then(<dyn Any>::downcast_ref::<T>)
    }
}

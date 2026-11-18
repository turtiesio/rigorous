//! The closed-world executor: one FIFO queue over one root unit, one log.

use super::base::{state_digest, Digest, Name, Path, StateHasher};
use super::log::{Entry, Log};
use super::unit::{Envelope, Fault, FaultOf, Payload, Unit, Who};
use std::any::Any;
use std::collections::VecDeque;
use std::hash::Hasher;

#[derive(Debug)]
pub struct Runtime<E: Payload> {
    root: Box<dyn Unit<E>>,
    log: Log<E>,
    fuel: u64,
}

/// An external stimulus: who did what.
pub type Stimulus<E> = (Name, E);

impl<E: Payload> Runtime<E> {
    #[must_use]
    pub fn new(root: Box<dyn Unit<E>>) -> Self {
        Self { root, log: Log::default(), fuel: 1_000_000 }
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
        let start = self.log.len();
        let settles = self.root.inputs().contains(&E::settle_kind());
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
            if self.root.inputs().contains(&kind) {
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

    #[must_use]
    pub fn fork(&self) -> Self {
        Self { root: self.root.fork(), log: self.log.clone(), fuel: self.fuel }
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

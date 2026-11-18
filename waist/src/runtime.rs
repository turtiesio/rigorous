//! The closed-world executor: one FIFO queue over one root unit, one log.
//! A stimulus runs to quiescence; then the units that judge whole situations are told to.

use crate::chain::Log;
use crate::digest::{state_digest, Digest, StateHasher};
use crate::entry::Entry;
use crate::envelope::{Envelope, Who};
use crate::fault::{Fault, FaultOf};
use crate::name::Name;
use crate::path::Path;
use crate::payload::Payload;
use crate::unit::Unit;
use std::any::Any;
use std::collections::VecDeque;
use std::hash::Hasher;

#[derive(Debug)]
pub struct Runtime<E: Payload> {
    pub(crate) root: Box<dyn Unit<E>>,
    pub(crate) inputs: std::collections::BTreeSet<E::Kind>,
    pub(crate) log: Log<E>,
    pub(crate) fuel: u64,
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
        if !event.may_come_from(&actor) {
            return Err(Fault::NotAStimulus(event.kind()));
        }
        self.inject_unchecked(actor, event)
    }

    /// Deliver any event as if it were a stimulus, WITHOUT checking that its kind may come from
    /// outside. For harnesses that exercise one unit in isolation by feeding it what its
    /// neighbours would have produced. Never use it at a boundary with the outside world.
    ///
    /// # Errors
    /// See [`Fault`].
    pub fn inject_unchecked(&mut self, actor: Path, event: E) -> Result<Vec<Entry<E>>, FaultOf<E>> {
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

    /// Typed read access to a leaf's state.
    #[must_use]
    pub fn leaf<T: Any>(&self, path: &[Name]) -> Option<&T> {
        self.root.find(path).and_then(<dyn Any>::downcast_ref::<T>)
    }
}

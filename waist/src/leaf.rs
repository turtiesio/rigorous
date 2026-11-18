//! The primitive unit: a transition function that owns its state.

use crate::digest::EncodingError;
use crate::envelope::{Emit, Envelope};
use crate::fault::{Fault, FaultOf, LeafError};
use crate::name::Name;
use crate::path::Path;
use crate::payload::Payload;
use crate::unit::Unit;
use std::any::Any;
use std::collections::BTreeSet;
use std::fmt::Debug;
use std::hash::Hasher;

/// A primitive unit. `on` is its transition function; it owns its state.
pub trait Leaf<E: Payload>: Clone + std::hash::Hash + Debug + serde::Serialize + serde::de::DeserializeOwned + 'static {
    fn name(&self) -> Name;
    fn inputs(&self) -> BTreeSet<E::Kind>;
    fn outputs(&self) -> BTreeSet<E::Kind>;
    /// # Errors
    /// Only for faults; refusals are ordinary events.
    fn on(&mut self, env: &Envelope<'_, E>) -> Result<Vec<E>, LeafError<E::Failure>>;
}

/// A leaf as a unit.
#[derive(Clone, Debug)]
pub struct Cell<L>(pub L);

impl<E: Payload, L: Leaf<E>> Unit<E> for Cell<L> {
    fn names(&self) -> Vec<Name> {
        vec![self.0.name()]
    }
    fn inputs(&self) -> BTreeSet<E::Kind> {
        self.0.inputs()
    }
    fn outputs(&self) -> BTreeSet<E::Kind> {
        self.0.outputs()
    }
    fn spies(&self) -> Vec<Name> {
        if self.0.inputs().contains(&E::scoped_kind()) {
            vec![self.0.name()]
        } else {
            Vec::new()
        }
    }
    fn forgers(&self) -> Vec<Name> {
        if self.0.outputs().contains(&E::scoped_kind()) {
            vec![self.0.name()]
        } else {
            Vec::new()
        }
    }
    fn step(&mut self, env: &Envelope<'_, E>) -> Result<Vec<Emit<E>>, FaultOf<E>> {
        let name = self.0.name();
        let declared = self.0.outputs();
        let path = Path::of(&name);
        let mut out = Vec::new();
        let events = self.0.on(env).map_err(|e| match e {
            LeafError::Encoding(x) => Fault::Encoding(x),
            LeafError::Broken(failure) => Fault::Unit { unit: name.clone(), failure },
        })?;
        for event in events {
            if !declared.contains(&event.kind()) {
                return Err(Fault::Undeclared { unit: name, kind: event.kind() });
            }
            out.push(Emit { event, actor: path.clone(), src: path.clone() });
        }
        Ok(out)
    }
    fn fork(&self) -> Box<dyn Unit<E>> {
        Box::new(self.clone())
    }
    fn hash_state(&self, mut h: &mut dyn Hasher) {
        std::hash::Hash::hash(&self.0, &mut h);
    }
    fn save(&self) -> Result<Vec<u8>, EncodingError> {
        bincode::serialize(&self.0).map_err(|e| EncodingError(e.to_string()))
    }
    fn load(&mut self, bytes: &[u8]) -> Result<(), EncodingError> {
        self.0 = bincode::deserialize(bytes).map_err(|e| EncodingError(e.to_string()))?;
        Ok(())
    }
    fn find(&self, path: &[Name]) -> Option<&dyn Any> {
        match path {
            [n] if *n == self.0.name() => Some(&self.0),
            _ => None,
        }
    }
}

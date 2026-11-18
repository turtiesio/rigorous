//! What a unit is: a deterministic open transition system, primitive or composite.

use crate::digest::EncodingError;
use crate::envelope::{Emit, Envelope};
use crate::fault::FaultOf;
use crate::name::Name;
use crate::payload::Payload;
use std::any::Any;
use std::collections::BTreeSet;
use std::fmt::Debug;
use std::hash::Hasher;

/// The object-safe view of any unit, primitive or composite.
pub trait Unit<E: Payload>: Debug {
    /// Names of all leaves and entities directly visible at this level.
    fn names(&self) -> Vec<Name>;
    fn inputs(&self) -> BTreeSet<E::Kind>;
    fn outputs(&self) -> BTreeSet<E::Kind>;
    fn is_boundary(&self) -> bool {
        false
    }
    /// Units at this level (through transparent `Par`s) that listen to scoped events without being a boundary.
    fn spies(&self) -> Vec<Name>;
    /// Units at this level that declare they emit scoped events without being a boundary.
    fn forgers(&self) -> Vec<Name>;
    /// # Errors
    /// See [`Fault`].
    fn step(&mut self, env: &Envelope<'_, E>) -> Result<Vec<Emit<E>>, FaultOf<E>>;
    fn fork(&self) -> Box<dyn Unit<E>>;
    fn hash_state(&self, h: &mut dyn Hasher);
    /// The unit's whole state, for a snapshot.
    ///
    /// # Errors
    /// If the state cannot be encoded.
    fn save(&self) -> Result<Vec<u8>, EncodingError>;
    /// Replace the unit's state with one produced by [`Unit::save`] on a unit built the same way.
    ///
    /// # Errors
    /// If the bytes are not such a state.
    fn load(&mut self, bytes: &[u8]) -> Result<(), EncodingError>;
    /// Typed read access to a leaf by local path (for views and tests; never for other units).
    fn find(&self, path: &[Name]) -> Option<&dyn Any>;
}

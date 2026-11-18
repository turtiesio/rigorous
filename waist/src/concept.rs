//! How a unit written in its own words becomes a leaf of any system that can carry them.
//!
//! A concept states what it hears, what it says and how it can break, as its own types. It is
//! a [`Leaf`] of every event type that implements [`Hears`] and [`Speaks`] for those types, so
//! it can be reasoned about, tested and replaced without reading any other concept.

use crate::envelope::{Envelope, Who};
use crate::fault::LeafError;
use crate::leaf::Leaf;
use crate::name::Name;
use crate::vocab::{Hears, Speaks};
use std::collections::BTreeSet;
use std::fmt::Debug;
use std::hash::Hash;

/// What may be kept in a unit's state and used as a key there.
pub trait Key: Clone + Ord + Hash + Debug + serde::Serialize + serde::de::DeserializeOwned + 'static {}
impl<T: Clone + Ord + Hash + Debug + serde::Serialize + serde::de::DeserializeOwned + 'static> Key for T {}

/// A primitive unit stated over its own vocabulary.
pub trait Concept: Clone + Hash + Debug + serde::Serialize + serde::de::DeserializeOwned + 'static {
    type Heard;
    type Said;
    /// The enumerated ways it can be unable to proceed.
    type Broken;
    fn name(&self) -> Name;
    /// # Errors
    /// Only when it cannot proceed; a refusal is something it says.
    fn step(&mut self, from: &Who, heard: Self::Heard) -> Result<Vec<Self::Said>, Self::Broken>;
}

impl<E, C> Leaf<E> for C
where
    C: Concept,
    E: Hears<C::Heard> + Speaks<C::Said>,
    E::Failure: From<C::Broken>,
{
    fn name(&self) -> Name {
        Concept::name(self)
    }
    fn inputs(&self) -> BTreeSet<E::Kind> {
        <E as Hears<C::Heard>>::audible()
    }
    fn outputs(&self) -> BTreeSet<E::Kind> {
        <E as Speaks<C::Said>>::spoken()
    }
    fn on(&mut self, env: &Envelope<'_, E>) -> Result<Vec<E>, LeafError<E::Failure>> {
        let Some(heard) = env.event.hear() else { return Ok(Vec::new()) };
        let said = self.step(&env.who, heard).map_err(|b| LeafError::Broken(b.into()))?;
        Ok(said.into_iter().map(E::say).collect())
    }
}

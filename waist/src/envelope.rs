//! How an event reaches a unit and how one leaves it: with provenance the waist assigned.

use crate::entry::Entry;
use crate::name::Name;
use crate::path::Path;

/// From whose point of view an actor is named.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Who {
    /// An actor inside the current boundary, by its local path.
    Here(Path),
    /// An actor outside the current boundary.
    Outer(Path),
}

impl Who {
    /// The local actor's name, if it is a single local name.
    #[must_use]
    pub fn local(&self) -> Option<&Name> {
        match self {
            Self::Here(p) => p.only(),
            Self::Outer(_) => None,
        }
    }
}

/// An event as delivered to a unit.
#[derive(Debug)]
pub struct Envelope<'a, E> {
    pub who: Who,
    pub entry: &'a Entry<E>,
    pub event: &'a E,
}

/// An event a unit emits, with the provenance the kernel assigned to it.
#[derive(Clone, Debug)]
pub struct Emit<E> {
    pub event: E,
    pub actor: Path,
    pub src: Path,
}

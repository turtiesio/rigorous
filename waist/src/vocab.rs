//! How a unit that knows only its own words lives in a system whose vocabulary is larger.
//!
//! A concept defines what it hears and what it says as its own types. The assembled event type
//! implements these two traits for it: that pair of implementations is the wiring, and it is
//! the only place where one concept's words are turned into another's.

use crate::payload::Payload;
use std::collections::BTreeSet;

/// The event type can carry what a concept says.
pub trait Speaks<Said>: Payload {
    /// The kinds a `Said` can turn into (the concept's output interface).
    fn spoken() -> BTreeSet<Self::Kind>;
    fn say(said: Said) -> Self;
}

/// The event type can be read as what a concept hears.
pub trait Hears<Heard>: Payload {
    /// The kinds that can be heard as a `Heard` (the concept's input interface).
    fn audible() -> BTreeSet<Self::Kind>;
    /// This event in the concept's own words, if it concerns the concept at all.
    fn hear(&self) -> Option<Heard>;
}

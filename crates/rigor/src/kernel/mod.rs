//! The trusted substrate: names, the log, units and their composition, the runtime.
//! It is generic in the event type and knows nothing about norms, evidence or authority.

mod base;
pub mod explore;
mod log;
mod runtime;
mod unit;

pub use base::{digest_bytes, digest_of, state_digest, Digest, EncodingError, Name, NameError, Path, StateHasher};
pub use log::{Entry, Log, LogError};
pub use runtime::{Runtime, Stimulus};
pub use unit::{Cell, Emit, Entity, Envelope, Fault, FaultOf, Leaf, LeafError, Par, Payload, Unit, Who};

//! The waist: the trusted substrate everything else stands on.
//!
//! Names, the hash-chained log, units and their two composition operators, the runtime. It is
//! generic in the event type and knows nothing about norms, evidence, authority or any domain.
//! One file answers one question; the size of this crate is budgeted and checked.

mod boundary;
mod chain;
mod concept;
mod digest;
mod entry;
mod envelope;
mod fault;
mod leaf;
mod name;
mod par;
mod path;
mod payload;
mod raw;
mod replay;
mod runtime;
mod snapshot;
mod stored;
mod unit;
mod vocab;

pub use boundary::Entity;
pub use chain::{BrokenChain, ChainError, Log};
pub use concept::{Concept, Key};
pub use digest::{digest_bytes, digest_of, state_digest, Digest, EncodingError, StateHasher};
pub use entry::Entry;
pub use envelope::{Emit, Envelope, Who};
pub use fault::{Fault, FaultOf, LeafError};
pub use leaf::{Cell, Leaf};
pub use name::{Name, NameError};
pub use par::Par;
pub use path::Path;
pub use payload::Payload;
pub use raw::{read_raw, RawEntry, RawLog};
pub use runtime::{Runtime, Stimulus};
pub use snapshot::Snapshot;
pub use stored::{LogError, SCHEMA};
pub use unit::Unit;
pub use vocab::{Hears, Speaks};

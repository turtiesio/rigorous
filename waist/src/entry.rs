//! What one recorded happening is.

use crate::digest::Digest;
use crate::path::Path;
use serde::{Deserialize, Serialize};

/// One thing that happened.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Entry<E> {
    pub seq: u64,
    pub id: Digest,
    /// Who performed it: stamped by the kernel from the emitter's position, never by the emitter.
    pub actor: Path,
    /// The entry whose processing emitted this one; `None` for an external stimulus.
    pub cause: Option<Digest>,
    /// The emitting leaf (provenance).
    pub src: Path,
    pub event: E,
}

//! How a value is fingerprinted: persisted digests (stable) and in-memory state digests (not).

use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use std::fmt;
use std::hash::{Hash, Hasher};

/// 64 bits of SHA-256.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct Digest(pub u64);

impl fmt::Display for Digest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:010x}", self.0 >> 24)
    }
}

impl fmt::Debug for Digest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

/// A value could not be put in canonical form (never happens for the types in this crate;
/// surfaced as an error rather than assumed away).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EncodingError(pub String);

fn first8(bytes: &[u8]) -> u64 {
    bytes.iter().take(8).fold(0u64, |acc, b| (acc << 8) | u64::from(*b))
}

/// Canonical digest of a serialisable value: SHA-256 of its JSON encoding.
/// Used for everything that is persisted (event ids, object ids), so it is stable across builds.
///
/// # Errors
/// If the value cannot be encoded.
pub fn digest_of<T: Serialize>(value: &T) -> Result<Digest, EncodingError> {
    let bytes = serde_json::to_vec(value).map_err(|e| EncodingError(e.to_string()))?;
    Ok(Digest(first8(&Sha256::digest(&bytes))))
}

/// Digest of raw bytes (file contents, command output).
#[must_use]
pub fn digest_bytes(bytes: &[u8]) -> Digest {
    Digest(first8(&Sha256::digest(bytes)))
}

/// A [`Hasher`] backed by SHA-256, for in-memory state digests only (model checking).
/// Not stable across compiler versions; never persisted.
#[derive(Clone, Default)]
pub struct StateHasher(Sha256);

impl fmt::Debug for StateHasher {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("StateHasher")
    }
}

impl Hasher for StateHasher {
    fn write(&mut self, bytes: &[u8]) {
        self.0.update(bytes);
    }
    fn finish(&self) -> u64 {
        first8(&self.0.clone().finalize())
    }
}

/// In-memory digest of any hashable state.
#[must_use]
pub fn state_digest<T: Hash + ?Sized>(value: &T) -> Digest {
    let mut h = StateHasher::default();
    value.hash(&mut h);
    Digest(h.finish())
}

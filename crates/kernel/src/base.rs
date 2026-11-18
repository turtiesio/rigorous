//! Names, paths and digests: the only identifiers the kernel knows.

use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use std::borrow::Cow;
use std::fmt;
use std::hash::{Hash, Hasher};

/// A validated identifier. It is an opaque label: nothing ever parses it.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Name(Cow<'static, str>);

/// Why a string is not a [`Name`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NameError {
    Empty,
    TooLong,
    BadCharacter,
}

const fn valid(s: &str) -> Result<(), NameError> {
    let b = s.as_bytes();
    if b.is_empty() {
        return Err(NameError::Empty);
    }
    if b.len() > 64 {
        return Err(NameError::TooLong);
    }
    let mut rest = b;
    while let [c, tail @ ..] = rest {
        if !(c.is_ascii_lowercase() || c.is_ascii_digit() || *c == b'-' || *c == b'_') {
            return Err(NameError::BadCharacter);
        }
        rest = tail;
    }
    Ok(())
}

impl Name {
    /// A name from a literal. Use in `const` position: an invalid literal is then a compile error.
    #[must_use]
    pub const fn lit(s: &'static str) -> Self {
        assert!(valid(s).is_ok(), "invalid name literal");
        Self(Cow::Borrowed(s))
    }

    /// A name from run-time data.
    ///
    /// # Errors
    /// If the string is empty, longer than 64 bytes, or not `[a-z0-9_-]+`.
    pub fn new(s: &str) -> Result<Self, NameError> {
        match valid(s) {
            Ok(()) => Ok(Self(Cow::Owned(s.to_owned()))),
            Err(e) => Err(e),
        }
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for Name {
    type Error = String;
    fn try_from(s: String) -> Result<Self, String> {
        Self::new(&s).map_err(|e| format!("{e:?}: {s:?}"))
    }
}

impl From<Name> for String {
    fn from(n: Name) -> Self {
        n.0.into_owned()
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl fmt::Debug for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Where something sits in the nesting of entities, outermost first.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct Path(pub Vec<Name>);

impl Path {
    #[must_use]
    pub fn of(n: &Name) -> Self {
        Self(vec![n.clone()])
    }

    #[must_use]
    pub fn inside(&self, outer: &Name) -> Self {
        let mut v = Vec::with_capacity(self.0.len() + 1);
        v.push(outer.clone());
        v.extend(self.0.iter().cloned());
        Self(v)
    }

    /// The rest of the path if it starts with `outer` and something remains.
    #[must_use]
    pub fn strip(&self, outer: &Name) -> Option<Self> {
        match self.0.split_first() {
            Some((first, rest)) if first == outer && !rest.is_empty() => Some(Self(rest.to_vec())),
            Some(_) | None => None,
        }
    }

    /// The single name, if the path has exactly one segment.
    #[must_use]
    pub fn only(&self) -> Option<&Name> {
        match self.0.as_slice() {
            [n] => Some(n),
            _ => None,
        }
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, n) in self.0.iter().enumerate() {
            if i > 0 {
                f.write_str("/")?;
            }
            f.write_str(n.as_str())?;
        }
        Ok(())
    }
}

impl fmt::Debug for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

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

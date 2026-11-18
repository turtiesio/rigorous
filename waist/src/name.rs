//! What can be an identifier. A name is an opaque label: nothing ever parses it.

use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt;

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

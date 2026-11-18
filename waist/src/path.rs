//! Where something sits in the nesting of boundaries.

use crate::name::Name;
use serde::{Deserialize, Serialize};
use std::fmt;

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

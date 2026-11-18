//! A work counter that is deliberately not part of a unit's state identity.

use std::hash::{Hash, Hasher};

/// Counts work done, for measurements. Ignored by hashing so that it never affects state digests.
#[derive(Clone, Copy, Debug, Default)]
pub struct Meter(pub u64);

impl Meter {
    pub fn add(&mut self, n: usize) {
        self.0 = self.0.saturating_add(u64::try_from(n).unwrap_or(u64::MAX));
    }
}

impl Hash for Meter {
    fn hash<H: Hasher>(&self, _: &mut H) {}
}

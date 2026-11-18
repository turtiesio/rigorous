//! The records. One file per kind of support; `properties` places each property in the six classes.

pub mod assumed;
pub mod compiler;
pub mod measured;
pub mod model_checked;
pub mod negative;
pub mod properties;
pub mod proved;
pub mod tested;
pub mod unresolved;

use crate::claim::Claim;

/// Every claim, in the order the status document presents them.
pub fn all() -> impl Iterator<Item = &'static Claim> {
    [proved::CLAIMS, compiler::CLAIMS, model_checked::CLAIMS, tested::CLAIMS, measured::CLAIMS, assumed::CLAIMS, unresolved::CLAIMS, negative::CLAIMS]
        .into_iter()
        .flatten()
}

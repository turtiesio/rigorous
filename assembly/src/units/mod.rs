//! The institutional units. Each is an ordinary leaf: none is known to the kernel.

pub mod adjudicator;
pub mod envoy;
pub mod epistemic;
pub mod governance;
pub mod institution;
pub mod meter;
pub mod norms;
pub mod reflector;

// Moved to its own crate; reachable here under its old name while the others follow.
pub use crate::wiring::value as ledger;

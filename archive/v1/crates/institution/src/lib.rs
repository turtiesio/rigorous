//! The institution: a closed, typed vocabulary and the units that give it meaning.
//!
//! Depends only on the kernel and the reasoners. It performs no I/O: everything that touches
//! files, processes or the network lives in adapter crates.

pub mod impact;
pub mod model;
pub mod org;
pub mod units;
pub mod world;

//! Adapter: a real package registry (the crates.io index).
//!
//! A third kind of real record, with two things the others lack: evidence that is DERIVED
//! from other evidence (a release rests on the releases it depends on), and RETRACTION (a
//! publisher yanks a release).
//!
//! The mapping:
//!
//! * the publisher undertakes to the registry that each release is fit;
//! * the registry records the release as listed: that record is the evidence of fitness,
//!   derived from the records of the releases its dependencies resolve to. (The publisher's own
//!   word would not count: it is the interested party.)
//! * a yank is the registry withdrawing that record; everything derived from it loses its
//!   support with it.
//!
//! Dependency resolution here is deliberately simple and is an INTERPRETATION: each
//! requirement resolves to the highest known release it admits, yanked or not, so "fit" means
//! "not yanked, and not resting on a yanked release by that resolution". It is not what a
//! package manager would install today.
//!
//! * `cast`  who is who at a package registry, and the constitution that founds it
//! * `error`  why an index could not be read or ingested
//! * `ingest`  turning index records into commitments and evidence: a release rests on what it requires
//! * `oracle`  independent resolution of the same index, and what the world concluded
//! * `published`  index records: releases, their requirements, yanks; parsed from the index

mod cast;
mod error;
mod ingest;
mod oracle;
mod published;

pub use cast::*;
pub use error::*;
pub use ingest::*;
pub use oracle::*;
pub use published::*;

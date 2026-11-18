//! What this project claims, and how each claim is supported.
//!
//! A claim is a typed record, not a sentence in a document: its support is one of a closed set
//! of kinds, and what it cites (a test, a theorem, another claim) is checked to exist. The
//! status document is generated from these records, so it cannot say what they do not.
//!
//! * `claim`     what a claim is and the ways one can be supported
//! * `property`  the six classes a property is placed in
//! * `book`      the records themselves, one file per kind of support
//! * `witness`   does everything a claim cites exist?
//! * `shape`     is the code still shaped the way the design says?
//! * `render`    the status document

pub mod book;
pub mod claim;
pub mod property;
pub mod render;
pub mod shape;
pub mod witness;

/// Where the generated status document lives, relative to the repository root.
pub const LEDGER_DOCUMENT: &str = "docs/03-status-ledger.md";

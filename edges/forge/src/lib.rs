//! Adapter: pull requests and their reviews, from a forge's public event archive.
//!
//! A fourth kind of real record, and the first with real RETRACTION AT SCALE: a reviewer's new
//! review supersedes their earlier one, and a review can be dismissed.
//!
//! The mapping:
//!
//! * opening a pull request: its author undertakes that the change will be found sound, and
//!   claims that it is;
//! * an approving review supports that; a review requesting changes rebuts it;
//! * a reviewer's later review withdraws their earlier one; a dismissal withdraws it too;
//! * a merge is the forge's own record that the change was applied.
//!
//! People and repositories are pseudonymised at the boundary. Only a window of the archive is
//! read, so a review given before the window is invisible: "merged with no approval on record"
//! means on record in the window, nothing more.
//!
//! * `cast`  who is who on a forge, and the constitution that founds it
//! * `error`  why an archive could not be read or ingested
//! * `happening`  a public event of a forge: read from the archive, exported to and imported from a fixture
//! * `ingest`  turning happenings into acts: reviews as evidence for and against, withdrawn when superseded
//! * `oracle`  what an independent reading of the same events concludes, and what the world concluded

mod cast;
mod error;
mod happening;
mod ingest;
mod oracle;

pub use cast::*;
pub use error::*;
pub use happening::*;
pub use ingest::*;
pub use oracle::*;

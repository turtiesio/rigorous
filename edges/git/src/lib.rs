//! Adapter: a real git history as a stream of typed institutional events.
//!
//! This is where untyped real-world text is turned into the closed vocabulary, once, at the
//! boundary. What is read from each commit:
//!
//! * who authored it and who committed it (pseudonymised: an actor id is a digest of the
//!   e-mail address, so no personal data enters the log);
//! * when it was committed (clocks in real histories are not monotone);
//! * whether it declares "This reverts commit <hash>".
//!
//! The mapping into the model:
//!
//! * a newcomer is admitted by whoever first committed their work (their sponsor), acting
//!   under admission authority DELEGATED by the registrar; someone who commits their own first
//!   change is admitted by the registrar itself;
//! * the author undertakes a MAINTENANCE commitment to the project: the change stays
//!   unrefuted for a soak period, counted from when the record arrives (a backdated record
//!   does not get a period that is already over);
//! * a committer other than the author is an independent acknowledgement (supporting evidence);
//! * a revert is rebutting evidence against the reverted change, and undercuts every piece of
//!   evidence the reverted commit had itself produced (so a revert of a revert reinstates).
//!
//! Nothing here judges people. The findings the rules derive are about pseudonymous ids and are
//! only ever reported in aggregate.
//!
//! * `cast`  who is who in a project read from a git history, and the constitution that founds it
//! * `commit`  a commit as the adapter sees it: parsed from git, exported to and imported from a fixture
//! * `error`  why a history could not be read or ingested
//! * `ingest`  turning commits into acts: admissions, changes, acknowledgements, reverts as refutations
//! * `oracle`  what an independent reading of the same history concludes, and what the world concluded
//! * `review`  reviews from a forge, attached to the commits they concern

mod cast;
mod commit;
mod error;
mod ingest;
mod oracle;
mod review;

pub use cast::*;
pub use commit::*;
pub use error::*;
pub use ingest::*;
pub use oracle::*;
pub use review::*;

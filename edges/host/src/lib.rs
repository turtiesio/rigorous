//! Adapters: where the formal system touches real software.
//!
//! An adapter turns something that happened outside (a file changed, a command exited 0) into
//! an event ABOUT AN OBSERVATION. It never asserts the fact itself: "this command exited 0
//! against these artifact digests" is evidence bound to those digests, and it goes stale the
//! moment an artifact's digest differs.

//!
//! * `watch`     real files as artifacts
//! * `check`     a real command as evidence
//! * `store`     a log on disk
//! * `snapshot`  snapshots on disk
//! * `refound`   a new constitution over an old history
//! * `error`     why any of these could not be done

mod check;
mod error;
mod refound;
mod snapshot;
mod store;
mod watch;

pub use check::run_check;
pub use error::AdapterError;
pub use refound::refound;
pub use snapshot::{audit_snapshot, faithful, resume, resume_attested, save_snapshot};
pub use store::{migrate, restore, save, MIGRATOR};
pub use watch::{files_under, ArtifactWatcher, FILESYSTEM};

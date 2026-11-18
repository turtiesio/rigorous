//! Self-stewardship: the system governing the repository it lives in.
//!
//! The project's standing claims about itself ("the tests pass", "the lints are clean", "the
//! formatting is canonical", "the proofs check") are propositions. Each day it is run, the
//! steward has the maintainer undertake that they hold that day; evidence comes only from
//! really running the checks, bound to digests of the real files. So:
//!
//! * evidence from an earlier run still counts while the files it was about are unchanged
//!   (nothing is re-run needlessly);
//! * the moment a file changes, the evidence about it is stale and the claim is unsupported
//!   until the check is run again;
//! * a day on which a claim did not hold stays in the log as a violated commitment, with a
//!   verdict: the project's record of itself cannot be quietly rewritten.
//!
//! The roadmap is in the same world: each open item is a commitment with a review-by day.
//!
//! What this is not: the steward does not write code. It notices, records, and holds to account.
//!
//! * `cast`       who is who, and the founding constitution
//! * `check`      the checks the project runs on itself
//! * `backlog`    the roadmap
//! * `steward`    the record and the acts that add to it
//! * `selfmodel`  what it can and cannot do, and what is lacking
//! * `research`   questions put to models, and how answers are judged
//! * `change`     changing the repository under control
//! * `dashboard`  what the owner sees
//! * `layout`     where things are in the repository
//! * `error`      why something could not be done

mod backlog;
mod cast;
pub mod change;
mod check;
pub mod dashboard;
mod error;
pub mod layout;
pub mod research;
pub mod selfmodel;
mod steward;

pub use backlog::{BacklogItem, BACKLOG};
pub use cast::{constitution, CI, GATEWAY, MAINTAINER, MICRO_USD, OWNER, OWNERSHIP, PROJECT, PROOFS, RESEARCHER, REVIEWER, SOURCES};
pub use check::Check;
pub use error::StewardError;
pub use steward::{today, Anchor, AnchorProblem, Outcome, Steward};

//! rigor: a strictly typed, compositional foundation for modelling and governing systems.
//!
//! Nothing in this crate decides anything by inspecting a string. Event kinds, actions,
//! propositions, statuses, findings, rule paths and templates are closed types; identifiers
//! are opaque newtypes; refusals are enumerated reasons.
//!
//! # What does not compile
//!
//! Each failing example sits next to a twin that differs only in the point being made.
//!
//! An invalid name literal is a compile error, not a run-time surprise:
//! ```
//! use rigor::kernel::Name;
//! const OK: Name = Name::lit("alice");
//! ```
//! ```compile_fail
//! use rigor::kernel::Name;
//! const BAD: Name = Name::lit("Alice Smith");
//! ```
//!
//! Identifiers of different things are not interchangeable:
//! ```
//! use rigor::{kernel::Digest, model::{Act, EvidenceId}};
//! let _ = Act::Retract { evidence: EvidenceId(Digest(1)) };
//! ```
//! ```compile_fail
//! use rigor::{kernel::Digest, model::{Act, CommitmentId}};
//! let _ = Act::Retract { evidence: CommitmentId(Digest(1)) };
//! ```
//!
//! Nobody can commit someone else: an offer has no debtor to fill in (the kernel supplies it):
//! ```
//! use rigor::model::{ActorId, Condition, Offer, Prop, Time};
//! use rigor::kernel::Name;
//! let _ = Offer { creditor: ActorId::lit("alice"), ant: None, cons: Condition::Holds(Prop::Fact(Name::lit("x"))),
//!                 deadline: Time(1), tags: Default::default(), parent: None, terms: None };
//! ```
//! ```compile_fail
//! use rigor::model::{ActorId, Condition, Offer, Prop, Time};
//! use rigor::kernel::Name;
//! let _ = Offer { debtor: ActorId::lit("bob"), creditor: ActorId::lit("alice"), ant: None,
//!                 cons: Condition::Holds(Prop::Fact(Name::lit("x"))),
//!                 deadline: Time(1), tags: Default::default(), parent: None, terms: None };
//! ```
//!
//! An amount is not a time, and neither is a bare number:
//! ```
//! use rigor::model::{Time, Version};
//! assert!(Time(3) < Time(4) && Version(1) < Version(2));
//! ```
//! ```compile_fail
//! use rigor::model::{Amount, Time};
//! assert!(Time(3) < Amount(4));
//! ```

pub mod actors;
pub mod adapters;
pub mod impact;
pub mod kernel;
pub mod logic;
pub mod model;
pub mod org;
pub mod scenario;
pub mod synth;
pub mod units;
pub mod world;

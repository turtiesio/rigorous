//! Typed identifiers and scalar quantities. None of them is interchangeable with another.

use crate::kernel::{Digest, Name};
use serde::{Deserialize, Serialize};
use std::fmt;

macro_rules! named {
    ($(#[$m:meta])* $t:ident) => {
        $(#[$m])*
        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $t(pub Name);
        impl $t {
            #[must_use]
            pub const fn lit(s: &'static str) -> Self { Self(Name::lit(s)) }
        }
        impl fmt::Display for $t {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Display::fmt(&self.0, f) }
        }
    };
}

macro_rules! hashed {
    ($(#[$m:meta])* $t:ident, $prefix:literal) => {
        $(#[$m])*
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $t(pub Digest);
        impl fmt::Display for $t {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, concat!($prefix, "-{}"), self.0) }
        }
        impl fmt::Debug for $t {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Display::fmt(self, f) }
        }
    };
}

named!(/// A member of an institution: human, AI agent, service or organisation alike.
    ActorId);
named!(/// A kind of countable resource.
    Resource);
named!(/// Something outside the system whose content can be digested (a file, a build).
    ArtifactId);
named!(/// A unit of requested work.
    TaskName);
named!(/// A stated requirement.
    Requirement);
named!(/// The identifier of a responsibility rule.
    RuleId);
named!(/// A change to some body of work, named by its content hash (for example a commit).
    ChangeId);
named!(/// A capability created by the constitution.
    CharterName);

hashed!(/// Identity of a commitment: the digest of its terms.
    CommitmentId, "c");
hashed!(/// Identity of an evidence item: the digest of its content.
    EvidenceId, "e");
hashed!(/// Identity of an amendment proposal.
    ProposalId, "p");
hashed!(/// Digest of an artifact's content, or of a rule set.
    ContentDigest, "d");

impl ContentDigest {
    /// A placeholder digest for tests that inject rule changes by hand.
    #[must_use]
    pub const fn default_for_tests() -> Self {
        Self(Digest(0))
    }
}

/// Identity of a capability. Its form says where it came from.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum CapId {
    /// Every member's capability to perform speech acts.
    Base(ActorId),
    /// Every member's ownership of its own account.
    Own(ActorId),
    /// A capability granted by the constitution.
    Charter(CharterName),
    /// A capability created by delegation: the digest of the grant.
    Delegated(Digest),
}

impl fmt::Display for CapId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Base(a) => write!(f, "base({a})"),
            Self::Own(a) => write!(f, "own({a})"),
            Self::Charter(n) => write!(f, "charter({n})"),
            Self::Delegated(d) => write!(f, "cap-{d}"),
        }
    }
}

/// Logical time, supplied by an external clock.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct Time(pub u64);

impl Time {
    #[must_use]
    pub const fn plus(self, d: u64) -> Self {
        Self(self.0.saturating_add(d))
    }
    #[must_use]
    pub const fn minus(self, d: u64) -> Self {
        Self(self.0.saturating_sub(d))
    }
}

/// A non-negative quantity of a resource.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct Amount(pub u64);

/// The version number of a rule set.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct Version(pub u32);

/// Strength of a method of evidence (ordinal only).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Rank(pub u8);

/// Position of a template in its rule list.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TemplateIx(pub u32);

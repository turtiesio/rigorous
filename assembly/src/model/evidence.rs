//! Evidence: how it was obtained, what it bears on, how a proposition is assessed.

#[allow(clippy::wildcard_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

/// How a piece of evidence was obtained.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Method {
    Receipt,
    Ack,
    Test,
    Probe,
    Audit,
    Inference,
    /// A recorded decision to undo something.
    Reversal,
    Hearsay,
}

/// The institution's current view of a proposition. Never "true" or "false".
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Assessment {
    Accepted,
    Refuted,
    Contested,
    Unsupported,
}

/// What a piece of evidence bears on. Exactly one, by construction.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum About {
    Supports(Prop),
    Rebuts(Prop),
    /// Attacks another piece of evidence rather than a proposition.
    Undercuts(EvidenceId),
}

/// Human-readable or machine-produced detail. Opaque: never interpreted.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Content {
    Note(String),
    /// A document held outside the log, identified by the digest of its content.
    Document {
        title: String,
        digest: ContentDigest,
    },
    Command {
        program: String,
        exit: i32,
        output: ContentDigest,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Attestation {
    pub about: About,
    pub method: Method,
    pub derived_from: BTreeSet<EvidenceId>,
    /// The artifact contents this evidence is about; it is stale once any of them differs.
    pub artifacts: BTreeMap<ArtifactId, ContentDigest>,
    pub content: Content,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Evidence {
    pub id: EvidenceId,
    pub source: ActorId,
    pub about: About,
    pub method: Method,
    pub derived_from: BTreeSet<EvidenceId>,
    pub artifacts: BTreeMap<ArtifactId, ContentDigest>,
    pub content: Content,
    pub at: Time,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EvidenceError {
    UnknownTarget,
    UnknownProvenance,
    /// An inference must cite what it is derived from.
    UngroundedInference,
    /// An inference can only support a proposition: it cannot rebut one or undercut evidence.
    InferenceMustSupport,
    /// The conclusion does not follow from what the inference cites.
    DoesNotFollow,
    Duplicate,
    /// Only the source may retract its evidence.
    NotTheSource,
}

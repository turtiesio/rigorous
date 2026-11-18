//! Verdicts: what is found, about whom, and what the finding took on trust.

#[allow(clippy::wildcard_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Finding {
    Accountable,
    Culpable,
    Negligent,
    Excused,
    Contributed,
    Liable,
}

/// Something a verdict's derivation took on trust.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Assumption {
    /// Actors are who the log says they are.
    Authentication,
    /// "Notified" and "claimed" are records of messages, not of what anyone knew.
    RecordsNotKnowledge,
    /// "Violated" rests on the current assessment of evidence, which is revisable.
    AssessmentIsRevisable,
    /// Closed world: the derivation relied on there being no record of this fact.
    NoRecordOf(RFact),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Verdict {
    pub commitment: CommitmentId,
    pub findings: BTreeSet<(Finding, ActorId)>,
    pub rules: Version,
    pub proofs: Vec<RProof>,
    pub assumptions: Vec<Assumption>,
}

//! Changing the rules: what a change is and why a proposal is refused.

#[allow(clippy::wildcard_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RulesChanged {
    pub version: Version,
    pub rules: Rules,
    pub digest: ContentDigest,
    pub prev: Option<ContentDigest>,
    pub changed: BTreeSet<RulePath>,
    /// Who approved it; empty for the constitution.
    pub by: BTreeSet<ActorId>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GovError {
    EmptyProposal,
    Entrenched(RulePath),
    Invalid(RulesError),
    DuplicateProposal,
    NoSuchOpenProposal,
    /// The proposal was made against an older rule version.
    Stale,
}

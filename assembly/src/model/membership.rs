//! Who is in an institution, and the founding act that says so.

#[allow(clippy::wildcard_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum MemberKind {
    Human,
    Ai,
    Service,
    Org,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Role {
    Auditor,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Member {
    pub id: ActorId,
    pub kind: MemberKind,
    pub principal: Option<ActorId>,
    pub role: Option<Role>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Charter {
    pub name: CharterName,
    pub holder: ActorId,
    pub actions: BTreeSet<ActionKind>,
    /// Whether the holder may delegate (attenuated) parts of it.
    pub delegable: bool,
}

/// The genesis payload: the only trust anchor.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Constitution {
    pub members: Vec<Member>,
    pub charters: Vec<Charter>,
    pub balances: Vec<(ActorId, Resource, Amount)>,
    pub rules: Rules,
}

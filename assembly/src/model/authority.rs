//! Capabilities: what they permit, how they are restricted and delegated, why an attempt is refused.

#[allow(clippy::wildcard_imports)]
use super::*;
use rigor_waist::Name;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ActionKind {
    Delegate,
    Revoke,
    Transfer,
    Claim,
    Attest,
    Retract,
    Commit,
    Release,
    Cancel,
    Assign,
    Request,
    Propose,
    Approve,
    Admit,
    Expel,
    SetRole,
}

/// What a capability is restricted to.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct Scope {
    /// If set, transfers may only draw on this account.
    pub from: Option<ActorId>,
    /// If set, the capability is good for one named purpose only: every transfer under it must
    /// carry that purpose as its memo (as a signature binds to one transaction).
    pub purpose: Option<Name>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Grant {
    pub to: ActorId,
    pub actions: BTreeSet<ActionKind>,
    pub scope: Scope,
    pub limits: BTreeMap<Resource, Amount>,
    pub expires: Option<Time>,
    pub delegable: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Issuer {
    Constitution,
    Member(ActorId),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Capability {
    pub id: CapId,
    pub issuer: Issuer,
    pub holder: ActorId,
    pub actions: BTreeSet<ActionKind>,
    pub scope: Scope,
    pub limits: BTreeMap<Resource, Amount>,
    pub expires: Option<Time>,
    pub delegable: bool,
    pub parent: Option<CapId>,
}

/// Why a delegation is not an attenuation of its parent.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttenuationError {
    ActionsExceedParent,
    ScopeWiderThanParent,
    PurposeDiffersFromParent,
    LimitExceedsParent(Resource),
    ExpiryExceedsParent,
}

/// Why an attempt did not count as an act.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DenyReason {
    NotAMember,
    UnknownCap(CapId),
    Revoked(CapId),
    Expired(CapId),
    NotHolder,
    NotDelegable,
    DelegateeNotAMember,
    EmptyDelegation,
    NotAnAttenuation(AttenuationError),
    DuplicateDelegation,
    ActionNotGranted,
    OutOfScope,
    UnknownTarget,
    NotAnIssuerOnTheChain,
    ZeroAmount,
    BudgetExceeded(CapId),
    AlreadyAMember,
    UnknownMember,
}

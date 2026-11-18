//! Commitments: their terms, their status, what they rest on, why one is refused.

#[allow(clippy::wildcard_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Tag {
    Task,
    Service,
    Paid,
    Payment,
    Audit,
    Remediation,
    Refund,
    Conformance,
    /// A change submitted to a body of work.
    Change,
    /// A standing duty of a project towards itself.
    Upkeep,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Status {
    Conditional,
    Detached,
    Satisfied,
    Violated,
    Expired,
    Released,
    Cancelled,
    Void,
}

impl Status {
    /// Statuses that are set by an explicit operation and never left.
    #[must_use]
    pub const fn absorbing(self) -> bool {
        matches!(self, Self::Released | Self::Cancelled | Self::Void)
    }
}

/// A condition a commitment can depend on.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Condition {
    /// Holds while the proposition is assessed accepted (revisable).
    Holds(Prop),
    /// A maintenance condition: it must not come to be refuted while the commitment is in force.
    /// As a consequent it turns the commitment from "bring this about by the deadline" into
    /// "let this not be defeated until the deadline". It cannot be an antecedent.
    Unrefuted(Prop),
    /// Holds once a transfer of at least this much has occurred (irrevocable).
    Paid { from: ActorId, to: ActorId, resource: Resource, at_least: Amount },
}

/// Commercial terms attached to a commitment.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Terms {
    pub price: Amount,
    pub resource: Resource,
    pub payer: ActorId,
}

/// The terms on which an actor binds itself.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Offer {
    pub creditor: ActorId,
    pub ant: Option<Condition>,
    pub cons: Condition,
    pub deadline: Time,
    pub tags: BTreeSet<Tag>,
    pub parent: Option<CommitmentId>,
    pub terms: Option<Terms>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Request {
    pub to: ActorId,
    pub task: Task,
    pub deadline: Time,
    pub parent: Option<CommitmentId>,
    pub terms: Option<Terms>,
}

/// What a rule-created commitment rests on.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum BasisKind {
    Violation { template: TemplateIx },
    Verdict { template: TemplateIx, finding: Finding, party: ActorId },
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Basis {
    pub of: CommitmentId,
    pub kind: BasisKind,
}

/// Whether a commitment was made by its debtor or imposed by a rule.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Source {
    Act,
    Rule,
}

/// The terms of a commitment (its identity is the digest of these).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CommitmentTerms {
    pub debtor: ActorId,
    pub creditor: ActorId,
    pub ant: Option<Condition>,
    pub cons: Condition,
    pub deadline: Time,
    pub tags: BTreeSet<Tag>,
    pub parent: Option<CommitmentId>,
    pub terms: Option<Terms>,
    pub basis: Option<Basis>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Commitment {
    pub id: CommitmentId,
    pub terms: CommitmentTerms,
    pub source: Source,
    pub rules: Version,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NormError {
    DeadlineInThePast,
    UnknownParent,
    /// A commitment made "in service of" another must be made TO that other's debtor, by someone else.
    NotASubcontract,
    /// A maintenance condition was used as an antecedent.
    MaintenanceAsAntecedent,
    Duplicate(CommitmentId),
    NoSuchLiveCommitment,
    /// Only the creditor may release or assign; only the debtor may cancel.
    NotWithinTheActorsPower,
}

/// Why a template could not be instantiated.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SkipReason {
    NoHolderOfRole(Role),
    NoTerms,
    NoParty,
    /// The terms name a price, but no such payment was ever made: there is nothing to refund.
    NothingWasPaid,
}

//! The rule set: data the institution can amend by its own procedure.

use super::*;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

/// Who may be believed for a method of evidence.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Trust {
    Anyone,
    Only(BTreeSet<ActorId>),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EvidenceRules {
    /// A method with no rank carries no weight.
    pub ranks: BTreeMap<Method, Rank>,
    pub trusted: BTreeMap<Method, Trust>,
    /// Whether support rooted in the claimant's own word counts.
    pub self_support: bool,
}

/// Who a templated commitment binds or benefits.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PartyExpr {
    /// The debtor of the commitment the template fired on.
    Debtor,
    Creditor,
    /// The party named by the verdict's finding.
    Party,
    RoleHolder(Role),
    /// The payer named in the commitment's terms.
    Payer,
}

/// What a templated commitment requires.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConsExpr {
    /// `Remediated(c)` for the commitment `c` the template fired on.
    Remediated,
    Audited,
    /// A transfer of at least the price in the terms, from the new debtor to the payer.
    RefundOfPrice,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Template {
    pub debtor: PartyExpr,
    pub creditor: PartyExpr,
    pub cons: ConsExpr,
    /// Deadline, relative to the moment the template fires.
    pub within: u64,
    pub tags: BTreeSet<Tag>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ViolationTemplate {
    pub tag: Tag,
    pub create: Template,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VerdictTemplate {
    pub finding: Finding,
    pub tag: Tag,
    pub create: Template,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NormRules {
    pub on_violation: Vec<ViolationTemplate>,
    pub on_verdict: Vec<VerdictTemplate>,
    /// How long a maintenance undertaking about a change is expected to run. Adapters read it;
    /// the reflector may propose revising it from the record of how late refutations arrive.
    pub maintenance_period: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ResponsibilityRules {
    pub rules: Vec<RRule>,
    pub findings: BTreeSet<Finding>,
    /// Whether a new rule version re-judges violations that arose under an older one.
    pub retroactive: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GovernanceRules {
    pub quorum: u32,
    pub entrenched: BTreeSet<RulePath>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Rules {
    pub evidence: EvidenceRules,
    pub norms: NormRules,
    pub responsibility: ResponsibilityRules,
    pub governance: GovernanceRules,
}

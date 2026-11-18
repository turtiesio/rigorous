//! The rule set: data that the institution can amend by its own procedure.

use super::event::{ActionKind, Assessment, Finding, MemberKind, Method, Prop, Role, Status, Tag};
use super::ids::{ActorId, CommitmentId, Rank, RuleId};
use rigor_logic::datalog::{self, Atom, Fact, Literal, Rule, RuleError, Signature, Sorted, Term};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

// ---------------------------------------------------------------- responsibility vocabulary

/// Predicates of the responsibility language. The first block is supplied as facts by the
/// adjudicator; the second is derived by rules.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Pred {
    Status,
    Debtor,
    Creditor,
    /// `Parent(child, parent)`: the child commitment was made in service of the parent.
    Parent,
    Cons,
    HasTag,
    Claimed,
    Notified,
    Assessed,
    Kind,
    Principal,
    HasRole,

    Accountable,
    SubFailed,
    HasSubFailure,
    Knew,
    Misreported,
    Culpable,
    Negligent,
    Excused,
    Contributed,
    AtFault,
    IsAi,
    Liable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sort {
    Actor,
    Commitment,
    Prop,
    Status,
    Tag,
    Kind,
    Role,
    Bool,
    Assessment,
}

impl Signature for Pred {
    type Sort = Sort;
    fn signature(self) -> &'static [Sort] {
        use Sort::{Actor, Assessment, Bool, Commitment, Kind, Prop, Role, Status, Tag};
        match self {
            Self::Status => &[Commitment, Status],
            Self::Debtor | Self::Creditor => &[Commitment, Actor],
            Self::Parent | Self::SubFailed => &[Commitment, Commitment],
            Self::Cons => &[Commitment, Prop],
            Self::HasTag => &[Commitment, Tag],
            Self::Claimed => &[Actor, Prop],
            Self::Notified => &[Actor, Prop, Bool],
            Self::Assessed => &[Prop, Assessment],
            Self::Kind => &[Actor, Kind],
            Self::Principal => &[Actor, Actor],
            Self::HasRole => &[Actor, Role],
            Self::Accountable
            | Self::Knew
            | Self::Misreported
            | Self::Culpable
            | Self::Negligent
            | Self::Excused
            | Self::Contributed
            | Self::AtFault
            | Self::Liable => &[Actor, Commitment],
            Self::HasSubFailure => &[Commitment],
            Self::IsAi => &[Actor],
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Const {
    Actor(ActorId),
    Commitment(CommitmentId),
    Prop(Prop),
    Status(Status),
    Tag(Tag),
    Kind(MemberKind),
    Role(Role),
    Bool(bool),
    Assessment(Assessment),
}

impl Sorted<Sort> for Const {
    fn sort(&self) -> Sort {
        match self {
            Self::Actor(_) => Sort::Actor,
            Self::Commitment(_) => Sort::Commitment,
            Self::Prop(_) => Sort::Prop,
            Self::Status(_) => Sort::Status,
            Self::Tag(_) => Sort::Tag,
            Self::Kind(_) => Sort::Kind,
            Self::Role(_) => Sort::Role,
            Self::Bool(_) => Sort::Bool,
            Self::Assessment(_) => Sort::Assessment,
        }
    }
}

pub type RRule = Rule<RuleId, Pred, Const>;
pub type RFact = Fact<Pred, Const>;
pub type RAtom = Atom<Pred, Const>;
pub type RLiteral = Literal<Pred, Const>;
pub type RTerm = Term<Const>;
pub type RProof = datalog::Proof<RuleId, Pred, Const>;
pub type RRuleError = RuleError<RuleId, Pred>;

impl Finding {
    #[must_use]
    pub const fn pred(self) -> Pred {
        match self {
            Self::Accountable => Pred::Accountable,
            Self::Culpable => Pred::Culpable,
            Self::Negligent => Pred::Negligent,
            Self::Excused => Pred::Excused,
            Self::Contributed => Pred::Contributed,
            Self::Liable => Pred::Liable,
        }
    }
    pub const ALL: [Self; 6] = [Self::Accountable, Self::Culpable, Self::Negligent, Self::Excused, Self::Contributed, Self::Liable];
}

// ---------------------------------------------------------------- the rule set

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

/// The amendable parts of a rule set.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum RulePath {
    Ranks,
    Trust,
    SelfSupport,
    Templates,
    Period,
    Responsibility,
    Retroactive,
    Quorum,
    Entrenched,
}

/// One change to the rule set.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Amendment {
    SetRank(Method, Option<Rank>),
    SetTrust(Method, Trust),
    SetSelfSupport(bool),
    AddViolationTemplate(ViolationTemplate),
    AddVerdictTemplate(VerdictTemplate),
    SetMaintenancePeriod(u64),
    AddResponsibilityRules(Vec<RRule>),
    SetRetroactive(bool),
    SetQuorum(u32),
    Entrench(RulePath),
    Disentrench(RulePath),
}

impl Amendment {
    #[must_use]
    pub const fn path(&self) -> RulePath {
        match self {
            Self::SetRank(..) => RulePath::Ranks,
            Self::SetTrust(..) => RulePath::Trust,
            Self::SetSelfSupport(_) => RulePath::SelfSupport,
            Self::AddViolationTemplate(_) | Self::AddVerdictTemplate(_) => RulePath::Templates,
            Self::SetMaintenancePeriod(_) => RulePath::Period,
            Self::AddResponsibilityRules(_) => RulePath::Responsibility,
            Self::SetRetroactive(_) => RulePath::Retroactive,
            Self::SetQuorum(_) => RulePath::Quorum,
            Self::Entrench(_) | Self::Disentrench(_) => RulePath::Entrenched,
        }
    }
}

/// Why a rule set is not acceptable.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RulesError {
    Responsibility(RRuleError),
    QuorumZero,
    PeriodZero,
    /// Obligations could regress forever through this tag.
    TemplateCycle(Tag),
}

impl Rules {
    /// The rule set after applying amendments (not yet validated).
    #[must_use]
    pub fn amended(&self, amendments: &[Amendment]) -> Self {
        let mut r = self.clone();
        for a in amendments {
            match a {
                Amendment::SetRank(m, Some(rank)) => {
                    let _ = r.evidence.ranks.insert(*m, *rank);
                }
                Amendment::SetRank(m, None) => {
                    let _ = r.evidence.ranks.remove(m);
                }
                Amendment::SetTrust(m, t) => {
                    let _ = r.evidence.trusted.insert(*m, t.clone());
                }
                Amendment::SetSelfSupport(b) => r.evidence.self_support = *b,
                Amendment::AddViolationTemplate(t) => r.norms.on_violation.push(t.clone()),
                Amendment::AddVerdictTemplate(t) => r.norms.on_verdict.push(t.clone()),
                Amendment::SetMaintenancePeriod(p) => r.norms.maintenance_period = *p,
                Amendment::AddResponsibilityRules(rs) => r.responsibility.rules.extend(rs.iter().cloned()),
                Amendment::SetRetroactive(b) => r.responsibility.retroactive = *b,
                Amendment::SetQuorum(q) => r.governance.quorum = *q,
                Amendment::Entrench(p) => {
                    let _ = r.governance.entrenched.insert(*p);
                }
                Amendment::Disentrench(p) => {
                    let _ = r.governance.entrenched.remove(p);
                }
            }
        }
        r
    }

    /// Static validation, performed on the constitution and before any amendment is enacted.
    ///
    /// # Errors
    /// See [`RulesError`].
    pub fn validate(&self) -> Result<(), RulesError> {
        let _ = datalog::check(&self.responsibility.rules).map_err(RulesError::Responsibility)?;
        if self.governance.quorum < 1 {
            return Err(RulesError::QuorumZero);
        }
        if self.norms.maintenance_period == 0 {
            return Err(RulesError::PeriodZero);
        }
        let mut edges: BTreeMap<Tag, BTreeSet<Tag>> = BTreeMap::new();
        for t in &self.norms.on_violation {
            edges.entry(t.tag).or_default().extend(t.create.tags.iter().copied());
        }
        for t in &self.norms.on_verdict {
            edges.entry(t.tag).or_default().extend(t.create.tags.iter().copied());
        }
        // A cycle exists iff some tag reaches itself.
        for start in edges.keys() {
            let mut seen: BTreeSet<Tag> = BTreeSet::new();
            let mut stack: Vec<Tag> = edges.get(start).into_iter().flatten().copied().collect();
            while let Some(t) = stack.pop() {
                if t == *start {
                    return Err(RulesError::TemplateCycle(t));
                }
                if seen.insert(t) {
                    stack.extend(edges.get(&t).into_iter().flatten().copied());
                }
            }
        }
        Ok(())
    }
}

/// The powers a member has by being a member.
#[must_use]
pub fn base_actions() -> BTreeSet<ActionKind> {
    use ActionKind::{Assign, Attest, Cancel, Claim, Commit, Release, Request, Retract, Revoke};
    [Claim, Attest, Retract, Commit, Release, Cancel, Assign, Request, Revoke].into_iter().collect()
}

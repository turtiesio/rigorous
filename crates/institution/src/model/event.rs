//! The closed vocabulary of events.

use super::ids::{
    ActorId, Amount, ArtifactId, CapId, ChangeId, CharterName, CommitmentId, ContentDigest, EvidenceId, ProposalId, Requirement, Resource, TaskName,
    TemplateIx, Time, Version,
};
use super::rules::{Amendment, RFact, RProof, RulePath, Rules, RulesError};
use rigor_kernel::{Name, Path, Payload};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

// ---------------------------------------------------------------- small closed vocabularies

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Finding {
    Accountable,
    Culpable,
    Negligent,
    Excused,
    Contributed,
    Liable,
}

/// A piece of work, possibly one step of a larger one.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Task {
    pub name: TaskName,
    pub step: Option<TaskName>,
}

impl Task {
    #[must_use]
    pub const fn whole(name: TaskName) -> Self {
        Self { name, step: None }
    }
    #[must_use]
    pub fn step(&self, step: TaskName) -> Self {
        Self { name: self.name.clone(), step: Some(step) }
    }
    /// The task this is a step of, if it is a step.
    #[must_use]
    pub fn parent(&self) -> Option<Self> {
        self.step.as_ref().map(|_| Self { name: self.name.clone(), step: None })
    }
}

/// One published version of a package.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Release {
    pub package: Name,
    pub version: (u64, u64, u64),
}

/// Something that can be claimed, evidenced and assessed. Structured, never parsed.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Prop {
    Done(Task),
    Remediated(CommitmentId),
    Audited(CommitmentId),
    Conforms(Requirement),
    /// The change is sound (should stand).
    Sound(ChangeId),
    /// The snapshot with this digest is the state its log really produces.
    Faithful(ContentDigest),
    /// The release is fit to depend on.
    Fit(Release),
    /// The system is able, now, to do the named thing (every need of it is met).
    Can(Name),
    /// The document with this digest adequately answers the named question.
    Answers {
        question: Name,
        answer: ContentDigest,
    },
    /// The change has been applied to the thing it changes.
    Applied(ChangeId),
    Fact(Name),
}

impl fmt::Display for Prop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Done(Task { name, step: None }) => write!(f, "done({name})"),
            Self::Done(Task { name, step: Some(s) }) => write!(f, "done({name}.{s})"),
            Self::Remediated(c) => write!(f, "remediated({c})"),
            Self::Audited(c) => write!(f, "audited({c})"),
            Self::Conforms(r) => write!(f, "conforms({r})"),
            Self::Sound(c) => write!(f, "sound({c})"),
            Self::Faithful(d) => write!(f, "faithful({d})"),
            Self::Fit(r) => write!(f, "fit({} {}.{}.{})", r.package, r.version.0, r.version.1, r.version.2),
            Self::Can(n) => write!(f, "can({n})"),
            Self::Answers { question, answer } => write!(f, "answers({question}, {answer})"),
            Self::Applied(c) => write!(f, "applied({c})"),
            Self::Fact(n) => write!(f, "fact({n})"),
        }
    }
}

// ---------------------------------------------------------------- acts (what an actor can attempt)

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
pub enum Memo {
    None,
    /// Part of the named settlement.
    Purpose(Name),
    Payment(Task),
    Refund(CommitmentId),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Transfer {
    pub from: ActorId,
    pub to: ActorId,
    pub resource: Resource,
    pub amount: Amount,
    pub memo: Memo,
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

/// Everything an actor can attempt.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Act {
    Delegate(Grant),
    Revoke {
        target: CapId,
    },
    Transfer(Transfer),
    Claim {
        prop: Prop,
    },
    Attest(Attestation),
    Retract {
        evidence: EvidenceId,
    },
    Commit(Offer),
    Release {
        id: CommitmentId,
    },
    Cancel {
        id: CommitmentId,
    },
    Assign {
        id: CommitmentId,
        to: ActorId,
    },
    Request(Request),
    Propose {
        amendments: Vec<Amendment>,
    },
    Approve {
        id: ProposalId,
    },
    /// Admit a new member (a power granted by charter).
    Admit(Member),
    /// Remove a member's standing. Their obligations remain; everything they delegated is revoked.
    Expel {
        member: ActorId,
    },
    /// Give a member a role, or take it away.
    SetRole {
        member: ActorId,
        role: Option<Role>,
    },
}

impl Act {
    #[must_use]
    pub const fn kind(&self) -> ActionKind {
        match self {
            Self::Delegate(_) => ActionKind::Delegate,
            Self::Revoke { .. } => ActionKind::Revoke,
            Self::Transfer(_) => ActionKind::Transfer,
            Self::Claim { .. } => ActionKind::Claim,
            Self::Attest(_) => ActionKind::Attest,
            Self::Retract { .. } => ActionKind::Retract,
            Self::Commit(_) => ActionKind::Commit,
            Self::Release { .. } => ActionKind::Release,
            Self::Cancel { .. } => ActionKind::Cancel,
            Self::Assign { .. } => ActionKind::Assign,
            Self::Request(_) => ActionKind::Request,
            Self::Propose { .. } => ActionKind::Propose,
            Self::Approve { .. } => ActionKind::Approve,
            Self::Admit(_) => ActionKind::Admit,
            Self::Expel { .. } => ActionKind::Expel,
            Self::SetRole { .. } => ActionKind::SetRole,
        }
    }
}

// ---------------------------------------------------------------- records carried by events

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

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LedgerError {
    UnknownAccount,
    InsufficientFunds,
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
    Duplicate,
    /// Only the source may retract its evidence.
    NotTheSource,
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

/// The enumerated ways an institutional unit can be unable to proceed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Failure {
    /// An invalid constitution is not a constitution.
    InvalidConstitution(RulesError),
    /// Rules that were validated at enactment no longer check (a bug).
    EnactedRulesDoNotCheck(super::rules::RRuleError),
}

/// A kernel fault in a world of institutional events.
pub type Fault = rigor_kernel::Fault<Kind, Failure>;
/// A leaf error in a world of institutional events.
pub type LeafError = rigor_kernel::LeafError<Failure>;

/// Bring an external stimulus written under an older log schema up to the current vocabulary.
/// Total: every older event has exactly one current form.
///
/// * 1 -> 2 changed only the file's header.
/// * 2 -> 3: a charter says whether it is delegable (older charters were not); the norm rules
///   carry a maintenance period (older rule sets meant the then-fixed 30 days). Fields that
///   became optional (a scope's purpose) need no step.
#[must_use]
pub fn upcast(from_schema: u32, mut event: serde_json::Value) -> serde_json::Value {
    use serde_json::{json, Value};
    if from_schema < 3 {
        if let Some(constitution) = event.get_mut("Genesis") {
            for charter in constitution.get_mut("charters").and_then(Value::as_array_mut).into_iter().flatten() {
                if let Some(c) = charter.as_object_mut() {
                    let _ = c.entry("delegable").or_insert(json!(false));
                }
            }
            if let Some(norms) = constitution.pointer_mut("/rules/norms").and_then(Value::as_object_mut) {
                let _ = norms.entry("maintenance_period").or_insert(json!(30 * 24 * 3600));
            }
        }
    }
    event
}

/// A unit of work inside a service.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Job {
    pub task: Task,
    pub client: ActorId,
}

// ---------------------------------------------------------------- the event type

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Event {
    // -- from outside the system
    Genesis(Box<Constitution>),
    Tick(Time),
    ArtifactChanged {
        artifact: ArtifactId,
        digest: ContentDigest,
    },
    // -- kernel
    Settle,
    Scoped {
        entity: Name,
        inner: Box<Event>,
    },
    /// Something that happened outside a federated boundary, as seen from inside it.
    Foreign(Box<Event>),
    /// Something an organisation's envoy addresses to the world outside its boundary.
    Abroad(Box<Event>),
    /// This log continues one written in an older format, whose chain was verified up to `old_head`.
    Migrated {
        from_schema: u32,
        old_head: rigor_kernel::Digest,
        stimuli: u64,
    },
    // -- brute: anyone may emit; the kernel stamps who did
    Do {
        act: Act,
        cap: Option<CapId>,
    },
    Signal {
        to: ActorId,
        prop: Prop,
        value: bool,
    },
    // -- authority (only the institution emits these)
    Ok {
        by: ActorId,
        via: CapId,
        act: Act,
    },
    Delegated(Capability),
    Denied {
        by: Path,
        act: Act,
        via: Option<CapId>,
        reason: DenyReason,
    },
    Member(Member),
    Expelled {
        member: ActorId,
    },
    // -- resources
    Transferred {
        transfer: Transfer,
        by: ActorId,
    },
    TransferFailed {
        transfer: Transfer,
        via: CapId,
        reason: LedgerError,
    },
    // -- evidence
    EvidenceRecorded(Evidence),
    EvidenceRejected {
        by: ActorId,
        reason: EvidenceError,
    },
    EvidenceRetracted {
        id: EvidenceId,
        by: ActorId,
    },
    Assessed {
        prop: Prop,
        assessment: Assessment,
        since: Option<Time>,
        basis: Vec<EvidenceId>,
        was: Assessment,
        rules: Version,
    },
    // -- norms
    NormCreated(Commitment),
    NormStatus {
        id: CommitmentId,
        was: Option<Status>,
        status: Status,
        at: Time,
        debtor: ActorId,
        creditor: ActorId,
    },
    NormRejected {
        by: Option<ActorId>,
        reason: NormError,
    },
    NormAssigned {
        id: CommitmentId,
        creditor: ActorId,
        by: ActorId,
    },
    NormSkipped {
        of: CommitmentId,
        kind: BasisKind,
        reason: SkipReason,
    },
    // -- responsibility
    Verdict(Verdict),
    // -- governance
    RulesChanged(Box<RulesChanged>),
    Proposed {
        id: ProposalId,
        by: ActorId,
        base: Version,
        paths: BTreeSet<RulePath>,
    },
    Approved {
        id: ProposalId,
        by: ActorId,
        approvals: BTreeSet<ActorId>,
    },
    GovRejected {
        by: ActorId,
        reason: GovError,
    },
    // -- inside a service
    JobQueued(Job),
    JobDone(Job),
    JobFailed(Job),
}

/// One kind per event variant: the alphabet of interfaces.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Kind {
    Genesis,
    Tick,
    ArtifactChanged,
    Settle,
    Scoped,
    Foreign,
    Abroad,
    Migrated,
    Do,
    Signal,
    Ok,
    Delegated,
    Denied,
    Member,
    Expelled,
    Transferred,
    TransferFailed,
    EvidenceRecorded,
    EvidenceRejected,
    EvidenceRetracted,
    Assessed,
    NormCreated,
    NormStatus,
    NormRejected,
    NormAssigned,
    NormSkipped,
    Verdict,
    RulesChanged,
    Proposed,
    Approved,
    GovRejected,
    JobQueued,
    JobDone,
    JobFailed,
}

impl Payload for Event {
    type Kind = Kind;
    type Failure = Failure;

    fn kind(&self) -> Kind {
        match self {
            Self::Genesis(_) => Kind::Genesis,
            Self::Tick(_) => Kind::Tick,
            Self::ArtifactChanged { .. } => Kind::ArtifactChanged,
            Self::Settle => Kind::Settle,
            Self::Scoped { .. } => Kind::Scoped,
            Self::Migrated { .. } => Kind::Migrated,
            Self::Foreign(_) => Kind::Foreign,
            Self::Abroad(_) => Kind::Abroad,
            Self::Do { .. } => Kind::Do,
            Self::Signal { .. } => Kind::Signal,
            Self::Ok { .. } => Kind::Ok,
            Self::Delegated(_) => Kind::Delegated,
            Self::Denied { .. } => Kind::Denied,
            Self::Member(_) => Kind::Member,
            Self::Expelled { .. } => Kind::Expelled,
            Self::Transferred { .. } => Kind::Transferred,
            Self::TransferFailed { .. } => Kind::TransferFailed,
            Self::EvidenceRecorded(_) => Kind::EvidenceRecorded,
            Self::EvidenceRejected { .. } => Kind::EvidenceRejected,
            Self::EvidenceRetracted { .. } => Kind::EvidenceRetracted,
            Self::Assessed { .. } => Kind::Assessed,
            Self::NormCreated(_) => Kind::NormCreated,
            Self::NormStatus { .. } => Kind::NormStatus,
            Self::NormRejected { .. } => Kind::NormRejected,
            Self::NormAssigned { .. } => Kind::NormAssigned,
            Self::NormSkipped { .. } => Kind::NormSkipped,
            Self::Verdict(_) => Kind::Verdict,
            Self::RulesChanged(_) => Kind::RulesChanged,
            Self::Proposed { .. } => Kind::Proposed,
            Self::Approved { .. } => Kind::Approved,
            Self::GovRejected { .. } => Kind::GovRejected,
            Self::JobQueued(_) => Kind::JobQueued,
            Self::JobDone(_) => Kind::JobDone,
            Self::JobFailed(_) => Kind::JobFailed,
        }
    }
    fn settle() -> Self {
        Self::Settle
    }
    fn settle_kind() -> Kind {
        Kind::Settle
    }
    fn scoped(entity: Name, inner: Self) -> Self {
        Self::Scoped { entity, inner: Box::new(inner) }
    }
    fn as_scoped(&self) -> Option<(&Name, &Self)> {
        match self {
            Self::Scoped { entity, inner } => Some((entity, inner)),
            _ => None,
        }
    }
    fn scoped_kind() -> Kind {
        Kind::Scoped
    }
    fn foreign(inner: Self) -> Self {
        Self::Foreign(Box::new(inner))
    }
    fn foreign_kind() -> Kind {
        Kind::Foreign
    }
    fn as_abroad(&self) -> Option<&Self> {
        match self {
            Self::Abroad(inner) => Some(inner),
            _ => None,
        }
    }
}

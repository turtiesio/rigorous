//! The closed sum of events, their kinds, and what the waist needs to know about them.

#[allow(clippy::wildcard_imports)]
use super::*;
use rigor_waist::{Name, Path, Payload};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

/// A unit of work inside a service.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Job {
    pub task: Task,
    pub client: ActorId,
}

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
        old_head: rigor_waist::Digest,
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
        /// No later fact can change it: released, cancelled, void, or repudiated by its debtor.
        closed: bool,
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
    fn may_come_from(&self, actor: &Path) -> bool {
        self.admissible_from(actor)
    }
}

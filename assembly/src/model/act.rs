//! Everything an actor can attempt: the one place where the concepts' requests meet.

#[allow(clippy::wildcard_imports)]
use super::*;
use serde::{Deserialize, Serialize};

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

//! Transfers of countable resources.

#[allow(clippy::wildcard_imports)]
use super::*;
use rigor_waist::Name;
use serde::{Deserialize, Serialize};

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

/// Why a transfer was not carried out: the value concept's own refusal.
pub use rigor_value::Refusal as LedgerError;

//! The enumerated ways a unit can be unable to proceed (as opposed to refusing).

#[allow(clippy::wildcard_imports)]
use super::*;

/// The enumerated ways an institutional unit can be unable to proceed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Failure {
    /// An invalid constitution is not a constitution.
    InvalidConstitution(RulesError),
    /// Rules that were validated at enactment no longer check (a bug).
    EnactedRulesDoNotCheck(RRuleError),
    /// The founding balances do not fit in an account.
    BalanceSheetOverflows,
}

/// A kernel fault in a world of institutional events.
pub type Fault = rigor_waist::Fault<Kind, Failure>;
/// A leaf error in a world of institutional events.
pub type LeafError = rigor_waist::LeafError<Failure>;

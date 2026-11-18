//! What the ledger assumes of its surroundings. It cannot check these itself.

/// An assumption about whatever the ledger is wired to.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Need {
    /// An order it hears has already been authorised: the ledger moves whatever it is told to.
    OrdersAreAuthorised,
    /// Nothing else changes balances: the ledger is the only writer of its accounts.
    SoleKeeperOfBalances,
}

pub const NEEDS: [Need; 2] = [Need::OrdersAreAuthorised, Need::SoleKeeperOfBalances];

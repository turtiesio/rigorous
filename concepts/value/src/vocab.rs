//! What can be said to the ledger and what it says.

use rigor_waist::Key;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// The nouns the surrounding system supplies.
pub trait Terms: 'static {
    /// Who can hold a balance.
    type Account: Key;
    /// What is counted.
    type Resource: Key;
    /// Whatever the system attaches to an order (who placed it, under what authority, for
    /// what). The ledger carries it back untouched and never looks inside.
    type Context: Clone + Debug;
}

/// A non-negative quantity of one resource.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct Quantity(pub u64);

/// An order to move a quantity of a resource from one account to another.
#[derive(Debug)]
pub struct Order<T: Terms> {
    pub from: T::Account,
    pub to: T::Account,
    pub resource: T::Resource,
    pub quantity: Quantity,
    pub context: T::Context,
}

#[derive(Debug)]
pub enum Heard<T: Terms> {
    /// The founding balance sheet. Only the first one counts.
    Founded {
        accounts: Vec<T::Account>,
        balances: Vec<(T::Account, T::Resource, Quantity)>,
    },
    /// A new account holder.
    Opened(T::Account),
    Ordered(Order<T>),
}

#[derive(Debug)]
pub enum Said<T: Terms> {
    Moved(Order<T>),
    Failed { order: Order<T>, reason: Refusal },
}

/// Why an order was not carried out. Nothing moved.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Refusal {
    UnknownAccount,
    InsufficientFunds,
    /// The receiving balance cannot hold the amount.
    Overflow,
}

/// Why the ledger cannot proceed at all.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Broken {
    /// The founding balance sheet does not fit an account.
    BalanceSheetOverflows,
}

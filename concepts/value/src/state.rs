//! What the ledger remembers.

use crate::vocab::{Quantity, Terms};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Serialize, Deserialize)]
#[serde(bound = "")]
pub struct Ledger<T: Terms> {
    pub(crate) founded: bool,
    pub(crate) accounts: BTreeSet<T::Account>,
    pub(crate) bal: BTreeMap<(T::Account, T::Resource), Quantity>,
    /// Total ever minted per resource, in a type wide enough for any sum of balances.
    pub(crate) minted: BTreeMap<T::Resource, u128>,
}

// Written out because a derive would demand the same of `T`, which is only a bundle of types.
impl<T: Terms> Default for Ledger<T> {
    fn default() -> Self {
        Self { founded: false, accounts: BTreeSet::new(), bal: BTreeMap::new(), minted: BTreeMap::new() }
    }
}

impl<T: Terms> Clone for Ledger<T> {
    fn clone(&self) -> Self {
        Self { founded: self.founded, accounts: self.accounts.clone(), bal: self.bal.clone(), minted: self.minted.clone() }
    }
}

impl<T: Terms> std::hash::Hash for Ledger<T> {
    fn hash<H: std::hash::Hasher>(&self, h: &mut H) {
        (self.founded, &self.accounts, &self.bal, &self.minted).hash(h);
    }
}

impl<T: Terms> std::fmt::Debug for Ledger<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Ledger")
            .field("founded", &self.founded)
            .field("accounts", &self.accounts)
            .field("bal", &self.bal)
            .field("minted", &self.minted)
            .finish()
    }
}

//! What can be asked of the ledger.

use crate::state::Ledger;
use crate::vocab::{Quantity, Terms};
use std::collections::BTreeMap;

impl<T: Terms> Ledger<T> {
    #[must_use]
    pub fn balance(&self, who: &T::Account, r: &T::Resource) -> Quantity {
        self.bal.get(&(who.clone(), r.clone())).copied().unwrap_or_default()
    }

    /// For every resource, balances sum to what was minted at the founding.
    #[must_use]
    pub fn conserved(&self) -> bool {
        // Summed in a wider type: the check must not be able to hide a loss by saturating.
        let mut total: BTreeMap<&T::Resource, u128> = BTreeMap::new();
        for ((_, r), n) in &self.bal {
            *total.entry(r).or_default() += u128::from(n.0);
        }
        total.len() <= self.minted.len() && self.minted.iter().all(|(r, n)| total.get(r).copied().unwrap_or(0) == *n)
    }

    /// Every balance is held by a known account.
    #[must_use]
    pub fn only_accounts_hold(&self) -> bool {
        self.bal.keys().all(|(a, _)| self.accounts.contains(a))
    }
}

//! Resources: balances over a commutative monoid. Totals are conserved; nothing goes negative.

use crate::model::LeafError;
use crate::model::{Act, ActorId, Amount, Event, Kind, LedgerError, Resource};
use rigor_kernel::{Envelope, Leaf, Name};
use std::collections::{BTreeMap, BTreeSet};

pub const LEDGER: Name = Name::lit("ledger");

#[derive(Clone, Debug, Hash, Default, serde::Serialize, serde::Deserialize)]
pub struct Ledger {
    founded: bool,
    accounts: BTreeSet<ActorId>,
    bal: BTreeMap<(ActorId, Resource), Amount>,
    /// Total ever minted per resource, in a type wide enough for any sum of balances.
    minted: BTreeMap<Resource, u128>,
}

impl Ledger {
    #[must_use]
    pub fn balance(&self, who: &ActorId, r: &Resource) -> Amount {
        self.bal.get(&(who.clone(), r.clone())).copied().unwrap_or_default()
    }

    /// Invariant: for every resource, balances sum to what was minted at genesis.
    #[must_use]
    pub fn conserved(&self) -> bool {
        // Summed in a wider type: the check must not be able to hide a loss by saturating.
        let mut total: BTreeMap<&Resource, u128> = BTreeMap::new();
        for ((_, r), n) in &self.bal {
            *total.entry(r).or_default() += u128::from(n.0);
        }
        total.len() <= self.minted.len() && self.minted.iter().all(|(r, n)| total.get(r).copied().unwrap_or(0) == *n)
    }
}

impl Leaf<Event> for Ledger {
    fn name(&self) -> Name {
        LEDGER
    }
    fn inputs(&self) -> BTreeSet<Kind> {
        [Kind::Genesis, Kind::Ok, Kind::Member].into_iter().collect()
    }
    fn outputs(&self) -> BTreeSet<Kind> {
        [Kind::Transferred, Kind::TransferFailed].into_iter().collect()
    }
    fn on(&mut self, env: &Envelope<'_, Event>) -> Result<Vec<Event>, LeafError> {
        match env.event {
            Event::Genesis(c) if !self.founded => {
                self.founded = true;
                self.accounts.extend(c.members.iter().map(|m| m.id.clone()));
                for (who, r, n) in &c.balances {
                    let _ = self.accounts.insert(who.clone());
                    // A founding balance sheet that overflows an account is not one this ledger can keep.
                    let b = self.bal.entry((who.clone(), r.clone())).or_default();
                    b.0 = b.0.checked_add(n.0).ok_or(LeafError::Broken(crate::model::Failure::BalanceSheetOverflows))?;
                    *self.minted.entry(r.clone()).or_default() += u128::from(n.0);
                }
                Ok(Vec::new())
            }
            Event::Member(m) => {
                let _ = self.accounts.insert(m.id.clone());
                Ok(Vec::new())
            }
            Event::Ok { by, via, act: Act::Transfer(t) } => {
                let fail = |reason| Event::TransferFailed { transfer: t.clone(), via: via.clone(), reason };
                if !self.accounts.contains(&t.from) || !self.accounts.contains(&t.to) {
                    return Ok(vec![fail(LedgerError::UnknownAccount)]);
                }
                let have = self.balance(&t.from, &t.resource);
                let Some(rest) = have.0.checked_sub(t.amount.0) else {
                    return Ok(vec![fail(LedgerError::InsufficientFunds)]);
                };
                // To oneself: nothing moves. Otherwise the credit must fit, or nothing is debited.
                if t.from != t.to {
                    let Some(more) = self.balance(&t.to, &t.resource).0.checked_add(t.amount.0) else {
                        return Ok(vec![fail(LedgerError::Overflow)]);
                    };
                    let _ = self.bal.insert((t.from.clone(), t.resource.clone()), Amount(rest));
                    let _ = self.bal.insert((t.to.clone(), t.resource.clone()), Amount(more));
                }
                Ok(vec![Event::Transferred { transfer: t.clone(), by: by.clone() }])
            }
            _ => Ok(Vec::new()),
        }
    }
}

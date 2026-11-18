//! Independent bookkeeping of the same blocks, and the comparison with the world.

use crate::block::*;
use crate::cast::*;
use rigor_institution::model::{ActorId, Amount, CapId};
use rigor_institution::world::{Error, World};
use rigor_waist::Name;
use std::collections::{BTreeMap, BTreeSet};

pub(crate) fn fee(t: &Tx) -> u64 {
    let spent: u64 = t.inputs.iter().map(|i| i.amount.0).sum();
    spent.saturating_sub(t.outputs.iter().map(|(_, a)| a.0).sum())
}

/// What every account must hold before the first block, for the window to be consistent:
/// coins spent in the window but created before it, and the treasury's stock of new coin.
#[must_use]
pub fn opening(blocks: &[Block]) -> BTreeMap<ActorId, u64> {
    let inside: BTreeSet<&Name> = blocks.iter().flat_map(|b| b.txs.iter().map(|t| &t.id)).collect();
    let mut open: BTreeMap<ActorId, u64> = BTreeMap::new();
    let mut new_coin = 0u64;
    for b in blocks {
        let fees: u64 = b.txs.iter().filter(|t| !t.reward).map(fee).sum();
        for t in &b.txs {
            if t.reward {
                let paid: u64 = t.outputs.iter().map(|(_, a)| a.0).sum();
                new_coin = new_coin.saturating_add(paid.saturating_sub(fees));
            }
            for i in t.inputs.iter().filter(|i| !inside.contains(&i.from_tx)) {
                *open.entry(i.owner.clone()).or_default() += i.amount.0;
            }
        }
    }
    *open.entry(TREASURY).or_default() += new_coin;
    open
}

/// Final balances by plain bookkeeping, independent of the institution: opening, plus received,
/// minus spent; fees to the treasury, rewards from it.
#[must_use]
pub fn oracle(blocks: &[Block]) -> BTreeMap<ActorId, u64> {
    // Signed running totals: within a block the reward is listed before the fees it collects.
    let mut bal: BTreeMap<ActorId, i128> = opening(blocks).into_iter().map(|(who, n)| (who, i128::from(n))).collect();
    for t in blocks.iter().flat_map(|b| &b.txs) {
        for i in &t.inputs {
            *bal.entry(i.owner.clone()).or_default() -= i128::from(i.amount.0);
        }
        let paid: u64 = t.outputs.iter().map(|(_, a)| a.0).sum();
        for (who, a) in &t.outputs {
            *bal.entry(who.clone()).or_default() += i128::from(a.0);
        }
        *bal.entry(TREASURY).or_default() += if t.reward { -i128::from(paid) } else { i128::from(fee(t)) };
    }
    bal.into_iter().map(|(who, n)| (who, u64::try_from(n).unwrap_or(u64::MAX))).collect()
}

/// What the checks found, comparing the institution's ledger with plain bookkeeping.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Verdict {
    pub accounts: usize,
    pub balances_equal_oracle: bool,
    pub conserved: bool,
    pub no_capability_amplifies: bool,
    /// Every delegated capability was spent to exactly its limit, never beyond.
    pub every_budget_exactly_spent: bool,
}

/// Compare the world with the oracle.
///
/// # Errors
/// If the world lacks an institutional unit.
pub fn check(w: &World, blocks: &[Block]) -> Result<Verdict, Error> {
    let (ledger, inst) = (w.ledger()?, w.inst()?);
    let expected = oracle(blocks);
    let balances_equal_oracle = expected.iter().all(|(who, n)| ledger.balance(who, &SAT) == Amount(*n));
    let every_budget_exactly_spent =
        inst.caps().values().filter(|c| matches!(c.id, CapId::Delegated(_))).all(|c| c.limits.get(&SAT).is_some_and(|lim| inst.spent(&c.id, &SAT) == *lim));
    Ok(Verdict {
        accounts: expected.len(),
        balances_equal_oracle,
        conserved: ledger.conserved(),
        no_capability_amplifies: inst.audit().is_empty(),
        every_budget_exactly_spent,
    })
}

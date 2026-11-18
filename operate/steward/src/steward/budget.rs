//! The budget for model calls: what is granted, what is left, paying.

use super::*;

impl Steward {
    /// The budgets the steward currently holds: capability, limit, and how much is spent.
    ///
    /// # Errors
    /// If the world lacks an institutional unit.
    pub fn budgets(&self) -> Result<Vec<(rigor_institution::model::CapId, u64, u64)>, StewardError> {
        let inst = self.world.inst()?;
        Ok(inst
            .caps()
            .values()
            .filter(|c| c.holder == CI && c.scope.from == Some(OWNER) && inst.valid(&c.id))
            .filter_map(|c| c.limits.get(&MICRO_USD).map(|lim| (c.id.clone(), lim.0, inst.spent(&c.id, &MICRO_USD).0)))
            .collect())
    }

    /// What is left to spend, across budgets.
    ///
    /// # Errors
    /// If the world lacks an institutional unit.
    pub fn budget_left(&self) -> Result<u64, StewardError> {
        Ok(self.budgets()?.iter().map(|(_, limit, spent)| limit.saturating_sub(*spent)).sum())
    }

    /// Pay for a consultation out of the budget. The institution's own authority and ledger
    /// units enforce the limit: returns false if the payment was refused.
    ///
    /// # Errors
    /// On a kernel fault.
    pub fn pay(&mut self, micro_usd: u64) -> Result<bool, StewardError> {
        use rigor_institution::model::{Amount, Memo, Transfer};
        if micro_usd == 0 {
            return Ok(true);
        }
        for (cap, limit, spent) in self.budgets()? {
            if limit.saturating_sub(spent) >= micro_usd {
                let t = Transfer { from: OWNER, to: GATEWAY, resource: MICRO_USD, amount: Amount(micro_usd), memo: Memo::None };
                let entries = self.world.act_with(&CI, &cap, Act::Transfer(t))?;
                return Ok(entries.iter().any(|e| matches!(e.event, Event::Transferred { .. })));
            }
        }
        Ok(false)
    }
}

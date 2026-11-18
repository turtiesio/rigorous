//! What happens when the ledger hears something.

use crate::state::Ledger;
use crate::vocab::{Broken, Heard, Quantity, Refusal, Said, Terms};
use rigor_waist::{Concept, Name, Who};

impl<T: Terms> Concept for Ledger<T> {
    type Heard = Heard<T>;
    type Said = Said<T>;
    type Broken = Broken;

    fn name(&self) -> Name {
        crate::LEDGER
    }

    fn step(&mut self, _from: &Who, heard: Heard<T>) -> Result<Vec<Said<T>>, Broken> {
        match heard {
            Heard::Founded { accounts, balances } if !self.founded => {
                self.founded = true;
                self.accounts.extend(accounts);
                for (who, r, n) in balances {
                    let _ = self.accounts.insert(who.clone());
                    let b = self.bal.entry((who, r.clone())).or_default();
                    b.0 = b.0.checked_add(n.0).ok_or(Broken::BalanceSheetOverflows)?;
                    *self.minted.entry(r).or_default() += u128::from(n.0);
                }
                Ok(Vec::new())
            }
            Heard::Founded { .. } => Ok(Vec::new()),
            Heard::Opened(a) => {
                let _ = self.accounts.insert(a);
                Ok(Vec::new())
            }
            Heard::Ordered(order) => {
                let fail = |order, reason| Ok(vec![Said::Failed { order, reason }]);
                if !self.accounts.contains(&order.from) || !self.accounts.contains(&order.to) {
                    return fail(order, Refusal::UnknownAccount);
                }
                let Some(rest) = self.balance(&order.from, &order.resource).0.checked_sub(order.quantity.0) else {
                    return fail(order, Refusal::InsufficientFunds);
                };
                // To oneself: nothing moves. Otherwise the credit must fit, or nothing is debited.
                if order.from != order.to {
                    let Some(more) = self.balance(&order.to, &order.resource).0.checked_add(order.quantity.0) else {
                        return fail(order, Refusal::Overflow);
                    };
                    let _ = self.bal.insert((order.from.clone(), order.resource.clone()), Quantity(rest));
                    let _ = self.bal.insert((order.to.clone(), order.resource.clone()), Quantity(more));
                }
                Ok(vec![Said::Moved(order)])
            }
        }
    }
}

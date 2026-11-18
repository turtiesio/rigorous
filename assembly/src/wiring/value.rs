//! Value in this system: members hold accounts; an authorised transfer is an order.

use crate::model::{Act, ActorId, CapId, Event, Failure, Kind, Resource, Transfer};
use rigor_value::{Broken, Heard, Order, Said, Terms};
use rigor_waist::{Hears, Speaks};
use std::collections::BTreeSet;

pub use rigor_value::LEDGER;

/// The nouns this system gives the ledger.
#[derive(Debug)]
pub struct Money;

/// What travels with an order: the transfer as it was requested, who requested it, and the
/// capability it was authorised under.
#[derive(Clone, Debug)]
pub struct Ticket {
    transfer: Transfer,
    by: ActorId,
    via: CapId,
}

impl Terms for Money {
    type Account = ActorId;
    type Resource = Resource;
    type Context = Ticket;
}

/// The ledger of this system.
pub type Ledger = rigor_value::Ledger<Money>;

impl Hears<Heard<Money>> for Event {
    fn audible() -> BTreeSet<Kind> {
        [Kind::Genesis, Kind::Ok, Kind::Member].into_iter().collect()
    }
    fn hear(&self) -> Option<Heard<Money>> {
        match self {
            Self::Genesis(c) => Some(Heard::Founded { accounts: c.members.iter().map(|m| m.id.clone()).collect(), balances: c.balances.clone() }),
            Self::Member(m) => Some(Heard::Opened(m.id.clone())),
            Self::Ok { by, via, act: Act::Transfer(t) } => Some(Heard::Ordered(Order {
                from: t.from.clone(),
                to: t.to.clone(),
                resource: t.resource.clone(),
                quantity: t.amount,
                context: Ticket { transfer: t.clone(), by: by.clone(), via: via.clone() },
            })),
            _ => None,
        }
    }
}

impl Speaks<Said<Money>> for Event {
    fn spoken() -> BTreeSet<Kind> {
        [Kind::Transferred, Kind::TransferFailed].into_iter().collect()
    }
    fn say(said: Said<Money>) -> Self {
        match said {
            Said::Moved(Order { context: Ticket { transfer, by, .. }, .. }) => Self::Transferred { transfer, by },
            Said::Failed { order: Order { context: Ticket { transfer, via, .. }, .. }, reason } => Self::TransferFailed { transfer, via, reason },
        }
    }
}

impl From<Broken> for Failure {
    fn from(b: Broken) -> Self {
        match b {
            Broken::BalanceSheetOverflows => Self::BalanceSheetOverflows,
        }
    }
}

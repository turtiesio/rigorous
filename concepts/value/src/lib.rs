//! Value: countable resources held in accounts. Totals are conserved; nothing goes negative.
//!
//! The concept knows accounts, resources and quantities. It does not know who may order a
//! movement or why: it hears orders that something else has already authorised.
//!
//! * `vocab`  what can be said to it and by it
//! * `state`  what it remembers
//! * `step`   what happens
//! * `query`  what can be asked of it
//! * `needs`  what it assumes of its surroundings
//! * `laws`   what it guarantees, as checkable statements

mod laws;
mod needs;
mod query;
mod state;
mod step;
mod vocab;

pub use laws::{Law, LAWS};
pub use needs::{Need, NEEDS};
pub use state::Ledger;
pub use vocab::{Broken, Heard, Order, Quantity, Refusal, Said, Terms};

use rigor_waist::Name;

/// The name this concept's unit goes by.
pub const LEDGER: Name = Name::lit("ledger");

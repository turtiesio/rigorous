//! Adapter: real transfers of value, as recorded in Bitcoin blocks.
//!
//! Each transaction spends named amounts from its inputs' owners and pays its outputs' owners.
//! The mapping into the model:
//!
//! * a signed input is a DELEGATION: the owner grants a settler the power to transfer from the
//!   owner's account, limited to exactly the input's amount and bound to this one transaction;
//! * the settler then moves the value to the outputs under those capabilities, and what is
//!   left over (the fee) to the treasury;
//! * new coin enters only from the treasury, which pays each block's reward.
//!
//! So real data exercises resource transfers, spending limits, purpose-bound capabilities and
//! conservation. Because the real network accepted every one of these transactions, the
//! institution must refuse none of them -- and must refuse a replay of any of them.
//!
//! Owners are pseudonymised at the boundary (an id is a digest of the output script).
//!
//! * `block`  blocks and transactions: parsed, and exported to a fixture
//! * `cast`  who is who on a ledger of transfers, and the constitution that founds it
//! * `error`  why blocks could not be read or settled
//! * `oracle`  independent bookkeeping of the same blocks, and the comparison with the world
//! * `settle`  settling transactions as transfers under budgets bound to a purpose

mod block;
mod cast;
mod error;
mod oracle;
mod settle;

pub use block::*;
pub use cast::*;
pub use error::*;
pub use oracle::*;
pub use settle::*;

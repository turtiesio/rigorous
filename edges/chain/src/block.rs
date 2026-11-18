//! Blocks and transactions: parsed, and exported to a fixture.

use crate::error::*;
use rigor_institution::model::{ActorId, Amount};
use rigor_waist::{digest_bytes, Name, NameError};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Input {
    pub owner: ActorId,
    pub amount: Amount,
    /// The transaction that created the coin being spent.
    pub from_tx: Name,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tx {
    pub id: Name,
    /// Whether this is the block's reward (new coin plus the block's fees).
    pub reward: bool,
    pub inputs: Vec<Input>,
    pub outputs: Vec<(ActorId, Amount)>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Block {
    pub height: u64,
    pub txs: Vec<Tx>,
}

pub(crate) fn short(hex: &str) -> Result<Name, NameError> {
    Name::new(&hex.chars().take(16).collect::<String>().to_lowercase())
}

/// Parse blocks. With `pseudonymise`, owners are given as output scripts and are replaced by a
/// digest; without it, they are already ids (the reduced form written by [`export`]).
///
/// Lines: `B <height>`, `T <txid>`, `C` (this transaction is the reward), `I <owner> <amount>
/// <creating txid>`, `O <owner> <amount>`.
///
/// # Errors
/// If a line is malformed.
pub fn parse(text: &str, pseudonymise: bool) -> Result<Vec<Block>, ChainError> {
    let owner = |s: &str| -> Result<ActorId, NameError> {
        if pseudonymise {
            Ok(ActorId(Name::new(&format!("a-{}", digest_bytes(s.as_bytes())))?))
        } else {
            Ok(ActorId(Name::new(s)?))
        }
    };
    let mut blocks: Vec<Block> = Vec::new();
    for (i, line) in text.lines().enumerate() {
        let bad = || ChainError::Malformed { line: i + 1 };
        let fields: Vec<&str> = line.split(' ').collect();
        let number = |s: &str| s.parse::<u64>().map_err(|_| bad());
        match fields.as_slice() {
            ["B", h] => blocks.push(Block { height: number(h)?, txs: Vec::new() }),
            ["T", id] => blocks.last_mut().ok_or_else(bad)?.txs.push(Tx { id: short(id)?, reward: false, inputs: Vec::new(), outputs: Vec::new() }),
            rest => {
                let tx = blocks.last_mut().and_then(|b| b.txs.last_mut()).ok_or_else(bad)?;
                match rest {
                    ["C"] => tx.reward = true,
                    ["I", who, amount, from] => tx.inputs.push(Input { owner: owner(who)?, amount: Amount(number(amount)?), from_tx: short(from)? }),
                    ["O", who, amount] => tx.outputs.push((owner(who)?, Amount(number(amount)?))),
                    _ => return Err(bad()),
                }
            }
        }
    }
    Ok(blocks)
}

/// The reduced, pseudonymous form: what [`parse`] reads back without pseudonymising again.
#[must_use]
pub fn export(blocks: &[Block]) -> String {
    let mut out = String::new();
    for b in blocks {
        out.push_str(&format!("B {}\n", b.height));
        for t in &b.txs {
            out.push_str(&format!("T {}\n", t.id));
            if t.reward {
                out.push_str("C\n");
            }
            for i in &t.inputs {
                out.push_str(&format!("I {} {} {}\n", i.owner, i.amount.0, i.from_tx));
            }
            for (who, amount) in &t.outputs {
                out.push_str(&format!("O {} {}\n", who, amount.0));
            }
        }
    }
    out
}

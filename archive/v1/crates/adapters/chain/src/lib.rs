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

use rigor_institution::model::{
    Act, ActionKind, ActorId, Amount, CapId, Charter, CharterName, Constitution, DenyReason, Event, Grant, Member, MemberKind, Memo, Resource, Scope, Transfer,
};
use rigor_institution::org::{org, standard_rules, Trusted};
use rigor_institution::world::{Error, World, CONSTITUTION};
use rigor_kernel::{digest_bytes, Name, NameError};
use std::collections::{BTreeMap, BTreeSet};

pub const SAT: Resource = Resource::lit("sat");
pub const TREASURY: ActorId = ActorId::lit("treasury");
pub const REGISTRAR: ActorId = ActorId::lit("registrar");
pub const SETTLER: ActorId = ActorId::lit("settler");
pub const ADMISSIONS: CharterName = CharterName::lit("admissions");

/// Why a block file could not be read or settled.
#[derive(Debug)]
pub enum ChainError {
    Malformed { line: usize },
    Name(NameError),
    World(Error),
}
impl From<Error> for ChainError {
    fn from(e: Error) -> Self {
        Self::World(e)
    }
}
impl From<NameError> for ChainError {
    fn from(e: NameError) -> Self {
        Self::Name(e)
    }
}

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

fn short(hex: &str) -> Result<Name, NameError> {
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

fn fee(t: &Tx) -> u64 {
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

/// What settlement met.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Stats {
    pub blocks: usize,
    pub transactions: usize,
    pub members_admitted: usize,
    /// Purpose-bound, limited capabilities granted by input owners.
    pub delegations: usize,
    pub transfers: usize,
    pub value_moved: u64,
    /// Attempts the institution refused although the real network had accepted the transaction.
    pub wrongly_refused: usize,
    /// Replays attempted after settlement (one per transaction with inputs)...
    pub replays_attempted: usize,
    /// ...and refused because the capability's budget was exhausted.
    pub replays_refused: usize,
    /// Attempts to use one transaction's capability for another purpose, refused.
    pub misuse_refused: usize,
}

/// A founded world whose accounts hold the window's opening balances.
///
/// # Errors
/// On a kernel fault.
pub fn found(blocks: &[Block]) -> Result<World, Error> {
    let m = |id: ActorId, kind| Member { id, kind, principal: None, role: None };
    let constitution = Constitution {
        members: vec![m(TREASURY, MemberKind::Org), m(REGISTRAR, MemberKind::Service), m(SETTLER, MemberKind::Service)],
        charters: vec![Charter { name: ADMISSIONS, holder: REGISTRAR, actions: std::iter::once(ActionKind::Admit).collect(), delegable: false }],
        balances: opening(blocks).into_iter().map(|(who, n)| (who, SAT, Amount(n))).collect(),
        rules: standard_rules(&Trusted { receipts: TREASURY, probes: TREASURY, tests: TREASURY, audits: TREASURY, acks: TREASURY }),
    };
    let mut w = World::new(Box::new(org(Vec::new())?));
    let _ = w.inject(&CONSTITUTION, Event::Genesis(Box::new(constitution)))?;
    Ok(w)
}

fn denied(entries: &[rigor_kernel::Entry<Event>]) -> Option<DenyReason> {
    entries.iter().find_map(|e| match &e.event {
        Event::Denied { reason, .. } => Some(reason.clone()),
        _ => None,
    })
}

/// Settles blocks into a world.
#[derive(Debug, Default)]
pub struct Settle {
    pub stats: Stats,
    known: BTreeSet<ActorId>,
}

impl Settle {
    fn admit(&mut self, w: &mut World, who: &ActorId) -> Result<(), Error> {
        if *who != TREASURY && self.known.insert(who.clone()) {
            let member = Member { id: who.clone(), kind: MemberKind::Human, principal: None, role: None };
            let entries = w.act_with(&REGISTRAR, &CapId::Charter(ADMISSIONS), Act::Admit(member))?;
            self.stats.wrongly_refused += usize::from(denied(&entries).is_some());
            self.stats.members_admitted += 1;
        }
        Ok(())
    }

    fn pay(&mut self, w: &mut World, (by, cap): (&ActorId, &CapId), from: &ActorId, to: &ActorId, amount: u64, purpose: &Name) -> Result<(), Error> {
        if amount == 0 {
            return Ok(());
        }
        let t = Transfer { from: from.clone(), to: to.clone(), resource: SAT, amount: Amount(amount), memo: Memo::Purpose(purpose.clone()) };
        let entries = w.act_with(by, cap, Act::Transfer(t))?;
        let moved = entries.iter().any(|e| matches!(e.event, Event::Transferred { .. }));
        self.stats.wrongly_refused += usize::from(!moved);
        self.stats.transfers += 1;
        self.stats.value_moved = self.stats.value_moved.saturating_add(amount);
        Ok(())
    }

    fn ordinary(&mut self, w: &mut World, t: &Tx) -> Result<(), Error> {
        // One delegation per owner: the sum of that owner's inputs, for this transaction only.
        let mut per_owner: BTreeMap<&ActorId, u64> = BTreeMap::new();
        for i in &t.inputs {
            *per_owner.entry(&i.owner).or_default() += i.amount.0;
        }
        let mut purse: Vec<(ActorId, CapId, u64)> = Vec::new();
        for (owner, amount) in per_owner {
            self.admit(w, owner)?;
            let grant = Grant {
                to: SETTLER,
                actions: std::iter::once(ActionKind::Transfer).collect(),
                scope: Scope { from: Some(owner.clone()), purpose: Some(t.id.clone()) },
                limits: std::iter::once((SAT, Amount(amount))).collect(),
                expires: None,
                delegable: false,
            };
            let entries = w.act_with(owner, &CapId::Own(owner.clone()), Act::Delegate(grant))?;
            match entries.iter().find_map(|e| match &e.event {
                Event::Delegated(c) => Some(c.id.clone()),
                _ => None,
            }) {
                Some(cap) => {
                    self.stats.delegations += 1;
                    purse.push((owner.clone(), cap, amount));
                }
                None => self.stats.wrongly_refused += 1,
            }
        }
        for (to, _) in &t.outputs {
            self.admit(w, to)?;
        }
        // Fill each output from the inputs in order; what remains is the fee.
        let mut source = 0usize;
        for (to, amount) in &t.outputs {
            let mut need = amount.0;
            while need > 0 {
                let Some((owner, cap, left)) = purse.get_mut(source) else { break };
                if *left == 0 {
                    source += 1;
                    continue;
                }
                let take = need.min(*left);
                let (owner, cap) = (owner.clone(), cap.clone());
                *left -= take;
                need -= take;
                self.pay(w, (&SETTLER, &cap), &owner, to, take, &t.id)?;
            }
            self.stats.wrongly_refused += usize::from(need > 0);
        }
        for (owner, cap, left) in &purse {
            self.pay(w, (&SETTLER, cap), owner, &TREASURY, *left, &t.id)?;
        }
        // Adversarial: the settler tries to take one unit more (a replay), and to use the
        // capability for something other than this transaction. Both must be refused.
        if let Some((owner, cap, _)) = purse.first() {
            let again = Transfer { from: owner.clone(), to: SETTLER, resource: SAT, amount: Amount(1), memo: Memo::Purpose(t.id.clone()) };
            self.stats.replays_attempted += 1;
            self.stats.replays_refused += usize::from(matches!(denied(&w.act_with(&SETTLER, cap, Act::Transfer(again))?), Some(DenyReason::BudgetExceeded(_))));
            let other = Transfer { from: owner.clone(), to: SETTLER, resource: SAT, amount: Amount(1), memo: Memo::None };
            self.stats.misuse_refused += usize::from(matches!(denied(&w.act_with(&SETTLER, cap, Act::Transfer(other))?), Some(DenyReason::OutOfScope)));
        }
        Ok(())
    }

    /// Settle one block: its ordinary transactions, then its reward (which includes their fees).
    ///
    /// # Errors
    /// On a kernel fault.
    pub fn block(&mut self, w: &mut World, b: &Block) -> Result<(), Error> {
        self.stats.blocks += 1;
        let _ = w.tick(b.height)?;
        for t in b.txs.iter().filter(|t| !t.reward) {
            self.stats.transactions += 1;
            self.ordinary(w, t)?;
        }
        for t in b.txs.iter().filter(|t| t.reward) {
            self.stats.transactions += 1;
            for (to, amount) in &t.outputs {
                self.admit(w, to)?;
                self.pay(w, (&TREASURY, &CapId::Own(TREASURY)), &TREASURY, to, amount.0, &t.id)?;
            }
        }
        Ok(())
    }
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

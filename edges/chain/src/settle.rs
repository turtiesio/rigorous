//! Settling transactions as transfers under budgets bound to a purpose.

use crate::block::*;
use crate::cast::*;
use rigor_institution::model::{Act, ActionKind, ActorId, Amount, CapId, DenyReason, Event, Grant, Member, MemberKind, Memo, Scope, Transfer};
use rigor_institution::world::{Error, World};
use rigor_waist::Name;
use std::collections::{BTreeMap, BTreeSet};

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

pub(crate) fn denied(entries: &[rigor_waist::Entry<Event>]) -> Option<DenyReason> {
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

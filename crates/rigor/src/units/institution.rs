//! Authority: the count-as gate.
//!
//! An actor's attempt is the brute event `Do { act, cap }`. It COUNTS AS the institutional
//! event `Ok { by, via, act }` iff the actor is a member holding a capability whose delegation
//! chain is rooted in the constitution, unrevoked and unexpired along its whole length, covers
//! the action and scope, and has budget. Otherwise the attempt is recorded as `Denied`.
//! This unit is the only one that may emit `Ok`.

use crate::kernel::{digest_of, Envelope, Leaf, Name, Path, Who};
use crate::model::LeafError;
use crate::model::{
    base_actions, Act, ActionKind, ActorId, Amount, AttenuationError, CapId, Capability, DenyReason, Event, Grant, Issuer, Kind, Member, Resource, Scope, Time,
};
use std::collections::{BTreeMap, BTreeSet};

pub const INST: Name = Name::lit("inst");

#[derive(Clone, Debug, Hash, Default)]
pub struct Institution {
    now: Time,
    founded: bool,
    members: BTreeMap<ActorId, Member>,
    caps: BTreeMap<CapId, Capability>,
    revoked: BTreeSet<CapId>,
    spent: BTreeMap<(CapId, Resource), Amount>,
}

/// `child` grants nothing that `parent` does not.
///
/// # Errors
/// The first way in which it does.
pub fn attenuates(child: &Capability, parent: &Capability) -> Result<(), AttenuationError> {
    if !child.actions.is_subset(&parent.actions) {
        return Err(AttenuationError::ActionsExceedParent);
    }
    if parent.scope.from.is_some() && child.scope.from != parent.scope.from {
        return Err(AttenuationError::ScopeWiderThanParent);
    }
    for (r, lim) in &parent.limits {
        if child.limits.get(r).is_none_or(|l| l > lim) {
            return Err(AttenuationError::LimitExceedsParent(r.clone()));
        }
    }
    if let Some(pe) = parent.expires {
        if child.expires.is_none_or(|ce| ce > pe) {
            return Err(AttenuationError::ExpiryExceedsParent);
        }
    }
    Ok(())
}

impl Institution {
    #[must_use]
    pub const fn caps(&self) -> &BTreeMap<CapId, Capability> {
        &self.caps
    }

    #[must_use]
    pub fn spent(&self, cap: &CapId, r: &Resource) -> Amount {
        self.spent.get(&(cap.clone(), r.clone())).copied().unwrap_or_default()
    }

    /// The capability and its ancestors, leaf first, ignoring validity.
    #[must_use]
    pub fn lineage(&self, id: &CapId) -> Vec<&Capability> {
        let mut out = Vec::new();
        let mut cur = self.caps.get(id);
        while let Some(c) = cur {
            out.push(c);
            cur = c.parent.as_ref().and_then(|p| self.caps.get(p));
        }
        out
    }

    /// The chain of a capability if every link is known, unrevoked and unexpired.
    ///
    /// # Errors
    /// The reason the chain is not valid now.
    pub fn chain(&self, id: &CapId) -> Result<Vec<&Capability>, DenyReason> {
        let mut out = Vec::new();
        let mut cur = Some(id.clone());
        while let Some(cid) = cur {
            let c = self.caps.get(&cid).ok_or_else(|| DenyReason::UnknownCap(cid.clone()))?;
            if self.revoked.contains(&cid) {
                return Err(DenyReason::Revoked(cid));
            }
            if c.expires.is_some_and(|e| self.now > e) {
                return Err(DenyReason::Expired(cid));
            }
            out.push(c);
            cur = c.parent.clone();
        }
        Ok(out)
    }

    #[must_use]
    pub fn valid(&self, id: &CapId) -> bool {
        self.chain(id).is_ok()
    }

    /// Invariant check: every delegated capability attenuates its parent and nothing is overspent.
    #[must_use]
    pub fn audit(&self) -> Vec<CapId> {
        let mut bad = Vec::new();
        for c in self.caps.values() {
            let parent_ok = c.parent.as_ref().is_none_or(|p| self.caps.get(p).is_some_and(|p| attenuates(c, p).is_ok()));
            let spend_ok = c.limits.iter().all(|(r, lim)| self.spent(&c.id, r) <= *lim);
            if !(parent_ok && spend_ok) {
                bad.push(c.id.clone());
            }
        }
        bad
    }

    fn root(&mut self, id: CapId, holder: ActorId, actions: BTreeSet<ActionKind>, scope: Scope, delegable: bool) {
        let cap = Capability {
            id: id.clone(),
            issuer: Issuer::Constitution,
            holder,
            actions,
            scope,
            limits: BTreeMap::new(),
            expires: None,
            delegable,
            parent: None,
        };
        let _ = self.caps.insert(id, cap);
    }

    fn attempt(&mut self, by: &ActorId, act: &Act, cap: Option<&CapId>) -> Result<Result<Event, DenyReason>, LeafError> {
        if !self.members.contains_key(by) {
            return Ok(Err(DenyReason::NotAMember));
        }
        let via = cap.cloned().unwrap_or_else(|| CapId::Base(by.clone()));
        let chain: Vec<Capability> = match self.chain(&via) {
            Ok(c) => c.into_iter().cloned().collect(),
            Err(e) => return Ok(Err(e)),
        };
        let Some(held) = chain.first() else {
            return Ok(Err(DenyReason::UnknownCap(via)));
        };
        if held.holder != *by {
            return Ok(Err(DenyReason::NotHolder));
        }
        if let Act::Delegate(grant) = act {
            return self.delegate(by, &via, held, grant);
        }
        if !held.actions.contains(&act.kind()) {
            return Ok(Err(DenyReason::ActionNotGranted));
        }
        if let Some(only) = &held.scope.from {
            match act {
                Act::Transfer(t) if t.from == *only => {}
                _ => return Ok(Err(DenyReason::OutOfScope)),
            }
        }
        match act {
            Act::Revoke { target } => {
                if !self.caps.contains_key(target) {
                    return Ok(Err(DenyReason::UnknownTarget));
                }
                if !self.lineage(target).iter().any(|c| c.issuer == Issuer::Member(by.clone())) {
                    return Ok(Err(DenyReason::NotAnIssuerOnTheChain));
                }
                let _ = self.revoked.insert(target.clone());
            }
            Act::Transfer(t) => {
                if t.amount == Amount(0) {
                    return Ok(Err(DenyReason::ZeroAmount));
                }
                for c in &chain {
                    if let Some(lim) = c.limits.get(&t.resource) {
                        if self.spent(&c.id, &t.resource).0.saturating_add(t.amount.0) > lim.0 {
                            return Ok(Err(DenyReason::BudgetExceeded(c.id.clone())));
                        }
                    }
                }
                for c in &chain {
                    if c.limits.contains_key(&t.resource) {
                        let s = self.spent.entry((c.id.clone(), t.resource.clone())).or_default();
                        s.0 = s.0.saturating_add(t.amount.0);
                    }
                }
            }
            _ => {}
        }
        Ok(Ok(Event::Ok { by: by.clone(), via, act: act.clone() }))
    }

    fn delegate(&mut self, by: &ActorId, via: &CapId, held: &Capability, g: &Grant) -> Result<Result<Event, DenyReason>, LeafError> {
        if !held.delegable {
            return Ok(Err(DenyReason::NotDelegable));
        }
        if !self.members.contains_key(&g.to) {
            return Ok(Err(DenyReason::DelegateeNotAMember));
        }
        if g.actions.is_empty() {
            return Ok(Err(DenyReason::EmptyDelegation));
        }
        let id = CapId::Delegated(digest_of(&(via, g))?);
        let new = Capability {
            id: id.clone(),
            issuer: Issuer::Member(by.clone()),
            holder: g.to.clone(),
            actions: g.actions.clone(),
            scope: g.scope.clone(),
            limits: g.limits.clone(),
            expires: g.expires,
            delegable: g.delegable,
            parent: Some(via.clone()),
        };
        if let Err(e) = attenuates(&new, held) {
            return Ok(Err(DenyReason::NotAnAttenuation(e)));
        }
        if self.caps.contains_key(&id) {
            return Ok(Err(DenyReason::DuplicateDelegation));
        }
        let _ = self.caps.insert(id, new.clone());
        Ok(Ok(Event::Delegated(new)))
    }
}

impl Leaf<Event> for Institution {
    fn name(&self) -> Name {
        INST
    }
    fn inputs(&self) -> BTreeSet<Kind> {
        [Kind::Genesis, Kind::Tick, Kind::Do, Kind::TransferFailed].into_iter().collect()
    }
    fn outputs(&self) -> BTreeSet<Kind> {
        [Kind::Ok, Kind::Delegated, Kind::Denied, Kind::Member].into_iter().collect()
    }
    fn on(&mut self, env: &Envelope<'_, Event>) -> Result<Vec<Event>, LeafError> {
        let mut out = Vec::new();
        match env.event {
            Event::Genesis(c) if !self.founded => {
                self.founded = true;
                for m in &c.members {
                    let _ = self.members.insert(m.id.clone(), m.clone());
                    self.root(CapId::Base(m.id.clone()), m.id.clone(), base_actions(), Scope::default(), false);
                    let own = Scope { from: Some(m.id.clone()) };
                    self.root(CapId::Own(m.id.clone()), m.id.clone(), [ActionKind::Transfer].into_iter().collect(), own, true);
                    out.push(Event::Member(m.clone()));
                }
                for ch in &c.charters {
                    self.root(CapId::Charter(ch.name.clone()), ch.holder.clone(), ch.actions.clone(), Scope::default(), false);
                }
            }
            Event::Tick(t) => self.now = self.now.max(*t),
            Event::TransferFailed { transfer, via, .. } => {
                let ids: Vec<CapId> = self.lineage(via).iter().map(|c| c.id.clone()).collect();
                for id in ids {
                    if let Some(s) = self.spent.get_mut(&(id, transfer.resource.clone())) {
                        s.0 = s.0.saturating_sub(transfer.amount.0);
                    }
                }
            }
            Event::Do { act, cap } => {
                let path: Path = match &env.who {
                    Who::Here(p) | Who::Outer(p) => p.clone(),
                };
                let verdict = match env.who.local() {
                    Some(n) => self.attempt(&ActorId(n.clone()), act, cap.as_ref())?,
                    None => Err(DenyReason::NotAMember),
                };
                out.push(match verdict {
                    Ok(ev) => ev,
                    Err(reason) => Event::Denied { by: path, act: act.clone(), via: cap.clone(), reason },
                });
            }
            _ => {}
        }
        Ok(out)
    }
}

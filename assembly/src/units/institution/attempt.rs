//! An attempt becomes an act only under a capability that covers it; otherwise it is refused with a reason.

use super::*;

impl Institution {
    pub(super) fn attempt(&mut self, by: &ActorId, act: &Act, cap: Option<&CapId>) -> Result<Result<Event, DenyReason>, LeafError> {
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
        if let Some(purpose) = &held.scope.purpose {
            match act {
                Act::Transfer(t) if t.memo == crate::model::Memo::Purpose(purpose.clone()) => {}
                _ => return Ok(Err(DenyReason::OutOfScope)),
            }
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
            Act::Admit(m) => {
                if self.members.contains_key(&m.id) {
                    return Ok(Err(DenyReason::AlreadyAMember));
                }
                self.enrol(m);
            }
            Act::Expel { member } => {
                if self.members.remove(member).is_none() {
                    return Ok(Err(DenyReason::UnknownMember));
                }
                // What they hold dies with their membership (every attempt needs a member);
                // what they handed on is revoked explicitly.
                let issued: Vec<CapId> = self.caps.values().filter(|c| c.issuer == Issuer::Member(member.clone())).map(|c| c.id.clone()).collect();
                self.revoked.extend(issued);
            }
            Act::SetRole { member, role } => match self.members.get_mut(member) {
                Some(m) => m.role = *role,
                None => return Ok(Err(DenyReason::UnknownMember)),
            },
            Act::Transfer(t) => {
                if t.amount == Amount(0) {
                    return Ok(Err(DenyReason::ZeroAmount));
                }
                for c in &chain {
                    match c.limits.get(&t.resource) {
                        Some(lim) => {
                            if self.spent(&c.id, &t.resource).0.saturating_add(t.amount.0) > lim.0 {
                                return Ok(Err(DenyReason::BudgetExceeded(c.id.clone())));
                            }
                        }
                        // A capability that is limited at all is a budget: it covers only the
                        // resources it names. "Fifty credits" is not also unlimited gold.
                        None if !c.limits.is_empty() => return Ok(Err(DenyReason::BudgetExceeded(c.id.clone()))),
                        None => {}
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
}

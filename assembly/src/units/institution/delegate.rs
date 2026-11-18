//! Delegation: a new capability that is an attenuation of the one it comes from.

use super::*;

impl Institution {
    pub(super) fn delegate(&mut self, by: &ActorId, via: &CapId, held: &Capability, g: &Grant) -> Result<Result<Event, DenyReason>, LeafError> {
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

//! Founding and enrolment: the root capabilities every member starts with.

use super::*;

impl Institution {
    pub(super) fn root(&mut self, id: CapId, holder: ActorId, actions: BTreeSet<ActionKind>, scope: Scope, delegable: bool) {
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

    pub(super) fn enrol(&mut self, m: &Member) {
        let _ = self.members.insert(m.id.clone(), m.clone());
        self.root(CapId::Base(m.id.clone()), m.id.clone(), base_actions(), Scope::default(), false);
        let own = Scope { from: Some(m.id.clone()), purpose: None };
        self.root(CapId::Own(m.id.clone()), m.id.clone(), std::iter::once(ActionKind::Transfer).collect(), own, true);
    }
}

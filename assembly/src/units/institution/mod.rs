//! Authority: the count-as gate.
//!
//! An actor's attempt is the brute event `Do { act, cap }`. It COUNTS AS the institutional
//! event `Ok { by, via, act }` iff the actor is a member holding a capability whose delegation
//! chain is rooted in the constitution, unrevoked and unexpired along its whole length, covers
//! the action and scope, and has budget. Otherwise the attempt is recorded as `Denied`.
//! This unit is the only one that may emit `Ok`.

use crate::model::LeafError;
use crate::model::{
    base_actions, Act, ActionKind, ActorId, Amount, AttenuationError, CapId, Capability, DenyReason, Event, Grant, Issuer, Kind, Member, Resource, Scope, Time,
};
use rigor_waist::{digest_of, Envelope, Leaf, Name, Path, Who};
use std::collections::{BTreeMap, BTreeSet};

mod attempt;
mod delegate;
mod found;
mod query;
mod step;

pub const INST: Name = Name::lit("inst");

#[derive(Clone, Debug, Hash, Default, serde::Serialize, serde::Deserialize)]
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
    if parent.scope.purpose.is_some() && child.scope.purpose != parent.scope.purpose {
        return Err(AttenuationError::PurposeDiffersFromParent);
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
}

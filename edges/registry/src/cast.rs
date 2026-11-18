//! Who is who at a package registry, and the constitution that founds it.

use rigor_institution::model::{ActionKind, ActorId, Charter, CharterName, Constitution, Event, Member, MemberKind, Method, Rank, Trust};
use rigor_institution::org::{org, standard_rules, Trusted};
use rigor_institution::world::{Error, World, CONSTITUTION};

pub const REGISTRY: ActorId = ActorId::lit("the-registry");

pub const REGISTRAR: ActorId = ActorId::lit("the-registrar");

pub const ADMISSIONS: CharterName = CharterName::lit("admissions");

/// A founded registry world.
///
/// # Errors
/// On a kernel fault.
pub fn found() -> Result<World, Error> {
    let m = |id: ActorId, kind| Member { id, kind, principal: None, role: None };
    let mut rules = standard_rules(&Trusted { receipts: REGISTRY, probes: REGISTRY, tests: REGISTRY, audits: REGISTRY, acks: REGISTRY });
    rules.evidence.ranks = std::iter::once((Method::Receipt, Rank(1))).collect();
    rules.evidence.trusted = std::iter::once((Method::Receipt, Trust::Only(std::iter::once(REGISTRY).collect()))).collect();
    let constitution = Constitution {
        members: vec![m(REGISTRY, MemberKind::Org), m(REGISTRAR, MemberKind::Service)],
        charters: vec![Charter { name: ADMISSIONS, holder: REGISTRAR, actions: std::iter::once(ActionKind::Admit).collect(), delegable: false }],
        balances: Vec::new(),
        rules,
    };
    let mut w = World::new(Box::new(org(Vec::new())?));
    let _ = w.inject(&CONSTITUTION, Event::Genesis(Box::new(constitution)))?;
    Ok(w)
}

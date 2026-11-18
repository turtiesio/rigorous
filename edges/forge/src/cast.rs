//! Who is who on a forge, and the constitution that founds it.

use rigor_institution::model::{ActionKind, ActorId, Charter, CharterName, Constitution, Event, Member, MemberKind, Method, Rank, Trust};
use rigor_institution::org::{org, standard_rules, Trusted};
use rigor_institution::world::{Error, World, CONSTITUTION};

pub const PROJECT: ActorId = ActorId::lit("the-forge-users");

pub const REGISTRAR: ActorId = ActorId::lit("the-registrar");

pub const FORGE: ActorId = ActorId::lit("the-forge");

pub const ADMISSIONS: CharterName = CharterName::lit("admissions");

/// A founded world for a forge.
///
/// # Errors
/// On a kernel fault.
pub fn found() -> Result<World, Error> {
    let m = |id: ActorId, kind| Member { id, kind, principal: None, role: None };
    let mut rules = standard_rules(&Trusted { receipts: FORGE, probes: FORGE, tests: FORGE, audits: FORGE, acks: FORGE });
    rules.evidence.ranks = [(Method::Ack, Rank(2)), (Method::Receipt, Rank(1))].into_iter().collect();
    rules.evidence.trusted = [(Method::Ack, Trust::Anyone), (Method::Receipt, Trust::Only(std::iter::once(FORGE).collect()))].into_iter().collect();
    let constitution = Constitution {
        members: vec![m(PROJECT, MemberKind::Org), m(REGISTRAR, MemberKind::Service), m(FORGE, MemberKind::Service)],
        charters: vec![Charter { name: ADMISSIONS, holder: REGISTRAR, actions: std::iter::once(ActionKind::Admit).collect(), delegable: false }],
        balances: Vec::new(),
        rules,
    };
    let mut w = World::new(Box::new(org(Vec::new())?));
    let _ = w.inject(&CONSTITUTION, Event::Genesis(Box::new(constitution)))?;
    Ok(w)
}

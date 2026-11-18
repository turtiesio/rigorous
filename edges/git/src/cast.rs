//! Who is who in a project read from a git history, and the constitution that founds it.

use rigor_institution::model::{ActionKind, ActorId, CapId, Charter, CharterName, Constitution, Event, Member, MemberKind, Method, Rank, Rules, Trust};
use rigor_institution::org::{org, standard_rules, Trusted};
use rigor_institution::world::{Error, World, CONSTITUTION};
use rigor_waist::Name;
use std::collections::BTreeSet;

pub const PROJECT: ActorId = ActorId::lit("project");

pub const REGISTRAR: ActorId = ActorId::lit("registrar");

pub const ADMISSIONS: CharterName = CharterName::lit("admissions");

pub const GOVERN_PROJECT: CharterName = CharterName::lit("govern-project");

pub const GOVERN_REGISTRAR: CharterName = CharterName::lit("govern-registrar");

pub const REFLECTOR: ActorId = ActorId::lit("reflector");

pub const REFLECT: CharterName = CharterName::lit("reflect");

/// A project's organisation: the institutional units plus a reflector that watches how each
/// method of evidence fares and may propose (never enact) a change of rank. It judges a method
/// after 200 acceptances and proposes when 5% or more were overturned; it judges the maintenance
/// period after 50 observed refutations.
///
/// # Errors
/// If the units cannot be composed.
pub fn project_org() -> Result<rigor_waist::Par<Event>, rigor_institution::model::Fault> {
    let reflector = rigor_institution::units::reflector::Reflector::new(REFLECTOR, CapId::Charter(REFLECT), 200, 5, 50);
    org(vec![Box::new(rigor_waist::Cell(reflector))])
}

/// Rules for a body of work: acknowledgements and reversals from anyone, a reversal outranking
/// an acknowledgement.
#[must_use]
pub fn rules() -> Rules {
    let mut r = standard_rules(&Trusted { receipts: PROJECT, probes: PROJECT, tests: PROJECT, audits: PROJECT, acks: PROJECT });
    r.evidence.ranks = [(Method::Ack, Rank(2)), (Method::Reversal, Rank(3))].into_iter().collect();
    r.evidence.trusted = [(Method::Ack, Trust::Anyone), (Method::Reversal, Trust::Anyone)].into_iter().collect();
    r
}

/// Found the project's institution in whatever scope the world currently addresses.
///
/// # Errors
/// On a kernel fault.
pub fn found_in(w: &mut World) -> Result<(), Error> {
    let _ = w.inject(&CONSTITUTION, Event::Genesis(Box::new(constitution())))?;
    Ok(())
}

/// A founded world with a project and a registrar who may admit members.
///
/// # Errors
/// On a kernel fault.
pub fn found() -> Result<World, Error> {
    let mut w = World::new(Box::new(project_org()?));
    found_in(&mut w)?;
    Ok(w)
}

/// Several projects as separately bounded organisations inside one world: each has its own
/// institution, members, rules and clock; nothing crosses a boundary.
///
/// # Errors
/// If the units cannot be composed.
pub fn federation(names: &[Name]) -> Result<World, Error> {
    let mut parts: Vec<Box<dyn rigor_waist::Unit<Event>>> = Vec::new();
    for n in names {
        parts.push(Box::new(rigor_waist::Entity::new(n.clone(), Box::new(project_org()?), BTreeSet::new(), BTreeSet::new())));
    }
    Ok(World::new(Box::new(rigor_waist::Par::new(parts, &rigor_institution::org::open_kinds())?)))
}

pub(crate) fn constitution() -> Constitution {
    let m = |id: ActorId, kind| Member { id, kind, principal: None, role: None };
    let govern: BTreeSet<ActionKind> = [ActionKind::Propose, ActionKind::Approve].into_iter().collect();
    Constitution {
        members: vec![m(PROJECT, MemberKind::Org), m(REGISTRAR, MemberKind::Service), m(REFLECTOR, MemberKind::Service)],
        charters: vec![
            Charter { name: ADMISSIONS, holder: REGISTRAR, actions: std::iter::once(ActionKind::Admit).collect(), delegable: true },
            Charter { name: GOVERN_PROJECT, holder: PROJECT, actions: govern.clone(), delegable: false },
            Charter { name: GOVERN_REGISTRAR, holder: REGISTRAR, actions: govern, delegable: false },
            Charter { name: REFLECT, holder: REFLECTOR, actions: std::iter::once(ActionKind::Propose).collect(), delegable: false },
        ],
        balances: Vec::new(),
        rules: rules(),
    }
}

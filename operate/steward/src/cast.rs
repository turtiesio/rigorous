//! Who is who in the project's own institution, and the constitution that founds it.

use rigor_institution::model::{ActorId, ArtifactId, Condition, Constitution, Event, Member, MemberKind, Offer, Prop, Tag, Time};
use rigor_institution::org::{org, standard_rules, Trusted};
use rigor_institution::world::{Error, World, CONSTITUTION};
use rigor_waist::Unit;

pub const PROJECT: ActorId = ActorId::lit("project");
pub const MAINTAINER: ActorId = ActorId::lit("maintainer");
pub const CI: ActorId = ActorId::lit("ci");
/// The person the project answers to. Decisions that are the owner's are never made by the system.
pub const OWNER: ActorId = ActorId::lit("owner");
/// A language model that drafts (answers, proposals). What it says is a claim, never evidence.
pub const RESEARCHER: ActorId = ActorId::lit("researcher");
/// A second, different language model that reviews what the first one drafted.
pub const REVIEWER: ActorId = ActorId::lit("reviewer");
/// Where money spent on consulting models goes.
pub const GATEWAY: ActorId = ActorId::lit("gateway");
/// Millionths of a US dollar.
pub const MICRO_USD: rigor_institution::model::Resource = rigor_institution::model::Resource::lit("micro-usd");
pub const OWNERSHIP: rigor_institution::model::CharterName = rigor_institution::model::CharterName::lit("ownership");

pub const SOURCES: ArtifactId = ArtifactId::lit("rust-sources");
pub const PROOFS: ArtifactId = ArtifactId::lit("lean-proofs");

pub(crate) fn root_unit() -> Result<Box<dyn Unit<Event>>, Error> {
    Ok(Box::new(org(Vec::new())?))
}

/// The project's constitution: who its founding members are and what they may do.
#[must_use]
pub fn constitution() -> Constitution {
    use rigor_institution::model::{ActionKind, Amount, Charter, Method, Trust};
    let m = |id: ActorId, kind| Member { id, kind, principal: None, role: None };
    let ai = |id: ActorId| Member { id, kind: MemberKind::Ai, principal: Some(OWNER), role: None };
    let mut rules = standard_rules(&Trusted { receipts: CI, probes: CI, tests: CI, audits: OWNER, acks: PROJECT });
    // An acknowledgement counts from the project, the owner, or the reviewing model -- never
    // from the drafting model, and (by the self-support rule) never from whoever made the claim.
    let _ = rules.evidence.trusted.insert(Method::Ack, Trust::Only([PROJECT, OWNER, REVIEWER].into_iter().collect()));
    Constitution {
        members: vec![
            m(PROJECT, MemberKind::Org),
            m(OWNER, MemberKind::Human),
            m(MAINTAINER, MemberKind::Human),
            m(CI, MemberKind::Service),
            ai(RESEARCHER),
            ai(REVIEWER),
            m(GATEWAY, MemberKind::Service),
        ],
        charters: vec![Charter {
            name: OWNERSHIP,
            holder: OWNER,
            actions: [ActionKind::Propose, ActionKind::Approve, ActionKind::Admit, ActionKind::Expel, ActionKind::SetRole].into_iter().collect(),
            delegable: false,
        }],
        // What the owner has set aside for the project to spend on consulting models.
        balances: vec![(OWNER, MICRO_USD, Amount(5_000_000))],
        rules,
    }
}

pub(crate) fn found_world(day: u64) -> Result<World, Error> {
    let mut w = World::new(root_unit()?);
    let _ = w.inject(&CONSTITUTION, Event::Genesis(Box::new(constitution())))?;
    let _ = w.tick(day)?;
    Ok(w)
}

pub(crate) fn undertaking(prop: Prop, deadline: Time, tag: Tag) -> Offer {
    Offer { creditor: PROJECT, ant: None, cons: Condition::Holds(prop), deadline, tags: std::iter::once(tag).collect(), parent: None, terms: None }
}

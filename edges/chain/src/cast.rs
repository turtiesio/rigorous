//! Who is who on a ledger of transfers, and the constitution that founds it.

use crate::block::*;
use crate::oracle::*;
use rigor_institution::model::{ActionKind, ActorId, Amount, Charter, CharterName, Constitution, Event, Member, MemberKind, Resource};
use rigor_institution::org::{org, standard_rules, Trusted};
use rigor_institution::world::{Error, World, CONSTITUTION};

pub const SAT: Resource = Resource::lit("sat");

pub const TREASURY: ActorId = ActorId::lit("treasury");

pub const REGISTRAR: ActorId = ActorId::lit("registrar");

pub const SETTLER: ActorId = ActorId::lit("settler");

pub const ADMISSIONS: CharterName = CharterName::lit("admissions");

/// A founded world whose accounts hold the window's opening balances.
///
/// # Errors
/// On a kernel fault.
pub fn found(blocks: &[Block]) -> Result<World, Error> {
    let m = |id: ActorId, kind| Member { id, kind, principal: None, role: None };
    let constitution = Constitution {
        members: vec![m(TREASURY, MemberKind::Org), m(REGISTRAR, MemberKind::Service), m(SETTLER, MemberKind::Service)],
        charters: vec![Charter { name: ADMISSIONS, holder: REGISTRAR, actions: std::iter::once(ActionKind::Admit).collect(), delegable: false }],
        balances: opening(blocks).into_iter().map(|(who, n)| (who, SAT, Amount(n))).collect(),
        rules: standard_rules(&Trusted { receipts: TREASURY, probes: TREASURY, tests: TREASURY, audits: TREASURY, acks: TREASURY }),
    };
    let mut w = World::new(Box::new(org(Vec::new())?));
    let _ = w.inject(&CONSTITUTION, Event::Genesis(Box::new(constitution)))?;
    Ok(w)
}

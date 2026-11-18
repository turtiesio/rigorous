//! Synthetic worlds for scaling measurements and locality tests.

use crate::actors::CREDIT;
use crate::kernel::{Entry, NameError};
use crate::model::{
    About, Act, ActorId, Amount, CommitmentId, Condition, Constitution, Event, Member, MemberKind, Method, Offer, Prop, Role, Tag, Task, TaskName, Time,
};
use crate::org::{org, standard_rules, Trusted};
use crate::scenario::attest;
use crate::world::{Error, World, CONSTITUTION};

pub const BOSS: ActorId = ActorId::lit("boss");
pub const WORKER: ActorId = ActorId::lit("w");
pub const DEPLOYER: ActorId = ActorId::lit("deployer");
pub const MONITOR: ActorId = ActorId::lit("monitor");
pub const AUDITOR: ActorId = ActorId::lit("auditor");

/// Why a synthetic world could not be built.
#[derive(Debug)]
pub enum SynthError {
    World(Error),
    Name(NameError),
}

impl From<Error> for SynthError {
    fn from(e: Error) -> Self {
        Self::World(e)
    }
}

/// The proposition "link `d` of task `i` is done".
///
/// # Errors
/// Never for in-range numbers; names are validated like any other.
pub fn link(i: usize, d: usize) -> Result<Prop, NameError> {
    let name = TaskName(crate::kernel::Name::new(&format!("t{i}"))?);
    let step = TaskName(crate::kernel::Name::new(&format!("s{d}"))?);
    Ok(Prop::Done(Task { name, step: Some(step) }))
}

/// `n` independent tasks, each a delegation chain of `chain` commitments, every link supported
/// by its own receipt and claimed by its debtor. Time is advanced past every deadline.
///
/// # Errors
/// On a kernel fault.
pub fn world_n(n: usize, chain: usize) -> Result<World, SynthError> {
    let m = |id: ActorId, kind, role| Member { id, kind, principal: None, role };
    let rules = standard_rules(&Trusted { receipts: DEPLOYER, probes: MONITOR, tests: MONITOR, audits: AUDITOR, acks: BOSS });
    let constitution = Constitution {
        members: vec![
            m(BOSS, MemberKind::Human, None),
            m(WORKER, MemberKind::Human, None),
            m(DEPLOYER, MemberKind::Service, None),
            m(MONITOR, MemberKind::Service, None),
            m(AUDITOR, MemberKind::Human, Some(Role::Auditor)),
        ],
        charters: Vec::new(),
        balances: vec![(BOSS, CREDIT, Amount(10))],
        rules,
    };
    let mut w = World::new(Box::new(org(Vec::new()).map_err(Error::Fault)?));
    let _ = w.inject(&CONSTITUTION, Event::Genesis(Box::new(constitution)))?;
    for i in 0..n {
        let mut parent: Option<CommitmentId> = None;
        for d in 0..chain {
            let p = link(i, d).map_err(SynthError::Name)?;
            let offer = Offer {
                creditor: BOSS,
                ant: None,
                cons: Condition::Holds(p.clone()),
                deadline: Time(5),
                tags: std::iter::once(Tag::Task).collect(),
                parent,
                terms: None,
            };
            parent = w.act(&WORKER, Act::Commit(offer))?.iter().find_map(|e| match &e.event {
                Event::NormCreated(c) => Some(c.id),
                _ => None,
            });
            let _ = w.act(&DEPLOYER, attest(About::Supports(p.clone()), Method::Receipt, "receipt"))?;
            let _ = w.act(&WORKER, Act::Claim { prop: p })?;
        }
    }
    let _ = w.tick(6)?;
    Ok(w)
}

/// The monitor refutes one link of one task.
///
/// # Errors
/// On a kernel fault.
pub fn refute(w: &mut World, i: usize, d: usize) -> Result<Vec<Entry<Event>>, SynthError> {
    let p = link(i, d).map_err(SynthError::Name)?;
    Ok(w.act(&MONITOR, attest(About::Rebuts(p), Method::Probe, "probe"))?)
}

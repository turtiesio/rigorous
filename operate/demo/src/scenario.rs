//! The end-to-end demonstrator.
//!
//! A human delegates a task and a budget to an AI agent. The agent subcontracts a service and
//! pays it. The service fails and says so; the agent reports success anyway, citing an
//! independent smoke test that passed. The claim is at first accepted. A monitor's probe later refutes it, and
//! the system works out the consequences.

use crate::actors::{service, Agent, Monolith, CREDIT};
use rigor_institution::model::Fault;
use rigor_institution::model::{
    About, Act, ActionKind, ActorId, Amendment, Amount, CapId, Charter, CharterName, CommitmentId, Condition, Constitution, DenyReason, Event, GovError, Grant,
    Member, MemberKind, Memo, Method, Offer, Prop, Request, Role, Rules, Scope, Status, Tag, Task, TaskName, Time, Transfer, Version,
};
use rigor_institution::org::{duty_to_verify, org, standard_rules, Trusted};
pub use rigor_institution::world::{attest, note};
use rigor_institution::world::{Error, World, CONSTITUTION};
use rigor_waist::{Cell, Entry, Unit};
use std::collections::{BTreeMap, BTreeSet};

pub const ALICE: ActorId = ActorId::lit("alice");
pub const AGENT: ActorId = ActorId::lit("agent");
pub const VENDOR: ActorId = ActorId::lit("vendor");
pub const DEPLOYER: ActorId = ActorId::lit("deployer");
pub const MONITOR: ActorId = ActorId::lit("monitor");
pub const CI: ActorId = ActorId::lit("ci");
pub const AUDITOR: ActorId = ActorId::lit("auditor");
pub const TASK1: TaskName = TaskName::lit("task1");
pub const GOV_ALICE: CharterName = CharterName::lit("gov-alice");
pub const GOV_AUDITOR: CharterName = CharterName::lit("gov-auditor");

/// How the cast behaves.
#[derive(Clone, Copy, Debug)]
pub struct Cast {
    /// Does the agent report what the service told it?
    pub honest_agent: bool,
    /// Does the service actually do the work?
    pub service_works: bool,
    /// Does the service tell the truth about failing?
    pub service_truthful: bool,
    /// Replace the two-part service by a single leaf with the same boundary behaviour.
    pub monolith: bool,
}

impl Default for Cast {
    fn default() -> Self {
        Self { honest_agent: false, service_works: false, service_truthful: true, monolith: false }
    }
}

#[must_use]
pub fn rules() -> Rules {
    standard_rules(&Trusted { receipts: DEPLOYER, probes: MONITOR, tests: CI, audits: AUDITOR, acks: ALICE })
}

#[must_use]
pub fn constitution(rules: Rules) -> Constitution {
    let m = |id: ActorId, kind| Member { id, kind, principal: None, role: None };
    let govern: BTreeSet<ActionKind> = [ActionKind::Propose, ActionKind::Approve].into_iter().collect();
    Constitution {
        members: vec![
            m(ALICE, MemberKind::Human),
            Member { id: AGENT, kind: MemberKind::Ai, principal: Some(VENDOR), role: None },
            m(VENDOR, MemberKind::Org),
            m(DEPLOYER, MemberKind::Service),
            m(MONITOR, MemberKind::Service),
            m(CI, MemberKind::Service),
            Member { id: AUDITOR, kind: MemberKind::Human, principal: None, role: Some(Role::Auditor) },
        ],
        charters: vec![
            Charter { name: GOV_ALICE, holder: ALICE, actions: govern.clone(), delegable: false },
            Charter { name: GOV_AUDITOR, holder: AUDITOR, actions: govern, delegable: false },
        ],
        balances: vec![(ALICE, CREDIT, Amount(100)), (DEPLOYER, CREDIT, Amount(40))],
        rules,
    }
}

/// The root unit of the scenario's world.
///
/// # Errors
/// If the units cannot be composed.
pub fn root(cast: Cast) -> Result<Box<dyn Unit<Event>>, Fault> {
    let agent: Box<dyn Unit<Event>> = Box::new(Cell(Agent::new(AGENT, DEPLOYER, Amount(30), cast.honest_agent)));
    let svc: Box<dyn Unit<Event>> =
        if cast.monolith { Box::new(Cell(Monolith { entity: DEPLOYER })) } else { Box::new(service(&DEPLOYER, cast.service_works, cast.service_truthful)?) };
    Ok(Box::new(org(vec![agent, svc])?))
}

/// A founded world, before anyone has acted.
///
/// # Errors
/// On a kernel fault (including an invalid constitution).
pub fn world(cast: Cast, rules: Rules) -> Result<World, Error> {
    let mut w = World::new(root(cast)?);
    let _ = w.inject(&CONSTITUTION, Event::Genesis(Box::new(constitution(rules))))?;
    Ok(w)
}

#[must_use]
pub fn pay(from: ActorId, to: ActorId, amount: u64) -> Act {
    Act::Transfer(Transfer { from, to, resource: CREDIT, amount: Amount(amount), memo: Memo::None })
}

#[must_use]
pub fn budget(to: ActorId, from: ActorId, limit: u64, delegable: bool) -> Act {
    Act::Delegate(Grant {
        to,
        actions: std::iter::once(ActionKind::Transfer).collect(),
        scope: Scope { from: Some(from), purpose: None },
        limits: std::iter::once((CREDIT, Amount(limit))).collect(),
        expires: Some(Time(20)),
        delegable,
    })
}

/// The reason an attempt was denied, if the entries contain a denial.
#[must_use]
pub fn denial(entries: &[Entry<Event>]) -> Option<DenyReason> {
    entries.iter().find_map(|e| match &e.event {
        Event::Denied { reason, .. } => Some(reason.clone()),
        _ => None,
    })
}

/// The capability created, if the entries contain a delegation.
#[must_use]
pub fn delegated(entries: &[Entry<Event>]) -> Option<CapId> {
    entries.iter().find_map(|e| match &e.event {
        Event::Delegated(c) => Some(c.id.clone()),
        _ => None,
    })
}

/// What happened, for assertions and for the narrated report.
#[derive(Debug)]
pub struct Outcome {
    pub cap: CapId,
    pub subcap: CapId,
    /// Alice's conditional promise to pay; the agent's task commitment; the service's.
    pub c0: CommitmentId,
    pub c1: CommitmentId,
    pub c2: CommitmentId,
    pub over_delegation: DenyReason,
    pub overspend: DenyReason,
    pub revoked_use: DenyReason,
    pub revoked_sub_use: DenyReason,
    pub before: BTreeMap<CommitmentId, Status>,
    pub after: BTreeMap<CommitmentId, Status>,
    /// Everything the single probe event caused.
    pub refutation: Vec<Entry<Event>>,
    pub entrenched: Option<GovError>,
    pub enacted: Option<Version>,
}

fn task1() -> Task {
    Task::whole(TASK1)
}

/// Run the story. `amend` includes the governance step; `say` receives the narration.
///
/// # Errors
/// On a kernel fault, or if the story does not unfold as scripted.
pub fn run(cast: Cast, amend: bool, mut say: impl FnMut(&str)) -> Result<(World, Outcome), Error> {
    let mut w = world(cast, rules())?;
    let done = Prop::Done(task1());
    let deployed = Prop::Done(task1().step(crate::actors::DEPLOY));

    say("1. Alice delegates a budget of 50 credits to the agent (attenuated from her ownership).");
    let cap = delegated(&w.act_with(&ALICE, &CapId::Own(ALICE), budget(AGENT, ALICE, 50, true))?).ok_or(Error::Expected("delegation to the agent"))?;

    say("2. Alice promises to pay 20 once the task is accepted as done, and requests the task.");
    let _ = w.act(
        &ALICE,
        Act::Commit(Offer {
            creditor: AGENT,
            ant: Some(Condition::Holds(done.clone())),
            cons: Condition::Paid { from: ALICE, to: AGENT, resource: CREDIT, at_least: Amount(20) },
            deadline: Time(8),
            tags: std::iter::once(Tag::Payment).collect(),
            parent: None,
            terms: None,
        }),
    )?;
    let _ = w.act(&ALICE, Act::Request(Request { to: AGENT, task: task1(), deadline: Time(5), parent: None, terms: None }))?;
    // An independent smoke test passes: the first evidence that the deployment happened.
    let _ = w.act(&CI, attest(About::Supports(deployed.clone()), Method::Test, "smoke test of the deployment passed"))?;
    let c0 = w.one(&ALICE, Tag::Payment)?.commitment.id;
    let c1 = w.one(&AGENT, Tag::Task)?.commitment.id;
    let c2 = w.one(&DEPLOYER, Tag::Service)?.commitment.id;

    say("3. The agent tries to hand the service more authority than it has, then stays within it.");
    let over_delegation = denial(&w.act_with(&AGENT, &cap, budget(DEPLOYER, ALICE, 80, false))?).ok_or(Error::Expected("over-delegation denied"))?;
    let subcap = delegated(&w.act_with(&AGENT, &cap, budget(DEPLOYER, ALICE, 10, false))?).ok_or(Error::Expected("sub-delegation"))?;
    let overspend = denial(&w.act_with(&AGENT, &cap, pay(ALICE, AGENT, 30))?).ok_or(Error::Expected("overspend denied"))?;
    for t in 1..=5 {
        let _ = w.tick(t)?;
    }
    let before = w.statuses()?;

    say(if cast.service_works {
        "4. t=6: the monitor probes the deployment and confirms it."
    } else {
        "4. t=6: the monitor probes the deployment and finds nothing there."
    });
    let _ = w.tick(6)?;
    let about = if cast.service_works { About::Supports(deployed) } else { About::Rebuts(deployed) };
    let refutation = w.act(&MONITOR, attest(about, Method::Probe, "GET /version"))?;
    let after = w.statuses()?;

    say("5. Alice revokes the agent's capability; the sub-delegated one dies with it.");
    let _ = w.act(&ALICE, Act::Revoke { target: cap.clone() })?;
    let revoked_use = denial(&w.act_with(&AGENT, &cap, pay(ALICE, DEPLOYER, 1))?).ok_or(Error::Expected("revoked capability denied"))?;
    let revoked_sub_use = denial(&w.act_with(&DEPLOYER, &subcap, pay(ALICE, DEPLOYER, 1))?).ok_or(Error::Expected("revoked sub-capability denied"))?;

    let (mut entrenched, mut enacted) = (None, None);
    if amend {
        say("6. t=7: governance. An entrenched rule cannot be changed; a valid amendment needs a quorum of 2.");
        let _ = w.tick(7)?;
        let (ga, gu) = (CapId::Charter(GOV_ALICE), CapId::Charter(GOV_AUDITOR));
        entrenched = w.act_with(&ALICE, &ga, Act::Propose { amendments: vec![Amendment::SetSelfSupport(true)] })?.iter().find_map(|e| match &e.event {
            Event::GovRejected { reason, .. } => Some(reason.clone()),
            _ => None,
        });
        let proposed = w.act_with(&ALICE, &ga, Act::Propose { amendments: vec![Amendment::AddResponsibilityRules(duty_to_verify())] })?;
        let id = proposed
            .iter()
            .find_map(|e| match &e.event {
                Event::Proposed { id, .. } => Some(*id),
                _ => None,
            })
            .ok_or(Error::Expected("proposal accepted for voting"))?;
        // Proposing is not approving: both holders of the power must approve.
        let _ = w.act_with(&ALICE, &ga, Act::Approve { id })?;
        enacted = w.act_with(&AUDITOR, &gu, Act::Approve { id })?.iter().find_map(|e| match &e.event {
            Event::RulesChanged(rc) => Some(rc.version),
            _ => None,
        });
    }

    say("7. t=8: remediation is evidenced by the monitor, the audit acknowledged by Alice.");
    let _ = w.tick(8)?;
    let props = |w: &World, tag| -> Result<Vec<Prop>, Error> {
        Ok(w.tagged(tag)?
            .into_iter()
            .filter_map(|r| match &r.terms().cons {
                Condition::Holds(p) => Some(p.clone()),
                Condition::Unrefuted(_) | Condition::Paid { .. } => None,
            })
            .collect())
    };
    for p in props(&w, Tag::Remediation)? {
        let _ = w.act(&MONITOR, attest(About::Supports(p), Method::Probe, "v2 live"))?;
    }
    for p in props(&w, Tag::Audit)? {
        let _ = w.act(&AUDITOR, Act::Claim { prop: p.clone() })?;
        let _ = w.act(&ALICE, attest(About::Supports(p), Method::Ack, "audit report received"))?;
    }
    let _ = w.tick(9)?;
    Ok((w, Outcome { cap, subcap, c0, c1, c2, over_delegation, overspend, revoked_use, revoked_sub_use, before, after, refutation, entrenched, enacted }))
}

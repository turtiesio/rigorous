//! Behavioural units: an AI agent, and a service that is itself a composite.
//!
//! They are ordinary units. Nothing in the kernel or the institution knows what kind of thing
//! they are; that difference lives in member attributes which only the rules read.

use rigor_institution::model::{
    About, Act, ActorId, Amount, Attestation, CapId, Commitment, Condition, Content, Event, EvidenceId, Job, Kind, Memo, Method, Offer, Prop, Request,
    Resource, Source, Tag, Task, TaskName, Terms, Time, Transfer,
};
use rigor_institution::model::{Fault, LeafError};
use rigor_institution::org::open_kinds;
use rigor_kernel::{Cell, Entity, Envelope, Leaf, Name, Par};
use std::collections::{BTreeMap, BTreeSet};

pub const CREDIT: Resource = Resource::lit("credit");
pub const DEPLOY: TaskName = TaskName::lit("deploy");

fn act(act: Act) -> Event {
    Event::Do { act, cap: None }
}

/// Accepts tasks, subcontracts a service, pays from a delegated budget, and reports.
/// With `honest == false` it reports success whatever the service said.
#[derive(Clone, Debug, Hash, serde::Serialize, serde::Deserialize)]
pub struct Agent {
    me: ActorId,
    service: ActorId,
    price: Amount,
    honest: bool,
    cap: Option<CapId>,
    payer: Option<ActorId>,
    tasks: BTreeMap<Task, (ActorId, Time)>,
    subs: BTreeMap<Task, Task>,
    receipts: BTreeMap<Prop, EvidenceId>,
    signals: BTreeMap<Prop, bool>,
    reported: BTreeSet<Task>,
}

impl Agent {
    #[must_use]
    pub const fn new(me: ActorId, service: ActorId, price: Amount, honest: bool) -> Self {
        Self {
            me,
            service,
            price,
            honest,
            cap: None,
            payer: None,
            tasks: BTreeMap::new(),
            subs: BTreeMap::new(),
            receipts: BTreeMap::new(),
            signals: BTreeMap::new(),
            reported: BTreeSet::new(),
        }
    }

    /// Report on a task once both the service's signal and something to cite have arrived
    /// (they can arrive in either order).
    fn report(&mut self, sub_prop: &Prop, out: &mut Vec<Event>) {
        let Prop::Done(sub) = sub_prop else { return };
        let (Some(task), Some(ok)) = (self.subs.get(sub).cloned(), self.signals.get(sub_prop).copied()) else { return };
        if self.reported.contains(&task) {
            return;
        }
        let done = Prop::Done(task.clone());
        if ok || !self.honest {
            let Some(receipt) = self.receipts.get(sub_prop).copied() else { return };
            let _ = self.reported.insert(task);
            out.push(act(Act::Claim { prop: done.clone() }));
            out.push(act(Act::Attest(Attestation {
                about: About::Supports(done),
                method: Method::Inference,
                derived_from: std::iter::once(receipt).collect(),
                artifacts: BTreeMap::new(),
                content: Content::Note("subtask reported complete".to_owned()),
            })));
        } else if let Some((creditor, _)) = self.tasks.get(&task) {
            out.push(Event::Signal { to: creditor.clone(), prop: done, value: false });
            let _ = self.reported.insert(task);
        }
    }

    fn subcontract(&mut self, c: &Commitment, out: &mut Vec<Event>) {
        let Condition::Holds(Prop::Done(task)) = &c.terms.cons else { return };
        if c.terms.debtor != self.me || c.source != Source::Act || !self.tasks.contains_key(task) {
            return;
        }
        let sub = task.step(DEPLOY);
        let _ = self.subs.insert(sub.clone(), task.clone());
        let terms = self.payer.as_ref().map(|payer| Terms { price: self.price, resource: CREDIT, payer: payer.clone() });
        out.push(act(Act::Request(Request { to: self.service.clone(), task: sub.clone(), deadline: c.terms.deadline.minus(2), parent: Some(c.id), terms })));
        if let (Some(cap), Some(payer)) = (&self.cap, &self.payer) {
            let transfer = Transfer { from: payer.clone(), to: self.service.clone(), resource: CREDIT, amount: self.price, memo: Memo::Payment(sub) };
            out.push(Event::Do { act: Act::Transfer(transfer), cap: Some(cap.clone()) });
        }
    }
}

impl Leaf<Event> for Agent {
    fn name(&self) -> Name {
        self.me.0.clone()
    }
    fn inputs(&self) -> BTreeSet<Kind> {
        [Kind::Ok, Kind::Delegated, Kind::NormCreated, Kind::Signal, Kind::EvidenceRecorded].into_iter().collect()
    }
    fn outputs(&self) -> BTreeSet<Kind> {
        [Kind::Do, Kind::Signal].into_iter().collect()
    }
    fn on(&mut self, env: &Envelope<'_, Event>) -> Result<Vec<Event>, LeafError> {
        let mut out = Vec::new();
        match env.event {
            Event::Delegated(cap) if cap.holder == self.me && cap.actions.contains(&rigor_institution::model::ActionKind::Transfer) => {
                self.cap = Some(cap.id.clone());
                self.payer.clone_from(&cap.scope.from);
            }
            Event::Ok { by, act: Act::Request(r), .. } if r.to == self.me => {
                let _ = self.tasks.insert(r.task.clone(), (by.clone(), r.deadline));
                out.push(act(Act::Commit(Offer {
                    creditor: by.clone(),
                    ant: None,
                    cons: Condition::Holds(Prop::Done(r.task.clone())),
                    deadline: r.deadline,
                    tags: std::iter::once(Tag::Task).collect(),
                    parent: None,
                    terms: None,
                })));
            }
            Event::NormCreated(c) => self.subcontract(c, &mut out),
            Event::EvidenceRecorded(e) if e.source != self.me => {
                if let About::Supports(p) = &e.about {
                    let _ = self.receipts.entry(p.clone()).or_insert(e.id);
                    self.report(p, &mut out);
                }
            }
            Event::Signal { to, prop, value } if *to == self.me => {
                let _ = self.signals.insert(prop.clone(), *value);
                self.report(prop, &mut out);
            }
            _ => {}
        }
        Ok(out)
    }
}

pub const API: Name = Name::lit("api");
pub const WORKER: Name = Name::lit("worker");

/// Front of the service: accepts jobs, issues receipts, reports outcomes, pays refunds.
#[derive(Clone, Debug, Hash, serde::Serialize, serde::Deserialize)]
pub struct ServiceApi {
    entity: ActorId,
    truthful: bool,
}

impl Leaf<Event> for ServiceApi {
    fn name(&self) -> Name {
        API
    }
    fn inputs(&self) -> BTreeSet<Kind> {
        [Kind::Ok, Kind::JobDone, Kind::JobFailed, Kind::NormCreated].into_iter().collect()
    }
    fn outputs(&self) -> BTreeSet<Kind> {
        [Kind::Do, Kind::Signal, Kind::JobQueued].into_iter().collect()
    }
    fn on(&mut self, env: &Envelope<'_, Event>) -> Result<Vec<Event>, LeafError> {
        let mut out = Vec::new();
        match env.event {
            Event::Ok { by, act: Act::Request(r), .. } if r.to == self.entity => {
                let prop = Prop::Done(r.task.clone());
                out.push(act(Act::Commit(Offer {
                    creditor: by.clone(),
                    ant: None,
                    cons: Condition::Holds(prop.clone()),
                    deadline: r.deadline,
                    tags: [Tag::Paid, Tag::Service].into_iter().collect(),
                    parent: r.parent,
                    terms: r.terms.clone(),
                })));
                out.push(Event::JobQueued(Job { task: r.task.clone(), client: by.clone() }));
                out.push(act(Act::Attest(Attestation {
                    about: About::Supports(prop),
                    method: Method::Receipt,
                    derived_from: BTreeSet::new(),
                    artifacts: BTreeMap::new(),
                    content: Content::Note("job accepted".to_owned()),
                })));
            }
            Event::JobDone(j) => out.push(Event::Signal { to: j.client.clone(), prop: Prop::Done(j.task.clone()), value: true }),
            Event::JobFailed(j) => out.push(Event::Signal { to: j.client.clone(), prop: Prop::Done(j.task.clone()), value: !self.truthful }),
            Event::NormCreated(c) if c.terms.debtor == self.entity && c.terms.tags.contains(&Tag::Refund) => {
                if let Condition::Paid { to, resource, at_least, .. } = &c.terms.cons {
                    let transfer =
                        Transfer { from: self.entity.clone(), to: to.clone(), resource: resource.clone(), amount: *at_least, memo: Memo::Refund(c.id) };
                    out.push(Event::Do { act: Act::Transfer(transfer), cap: Some(CapId::Own(self.entity.clone())) });
                }
            }
            _ => {}
        }
        Ok(out)
    }
}

/// Back of the service: where the work really happens, or does not.
#[derive(Clone, Debug, Hash, serde::Serialize, serde::Deserialize)]
pub struct Worker {
    works: bool,
}

impl Leaf<Event> for Worker {
    fn name(&self) -> Name {
        WORKER
    }
    fn inputs(&self) -> BTreeSet<Kind> {
        std::iter::once(Kind::JobQueued).collect()
    }
    fn outputs(&self) -> BTreeSet<Kind> {
        [Kind::JobDone, Kind::JobFailed].into_iter().collect()
    }
    fn on(&mut self, env: &Envelope<'_, Event>) -> Result<Vec<Event>, LeafError> {
        Ok(match env.event {
            Event::JobQueued(j) if self.works => vec![Event::JobDone(j.clone())],
            Event::JobQueued(j) => vec![Event::JobFailed(j.clone())],
            _ => Vec::new(),
        })
    }
}

/// A two-part system that the outside sees as one actor.
///
/// # Errors
/// If the parts cannot be composed.
pub fn service(name: &ActorId, works: bool, truthful: bool) -> Result<Entity<Event>, Fault> {
    let inner = Par::new(vec![Box::new(Cell(ServiceApi { entity: name.clone(), truthful })), Box::new(Cell(Worker { works }))], &open_kinds())?;
    Ok(Entity::new(name.0.clone(), Box::new(inner), [Kind::Ok, Kind::NormCreated].into_iter().collect(), [Kind::Do, Kind::Signal].into_iter().collect()))
}

/// The same boundary behaviour as [`service`] with a failing worker, as a single stateless leaf.
/// Used to show that a composite can be substituted by an atomic unit.
#[derive(Clone, Debug, Hash, serde::Serialize, serde::Deserialize)]
pub struct Monolith {
    pub entity: ActorId,
}

impl Leaf<Event> for Monolith {
    fn name(&self) -> Name {
        self.entity.0.clone()
    }
    fn inputs(&self) -> BTreeSet<Kind> {
        [Kind::Ok, Kind::NormCreated].into_iter().collect()
    }
    fn outputs(&self) -> BTreeSet<Kind> {
        [Kind::Do, Kind::Signal].into_iter().collect()
    }
    fn on(&mut self, env: &Envelope<'_, Event>) -> Result<Vec<Event>, LeafError> {
        let mut api = ServiceApi { entity: self.entity.clone(), truthful: true };
        let mut out = Vec::new();
        for e in api.on(env)? {
            match e {
                Event::JobQueued(j) => out.push(Event::Signal { to: j.client, prop: Prop::Done(j.task), value: false }),
                other => out.push(other),
            }
        }
        Ok(out)
    }
}

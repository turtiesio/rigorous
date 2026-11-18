//! An envoy: how one organisation acts as a member of another.
//!
//! Inside a federated boundary sits a whole organisation with its own institution, members,
//! rules and evidence. The envoy is one of its members. It is the only unit that sees what
//! happens outside (marked as foreign), and the only one that can address the outside.
//!
//! * When the outer institution holds the organisation to a duty, the envoy turns it into a
//!   request to one of its own members, under the INNER institution.
//! * When the inner commitment is satisfied by the inner institution's own standards, the envoy
//!   CLAIMS outward that the duty is done. A claim is all it is: the outer institution still
//!   wants evidence it trusts. Inner evidence does not cross the boundary.
//! * When the inner commitment is violated, the envoy says so outward.
//!
//! Identities are translated here and nowhere else: outside, there is one actor (the
//! organisation); inside, its members.

use crate::model::{Act, ActorId, CommitmentId, Condition, Event, Kind, LeafError, Prop, Request, Status, Task, TaskName, Time};
use rigor_kernel::{Envelope, Leaf, Name};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
struct Job {
    /// What the outer institution wants: the proposition of the outer commitment.
    wanted: Prop,
    /// Who it is owed to, outside.
    outer_creditor: ActorId,
    reported: bool,
}

#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct Envoy {
    /// The envoy's own name as a member of the inner organisation.
    me: ActorId,
    /// The organisation's name as a member of the outer institution.
    outside_as: ActorId,
    /// The inner member the work is given to.
    worker: ActorId,
    jobs: BTreeMap<Task, Job>,
    inner: BTreeMap<CommitmentId, Task>,
}

impl Envoy {
    #[must_use]
    pub const fn new(me: ActorId, outside_as: ActorId, worker: ActorId) -> Self {
        Self { me, outside_as, worker, jobs: BTreeMap::new(), inner: BTreeMap::new() }
    }

    /// The inner task that stands for an outer commitment.
    ///
    /// # Errors
    /// Never for a well-formed id; names are validated like any other.
    pub fn task_for(outer: CommitmentId) -> Result<Task, rigor_kernel::NameError> {
        // The whole digest, not its shortened display form: two duties must not share a task.
        Ok(Task::whole(TaskName(Name::new(&format!("job-{:016x}", outer.0 .0))?)))
    }
}

impl Leaf<Event> for Envoy {
    fn name(&self) -> Name {
        self.me.0.clone()
    }
    fn inputs(&self) -> BTreeSet<Kind> {
        [Kind::Foreign, Kind::NormCreated, Kind::NormStatus].into_iter().collect()
    }
    fn outputs(&self) -> BTreeSet<Kind> {
        [Kind::Do, Kind::Tick, Kind::Abroad].into_iter().collect()
    }
    fn on(&mut self, env: &Envelope<'_, Event>) -> Result<Vec<Event>, LeafError> {
        let mut out = Vec::new();
        match env.event {
            Event::Foreign(outer) => match outer.as_ref() {
                // The outer clock is the inner clock.
                Event::Tick(t) => out.push(Event::Tick(*t)),
                Event::NormCreated(c) if c.terms.debtor == self.outside_as => {
                    if let (Condition::Holds(wanted), Ok(task)) = (&c.terms.cons, Self::task_for(c.id)) {
                        let job = Job { wanted: wanted.clone(), outer_creditor: c.terms.creditor.clone(), reported: false };
                        let _ = self.jobs.insert(task.clone(), job);
                        let deadline = Time(c.terms.deadline.0.saturating_sub(1));
                        out.push(Event::Do { act: Act::Request(Request { to: self.worker.clone(), task, deadline, parent: None, terms: None }), cap: None });
                    }
                }
                _ => {}
            },
            // An inner member has undertaken one of the jobs, to the envoy.
            // Only the member the work was given to can take it on: nobody else inside can speak
            // for the job by committing to it and walking away.
            Event::NormCreated(c) if c.terms.creditor == self.me && c.terms.debtor == self.worker => {
                if let Condition::Holds(Prop::Done(task)) = &c.terms.cons {
                    if self.jobs.contains_key(task) {
                        let _ = self.inner.insert(c.id, task.clone());
                    }
                }
            }
            Event::NormStatus { id, status, .. } => {
                let job = self.inner.get(id).and_then(|task| self.jobs.get_mut(task)).filter(|j| !j.reported);
                if let (Some(job), Status::Satisfied | Status::Violated) = (job, status) {
                    job.reported = true;
                    let done = *status == Status::Satisfied;
                    if done {
                        out.push(Event::Abroad(Box::new(Event::Do { act: Act::Claim { prop: job.wanted.clone() }, cap: None })));
                    }
                    out.push(Event::Abroad(Box::new(Event::Signal { to: job.outer_creditor.clone(), prop: job.wanted.clone(), value: done })));
                }
            }
            _ => {}
        }
        Ok(out)
    }
}

//! A typed facade over a runtime that hosts one organisation: acting, and reading state.

use crate::kernel::{Entry, Name, Runtime, Unit};
use crate::model::Fault;
use crate::model::{Act, ActorId, CapId, Event, Time};
use crate::units::{
    adjudicator::{Adjudicator, ADJ},
    epistemic::{Epistemic, EPI},
    governance::{Governance, GOV},
    institution::{Institution, INST},
    ledger::{Ledger, LEDGER},
    norms::{Norms, NORMS},
};

/// Why an operation on a world failed.
#[derive(Debug)]
pub enum Error {
    Fault(Fault),
    /// The world does not contain the named unit.
    Missing(Name),
    /// Something a scripted scenario relied on did not happen.
    Expected(&'static str),
}

impl From<Fault> for Error {
    fn from(f: Fault) -> Self {
        Self::Fault(f)
    }
}

pub const CLOCK: Name = Name::lit("clock");
pub const CONSTITUTION: Name = Name::lit("constitution");

#[derive(Debug)]
pub struct World {
    pub rt: Runtime<Event>,
}

impl World {
    #[must_use]
    pub fn new(root: Box<dyn Unit<Event>>) -> Self {
        Self { rt: Runtime::new(root) }
    }

    /// # Errors
    /// On a kernel fault.
    pub fn inject(&mut self, actor: &Name, event: Event) -> Result<Vec<Entry<Event>>, Error> {
        Ok(self.rt.inject(actor, event)?)
    }

    /// An actor attempts an act under its membership capability.
    ///
    /// # Errors
    /// On a kernel fault. A refusal is not an error: it is a `Denied` entry.
    pub fn act(&mut self, actor: &ActorId, act: Act) -> Result<Vec<Entry<Event>>, Error> {
        self.inject(&actor.0, Event::Do { act, cap: None })
    }

    /// An actor attempts an act under a specific capability.
    ///
    /// # Errors
    /// On a kernel fault.
    pub fn act_with(&mut self, actor: &ActorId, cap: &CapId, act: Act) -> Result<Vec<Entry<Event>>, Error> {
        self.inject(&actor.0, Event::Do { act, cap: Some(cap.clone()) })
    }

    /// # Errors
    /// On a kernel fault.
    pub fn tick(&mut self, t: u64) -> Result<Vec<Entry<Event>>, Error> {
        self.inject(&CLOCK, Event::Tick(Time(t)))
    }

    #[must_use]
    pub fn fork(&self) -> Self {
        Self { rt: self.rt.fork() }
    }

    fn leaf<T: 'static>(&self, name: Name) -> Result<&T, Error> {
        self.rt.leaf::<T>(std::slice::from_ref(&name)).ok_or(Error::Missing(name))
    }

    /// # Errors
    /// If the world has no such unit.
    pub fn inst(&self) -> Result<&Institution, Error> {
        self.leaf(INST)
    }
    /// # Errors
    /// If the world has no such unit.
    pub fn ledger(&self) -> Result<&Ledger, Error> {
        self.leaf(LEDGER)
    }
    /// # Errors
    /// If the world has no such unit.
    pub fn epi(&self) -> Result<&Epistemic, Error> {
        self.leaf(EPI)
    }
    /// # Errors
    /// If the world has no such unit.
    pub fn norms(&self) -> Result<&Norms, Error> {
        self.leaf(NORMS)
    }
    /// # Errors
    /// If the world has no such unit.
    pub fn adj(&self) -> Result<&Adjudicator, Error> {
        self.leaf(ADJ)
    }
    /// # Errors
    /// If the world has no such unit.
    pub fn gov(&self) -> Result<&Governance, Error> {
        self.leaf(GOV)
    }
}

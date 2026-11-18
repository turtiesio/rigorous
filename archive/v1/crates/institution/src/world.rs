//! A typed facade over a runtime that hosts one organisation: acting, and reading state.

use crate::model::Fault;
use crate::model::{About, Act, ActorId, Attestation, CapId, CommitmentId, Content, Event, Method, Status, Tag, Time};
use crate::units::norms::Record;
use crate::units::{
    adjudicator::{Adjudicator, ADJ},
    epistemic::{Epistemic, EPI},
    governance::{Governance, GOV},
    institution::{Institution, INST},
    ledger::{Ledger, LEDGER},
    norms::{Norms, NORMS},
};
use rigor_kernel::{Entry, Name, Runtime, Unit};
use std::collections::{BTreeMap, BTreeSet};

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
    /// The boundaries this facade currently addresses, outermost first. Empty for the root.
    /// With a scope set, acts are performed by members of that nested organisation and reads
    /// are of its units: the same facade works at every level of nesting.
    pub scope: Vec<Name>,
}

impl World {
    #[must_use]
    pub fn new(root: Box<dyn Unit<Event>>) -> Self {
        Self { rt: Runtime::new(root), scope: Vec::new() }
    }

    /// # Errors
    /// On a kernel fault.
    pub fn inject(&mut self, actor: &Name, event: Event) -> Result<Vec<Entry<Event>>, Error> {
        let (mut path, mut event) = (rigor_kernel::Path::of(actor), event);
        for entity in self.scope.iter().rev() {
            path = path.inside(entity);
            event = Event::Scoped { entity: entity.clone(), inner: Box::new(event) };
        }
        Ok(self.rt.inject_as(path, event)?)
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
        Self { rt: self.rt.fork(), scope: self.scope.clone() }
    }

    fn leaf<T: 'static>(&self, name: Name) -> Result<&T, Error> {
        let mut path = self.scope.clone();
        path.push(name.clone());
        self.rt.leaf::<T>(&path).ok_or(Error::Missing(name))
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

/// Free-text detail for an attestation or record. Opaque: never interpreted.
#[must_use]
pub fn note(s: &str) -> Content {
    Content::Note(s.to_owned())
}

/// An attestation with no provenance and no bound artifacts.
#[must_use]
pub fn attest(about: About, method: Method, text: &str) -> Act {
    Act::Attest(Attestation { about, method, derived_from: BTreeSet::new(), artifacts: BTreeMap::new(), content: note(text) })
}

impl World {
    /// The single commitment with this debtor and tag.
    ///
    /// # Errors
    /// If there is not exactly one.
    pub fn one(&self, debtor: &ActorId, tag: Tag) -> Result<&Record, Error> {
        let mut it = self.norms()?.records().filter(|r| r.terms().debtor == *debtor && r.terms().tags.contains(&tag));
        match (it.next(), it.next()) {
            (Some(r), None) => Ok(r),
            _ => Err(Error::Expected("exactly one commitment with this debtor and tag")),
        }
    }

    /// Commitments carrying a tag.
    ///
    /// # Errors
    /// If the world has no norms unit.
    pub fn tagged(&self, tag: Tag) -> Result<Vec<&Record>, Error> {
        Ok(self.norms()?.records().filter(|r| r.terms().tags.contains(&tag)).collect())
    }

    /// Every commitment's current status.
    ///
    /// # Errors
    /// If the world has no norms unit.
    pub fn statuses(&self) -> Result<BTreeMap<CommitmentId, Status>, Error> {
        Ok(self.norms()?.records().filter_map(|r| r.status.map(|s| (r.commitment.id, s))).collect())
    }
}

//! Directed conditional commitments: the single normative primitive.
//!
//! A commitment's [`Status`] is not stored truth but a function of the current facts
//! ([`Norms::evaluate`]). So when a claim is refuted, `Satisfied` falls back to `Detached` or
//! `Violated` by re-evaluation, and everything that was derived from a violation that no longer
//! stands is voided. Violations and verdicts instantiate secondary commitments from rule
//! templates (contrary-to-duty obligations: remediation, refund, audit).

use crate::model::LeafError;
use crate::model::{
    Act, ActorId, Assessment, Basis, BasisKind, Commitment, CommitmentId, CommitmentTerms, Condition, ConsExpr, Event, Finding, Kind, NormError, NormRules,
    PartyExpr, Prop, Role, SkipReason, Source, Status, Template, TemplateIx, Time, Transfer, Version,
};
use rigor_waist::{digest_of, Envelope, Leaf, Name};
use std::collections::{BTreeMap, BTreeSet};

mod create;
mod evaluate;
mod operate;
mod query;
mod refresh;
mod secondary;
mod settle;
mod step;

pub const NORMS: Name = Name::lit("norms");

/// An explicit operation performed on a commitment.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
enum Op {
    Released,
    Cancelled,
    /// Its basis no longer stands.
    Void,
    /// The debtor repudiated it after it had detached.
    Breach,
}

#[derive(Clone, Debug, Hash, serde::Serialize, serde::Deserialize)]
pub struct Record {
    pub commitment: Commitment,
    op: Option<Op>,
    pub status: Option<Status>,
    pub created_at: Time,
    ant_at: Option<Time>,
    cons_at: Option<Time>,
}

impl Record {
    #[must_use]
    pub const fn terms(&self) -> &CommitmentTerms {
        &self.commitment.terms
    }
}

#[derive(Clone, Debug, Hash, serde::Serialize, serde::Deserialize)]
pub struct Norms {
    now: Time,
    version: Version,
    rules: NormRules,
    c: BTreeMap<CommitmentId, Record>,
    /// Per proposition: its assessment, since when it has been accepted, and when this unit learned of it.
    prop: BTreeMap<Prop, (Assessment, Option<Time>, Time)>,
    by_prop: BTreeMap<Prop, BTreeSet<CommitmentId>>,
    watch: BTreeSet<CommitmentId>,
    due: BTreeMap<Time, BTreeSet<CommitmentId>>,
    roles: BTreeMap<Role, ActorId>,
    derived: BTreeMap<CommitmentId, BTreeMap<BasisKind, CommitmentId>>,
    /// Who a debtor has asked to take on part of a commitment: (the commitment, the one asked).
    asked: BTreeSet<(CommitmentId, ActorId)>,
    /// What has actually been transferred, by (from, to, resource).
    paid: BTreeMap<(ActorId, ActorId, crate::model::Resource), u64>,
}

impl Default for Norms {
    fn default() -> Self {
        Self {
            now: Time::default(),
            version: Version::default(),
            rules: NormRules { on_violation: Vec::new(), on_verdict: Vec::new(), maintenance_period: 1 },
            c: BTreeMap::new(),
            prop: BTreeMap::new(),
            by_prop: BTreeMap::new(),
            watch: BTreeSet::new(),
            due: BTreeMap::new(),
            roles: BTreeMap::new(),
            derived: BTreeMap::new(),
            asked: BTreeSet::new(),
            paid: BTreeMap::new(),
        }
    }
}

fn ix(i: usize) -> TemplateIx {
    TemplateIx(u32::try_from(i).unwrap_or(u32::MAX))
}

impl Norms {
    #[must_use]
    pub const fn now(&self) -> Time {
        self.now
    }
}

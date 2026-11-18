//! Claims, evidence, provenance, and their revisable assessment.
//!
//! A proposition is never "true" here. It has an [`Assessment`] computed from the admissible
//! evidence by grounded argumentation under the evidence rules in force. New evidence, a
//! retraction, a changed artifact or a changed rule all revise it.
//!
//! * support and rebuttal of one proposition attack each other; an attack on strictly
//!   stronger evidence fails;
//! * an undercutter attacks a piece of evidence itself, and fails only against strictly
//!   stronger evidence;
//! * an inference can only SUPPORT, and only a conclusion that follows from what it cites by
//!   one of the fixed lines of [`Prop::follows_from`];
//! * a successful attack on evidence also hits everything derived from it;
//! * evidence bound to an artifact digest is stale once the artifact differs;
//! * support rooted in the word of an INTERESTED party is inadmissible: whoever claimed the
//!   proposition, and whoever owes a commitment that it would discharge.
//!
//! Reassessment is local to what lies DOWNSTREAM of the evidence that changed: what it
//! attacks, what is derived from it, and so on. Everything upstream keeps its recorded label
//! and enters only as a fixed boundary condition. (An earlier version re-labelled the whole
//! connected component; on a real dependency graph, where derivation connects everything,
//! that made each change cost as much as the whole graph.)

use super::meter::Meter;
use crate::model::LeafError;
use crate::model::{
    About, Act, ActorId, ArtifactId, Assessment, ContentDigest, Event, Evidence, EvidenceError, EvidenceId, EvidenceRules, Kind, Method, Prop, Rank, Time,
    Trust, Version,
};
use rigor_logic::argue::{grounded, Label};
use rigor_waist::{digest_of, Envelope, Leaf, Name};
use std::collections::{BTreeMap, BTreeSet};

mod admit;
mod assess;
mod attest;
mod graph;
mod query;
mod revise;
mod step;

pub const EPI: Name = Name::lit("epi");

#[derive(Clone, Debug, Hash, serde::Serialize, serde::Deserialize)]
struct Item {
    ev: Evidence,
    retracted: bool,
    /// Depth in the derivation graph: what an item is derived from always has a smaller one.
    /// (Not arrival order: independent arrivals must not change the state.)
    order: u64,
    /// Sources of its underived ancestors (fixed at arrival).
    roots: BTreeSet<ActorId>,
    /// Effective strength under the rules in force: the weakest link down to its roots.
    rank: Option<Rank>,
    /// When it most recently began to count (became admissible). Evidence that was stale or
    /// untrusted for a while counts from when that ended, not from when it first arrived.
    counts_from: Time,
    /// Recorded results of the last revision that touched it.
    admissible: bool,
    label: Label,
}

impl Item {
    const fn prop(&self) -> Option<&Prop> {
        match &self.ev.about {
            About::Supports(p) | About::Rebuts(p) => Some(p),
            About::Undercuts(_) => None,
        }
    }
}

/// The current standing of a proposition.
#[derive(Clone, Debug, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Standing {
    pub assessment: Assessment,
    /// When the earliest evidence now supporting it was given.
    pub since: Option<Time>,
    pub basis: Vec<EvidenceId>,
}

impl Default for Standing {
    fn default() -> Self {
        Self { assessment: Assessment::Unsupported, since: None, basis: Vec::new() }
    }
}

#[derive(Clone, Debug, Hash, serde::Serialize, serde::Deserialize)]
pub struct Epistemic {
    now: Time,
    version: Version,
    rules: EvidenceRules,
    claims: BTreeMap<Prop, BTreeMap<ActorId, Time>>,
    /// Who owes a commitment that the proposition would discharge.
    owing: BTreeMap<Prop, BTreeMap<ActorId, BTreeSet<crate::model::CommitmentId>>>,
    /// The open commitments this unit knows of: what each would be discharged by, and who owes it.
    owed: BTreeMap<crate::model::CommitmentId, (Prop, ActorId)>,
    ev: BTreeMap<EvidenceId, Item>,
    by_prop: BTreeMap<Prop, BTreeSet<EvidenceId>>,
    kids: BTreeMap<EvidenceId, BTreeSet<EvidenceId>>,
    cutters: BTreeMap<EvidenceId, BTreeSet<EvidenceId>>,
    art: BTreeMap<ArtifactId, ContentDigest>,
    by_art: BTreeMap<ArtifactId, BTreeSet<EvidenceId>>,
    status: BTreeMap<Prop, Standing>,
    /// Evidence nodes re-labelled so far (instrumentation).
    pub work: Meter,
}

impl Default for Epistemic {
    fn default() -> Self {
        Self {
            now: Time::default(),
            version: Version::default(),
            rules: EvidenceRules { ranks: BTreeMap::new(), trusted: BTreeMap::new(), self_support: false },
            claims: BTreeMap::new(),
            owing: BTreeMap::new(),
            owed: BTreeMap::new(),
            ev: BTreeMap::new(),
            by_prop: BTreeMap::new(),
            kids: BTreeMap::new(),
            cutters: BTreeMap::new(),
            art: BTreeMap::new(),
            by_art: BTreeMap::new(),
            status: BTreeMap::new(),
            work: Meter::default(),
        }
    }
}

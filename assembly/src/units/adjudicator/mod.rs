//! Responsibility: findings derived by explicit, versioned rules, with proofs.
//!
//! The adjudicator holds no opinions. It projects the events it has seen into sorted facts,
//! evaluates the responsibility rules in force, and publishes for each violated commitment the
//! findings, a derivation of each, and every closed-world step that derivation relied on.
//! Only the family (delegation tree) of a touched commitment is re-judged, and only once the
//! cascade that touched it has settled.

use super::meter::Meter;
use crate::model::{
    Act, ActorId, Assessment, Assumption, CommitmentId, Condition, Const, Event, Finding, Kind, Member, Pred, Prop, RFact, ResponsibilityRules, Status, Tag,
    Verdict, Version,
};
use crate::model::{Failure, LeafError};
use rigor_logic::datalog;
use rigor_waist::{Envelope, Leaf, Name};
use std::collections::{BTreeMap, BTreeSet};

mod adjudicate;
mod facts;
mod query;
mod step;

pub const ADJ: Name = Name::lit("adj");

#[derive(Clone, Debug, Hash, serde::Serialize, serde::Deserialize)]
struct Known {
    debtor: ActorId,
    creditor: ActorId,
    parent: Option<CommitmentId>,
    prop: Option<Prop>,
    tags: BTreeSet<Tag>,
    status: Option<Status>,
}

type Findings = BTreeSet<(Finding, ActorId)>;

/// One telling: who said it, what they said, and how many claims the hearer had made by then.
#[derive(Clone, Debug, Hash, serde::Serialize, serde::Deserialize)]
struct Notice {
    from: ActorId,
    value: bool,
    told: u64,
}

#[derive(Clone, Debug, Hash, serde::Serialize, serde::Deserialize)]
pub struct Adjudicator {
    version: Version,
    rules: ResponsibilityRules,
    versions: BTreeMap<Version, ResponsibilityRules>,
    members: BTreeMap<ActorId, Member>,
    c: BTreeMap<CommitmentId, Known>,
    kids: BTreeMap<CommitmentId, BTreeSet<CommitmentId>>,
    /// Who claimed what, as the ordinal of that claim among the claimant's own claims.
    claims: BTreeMap<Prop, BTreeMap<ActorId, u64>>,
    /// Who was told what, by whom, and how many claims the recipient had made by then: the
    /// first telling from each sender, in arrival order. (Order is kept per actor, not as a
    /// global count: events that concern different actors must not change each other's record.)
    /// Whether the sender was entitled to tell is decided when judging, not on arrival, so a
    /// notice that overtakes the news of the sender's own commitment is not lost.
    notified: BTreeMap<Prop, BTreeMap<ActorId, Vec<Notice>>>,
    /// How many claims each actor has made.
    claimed_so_far: BTreeMap<ActorId, u64>,
    prop: BTreeMap<Prop, Assessment>,
    by_prop: BTreeMap<Prop, BTreeSet<CommitmentId>>,
    verdicts: BTreeMap<CommitmentId, Findings>,
    /// The rule version in force when a family's first violation arose.
    arose: BTreeMap<CommitmentId, Version>,
    dirty: BTreeSet<CommitmentId>,
    /// Families re-judged so far (instrumentation).
    pub work: Meter,
}

impl Default for Adjudicator {
    fn default() -> Self {
        Self {
            version: Version::default(),
            rules: ResponsibilityRules { rules: Vec::new(), findings: BTreeSet::new(), retroactive: true },
            versions: BTreeMap::new(),
            members: BTreeMap::new(),
            c: BTreeMap::new(),
            kids: BTreeMap::new(),
            claims: BTreeMap::new(),
            notified: BTreeMap::new(),
            claimed_so_far: BTreeMap::new(),
            prop: BTreeMap::new(),
            by_prop: BTreeMap::new(),
            verdicts: BTreeMap::new(),
            arose: BTreeMap::new(),
            dirty: BTreeSet::new(),
            work: Meter::default(),
        }
    }
}

fn fact(pred: Pred, args: impl IntoIterator<Item = Const>) -> RFact {
    RFact { pred, args: args.into_iter().collect() }
}

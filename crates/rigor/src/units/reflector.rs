//! Reflection: the institution examining its own record and proposing changes to its own rules.
//!
//! The reflector watches how evidence fares. For each method of evidence it counts how often a
//! proposition accepted on the strength of that method was later refuted. When a method has been
//! relied on often enough and overturned too often, it PROPOSES lowering that method's rank,
//! through the ordinary governance procedure, as an ordinary member holding a charter to
//! propose. It cannot enact anything: with a quorum above one, someone else must approve.
//!
//! It also watches how long refutations of maintained propositions take to arrive. When the
//! period the rules prescribe for such undertakings is too short to catch nine in ten of the
//! refutations actually observed, it proposes the period that would.
//!
//! This is the whole of "self-improvement" here: the rules are revised on the evidence of how
//! they have performed, by the rules' own procedure, under human approval.

use crate::kernel::{Envelope, Leaf, Name};
use crate::model::{Act, ActorId, Amendment, Assessment, CapId, Condition, Event, EvidenceId, Kind, LeafError, Method, Prop, Rank, Time};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

/// How a method of evidence has fared.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Record {
    /// Times a proposition was accepted on evidence of this method.
    pub relied_on: u64,
    /// Times such an acceptance was later replaced by a refutation.
    pub overturned: u64,
}

#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct Reflector {
    me: ActorId,
    /// The capability under which it proposes.
    cap: CapId,
    /// Do not judge a method on fewer acceptances than this.
    min_sample: u64,
    /// Propose a lower rank when at least this percentage of acceptances were overturned.
    max_overturned_percent: u64,
    method_of: BTreeMap<EvidenceId, Method>,
    /// For each proposition currently accepted: the methods it is accepted on.
    resting_on: BTreeMap<Prop, BTreeSet<Method>>,
    record: BTreeMap<Method, Record>,
    ranks: BTreeMap<Method, Rank>,
    proposed: BTreeSet<Method>,
    now: Time,
    /// When each maintenance undertaking about a proposition began.
    maintained_since: BTreeMap<Prop, Time>,
    /// How long after that each observed refutation arrived, sorted.
    delays: Vec<u64>,
    /// Do not judge the period on fewer refutations than this.
    period_sample: usize,
    period: u64,
    period_proposed: bool,
}

impl Reflector {
    #[must_use]
    pub const fn new(me: ActorId, cap: CapId, min_sample: u64, max_overturned_percent: u64, period_sample: usize) -> Self {
        Self {
            me,
            cap,
            min_sample,
            max_overturned_percent,
            method_of: BTreeMap::new(),
            resting_on: BTreeMap::new(),
            record: BTreeMap::new(),
            ranks: BTreeMap::new(),
            proposed: BTreeSet::new(),
            now: Time(0),
            maintained_since: BTreeMap::new(),
            delays: Vec::new(),
            period_sample,
            period: 0,
            period_proposed: false,
        }
    }

    /// The delay within which nine in ten observed refutations arrived, once there are enough.
    #[must_use]
    pub fn ninety_percent_delay(&self) -> Option<u64> {
        if self.delays.len() < self.period_sample.max(1) {
            return None;
        }
        self.delays.get(self.delays.len().saturating_mul(9) / 10).copied()
    }

    #[must_use]
    pub fn refutations_observed(&self) -> usize {
        self.delays.len()
    }

    /// What it has observed so far.
    #[must_use]
    pub const fn record(&self) -> &BTreeMap<Method, Record> {
        &self.record
    }
}

impl Leaf<Event> for Reflector {
    fn name(&self) -> Name {
        self.me.0.clone()
    }
    fn inputs(&self) -> BTreeSet<Kind> {
        [Kind::EvidenceRecorded, Kind::Assessed, Kind::RulesChanged, Kind::NormCreated, Kind::Tick].into_iter().collect()
    }
    fn outputs(&self) -> BTreeSet<Kind> {
        std::iter::once(Kind::Do).collect()
    }
    fn on(&mut self, env: &Envelope<'_, Event>) -> Result<Vec<Event>, LeafError> {
        let mut out = Vec::new();
        match env.event {
            Event::Tick(t) => self.now = self.now.max(*t),
            Event::NormCreated(c) => {
                if let Condition::Unrefuted(p) = &c.terms.cons {
                    let _ = self.maintained_since.entry(p.clone()).or_insert(self.now);
                }
            }
            Event::EvidenceRecorded(e) => {
                let _ = self.method_of.insert(e.id, e.method);
            }
            Event::RulesChanged(rc) => {
                // A method whose rank changed starts a fresh record under its new rank.
                for (m, r) in &rc.rules.evidence.ranks {
                    if self.ranks.get(m).is_some_and(|old| old != r) {
                        let _ = self.record.remove(m);
                        let _ = self.proposed.remove(m);
                    }
                }
                self.ranks.clone_from(&rc.rules.evidence.ranks);
                self.period = rc.rules.norms.maintenance_period;
                self.period_proposed = false;
            }
            Event::Assessed { prop, assessment, basis, was, .. } => {
                let before = self.resting_on.remove(prop).unwrap_or_default();
                match assessment {
                    Assessment::Accepted => {
                        let now: BTreeSet<Method> = basis.iter().filter_map(|e| self.method_of.get(e).copied()).collect();
                        for m in now.difference(&before) {
                            self.record.entry(*m).or_default().relied_on += 1;
                        }
                        let _ = self.resting_on.insert(prop.clone(), now);
                    }
                    Assessment::Refuted if *was == Assessment::Accepted => {
                        for m in before {
                            self.record.entry(m).or_default().overturned += 1;
                        }
                    }
                    Assessment::Refuted | Assessment::Contested | Assessment::Unsupported => {}
                }
                if *assessment == Assessment::Refuted {
                    // Each proposition's first refutation counts once.
                    if let Some(since) = self.maintained_since.remove(prop) {
                        let delay = self.now.0.saturating_sub(since.0);
                        let at = self.delays.partition_point(|d| *d <= delay);
                        self.delays.insert(at, delay);
                    }
                }
                // Too short by a clear margin (a quarter), and not already put to the vote.
                if let Some(needed) = self.ninety_percent_delay() {
                    if !self.period_proposed && needed.saturating_mul(4) > self.period.saturating_mul(5) {
                        self.period_proposed = true;
                        out.push(Event::Do { act: Act::Propose { amendments: vec![Amendment::SetMaintenancePeriod(needed)] }, cap: Some(self.cap.clone()) });
                    }
                }
                for (m, r) in &self.record {
                    let poor = r.relied_on >= self.min_sample && r.overturned.saturating_mul(100) >= self.max_overturned_percent.saturating_mul(r.relied_on);
                    let lower = self.ranks.get(m).and_then(|rank| rank.0.checked_sub(1));
                    if let (true, Some(lower), false) = (poor, lower, self.proposed.contains(m)) {
                        let _ = self.proposed.insert(*m);
                        out.push(Event::Do { act: Act::Propose { amendments: vec![Amendment::SetRank(*m, Some(Rank(lower)))] }, cap: Some(self.cap.clone()) });
                    }
                }
            }
            _ => {}
        }
        Ok(out)
    }
}

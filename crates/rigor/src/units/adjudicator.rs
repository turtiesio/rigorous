//! Responsibility: findings derived by explicit, versioned rules, with proofs.
//!
//! The adjudicator holds no opinions. It projects the events it has seen into sorted facts,
//! evaluates the responsibility rules in force, and publishes for each violated commitment the
//! findings, a derivation of each, and every closed-world step that derivation relied on.
//! Only the family (delegation tree) of a touched commitment is re-judged, and only once the
//! cascade that touched it has settled.

use super::meter::Meter;
use crate::kernel::{Envelope, Leaf, Name};
use crate::logic::datalog;
use crate::model::{
    Act, ActorId, Assessment, Assumption, CommitmentId, Condition, Const, Event, Finding, Kind, Member, Pred, Prop, RFact, ResponsibilityRules, Status, Tag,
    Verdict, Version,
};
use crate::model::{Failure, LeafError};
use std::collections::{BTreeMap, BTreeSet};

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

#[derive(Clone, Debug, Hash, serde::Serialize, serde::Deserialize)]
pub struct Adjudicator {
    version: Version,
    rules: ResponsibilityRules,
    versions: BTreeMap<Version, ResponsibilityRules>,
    members: BTreeMap<ActorId, Member>,
    c: BTreeMap<CommitmentId, Known>,
    kids: BTreeMap<CommitmentId, BTreeSet<CommitmentId>>,
    claims: BTreeMap<Prop, BTreeSet<ActorId>>,
    notified: BTreeMap<Prop, BTreeMap<ActorId, bool>>,
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

impl Adjudicator {
    /// The findings currently standing against a commitment.
    #[must_use]
    pub fn findings(&self, id: &CommitmentId) -> Findings {
        self.verdicts.get(id).cloned().unwrap_or_default()
    }

    pub fn verdicts(&self) -> impl Iterator<Item = (&CommitmentId, &Findings)> {
        self.verdicts.iter().filter(|(_, f)| !f.is_empty())
    }

    /// The top of the delegation tree a commitment belongs to.
    #[must_use]
    pub fn root_of(&self, id: CommitmentId) -> CommitmentId {
        let mut cur = id;
        while let Some(p) = self.c.get(&cur).and_then(|k| k.parent).filter(|p| self.c.contains_key(p)) {
            cur = p;
        }
        cur
    }

    /// A delegation tree, as a set.
    #[must_use]
    pub fn family(&self, root: CommitmentId) -> BTreeSet<CommitmentId> {
        let mut out = BTreeSet::new();
        let mut stack = vec![root];
        while let Some(x) = stack.pop() {
            if out.insert(x) {
                stack.extend(self.kids.get(&x).into_iter().flatten().copied());
            }
        }
        out
    }

    fn facts(&self, fam: &BTreeSet<CommitmentId>) -> Vec<RFact> {
        let mut f = Vec::new();
        // Only the actors this family mentions, and their principals: the cost of judging a
        // family must not depend on how many members the institution has.
        let mut actors: BTreeSet<&ActorId> = BTreeSet::new();
        for k in fam.iter().filter_map(|id| self.c.get(id)) {
            actors.extend([&k.debtor, &k.creditor]);
            if let Some(p) = &k.prop {
                actors.extend(self.claims.get(p).into_iter().flatten());
                actors.extend(self.notified.get(p).into_iter().flat_map(BTreeMap::keys));
            }
        }
        let mut pending: Vec<&ActorId> = actors.iter().copied().collect();
        while let Some(a) = pending.pop() {
            if let Some(p) = self.members.get(a).and_then(|m| m.principal.as_ref()) {
                if actors.insert(p) {
                    pending.push(p);
                }
            }
        }
        for m in actors.into_iter().filter_map(|a| self.members.get(a)) {
            let a = || Const::Actor(m.id.clone());
            f.push(fact(Pred::Kind, [a(), Const::Kind(m.kind)]));
            if let Some(p) = &m.principal {
                f.push(fact(Pred::Principal, [a(), Const::Actor(p.clone())]));
            }
            if let Some(r) = m.role {
                f.push(fact(Pred::HasRole, [a(), Const::Role(r)]));
            }
        }
        for id in fam {
            let Some(k) = self.c.get(id) else { continue };
            let c = || Const::Commitment(*id);
            f.push(fact(Pred::Debtor, [c(), Const::Actor(k.debtor.clone())]));
            f.push(fact(Pred::Creditor, [c(), Const::Actor(k.creditor.clone())]));
            if let Some(s) = k.status {
                f.push(fact(Pred::Status, [c(), Const::Status(s)]));
            }
            f.extend(k.tags.iter().map(|t| fact(Pred::HasTag, [c(), Const::Tag(*t)])));
            if let Some(p) = k.parent.filter(|p| self.c.contains_key(p)) {
                f.push(fact(Pred::Parent, [c(), Const::Commitment(p)]));
            }
            if let Some(p) = &k.prop {
                let pc = || Const::Prop(p.clone());
                f.push(fact(Pred::Cons, [c(), pc()]));
                let a = self.prop.get(p).copied().unwrap_or(Assessment::Unsupported);
                f.push(fact(Pred::Assessed, [pc(), Const::Assessment(a)]));
                for x in self.claims.get(p).into_iter().flatten() {
                    f.push(fact(Pred::Claimed, [Const::Actor(x.clone()), pc()]));
                }
                for (x, v) in self.notified.get(p).into_iter().flatten() {
                    f.push(fact(Pred::Notified, [Const::Actor(x.clone()), pc(), Const::Bool(*v)]));
                }
            }
        }
        f
    }

    fn adjudicate(&mut self, root: CommitmentId, out: &mut Vec<Event>) -> Result<(), LeafError> {
        let fam = self.family(root);
        // Nothing violated and nothing previously found: there is nothing to judge or withdraw.
        let live =
            fam.iter().any(|id| self.c.get(id).is_some_and(|k| k.status == Some(Status::Violated)) || self.verdicts.get(id).is_some_and(|f| !f.is_empty()));
        if !live {
            return Ok(());
        }
        self.work.add(1);
        let version = if self.rules.retroactive { self.version } else { self.arose.get(&root).copied().unwrap_or(self.version) };
        let rules = self.versions.get(&version).unwrap_or(&self.rules).clone();
        let model = datalog::solve(&rules.rules, self.facts(&fam)).map_err(|e| LeafError::Broken(Failure::EnactedRulesDoNotCheck(e)))?;
        for id in fam {
            let violated = self.c.get(&id).is_some_and(|k| k.status == Some(Status::Violated));
            let mut findings = Findings::new();
            let mut hits: Vec<RFact> = Vec::new();
            if violated {
                for finding in &rules.findings {
                    for f in model.of(finding.pred()) {
                        if let [Const::Actor(who), Const::Commitment(c)] = f.args.as_slice() {
                            if *c == id {
                                let _ = findings.insert((*finding, who.clone()));
                                hits.push(f.clone());
                            }
                        }
                    }
                }
            }
            if findings == self.findings(&id) {
                continue;
            }
            let _ = self.verdicts.insert(id, findings.clone());
            let mut assumptions = vec![Assumption::Authentication, Assumption::RecordsNotKnowledge, Assumption::AssessmentIsRevisable];
            let absent: BTreeSet<RFact> = hits.iter().flat_map(|f| model.assumptions(f)).collect();
            assumptions.extend(absent.into_iter().map(Assumption::NoRecordOf));
            out.push(Event::Verdict(Verdict { commitment: id, findings, rules: version, proofs: hits.iter().map(|f| model.proof(f)).collect(), assumptions }));
        }
        Ok(())
    }

    fn touch_prop(&mut self, p: &Prop) {
        let roots: Vec<CommitmentId> = self.by_prop.get(p).into_iter().flatten().map(|c| self.root_of(*c)).collect();
        self.dirty.extend(roots);
    }
}

impl Leaf<Event> for Adjudicator {
    fn name(&self) -> Name {
        ADJ
    }
    fn inputs(&self) -> BTreeSet<Kind> {
        [Kind::Member, Kind::RulesChanged, Kind::NormCreated, Kind::NormStatus, Kind::NormAssigned, Kind::Ok, Kind::Assessed, Kind::Signal, Kind::Settle]
            .into_iter()
            .collect()
    }
    fn outputs(&self) -> BTreeSet<Kind> {
        std::iter::once(Kind::Verdict).collect()
    }
    fn on(&mut self, env: &Envelope<'_, Event>) -> Result<Vec<Event>, LeafError> {
        let mut out = Vec::new();
        match env.event {
            Event::Member(m) => {
                let _ = self.members.insert(m.id.clone(), m.clone());
            }
            Event::RulesChanged(rc) => {
                self.rules = rc.rules.responsibility.clone();
                self.version = rc.version;
                let _ = self.versions.insert(rc.version, self.rules.clone());
                let roots: Vec<CommitmentId> = self.c.keys().map(|c| self.root_of(*c)).collect();
                self.dirty.extend(roots);
            }
            Event::NormCreated(c) => {
                let t = &c.terms;
                let prop = match &t.cons {
                    Condition::Holds(p) | Condition::Unrefuted(p) => Some(p.clone()),
                    Condition::Paid { .. } => None,
                };
                if let Some(p) = &prop {
                    let _ = self.by_prop.entry(p.clone()).or_default().insert(c.id);
                }
                if let Some(parent) = t.parent {
                    let _ = self.kids.entry(parent).or_default().insert(c.id);
                }
                let known = Known { debtor: t.debtor.clone(), creditor: t.creditor.clone(), parent: t.parent, prop, tags: t.tags.clone(), status: None };
                let _ = self.c.insert(c.id, known);
            }
            Event::NormStatus { id, status, .. } => {
                if let Some(k) = self.c.get_mut(id) {
                    k.status = Some(*status);
                }
                let root = self.root_of(*id);
                if *status == Status::Violated {
                    let _ = self.arose.entry(root).or_insert(self.version);
                }
                let _ = self.dirty.insert(root);
            }
            Event::NormAssigned { id, creditor, .. } => {
                if let Some(k) = self.c.get_mut(id) {
                    k.creditor = creditor.clone();
                }
                let root = self.root_of(*id);
                let _ = self.dirty.insert(root);
            }
            Event::Ok { by, act: Act::Claim { prop }, .. } => {
                let _ = self.claims.entry(prop.clone()).or_default().insert(by.clone());
                self.touch_prop(prop);
            }
            Event::Assessed { prop, assessment, .. } => {
                let _ = self.prop.insert(prop.clone(), *assessment);
                self.touch_prop(prop);
            }
            Event::Signal { to, prop, value } => {
                let _ = self.notified.entry(prop.clone()).or_default().insert(to.clone(), *value);
                self.touch_prop(prop);
            }
            Event::Settle => {
                // Judge only quiescent states: never a half-propagated one.
                for root in std::mem::take(&mut self.dirty) {
                    self.adjudicate(root, &mut out)?;
                }
            }
            _ => {}
        }
        Ok(out)
    }
}

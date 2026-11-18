//! Claims, evidence, provenance, and their revisable assessment.
//!
//! A proposition is never "true" here. It has an [`Assessment`] computed from the admissible
//! evidence by grounded argumentation under the evidence rules in force. New evidence, a
//! retraction, a changed artifact or a changed rule all revise it.
//!
//! * support and rebuttal of one proposition attack each other; an attack on strictly
//!   stronger evidence fails;
//! * an undercutter attacks a piece of evidence itself and always succeeds;
//! * a successful attack on evidence also hits everything derived from it;
//! * evidence bound to an artifact digest is stale once the artifact differs;
//! * support rooted in the claimant's own word is inadmissible.
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
use rigor_kernel::{digest_of, Envelope, Leaf, Name};
use rigor_logic::argue::{grounded, Label};
use std::collections::{BTreeMap, BTreeSet};

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

impl Epistemic {
    #[must_use]
    pub fn standing(&self, p: &Prop) -> Standing {
        self.status.get(p).cloned().unwrap_or_default()
    }

    #[must_use]
    pub fn assessment(&self, p: &Prop) -> Assessment {
        self.standing(p).assessment
    }

    pub fn standings(&self) -> impl Iterator<Item = (&Prop, &Standing)> {
        self.status.iter()
    }

    pub fn evidence(&self) -> impl Iterator<Item = &Evidence> {
        self.ev.values().map(|i| &i.ev)
    }

    #[must_use]
    pub fn evidence_on(&self, p: &Prop) -> BTreeSet<EvidenceId> {
        self.by_prop.get(p).cloned().unwrap_or_default()
    }

    #[must_use]
    pub fn evidence_about_artifact(&self, a: &ArtifactId) -> BTreeSet<EvidenceId> {
        self.by_art.get(a).cloned().unwrap_or_default()
    }

    /// The sources of the underived ancestors of an item. Memoised: a derivation graph shares
    /// ancestors, and walking every path separately is exponential in its depth.
    fn roots(&self, id: EvidenceId, memo: &mut BTreeMap<EvidenceId, BTreeSet<ActorId>>) -> BTreeSet<ActorId> {
        if let Some(known) = memo.get(&id) {
            return known.clone();
        }
        let out = match self.ev.get(&id) {
            None => BTreeSet::new(),
            Some(item) if item.ev.derived_from.is_empty() => std::iter::once(item.ev.source.clone()).collect(),
            Some(item) => item.ev.derived_from.iter().flat_map(|p| self.roots(*p, memo)).collect(),
        };
        let _ = memo.insert(id, out.clone());
        out
    }

    /// Effective strength: the weakest link among the item and what it is derived from.
    fn rank(&self, id: EvidenceId, memo: &mut BTreeMap<EvidenceId, Option<Rank>>) -> Option<Rank> {
        if let Some(known) = memo.get(&id) {
            return *known;
        }
        let out = self.ev.get(&id).and_then(|item| {
            let mut r = *self.rules.ranks.get(&item.ev.method)?;
            for p in &item.ev.derived_from {
                r = r.min(self.rank(*p, memo)?);
            }
            Some(r)
        });
        let _ = memo.insert(id, out);
        out
    }

    fn admissible(&self, id: EvidenceId, memo: &mut BTreeMap<EvidenceId, bool>, roots: &mut BTreeMap<EvidenceId, BTreeSet<ActorId>>) -> bool {
        if let Some(known) = memo.get(&id) {
            return *known;
        }
        let ok = self.ev.get(&id).is_some_and(|item| {
            let e = &item.ev;
            let trusted = match self.rules.trusted.get(&e.method) {
                Some(Trust::Anyone) => true,
                Some(Trust::Only(set)) => set.contains(&e.source),
                None => false,
            };
            let fresh = e.artifacts.iter().all(|(a, d)| self.art.get(a) == Some(d));
            let independent = match &e.about {
                About::Supports(p) if !self.rules.self_support => match self.claims.get(p) {
                    // Nobody has claimed it: nothing can be self-support.
                    None => true,
                    Some(claimants) => !self.roots(id, roots).iter().any(|s| claimants.contains_key(s)),
                },
                About::Supports(_) | About::Rebuts(_) | About::Undercuts(_) => true,
            };
            !item.retracted
                && self.rules.ranks.contains_key(&e.method)
                && trusted
                && fresh
                && independent
                && e.derived_from.iter().all(|p| self.admissible(*p, memo, roots))
        });
        let _ = memo.insert(id, ok);
        ok
    }

    fn descendants(&self, id: EvidenceId) -> BTreeSet<EvidenceId> {
        let mut out = BTreeSet::new();
        let mut stack = vec![id];
        while let Some(x) = stack.pop() {
            for k in self.kids.get(&x).into_iter().flatten() {
                if out.insert(*k) {
                    stack.push(*k);
                }
            }
        }
        out
    }

    /// Everything connected to the seeds by shared propositions, undercuts or derivation.
    #[must_use]
    pub fn component(&self, seeds: impl IntoIterator<Item = EvidenceId>) -> BTreeSet<EvidenceId> {
        let mut out = BTreeSet::new();
        let mut stack: Vec<EvidenceId> = seeds.into_iter().filter(|s| self.ev.contains_key(s)).collect();
        while let Some(x) = stack.pop() {
            if !out.insert(x) {
                continue;
            }
            let Some(item) = self.ev.get(&x) else { continue };
            stack.extend(item.ev.derived_from.iter().copied());
            stack.extend(self.kids.get(&x).into_iter().flatten().copied());
            stack.extend(self.cutters.get(&x).into_iter().flatten().copied());
            match &item.ev.about {
                About::Undercuts(t) => stack.push(*t),
                About::Supports(p) | About::Rebuts(p) => stack.extend(self.by_prop.get(p).into_iter().flatten().copied()),
            }
        }
        out
    }

    /// The proposition every item in the set bears on.
    #[must_use]
    pub fn props_of(&self, ids: &BTreeSet<EvidenceId>) -> BTreeSet<Prop> {
        ids.iter().filter_map(|e| self.ev.get(e)).filter_map(|i| i.prop().cloned()).collect()
    }

    fn label(&self, ids: &BTreeSet<EvidenceId>) -> BTreeMap<EvidenceId, Label> {
        let (mut memo, mut roots, mut ranks) = (BTreeMap::new(), BTreeMap::new(), BTreeMap::new());
        let adm: BTreeSet<EvidenceId> = ids.iter().copied().filter(|e| self.admissible(*e, &mut memo, &mut roots)).collect();
        let rank: BTreeMap<EvidenceId, Rank> = adm.iter().filter_map(|e| self.rank(*e, &mut ranks).map(|r| (*e, r))).collect();
        let mut direct: Vec<(EvidenceId, EvidenceId)> = Vec::new();
        let mut sides: BTreeMap<&Prop, (Vec<EvidenceId>, Vec<EvidenceId>)> = BTreeMap::new();
        for e in &adm {
            let Some(item) = self.ev.get(e) else { continue };
            match &item.ev.about {
                About::Supports(p) => sides.entry(p).or_default().0.push(*e),
                About::Rebuts(p) => sides.entry(p).or_default().1.push(*e),
                About::Undercuts(t) if adm.contains(t) => direct.push((*e, *t)),
                About::Undercuts(_) => {}
            }
        }
        for (sup, reb) in sides.values() {
            for a in sup {
                for b in reb {
                    if let (Some(ra), Some(rb)) = (rank.get(a), rank.get(b)) {
                        if rb <= ra {
                            direct.push((*a, *b));
                        }
                        if ra <= rb {
                            direct.push((*b, *a));
                        }
                    }
                }
            }
        }
        let mut attacks: BTreeSet<(EvidenceId, EvidenceId)> = direct.iter().copied().collect();
        for (a, b) in &direct {
            attacks.extend(self.descendants(*b).into_iter().filter(|d| adm.contains(d)).map(|d| (*a, d)));
        }
        grounded(&adm, &attacks)
    }

    fn assess(&self, p: &Prop, labels: &BTreeMap<EvidenceId, Label>) -> Standing {
        let (mut sup, mut reb, mut undecided) = (Vec::new(), Vec::new(), false);
        for e in self.by_prop.get(p).into_iter().flatten() {
            let Some(item) = self.ev.get(e) else { continue };
            match (labels.get(e), &item.ev.about) {
                (Some(Label::In), About::Supports(_)) => sup.push(item),
                (Some(Label::In), About::Rebuts(_)) => reb.push(item),
                (Some(Label::Undecided), _) => undecided = true,
                (Some(Label::In | Label::Out) | None, _) => {}
            }
        }
        if !reb.is_empty() {
            Standing { assessment: Assessment::Refuted, since: None, basis: reb.iter().map(|i| i.ev.id).collect() }
        } else if !sup.is_empty() {
            Standing { assessment: Assessment::Accepted, since: sup.iter().map(|i| i.ev.at).min(), basis: sup.iter().map(|i| i.ev.id).collect() }
        } else if undecided {
            Standing { assessment: Assessment::Contested, since: None, basis: Vec::new() }
        } else {
            Standing::default()
        }
    }

    /// From-scratch assessment of every proposition (the reference for the incremental path).
    #[must_use]
    pub fn full(&self) -> BTreeMap<Prop, Standing> {
        let all: BTreeSet<EvidenceId> = self.ev.keys().copied().collect();
        let labels = self.label(&all);
        self.by_prop.keys().chain(self.status.keys()).map(|p| (p.clone(), self.assess(p, &labels))).collect()
    }

    /// Whether an item passes every test that does not depend on other items' labels,
    /// given that what it is derived from is admissible.
    fn passes(&self, item: &Item) -> bool {
        let e = &item.ev;
        let trusted = match self.rules.trusted.get(&e.method) {
            Some(Trust::Anyone) => true,
            Some(Trust::Only(set)) => set.contains(&e.source),
            None => false,
        };
        let independent = match &e.about {
            About::Supports(p) if !self.rules.self_support => self.claims.get(p).is_none_or(|claimants| !item.roots.iter().any(|s| claimants.contains_key(s))),
            About::Supports(_) | About::Rebuts(_) | About::Undercuts(_) => true,
        };
        !item.retracted
            && self.rules.ranks.contains_key(&e.method)
            && trusted
            && e.artifacts.iter().all(|(a, d)| self.art.get(a) == Some(d))
            && independent
            && e.derived_from.iter().all(|p| self.ev.get(p).is_some_and(|x| x.admissible))
    }

    /// Everything a change to the seeds can reach: evidence on the same proposition, the
    /// target of an undercut, and whatever is derived from any of those.
    fn downstream(&self, seeds: impl IntoIterator<Item = EvidenceId>) -> BTreeSet<EvidenceId> {
        let mut out = BTreeSet::new();
        let mut stack: Vec<EvidenceId> = seeds.into_iter().filter(|s| self.ev.contains_key(s)).collect();
        while let Some(x) = stack.pop() {
            if !out.insert(x) {
                continue;
            }
            let Some(item) = self.ev.get(&x) else { continue };
            stack.extend(self.kids.get(&x).into_iter().flatten().copied());
            match &item.ev.about {
                About::Undercuts(t) => stack.push(*t),
                About::Supports(p) | About::Rebuts(p) => stack.extend(self.by_prop.get(p).into_iter().flatten().copied()),
            }
        }
        out
    }

    /// The admissible items that attack `y` itself (not through what it is derived from).
    fn direct_attackers(&self, y: EvidenceId, into: &mut BTreeSet<EvidenceId>) {
        let Some(item) = self.ev.get(&y) else { return };
        into.extend(self.cutters.get(&y).into_iter().flatten().copied().filter(|c| self.ev.get(c).is_some_and(|x| x.admissible)));
        let supports = match &item.ev.about {
            About::Supports(_) => true,
            About::Rebuts(_) => false,
            About::Undercuts(_) => return,
        };
        let Some(p) = item.prop() else { return };
        for other in self.by_prop.get(p).into_iter().flatten() {
            let Some(o) = self.ev.get(other).filter(|o| o.admissible) else { continue };
            let opposed = matches!(o.ev.about, About::Rebuts(_)) == supports;
            // An attack on strictly stronger evidence fails.
            if opposed && item.rank <= o.rank {
                let _ = into.insert(*other);
            }
        }
    }

    /// Everything that attacks `x`: its own attackers and those of everything it is derived from.
    fn attackers(&self, x: EvidenceId) -> BTreeSet<EvidenceId> {
        let mut out = BTreeSet::new();
        let mut seen = BTreeSet::new();
        let mut stack = vec![x];
        while let Some(y) = stack.pop() {
            if seen.insert(y) {
                self.direct_attackers(y, &mut out);
                stack.extend(self.ev.get(&y).into_iter().flat_map(|i| i.ev.derived_from.iter().copied()));
            }
        }
        out
    }

    /// Revise what lies downstream of the seeds, and announce every assessment that changed.
    fn revise(&mut self, seeds: impl IntoIterator<Item = EvidenceId>, extra: impl IntoIterator<Item = Prop>, out: &mut Vec<Event>) {
        let area = self.downstream(seeds);
        self.work.add(area.len());
        // 1. Admissibility, shallowest first so that what an item rests on is settled first.
        let mut ordered: Vec<(u64, EvidenceId)> = area.iter().filter_map(|e| self.ev.get(e).map(|i| (i.order, *e))).collect();
        ordered.sort_unstable();
        for (_, e) in &ordered {
            let ok = self.ev.get(e).is_some_and(|i| self.passes(i));
            if let Some(i) = self.ev.get_mut(e) {
                i.admissible = ok;
            }
        }
        // 2. The attack graph over the area; attackers outside it enter with their recorded
        //    labels as fixed boundary conditions (`None` is a node that is always in).
        let mut nodes: BTreeSet<Option<EvidenceId>> = BTreeSet::new();
        let mut attacks: BTreeSet<(Option<EvidenceId>, Option<EvidenceId>)> = BTreeSet::new();
        for (_, x) in &ordered {
            if !self.ev.get(x).is_some_and(|i| i.admissible) {
                continue;
            }
            let _ = nodes.insert(Some(*x));
            for a in self.attackers(*x) {
                let _ = attacks.insert((Some(a), Some(*x)));
                if !area.contains(&a) && nodes.insert(Some(a)) {
                    match self.ev.get(&a).map(|i| i.label) {
                        Some(Label::In) | None => {}
                        Some(Label::Out) => {
                            let _ = nodes.insert(None);
                            let _ = attacks.insert((None, Some(a)));
                        }
                        Some(Label::Undecided) => {
                            let _ = attacks.insert((Some(a), Some(a)));
                        }
                    }
                }
            }
        }
        let labels = grounded(&nodes, &attacks);
        for (_, x) in &ordered {
            let label = labels.get(&Some(*x)).copied().unwrap_or(Label::Out);
            if let Some(i) = self.ev.get_mut(x) {
                i.label = label;
            }
        }
        // 3. The propositions touched.
        let mut props = self.props_of(&area);
        props.extend(extra);
        for p in props {
            let new = self.assess_recorded(&p);
            let old = self.standing(&p);
            if new != old {
                out.push(Event::Assessed {
                    prop: p.clone(),
                    assessment: new.assessment,
                    since: new.since,
                    basis: new.basis.clone(),
                    was: old.assessment,
                    rules: self.version,
                });
                let _ = self.status.insert(p, new);
            }
        }
    }

    fn assess_recorded(&self, p: &Prop) -> Standing {
        let labels: BTreeMap<EvidenceId, Label> =
            self.by_prop.get(p).into_iter().flatten().filter_map(|e| self.ev.get(e).filter(|i| i.admissible).map(|i| (*e, i.label))).collect();
        self.assess(p, &labels)
    }

    /// Effective rank from the recorded ranks of what an item is derived from.
    fn rank_of(&self, method: Method, derived_from: &BTreeSet<EvidenceId>) -> Option<Rank> {
        let mut r = *self.rules.ranks.get(&method)?;
        for p in derived_from {
            r = r.min(self.ev.get(p)?.rank?);
        }
        Some(r)
    }

    fn attest(&mut self, by: &ActorId, a: &crate::model::Attestation, out: &mut Vec<Event>) -> Result<(), LeafError> {
        let reject = |reason| Event::EvidenceRejected { by: by.clone(), reason };
        if let About::Undercuts(t) = &a.about {
            if !self.ev.contains_key(t) {
                out.push(reject(EvidenceError::UnknownTarget));
                return Ok(());
            }
        }
        if a.derived_from.iter().any(|p| !self.ev.contains_key(p)) {
            out.push(reject(EvidenceError::UnknownProvenance));
            return Ok(());
        }
        if a.method == Method::Inference && a.derived_from.is_empty() {
            out.push(reject(EvidenceError::UngroundedInference));
            return Ok(());
        }
        let id = EvidenceId(digest_of(&(by, a, self.now))?);
        if self.ev.contains_key(&id) {
            out.push(reject(EvidenceError::Duplicate));
            return Ok(());
        }
        let ev = Evidence {
            id,
            source: by.clone(),
            about: a.about.clone(),
            method: a.method,
            derived_from: a.derived_from.clone(),
            artifacts: a.artifacts.clone(),
            content: a.content.clone(),
            at: self.now,
        };
        match &ev.about {
            About::Supports(p) | About::Rebuts(p) => {
                let _ = self.by_prop.entry(p.clone()).or_default().insert(id);
            }
            About::Undercuts(t) => {
                let _ = self.cutters.entry(*t).or_default().insert(id);
            }
        }
        for p in &ev.derived_from {
            let _ = self.kids.entry(*p).or_default().insert(id);
        }
        for art in ev.artifacts.keys() {
            let _ = self.by_art.entry(art.clone()).or_default().insert(id);
        }
        out.push(Event::EvidenceRecorded(ev.clone()));
        let roots: BTreeSet<ActorId> = if ev.derived_from.is_empty() {
            std::iter::once(ev.source.clone()).collect()
        } else {
            ev.derived_from.iter().filter_map(|p| self.ev.get(p)).flat_map(|i| i.roots.iter().cloned()).collect()
        };
        let rank = self.rank_of(ev.method, &ev.derived_from);
        let depth = ev.derived_from.iter().filter_map(|p| self.ev.get(p)).map(|i| i.order).max().map_or(0, |d| d + 1);
        let _ = self.ev.insert(id, Item { ev, retracted: false, order: depth, roots, rank, admissible: false, label: Label::Out });
        self.revise([id], [], out);
        Ok(())
    }
}

impl Leaf<Event> for Epistemic {
    fn name(&self) -> Name {
        EPI
    }
    fn inputs(&self) -> BTreeSet<Kind> {
        [Kind::Tick, Kind::RulesChanged, Kind::Ok, Kind::ArtifactChanged].into_iter().collect()
    }
    fn outputs(&self) -> BTreeSet<Kind> {
        [Kind::Assessed, Kind::EvidenceRecorded, Kind::EvidenceRejected, Kind::EvidenceRetracted].into_iter().collect()
    }
    fn on(&mut self, env: &Envelope<'_, Event>) -> Result<Vec<Event>, LeafError> {
        let mut out = Vec::new();
        match env.event {
            Event::Tick(t) => self.now = self.now.max(*t),
            Event::RulesChanged(rc) => {
                self.rules = rc.rules.evidence.clone();
                self.version = rc.version;
                // Ranks are rule-dependent: recompute them shallowest first, then revise everything.
                let mut ordered: Vec<(u64, EvidenceId)> = self.ev.iter().map(|(e, i)| (i.order, *e)).collect();
                ordered.sort_unstable();
                for (_, e) in &ordered {
                    let rank = self.ev.get(e).and_then(|i| self.rank_of(i.ev.method, &i.ev.derived_from));
                    if let Some(i) = self.ev.get_mut(e) {
                        i.rank = rank;
                    }
                }
                let known: Vec<Prop> = self.status.keys().cloned().collect();
                self.revise(ordered.into_iter().map(|(_, e)| e), known, &mut out);
            }
            Event::ArtifactChanged { artifact, digest } => {
                if self.art.get(artifact) != Some(digest) {
                    let _ = self.art.insert(artifact.clone(), *digest);
                    self.revise(self.evidence_about_artifact(artifact), [], &mut out);
                }
            }
            Event::Ok { by, act: Act::Claim { prop }, .. } => {
                let claimants = self.claims.entry(prop.clone()).or_default();
                if !claimants.contains_key(by) {
                    let _ = claimants.insert(by.clone(), self.now);
                    self.revise(self.evidence_on(prop), [prop.clone()], &mut out);
                }
            }
            Event::Ok { by, act: Act::Retract { evidence }, .. } => match self.ev.get_mut(evidence) {
                Some(item) if item.ev.source == *by => {
                    if !item.retracted {
                        item.retracted = true;
                        out.push(Event::EvidenceRetracted { id: *evidence, by: by.clone() });
                        self.revise([*evidence], [], &mut out);
                    }
                }
                Some(_) | None => out.push(Event::EvidenceRejected { by: by.clone(), reason: EvidenceError::NotTheSource }),
            },
            Event::Ok { by, act: Act::Attest(a), .. } => self.attest(by, a, &mut out)?,
            _ => {}
        }
        Ok(out)
    }
}

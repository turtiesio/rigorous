//! Turning index records into commitments and evidence: a release rests on what it requires.

use crate::cast::*;
use crate::oracle::*;
use crate::published::*;
use rigor_institution::model::{
    About, Act, ActorId, Attestation, CapId, Condition, Content, Event, EvidenceId, Member, MemberKind, Method, Offer, Prop, Release, Tag, Time,
};
use rigor_institution::world::{Error, World};
use std::collections::{BTreeMap, BTreeSet};

/// What ingestion met.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Stats {
    pub packages: usize,
    pub releases: usize,
    pub yanked: usize,
    pub dependencies_resolved: usize,
    pub dependencies_unresolved: usize,
    /// Releases left out because their dependencies form a cycle under this resolution.
    pub on_a_cycle: usize,
    pub refused: usize,
}

/// Publish every release (dependencies first), then apply every yank.
///
/// # Errors
/// On a kernel fault.
pub fn ingest(w: &mut World, all: &[Published]) -> Result<Stats, Error> {
    let deps = resolve(all);
    let mut stats = Stats { releases: all.len(), yanked: all.iter().filter(|p| p.yanked).count(), ..Stats::default() };
    for p in all {
        let resolved = deps.get(&p.release).map_or(0, Vec::len);
        stats.dependencies_resolved += resolved;
        stats.dependencies_unresolved += p.requires.len().saturating_sub(resolved);
    }
    // Dependencies first: an iterative depth-first order; whatever sits on a cycle is left out.
    let mut order: Vec<&Release> = Vec::new();
    let mut state: BTreeMap<&Release, bool> = BTreeMap::new(); // false = in progress, true = done
    for p in all {
        let mut stack: Vec<(&Release, usize)> = vec![(&p.release, 0)];
        while let Some((r, i)) = stack.pop() {
            if i == 0 {
                if state.contains_key(r) {
                    continue;
                }
                let _ = state.insert(r, false);
            }
            match deps.get(r).and_then(|ds| ds.get(i)) {
                Some(d) => {
                    stack.push((r, i + 1));
                    if !state.contains_key(d) {
                        stack.push((d, 0));
                    }
                }
                None => {
                    let _ = state.insert(r, true);
                    order.push(r);
                }
            }
        }
    }
    let mut evidence: BTreeMap<&Release, EvidenceId> = BTreeMap::new();
    let mut known: BTreeSet<ActorId> = BTreeSet::new();
    for r in order {
        let publisher = ActorId(r.package.clone());
        if known.insert(publisher.clone()) {
            stats.packages += 1;
            let member = Member { id: publisher.clone(), kind: MemberKind::Org, principal: None, role: None };
            let _ = w.act_with(&REGISTRAR, &CapId::Charter(ADMISSIONS), Act::Admit(member))?;
        }
        let parents: Option<BTreeSet<EvidenceId>> = deps.get(r).map(|ds| ds.iter().map(|d| evidence.get(d).copied()).collect()).unwrap_or_default();
        let Some(derived_from) = parents else {
            stats.on_a_cycle += 1;
            continue;
        };
        let fit = Prop::Fit(r.clone());
        let offer = Offer {
            creditor: REGISTRY,
            ant: None,
            cons: Condition::Holds(fit.clone()),
            deadline: Time(1),
            tags: std::iter::once(Tag::Upkeep).collect(),
            parent: None,
            terms: None,
        };
        let _ = w.act(&publisher, Act::Commit(offer))?;
        let a = Attestation {
            about: About::Supports(fit),
            method: Method::Receipt,
            derived_from,
            artifacts: BTreeMap::new(),
            content: Content::Note("published".to_owned()),
        };
        let entries = w.act(&REGISTRY, Act::Attest(a))?;
        match entries.iter().find_map(|e| match &e.event {
            Event::EvidenceRecorded(ev) => Some(ev.id),
            _ => None,
        }) {
            Some(id) => {
                let _ = evidence.insert(r, id);
            }
            None => stats.refused += 1,
        }
    }
    for p in all.iter().filter(|p| p.yanked) {
        if let Some(id) = evidence.get(&p.release) {
            let entries = w.act(&REGISTRY, Act::Retract { evidence: *id })?;
            stats.refused += usize::from(!entries.iter().any(|e| matches!(e.event, Event::EvidenceRetracted { .. })));
        }
    }
    Ok(stats)
}

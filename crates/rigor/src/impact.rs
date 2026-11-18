//! Impact analysis: what does a change touch?
//!
//! * [`cone`] is a static over-approximation obtained by walking dependency indices
//!   (evidence → propositions → commitments → families → verdicts → derived commitments).
//!   Nothing is executed.
//! * [`what_if`] is exact: fork the world, apply the stimulus, compare judgments.
//!
//! Soundness (tested, not proved): every existing judgment that `what_if` reports as changed
//! lies inside the cone of the change.

use crate::kernel::Name;
use crate::model::{ActorId, ArtifactId, Assessment, CapId, CommitmentId, Event, EvidenceId, Finding, Prop, RulePath, Status};
use crate::world::{Error, World};
use std::collections::{BTreeMap, BTreeSet};

/// Something the institution currently holds to be the case.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Judgment {
    Prop(Prop),
    Commitment(CommitmentId),
    Verdict(CommitmentId),
    Cap(CapId),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    Assessment(Assessment),
    Status(Status),
    Findings(BTreeSet<(Finding, ActorId)>),
    Valid(bool),
}

/// A hypothetical change whose reach is to be bounded.
#[derive(Clone, Debug)]
pub enum Change {
    Evidence(EvidenceId),
    Prop(Prop),
    Artifact(ArtifactId),
    Cap(CapId),
    Rules(RulePath),
}

/// Every judgment the world currently holds.
///
/// # Errors
/// If the world lacks an institutional unit.
pub fn judgments(w: &World) -> Result<BTreeMap<Judgment, Value>, Error> {
    let mut j = BTreeMap::new();
    for (p, s) in w.epi()?.standings() {
        let _ = j.insert(Judgment::Prop(p.clone()), Value::Assessment(s.assessment));
    }
    for r in w.norms()?.records() {
        if let Some(s) = r.status {
            let _ = j.insert(Judgment::Commitment(r.commitment.id), Value::Status(s));
        }
    }
    for (c, f) in w.adj()?.verdicts() {
        let _ = j.insert(Judgment::Verdict(*c), Value::Findings(f.clone()));
    }
    let inst = w.inst()?;
    for id in inst.caps().keys() {
        let _ = j.insert(Judgment::Cap(id.clone()), Value::Valid(inst.valid(id)));
    }
    Ok(j)
}

/// A judgment before and after; `None` means it did not exist.
pub type Delta = (Option<Value>, Option<Value>);

/// Exactly which judgments a stimulus would change. The world itself is not touched.
///
/// # Errors
/// On a kernel fault in the fork.
pub fn what_if(w: &World, actor: &Name, event: Event) -> Result<BTreeMap<Judgment, Delta>, Error> {
    let before = judgments(w)?;
    let mut fork = w.fork();
    let _ = fork.inject(actor, event)?;
    let after = judgments(&fork)?;
    let keys: BTreeSet<&Judgment> = before.keys().chain(after.keys()).collect();
    Ok(keys.into_iter().filter(|k| before.get(k) != after.get(k)).map(|k| (k.clone(), (before.get(k).cloned(), after.get(k).cloned()))).collect())
}

fn closure(w: &World, seeds: BTreeSet<CommitmentId>) -> Result<BTreeSet<CommitmentId>, Error> {
    let (norms, adj) = (w.norms()?, w.adj()?);
    let mut out = BTreeSet::new();
    let mut stack: Vec<CommitmentId> = seeds.into_iter().collect();
    while let Some(c) = stack.pop() {
        if norms.record(&c).is_none() || !out.insert(c) {
            continue;
        }
        stack.extend(adj.family(adj.root_of(c)));
        stack.extend(norms.derived_from(&c));
    }
    Ok(out)
}

/// A static bound on the judgments a change can affect.
///
/// # Errors
/// If the world lacks an institutional unit.
pub fn cone(w: &World, change: &Change) -> Result<BTreeSet<Judgment>, Error> {
    let (epi, norms, inst) = (w.epi()?, w.norms()?, w.inst()?);
    let all_evidence = || epi.evidence().map(|e| e.id).collect::<BTreeSet<EvidenceId>>();
    let (evidence, mut props): (BTreeSet<EvidenceId>, BTreeSet<Prop>) = match change {
        Change::Evidence(e) => (std::iter::once(*e).collect(), BTreeSet::new()),
        Change::Prop(p) => (epi.evidence_on(p), std::iter::once(p.clone()).collect()),
        Change::Artifact(a) => (epi.evidence_about_artifact(a), BTreeSet::new()),
        Change::Rules(RulePath::Ranks | RulePath::Trust | RulePath::SelfSupport) => (all_evidence(), epi.standings().map(|(p, _)| p.clone()).collect()),
        Change::Rules(RulePath::Responsibility | RulePath::Retroactive) => {
            let all = closure(w, norms.records().map(|r| r.commitment.id).collect())?;
            return Ok(all.into_iter().flat_map(|c| [Judgment::Commitment(c), Judgment::Verdict(c)]).collect());
        }
        Change::Rules(RulePath::Templates | RulePath::Quorum | RulePath::Entrenched) => return Ok(BTreeSet::new()),
        Change::Cap(id) => {
            let mut hit: BTreeSet<CapId> = std::iter::once(id.clone()).collect();
            loop {
                let more: Vec<CapId> =
                    inst.caps().values().filter(|c| c.parent.as_ref().is_some_and(|p| hit.contains(p)) && !hit.contains(&c.id)).map(|c| c.id.clone()).collect();
                if more.is_empty() {
                    break;
                }
                hit.extend(more);
            }
            return Ok(hit.into_iter().map(Judgment::Cap).collect());
        }
    };
    let component = epi.component(evidence);
    props.extend(epi.props_of(&component));
    let seeds: BTreeSet<CommitmentId> = props.iter().flat_map(|p| norms.depending_on(p)).collect();
    let mut out: BTreeSet<Judgment> = props.into_iter().map(Judgment::Prop).collect();
    out.extend(closure(w, seeds)?.into_iter().flat_map(|c| [Judgment::Commitment(c), Judgment::Verdict(c)]));
    Ok(out)
}

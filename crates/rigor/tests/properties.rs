//! Randomised invariants (seeded, reproducible) and bounded model checks.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::indexing_slicing)]

mod common;

use common::Rng;
use rigor::actors::CREDIT;
use rigor::kernel::explore::{interleavings, reach};
use rigor::kernel::{Cell, Name, Par, Runtime, Unit};
use rigor::model::{
    About, Act, ActionKind, ActorId, Amendment, Amount, ArtifactId, Assessment, Attestation, CapId, Condition, Content, ContentDigest, Event, EvidenceId,
    Grant, Memo, Method, Offer, Prop, Rank, RulePath, RulesChanged, Scope, Status, Time, Transfer, Trust, Version,
};
use rigor::org::{open_kinds, org};
use rigor::scenario::{attest, budget, constitution, pay, rules, world, Cast, AGENT, ALICE, AUDITOR, DEPLOYER, MONITOR, VENDOR};
use rigor::synth::{refute, world_n};
use rigor::units::epistemic::{Epistemic, EPI};
use rigor::units::norms::{Norms, NORMS};
use rigor::world::{World, CONSTITUTION};
use std::collections::{BTreeMap, BTreeSet};

/// A single unit as a root.
fn alone(unit: Box<dyn Unit<Event>>) -> Box<dyn Unit<Event>> {
    Box::new(Par::new(vec![unit], &open_kinds()).unwrap())
}

#[test]
fn random_delegation_trees_never_amplify_authority() {
    // Whatever members attempt: every capability attenuates its parent, no chain overspends any
    // ancestor's limit, a revoked subtree never acts again, only the owner's line moves funds,
    // and credit is conserved.
    let who = [ALICE, AGENT, VENDOR, DEPLOYER, MONITOR];
    for seed in 0..25 {
        let mut rng = Rng::new(seed);
        let mut c = constitution(rules());
        c.balances = who.iter().map(|m| (m.clone(), CREDIT, Amount(60))).collect();
        let mut w = World::new(Box::new(org(Vec::new()).unwrap()));
        let _ = w.inject(&CONSTITUTION, Event::Genesis(Box::new(c))).unwrap();
        let mut dead: BTreeSet<CapId> = BTreeSet::new();
        let mut now = 0u64;
        for step in 0..120 {
            let caps: Vec<CapId> = w.inst().unwrap().caps().keys().cloned().collect();
            let (actor, cap) = (rng.pick(&who).clone(), rng.pick(&caps).clone());
            let roll = rng.below(100);
            let entries = if roll < 35 {
                let grant = Grant {
                    to: rng.pick(&who).clone(),
                    actions: std::iter::once(ActionKind::Transfer).collect(),
                    scope: Scope { from: Some(rng.pick(&who).clone()), purpose: None },
                    limits: std::iter::once((CREDIT, Amount(1 + rng.below_u64(80)))).collect(),
                    expires: *rng.pick(&[None, Some(Time(5)), Some(Time(50))]),
                    delegable: rng.percent(80),
                };
                w.act_with(&actor, &cap, Act::Delegate(grant)).unwrap()
            } else if roll < 85 {
                let t = Transfer {
                    from: rng.pick(&who).clone(),
                    to: rng.pick(&who).clone(),
                    resource: CREDIT,
                    amount: Amount(rng.below_u64(40)),
                    memo: Memo::None,
                };
                w.act_with(&actor, &cap, Act::Transfer(t)).unwrap()
            } else if roll < 95 {
                w.act(&actor, Act::Revoke { target: cap }).unwrap()
            } else {
                now += 1;
                w.tick(now).unwrap()
            };
            let inst = w.inst().unwrap();
            for e in &entries {
                let via = match &e.event {
                    Event::Ok { act: Act::Revoke { target }, .. } => {
                        let _ = dead.insert(target.clone());
                        continue;
                    }
                    Event::Ok { via, .. } => via.clone(),
                    Event::Delegated(c) => c.parent.clone().unwrap(),
                    _ => continue,
                };
                let lineage = inst.lineage(&via);
                assert!(!lineage.iter().any(|c| dead.contains(&c.id)), "seed {seed} step {step}: revoked authority was exercised");
                if let Event::Ok { by, act: Act::Transfer(t), .. } = &e.event {
                    assert_eq!(lineage[0].holder, *by);
                    assert_eq!(lineage.last().unwrap().scope.from.as_ref(), Some(&t.from), "only the owner's line moves funds");
                }
            }
            assert!(inst.audit().is_empty(), "seed {seed} step {step}");
            assert!(w.ledger().unwrap().conserved(), "seed {seed} step {step}");
        }
    }
}

fn fact(i: usize) -> Prop {
    Prop::Fact(Name::new(&format!("p{i}")).unwrap())
}

#[test]
fn incremental_assessment_equals_from_scratch() {
    // After every event, the locally revised assessment equals a full recomputation.
    let sources = [DEPLOYER, MONITOR, ALICE, AUDITOR, VENDOR];
    let methods = [Method::Receipt, Method::Probe, Method::Ack, Method::Audit, Method::Test, Method::Inference, Method::Hearsay];
    let art = ArtifactId::lit("f");
    for seed in 0..40 {
        let mut rng = Rng::new(seed);
        let mut r = rules();
        r.evidence.trusted = methods.iter().filter(|m| **m != Method::Hearsay).map(|m| (*m, Trust::Anyone)).collect();
        let mut rt: Runtime<Event> = Runtime::new(alone(Box::new(Cell(Epistemic::default()))));
        let rc = RulesChanged {
            version: Version(1),
            rules: r,
            digest: ContentDigest::default_for_tests(),
            prev: None,
            changed: BTreeSet::new(),
            by: BTreeSet::new(),
        };
        let env = Name::lit("env");
        let _ = rt.inject_unchecked(rigor::kernel::Path::of(&env), Event::RulesChanged(Box::new(rc))).unwrap();
        let mut now = 0u64;
        for step in 0..60 {
            let epi = rt.leaf::<Epistemic>(&[EPI]).unwrap();
            let ids: Vec<EvidenceId> = epi.evidence().map(|e| e.id).collect();
            let by = rng.pick(&sources).clone();
            let ok = |act: Act| Event::Ok { by: by.clone(), via: CapId::Base(by.clone()), act };
            let roll = rng.below(100);
            let event = if roll < 55 || ids.is_empty() {
                let n = (*rng.pick(&[0usize, 0, 1, 2])).min(ids.len());
                let derived_from: BTreeSet<EvidenceId> = (0..n).map(|_| *rng.pick(&ids)).collect();
                let mut method = *rng.pick(&methods);
                if method == Method::Inference && derived_from.is_empty() {
                    method = Method::Ack;
                }
                let about = if !ids.is_empty() && rng.percent(20) {
                    About::Undercuts(*rng.pick(&ids))
                } else if rng.percent(50) {
                    About::Supports(fact(rng.below(4)))
                } else {
                    About::Rebuts(fact(rng.below(4)))
                };
                let artifacts: BTreeMap<ArtifactId, ContentDigest> = if rng.percent(20) {
                    std::iter::once((art.clone(), ContentDigest(rigor::kernel::Digest(rng.below_u64(2))))).collect()
                } else {
                    BTreeMap::new()
                };
                ok(Act::Attest(Attestation { about, method, derived_from, artifacts, content: Content::Note(format!("{step}")) }))
            } else if roll < 70 {
                ok(Act::Claim { prop: fact(rng.below(4)) })
            } else if roll < 80 {
                let target = epi.evidence().nth(rng.below(ids.len())).unwrap();
                Event::Ok { by: target.source.clone(), via: CapId::Base(target.source.clone()), act: Act::Retract { evidence: target.id } }
            } else if roll < 90 {
                Event::ArtifactChanged { artifact: art.clone(), digest: ContentDigest(rigor::kernel::Digest(rng.below_u64(2))) }
            } else {
                now += 1;
                Event::Tick(Time(now))
            };
            let _ = rt.inject_unchecked(rigor::kernel::Path::of(&env), event).unwrap();
            let epi = rt.leaf::<Epistemic>(&[EPI]).unwrap();
            for (prop, standing) in epi.full() {
                assert_eq!(epi.standing(&prop), standing, "seed {seed} step {step} {prop}");
            }
        }
    }
}

const D: ActorId = ActorId::lit("d");
const C: ActorId = ActorId::lit("c");

fn lifecycle() -> (Runtime<Event>, rigor::model::CommitmentId) {
    let mut rt: Runtime<Event> = Runtime::new(alone(Box::new(Cell(Norms::default()))));
    let offer = Offer {
        creditor: C,
        ant: Some(Condition::Holds(Prop::Fact(Name::lit("a")))),
        cons: Condition::Holds(Prop::Fact(Name::lit("p"))),
        deadline: Time(2),
        tags: BTreeSet::new(),
        parent: None,
        terms: None,
    };
    let _ = rt.inject_unchecked(rigor::kernel::Path::of(&Name::lit("env")), Event::Ok { by: D, via: CapId::Base(D), act: Act::Commit(offer) }).unwrap();
    let id = rt.leaf::<Norms>(&[NORMS]).unwrap().records().next().unwrap().commitment.id;
    (rt, id)
}

fn status(rt: &Runtime<Event>, id: rigor::model::CommitmentId) -> Status {
    rt.leaf::<Norms>(&[NORMS]).unwrap().status(&id).unwrap()
}

/// The ways the lifecycle properties could fail.
#[derive(Debug, PartialEq, Eq)]
enum Broke {
    LeftAbsorbing(Status),
    SatisfactionUndoneWithoutReassessment,
    PendingAfterDeadline,
    DebtorReleasedItself,
}

#[test]
fn model_check_single_commitment() {
    // Exhaustive exploration, to a fixed point, of one commitment under every interleaving of
    // time, (re)assessment of antecedent and consequent, release and cancel.
    let (rt, id) = lifecycle();
    let env = Name::lit("env");
    let ok = |by: ActorId, act: Act| (env.clone(), Event::Ok { by: by.clone(), via: CapId::Base(by), act });
    let mut alphabet: Vec<(Name, Event)> = (1..=3).map(|t| (env.clone(), Event::Tick(Time(t)))).collect();
    for name in ["a", "p"] {
        let prop = Prop::Fact(Name::new(name).unwrap());
        for (assessment, since) in [(Assessment::Accepted, Some(Time(1))), (Assessment::Accepted, Some(Time(3))), (Assessment::Refuted, None)] {
            alphabet.push((
                env.clone(),
                Event::Assessed { prop: prop.clone(), assessment, since, basis: Vec::new(), was: Assessment::Unsupported, rules: Version(1) },
            ));
        }
    }
    let debtor_release = ok(D, Act::Release { id }); // the debtor cannot release itself
    alphabet.extend([ok(C, Act::Release { id }), ok(D, Act::Cancel { id }), debtor_release.clone()]);
    assert_eq!(alphabet.len(), 12);

    let mut seen: BTreeSet<(Status, Status)> = BTreeSet::new();
    let res = reach(&rt, &alphabet, 12, |before, action, _, after| {
        let (b, a) = (status(before, id), status(after, id));
        let revises = matches!(&action.1, Event::Assessed { .. } | Event::Ok { act: Act::Release { .. }, .. });
        if b.absorbing() && a != b {
            return Some(Broke::LeftAbsorbing(b));
        }
        if b == Status::Satisfied && a != Status::Satisfied && !revises {
            return Some(Broke::SatisfactionUndoneWithoutReassessment);
        }
        if after.leaf::<Norms>(&[NORMS]).unwrap().now() > Time(2) && matches!(a, Status::Conditional | Status::Detached) {
            return Some(Broke::PendingAfterDeadline);
        }
        if action.1 == debtor_release.1 && a != b {
            return Some(Broke::DebtorReleasedItself);
        }
        if a != b {
            let _ = seen.insert((b, a));
        }
        None
    })
    .unwrap();
    assert_eq!(res.violation.map(|(why, _)| why), None::<Broke>);
    assert!(res.complete, "state space not exhausted within the bound");
    assert!(seen.contains(&(Status::Satisfied, Status::Violated))); // revision is real
    assert!(seen.contains(&(Status::Violated, Status::Satisfied))); // ...in both directions
    assert!(!seen.iter().any(|(from, _)| from.absorbing()));
}

#[test]
fn model_checker_finds_a_seeded_bug() {
    // Sanity of the checker itself: a false invariant yields a shortest counterexample trace.
    let (rt, id) = lifecycle();
    let env = Name::lit("env");
    let alphabet = vec![
        (env.clone(), Event::Tick(Time(3))),
        (
            env,
            Event::Assessed {
                prop: Prop::Fact(Name::lit("a")),
                assessment: Assessment::Accepted,
                since: Some(Time(1)),
                basis: Vec::new(),
                was: Assessment::Unsupported,
                rules: Version(1),
            },
        ),
    ];
    let res = reach(&rt, &alphabet, 4, |_, _, _, after| (status(after, id) == Status::Violated).then_some(())).unwrap();
    assert_eq!(res.violation.unwrap().1.len(), 2);
}

#[test]
fn independent_stimuli_commute() {
    // Five stimuli touching disjoint facts: all 120 orderings reach one state.
    let w = world(Cast::default(), rules()).unwrap();
    let f = |n: &'static str| Prop::Fact(Name::new(n).unwrap());
    let act = |a: Act| Event::Do { act: a, cap: None };
    let offer = Offer { creditor: ALICE, ant: None, cons: Condition::Holds(f("a")), deadline: Time(9), tags: BTreeSet::new(), parent: None, terms: None };
    let stimuli = vec![
        (MONITOR.0, act(attest(About::Supports(f("a")), Method::Probe, "x"))),
        (DEPLOYER.0, act(attest(About::Supports(f("b")), Method::Receipt, "x"))),
        (ALICE.0, Event::Do { act: pay(ALICE, VENDOR, 5), cap: Some(CapId::Own(ALICE)) }),
        (AUDITOR.0, act(Act::Claim { prop: f("c") })),
        (VENDOR.0, act(Act::Commit(offer))),
    ];
    assert_eq!(interleavings(&w.rt, &stimuli).unwrap().len(), 1);
}

#[test]
fn dependent_stimuli_do_not_commute() {
    // A use and a revocation of the same capability: the log order IS the fact.
    let mut w = world(Cast::default(), rules()).unwrap();
    let cap = rigor::scenario::delegated(&w.act_with(&ALICE, &CapId::Own(ALICE), budget(AGENT, ALICE, 50, false)).unwrap()).unwrap();
    let race =
        vec![(ALICE.0, Event::Do { act: Act::Revoke { target: cap.clone() }, cap: None }), (AGENT.0, Event::Do { act: pay(ALICE, AGENT, 5), cap: Some(cap) })];
    assert_eq!(interleavings(&w.rt, &race).unwrap().len(), 2);
}

#[test]
fn revision_work_is_independent_of_world_size() {
    let mut costs = Vec::new();
    for n in [20usize, 200] {
        let mut w = world_n(n, 2).unwrap();
        let (e0, a0) = (w.epi().unwrap().work.0, w.adj().unwrap().work.0);
        let before = w.statuses().unwrap();
        let entries = refute(&mut w, n / 2, 1).unwrap();
        let after = w.statuses().unwrap();
        let changed = after.iter().filter(|(k, v)| before.get(k) != Some(v)).count();
        costs.push((entries.len(), w.epi().unwrap().work.0 - e0, w.adj().unwrap().work.0 - a0, changed));
    }
    assert_eq!(costs[0], costs[1]);
    assert!(costs[0].3 > 0);
}

#[test]
fn amendments_touch_only_their_path() {
    let r = rules();
    for (a, path) in [
        (Amendment::SetRank(Method::Hearsay, Some(Rank(0))), RulePath::Ranks),
        (Amendment::SetQuorum(3), RulePath::Quorum),
        (Amendment::SetRetroactive(false), RulePath::Retroactive),
    ] {
        assert_eq!(a.path(), path);
        let after = r.amended(std::slice::from_ref(&a));
        assert_ne!(after, r);
        assert_eq!(after.norms, r.norms);
    }
}

#[test]
fn exact_impact_always_lies_inside_the_static_cone() {
    // Random worlds of delegation chains with random extra evidence; random hypothetical
    // stimuli. Every existing judgment that the stimulus really changes is inside the bound
    // computed without executing anything; every new commitment hangs off something inside it.
    use rigor::impact::{cone, what_if, Change, Judgment};
    use rigor::synth::{link, world_n, AUDITOR as S_AUDITOR, DEPLOYER as S_DEPLOYER, MONITOR as S_MONITOR, WORKER};
    let mut cases = 0;
    let mut changed_something = 0;
    for seed in 0..60 {
        let mut rng = Rng::new(seed);
        let (n, chain) = (1 + rng.below(4), 1 + rng.below(3));
        let mut w = world_n(n, chain).unwrap();
        // Roughen it: some links already rebutted, some rebuttals undercut, some derived evidence.
        for _ in 0..rng.below(4) {
            let p = link(rng.below(n), rng.below(chain)).unwrap();
            let entries = w.act(&S_MONITOR, attest(About::Rebuts(p), Method::Probe, "rough")).unwrap();
            if rng.percent(40) {
                if let Some(e) = entries.iter().find_map(|e| match &e.event {
                    Event::EvidenceRecorded(ev) => Some(ev.id),
                    _ => None,
                }) {
                    let _ = w.act(&S_AUDITOR, attest(About::Undercuts(e), Method::Audit, "rough")).unwrap();
                }
            }
        }
        for _ in 0..6 {
            let p = link(rng.below(n), rng.below(chain)).unwrap();
            let ids: Vec<EvidenceId> = w.epi().unwrap().evidence().map(|e| e.id).collect();
            let (actor, act, change) = match rng.below(4) {
                0 => (S_MONITOR, attest(About::Rebuts(p.clone()), Method::Probe, "what if"), Change::Prop(p)),
                1 => (S_AUDITOR, attest(About::Supports(p.clone()), Method::Audit, "what if"), Change::Prop(p)),
                2 => {
                    let e = *rng.pick(&ids);
                    (S_AUDITOR, attest(About::Undercuts(e), Method::Audit, "what if"), Change::Evidence(e))
                }
                _ => {
                    let e = w.epi().unwrap().evidence().nth(rng.below(ids.len())).unwrap();
                    let _ = (&WORKER, &S_DEPLOYER);
                    (e.source.clone(), Act::Retract { evidence: e.id }, Change::Evidence(e.id))
                }
            };
            let bound = cone(&w, &change).unwrap();
            let event = Event::Do { act, cap: None };
            let exact = what_if(&w, &actor.0, event.clone()).unwrap();
            let mut after = w.fork();
            let _ = after.inject(&actor.0, event.clone()).unwrap();
            for (j, (before, _)) in &exact {
                match (j, before) {
                    (Judgment::Commitment(c), None) => {
                        let of = after.norms().unwrap().record(c).unwrap().terms().basis.as_ref().unwrap().of;
                        assert!(bound.contains(&Judgment::Commitment(of)), "seed {seed}: new commitment outside the cone");
                    }
                    _ => assert!(bound.contains(j), "seed {seed}: {j:?} changed outside the cone of {change:?}"),
                }
            }
            cases += 1;
            changed_something += usize::from(!exact.is_empty());
            // Sometimes let the hypothetical become real, so later cases start from revised worlds.
            if rng.percent(50) {
                let _ = w.inject(&actor.0, event).unwrap();
            }
        }
    }
    assert_eq!(cases, 360);
    assert!(changed_something > 100, "the property must not hold vacuously: {changed_something}");
}

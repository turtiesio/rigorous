//! Red-team fuzzer: incremental `revise` against the from-scratch `full()` reference, with a
//! wider mix than tests/properties.rs (rule changes mid-run, more undercuts, deeper derivation).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::indexing_slicing)]
mod common;
use common::Rng;
use rigor::kernel::{Cell, Name, Par, Runtime, Unit};
use rigor::model::{
    About, Act, ArtifactId, Attestation, CapId, Content, ContentDigest, Event, EvidenceId, Method, Prop, Rank, RulesChanged, Time, Trust, Version,
};
use rigor::org::open_kinds;
use rigor::scenario::{rules, ALICE, AUDITOR, DEPLOYER, MONITOR, VENDOR};
use rigor::units::epistemic::{Epistemic, EPI};
use std::collections::{BTreeMap, BTreeSet};

fn fact(i: usize) -> Prop {
    Prop::Fact(Name::new(&format!("p{i}")).unwrap())
}

#[test]
fn incremental_equals_full_under_a_wider_mix() {
    let sources = [DEPLOYER, MONITOR, ALICE, AUDITOR, VENDOR];
    let methods = [Method::Receipt, Method::Probe, Method::Ack, Method::Audit, Method::Test, Method::Inference, Method::Hearsay, Method::Reversal];
    let arts = [ArtifactId::lit("f"), ArtifactId::lit("g")];
    let seeds: u64 = std::env::var("FUZZ_SEEDS").ok().and_then(|s| s.parse().ok()).unwrap_or(300);
    for seed in 0..seeds {
        let mut rng = Rng::new(seed.wrapping_add(7919));
        let mut r = rules();
        r.evidence.trusted = methods.iter().filter(|m| **m != Method::Hearsay).map(|m| (*m, Trust::Anyone)).collect();
        let mut version = 1u32;
        let rc = |r: &rigor::model::Rules, v: u32| {
            Event::RulesChanged(Box::new(RulesChanged {
                version: Version(v),
                rules: r.clone(),
                digest: ContentDigest::default_for_tests(),
                prev: None,
                changed: BTreeSet::new(),
                by: BTreeSet::new(),
            }))
        };
        let root: Box<dyn Unit<Event>> = Box::new(Par::new(vec![Box::new(Cell(Epistemic::default()))], &open_kinds()).unwrap());
        let mut rt: Runtime<Event> = Runtime::new(root);
        let env = Name::lit("env");
        let _ = rt.inject(&env, rc(&r, version)).unwrap();
        let mut now = 0u64;
        let nprops = 2 + rng.below(4);
        for step in 0..80 {
            let epi = rt.leaf::<Epistemic>(&[EPI]).unwrap();
            let ids: Vec<EvidenceId> = epi.evidence().map(|e| e.id).collect();
            let by = rng.pick(&sources).clone();
            let ok = |act: Act| Event::Ok { by: by.clone(), via: CapId::Base(by.clone()), act };
            let roll = rng.below(100);
            let event = if roll < 50 || ids.is_empty() {
                let n = (*rng.pick(&[0usize, 0, 1, 1, 2, 3])).min(ids.len());
                let derived_from: BTreeSet<EvidenceId> = (0..n).map(|_| *rng.pick(&ids)).collect();
                let mut method = *rng.pick(&methods);
                if method == Method::Inference && derived_from.is_empty() {
                    method = Method::Ack;
                }
                let about = if !ids.is_empty() && rng.percent(35) {
                    About::Undercuts(*rng.pick(&ids))
                } else if rng.percent(50) {
                    About::Supports(fact(rng.below(nprops)))
                } else {
                    About::Rebuts(fact(rng.below(nprops)))
                };
                let artifacts: BTreeMap<ArtifactId, ContentDigest> = if rng.percent(25) {
                    std::iter::once((rng.pick(&arts).clone(), ContentDigest(rigor::kernel::Digest(rng.below_u64(2))))).collect()
                } else {
                    BTreeMap::new()
                };
                ok(Act::Attest(Attestation { about, method, derived_from, artifacts, content: Content::Note(format!("{step}")) }))
            } else if roll < 62 {
                ok(Act::Claim { prop: fact(rng.below(nprops)) })
            } else if roll < 74 {
                let target = epi.evidence().nth(rng.below(ids.len())).unwrap();
                Event::Ok { by: target.source.clone(), via: CapId::Base(target.source.clone()), act: Act::Retract { evidence: target.id } }
            } else if roll < 84 {
                Event::ArtifactChanged { artifact: rng.pick(&arts).clone(), digest: ContentDigest(rigor::kernel::Digest(rng.below_u64(2))) }
            } else if roll < 96 && std::env::var("FUZZ_NORULES").is_err() {
                version += 1;
                let m = *rng.pick(&methods);
                match rng.below(4) {
                    0 => {
                        let _ = r.evidence.ranks.remove(&m);
                    }
                    1 => {
                        let _ = r.evidence.ranks.insert(m, Rank(u8::try_from(rng.below(4)).unwrap()));
                    }
                    2 => {
                        let t =
                            if rng.percent(50) { Trust::Anyone } else { Trust::Only(BTreeSet::from([rng.pick(&sources).clone(), rng.pick(&sources).clone()])) };
                        let _ = r.evidence.trusted.insert(m, t);
                    }
                    _ => r.evidence.self_support = !r.evidence.self_support,
                }
                rc(&r, version)
            } else {
                now += 1;
                Event::Tick(Time(now))
            };
            let _ = rt.inject(&env, event.clone()).unwrap();
            let epi = rt.leaf::<Epistemic>(&[EPI]).unwrap();
            for (prop, standing) in epi.full() {
                assert_eq!(epi.standing(&prop), standing, "seed {seed} step {step} {prop} after {event:?}");
            }
        }
    }
}

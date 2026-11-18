//! One history on its own: ingest, check every invariant, compare with the oracle, resume from a snapshot.

use crate::common::*;
use rigor::adapters::{resume, save_snapshot};
use rigor::gitlog::{conclusions, found, oracle, Commit, GitError, GOVERN_PROJECT, GOVERN_REGISTRAR, PROJECT, REGISTRAR};
use rigor::model::{Act, Amendment, CapId, ChangeId, Event, Method, Rank};
use rigor::waist::Runtime;
use rigor::world::Error;
use std::collections::BTreeSet;
use std::time::Instant;

pub(crate) fn solo(name: &str, commits: &[Commit]) -> Result<BTreeSet<ChangeId>, GitError> {
    println!("\n================ {name}: {} commits", commits.len());
    let mut w = found()?;
    let t = Instant::now();
    let ingest = ingest_all(&mut w, commits)?;
    let took = secs(t);
    let events = w.rt.log().next_seq();
    println!("ingested in {took:.2}s: {events} events, {:.0} events/s", f64::from(u32::try_from(events).unwrap_or(u32::MAX)) / took);
    println!("\nWHAT THE DATA CONTAINED\n{:#?}", ingest.stats);
    let got = conclusions(&w)?;
    println!(
        "\nWHAT THE SYSTEM CONCLUDED\n  changes refuted: {}\n  commitments by status: {:?}\n  commitments with findings: {}",
        got.refuted.len(),
        got.by_status,
        got.commitments_with_findings
    );

    let periods: Vec<u64> =
        w.rt.log()
            .entries()
            .iter()
            .filter_map(|e| match &e.event {
                Event::RulesChanged(rc) => Some(rc.rules.norms.maintenance_period / 86_400),
                _ => None,
            })
            .collect();
    println!("  maintenance period over time, in days (each change proposed by the reflector from the record, approved by the project): {periods:?}");
    if let Some(r) = w.rt.leaf::<rigor::units::reflector::Reflector>(&[rigor::gitlog::REFLECTOR.0]) {
        println!(
            "  refutations the reflector observed: {}; nine in ten arrived within {:?} days",
            r.refutations_observed(),
            r.ninety_percent_delay().map(|s| s / 86_400)
        );
        println!("  the reflector's record of how evidence fared: {:?}", r.record());
        let proposals = w.rt.log().entries().iter().filter(|e| matches!(e.event, Event::Proposed { .. })).count();
        println!("  rule changes it proposed on that record: {proposals}");
    }

    println!("\nCHECKS");
    let expected = oracle(commits);
    println!("  refuted set equals independent oracle ({} changes): {}", expected.len(), expected == got.refuted);
    let full = w.epi()?.full();
    println!(
        "  incremental assessment equals from-scratch over {} propositions: {}",
        full.len(),
        full.iter().all(|(p, s)| w.epi().is_ok_and(|e| e.standing(p) == *s))
    );
    println!("  every act was authorised (nothing denied or rejected): {}", ingest.stats.denied == 0);
    println!("  no capability amplifies its parent ({} capabilities): {}", w.inst()?.caps().len(), w.inst()?.audit().is_empty());
    println!("  log hash chain verifies: {}", w.rt.log().verify().is_ok());
    let t = Instant::now();
    let replayed = Runtime::replay(Box::new(rigor::gitlog::project_org().map_err(Error::Fault)?), &w.rt.stimuli()).map_err(Error::Fault)?;
    let replay_secs = secs(t);
    println!("  replay from stimuli reproduces the log exactly: {} ({replay_secs:.2}s)", replayed.log().head() == w.rt.log().head());
    drop(replayed);

    println!("\nSNAPSHOT (taken half-way, then the second half is ingested from the snapshot)");
    let (first, second) = commits.split_at(commits.len() / 2);
    let mut a = found()?;
    let mut ia = self_tuning();
    for c in first {
        ia.commit(&mut a, c)?;
    }
    let path = std::env::temp_dir().join(format!("rigor-{}.snapshot", std::process::id()));
    let t = Instant::now();
    let snap = save_snapshot(&a, &path)?;
    let save_secs = secs(t);
    let t = Instant::now();
    let mut b = resume(Box::new(rigor::gitlog::project_org().map_err(Error::Fault)?), &path)?;
    let resume_secs = secs(t);
    let _ = std::fs::remove_file(&path);
    for c in second {
        ia.commit(&mut b, c)?;
    }
    ia.close(&mut b)?;
    println!(
        "  snapshot of {} events: {:.1} MB, written in {save_secs:.2}s, resumed in {resume_secs:.2}s (full replay: {replay_secs:.2}s)",
        snap.seq,
        f64::from(u32::try_from(snap.state.len() / 1024).unwrap_or(u32::MAX)) / 1024.0
    );
    println!(
        "  resumed-then-continued world equals the uninterrupted one (state and chain head): {}",
        b.rt.digest() == w.rt.digest() && b.rt.log().head() == w.rt.log().head()
    );
    println!("  entries the resumed world had to hold: {} of {}", b.rt.log().entries().len(), b.rt.log().next_seq());

    println!("\nGOVERNANCE ON THE REAL RECORD (the project amends its own evidence rules: an acknowledgement now outranks a reversal)");
    let acknowledged_and_refuted =
        got.refuted.iter().filter(|c| w.epi().is_ok_and(|e| e.evidence_on(&rigor::model::Prop::Sound((*c).clone())).len() > 1)).count();
    let violated_before = got.by_status.get(&rigor::model::Status::Violated).copied().unwrap_or(0);
    let t = Instant::now();
    let proposed = w.act_with(&PROJECT, &CapId::Charter(GOVERN_PROJECT), Act::Propose { amendments: vec![Amendment::SetRank(Method::Ack, Some(Rank(4)))] })?;
    let id = proposed.iter().find_map(|e| match &e.event {
        Event::Proposed { id, .. } => Some(*id),
        _ => None,
    });
    let Some(id) = id else { return Err(GitError::World(Error::Expected("proposal accepted for voting"))) };
    let caused = w.act_with(&REGISTRAR, &CapId::Charter(GOVERN_REGISTRAR), Act::Approve { id })?;
    let after = conclusions(&w)?;
    println!("  enacted as version {} in {:.2}s; one approval caused {} events", w.gov()?.version().0, secs(t), caused.len());
    println!(
        "  changes held refuted: {} -> {}   (those with more than one piece of evidence before: {acknowledged_and_refuted})",
        got.refuted.len(),
        after.refuted.len()
    );
    println!(
        "  commitments violated: {violated_before} -> {}; commitments with findings: {} -> {}",
        after.by_status.get(&rigor::model::Status::Violated).copied().unwrap_or(0),
        got.commitments_with_findings,
        after.commitments_with_findings
    );
    let full = w.epi()?.full();
    println!(
        "  after the amendment, incremental assessment still equals from-scratch: {}",
        full.iter().all(|(p, s)| w.epi().is_ok_and(|e| e.standing(p) == *s))
    );
    Ok(got.refuted)
}

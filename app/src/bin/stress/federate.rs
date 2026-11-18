//! Several histories as separately bounded organisations in one world.

use crate::common::*;
use rigor::gitlog::{conclusions, federation, found_in, oracle, Commit, GitError, Ingest};
use rigor::model::ChangeId;
use rigor::waist::Name;
use std::collections::BTreeSet;
use std::time::Instant;

pub(crate) fn federate(histories: &[(Name, Vec<Commit>)], solo_results: &[BTreeSet<ChangeId>]) -> Result<(), GitError> {
    println!("\n================ FEDERATION: {} organisations in one world, interleaved by time", histories.len());
    let names: Vec<Name> = histories.iter().map(|(n, _)| n.clone()).collect();
    let mut w = federation(&names)?;
    let mut ingests: Vec<Ingest> = Vec::new();
    for n in &names {
        w.scope = vec![n.clone()];
        found_in(&mut w)?;
        ingests.push(self_tuning());
    }
    // Merge the histories by commit time, keeping each one's own order.
    let mut order: Vec<(u64, usize, usize)> = Vec::new();
    for (h, (_, commits)) in histories.iter().enumerate() {
        let mut clock = 0;
        for (i, c) in commits.iter().enumerate() {
            clock = clock.max(c.committed.0);
            order.push((clock, h, i));
        }
    }
    order.sort_unstable();
    let t = Instant::now();
    let mut chain_ok = true;
    for (n, (_, h, i)) in order.into_iter().enumerate() {
        if let (Some((name, commits)), Some(ingest)) = (histories.get(h), ingests.get_mut(h)) {
            if let Some(c) = commits.get(i) {
                w.scope = vec![name.clone()];
                ingest.commit(&mut w, c)?;
            }
        }
        // Verify the retained stretch of the chain, then let it go: memory stays bounded.
        if n % 5_000 == 4_999 {
            chain_ok = chain_ok && w.rt.log().verify().is_ok();
            w.rt.compact();
        }
    }
    for (name, ingest) in names.iter().zip(&mut ingests) {
        w.scope = vec![name.clone()];
        ingest.close(&mut w)?;
    }
    let took = secs(t);
    let events = w.rt.log().next_seq();
    println!("ingested in {took:.2}s: {events} events in one log, {:.0} events/s", f64::from(u32::try_from(events).unwrap_or(u32::MAX)) / took);
    for ((name, commits), alone) in histories.iter().zip(solo_results) {
        w.scope = vec![name.clone()];
        let got = conclusions(&w)?;
        println!(
            "  {name}: refuted {} — equals oracle: {}; equals the same organisation run alone: {}",
            got.refuted.len(),
            got.refuted == oracle(commits),
            if alone.is_empty() { "not compared in this run".to_owned() } else { (got.refuted == *alone).to_string() }
        );
    }
    println!("  one hash chain over all of it verifies, stretch by stretch: {}", chain_ok && w.rt.log().verify().is_ok());
    println!("  entries held at the end: {} of {} (compacted as it went)", w.rt.log().entries().len(), w.rt.log().next_seq());
    Ok(())
}

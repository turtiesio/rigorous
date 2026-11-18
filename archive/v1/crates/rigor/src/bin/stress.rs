//! `cargo run --release --bin stress -- <bare git repository>...`
//! `cargo run --release --bin stress -- --export <file> <first> <count> <bare git repository>`
//!
//! Feeds real histories through the whole system and reports, in aggregate, what the data
//! contained, what the system concluded, whether an independent computation agrees, whether
//! the system's own invariants held, and what it cost. With several repositories it also runs
//! them as separately bounded organisations inside ONE world, interleaved by time.

use rigor::adapters::{resume, save_snapshot};
use rigor::gitlog::{
    conclusions, export, federation, found, found_in, oracle, read, Commit, GitError, Ingest, GOVERN_PROJECT, GOVERN_REGISTRAR, PROJECT, REGISTRAR,
};
use rigor::kernel::{Name, Runtime};
use rigor::model::{Act, Amendment, CapId, ChangeId, Event, Method, Rank};
use rigor::world::{Error, World};
use std::collections::BTreeSet;
use std::path::PathBuf;
use std::time::Instant;

fn secs(t: Instant) -> f64 {
    t.elapsed().as_secs_f64()
}

/// The period comes from the project's rules, and the project approves what its reflector proposes.
fn self_tuning() -> Ingest {
    let mut i = Ingest::by_the_rules();
    i.approve_reflector = true;
    i
}

fn ingest_all(w: &mut World, commits: &[Commit]) -> Result<Ingest, Error> {
    let mut ingest = self_tuning();
    for c in commits {
        ingest.commit(w, c)?;
    }
    ingest.close(w)?;
    Ok(ingest)
}

fn solo(name: &str, commits: &[Commit]) -> Result<BTreeSet<ChangeId>, GitError> {
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

fn federate(histories: &[(Name, Vec<Commit>)], solo_results: &[BTreeSet<ChangeId>]) -> Result<(), GitError> {
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

fn main() -> Result<(), GitError> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.first().map(String::as_str) == Some("--export") {
        let [_, file, first, count, repo] = args.as_slice() else {
            eprintln!("usage: stress --export <file> <first> <count> <bare git repository>");
            return Ok(());
        };
        let commits = read(&PathBuf::from(repo), None)?;
        let first = first.parse::<usize>().unwrap_or(0);
        let count = count.parse::<usize>().unwrap_or(0);
        let window: Vec<Commit> = commits.into_iter().skip(first).take(count).collect();
        std::fs::write(file, export(&window)).map_err(GitError::Io)?;
        println!("wrote {} commits to {file}", window.len());
        return Ok(());
    }
    if args.first().map(String::as_str) == Some("--forge") {
        // stress --forge <directory of hourly .json.gz archive files | reduced file> [--export <reduced file> <max>]
        let Some(source) = args.get(1) else {
            eprintln!("usage: stress --forge <directory of archive files, or a reduced file> [--export <file> <max happenings>]");
            return Ok(());
        };
        let path = PathBuf::from(source);
        let mut all = Vec::new();
        let t = Instant::now();
        if path.is_dir() {
            let mut files: Vec<PathBuf> = std::fs::read_dir(&path)
                .map_err(GitError::Io)?
                .filter_map(|e| e.ok().map(|e| e.path()))
                .filter(|p| p.extension().is_some_and(|e| e == "gz"))
                .collect();
            // Hourly files are named YYYY-MM-DD-H: order by date, then hour as a number.
            files.sort_by_key(|p| {
                let stem = p.file_name().and_then(|s| s.to_str()).unwrap_or("").trim_end_matches(".json.gz").to_owned();
                let (date, hour) = stem.rsplit_once('-').map_or((stem.clone(), 0), |(d, h)| (d.to_owned(), h.parse::<u32>().unwrap_or(0)));
                (date, hour)
            });
            for f in &files {
                let _ = rigor::forge::read_archive(f, &mut all);
            }
            println!("read {} archive files in {:.1}s", files.len(), secs(t));
        } else {
            all = rigor::forge::import(&std::fs::read_to_string(&path).map_err(GitError::Io)?).map_err(|_| GitError::Malformed { record: 0 })?;
        }
        all.sort_by_key(|h| h.at);
        if let Some(i) = args.iter().position(|a| a == "--export") {
            if let (Some(out), Some(max)) = (args.get(i + 1), args.get(i + 2).and_then(|s| s.parse::<usize>().ok())) {
                // Keep whole stories: the first `max` happenings about changes that were reviewed.
                let reviewed: BTreeSet<_> = all.iter().filter(|h| matches!(h.what, rigor::forge::What::Reviewed(_))).map(|h| h.change.clone()).collect();
                let kept: Vec<_> = all.iter().filter(|h| reviewed.contains(&h.change)).take(max).cloned().collect();
                std::fs::write(out, rigor::forge::export(&kept)).map_err(GitError::Io)?;
                println!("wrote {} happenings to {out}", kept.len());
            }
        }
        let mut w = rigor::forge::found()?;
        let mut ingest = rigor::forge::Ingest::default();
        let t = Instant::now();
        for (n, h) in all.iter().enumerate() {
            ingest.happen(&mut w, h)?;
            if n % 20_000 == 19_999 {
                w.rt.compact();
            }
        }
        let took = secs(t);
        let events = w.rt.log().next_seq();
        println!("================ real forge events: {} happenings", all.len());
        println!("ingested in {took:.2}s: {events} events, {:.0} events/s\n{:#?}", f64::from(u32::try_from(events).unwrap_or(u32::MAX)) / took, ingest.stats);
        let expected = rigor::forge::oracle(&all);
        let got = rigor::forge::conclusions(&w, &expected)?;
        let mut tally: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
        for a in got.values() {
            *tally.entry(format!("{a:?}")).or_default() += 1;
        }
        println!("changes reviewed in the window: {}; as the system assesses them: {tally:?}", expected.len());
        println!("every assessment equals the independent oracle: {}", got == expected);
        let merged_unapproved = ingest.merged().iter().filter(|c| got.get(*c).is_none_or(|a| *a != rigor::model::Assessment::Accepted)).count();
        println!("merged in the window: {}; of those, with no approval standing on record in the window: {merged_unapproved}", ingest.merged().len());
        let full = w.epi()?.full();
        println!(
            "incremental assessment equals from-scratch over {} propositions: {}",
            full.len(),
            full.iter().all(|(p, s)| w.epi().is_ok_and(|e| e.standing(p) == *s))
        );
        return Ok(());
    }
    if args.first().map(String::as_str) == Some("--registry") {
        let Some(file) = args.get(1) else {
            eprintln!("usage: stress --registry <index records, one JSON object per line>");
            return Ok(());
        };
        let text = std::fs::read_to_string(file).map_err(GitError::Io)?;
        let (all, skipped) = rigor::registry::parse(&text).map_err(|_| GitError::Malformed { record: 0 })?;
        let mut w = rigor::registry::found()?;
        let t = Instant::now();
        let stats = rigor::registry::ingest(&mut w, &all)?;
        let took = secs(t);
        println!("================ real package registry records ({skipped} pre-releases or unusable names skipped)");
        println!("ingested in {took:.2}s: {} events\n{stats:#?}", w.rt.log().next_seq());
        let (fit, by_status) = rigor::registry::conclusions(&w)?;
        let expected = rigor::registry::oracle(&all);
        let not_yanked = all.iter().filter(|p| !p.yanked).count();
        println!(
            "releases assessed fit: {} of {} not yanked ({} lost their support with a dependency)",
            fit.len(),
            not_yanked,
            not_yanked.saturating_sub(fit.len())
        );
        println!("commitments by status: {by_status:?}");
        println!("fit set equals independent oracle ({}): {}", expected.len(), fit == expected);
        let full = w.epi()?.full();
        println!(
            "incremental assessment equals from-scratch over {} propositions: {}",
            full.len(),
            full.iter().all(|(p, s)| w.epi().is_ok_and(|e| e.standing(p) == *s))
        );
        println!("log hash chain verifies: {}", w.rt.log().verify().is_ok());
        return Ok(());
    }
    if args.first().map(String::as_str) == Some("--chain") {
        let Some(file) = args.get(1) else {
            eprintln!("usage: stress --chain <block file> [--raw] [--export <reduced file>]");
            return Ok(());
        };
        let text = std::fs::read_to_string(file).map_err(GitError::Io)?;
        let blocks = rigor::chain::parse(&text, args.iter().any(|a| a == "--raw")).map_err(|_| GitError::Malformed { record: 0 })?;
        if let Some(i) = args.iter().position(|a| a == "--export") {
            if let Some(out) = args.get(i + 1) {
                std::fs::write(out, rigor::chain::export(&blocks)).map_err(GitError::Io)?;
            }
        }
        let mut w = rigor::chain::found(&blocks)?;
        let mut settle = rigor::chain::Settle::default();
        let t = Instant::now();
        for b in &blocks {
            settle.block(&mut w, b)?;
        }
        let took = secs(t);
        println!("================ real Bitcoin blocks {}..={}", blocks.first().map_or(0, |b| b.height), blocks.last().map_or(0, |b| b.height));
        println!("settled in {took:.2}s: {} events\n{:#?}", w.rt.log().next_seq(), settle.stats);
        println!("{:#?}", rigor::chain::check(&w, &blocks)?);
        println!("log hash chain verifies: {}", w.rt.log().verify().is_ok());
        let replayed = Runtime::replay(Box::new(rigor::org::org(Vec::new()).map_err(Error::Fault)?), &w.rt.stimuli()).map_err(Error::Fault)?;
        println!("replay from stimuli reproduces the log exactly: {}", replayed.log().head() == w.rt.log().head());
        return Ok(());
    }
    if args.is_empty() {
        eprintln!("usage: stress <bare git repository>...");
        return Ok(());
    }
    let only_federation = args.first().map(String::as_str) == Some("--federate-only");
    let mut histories = Vec::new();
    let mut results = Vec::new();
    for (i, repo) in args.iter().filter(|a| !a.starts_with("--")).enumerate() {
        let name = Name::new(&format!("org{i}")).map_err(GitError::Name)?;
        let commits = read(&PathBuf::from(repo), None)?;
        results.push(if only_federation { BTreeSet::new() } else { solo(repo, &commits)? });
        histories.push((name, commits));
    }
    if histories.len() > 1 {
        federate(&histories, &results)?;
    }
    Ok(())
}

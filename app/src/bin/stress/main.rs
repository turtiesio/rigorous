//! `cargo run --release --bin stress -- <bare git repository>...`
//! `cargo run --release --bin stress -- --export <file> <first> <count> <bare git repository>`
//!
//! Feeds real histories through the whole system and reports, in aggregate, what the data
//! contained, what the system concluded, whether an independent computation agrees, whether
//! the system's own invariants held, and what it cost. With several repositories it also runs
//! them as separately bounded organisations inside ONE world, interleaved by time.

mod common;
mod federate;
mod solo;

use crate::common::*;
use crate::federate::*;
use crate::solo::*;
use rigor::gitlog::{export, read, Commit, GitError};
use rigor::waist::{Name, Runtime};
use rigor::world::Error;
use std::collections::BTreeSet;
use std::path::PathBuf;
use std::time::Instant;

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

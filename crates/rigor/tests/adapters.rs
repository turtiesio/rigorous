//! Continuous conformance against real files and a real subprocess.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::indexing_slicing)]

use rigor::adapters::{restore, run_check, save, AdapterError, ArtifactWatcher};
use rigor::kernel::LogError;
use rigor::model::{About, Act, ArtifactId, Assessment, Condition, Method, Offer, Prop, Requirement, Status, Tag, Time};
use rigor::scenario::{attest, root, rules, run, world, Cast, ALICE, CI, MONITOR, VENDOR};
use std::collections::BTreeSet;
use std::path::PathBuf;
use std::process::Command;

fn scratch(name: &str) -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("{name}-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

const GOOD: &str = "def price(qty):\n    return qty * 3\n";
const BAD: &str = "def price(qty):\n    return qty * 4\n";

#[test]
fn evidence_is_bound_to_the_artifact_it_was_about() {
    let dir = scratch("conformance");
    let file = dir.join("impl.py");
    std::fs::write(&file, GOOD).unwrap();
    let artifact = ArtifactId::lit("impl");
    let r1 = Prop::Conforms(Requirement::lit("r1"));
    // The "test suite": a real process whose exit status depends on the real file.
    let check = || {
        let mut c = Command::new("grep");
        let _ = c.arg("-q").arg("return qty \\* 3").arg(&file);
        c
    };
    let mut w = world(Cast::default(), rules()).unwrap();
    let mut watcher = ArtifactWatcher::new([(artifact.clone(), file.clone())]);
    let offer = Offer {
        creditor: ALICE,
        ant: None,
        cons: Condition::Holds(r1.clone()),
        deadline: Time(50),
        tags: BTreeSet::from([Tag::Conformance]),
        parent: None,
        terms: None,
    };
    let _ = w.act(&VENDOR, Act::Commit(offer)).unwrap();
    let _ = w.act(&VENDOR, Act::Claim { prop: r1.clone() }).unwrap();
    let status = |w: &rigor::world::World| w.one(&VENDOR, Tag::Conformance).unwrap().status.unwrap();
    let assessed = |w: &rigor::world::World| w.epi().unwrap().assessment(&r1);
    assert_eq!(status(&w), Status::Detached); // a claim alone is nothing

    assert!(run_check(&mut w, &mut watcher, &mut check(), &r1, std::slice::from_ref(&artifact), &CI).unwrap());
    assert_eq!((assessed(&w), status(&w)), (Assessment::Accepted, Status::Satisfied));

    std::fs::write(&file, BAD).unwrap(); // the implementation changes
    assert_eq!(watcher.scan(&mut w).unwrap(), vec![artifact.clone()]);
    assert_eq!((assessed(&w), status(&w)), (Assessment::Unsupported, Status::Detached)); // the old pass is stale, not false

    assert!(!run_check(&mut w, &mut watcher, &mut check(), &r1, std::slice::from_ref(&artifact), &CI).unwrap());
    assert_eq!(assessed(&w), Assessment::Refuted);

    std::fs::write(&file, GOOD).unwrap(); // reverting restores the old evidence
    let _ = watcher.scan(&mut w).unwrap();
    assert_eq!((assessed(&w), status(&w)), (Assessment::Accepted, Status::Satisfied));

    // A passed test is evidence, not proof: an independent observation of production outranks it.
    let _ = w.act(&MONITOR, attest(About::Rebuts(r1.clone()), Method::Probe, "prod returns 8 for qty=2")).unwrap();
    assert_eq!((assessed(&w), status(&w)), (Assessment::Refuted, Status::Detached));

    // Evidence about an artifact nobody is watching cannot be produced.
    let unknown = ArtifactId::lit("other");
    assert!(matches!(run_check(&mut w, &mut watcher, &mut check(), &r1, &[unknown], &CI), Err(AdapterError::UnknownArtifact(_))));
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn log_persistence_and_tamper_detection() {
    let dir = scratch("log");
    let path = dir.join("log.jsonl");
    let (w, _) = run(Cast::default(), true, |_| {}).unwrap();
    save(&w, &path).unwrap();
    let back = restore(root(Cast::default()).unwrap(), &path).unwrap();
    assert_eq!(back.rt.digest(), w.rt.digest());

    // Rewrite history: change the amount of the first recorded transfer from 30 to 3.
    let text = std::fs::read_to_string(&path).unwrap();
    let needle = "\"amount\":30";
    let mut lines: Vec<String> = text.lines().map(str::to_owned).collect();
    let i = lines.iter().position(|l| l.contains("\"Transferred\"") && l.contains(needle)).unwrap();
    lines[i] = lines[i].replace(needle, "\"amount\":3");
    std::fs::write(&path, lines.join("\n") + "\n").unwrap();
    assert!(matches!(restore(root(Cast::default()).unwrap(), &path), Err(AdapterError::Log(LogError::BrokenChain { .. }))));

    // Something that is not an event at all is refused when parsed, before any checking.
    let bogus = format!("{{\"rigor_log_schema\":{}}}\n", rigor::kernel::SCHEMA)
        + "{\"seq\":0,\"id\":1,\"actor\":[\"x\"],\"cause\":null,\"src\":[\"x\"],\"event\":{\"Frobnicate\":1}}\n";
    std::fs::write(&path, bogus).unwrap();
    assert!(matches!(restore(root(Cast::default()).unwrap(), &path), Err(AdapterError::Log(LogError::Parse { line: 2, .. }))));
    // A file from a newer or older format is not guessed at: it must be migrated.
    std::fs::write(&path, "{\"rigor_log_schema\":99}\n").unwrap();
    assert!(matches!(restore(root(Cast::default()).unwrap(), &path), Err(AdapterError::Log(LogError::Schema { found: 99 }))));
    std::fs::remove_dir_all(dir).unwrap();
}

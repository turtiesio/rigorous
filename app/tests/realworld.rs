//! What handling real data required: members who arrive at run time, obligations to keep
//! something true, a real git history as input, and the project stewarding itself.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::indexing_slicing)]

use rigor::adapters::{migrate, restore, resume, save, save_snapshot, AdapterError};
use rigor::explore::reach;
use rigor::gitlog::{self, conclusions, federation, found_in, import, oracle, Commit, Ingest, ADMISSIONS, PROJECT, REGISTRAR};
use rigor::model::{
    About, Act, ActorId, Assessment, CapId, ChangeId, Condition, DenyReason, Event, EvidenceId, Finding, Member, MemberKind, Method, NormError, Offer, Prop,
    Requirement, Status, Tag, Time, Version,
};
use rigor::org::open_kinds;
use rigor::scenario::attest;
use rigor::steward::{Check, Outcome, Steward, BACKLOG, MAINTAINER};
use rigor::units::norms::{Norms, NORMS};
use rigor::units::reflector::Reflector;
use rigor::waist::LogError;
use rigor::waist::{Cell, Name, Par, Runtime, Unit};
use rigor::world::World;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;

const NEWCOMER: ActorId = ActorId::lit("newcomer");

fn member(id: ActorId) -> Member {
    Member { id, kind: MemberKind::Human, principal: None, role: None }
}
fn denial(entries: &[rigor::waist::Entry<Event>]) -> Option<DenyReason> {
    rigor::scenario::denial(entries)
}

// ---------------------------------------------------------------- admission

#[test]
fn members_can_be_admitted_at_run_time_only_by_charter() {
    let mut w = gitlog::found().unwrap();
    let x = Prop::Fact(Name::lit("x"));
    // Before admission, the newcomer's attempts do not count.
    assert_eq!(denial(&w.act(&NEWCOMER, Act::Claim { prop: x.clone() }).unwrap()), Some(DenyReason::NotAMember));
    // Membership does not confer the power to admit; nor can one admit oneself.
    assert_eq!(denial(&w.act(&PROJECT, Act::Admit(member(NEWCOMER))).unwrap()), Some(DenyReason::ActionNotGranted));
    assert_eq!(denial(&w.act(&NEWCOMER, Act::Admit(member(NEWCOMER))).unwrap()), Some(DenyReason::NotAMember));
    // Nor can someone else's charter be borrowed.
    assert_eq!(denial(&w.act_with(&PROJECT, &CapId::Charter(ADMISSIONS), Act::Admit(member(NEWCOMER))).unwrap()), Some(DenyReason::NotHolder));
    let admitted = w.act_with(&REGISTRAR, &CapId::Charter(ADMISSIONS), Act::Admit(member(NEWCOMER))).unwrap();
    assert!(admitted.iter().any(|e| matches!(&e.event, Event::Member(m) if m.id == NEWCOMER)));
    // Now the same attempt counts, and the newcomer owns an account like any founding member.
    assert_eq!(denial(&w.act(&NEWCOMER, Act::Claim { prop: x }).unwrap()), None);
    assert!(w.inst().unwrap().valid(&CapId::Own(NEWCOMER)));
    assert_eq!(denial(&w.act_with(&REGISTRAR, &CapId::Charter(ADMISSIONS), Act::Admit(member(NEWCOMER))).unwrap()), Some(DenyReason::AlreadyAMember));
    assert!(w.inst().unwrap().audit().is_empty());
}

// ---------------------------------------------------------------- maintenance obligations

fn keep(w: &mut World, who: &ActorId, p: &Prop, until: u64) -> rigor::model::CommitmentId {
    let o = Offer {
        creditor: PROJECT,
        ant: None,
        cons: Condition::Unrefuted(p.clone()),
        deadline: Time(until),
        tags: BTreeSet::from([Tag::Change]),
        parent: None,
        terms: None,
    };
    w.act(who, Act::Commit(o))
        .unwrap()
        .iter()
        .find_map(|e| match &e.event {
            Event::NormCreated(c) => Some(c.id),
            _ => None,
        })
        .unwrap()
}
fn recorded(entries: &[rigor::waist::Entry<Event>]) -> EvidenceId {
    entries
        .iter()
        .find_map(|e| match &e.event {
            Event::EvidenceRecorded(ev) => Some(ev.id),
            _ => None,
        })
        .unwrap()
}
fn sound(n: &'static str) -> Prop {
    Prop::Sound(ChangeId(Name::new(n).unwrap()))
}

#[test]
fn a_maintenance_commitment_is_about_not_being_refuted() {
    let mut w = gitlog::found().unwrap();
    let _ = w.act_with(&REGISTRAR, &CapId::Charter(ADMISSIONS), Act::Admit(member(NEWCOMER))).unwrap();
    let status = |w: &World, id| w.norms().unwrap().status(&id).unwrap();

    // Left alone for the whole period, it is discharged -- with no evidence at all in its favour.
    let (a, b, c) = (sound("aaaa"), sound("bbbb"), sound("cccc"));
    let (ka, kb, kc) = (keep(&mut w, &NEWCOMER, &a, 10), keep(&mut w, &NEWCOMER, &b, 10), keep(&mut w, &NEWCOMER, &c, 10));
    assert_eq!(status(&w, ka), Status::Detached);

    // Refuted within the period: violated, and judged.
    let _ = w.tick(5).unwrap();
    let rebuttal = recorded(&w.act(&PROJECT, attest(About::Rebuts(b.clone()), Method::Reversal, "undo b")).unwrap());
    assert_eq!(status(&w, kb), Status::Violated);
    assert!(w.adj().unwrap().findings(&kb).contains(&(Finding::Accountable, NEWCOMER)));

    // The refutation is itself undone: the violation and its verdict are withdrawn.
    let _ = w.act(&PROJECT, attest(About::Undercuts(rebuttal), Method::Reversal, "undo the undo")).unwrap();
    assert_eq!(status(&w, kb), Status::Detached);
    assert!(w.adj().unwrap().findings(&kb).is_empty());

    let _ = w.tick(11).unwrap();
    assert_eq!([status(&w, ka), status(&w, kb), status(&w, kc)], [Status::Satisfied; 3]);

    // Refuted only after the period: the duty was already discharged and stays so.
    let _ = w.act(&PROJECT, attest(About::Rebuts(c.clone()), Method::Reversal, "too late")).unwrap();
    assert_eq!(w.epi().unwrap().assessment(&c), Assessment::Refuted);
    assert_eq!(status(&w, kc), Status::Satisfied);

    // A maintenance condition cannot be what triggers a commitment.
    let bad = Offer {
        creditor: PROJECT,
        ant: Some(Condition::Unrefuted(a.clone())),
        cons: Condition::Holds(a),
        deadline: Time(20),
        tags: BTreeSet::new(),
        parent: None,
        terms: None,
    };
    let refused = w.act(&NEWCOMER, Act::Commit(bad)).unwrap();
    assert!(refused.iter().any(|e| matches!(&e.event, Event::NormRejected { reason: NormError::MaintenanceAsAntecedent, .. })));
}

#[derive(Debug, PartialEq, Eq)]
enum Broke {
    ViolatedWithoutRefutation,
    DischargedEarly,
    PendingAfterDeadline,
}

#[test]
fn model_check_maintenance_commitment() {
    // Exhaustive, to a fixed point: one maintenance commitment under every interleaving of time
    // and (re)assessment.
    let env = Name::lit("env");
    let d = ActorId::lit("d");
    let p = Prop::Fact(Name::lit("p"));
    let norms: Box<dyn Unit<Event>> = Box::new(Cell(Norms::default()));
    let mut rt: Runtime<Event> = Runtime::new(Box::new(Par::new(vec![norms], &open_kinds()).unwrap()));
    let o = Offer { creditor: PROJECT, ant: None, cons: Condition::Unrefuted(p.clone()), deadline: Time(2), tags: BTreeSet::new(), parent: None, terms: None };
    let _ = rt.inject_unchecked(rigor::waist::Path::of(&env), Event::Ok { by: d.clone(), via: CapId::Base(d), act: Act::Commit(o) }).unwrap();
    let id = rt.leaf::<Norms>(&[NORMS]).unwrap().records().next().unwrap().commitment.id;
    let assessed = |a: Assessment| {
        (env.clone(), Event::Assessed { prop: p.clone(), assessment: a, since: None, basis: Vec::new(), was: Assessment::Unsupported, rules: Version(1) })
    };
    let mut alphabet: Vec<(Name, Event)> = (1..=3).map(|t| (env.clone(), Event::Tick(Time(t)))).collect();
    alphabet.extend([Assessment::Refuted, Assessment::Contested, Assessment::Unsupported, Assessment::Accepted].map(assessed));
    let mut seen = BTreeSet::new();
    let res = reach(&rt, &alphabet, 12, |_, action, _, after| {
        let n = after.leaf::<Norms>(&[NORMS]).unwrap();
        let s = n.status(&id).unwrap();
        // Immediately after any assessment other than "refuted", it cannot be violated.
        if matches!(&action.1, Event::Assessed { assessment, .. } if *assessment != Assessment::Refuted) && s == Status::Violated {
            return Some(Broke::ViolatedWithoutRefutation);
        }
        if s == Status::Satisfied && n.now() <= Time(2) {
            return Some(Broke::DischargedEarly);
        }
        if n.now() > Time(2) && matches!(s, Status::Detached | Status::Conditional) {
            return Some(Broke::PendingAfterDeadline);
        }
        let _ = seen.insert(s);
        None
    })
    .unwrap();
    assert_eq!(res.violation.map(|(why, _)| why), None);
    assert!(res.complete);
    assert_eq!(seen, BTreeSet::from([Status::Detached, Status::Violated, Status::Satisfied]));
}

// ---------------------------------------------------------------- a real git history

fn git(dir: &Path, who: (&str, &str), committer: (&str, &str), args: &[&str]) -> String {
    let out = Command::new("git")
        .current_dir(dir)
        .args(["-c", "commit.gpgsign=false", "-c", "core.hooksPath=/dev/null"])
        .args(args)
        .env("GIT_AUTHOR_NAME", who.0)
        .env("GIT_AUTHOR_EMAIL", who.1)
        .env("GIT_COMMITTER_NAME", committer.0)
        .env("GIT_COMMITTER_EMAIL", committer.1)
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_CONFIG_SYSTEM", "/dev/null")
        .output()
        .unwrap();
    assert!(out.status.success(), "git {args:?}: {}", String::from_utf8_lossy(&out.stderr));
    String::from_utf8_lossy(&out.stdout).trim().to_owned()
}

fn scratch(name: &str) -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn a_real_git_history_with_reverts_and_reverts_of_reverts() {
    let dir = scratch("history");
    let (ann, bob, bot) = (("Ann Åse", "ann@example.org"), ("Bob", "BOB@example.org"), ("merge-bot[bot]", "bot@example.org"));
    let _ = git(&dir, ann, ann, &["init", "-q", "-b", "main"]);
    let change = |who, committer, text: &str| {
        std::fs::write(dir.join("f.txt"), text).unwrap();
        let _ = git(&dir, who, committer, &["add", "f.txt"]);
        let _ = git(&dir, who, committer, &["commit", "-q", "-m", text]);
        git(&dir, who, committer, &["rev-parse", "HEAD"])
    };
    let revert = |who, what: &str| {
        let _ = git(&dir, who, who, &["revert", "--no-edit", what]);
        git(&dir, who, who, &["rev-parse", "HEAD"])
    };
    let c1 = change(ann, ann, "one"); // self-committed: no independent acknowledgement
    let c2 = change(bob, bot, "two"); // Bob's first change, committed by an automated account: his sponsor
    let r1 = revert(ann, &c2); // Ann reverts Bob's change...
    let r2 = revert(bob, &r1); // ...and Bob reverts the revert: c2 stands again, r1 does not.
    let c3 = change(bob, bob, "three");
    let r3 = revert(bob, &c3); // a self-revert

    let commits = gitlog::read(&dir.join(".git"), None).unwrap();
    assert_eq!(commits.iter().map(|c| c.id.to_string()).collect::<Vec<_>>(), vec![c1.clone(), c2.clone(), r1.clone(), r2.clone(), c3.clone(), r3.clone()]);
    // Identity is by address, case-insensitively, and pseudonymous: no name or address enters the model.
    assert_eq!(commits[1].author.id, commits[4].author.id);
    assert!(commits[1].committer.automated && !commits[1].author.automated);
    let all = format!("{commits:?}");
    assert!(!all.contains("example.org") && !all.contains("Ann") && !all.contains("Bob"));
    assert_eq!(import(&gitlog::export(&commits)).unwrap(), commits); // the reduced form loses nothing the model uses

    let mut w = gitlog::found().unwrap();
    let mut ingest = Ingest::new(1_000_000);
    for c in &commits {
        ingest.commit(&mut w, c).unwrap();
    }
    ingest.close(&mut w).unwrap();
    let id = |h: &str| ChangeId(Name::new(h).unwrap());
    let got = conclusions(&w).unwrap();
    assert_eq!(got.refuted, BTreeSet::from([id(&r1), id(&c3)]));
    assert_eq!(got.refuted, oracle(&commits)); // the independent computation agrees
    let s = &ingest.stats;
    assert_eq!((s.commits, s.members_admitted, s.automated_members, s.independently_acknowledged), (6, 3, 1, 1));
    assert_eq!((s.reverts_declared, s.reverts_unresolved, s.reverts_of_reverts, s.self_reverts), (3, 0, 1, 1));
    // Bob was admitted by his sponsor, who held only a delegated, non-delegable power to admit.
    assert_eq!((s.sponsors_empowered, s.admitted_by_sponsor, s.denied), (1, 1, 0));
    let bob_id = &commits[1].author.id;
    let admitted_bob = w.rt.log().entries().iter().find_map(|e| match &e.event {
        Event::Ok { by, via, act: Act::Admit(m) } if m.id == *bob_id => Some((by.clone(), via.clone())),
        _ => None,
    });
    let (sponsor, via) = admitted_bob.unwrap();
    assert_eq!(sponsor, commits[1].committer.id);
    assert!(matches!(via, CapId::Delegated(_)) && w.inst().unwrap().lineage(&via).len() == 2);
    assert_eq!((got.by_status.get(&Status::Violated), got.by_status.get(&Status::Satisfied)), (Some(&2), Some(&4)));
    assert!(w.rt.log().verify().is_ok());
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn messy_history_text_is_typed_once_at_the_boundary() {
    let rec = |hash: &str, at: &str, body: &str| format!("{hash}\u{1f}a@x\u{1f}A\u{1f}a@x\u{1f}A\u{1f}{at}\u{1f}{body}\u{1e}\n");
    let h1 = "a".repeat(40);
    let h2 = "b".repeat(40);
    let text = rec(&h1, "200", "first")
        + &rec(&h2, "100", &format!("Revert\n\nThis reverts commit {}.\nThis reverts commit zzzz.\nThis reverts commit abc.", "a".repeat(12)));
    let commits = gitlog::parse(&text).unwrap();
    assert_eq!(commits[1].reverts, vec!["a".repeat(12)]); // abbreviated is kept; non-hashes and too-short ones are not
    assert!(commits[1].committed < commits[0].committed); // real clocks go backwards
    assert!(matches!(gitlog::parse("only\u{1f}three\u{1f}fields\u{1e}"), Err(gitlog::GitError::Malformed { record: 0 })));
    assert!(matches!(gitlog::parse(&rec("NOT-A-HASH!", "1", "x")), Err(gitlog::GitError::Name(_))));
    assert!(matches!(gitlog::parse(&rec(&h1, "yesterday", "x")), Err(gitlog::GitError::Malformed { record: 0 })));
    assert!(matches!(import("too few fields"), Err(gitlog::GitError::Malformed { record: 0 })));
    // An abbreviated revert is resolved against earlier history. A backdated commit does not move
    // the clock back, and its period runs from arrival: it is accepted, not refused.
    let mut w = gitlog::found().unwrap();
    let mut ingest = Ingest::new(50);
    for c in &commits {
        ingest.commit(&mut w, c).unwrap();
    }
    assert_eq!((ingest.stats.clock_regressions, ingest.stats.backdated, ingest.stats.reverts_unresolved, ingest.stats.denied), (1, 1, 0, 0));
    assert_eq!(conclusions(&w).unwrap().refuted.len(), 1);
    assert_eq!(w.norms().unwrap().records().count(), 2);
}

#[test]
fn judging_costs_nothing_until_something_is_violated_whatever_the_membership() {
    // The scaling defect the first stress run found, pinned: with hundreds of members and
    // commitments, the adjudicator does no work at all while nothing is violated.
    let mut w = gitlog::found().unwrap();
    let mut ingest = Ingest::new(1_000_000);
    for i in 0..600u64 {
        let who = gitlog::parse(&format!("{:040x}\u{1f}u{i}@x\u{1f}U\u{1f}u{i}@x\u{1f}U\u{1f}{}\u{1f}m\u{1e}", i + 1, i + 1)).unwrap();
        ingest.commit(&mut w, &who[0]).unwrap();
    }
    assert_eq!(ingest.stats.members_admitted, 600);
    assert_eq!(w.adj().unwrap().work.0, 0);
}

// ---------------------------------------------------------------- real data, in the test suite

/// 4,000 consecutive real commits of the Node.js history (2011-2012), in reduced pseudonymous
/// form: 84 reverts that resolve inside the window, 6 of them reverts of reverts.
fn real_window() -> Vec<Commit> {
    import(include_str!("data/node-window.txt")).unwrap()
}

fn ingest_into(w: &mut World, commits: &[Commit]) -> Ingest {
    let mut ingest = Ingest::new(30 * 24 * 3600);
    for c in commits {
        ingest.commit(w, c).unwrap();
    }
    ingest
}

#[test]
fn four_thousand_real_commits_agree_with_the_oracle_and_keep_every_invariant() {
    let commits = real_window();
    let mut w = gitlog::found().unwrap();
    let mut ingest = ingest_into(&mut w, &commits);
    ingest.close(&mut w).unwrap();
    let got = conclusions(&w).unwrap();
    let s = &ingest.stats;
    assert_eq!((s.commits, s.reverts_of_reverts, s.denied), (4000, 6, 0));
    assert_eq!((s.members_admitted, s.admitted_by_sponsor, s.sponsors_empowered), (276, 270, 12));
    assert_eq!((s.reverts_declared, s.reverts_unresolved, s.self_reverts, s.independently_acknowledged), (85, 1, 28, 996));
    assert_eq!(got.refuted.len(), 69);
    assert_eq!(got.refuted, oracle(&commits));
    let epi = w.epi().unwrap();
    assert!(epi.full().iter().all(|(p, st)| epi.standing(p) == *st));
    assert!(w.inst().unwrap().audit().is_empty() && w.ledger().unwrap().conserved() && w.rt.log().verify().is_ok());
    let again = Runtime::replay(Box::new(gitlog::project_org().unwrap()), &w.rt.stimuli()).unwrap();
    assert_eq!(again.log().head(), w.rt.log().head());
}

#[test]
fn a_snapshot_resumes_exactly_and_memory_need_not_grow_with_history() {
    let commits = real_window();
    let (first, second) = commits.split_at(2500);
    let root = || -> Box<dyn Unit<Event>> { Box::new(gitlog::project_org().unwrap()) };
    // The uninterrupted world.
    let mut whole = gitlog::found().unwrap();
    let mut i1 = ingest_into(&mut whole, &commits);
    i1.close(&mut whole).unwrap();
    // Stop half-way, write a snapshot, start a new process' worth of state from the file, go on.
    let mut a = gitlog::found().unwrap();
    let mut i2 = ingest_into(&mut a, first);
    let dir = scratch("snapshot");
    let path = dir.join("state.snapshot");
    let snap = save_snapshot(&a, &path).unwrap();
    let mut b = resume(root(), &path).unwrap();
    assert_eq!(b.rt.log().entries().len(), 0, "a resumed world holds none of the earlier history");
    for c in second {
        i2.commit(&mut b, c).unwrap();
        b.rt.compact(); // ...and need not hold the later history either
    }
    i2.close(&mut b).unwrap();
    assert_eq!((b.rt.digest(), b.rt.log().head(), b.rt.log().next_seq()), (whole.rt.digest(), whole.rt.log().head(), whole.rt.log().next_seq()));
    assert_eq!(conclusions(&b).unwrap(), conclusions(&whole).unwrap());
    assert!(b.rt.log().entries().len() < 1_000 && whole.rt.log().entries().len() > 40_000);
    // A snapshot is checked for corruption and for fitting the units it is loaded into.
    let mut corrupt = snap.clone();
    corrupt.state[100] ^= 1;
    assert!(Runtime::resume(root(), &corrupt).is_err());
    assert!(Runtime::resume(rigor::scenario::root(rigor::scenario::Cast::default()).unwrap(), &snap).is_err());
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn nested_organisations_on_real_data_behave_exactly_as_they_do_alone() {
    // Two real histories (two halves of the window) as separately bounded organisations in one
    // world, interleaved commit by commit. Each concludes exactly what it concludes alone.
    let commits = real_window();
    let (left, right) = commits.split_at(2000);
    let names = [Name::lit("left"), Name::lit("right")];
    let mut w = federation(&names).unwrap();
    for n in &names {
        w.scope = vec![n.clone()];
        found_in(&mut w).unwrap();
    }
    let mut ingests = [Ingest::new(30 * 24 * 3600), Ingest::new(30 * 24 * 3600)];
    for i in 0..2000 {
        for (k, half) in [left, right].iter().enumerate() {
            w.scope = vec![names[k].clone()];
            ingests[k].commit(&mut w, &half[i]).unwrap();
        }
    }
    for (k, half) in [left, right].iter().enumerate() {
        w.scope = vec![names[k].clone()];
        ingests[k].close(&mut w).unwrap();
        let mut alone = gitlog::found().unwrap();
        let mut solo = ingest_into(&mut alone, half);
        solo.close(&mut alone).unwrap();
        assert_eq!(conclusions(&w).unwrap(), conclusions(&alone).unwrap());
        assert_eq!(conclusions(&w).unwrap().refuted, oracle(half));
        assert_eq!(ingests[k].stats, solo.stats);
    }
    assert!(w.rt.log().verify().is_ok());
    // Nothing is addressed to the root: every recorded act is inside one of the two boundaries.
    assert!(w.rt.log().entries().iter().all(|e| matches!(e.event, Event::Scoped { .. } | Event::Settle)));
}

// ---------------------------------------------------------------- carrying history across formats

/// A stored log as schema 1 wrote it: no header line and no end mark.
fn schema_1(current: &str) -> String {
    let lines: Vec<&str> = current.lines().collect();
    lines[1..lines.len() - 1].iter().map(|l| format!("{l}\n")).collect()
}

#[test]
fn a_log_in_an_older_format_is_verified_from_its_bytes_and_migrated() {
    let dir = scratch("migrate");
    let path = dir.join("old.jsonl");
    let (w, o) = rigor::scenario::run(rigor::scenario::Cast::default(), true, |_| {}).unwrap();
    save(&w, &path).unwrap();
    // Schema 1 is the same lines without the header and without the end mark (the format the
    // steward's first logs were in). (Its events here are in today's vocabulary; a real change
    // of shape is tested separately below.)
    let body = schema_1(&std::fs::read_to_string(&path).unwrap());
    std::fs::write(&path, &body).unwrap();
    let root = || rigor::scenario::root(rigor::scenario::Cast::default()).unwrap();
    assert!(matches!(restore(root(), &path), Err(AdapterError::Log(LogError::Schema { found: 1 }))));

    let m = migrate(root(), &path).unwrap();
    // The new log opens by recording what it continues from, and reaches the same judgments.
    assert!(matches!(m.rt.log().entries()[0].event, Event::Migrated { from_schema: 1, old_head, .. } if old_head == w.rt.log().head()));
    assert_eq!(m.statuses().unwrap(), w.statuses().unwrap());
    assert_eq!(m.adj().unwrap().findings(&o.c1), w.adj().unwrap().findings(&o.c1));
    assert_eq!(m.rt.digest(), w.rt.digest());

    // An old log that was edited is refused before anything in it is interpreted.
    std::fs::write(&path, body.replacen("\"amount\":30", "\"amount\":31", 1)).unwrap();
    assert!(matches!(migrate(root(), &path), Err(AdapterError::Log(LogError::BrokenChain { .. }))));
    // So is one in a vocabulary with no current form.
    std::fs::remove_dir_all(dir).unwrap();
}

// ---------------------------------------------------------------- the institution improving its own rules

#[test]
fn the_reflector_proposes_a_rule_change_from_the_record_and_cannot_enact_it_alone() {
    use rigor::model::{Amendment, Charter, CharterName, GovError, Rank};
    use rigor::scenario::{constitution, rules, ALICE, AUDITOR, CI, GOV_ALICE, MONITOR};
    const REFLECTOR: ActorId = ActorId::lit("reflector");
    const REFLECT: CharterName = CharterName::lit("reflect");
    let mut c = constitution(rules());
    c.members.push(member(REFLECTOR));
    c.charters.push(Charter { name: REFLECT, holder: REFLECTOR, actions: BTreeSet::from([rigor::model::ActionKind::Propose]), delegable: false });
    // Judge a method after 4 acceptances; propose when half or more were overturned.
    let reflector: Box<dyn Unit<Event>> = Box::new(Cell(Reflector::new(REFLECTOR, CapId::Charter(REFLECT), 4, 50, 1_000)));
    let mut w = World::new(Box::new(rigor::org::org(vec![reflector]).unwrap()));
    let _ = w.inject(&rigor::world::CONSTITUTION, Event::Genesis(Box::new(c))).unwrap();
    let p = |i: usize| Prop::Fact(Name::new(&format!("p{i}")).unwrap());
    let rank = |w: &World, m| w.gov().unwrap().rules().unwrap().evidence.ranks[&m];

    // A standing case that will show the effect: a probe for q against an equally... weaker test.
    let q = Prop::Fact(Name::lit("q"));
    let _ = w.act(&MONITOR, attest(About::Supports(q.clone()), Method::Probe, "probe says q")).unwrap();
    let _ = w.act(&CI, attest(About::Rebuts(q.clone()), Method::Test, "test says not q")).unwrap();
    assert_eq!(w.epi().unwrap().assessment(&q), Assessment::Accepted); // probe (3) outranks test (2)

    // Probes keep being relied on and keep being overturned by audits.
    let mut proposal = None;
    for i in 0..4 {
        let _ = w.act(&MONITOR, attest(About::Supports(p(i)), Method::Probe, "probe")).unwrap();
        if i >= 2 {
            let entries = w.act(&AUDITOR, attest(About::Rebuts(p(i)), Method::Audit, "audit")).unwrap();
            proposal = proposal.or(entries.iter().find_map(|e| match &e.event {
                Event::Proposed { id, by, .. } if *by == REFLECTOR => Some(*id),
                _ => None,
            }));
        }
    }
    let record = w.rt.leaf::<Reflector>(&[REFLECTOR.0]).unwrap().record()[&Method::Probe];
    assert_eq!((record.relied_on, record.overturned), (5, 2)); // q and p0..p3; two overturned... not yet half
    assert!(proposal.is_none());
    // A third acceptance is overturned: three of five. That crosses the threshold.
    let entries = w.act(&AUDITOR, attest(About::Rebuts(p(0)), Method::Audit, "audit")).unwrap();
    let id = entries
        .iter()
        .find_map(|e| match &e.event {
            Event::Proposed { id, by, .. } if *by == REFLECTOR => Some(*id),
            _ => None,
        })
        .unwrap();
    // It proposes once, not every time the record gets worse.
    let more = w.act(&AUDITOR, attest(About::Rebuts(p(1)), Method::Audit, "audit")).unwrap();
    assert!(!more.iter().any(|e| matches!(e.event, Event::Proposed { .. })));

    // It has proposed, by the ordinary procedure; nothing has changed, and it cannot approve its own proposal into force.
    assert_eq!((w.gov().unwrap().version(), rank(&w, Method::Probe)), (Version(1), Rank(3)));
    let alone = w.act_with(&REFLECTOR, &CapId::Charter(REFLECT), Act::Approve { id }).unwrap();
    assert_eq!(denial(&alone), Some(DenyReason::ActionNotGranted));
    // It cannot propose what the rules entrench, either.
    let entrenched = w.act_with(&REFLECTOR, &CapId::Charter(REFLECT), Act::Propose { amendments: vec![Amendment::SetSelfSupport(true)] }).unwrap();
    assert!(entrenched.iter().any(|e| matches!(&e.event, Event::GovRejected { reason: GovError::Entrenched(_), .. })));

    // Its having proposed counts for nothing towards the quorum: two holders of the power to
    // approve must approve. Then the rule set changes, and existing assessments are revised.
    let _ = w.act_with(&ALICE, &CapId::Charter(GOV_ALICE), Act::Approve { id }).unwrap();
    assert_eq!(w.gov().unwrap().version(), Version(1));
    let _ = w.act_with(&AUDITOR, &CapId::Charter(rigor::scenario::GOV_AUDITOR), Act::Approve { id }).unwrap();
    assert_eq!((w.gov().unwrap().version(), rank(&w, Method::Probe)), (Version(2), Rank(2)));
    assert_eq!(w.epi().unwrap().assessment(&q), Assessment::Contested); // the probe no longer outranks the test
                                                                        // The record of the re-ranked method starts afresh.
    assert!(w.rt.leaf::<Reflector>(&[REFLECTOR.0]).unwrap().record().get(&Method::Probe).is_none_or(|r| r.overturned == 0));
}

// ---------------------------------------------------------------- the project stewarding itself

fn fake_repo(name: &str) -> PathBuf {
    let root = scratch(name);
    std::fs::create_dir_all(root.join("waist")).unwrap();
    std::fs::create_dir_all(root.join("proofs")).unwrap();
    for f in ["waist/lib.rs", "proofs/t.lean", "Cargo.toml", "Cargo.lock", "rustfmt.toml"] {
        std::fs::write(root.join(f), f).unwrap();
    }
    root
}

#[test]
fn the_steward_keeps_an_honest_record_across_runs() {
    let root = fake_repo("steward");
    let log = root.join(".rigor/steward.jsonl");
    let all: BTreeSet<Check> = Check::ALL.into_iter().collect();

    // Day 100. No check is run (all skipped): nothing is established, and the steward says so.
    let mut s = Steward::open(&root, &log, 100).unwrap();
    let outcomes = s.run(100, false, &all).unwrap();
    assert!(outcomes.iter().all(|(_, o)| *o == Outcome::Skipped));
    assert!(!s.conformant(&BTreeSet::new()).unwrap(), "skipping a check is not passing it");
    assert_eq!(s.commitments(Tag::Task).unwrap().len(), BACKLOG.len());
    let _ = s.run(100, false, &all).unwrap(); // a second run the same day undertakes nothing twice
    assert_eq!(s.commitments(Tag::Task).unwrap().len(), BACKLOG.len());
    let events_day_100 = s.world.rt.log().next_seq();

    // Day 101, a new process: the history is replayed from disk, not remembered.
    let mut s = Steward::open(&root, &log, 101).unwrap();
    assert!(s.previous_log_rejected.is_none() && s.migrated_from.is_none());
    assert_eq!(s.world.rt.log().next_seq(), events_day_100);
    let _ = s.run(101, false, &all).unwrap();
    // Yesterday's undertakings were never evidenced: they are now violated, on the record, with
    // findings -- and each lapse has created a duty to put it right within a week.
    let upkeep = s.commitments(Tag::Upkeep).unwrap();
    let violated: Vec<_> = upkeep.iter().filter(|(_, st)| *st == Status::Violated).collect();
    assert_eq!((upkeep.len(), violated.len()), (8, 4));
    assert!(violated
        .iter()
        .all(|(c, _)| c.terms.deadline == Time(100) && s.world.adj().unwrap().findings(&c.id).contains(&(Finding::Accountable, MAINTAINER))));
    let duties = s.commitments(Tag::Remediation).unwrap();
    assert_eq!(duties.len(), 4);
    assert!(duties.iter().all(|(c, st)| *st == Status::Detached && c.terms.deadline == Time(108) && c.terms.debtor == MAINTAINER));

    // A roadmap item is closed only by the project's acceptance, not by the maintainer's say-so.
    let item = BACKLOG[0].requirement.clone();
    assert!(s.accept(item.clone(), 101).unwrap());
    assert_eq!(s.commitments(Tag::Task).unwrap().iter().filter(|(_, st)| *st == Status::Satisfied).count(), 1);
    assert_eq!(s.world.epi().unwrap().assessment(&Prop::Conforms(item)), Assessment::Accepted);
    assert_eq!(s.world.epi().unwrap().assessment(&Prop::Conforms(Requirement::lit("never-heard-of-it"))), Assessment::Unsupported);

    // A log left in the older format is not rejected: its chain is checked and its history carried over.
    let text = std::fs::read_to_string(&log).unwrap();
    let legacy = schema_1(&text);
    std::fs::write(&log, &legacy).unwrap();
    let s = Steward::open(&root, &log, 102).unwrap();
    assert_eq!(s.migrated_from, Some(1));
    assert!(s.world.rt.log().entries().iter().any(|e| matches!(e.event, Event::Migrated { from_schema: 1, .. })));
    assert_eq!(s.commitments(Tag::Upkeep).unwrap().len(), 8);

    // A log that has been edited is not trusted and not deleted: it is set aside, and a new record begins.
    std::fs::write(&log, text.replace("\"Tick\":100", "\"Tick\":99")).unwrap();
    let s = Steward::open(&root, &log, 102).unwrap();
    let aside = s.previous_log_rejected.clone().unwrap();
    assert!(aside.exists() && s.commitments(Tag::Upkeep).unwrap().is_empty());
    std::fs::remove_dir_all(root).unwrap();
}

// ---------------------------------------------------------------- real review evidence; the period learned from the record

#[test]
fn review_trailers_are_evidence_and_the_period_is_calibrated_from_the_record() {
    // 1,500 consecutive real Node.js commits from a period when changes carried `Reviewed-By`
    // trailers. The period comes from the project's rules, and the project approves what its
    // reflector proposes.
    let commits = import(include_str!("data/node-reviewed.txt")).unwrap();
    let mut w = gitlog::found().unwrap();
    let mut ingest = Ingest::by_the_rules();
    ingest.approve_reflector = true;
    for c in &commits {
        ingest.commit(&mut w, c).unwrap();
    }
    ingest.close(&mut w).unwrap();
    let s = &ingest.stats;
    assert!(s.reviewed > 1_000 && s.reviews > s.reviewed, "{s:?}");
    assert_eq!(s.denied, 0);
    // Reviewers are members who attested: each reviewed change is accepted on their evidence.
    let reviewed = commits.iter().find(|c| !c.reviewers.is_empty() && c.reverts.is_empty()).unwrap();
    let standing = w.epi().unwrap().standing(&Prop::Sound(reviewed.id.clone()));
    assert!(standing.basis.len() >= reviewed.reviewers.iter().filter(|r| **r != reviewed.committer.id).count());
    assert_eq!(conclusions(&w).unwrap().refuted, oracle(&commits));
    let epi = w.epi().unwrap();
    assert!(epi.full().iter().all(|(p, st)| epi.standing(p) == *st));
    // The same record, read without trailers, would have had far less evidence.
    assert!(s.reviews > 2 * s.independently_acknowledged);
}

#[test]
fn the_reflector_learns_the_maintenance_period_from_how_late_refutations_arrive() {
    const REFLECTOR: ActorId = ActorId::lit("reflector");
    // The standard project world, but with a reflector that judges the period after 4 refutations.
    let reflector: Box<dyn Unit<Event>> = Box::new(Cell(Reflector::new(REFLECTOR, CapId::Charter(gitlog::REFLECT), 1_000, 50, 4)));
    let mut w = World::new(Box::new(rigor::org::org(vec![reflector]).unwrap()));
    found_in(&mut w).unwrap();
    let day = 86_400u64;
    let period = |w: &World| w.gov().unwrap().rules().unwrap().norms.maintenance_period / day;
    assert_eq!(period(&w), 30);
    let _ = w.act_with(&REGISTRAR, &CapId::Charter(ADMISSIONS), Act::Admit(member(NEWCOMER))).unwrap();
    // Five changes are undertaken on day 0 and refuted 10, 50, 60, 70 and 80 days later.
    let names = ["aa", "bb", "cc", "dd", "ee"];
    for n in names {
        let _ = keep(&mut w, &NEWCOMER, &sound(n), 30 * day);
    }
    let mut proposal = None;
    for (n, after) in names.iter().zip([10u64, 50, 60, 70, 80]) {
        let _ = w.tick(after * day).unwrap();
        let entries = w.act(&PROJECT, attest(About::Rebuts(sound(n)), Method::Reversal, "undo")).unwrap();
        proposal = proposal.or(entries.iter().find_map(|e| match &e.event {
            Event::Proposed { id, by, .. } if *by == REFLECTOR => Some(*id),
            _ => None,
        }));
    }
    // Only the first refutation fell inside the 30 days: the period is plainly too short, and
    // the reflector has said so once, with a figure taken from the record.
    let r = w.rt.leaf::<Reflector>(&[REFLECTOR.0]).unwrap();
    assert_eq!((r.refutations_observed(), r.ninety_percent_delay()), (5, Some(80 * day)));
    assert_eq!(period(&w), 30, "proposing changes nothing");
    // Proposing is not approving, and one approval is not a quorum of two.
    let _ = w.act_with(&PROJECT, &CapId::Charter(gitlog::GOVERN_PROJECT), Act::Approve { id: proposal.unwrap() }).unwrap();
    assert_eq!(period(&w), 30);
    let _ = w.act_with(&REGISTRAR, &CapId::Charter(gitlog::GOVERN_REGISTRAR), Act::Approve { id: proposal.unwrap() }).unwrap();
    assert_eq!((w.gov().unwrap().version(), period(&w)), (Version(2), 70)); // nine in ten of the first four
                                                                            // The rule guards itself: a period of zero cannot be enacted by anyone.
    let zero = w
        .act_with(&PROJECT, &CapId::Charter(gitlog::GOVERN_PROJECT), Act::Propose { amendments: vec![rigor::model::Amendment::SetMaintenancePeriod(0)] })
        .unwrap();
    assert!(zero.iter().any(|e| matches!(&e.event, Event::GovRejected { reason: rigor::model::GovError::Invalid(rigor::model::RulesError::PeriodZero), .. })));
}

// ---------------------------------------------------------------- real transfers of value

#[test]
fn real_bitcoin_blocks_settle_with_every_budget_enforced() {
    // 22 consecutive real blocks (heights 150000-150021, 938 transactions), owners pseudonymised.
    // Each signed input is a delegation limited to its amount and bound to its transaction.
    use rigor::chain::{check, found, oracle as balances, parse, Settle, SAT, TREASURY};
    let blocks = parse(include_str!("data/btc-window.txt"), false).unwrap();
    let mut w = found(&blocks).unwrap();
    let mut settle = Settle::default();
    for b in &blocks {
        settle.block(&mut w, b).unwrap();
    }
    let s = &settle.stats;
    assert_eq!((s.blocks, s.transactions, s.delegations, s.transfers), (22, 938, 1359, 2532));
    // The real network accepted all of it, so the institution must refuse none of it...
    assert_eq!(s.wrongly_refused, 0);
    // ...and must refuse every replay and every use of a capability for another purpose.
    assert_eq!((s.replays_attempted, s.replays_refused, s.misuse_refused), (916, 916, 916));
    let v = check(&w, &blocks).unwrap();
    assert!(v.balances_equal_oracle && v.conserved && v.no_capability_amplifies && v.every_budget_exactly_spent, "{v:?}");
    assert_eq!(v.accounts, 2365);
    // The treasury ends empty: every unit of new coin and every fee was paid out as a reward.
    assert_eq!((w.ledger().unwrap().balance(&TREASURY, &SAT).0, balances(&blocks)[&TREASURY]), (0, 0));
    assert!(w.rt.log().verify().is_ok());
    let again = Runtime::replay(Box::new(rigor::org::org(Vec::new()).unwrap()), &w.rt.stimuli()).unwrap();
    assert_eq!(again.log().head(), w.rt.log().head());
    assert!(matches!(parse("O nobody 5", false), Err(rigor::chain::ChainError::Malformed { line: 1 })));
}

#[test]
fn a_capability_can_be_bound_to_one_purpose() {
    use rigor::model::{AttenuationError, Grant, Memo, Scope, Transfer};
    use rigor::scenario::{delegated, rules, world, Cast, AGENT, ALICE, DEPLOYER};
    let credit = rigor::actors::CREDIT;
    let mut w = world(Cast::default(), rules()).unwrap();
    let (rent, other) = (Name::lit("rent"), Name::lit("other"));
    let grant = |to: ActorId, purpose: Option<Name>, limit: u64| Grant {
        to,
        actions: BTreeSet::from([rigor::model::ActionKind::Transfer]),
        scope: Scope { from: Some(ALICE), purpose },
        limits: std::iter::once((credit.clone(), rigor::model::Amount(limit))).collect(),
        expires: None,
        delegable: true,
    };
    let cap = delegated(&w.act_with(&ALICE, &CapId::Own(ALICE), Act::Delegate(grant(AGENT, Some(rent.clone()), 40))).unwrap()).unwrap();
    let pay = |memo| Act::Transfer(Transfer { from: ALICE, to: DEPLOYER, resource: credit.clone(), amount: rigor::model::Amount(10), memo });
    assert_eq!(denial(&w.act_with(&AGENT, &cap, pay(Memo::None)).unwrap()), Some(DenyReason::OutOfScope));
    assert_eq!(denial(&w.act_with(&AGENT, &cap, pay(Memo::Purpose(other.clone()))).unwrap()), Some(DenyReason::OutOfScope));
    assert_eq!(denial(&w.act_with(&AGENT, &cap, pay(Memo::Purpose(rent.clone()))).unwrap()), None);
    // The binding survives delegation: it can be passed on, but not loosened or changed.
    for wider in [None, Some(other)] {
        let d = w.act_with(&AGENT, &cap, Act::Delegate(grant(DEPLOYER, wider, 5))).unwrap();
        assert_eq!(denial(&d), Some(DenyReason::NotAnAttenuation(AttenuationError::PurposeDiffersFromParent)));
    }
    assert!(delegated(&w.act_with(&AGENT, &cap, Act::Delegate(grant(DEPLOYER, Some(rent), 5))).unwrap()).is_some());
}

// ---------------------------------------------------------------- losing standing; changing role

#[test]
fn a_member_can_be_expelled_and_roles_reassigned_by_charter() {
    use rigor::model::{ActionKind, Charter, CharterName, Role};
    use rigor::scenario::{budget, constitution, delegated, pay, rules, AGENT, ALICE, AUDITOR, DEPLOYER, MONITOR, VENDOR};
    const MEMBERSHIP: CharterName = CharterName::lit("membership");
    let mut c = constitution(rules());
    c.charters.push(Charter { name: MEMBERSHIP, holder: ALICE, actions: BTreeSet::from([ActionKind::Expel, ActionKind::SetRole]), delegable: false });
    let mut w = World::new(Box::new(rigor::org::org(Vec::new()).unwrap()));
    let _ = w.inject(&rigor::world::CONSTITUTION, Event::Genesis(Box::new(c))).unwrap();
    let power = CapId::Charter(MEMBERSHIP);

    // The agent holds a budget from Alice and has passed part of it on; the vendor owes something.
    let cap = delegated(&w.act_with(&ALICE, &CapId::Own(ALICE), budget(AGENT, ALICE, 50, true)).unwrap()).unwrap();
    let sub = delegated(&w.act_with(&AGENT, &cap, budget(DEPLOYER, ALICE, 10, false)).unwrap()).unwrap();
    let q = Prop::Fact(Name::lit("q"));
    let owed = Offer { creditor: ALICE, ant: None, cons: Condition::Holds(q), deadline: Time(5), tags: BTreeSet::from([Tag::Task]), parent: None, terms: None };
    let _ = w.act(&AGENT, Act::Commit(owed)).unwrap();

    // Expulsion is a power, not a right of membership.
    assert_eq!(denial(&w.act(&VENDOR, Act::Expel { member: AGENT }).unwrap()), Some(DenyReason::ActionNotGranted));
    let out = w.act_with(&ALICE, &power, Act::Expel { member: AGENT }).unwrap();
    assert!(out.iter().any(|e| matches!(&e.event, Event::Expelled { member } if *member == AGENT)));
    // The expelled member can no longer act; what it had handed on is revoked with it.
    assert_eq!(denial(&w.act_with(&AGENT, &cap, pay(ALICE, DEPLOYER, 1)).unwrap()), Some(DenyReason::NotAMember));
    assert_eq!(denial(&w.act_with(&DEPLOYER, &sub, pay(ALICE, DEPLOYER, 1)).unwrap()), Some(DenyReason::Revoked(sub)));
    assert_eq!(denial(&w.act_with(&ALICE, &power, Act::Expel { member: AGENT }).unwrap()), Some(DenyReason::UnknownMember));
    // Its obligations do not leave with it: the commitment lapses and is judged as before.
    let _ = w.tick(6).unwrap();
    let id = w.one(&AGENT, Tag::Task).unwrap().commitment.id;
    assert_eq!(w.norms().unwrap().status(&id), Some(Status::Violated));
    assert!(w.adj().unwrap().findings(&id).contains(&(Finding::Accountable, AGENT)));
    // The audit duty that violation created fell on the then auditor.
    assert_eq!(w.tagged(Tag::Audit).unwrap()[0].terms().debtor, AUDITOR);

    // The auditor role moves to the monitor: duties created from now on fall on the new holder.
    let _ = w.act_with(&ALICE, &power, Act::SetRole { member: AUDITOR, role: None }).unwrap();
    let _ = w.act_with(&ALICE, &power, Act::SetRole { member: MONITOR, role: Some(Role::Auditor) }).unwrap();
    let second = Offer {
        creditor: ALICE,
        ant: None,
        cons: Condition::Holds(Prop::Fact(Name::lit("r"))),
        deadline: Time(7),
        tags: BTreeSet::from([Tag::Task]),
        parent: None,
        terms: None,
    };
    let _ = w.act(&VENDOR, Act::Commit(second)).unwrap();
    let _ = w.tick(8).unwrap();
    let debtors: BTreeSet<ActorId> = w.tagged(Tag::Audit).unwrap().iter().map(|r| r.terms().debtor.clone()).collect();
    assert_eq!(debtors, BTreeSet::from([AUDITOR, MONITOR]));
    assert!(w.inst().unwrap().audit().is_empty());
}

// ---------------------------------------------------------------- a snapshot is a claim like any other

#[test]
fn a_snapshot_is_resumed_only_once_someone_trusted_has_shown_it_faithful() {
    use rigor::adapters::{audit_snapshot, resume_attested};
    use rigor::scenario::{root, run, Cast, AUDITOR};
    let dir = scratch("attest");
    let (mut w, _) = run(Cast::default(), true, |_| {}).unwrap();
    let fresh = || root(Cast::default()).unwrap();
    let good = dir.join("good.snapshot");
    let snap = save_snapshot(&w, &good).unwrap();
    // Unattested: refused, however well-formed.
    assert!(matches!(resume_attested(fresh(), &good, &w), Err(AdapterError::SnapshotNotAttested)));
    // The auditor replays the whole history, finds the same state, and says so as evidence.
    assert!(audit_snapshot(&mut w, fresh(), &snap, &AUDITOR).unwrap());
    let resumed = resume_attested(fresh(), &good, &w).unwrap();
    assert_eq!(resumed.rt.log().head(), snap.head);

    // A snapshot that is internally consistent but is NOT the state the log produces: a plain
    // resume cannot tell; the audit can, and its finding blocks the resume.
    let mut forged = snap.clone();
    let end = forged.state.len() - 1;
    forged.state[end] ^= 1;
    forged.digest = rigor::waist::digest_bytes(&forged.state);
    let bad = dir.join("bad.snapshot");
    std::fs::write(&bad, bincode_of(&forged)).unwrap();
    assert!(!audit_snapshot(&mut w, fresh(), &forged, &AUDITOR).unwrap());
    assert!(matches!(resume_attested(fresh(), &bad, &w), Err(AdapterError::SnapshotNotAttested)));
    assert_eq!(w.epi().unwrap().assessment(&rigor::adapters::faithful(&forged).unwrap()), Assessment::Refuted);
    // The attestation names the whole snapshot, not just its state: the audited state re-pointed
    // at another place in the log is a different snapshot, which nobody has attested.
    let mut repointed = snap.clone();
    repointed.seq = 3;
    repointed.head = rigor::waist::Digest(0xDEAD_BEEF);
    let moved = dir.join("moved.snapshot");
    std::fs::write(&moved, bincode_of(&repointed)).unwrap();
    assert!(matches!(resume_attested(fresh(), &moved, &w), Err(AdapterError::SnapshotNotAttested)));
    // An audit needs the history: a compacted world cannot vouch for a snapshot.
    w.rt.compact();
    assert!(matches!(audit_snapshot(&mut w, fresh(), &snap, &AUDITOR), Err(AdapterError::NotAuditable)));
    std::fs::remove_dir_all(dir).unwrap();
}

fn bincode_of(s: &rigor::waist::Snapshot) -> Vec<u8> {
    bincode::serialize(s).unwrap()
}

#[test]
fn an_event_that_changed_shape_is_upgraded_not_lost() {
    // A genesis exactly as schema 2 wrote it: charters had no `delegable`, norm rules had no
    // maintenance period. It does not parse as today's event; upgraded, it does, with the
    // meaning it had then.
    use rigor::model::upcast;
    use rigor::scenario::{constitution, rules};
    let today = serde_json::to_value(Event::Genesis(Box::new(constitution(rules())))).unwrap();
    let mut old = today.clone();
    for charter in old.pointer_mut("/Genesis/charters").unwrap().as_array_mut().unwrap() {
        let _ = charter.as_object_mut().unwrap().remove("delegable").unwrap();
    }
    let _ = old.pointer_mut("/Genesis/rules/norms").unwrap().as_object_mut().unwrap().remove("maintenance_period").unwrap();
    assert!(serde_json::from_value::<Event>(old.clone()).is_err());
    let upgraded: Event = serde_json::from_value(upcast(2, old.clone())).unwrap();
    let Event::Genesis(c) = &upgraded else { panic!("not a genesis") };
    assert!(c.charters.iter().all(|ch| !ch.delegable));
    assert_eq!(c.rules.norms.maintenance_period, 30 * 24 * 3600);
    // The step is the identity on what is already current, and on events it does not concern.
    assert_eq!(upcast(2, today.clone()), today);
    let tick = serde_json::to_value(Event::Tick(Time(7))).unwrap();
    assert_eq!(upcast(1, tick.clone()), tick);
}

#[test]
fn changing_the_vocabulary_without_bumping_the_schema_fails_here() {
    // A fixed sample of events, encoded. If this digest changes, the stored form of events has
    // changed: bump `waist::SCHEMA`, add a step to `model::upcast`, then update this constant.
    use rigor::scenario::{budget, constitution, pay, rules, AGENT, ALICE, DEPLOYER};
    let sample = vec![
        Event::Genesis(Box::new(constitution(rules()))),
        Event::Tick(Time(1)),
        Event::Do { act: budget(AGENT, ALICE, 50, true), cap: Some(CapId::Own(ALICE)) },
        Event::Do { act: pay(ALICE, DEPLOYER, 5), cap: None },
        Event::Do { act: attest(About::Supports(Prop::Fact(Name::lit("x"))), Method::Probe, "note"), cap: None },
        Event::Do { act: Act::Admit(member(NEWCOMER)), cap: None },
        Event::Do { act: Act::Expel { member: NEWCOMER }, cap: None },
        Event::ArtifactChanged { artifact: rigor::model::ArtifactId::lit("a"), digest: rigor::model::ContentDigest(rigor::waist::Digest(1)) },
        Event::Settle,
    ];
    let digest = rigor::waist::digest_of(&sample).unwrap();
    assert_eq!((rigor::waist::SCHEMA, digest.0), (4, SAMPLE_DIGEST), "vocabulary changed: see the comment above");
}

const SAMPLE_DIGEST: u64 = 17_928_536_702_078_334_411;

// ---------------------------------------------------------------- real forge reviews

#[test]
fn real_pull_request_reviews_enter_as_evidence_for_and_against() {
    // 935 real Kubernetes commits and the reviews GitHub recorded for 52 pull requests merged
    // among them. An approval supports the change; a request for changes that was never
    // followed by that reviewer's approval is standing evidence against it.
    let mut commits = import(include_str!("data/k8s-window.txt")).unwrap();
    let reviews = gitlog::parse_reviews(include_str!("data/k8s-reviews.txt")).unwrap();
    assert_eq!(gitlog::apply_reviews(&mut commits, &reviews), 52);
    let with_approval = commits.iter().filter(|c| !c.reviewers.is_empty()).count();
    let objected: Vec<_> = commits.iter().filter(|c| !c.objectors.is_empty()).collect();
    assert_eq!((with_approval, objected.len()), (9, 2));
    let mut w = gitlog::found().unwrap();
    let mut ingest = Ingest::by_the_rules();
    for c in &commits {
        ingest.commit(&mut w, c).unwrap();
    }
    assert_eq!((ingest.stats.objections, ingest.stats.denied), (2, 0));
    assert!(ingest.stats.reviews >= 9);
    let epi = w.epi().unwrap();
    // An objection of the same standing as the acknowledgements leaves the change contested --
    // or, where nobody else acknowledged it, refuted. Neither is "accepted".
    for c in &objected {
        assert_ne!(epi.assessment(&Prop::Sound(c.id.clone())), Assessment::Accepted, "{}", c.id);
    }
    let approved = commits.iter().find(|c| !c.reviewers.is_empty() && c.objectors.is_empty() && c.reverts.is_empty()).unwrap();
    assert_eq!(epi.assessment(&Prop::Sound(approved.id.clone())), Assessment::Accepted);
    assert!(epi.full().iter().all(|(p, st)| epi.standing(p) == *st));
    assert!(matches!(gitlog::parse_reviews("too few"), Err(gitlog::GitError::Malformed { record: 0 })));
}

// ---------------------------------------------------------------- a real package registry

#[test]
fn a_real_registry_with_yanks_agrees_with_the_oracle() {
    // The crates.io index records of 45 real packages (3,679 releases, 282 of them yanked).
    // A release rests on the releases it depends on; a yank is a retraction, and whatever
    // rested on it loses its support.
    use rigor::registry::{conclusions as fit_now, found, ingest, oracle as fit_oracle, parse, resolve};
    let (all, skipped) = parse(include_str!("data/crates-window.jsonl")).unwrap();
    assert_eq!((all.len(), skipped, all.iter().filter(|p| p.yanked).count()), (3679, 200, 282));
    let mut w = found().unwrap();
    let stats = ingest(&mut w, &all).unwrap();
    assert_eq!((stats.packages, stats.refused, stats.on_a_cycle), (45, 0, 0));
    assert!(stats.dependencies_resolved > 4_000);
    let (fit, by_status) = fit_now(&w).unwrap();
    let expected = fit_oracle(&all);
    assert_eq!(fit, expected);
    // Some releases that were never yanked are nevertheless not fit: they rest on one that was.
    let not_yanked = all.iter().filter(|p| !p.yanked).count();
    assert_eq!((fit.len(), not_yanked - fit.len()), (3390, 7));
    let deps = resolve(&all);
    let fell = all.iter().find(|p| !p.yanked && !fit.contains(&p.release)).unwrap();
    assert!(deps[&fell.release].iter().any(|d| !fit.contains(d)));
    // The publishers' undertakings follow the assessment: satisfied where fit, open again where not.
    assert_eq!((by_status.get(&Status::Satisfied), by_status.get(&Status::Detached)), (Some(&3390), Some(&289)));
    let epi = w.epi().unwrap();
    assert!(epi.full().iter().all(|(p, st)| epi.standing(p) == *st));
    // Revision is local: the last yank re-labelled only what rested on that release.
    assert!(epi.work.0 < 40_000, "work {} would be millions if each change re-labelled the graph", epi.work.0);
    assert!(matches!(parse("not json"), Err(rigor::registry::RegistryError::Malformed { line: 1 })));
}

#[test]
fn version_requirements_are_typed_and_mean_what_they_say() {
    use rigor::registry::{Op, Requirement};
    let req = |op, major, minor, patch| Requirement { package: Name::lit("x"), op, major, minor, patch };
    assert!(req(Op::Caret, 1, Some(2), None).admits((1, 9, 0)) && !req(Op::Caret, 1, Some(2), None).admits((2, 0, 0)));
    assert!(!req(Op::Caret, 1, Some(2), None).admits((1, 1, 9)));
    assert!(req(Op::Caret, 0, Some(3), None).admits((0, 3, 7)) && !req(Op::Caret, 0, Some(3), None).admits((0, 4, 0)));
    assert!(req(Op::Tilde, 1, Some(2), None).admits((1, 2, 9)) && !req(Op::Tilde, 1, Some(2), None).admits((1, 3, 0)));
    assert!(req(Op::Exact, 1, Some(0), Some(229)).admits((1, 0, 229)) && !req(Op::Exact, 1, Some(0), Some(229)).admits((1, 0, 228)));
    assert!(req(Op::AtLeast, 2, None, None).admits((5, 0, 0)) && req(Op::Any, 0, None, None).admits((0, 0, 1)));
}

// ---------------------------------------------------------------- one organisation as a member of another

mod federation {
    use super::*;
    use rigor::actors::{service, Agent};
    use rigor::model::{Constitution, Kind, MemberKind, Request, Task, TaskName};
    use rigor::org::{org, standard_rules, Trusted};
    use rigor::scenario::{budget, constitution, rules, AGENT, ALICE, DEPLOYER, MONITOR, TASK1, VENDOR};
    use rigor::units::envoy::Envoy;
    use rigor::waist::Entity;

    const ENVOY: ActorId = ActorId::lit("envoy");
    const ENG: ActorId = ActorId::lit("eng");
    const QA: ActorId = ActorId::lit("qa");

    /// The scenario's world, except that the vendor is a whole organisation of its own:
    /// an institution, an engineer, a QA service and an envoy, behind a federated boundary.
    fn world_with_vendor_org() -> World {
        let inner = org(vec![Box::new(Cell(Envoy::new(ENVOY, VENDOR, ENG)))]).unwrap();
        let vendor = Entity::federated(VENDOR.0, Box::new(inner), [Kind::Tick, Kind::NormCreated].into(), [Kind::Do, Kind::Signal].into());
        let agent: Box<dyn Unit<Event>> = Box::new(Cell(Agent::new(AGENT, DEPLOYER, rigor::model::Amount(30), false)));
        let parts: Vec<Box<dyn Unit<Event>>> = vec![agent, Box::new(service(&DEPLOYER, false, true).unwrap()), Box::new(vendor)];
        let mut w = World::new(Box::new(org(parts).unwrap()));
        let _ = w.inject(&rigor::world::CONSTITUTION, Event::Genesis(Box::new(constitution(rules())))).unwrap();
        // The vendor's own constitution: its own members, its own rules, its own trusted sources.
        let m = |id: ActorId, kind| Member { id, kind, principal: None, role: None };
        let inner_rules = standard_rules(&Trusted { receipts: QA, probes: QA, tests: QA, audits: QA, acks: ENVOY });
        let inner = Constitution {
            members: vec![m(ENVOY, MemberKind::Service), m(ENG, MemberKind::Human), m(QA, MemberKind::Service)],
            charters: Vec::new(),
            balances: Vec::new(),
            rules: inner_rules,
        };
        w.scope = vec![VENDOR.0];
        let _ = w.inject(&rigor::world::CONSTITUTION, Event::Genesis(Box::new(inner))).unwrap();
        w.scope = Vec::new();
        w
    }

    /// Run the outer story up to the point where the outer institution holds the vendor to a
    /// remediation duty. Returns that duty's id and proposition.
    fn until_vendor_is_liable(w: &mut World) -> (rigor::model::CommitmentId, Prop) {
        let _ = w.act_with(&ALICE, &CapId::Own(ALICE), budget(AGENT, ALICE, 50, true)).unwrap();
        let _ = w.act(&ALICE, Act::Request(Request { to: AGENT, task: Task::whole(TASK1), deadline: Time(5), parent: None, terms: None })).unwrap();
        let deployed = Prop::Done(Task::whole(TASK1).step(TaskName::lit("deploy")));
        let _ = w.act(&rigor::scenario::CI, attest(About::Supports(deployed.clone()), Method::Test, "smoke test passed")).unwrap();
        let _ = w.tick(6).unwrap();
        let _ = w.act(&MONITOR, attest(About::Rebuts(deployed), Method::Probe, "not there")).unwrap();
        let duty = w.one(&VENDOR, Tag::Remediation).unwrap();
        let Condition::Holds(p) = duty.terms().cons.clone() else { panic!("remediation is a proposition") };
        (duty.commitment.id, p)
    }

    fn inner_commit(w: &mut World, outer_duty: rigor::model::CommitmentId, deadline: u64) {
        let task = Envoy::task_for(outer_duty).unwrap();
        let offer = Offer {
            creditor: ENVOY,
            ant: None,
            cons: Condition::Holds(Prop::Done(task)),
            deadline: Time(deadline),
            tags: BTreeSet::from([Tag::Task]),
            parent: None,
            terms: None,
        };
        w.scope = vec![VENDOR.0];
        let _ = w.act(&ENG, Act::Commit(offer)).unwrap();
        w.scope = Vec::new();
    }

    #[test]
    fn a_duty_laid_on_an_organisation_becomes_work_inside_it_and_its_report_is_only_a_claim() {
        let mut w = world_with_vendor_org();
        let (duty, remediated) = until_vendor_is_liable(&mut w);
        // The envoy turned the outer duty into an inner request, under the inner institution.
        let task = Envoy::task_for(duty).unwrap();
        let asked = w.rt.log().entries().iter().any(|e| matches!(&e.event, Event::Scoped { entity, inner } if *entity == VENDOR.0 && matches!(inner.as_ref(), Event::Ok { by, act: Act::Request(r), .. } if *by == ENVOY && r.to == ENG && r.task == task)));
        assert!(asked);
        // Inside: the engineer undertakes it, QA (trusted inside) attests it done.
        inner_commit(&mut w, duty, 9);
        w.scope = vec![VENDOR.0];
        let _ = w.act(&QA, attest(About::Supports(Prop::Done(task.clone())), Method::Test, "inner checks pass")).unwrap();
        assert_eq!(w.one(&ENG, Tag::Task).unwrap().status, Some(Status::Satisfied));
        assert_eq!(w.epi().unwrap().assessment(&Prop::Done(task.clone())), Assessment::Accepted);
        w.scope = Vec::new();
        // Outside: the organisation (one actor) has CLAIMED the duty done, and signalled so.
        let claimed =
            w.rt.log()
                .entries()
                .iter()
                .any(|e| e.actor.only() == Some(&VENDOR.0) && matches!(&e.event, Event::Do { act: Act::Claim { prop }, .. } if *prop == remediated));
        assert!(claimed);
        // The claim is not evidence: inner evidence did not cross the boundary, and the outer
        // institution knows nothing of the inner task. The outer duty is still open.
        assert_eq!(w.epi().unwrap().assessment(&remediated), Assessment::Unsupported);
        assert_eq!(w.epi().unwrap().assessment(&Prop::Done(task)), Assessment::Unsupported);
        assert_eq!(w.norms().unwrap().status(&duty), Some(Status::Detached));
        // The outer institution's own trusted source settles it.
        let _ = w.act(&MONITOR, attest(About::Supports(remediated), Method::Probe, "v2 live")).unwrap();
        assert_eq!(w.norms().unwrap().status(&duty), Some(Status::Satisfied));
        // Outside, nobody inside the vendor is a member: its engineer cannot act there.
        assert_eq!(denial(&w.act(&ENG, Act::Claim { prop: Prop::Fact(Name::lit("x")) }).unwrap()), Some(DenyReason::NotAMember));
        assert!(w.rt.log().verify().is_ok());
    }

    #[test]
    fn each_institution_judges_at_its_own_level_when_the_inner_work_fails() {
        let mut w = world_with_vendor_org();
        let (duty, remediated) = until_vendor_is_liable(&mut w);
        inner_commit(&mut w, duty, 8);
        // Nobody delivers. The outer clock is the inner clock (the envoy relays it).
        let _ = w.tick(9).unwrap();
        w.scope = vec![VENDOR.0];
        let inner_duty = w.one(&ENG, Tag::Task).unwrap().commitment.id;
        assert_eq!(w.norms().unwrap().status(&inner_duty), Some(Status::Violated));
        // Inside, the engineer is found accountable -- by the inner institution's rules.
        assert!(w.adj().unwrap().findings(&inner_duty).contains(&(Finding::Accountable, ENG)));
        w.scope = Vec::new();
        // The envoy reported the failure outward, honestly, and did not claim success.
        let reported =
            w.rt.log().entries().iter().any(|e| {
                e.actor.only() == Some(&VENDOR.0) && matches!(&e.event, Event::Signal { to, prop, value: false } if *to == ALICE && *prop == remediated)
            });
        let claimed = w.rt.log().entries().iter().any(|e| matches!(&e.event, Event::Do { act: Act::Claim { prop }, .. } if *prop == remediated));
        assert!(reported && !claimed);
        // Outside, when the outer deadline passes, the ORGANISATION is accountable: the outer
        // institution neither sees nor judges the engineer.
        let _ = w.tick(12).unwrap();
        assert_eq!(w.norms().unwrap().status(&duty), Some(Status::Violated));
        let outer = w.adj().unwrap().findings(&duty);
        assert!(outer.contains(&(Finding::Accountable, VENDOR)) && !outer.iter().any(|(_, who)| *who == ENG));
    }
}

// ---------------------------------------------------------------- a forge's public event archive

#[test]
fn real_forge_reviews_with_superseded_and_dismissed_reviews_agree_with_the_oracle() {
    // 6,000 real happenings from GitHub's public event archive (November 2026), about pull
    // requests that were reviewed: openings, approvals, change requests, dismissals, merges.
    // A reviewer's later review withdraws the earlier one: real retraction.
    use rigor::forge::{conclusions as assessed, found, import, oracle as expected_standing, Ingest as Forge, Verdict, What};
    let all = import(include_str!("data/forge-window.txt")).unwrap();
    assert_eq!(all.len(), 6000);
    let mut w = found().unwrap();
    let mut ingest = Forge::default();
    for h in &all {
        ingest.happen(&mut w, h).unwrap();
    }
    let s = &ingest.stats;
    assert_eq!(s.refused, 0);
    assert!(s.approvals > 1000 && s.change_requests > 100 && s.dismissals > 50 && s.reviews_withdrawn > 20, "{s:?}");
    let expected = expected_standing(&all);
    let got = assessed(&w, &expected).unwrap();
    assert_eq!(got, expected);
    // All four standings occur in real data.
    for a in [Assessment::Accepted, Assessment::Refuted, Assessment::Unsupported] {
        assert!(got.values().any(|x| *x == a), "{a:?}");
    }
    // A change whose only review was dismissed stands unsupported, not refuted.
    let dismissed_only = all.iter().find(|h| h.what == What::Reviewed(Verdict::Dismissed) && got.get(&h.change) == Some(&Assessment::Unsupported));
    assert!(dismissed_only.is_some());
    let epi = w.epi().unwrap();
    assert!(epi.full().iter().all(|(p, st)| epi.standing(p) == *st));
    assert!(w.rt.log().verify().is_ok());
    assert!(matches!(import("X a b 1"), Err(rigor::forge::ForgeError::Malformed { line: 1 })));
}

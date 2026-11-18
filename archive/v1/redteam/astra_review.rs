//! Run explicitly with cargo test --test astra_review -- --ignored.
#![allow(clippy::unwrap_used)]

use rigor::actors::CREDIT;
use rigor::kernel::Name;
use rigor::model::*;
use rigor::org::org;
use rigor::scenario::*;
use rigor::steward::{Check, Outcome, Steward};
use rigor::world::{World, CONSTITUTION};
use std::collections::BTreeSet;
use std::fs;
use std::process::Command;

#[test]
#[ignore]
fn ledger_total() {
    let mut c = constitution(rules());
    c.balances = vec![(ALICE, CREDIT, Amount(1)), (AGENT, CREDIT, Amount(u64::MAX))];
    let mut w = World::new(Box::new(org(vec![]).unwrap()));
    let _ = w.inject(&CONSTITUTION, Event::Genesis(Box::new(c))).unwrap();
    let events = w.act_with(&ALICE, &CapId::Own(ALICE), pay(ALICE, AGENT, 1)).unwrap();
    assert!(events.iter().any(|e| matches!(e.event, Event::Transferred { .. })));
    let l = w.ledger().unwrap();
    assert!(l.conserved());
    let total = u128::from(l.balance(&ALICE, &CREDIT).0) + u128::from(l.balance(&AGENT, &CREDIT).0);
    assert_eq!(total, 1 + u128::from(u64::MAX));
}

fn maintenance() -> (World, Prop, CommitmentId) {
    let mut w = world(Cast::default(), rules()).unwrap();
    let p = Prop::Fact(Name::lit("maintenance"));
    let offer = Offer {
        creditor: ALICE,
        ant: None,
        cons: Condition::Unrefuted(p.clone()),
        deadline: Time(10),
        tags: [Tag::Task].into_iter().collect(),
        parent: None,
        terms: None,
    };
    let _ = w.act(&AGENT, Act::Commit(offer)).unwrap();
    let id = w.one(&AGENT, Tag::Task).unwrap().commitment.id;
    (w, p, id)
}

#[test]
#[ignore]
fn corroboration() {
    let (mut w, p, id) = maintenance();
    let _ = w.tick(5).unwrap();
    let _ = w.act(&MONITOR, attest(About::Rebuts(p.clone()), Method::Probe, "first")).unwrap();
    let _ = w.tick(11).unwrap();
    assert_eq!(w.norms().unwrap().status(&id), Some(Status::Violated));
    let _ = w.act(&MONITOR, attest(About::Rebuts(p.clone()), Method::Probe, "second")).unwrap();
    assert_eq!(w.epi().unwrap().standing(&p).basis.len(), 2);
    assert_eq!(w.epi().unwrap().assessment(&p), Assessment::Refuted);
    assert_eq!(w.norms().unwrap().status(&id), Some(Status::Violated));
}

#[test]
#[ignore]
fn withdrawn_basis() {
    let (mut w, p, id) = maintenance();
    let entries = w.act(&MONITOR, attest(About::Rebuts(p), Method::Probe, "first")).unwrap();
    let evidence = entries
        .iter()
        .find_map(|e| match &e.event {
            Event::EvidenceRecorded(v) => Some(v.id),
            _ => None,
        })
        .unwrap();
    let child = w.one(&AUDITOR, Tag::Audit).unwrap().commitment.id;
    let _ = w.act(&AUDITOR, Act::Cancel { id: child }).unwrap();
    assert_eq!(w.norms().unwrap().status(&child), Some(Status::Violated));
    let _ = w.act(&MONITOR, Act::Retract { evidence }).unwrap();
    assert_eq!(w.norms().unwrap().status(&id), Some(Status::Detached));
    assert_eq!(w.norms().unwrap().status(&child), Some(Status::Void));
}

fn check_days(fixture_changes: bool) {
    let root = std::env::temp_dir().join(format!("astra-review-{}-{fixture_changes}", std::process::id()));
    fs::create_dir(&root).unwrap();
    fs::create_dir(root.join("crates")).unwrap();
    fs::create_dir(root.join("proofs")).unwrap();
    for (file, contents) in [
        ("Cargo.toml", "[package]\nname='review'\nversion='0.1.0'\nedition='2021'\n[lib]\npath='crates/lib.rs'\n"),
        ("rustfmt.toml", ""),
        ("crates/data.txt", "ok"),
        ("crates/lib.rs", "#[test] fn fixture() { assert_eq!(include_str!(\"data.txt\"), \"ok\"); }\n"),
    ] {
        fs::write(root.join(file), contents).unwrap();
    }
    let run = |args: &[&str]| Command::new("cargo").args(args).current_dir(&root).output().unwrap().status;
    assert!(run(&["generate-lockfile"]).success());
    let log = root.join(".rigor/steward.jsonl");
    let skip: BTreeSet<_> = [Check::Lints, Check::Format, Check::Proofs].into_iter().collect();
    let mut s = Steward::open(&root, &log, 100).unwrap();
    assert!(s.run(100, false, &skip).unwrap().contains(&(Check::Tests, Outcome::RanAndPassed)));
    let id = s.commitments(Tag::Upkeep).unwrap().into_iter().find(|(c, _)| c.terms.cons == Condition::Holds(Check::Tests.prop())).unwrap().0.id;
    assert_eq!(s.world.norms().unwrap().status(&id), Some(Status::Satisfied));
    if fixture_changes {
        fs::write(root.join("crates/data.txt"), "changed").unwrap();
    } else {
        fs::write(root.join("crates/lib.rs"), "#[test] fn still_passes() {}\n").unwrap();
    }
    let actual = run(&["test", "--quiet"]).success();
    let mut s = Steward::open(&root, &log, 101).unwrap();
    let outcomes = s.run(101, false, &skip).unwrap();
    fs::remove_dir_all(root).unwrap();
    if fixture_changes {
        assert!(!actual && outcomes.contains(&(Check::Tests, Outcome::StillEvidenced)));
        assert_eq!(s.conformant(&skip).unwrap(), actual);
    } else {
        assert!(actual && outcomes.contains(&(Check::Tests, Outcome::RanAndPassed)));
        assert_eq!(s.world.norms().unwrap().status(&id), Some(Status::Satisfied));
    }
}

#[test]
#[ignore]
fn test_fixture_change() {
    check_days(true);
}

#[test]
#[ignore]
fn historical_upkeep() {
    check_days(false);
}

#[test]
#[ignore]
fn auditor_finding() {
    use rigor::logic::datalog::{Atom, Literal, Term, Var};
    let mut rules = rules();
    let party = Term::Var(Var(0));
    let id = Term::Var(Var(1));
    rules.responsibility.rules.push(RRule {
        id: RuleId::lit("role-duty"),
        head: Atom { pred: Pred::Accountable, args: vec![party.clone(), id.clone()] },
        body: vec![
            Literal::Pos(Atom { pred: Pred::Status, args: vec![id, Term::Const(Const::Status(Status::Violated))] }),
            Literal::Pos(Atom { pred: Pred::HasRole, args: vec![party, Term::Const(Const::Role(Role::Auditor))] }),
        ],
    });
    assert!(rules.validate().is_ok());
    let mut w = world(Cast::default(), rules).unwrap();
    let offer = Offer {
        creditor: ALICE,
        ant: None,
        cons: Condition::Holds(Prop::Fact(Name::lit("deliver"))),
        deadline: Time(1),
        tags: [Tag::Payment].into_iter().collect(),
        parent: None,
        terms: None,
    };
    let _ = w.act(&AGENT, Act::Commit(offer)).unwrap();
    let id = w.one(&AGENT, Tag::Payment).unwrap().commitment.id;
    let _ = w.tick(2).unwrap();
    let finding = (Finding::Accountable, AUDITOR);
    let before = w.adj().unwrap().findings(&id).contains(&finding);
    let _ = w.act(&ALICE, Act::Assign { id, to: AUDITOR }).unwrap();
    let after = w.adj().unwrap().findings(&id).contains(&finding);
    assert_eq!((before, after), (true, true));
}

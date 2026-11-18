//! The system's knowledge of itself, its budget, and the controlled way it is changed.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::indexing_slicing)]

use rigor::kernel::Name;
use rigor::mind::Gateway;
use rigor::model::{About, Act, Assessment, CapId, DenyReason, Event, Method, Prop, Requirement, Status, Tag};
use rigor::steward::change::{ChangeRequest, Missing, NewFile, Refusal};
use rigor::steward::selfmodel::{Ability, Need, Standing};
use rigor::steward::{Check, Steward, StewardError, CI, GATEWAY, MICRO_USD, OWNER, RESEARCHER, REVIEWER};
use rigor::world::attest;
use std::collections::BTreeSet;
use std::path::PathBuf;

fn fake_repo(name: &str) -> PathBuf {
    let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join("crates")).unwrap();
    std::fs::create_dir_all(root.join("proofs")).unwrap();
    for f in ["crates/lib.rs", "proofs/t.lean", "Cargo.toml", "Cargo.lock", "rustfmt.toml"] {
        std::fs::write(root.join(f), f).unwrap();
    }
    root
}

fn open(root: &std::path::Path, day: u64) -> Steward {
    Steward::open(root, &root.join(".rigor/steward.jsonl"), day).unwrap()
}

/// A gateway for which no credential can be found.
fn no_gateway() -> Gateway {
    Gateway::vercel(vec![PathBuf::from("/nonexistent/key.json")])
}

#[test]
fn it_knows_what_it_cannot_do_and_exactly_what_is_lacking() {
    if std::env::var("AI_GATEWAY_API_KEY").is_ok() {
        return; // this test is about the case where no credential exists
    }
    let root = fake_repo("selfmodel");
    let mut s = open(&root, 100);
    let report = s.examine_itself(&no_gateway()).unwrap();

    // Without a credential or a budget it cannot consult models, and says which needs are unmet.
    let Some(Standing::Cannot(lacking)) = report.standing(Ability::ConsultModels) else { panic!("should not be able to consult models") };
    assert_eq!(lacking, &vec![Need::Credential, Need::GatewayAnswers, Need::Budget]);
    // What depends on it is lacking it, by name.
    assert_eq!(report.standing(Ability::Research), Some(&Standing::Cannot(vec![Need::Ability(Ability::ConsultModels)])));
    // Some things it will never do, and that is stated as design, not as a gap.
    assert_eq!(report.standing(Ability::EstablishTruth), Some(&Standing::Cannot(vec![Need::ByDesignNever])));
    // Applying a change is something it can do only one approval at a time.
    assert_eq!(report.standing(Ability::ApplyChanges), Some(&Standing::CanWithApproval));
    // Its account of itself is on the record as assessed evidence, not just printed.
    let can = |s: &Steward, a: Ability| s.world.epi().unwrap().assessment(&Prop::Can(a.name()));
    assert_eq!(can(&s, Ability::ConsultModels), Assessment::Refuted);
    assert_eq!(can(&s, Ability::ReadHistories), Assessment::Accepted);

    // The owner grants a budget: that need is met; the others are still named.
    assert!(s.owner_grants_budget(50_000).unwrap());
    let report = s.examine_itself(&no_gateway()).unwrap();
    assert_eq!(report.standing(Ability::ConsultModels), Some(&Standing::Cannot(vec![Need::Credential, Need::GatewayAnswers])));

    // Acting on itself without supervision needs the owner's explicit decision.
    let unsupervised = Requirement::lit("unsupervised-changes-allowed");
    use rigor::steward::selfmodel::Unbuilt;
    let Some(Standing::Cannot(lacking)) = report.standing(Ability::ChangeItselfUnsupervised) else { panic!() };
    assert_eq!(lacking, &vec![Need::OwnerSaysYes(unsupervised.clone()), Need::Unbuilt(Unbuilt::UnsupervisedApply)]);
    // Even the owner's yes does not create the ability: there is deliberately no code path for it.
    s.owner_decides(unsupervised.clone(), true, 100).unwrap();
    assert_eq!(
        s.examine_itself(&no_gateway()).unwrap().standing(Ability::ChangeItselfUnsupervised),
        Some(&Standing::Cannot(vec![Need::Unbuilt(Unbuilt::UnsupervisedApply)]))
    );
    assert_eq!(can(&s, Ability::ChangeItselfUnsupervised), Assessment::Refuted);
    // A decision is recorded either way, and a need that depends on it follows the record.
    let forge = Requirement::lit("use-the-owners-forge-credential");
    s.owner_decides(forge.clone(), false, 101).unwrap();
    assert_eq!(s.world.epi().unwrap().assessment(&Prop::Conforms(forge)), Assessment::Refuted);
    let _ = unsupervised;
    // Examining again with nothing changed adds nothing to the record.
    let before = s.world.rt.log().next_seq();
    let _ = s.examine_itself(&no_gateway()).unwrap();
    assert_eq!(s.world.rt.log().next_seq(), before);
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn the_model_budget_is_enforced_by_the_institutions_own_ledger() {
    let root = fake_repo("budget");
    let mut s = open(&root, 100);
    assert_eq!(s.budget_left().unwrap(), 0);
    assert!(!s.pay(1).unwrap(), "nothing can be spent before the owner sets a budget");
    assert!(s.owner_grants_budget(1_000).unwrap());
    assert!(s.pay(400).unwrap() && s.pay(600).unwrap());
    assert_eq!(s.budget_left().unwrap(), 0);
    assert!(!s.pay(1).unwrap(), "the limit is the limit");
    let ledger = s.world.ledger().unwrap();
    assert_eq!((ledger.balance(&GATEWAY, &MICRO_USD).0, ledger.balance(&OWNER, &MICRO_USD).0), (1_000, 4_999_000));
    assert!(ledger.conserved());
    // The steward's service account cannot simply take the owner's money without a budget...
    let take = rigor::model::Transfer { from: OWNER, to: CI, resource: MICRO_USD, amount: rigor::model::Amount(5), memo: rigor::model::Memo::None };
    let entries = s.world.act(&CI, Act::Transfer(take.clone())).unwrap();
    assert!(entries.iter().any(|e| matches!(&e.event, Event::Denied { reason: DenyReason::ActionNotGranted, .. })));
    // ...and a model cannot grant itself one.
    let entries = s.world.act_with(&RESEARCHER, &CapId::Own(OWNER), Act::Transfer(take)).unwrap();
    assert!(entries.iter().any(|e| matches!(&e.event, Event::Denied { reason: DenyReason::NotHolder, .. })));
    std::fs::remove_dir_all(root).unwrap();
}

fn request(path: &str) -> ChangeRequest {
    ChangeRequest {
        title: "Add a note".to_owned(),
        reason: "to keep it".to_owned(),
        proposed_by: RESEARCHER,
        files: vec![NewFile { path: path.to_owned(), content: "a note\n".to_owned() }],
        edits: Vec::new(),
    }
}

fn all_pass(_: &std::path::Path) -> Result<Vec<(Check, bool)>, StewardError> {
    Ok(Check::ALL.into_iter().map(|c| (c, true)).collect())
}

#[test]
fn a_change_is_applied_only_when_verified_and_approved_by_the_owner() {
    let root = fake_repo("change");
    let mut s = open(&root, 100);
    // What may be requested at all.
    let refused = |s: &mut Steward, path: &str| s.propose(&request(path)).unwrap().unwrap_err();
    assert_eq!(refused(&mut s, "../outside.md"), Refusal::PathEscapes("../outside.md".to_owned()));
    assert_eq!(refused(&mut s, "/etc/passwd"), Refusal::PathEscapes("/etc/passwd".to_owned()));
    assert_eq!(refused(&mut s, "crates/kernel/src/lib.rs"), Refusal::OutsidePermittedPlaces("crates/kernel/src/lib.rs".to_owned()));
    assert_eq!(s.propose(&ChangeRequest { files: Vec::new(), ..request("x") }).unwrap(), Err(Refusal::Empty));

    let id = s.propose(&request("docs/research/note.md")).unwrap().unwrap();
    let missing = |s: &Steward| s.progress(&id).unwrap().missing;
    assert_eq!(missing(&s), vec![Missing::AssessedSound, Missing::PassingVerification, Missing::OwnersApproval]);
    // The proposer's own word counts for nothing.
    let _ = s.world.act(&RESEARCHER, attest(About::Supports(Prop::Sound(id.clone())), Method::Ack, "trust me")).unwrap();
    assert_eq!(s.world.epi().unwrap().assessment(&Prop::Sound(id.clone())), Assessment::Unsupported);

    // A verification in which not every check ran establishes nothing.
    assert!(!s.verify_with(&id, |_| Ok(vec![(Check::Format, true)])).unwrap());
    assert!(missing(&s).contains(&Missing::PassingVerification));
    // The owner's approval alone is not enough either.
    s.owner_rules_on(&id, true).unwrap();
    assert_eq!(missing(&s), vec![Missing::PassingVerification]);
    assert_eq!(s.apply(&id).unwrap(), Err(vec![Missing::PassingVerification]));
    assert!(!root.join("docs/research/note.md").exists());

    // Verified (every check, passing), with the reviewer objecting: the owner's approval outranks
    // the objection, and now -- only now -- it is applied.
    let mut seen_file = false;
    assert!(s
        .verify_with(&id, |sandbox| {
            seen_file = sandbox.join("docs/research/note.md").exists() && sandbox.join("Cargo.toml").exists();
            all_pass(sandbox)
        })
        .unwrap());
    assert!(seen_file, "the checks run against a copy of the repository with the change in it");
    let _ = s.world.act(&REVIEWER, attest(About::Rebuts(Prop::Sound(id.clone())), Method::Ack, "I object")).unwrap();
    assert_eq!(missing(&s), Vec::<Missing>::new());
    assert_eq!(s.apply(&id).unwrap(), Ok(()));
    assert_eq!(std::fs::read_to_string(root.join("docs/research/note.md")).unwrap(), "a note\n");
    assert_eq!(s.apply(&id).unwrap(), Err(vec![Missing::AlreadyApplied]));
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn neither_a_failed_verification_nor_the_owners_rejection_can_be_outweighed() {
    let root = fake_repo("change-no");
    let mut s = open(&root, 100);
    // Failed verification, then approval: assessed sound (the owner outranks a test), still not applicable.
    let a = s.propose(&request("docs/research/a.md")).unwrap().unwrap();
    assert!(!s.verify_with(&a, |_| Ok(Check::ALL.into_iter().map(|c| (c, c != Check::Tests)).collect())).unwrap());
    s.owner_rules_on(&a, true).unwrap();
    assert_eq!(s.world.epi().unwrap().assessment(&Prop::Sound(a.clone())), Assessment::Accepted);
    assert_eq!(s.apply(&a).unwrap(), Err(vec![Missing::PassingVerification]));
    // ...and a later passing run does not erase the failure.
    assert!(s.verify_with(&a, all_pass).is_ok());
    assert!(s.progress(&a).unwrap().missing.contains(&Missing::PassingVerification));

    // Verified, then rejected by the owner: refuted, and a later approval does not undo the rejection.
    let b = s.propose(&ChangeRequest { title: "Another".to_owned(), ..request("docs/research/b.md") }).unwrap().unwrap();
    assert!(s.verify_with(&b, all_pass).unwrap());
    s.owner_rules_on(&b, false).unwrap();
    assert_eq!(s.world.epi().unwrap().assessment(&Prop::Sound(b.clone())), Assessment::Refuted);
    s.owner_rules_on(&b, true).unwrap();
    assert!(s.apply(&b).unwrap().unwrap_err().contains(&Missing::OwnersApproval));
    assert!(!root.join("docs/research/b.md").exists());
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn what_is_applied_is_exactly_what_was_verified_and_approved() {
    let root = fake_repo("change-tamper");
    let mut s = open(&root, 100);
    let id = s.propose(&request("docs/research/note.md")).unwrap().unwrap();
    assert!(s.verify_with(&id, all_pass).unwrap());
    s.owner_rules_on(&id, true).unwrap();
    // Someone edits the stored request after it was verified and approved.
    let stored = root.join(format!(".rigor/changes/{id}.json"));
    std::fs::write(&stored, std::fs::read_to_string(&stored).unwrap().replace("a note", "something else entirely")).unwrap();
    assert!(matches!(s.apply(&id), Err(StewardError::ChangeAltered(_))));
    assert!(!root.join("docs/research/note.md").exists());
    // A file that appeared in the meantime is never overwritten.
    let other = s.propose(&request("docs/research/late.md")).unwrap().unwrap();
    assert!(s.verify_with(&other, all_pass).unwrap());
    s.owner_rules_on(&other, true).unwrap();
    std::fs::create_dir_all(root.join("docs/research")).unwrap();
    std::fs::write(root.join("docs/research/late.md"), "written by a person meanwhile").unwrap();
    assert_eq!(s.apply(&other).unwrap(), Err(vec![Missing::StillFits]));
    assert_eq!(std::fs::read_to_string(root.join("docs/research/late.md")).unwrap(), "written by a person meanwhile");
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn a_project_founded_under_an_older_constitution_is_refounded_with_its_history() {
    use rigor::model::{Constitution, Member, MemberKind};
    use rigor::org::{org, standard_rules, Trusted};
    use rigor::steward::{MAINTAINER, PROJECT};
    let root = fake_repo("refound");
    let log = root.join(".rigor/steward.jsonl");
    // A log as the first steward wrote it: three members, no owner, no models, no budget.
    let m = |id, kind| Member { id, kind, principal: None, role: None };
    let old = Constitution {
        members: vec![m(PROJECT, MemberKind::Org), m(MAINTAINER, MemberKind::Human), m(CI, MemberKind::Service)],
        charters: Vec::new(),
        balances: Vec::new(),
        rules: standard_rules(&Trusted { receipts: CI, probes: CI, tests: CI, audits: PROJECT, acks: PROJECT }),
    };
    let mut w = rigor::world::World::new(Box::new(org(Vec::new()).unwrap()));
    let _ = w.inject(&rigor::world::CONSTITUTION, Event::Genesis(Box::new(old))).unwrap();
    let _ = w.tick(90).unwrap();
    let offer = rigor::model::Offer {
        creditor: PROJECT,
        ant: None,
        cons: rigor::model::Condition::Holds(Prop::Conforms(Requirement::lit("an-old-undertaking"))),
        deadline: rigor::model::Time(95),
        tags: BTreeSet::from([Tag::Task]),
        parent: None,
        terms: None,
    };
    let _ = w.act(&MAINTAINER, Act::Commit(offer)).unwrap();
    std::fs::create_dir_all(root.join(".rigor")).unwrap();
    rigor::adapters::save(&w, &log).unwrap();
    let old_head = w.rt.log().head();

    let s = open(&root, 100);
    assert!(s.refounded);
    // The new record says what it continues from, the old file is kept, and nothing was lost.
    assert!(matches!(s.world.rt.log().entries()[0].event, Event::Migrated { old_head: h, .. } if h == old_head));
    assert!(root.join(".rigor/steward.before-refounding-day-100.jsonl").exists());
    let tasks = s.commitments(Tag::Task).unwrap();
    // (It is still shown as in force: re-founding replays the old record, it does not advance the clock.)
    assert!(tasks.iter().any(|(c, st)| c.terms.deadline == rigor::model::Time(95) && c.terms.debtor == MAINTAINER && *st == Status::Detached));
    // And the owner now exists, with an account and the power to set a budget.
    assert!(s.world.inst().unwrap().valid(&CapId::Own(OWNER)));
    assert_eq!(s.world.ledger().unwrap().balance(&OWNER, &MICRO_USD).0, 5_000_000);
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn the_dashboard_puts_what_needs_the_owner_first() {
    let root = fake_repo("dashboard");
    let mut s = open(&root, 100);
    let id = s.propose(&request("docs/research/note.md")).unwrap().unwrap();
    assert!(s.verify_with(&id, all_pass).unwrap());
    let report = s.examine_itself(&no_gateway()).unwrap();
    let page = s.dashboard(100, &report).unwrap();
    let needs = page.find("Needs you").unwrap();
    let ask = page.find("it is verified and waits only for you").unwrap();
    let glance = page.find("At a glance").unwrap();
    assert!(needs < ask && ask < glance);
    assert!(page.contains("The budget for consulting models is spent or was never set"));
    assert!(page.contains("establish-truth") && page.contains("by design it never does this"));
    // Text that came from outside is escaped.
    let hostile = ChangeRequest { title: "<script>alert(1)</script>".to_owned(), ..request("docs/research/h.md") };
    let _ = s.propose(&hostile).unwrap().unwrap();
    let page = s.dashboard(100, &report).unwrap();
    assert!(!page.contains("<script>alert(1)</script>") && page.contains("&lt;script&gt;"));
    let _ = Name::lit("x");
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn an_existing_file_is_edited_only_against_the_content_the_edit_was_written_for() {
    use rigor::kernel::digest_bytes;
    use rigor::model::ContentDigest;
    use rigor::steward::change::Edit;
    let root = fake_repo("change-edit");
    std::fs::create_dir_all(root.join("docs/research")).unwrap();
    let note = root.join("docs/research/note.md");
    std::fs::write(&note, "first version\n").unwrap();
    let was = ContentDigest(digest_bytes(b"first version\n"));
    // (A one-line file whose one line changes keeps nothing; the guard is about losing most of
    // a file of some length, so the note used below has a line that stays.)
    std::fs::write(&note, "title\nfirst version\n").unwrap();
    let was = {
        let _ = was;
        ContentDigest(digest_bytes(b"title\nfirst version\n"))
    };
    let edit = |path: &str, was: ContentDigest| ChangeRequest {
        title: "Revise the note".to_owned(),
        reason: "it was wrong".to_owned(),
        proposed_by: RESEARCHER,
        files: Vec::new(),
        edits: vec![Edit { path: path.to_owned(), was, content: "title\nsecond version\n".to_owned() }],
    };
    let mut s = open(&root, 100);
    // Nothing the system proposes can touch the code or proofs it is trusted for.
    assert_eq!(s.propose(&edit("crates/lib.rs", was)).unwrap(), Err(Refusal::OutsidePermittedPlaces("crates/lib.rs".to_owned())));
    assert_eq!(s.propose(&edit("proofs/t.lean", was)).unwrap(), Err(Refusal::OutsidePermittedPlaces("proofs/t.lean".to_owned())));
    assert_eq!(s.propose(&edit("docs/research/missing.md", was)).unwrap(), Err(Refusal::NothingToEdit("docs/research/missing.md".to_owned())));
    assert_eq!(
        s.propose(&edit("docs/research/note.md", ContentDigest(digest_bytes(b"something else")))).unwrap(),
        Err(Refusal::Stale("docs/research/note.md".to_owned()))
    );

    // An "edit" that throws most of the file away is not a revision and is refused outright,
    // whatever a reviewer or the owner might have thought of it. (Found in real use: a model
    // returned only the paragraph it had changed, and both the checks and the reviewing model
    // let the one-line replacement through.)
    let long = (0..20).map(|i| format!("line {i}\n")).collect::<String>();
    std::fs::write(root.join("docs/research/long.md"), &long).unwrap();
    let gutted = ChangeRequest {
        edits: vec![Edit {
            path: "docs/research/long.md".to_owned(),
            was: ContentDigest(digest_bytes(long.as_bytes())),
            content: "line 3, reworded\n".to_owned(),
        }],
        ..edit("x", was)
    };
    assert_eq!(s.propose(&gutted).unwrap(), Err(Refusal::DiscardsMostOfTheFile { path: "docs/research/long.md".to_owned(), lines_before: 20, lines_kept: 0 }));
    assert_eq!(rigor::steward::change::line_change("a\nb\nc\n", "a\nB\nc\nd\n"), (3, 2, 2));

    let id = s.propose(&edit("docs/research/note.md", was)).unwrap().unwrap();
    assert_eq!(s.progress(&id).unwrap().size, rigor::steward::change::Size { files_added: 0, files_edited: 1, lines_removed: 1, lines_new: 1 });
    let mut sandboxed = String::new();
    assert!(s
        .verify_with(&id, |sandbox| {
            sandboxed = std::fs::read_to_string(sandbox.join("docs/research/note.md")).unwrap();
            all_pass(sandbox)
        })
        .unwrap());
    assert_eq!(sandboxed, "title\nsecond version\n"); // the checks saw the edited file
    assert_eq!(std::fs::read_to_string(&note).unwrap(), "title\nfirst version\n"); // the real one is untouched
    s.owner_rules_on(&id, true).unwrap();
    // A person edits the file between approval and application: the change no longer fits, and is not forced.
    std::fs::write(&note, "edited by a person meanwhile\n").unwrap();
    assert_eq!(s.apply(&id).unwrap(), Err(vec![Missing::StillFits]));
    assert_eq!(std::fs::read_to_string(&note).unwrap(), "edited by a person meanwhile\n");
    // Restored to what was verified, it applies.
    std::fs::write(&note, "title\nfirst version\n").unwrap();
    assert_eq!(s.apply(&id).unwrap(), Ok(()));
    assert_eq!(std::fs::read_to_string(&note).unwrap(), "title\nsecond version\n");
    std::fs::remove_dir_all(root).unwrap();
}

//! Red-team reproductions. EACH TEST PASSES WHEN THE DEFECT IS PRESENT: the assertions state
//! the wrong behaviour that was observed. See redteam/attack_fable.md.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::indexing_slicing)]

use rigor::actors::CREDIT;
use rigor::adapters::{audit_snapshot, restore, resume_attested, save, save_snapshot};
use rigor::kernel::{Cell, Digest, Entity, Entry, Envelope, Leaf, Name, Unit};
use rigor::model::{
    About, Act, ActionKind, ActorId, Amendment, Amount, Assessment, Attestation, CapId, Charter, CharterName, CommitmentId, Condition, Constitution, Content,
    ContentDigest, Event, EvidenceId, Finding, Kind, LeafError, Member, MemberKind, Memo, Method, Offer, Prop, Requirement, Resource, Status, Tag, Task, Terms,
    Time, Transfer, Version,
};
use rigor::org::{org, standard_rules, Trusted};
use rigor::scenario::{
    attest, budget, constitution, delegated, denial, pay, root, rules, run, Cast, AGENT, ALICE, AUDITOR, CI, DEPLOYER, GOV_ALICE, GOV_AUDITOR, MONITOR, TASK1,
    VENDOR,
};
use rigor::steward::{Check, Outcome, Steward};
use rigor::units::envoy::Envoy;
use rigor::world::{World, CONSTITUTION};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::Command;

fn scratch(name: &str) -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("redteam-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}
fn fact(n: &'static str) -> Prop {
    Prop::Fact(Name::new(n).unwrap())
}
fn plain() -> World {
    let mut w = World::new(Box::new(org(Vec::new()).unwrap()));
    let _ = w.inject(&CONSTITUTION, Event::Genesis(Box::new(constitution(rules())))).unwrap();
    w
}
fn recorded(entries: &[Entry<Event>]) -> EvidenceId {
    entries
        .iter()
        .find_map(|e| match &e.event {
            Event::EvidenceRecorded(ev) => Some(ev.id),
            _ => None,
        })
        .expect("evidence recorded")
}
fn unscoped(e: &Event) -> &Event {
    match e {
        Event::Scoped { inner, .. } => unscoped(inner),
        other => other,
    }
}
fn created(entries: &[Entry<Event>]) -> CommitmentId {
    entries
        .iter()
        .find_map(|e| match unscoped(&e.event) {
            Event::NormCreated(c) => Some(c.id),
            _ => None,
        })
        .expect("commitment created")
}
fn infer(about: About, from: EvidenceId) -> Act {
    Act::Attest(Attestation {
        about,
        method: Method::Inference,
        derived_from: BTreeSet::from([from]),
        artifacts: BTreeMap::new(),
        content: Content::Note("because I say so".to_owned()),
    })
}
fn offer(creditor: ActorId, cons: Condition, deadline: u64, tags: &[Tag]) -> Offer {
    Offer { creditor, ant: None, cons, deadline: Time(deadline), tags: tags.iter().copied().collect(), parent: None, terms: None }
}
fn nobody() -> Name {
    Name::lit("nobody")
}

// ================================================================ 1. the boundary

const ENVOY: ActorId = ActorId::lit("envoy");
const ENG: ActorId = ActorId::lit("eng");
const QA: ActorId = ActorId::lit("qa");

/// A unit OUTSIDE a boundary that emits whatever it was built with, wrapped as "internal".
#[derive(Clone, Debug, Hash, serde::Serialize, serde::Deserialize)]
struct Rogue {
    emits: Vec<Event>,
}
impl Leaf<Event> for Rogue {
    fn name(&self) -> Name {
        Name::lit("mallory")
    }
    fn inputs(&self) -> BTreeSet<Kind> {
        BTreeSet::from([Kind::Tick])
    }
    fn outputs(&self) -> BTreeSet<Kind> {
        BTreeSet::from([Kind::Scoped])
    }
    fn on(&mut self, _: &Envelope<'_, Event>) -> Result<Vec<Event>, LeafError> {
        Ok(std::mem::take(&mut self.emits))
    }
}

fn vendor_org(extra: Vec<Box<dyn Unit<Event>>>) -> World {
    let inner = org(vec![Box::new(Cell(Envoy::new(ENVOY, VENDOR, ENG)))]).unwrap();
    let vendor = Entity::federated(VENDOR.0, Box::new(inner), [Kind::Tick, Kind::NormCreated].into(), [Kind::Do, Kind::Signal].into());
    let mut parts: Vec<Box<dyn Unit<Event>>> = vec![Box::new(vendor)];
    parts.extend(extra);
    let mut w = World::new(Box::new(org(parts).unwrap()));
    let _ = w.inject(&CONSTITUTION, Event::Genesis(Box::new(constitution(rules())))).unwrap();
    let m = |id: ActorId, kind| Member { id, kind, principal: None, role: None };
    let inner = Constitution {
        members: vec![m(ENVOY, MemberKind::Service), m(ENG, MemberKind::Human), m(QA, MemberKind::Service)],
        charters: Vec::new(),
        balances: vec![(ENG, CREDIT, Amount(100))],
        rules: standard_rules(&Trusted { receipts: QA, probes: QA, tests: QA, audits: QA, acks: ENVOY }),
    };
    w.scope = vec![VENDOR.0];
    let _ = w.inject(&CONSTITUTION, Event::Genesis(Box::new(inner))).unwrap();
    w.scope = Vec::new();
    w
}

/// An outer unit that is not even a member forges the INNER institution's `Ok` and an inner
/// `Assessed` by wrapping them as `Scoped`. Composition accepts the unit; the inner ledger
/// moves money and the inner norms unit believes the assessment.
#[test]
fn rt01_an_outside_unit_forges_events_inside_a_federated_boundary() {
    let steal = Transfer { from: ENG, to: QA, resource: CREDIT, amount: Amount(100), memo: Memo::None };
    let p = fact("inner-work-done");
    let forged = vec![
        Event::Scoped { entity: VENDOR.0, inner: Box::new(Event::Ok { by: ENG, via: CapId::Own(ENG), act: Act::Transfer(steal) }) },
        Event::Scoped {
            entity: VENDOR.0,
            inner: Box::new(Event::Assessed {
                prop: p.clone(),
                assessment: Assessment::Accepted,
                since: Some(Time(0)),
                basis: Vec::new(),
                was: Assessment::Unsupported,
                rules: Version(1),
            }),
        },
    ];
    let mut w = vendor_org(vec![Box::new(Cell(Rogue { emits: forged }))]);
    w.scope = vec![VENDOR.0];
    let c = created(&w.act(&ENG, Act::Commit(offer(ENVOY, Condition::Holds(p.clone()), 9, &[Tag::Task]))).unwrap());
    assert_eq!(w.norms().unwrap().status(&c), Some(Status::Detached));
    w.scope = Vec::new();
    let _ = w.tick(1).unwrap();
    w.scope = vec![VENDOR.0];
    assert_eq!(w.ledger().unwrap().balance(&QA, &CREDIT), Amount(100), "inner funds moved with no inner Do and no capability");
    assert_eq!(w.ledger().unwrap().balance(&ENG, &CREDIT), Amount(0));
    assert_eq!(w.epi().unwrap().assessment(&p), Assessment::Unsupported, "the inner evidence unit never assessed it");
    assert_eq!(w.norms().unwrap().status(&c), Some(Status::Satisfied), "yet the inner commitment reads satisfied");
    // No inner Do by eng for a transfer exists anywhere in the log.
    assert!(!w
        .rt
        .log()
        .entries()
        .iter()
        .any(|e| matches!(&e.event, Event::Scoped { inner, .. } if matches!(inner.as_ref(), Event::Do { act: Act::Transfer(_), .. }))));
}

/// The runtime accepts ANY event kind as an external stimulus, from anyone. Institutional
/// events injected directly bypass the count-as gate; a stored log containing them restores.
#[test]
fn rt02_institutional_events_are_accepted_as_external_stimuli() {
    let mut w = plain();
    // (a) a non-member moves Alice's money.
    let _ = w.inject(&nobody(), Event::Ok { by: ALICE, via: CapId::Own(ALICE), act: pay(ALICE, VENDOR, 10) }).unwrap();
    assert_eq!(w.ledger().unwrap().balance(&VENDOR, &CREDIT), Amount(10));
    // (b) a forged TransferFailed refunds a budget that was really spent: 50-credit budget, 80 spent.
    let cap = delegated(&w.act_with(&ALICE, &CapId::Own(ALICE), budget(AGENT, ALICE, 50, true)).unwrap()).unwrap();
    assert_eq!(denial(&w.act_with(&AGENT, &cap, pay(ALICE, AGENT, 40)).unwrap()), None);
    assert!(denial(&w.act_with(&AGENT, &cap, pay(ALICE, AGENT, 40)).unwrap()).is_some());
    let spent = Transfer { from: ALICE, to: AGENT, resource: CREDIT, amount: Amount(40), memo: Memo::None };
    let _ = w.inject(&nobody(), Event::TransferFailed { transfer: spent, via: cap.clone(), reason: rigor::model::LedgerError::InsufficientFunds }).unwrap();
    assert_eq!(denial(&w.act_with(&AGENT, &cap, pay(ALICE, AGENT, 40)).unwrap()), None);
    assert_eq!(w.ledger().unwrap().balance(&AGENT, &CREDIT), Amount(80), "80 moved under a 50 budget");
    assert!(w.inst().unwrap().audit().is_empty(), "and the institution's own audit sees nothing wrong");
    // (c) a forged assessment satisfies a commitment; a forged verdict imposes a duty.
    let p = fact("p");
    let c = created(&w.act(&VENDOR, Act::Commit(offer(ALICE, Condition::Holds(p.clone()), 5, &[Tag::Task]))).unwrap());
    let forged =
        Event::Assessed { prop: p, assessment: Assessment::Accepted, since: Some(Time(0)), basis: Vec::new(), was: Assessment::Unsupported, rules: Version(1) };
    let _ = w.inject(&nobody(), forged).unwrap();
    assert_eq!(w.norms().unwrap().status(&c), Some(Status::Satisfied));
    // (d) anyone is the clock: one tick to the end of time expires every capability for good.
    let _ = w.inject(&nobody(), Event::Tick(Time(u64::MAX))).unwrap();
    assert!(denial(&w.act_with(&AGENT, &cap, pay(ALICE, AGENT, 1)).unwrap()).is_some());
    // (e) the whole thing persists and restores as a valid history.
    let dir = scratch("rt02");
    let path = dir.join("log.jsonl");
    save(&w, &path).unwrap();
    let back = restore(Box::new(org(Vec::new()).unwrap()), &path).unwrap();
    assert_eq!(back.rt.log().head(), w.rt.log().head());
    std::fs::remove_dir_all(dir).unwrap();
}

// ================================================================ 2. evidence

/// "Inference" is open to anyone and inherits the rank of whatever it cites. A member trusted
/// for nothing overturns a probe, accepts and refutes arbitrary propositions, and certifies
/// its own claim, by citing an unrelated audit.
#[test]
fn rt03_an_untrusted_member_decides_any_assessment_by_citing_unrelated_evidence() {
    let mut w = plain();
    let (deployed, other) = (fact("deployed"), fact("unrelated"));
    let _ = w.act(&DEPLOYER, attest(About::Supports(deployed.clone()), Method::Receipt, "job accepted")).unwrap();
    let probe = recorded(&w.act(&MONITOR, attest(About::Rebuts(deployed.clone()), Method::Probe, "nothing there")).unwrap());
    assert_eq!(w.epi().unwrap().assessment(&deployed), Assessment::Refuted);
    let audit = recorded(&w.act(&AUDITOR, attest(About::Supports(other), Method::Audit, "books are fine")).unwrap());

    // VENDOR is trusted for no method at all.
    // (a) it knocks out the monitor's probe.
    let _ = w.act(&VENDOR, infer(About::Undercuts(probe), audit)).unwrap();
    assert_eq!(w.epi().unwrap().assessment(&deployed), Assessment::Accepted);
    // (b) it makes anything accepted...
    let moon = fact("the-moon-is-cheese");
    let _ = w.act(&VENDOR, infer(About::Supports(moon.clone()), audit)).unwrap();
    assert_eq!(w.epi().unwrap().assessment(&moon), Assessment::Accepted);
    // (c) ...refutes what a trusted probe supports (its inference carries the audit's rank, 4 > 3)...
    let live = fact("service-is-live");
    let _ = w.act(&MONITOR, attest(About::Supports(live.clone()), Method::Probe, "200 OK")).unwrap();
    let _ = w.act(&VENDOR, infer(About::Rebuts(live.clone()), audit)).unwrap();
    assert_eq!(w.epi().unwrap().assessment(&live), Assessment::Refuted);
    // (d) ...and certifies its own claim although self-support is off and entrenched.
    let mine = fact("vendor-did-the-work");
    let _ = w.act(&VENDOR, Act::Claim { prop: mine.clone() }).unwrap();
    let _ = w.act(&VENDOR, infer(About::Supports(mine.clone()), audit)).unwrap();
    assert_eq!(w.epi().unwrap().assessment(&mine), Assessment::Accepted);
    assert!(!w.gov().unwrap().rules().unwrap().evidence.self_support);
}

/// The demonstrator, reversed by the culpable agent with one act.
#[test]
fn rt04_the_culpable_agent_undoes_the_demonstrators_refutation() {
    let (mut w, o) = run(Cast::default(), false, |_| {}).unwrap();
    assert_eq!(w.norms().unwrap().status(&o.c1), Some(Status::Violated));
    assert!(w.adj().unwrap().findings(&o.c1).contains(&(Finding::Culpable, AGENT)));
    let deployed = Prop::Done(Task::whole(TASK1).step(rigor::actors::DEPLOY));
    let ev: Vec<_> = w.epi().unwrap().evidence().cloned().collect();
    let probe = ev.iter().find(|e| e.about == About::Rebuts(deployed.clone())).unwrap().id;
    // Anything admissible and unrelated will do: Alice's acknowledgement of the audit report.
    let unrelated = ev.iter().find(|e| e.source == ALICE && e.method == Method::Ack).unwrap().id;
    let _ = w.act(&AGENT, infer(About::Undercuts(probe), unrelated)).unwrap();
    assert_eq!(w.epi().unwrap().assessment(&deployed), Assessment::Accepted);
    assert_eq!(w.norms().unwrap().status(&o.c1), Some(Status::Satisfied));
    assert_eq!(w.norms().unwrap().status(&o.c2), Some(Status::Satisfied));
    assert!(w.adj().unwrap().findings(&o.c1).is_empty(), "the verdict against the agent is withdrawn");
}

// ================================================================ 3. responsibility

/// `Signal` is unauthenticated, untimed, and last-write-wins. The culpable agent (or a
/// non-member) erases "it was notified of the failure" after the verdict.
#[test]
fn rt05_a_signal_from_anyone_erases_culpability_and_liability() {
    let (mut w, o) = run(Cast::default(), false, |_| {}).unwrap();
    let before = w.adj().unwrap().findings(&o.c1);
    assert!(before.contains(&(Finding::Culpable, AGENT)) && before.contains(&(Finding::Liable, VENDOR)));
    let deployed = Prop::Done(Task::whole(TASK1).step(rigor::actors::DEPLOY));
    let _ = w.inject(&nobody(), Event::Signal { to: AGENT, prop: deployed, value: true }).unwrap();
    let after = w.adj().unwrap().findings(&o.c1);
    assert_eq!(after, BTreeSet::from([(Finding::Accountable, AGENT), (Finding::Contributed, DEPLOYER)]));
    assert_eq!(w.one(&VENDOR, Tag::Remediation).unwrap().status, Some(Status::Void), "the vendor's remediation duty is voided");
}

/// The converse: a notification sent long AFTER the report makes the agent "culpable: it knew".
#[test]
fn rt06_a_notification_after_the_fact_manufactures_culpability() {
    let cast = Cast { service_truthful: false, ..Cast::default() };
    let (mut w, o) = run(cast, false, |_| {}).unwrap();
    let before = w.adj().unwrap().findings(&o.c1);
    assert!(!before.iter().any(|(f, _)| matches!(f, Finding::Culpable | Finding::Liable)), "{before:?}");
    let deployed = Prop::Done(Task::whole(TASK1).step(rigor::actors::DEPLOY));
    let _ = w.inject(&nobody(), Event::Signal { to: AGENT, prop: deployed, value: false }).unwrap();
    let after = w.adj().unwrap().findings(&o.c1);
    assert!(after.contains(&(Finding::Culpable, AGENT)) && after.contains(&(Finding::Liable, VENDOR)), "{after:?}");
}

/// Anyone may hang a commitment under anyone's commitment as its "sub-commitment". A debtor
/// who fabricates a failed one of its own is "excused" and no longer liable.
#[test]
fn rt07_a_debtor_escapes_liability_by_fabricating_a_failed_subcommitment() {
    let mut honest = plain();
    let main = created(&honest.act(&VENDOR, Act::Commit(offer(ALICE, Condition::Holds(fact("job")), 5, &[Tag::Task]))).unwrap());
    let mut cheat = honest.fork();
    // Baseline: the debtor lets it lapse.
    let _ = honest.tick(6).unwrap();
    let f = honest.adj().unwrap().findings(&main);
    assert_eq!(f, BTreeSet::from([(Finding::Accountable, VENDOR), (Finding::Liable, VENDOR)]));
    assert_eq!(honest.one(&VENDOR, Tag::Remediation).unwrap().status, Some(Status::Detached));
    // Same lapse, but first: a commitment to itself "in service of" the main one, repudiated at once.
    let mut sub = offer(VENDOR, Condition::Holds(fact("whatever")), 5, &[]);
    sub.parent = Some(main);
    let sub = created(&cheat.act(&VENDOR, Act::Commit(sub)).unwrap());
    let _ = cheat.act(&VENDOR, Act::Cancel { id: sub }).unwrap();
    let _ = cheat.tick(6).unwrap();
    let f = cheat.adj().unwrap().findings(&main);
    assert_eq!(f, BTreeSet::from([(Finding::Accountable, VENDOR), (Finding::Excused, VENDOR), (Finding::Contributed, VENDOR)]));
    assert!(cheat.tagged(Tag::Remediation).unwrap().is_empty(), "no remediation duty arises");
}

/// The refund duty takes its amount and its beneficiary from terms the debtor wrote itself;
/// nothing checks that anything was ever paid. An AI agent thereby puts its principal in debt.
#[test]
fn rt08_an_ai_agent_indebts_its_principal_for_a_refund_of_money_never_paid() {
    let mut w = plain();
    let mut o = offer(DEPLOYER, Condition::Holds(fact("never")), 1, &[Tag::Task, Tag::Paid]);
    o.terms = Some(Terms { price: Amount(1_000_000), resource: CREDIT, payer: DEPLOYER });
    let c = created(&w.act(&AGENT, Act::Commit(o)).unwrap());
    let _ = w.tick(2).unwrap();
    assert!(w.adj().unwrap().findings(&c).contains(&(Finding::Liable, VENDOR)));
    let refund = w.one(&VENDOR, Tag::Refund).unwrap();
    assert_eq!(refund.terms().cons, Condition::Paid { from: VENDOR, to: DEPLOYER, resource: CREDIT, at_least: Amount(1_000_000) });
    assert_eq!(refund.status, Some(Status::Detached), "the vendor now owes a million to a 'payer' who paid nothing");
    assert!(!w.rt.log().entries().iter().any(|e| matches!(e.event, Event::Transferred { .. })));
}

// ================================================================ 4. commitments

/// A maintenance commitment that is violated (refuted within its period, still refuted) flips
/// to SATISFIED when one more refutation arrives after the deadline.
#[test]
fn rt09_more_refutation_after_the_deadline_cures_a_violated_maintenance_commitment() {
    use rigor::gitlog::{found, ADMISSIONS, PROJECT, REGISTRAR};
    const NEWCOMER: ActorId = ActorId::lit("newcomer");
    let mut w = found().unwrap();
    let m = Member { id: NEWCOMER, kind: MemberKind::Human, principal: None, role: None };
    let _ = w.act_with(&REGISTRAR, &CapId::Charter(ADMISSIONS), Act::Admit(m)).unwrap();
    let b = Prop::Sound(rigor::model::ChangeId(Name::lit("bbbb")));
    let k = created(&w.act(&NEWCOMER, Act::Commit(offer(PROJECT, Condition::Unrefuted(b.clone()), 10, &[Tag::Change]))).unwrap());
    let _ = w.tick(5).unwrap();
    let _ = w.act(&PROJECT, attest(About::Rebuts(b.clone()), Method::Reversal, "undo b")).unwrap();
    let _ = w.tick(11).unwrap();
    assert_eq!(w.norms().unwrap().status(&k), Some(Status::Violated));
    assert!(w.adj().unwrap().findings(&k).contains(&(Finding::Accountable, NEWCOMER)));
    let _ = w.act(&PROJECT, attest(About::Rebuts(b.clone()), Method::Reversal, "and it is still broken")).unwrap();
    assert_eq!(w.epi().unwrap().assessment(&b), Assessment::Refuted, "still refuted, now twice over");
    assert_eq!(w.norms().unwrap().status(&k), Some(Status::Satisfied));
    assert!(w.adj().unwrap().findings(&k).is_empty());
}

/// One payment discharges every debt between the same two parties; the memo is never read.
#[test]
fn rt10_one_transfer_discharges_several_debts_and_an_unrelated_one_discharges_a_refund() {
    let mut w = plain();
    let owe = |d| offer(DEPLOYER, Condition::Paid { from: ALICE, to: DEPLOYER, resource: CREDIT, at_least: Amount(30) }, d, &[Tag::Payment]);
    let ids: Vec<_> = (5..8).map(|d| created(&w.act(&ALICE, Act::Commit(owe(d))).unwrap())).collect();
    let t = Transfer { from: ALICE, to: DEPLOYER, resource: CREDIT, amount: Amount(30), memo: Memo::Payment(Task::whole(TASK1)) };
    let _ = w.act_with(&ALICE, &CapId::Own(ALICE), Act::Transfer(t)).unwrap();
    assert!(ids.iter().all(|c| w.norms().unwrap().status(c) == Some(Status::Satisfied)), "three debts of 30");
    assert_eq!(w.ledger().unwrap().balance(&ALICE, &CREDIT), Amount(70), "one payment of 30");
}

/// "A day on which a claim did not hold stays as a violated commitment" and "late evidence
/// does not cure a missed deadline": both fail when the artifact returns to an earlier digest.
#[test]
fn rt11_reverting_an_artifact_turns_a_violation_into_on_time_satisfaction() {
    let mut w = plain();
    let art = rigor::model::ArtifactId::lit("src");
    let (a, b) = (ContentDigest(Digest(1)), ContentDigest(Digest(2)));
    let fs = Name::lit("fs");
    let p = Prop::Conforms(Requirement::lit("tests-pass"));
    let test = |about, d| {
        Act::Attest(Attestation {
            about,
            method: Method::Test,
            derived_from: BTreeSet::new(),
            artifacts: BTreeMap::from([(art.clone(), d)]),
            content: Content::Note(String::new()),
        })
    };
    let _ = w.inject(&fs, Event::ArtifactChanged { artifact: art.clone(), digest: a }).unwrap();
    let _ = w.tick(1).unwrap();
    let _ = w.act(&CI, test(About::Supports(p.clone()), a)).unwrap(); // day 1: passes on A
    let _ = w.tick(2).unwrap();
    let _ = w.inject(&fs, Event::ArtifactChanged { artifact: art.clone(), digest: b }).unwrap(); // day 2: broken
    let _ = w.act(&CI, test(About::Rebuts(p.clone()), b)).unwrap();
    let day2 = created(&w.act(&VENDOR, Act::Commit(offer(ALICE, Condition::Holds(p.clone()), 2, &[Tag::Upkeep]))).unwrap());
    let _ = w.tick(3).unwrap();
    assert_eq!(w.norms().unwrap().status(&day2), Some(Status::Violated));
    assert_eq!(w.one(&VENDOR, Tag::Remediation).unwrap().status, Some(Status::Detached));
    let _ = w.inject(&fs, Event::ArtifactChanged { artifact: art, digest: a }).unwrap(); // day 3: revert
    assert_eq!(w.norms().unwrap().status(&day2), Some(Status::Satisfied), "day 2 now reads as having held on day 2");
    assert_eq!(w.one(&VENDOR, Tag::Remediation).unwrap().status, Some(Status::Void));
    assert!(w.adj().unwrap().findings(&day2).is_empty());
}

/// A derived duty the debtor repudiated is not voided when its basis falls: it stays violated.
#[test]
fn rt12_a_repudiated_secondary_duty_survives_its_basis() {
    let mut w = plain();
    let p = fact("job");
    let main = created(&w.act(&VENDOR, Act::Commit(offer(ALICE, Condition::Holds(p.clone()), 5, &[Tag::Task]))).unwrap());
    let _ = w.tick(1).unwrap();
    let _ = w.act(&MONITOR, attest(About::Supports(p.clone()), Method::Probe, "done")).unwrap();
    let no = recorded(&w.act(&AUDITOR, attest(About::Rebuts(p), Method::Audit, "not done")).unwrap());
    let _ = w.tick(6).unwrap();
    assert_eq!(w.norms().unwrap().status(&main), Some(Status::Violated));
    let rem = w.one(&VENDOR, Tag::Remediation).unwrap().commitment.id;
    let _ = w.act(&VENDOR, Act::Cancel { id: rem }).unwrap();
    let _ = w.act(&AUDITOR, Act::Retract { evidence: no }).unwrap();
    assert_eq!(w.norms().unwrap().status(&main), Some(Status::Satisfied), "the violation never was");
    assert!(w.adj().unwrap().findings(&main).is_empty());
    assert_eq!(w.norms().unwrap().status(&rem), Some(Status::Violated), "but the duty it created is still violated, not void");
    assert!(w.adj().unwrap().findings(&rem).contains(&(Finding::Liable, VENDOR)));
}

// ================================================================ 5. authority, ledger

/// A "50-credit budget" limits credits only: every other resource of the grantor is unlimited.
#[test]
fn rt13_a_budget_in_one_resource_is_unlimited_in_every_other() {
    let gold = Resource::lit("gold");
    let mut c = constitution(rules());
    c.balances.push((ALICE, gold.clone(), Amount(1_000_000)));
    let mut w = World::new(Box::new(org(Vec::new()).unwrap()));
    let _ = w.inject(&CONSTITUTION, Event::Genesis(Box::new(c))).unwrap();
    let cap = delegated(&w.act_with(&ALICE, &CapId::Own(ALICE), budget(AGENT, ALICE, 50, true)).unwrap()).unwrap();
    let all = Transfer { from: ALICE, to: AGENT, resource: gold.clone(), amount: Amount(1_000_000), memo: Memo::None };
    assert_eq!(denial(&w.act_with(&AGENT, &cap, Act::Transfer(all)).unwrap()), None);
    assert_eq!(w.ledger().unwrap().balance(&AGENT, &gold), Amount(1_000_000));
    assert!(w.inst().unwrap().audit().is_empty());
}

/// Saturating arithmetic destroys value while `conserved()` keeps saying true.
#[test]
fn rt14_the_ledger_destroys_value_and_still_reports_conservation() {
    let mut c = constitution(rules());
    c.balances = vec![(ALICE, CREDIT, Amount(u64::MAX)), (DEPLOYER, CREDIT, Amount(40))];
    let mut w = World::new(Box::new(org(Vec::new()).unwrap()));
    let _ = w.inject(&CONSTITUTION, Event::Genesis(Box::new(c))).unwrap();
    let _ = w.act_with(&DEPLOYER, &CapId::Own(DEPLOYER), pay(DEPLOYER, ALICE, 40)).unwrap();
    let l = w.ledger().unwrap();
    assert_eq!((l.balance(&ALICE, &CREDIT), l.balance(&DEPLOYER, &CREDIT)), (Amount(u64::MAX), Amount(0)), "40 credits vanished");
    assert!(l.conserved());
}

// ================================================================ 6. governance

/// A proposer's own approval is counted without the approve power. The reflector ("a charter
/// to propose and nothing else") supplies one of the two approvals; with quorum 1 it enacts alone.
#[test]
fn rt15_a_proposal_only_charter_counts_towards_the_quorum_and_an_expelled_approver_still_counts() {
    const REFLECTOR: ActorId = ActorId::lit("reflector");
    const REFLECT: CharterName = CharterName::lit("reflect");
    const MEMBERSHIP: CharterName = CharterName::lit("membership");
    let founded = |quorum: u32| {
        let mut r = rules();
        r.governance.quorum = quorum;
        let mut c = constitution(r);
        c.members.push(Member { id: REFLECTOR, kind: MemberKind::Service, principal: None, role: None });
        c.charters.push(Charter { name: REFLECT, holder: REFLECTOR, actions: BTreeSet::from([ActionKind::Propose]), delegable: false });
        c.charters.push(Charter { name: MEMBERSHIP, holder: AUDITOR, actions: BTreeSet::from([ActionKind::Expel]), delegable: false });
        let mut w = World::new(Box::new(org(Vec::new()).unwrap()));
        let _ = w.inject(&CONSTITUTION, Event::Genesis(Box::new(c))).unwrap();
        w
    };
    let proposal = |entries: &[Entry<Event>]| {
        entries.iter().find_map(|e| match &e.event {
            Event::Proposed { id, .. } => Some(*id),
            _ => None,
        })
    };
    let change = || Act::Propose { amendments: vec![Amendment::SetRank(Method::Probe, None)] };
    // Quorum 2: one holder of the approve power suffices.
    let mut w = founded(2);
    let id = proposal(&w.act_with(&REFLECTOR, &CapId::Charter(REFLECT), change()).unwrap()).unwrap();
    let _ = w.act_with(&ALICE, &CapId::Charter(GOV_ALICE), Act::Approve { id }).unwrap();
    assert_eq!(w.gov().unwrap().version(), Version(2));
    assert_eq!(w.gov().unwrap().history()[1].by, BTreeSet::from([ALICE, REFLECTOR]));
    // Quorum 1: the proposal-only charter enacts by itself.
    let mut w = founded(1);
    let _ = w.act_with(&REFLECTOR, &CapId::Charter(REFLECT), change()).unwrap();
    assert_eq!(w.gov().unwrap().version(), Version(2));
    assert_eq!(w.gov().unwrap().history()[1].by, BTreeSet::from([REFLECTOR]));
    // Quorum 2: Alice proposes, is expelled, and her approval still makes the quorum.
    let mut w = founded(2);
    let id = proposal(&w.act_with(&ALICE, &CapId::Charter(GOV_ALICE), change()).unwrap()).unwrap();
    let _ = w.act_with(&AUDITOR, &CapId::Charter(MEMBERSHIP), Act::Expel { member: ALICE }).unwrap();
    let _ = w.act_with(&AUDITOR, &CapId::Charter(GOV_AUDITOR), Act::Approve { id }).unwrap();
    assert_eq!(w.gov().unwrap().version(), Version(2));
}

// ================================================================ 7. snapshots, logs

/// The attested proposition `Faithful(digest)` covers the state bytes only. The snapshot's
/// position and chain head are not covered, so a snapshot re-pointed at another history is
/// "attested". And since anyone can get any proposition accepted (rt03), so is a forged state.
#[test]
fn rt16_an_attested_snapshot_can_be_repointed_and_a_forged_one_attested_by_anyone() {
    let dir = scratch("rt16");
    let (mut w, _) = run(Cast::default(), true, |_| {}).unwrap();
    let fresh = || root(Cast::default()).unwrap();
    let good = dir.join("good.snapshot");
    let snap = save_snapshot(&w, &good).unwrap();
    assert!(audit_snapshot(&mut w, fresh(), &snap, &AUDITOR).unwrap());
    // (a) same state, different history.
    let mut moved = snap.clone();
    moved.seq = 3;
    moved.head = Digest(0xDEAD_BEEF);
    let path = dir.join("moved.snapshot");
    std::fs::write(&path, bincode::serialize(&moved).unwrap()).unwrap();
    let resumed = resume_attested(fresh(), &path, &w).unwrap();
    assert_eq!((resumed.rt.log().next_seq(), resumed.rt.log().head()), (3, Digest(0xDEAD_BEEF)));
    assert_ne!(resumed.rt.log().head(), w.rt.log().head());
    // (b) a forged state nobody audited, vouched for by a member trusted for nothing.
    let mut forged = snap.clone();
    let end = forged.state.len() - 1;
    forged.state[end] ^= 1;
    forged.digest = rigor::kernel::digest_bytes(&forged.state);
    let path = dir.join("forged.snapshot");
    std::fs::write(&path, bincode::serialize(&forged).unwrap()).unwrap();
    let audit = w.epi().unwrap().evidence().find(|e| e.method == Method::Audit && e.source == AUDITOR).unwrap().id;
    let _ = w.act(&VENDOR, infer(About::Supports(Prop::Faithful(ContentDigest(forged.digest))), audit)).unwrap();
    assert!(resume_attested(fresh(), &path, &w).is_ok());
    std::fs::remove_dir_all(dir).unwrap();
}

/// A stored log cut off at an earlier point is a valid log: the refutation, the verdicts and
/// everything after them are gone and `restore` raises nothing.
#[test]
fn rt17_a_truncated_log_restores_without_complaint() {
    let dir = scratch("rt17");
    let path = dir.join("log.jsonl");
    let (w, o) = run(Cast::default(), false, |_| {}).unwrap();
    assert_eq!(w.norms().unwrap().status(&o.c1), Some(Status::Violated));
    save(&w, &path).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    let cut = text.lines().position(|l| l.contains("\"Tick\":6")).unwrap();
    let kept: String = text.lines().take(cut).map(|l| format!("{l}\n")).collect();
    std::fs::write(&path, kept).unwrap();
    let back = restore(root(Cast::default()).unwrap(), &path).unwrap();
    assert_eq!(back.norms().unwrap().status(&o.c1), Some(Status::Satisfied));
    assert!(back.adj().unwrap().verdicts().next().is_none());
    std::fs::remove_dir_all(dir).unwrap();
}

// ================================================================ 8. the envoy

/// Any inner member, not only the worker, can make the envoy report failure outward and
/// thereby silence the worker's later success: the first status wins.
#[test]
fn rt18_any_inner_member_makes_the_envoy_report_failure_for_good() {
    let mut w = vendor_org(Vec::new());
    // An outer duty on the vendor.
    let duty = created(&w.act(&VENDOR, Act::Commit(offer(ALICE, Condition::Holds(fact("vendor-delivers")), 20, &[Tag::Task]))).unwrap());
    let task = Envoy::task_for(duty).unwrap();
    w.scope = vec![VENDOR.0];
    let done = Condition::Holds(Prop::Done(task.clone()));
    // QA is not the worker. It "undertakes" the job and repudiates it.
    let spoiler = created(&w.act(&QA, Act::Commit(offer(ENVOY, done.clone(), 3, &[]))).unwrap());
    let _ = w.act(&QA, Act::Cancel { id: spoiler }).unwrap();
    // The real worker then does the job, to the inner institution's satisfaction.
    let real = created(&w.act(&ENG, Act::Commit(offer(ENVOY, done, 9, &[Tag::Task]))).unwrap());
    let _ = w.act(&QA, attest(About::Supports(Prop::Done(task)), Method::Test, "inner checks pass")).unwrap();
    assert_eq!(w.norms().unwrap().status(&real), Some(Status::Satisfied));
    w.scope = Vec::new();
    let outward: Vec<_> = w.rt.log().entries().iter().filter(|e| e.actor.only() == Some(&VENDOR.0)).map(|e| e.event.clone()).collect();
    assert!(outward.iter().any(|e| matches!(e, Event::Signal { to, value: false, .. } if *to == ALICE)), "failure reported outward");
    assert!(!outward.iter().any(|e| matches!(e, Event::Do { act: Act::Claim { .. }, .. })), "success never claimed");
}

// ================================================================ 9. the steward

fn sh(root: &Path, args: &[&str]) -> bool {
    Command::new("cargo").args(args).current_dir(root).output().unwrap().status.success()
}

/// A real, tiny cargo project laid out as the steward expects.
fn tiny_repo(name: &str, lib: &str) -> PathBuf {
    let root = scratch(name);
    std::fs::create_dir_all(root.join("crates")).unwrap();
    std::fs::create_dir_all(root.join("proofs")).unwrap();
    std::fs::write(
        root.join("Cargo.toml"),
        "[package]\nname = \"tiny\"\nversion = \"0.1.0\"\nedition = \"2021\"\n[lib]\npath = \"crates/lib.rs\"\n[workspace]\n",
    )
    .unwrap();
    std::fs::write(root.join("rustfmt.toml"), "").unwrap();
    std::fs::write(root.join("proofs/t.lean"), "").unwrap();
    std::fs::write(root.join("crates/lib.rs"), lib).unwrap();
    std::fs::write(root.join("crates/expected.txt"), "1\n").unwrap();
    assert!(sh(&root, &["generate-lockfile"]));
    root
}
const LIB_OK: &str = "pub fn f() -> &'static str {\n    include_str!(\"expected.txt\").trim()\n}\n#[test]\nfn t() {\n    assert_eq!(f(), \"1\");\n}\n";
const LIB_BROKEN: &str = "pub fn f() -> &'static str {\n    \"2\"\n}\n#[test]\nfn t() {\n    assert_eq!(f(), \"1\");\n}\n";

fn only_tests() -> BTreeSet<Check> {
    BTreeSet::from([Check::Lints, Check::Format, Check::Proofs])
}
fn tests_outcome(o: &[(Check, Outcome)]) -> Outcome {
    o.iter().find(|(c, _)| *c == Check::Tests).unwrap().1
}

/// `steward accept tests-pass`: the project "accepts" that its tests pass. Nothing binds that
/// to any file, so the tests are never run again and the steward reports conformance (exit 0)
/// over a test suite that fails.
#[test]
fn rt19_the_steward_accepts_tests_pass_by_acclamation() {
    let root = tiny_repo("rt19", LIB_BROKEN);
    assert!(!sh(&root, &["test", "--quiet"]), "the real tests fail");
    let log = root.join(".rigor/steward.jsonl");
    let mut s = Steward::open(&root, &log, 100).unwrap();
    s.accept(Requirement::lit("tests-pass"), 100).unwrap(); // what `steward accept tests-pass` does
    let outcomes = s.run(100, false, &only_tests()).unwrap();
    assert_eq!(tests_outcome(&outcomes), Outcome::StillEvidenced);
    assert!(s.conformant(&only_tests()).unwrap(), "exit code 0");
    // And it stays that way whatever the sources become.
    std::fs::write(root.join("crates/lib.rs"), LIB_BROKEN.replace("\"2\"", "\"3\"")).unwrap();
    let mut s = Steward::open(&root, &log, 101).unwrap();
    assert_eq!(tests_outcome(&s.run(101, false, &only_tests()).unwrap()), Outcome::StillEvidenced);
    assert!(s.conformant(&only_tests()).unwrap());
    std::fs::remove_dir_all(root).unwrap();
}

/// The steward watches `*.rs` and `*.toml` only. A test that fails because of any other file
/// (a fixture, here) is still "evidenced by an earlier run".
#[test]
fn rt20_the_steward_does_not_notice_a_change_to_a_test_fixture() {
    let root = tiny_repo("rt20", LIB_OK);
    let log = root.join(".rigor/steward.jsonl");
    let mut s = Steward::open(&root, &log, 100).unwrap();
    assert_eq!(tests_outcome(&s.run(100, false, &only_tests()).unwrap()), Outcome::RanAndPassed);
    std::fs::write(root.join("crates/expected.txt"), "2\n").unwrap();
    assert!(!sh(&root, &["test", "--quiet"]), "the real tests now fail");
    let mut s = Steward::open(&root, &log, 101).unwrap();
    assert_eq!(tests_outcome(&s.run(101, false, &only_tests()).unwrap()), Outcome::StillEvidenced);
    assert!(s.conformant(&only_tests()).unwrap(), "exit code 0");
    std::fs::remove_dir_all(root).unwrap();
}

/// The steward's record of a bad day is erased (a) by reverting the files, (b) by cutting the
/// tail off the log file. Neither is noticed.
#[test]
fn rt21_the_stewards_record_of_a_failed_day_can_be_made_to_disappear() {
    let root = tiny_repo("rt21", LIB_OK);
    let log = root.join(".rigor/steward.jsonl");
    let tests = Prop::Conforms(Requirement::lit("tests-pass"));
    let violated = |s: &Steward| {
        s.commitments(Tag::Upkeep).unwrap().into_iter().filter(|(c, st)| c.terms.cons == Condition::Holds(tests.clone()) && *st == Status::Violated).count()
    };
    let mut s = Steward::open(&root, &log, 100).unwrap();
    assert_eq!(tests_outcome(&s.run(100, false, &only_tests()).unwrap()), Outcome::RanAndPassed);
    let day100 = std::fs::read_to_string(&log).unwrap().lines().count();
    std::fs::write(root.join("crates/lib.rs"), LIB_BROKEN).unwrap();
    let mut s = Steward::open(&root, &log, 101).unwrap();
    assert_eq!(tests_outcome(&s.run(101, false, &only_tests()).unwrap()), Outcome::RanAndFailed);
    let with_bad_day = std::fs::read_to_string(&log).unwrap();
    // Day 102, still broken: day 101 is a violation on the record.
    let mut s = Steward::open(&root, &log, 102).unwrap();
    let _ = s.run(102, false, &only_tests()).unwrap();
    assert_eq!(violated(&s), 2, "day 101, and day 100 too: see rt22");
    // (a) revert the file: day 101 is no longer a violation.
    std::fs::write(root.join("crates/lib.rs"), LIB_OK).unwrap();
    let mut s = Steward::open(&root, &log, 102).unwrap();
    assert_eq!(tests_outcome(&s.run(102, false, &only_tests()).unwrap()), Outcome::StillEvidenced);
    assert_eq!(violated(&s), 0, "the day the tests failed now reads as satisfied");
    // (b) or cut the log back to the end of day 100 and carry on: no trace, no complaint.
    let kept: String = with_bad_day.lines().take(day100).map(|l| format!("{l}\n")).collect();
    std::fs::write(&log, kept).unwrap();
    let mut s = Steward::open(&root, &log, 102).unwrap();
    assert!(s.previous_log_rejected.is_none() && s.migrated_from.is_none());
    let _ = s.run(102, false, &only_tests()).unwrap();
    assert_eq!(violated(&s), 0);
    assert!(!std::fs::read_to_string(&log).unwrap().contains("\"Tick\":101"));
    std::fs::remove_dir_all(root).unwrap();
}

/// Any edit to the sources, however good, retroactively turns every EARLIER day -- on which
/// the tests ran and passed -- into a violation, with a verdict and a remediation duty.
#[test]
fn rt22_an_ordinary_edit_makes_every_past_good_day_a_violation() {
    let root = tiny_repo("rt22", LIB_OK);
    let log = root.join(".rigor/steward.jsonl");
    let tests = Prop::Conforms(Requirement::lit("tests-pass"));
    let mut s = Steward::open(&root, &log, 100).unwrap();
    assert_eq!(tests_outcome(&s.run(100, false, &only_tests()).unwrap()), Outcome::RanAndPassed);
    // Day 101: a harmless edit. The tests are re-run and pass.
    std::fs::write(root.join("crates/lib.rs"), format!("{LIB_OK}// a comment\n")).unwrap();
    let mut s = Steward::open(&root, &log, 101).unwrap();
    assert_eq!(tests_outcome(&s.run(101, false, &only_tests()).unwrap()), Outcome::RanAndPassed);
    assert!(s.conformant(&only_tests()).unwrap());
    let day100: Vec<_> = s
        .commitments(Tag::Upkeep)
        .unwrap()
        .into_iter()
        .filter(|(c, _)| c.terms.cons == Condition::Holds(tests.clone()) && c.terms.deadline == Time(100))
        .collect();
    assert_eq!(day100.len(), 1);
    assert_eq!(day100[0].1, Status::Violated, "day 100, on which the tests passed, is now recorded as violated");
    assert!(s.world.adj().unwrap().findings(&day100[0].0.id).contains(&(Finding::Accountable, rigor::steward::MAINTAINER)));
    let duty = s.commitments(Tag::Remediation).unwrap().into_iter().find(|(c, _)| c.terms.basis.as_ref().is_some_and(|b| b.of == day100[0].0.id));
    assert!(duty.is_some(), "and a duty to put day 100 right exists");
    std::fs::remove_dir_all(root).unwrap();
}

/// "No self-certification" binds only an actor who has formally filed a claim. A debtor who
/// simply attests, and never claims, satisfies its own commitment on its own word.
#[test]
fn rt23_a_debtor_certifies_its_own_commitment_by_not_claiming() {
    let mut w = plain();
    let x = fact("alice-did-it");
    let c = created(&w.act(&ALICE, Act::Commit(offer(VENDOR, Condition::Holds(x.clone()), 5, &[Tag::Task]))).unwrap());
    let _ = w.act(&ALICE, attest(About::Supports(x.clone()), Method::Ack, "trust me")).unwrap();
    assert_eq!(w.epi().unwrap().assessment(&x), Assessment::Accepted);
    assert_eq!(w.norms().unwrap().status(&c), Some(Status::Satisfied));
}

//! Regression suite for the red team's findings (redteam/attack_fable.md, redteam/attack_astra.md).
//!
//! EACH TEST PASSES WHEN THE DEFECT IS ABSENT: the assertions state the correct behaviour, and
//! name the specific refusal or outcome. A test marked `#[ignore = "still reproduces: ..."]`
//! states the correct behaviour of something that is not fixed yet.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::indexing_slicing, missing_docs)]

mod common;
use common::Rng;
use rigor::actors::{CREDIT, DEPLOY};
use rigor::adapters::{audit_snapshot, faithful, restore, resume_attested, save, save_snapshot, AdapterError};
use rigor::logic::datalog::{Atom, Literal, Term, Var};
use rigor::model::{
    upcast, About, Act, ActionKind, ActorId, Amendment, Amount, ArtifactId, Assessment, Attestation, CapId, ChangeId, Charter, CharterName, CommitmentId,
    Condition, Const, Constitution, Content, ContentDigest, DenyReason, Event, EvidenceError, EvidenceId, Fault, Finding, Kind, LeafError, LedgerError, Member,
    MemberKind, Memo, Method, NormError, Offer, Pred, Prop, RRule, Rank, Requirement, Resource, Role, RuleId, RulesChanged, SkipReason, Status, Tag, Task,
    TaskName, Terms, Time, Transfer, Trust, Version,
};
use rigor::org::{open_kinds, org, standard_rules, Trusted};
use rigor::scenario::{
    attest, budget, constitution, delegated, denial, pay, root, rules, run, world, Cast, AGENT, ALICE, AUDITOR, CI, DEPLOYER, GOV_ALICE, GOV_AUDITOR, MONITOR,
    TASK1, VENDOR,
};
use rigor::steward::{AnchorProblem, Check, Outcome, Steward, MAINTAINER, PROJECT};
use rigor::units::envoy::Envoy;
use rigor::units::epistemic::{Epistemic, EPI};
use rigor::waist::{Cell, Digest, Entity, Entry, Envelope, Leaf, Name, Par, Path, Runtime, Unit};
use rigor::world::{Error, World, CONSTITUTION};
use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::process::Command;

// ================================================================ helpers

fn scratch(name: &str) -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("redteam-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}
fn fact(n: &'static str) -> Prop {
    Prop::Fact(Name::new(n).unwrap())
}
fn plain_with(c: Constitution) -> World {
    let mut w = World::new(Box::new(org(Vec::new()).unwrap()));
    let _ = w.inject(&CONSTITUTION, Event::Genesis(Box::new(c))).unwrap();
    w
}
/// The scenario's institution with nobody acting on their own.
fn plain() -> World {
    plain_with(constitution(rules()))
}
fn unscoped(e: &Event) -> &Event {
    match e {
        Event::Scoped { inner, .. } => unscoped(inner),
        other => other,
    }
}
fn recorded(entries: &[Entry<Event>]) -> EvidenceId {
    entries
        .iter()
        .find_map(|e| match unscoped(&e.event) {
            Event::EvidenceRecorded(ev) => Some(ev.id),
            _ => None,
        })
        .expect("evidence recorded")
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
/// Why the evidence unit refused what was offered, if it did.
fn evidence_refusal(entries: &[Entry<Event>]) -> Option<EvidenceError> {
    entries.iter().find_map(|e| match &e.event {
        Event::EvidenceRejected { reason, .. } => Some(reason.clone()),
        _ => None,
    })
}
/// Why the norms unit refused what was offered, if it did.
fn norm_refusal(entries: &[Entry<Event>]) -> Option<NormError> {
    entries.iter().find_map(|e| match &e.event {
        Event::NormRejected { reason, .. } => Some(reason.clone()),
        _ => None,
    })
}
/// The kind the runtime refused to take as a stimulus, if that is why it refused.
fn not_a_stimulus(r: Result<Vec<Entry<Event>>, Error>) -> Option<Kind> {
    match r {
        Err(Error::Fault(Fault::NotAStimulus(k))) => Some(k),
        Err(_) | Ok(_) => None,
    }
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
fn deployed() -> Prop {
    Prop::Done(Task::whole(TASK1).step(DEPLOY))
}

// ---------------------------------------------------------------- a vendor that is an organisation

const ENVOY: ActorId = ActorId::lit("envoy");
const ENG: ActorId = ActorId::lit("eng");
const QA: ActorId = ActorId::lit("qa");
const INTERN: ActorId = ActorId::lit("intern");
const MALLORY: Name = Name::lit("mallory");

/// A unit OUTSIDE a boundary that declares it emits events wrapped as that boundary's own.
#[derive(Clone, Debug, Hash, serde::Serialize, serde::Deserialize)]
struct Rogue {
    emits: Vec<Event>,
}
impl Leaf<Event> for Rogue {
    fn name(&self) -> Name {
        MALLORY
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

fn vendor_entity() -> Box<dyn Unit<Event>> {
    let inner = org(vec![Box::new(Cell(Envoy::new(ENVOY, VENDOR, ENG)))]).unwrap();
    Box::new(Entity::federated(VENDOR.0, Box::new(inner), [Kind::Tick, Kind::NormCreated].into(), [Kind::Do, Kind::Signal].into()))
}

/// The scenario's institution, with the vendor a whole organisation behind a federated boundary.
fn vendor_org() -> World {
    let mut w = plain_with_root(org(vec![vendor_entity()]).unwrap());
    let m = |id: ActorId, kind| Member { id, kind, principal: None, role: None };
    let inner = Constitution {
        members: vec![m(ENVOY, MemberKind::Service), m(ENG, MemberKind::Human), m(QA, MemberKind::Service), m(INTERN, MemberKind::Human)],
        charters: Vec::new(),
        balances: vec![(ENG, CREDIT, Amount(100))],
        rules: standard_rules(&Trusted { receipts: QA, probes: QA, tests: QA, audits: QA, acks: ENVOY }),
    };
    w.scope = vec![VENDOR.0];
    let _ = w.inject(&CONSTITUTION, Event::Genesis(Box::new(inner))).unwrap();
    w.scope = Vec::new();
    w
}
fn plain_with_root(root: Par<Event>) -> World {
    let mut w = World::new(Box::new(root));
    let _ = w.inject(&CONSTITUTION, Event::Genesis(Box::new(constitution(rules())))).unwrap();
    w
}

// ================================================================ fable: critical

// 1. Any member decides any assessment: "inference" launders trust and rank
#[test]
fn fable_01_an_inference_only_supports_what_follows_and_no_interested_party_certifies() {
    let mut w = plain();
    let unrelated = fact("unrelated");
    let _ = w.act(&DEPLOYER, attest(About::Supports(deployed()), Method::Receipt, "job accepted")).unwrap();
    let probe = recorded(&w.act(&MONITOR, attest(About::Rebuts(deployed()), Method::Probe, "nothing there")).unwrap());
    assert_eq!(w.epi().unwrap().assessment(&deployed()), Assessment::Refuted);
    let audit = recorded(&w.act(&AUDITOR, attest(About::Supports(unrelated), Method::Audit, "books are fine")).unwrap());

    // VENDOR is trusted for no method at all.
    // (a) An inference cannot knock out the monitor's probe: it can only support.
    assert_eq!(evidence_refusal(&w.act(&VENDOR, infer(About::Undercuts(probe), audit)).unwrap()), Some(EvidenceError::InferenceMustSupport));
    assert_eq!(w.epi().unwrap().assessment(&deployed()), Assessment::Refuted);
    // (b) Nothing follows from an unrelated audit.
    let moon = fact("the-moon-is-cheese");
    assert_eq!(evidence_refusal(&w.act(&VENDOR, infer(About::Supports(moon.clone()), audit)).unwrap()), Some(EvidenceError::DoesNotFollow));
    assert_eq!(w.epi().unwrap().assessment(&moon), Assessment::Unsupported);
    // (c) An inference cannot rebut what a trusted probe supports.
    let live = fact("service-is-live");
    let _ = w.act(&MONITOR, attest(About::Supports(live.clone()), Method::Probe, "200 OK")).unwrap();
    assert_eq!(evidence_refusal(&w.act(&VENDOR, infer(About::Rebuts(live.clone()), audit)).unwrap()), Some(EvidenceError::InferenceMustSupport));
    assert_eq!(w.epi().unwrap().assessment(&live), Assessment::Accepted);
    // (d) A claimant cannot certify its own claim by "inferring" it.
    let mine = fact("vendor-did-the-work");
    let _ = w.act(&VENDOR, Act::Claim { prop: mine.clone() }).unwrap();
    assert_eq!(evidence_refusal(&w.act(&VENDOR, infer(About::Supports(mine.clone()), audit)).unwrap()), Some(EvidenceError::DoesNotFollow));
    assert_eq!(w.epi().unwrap().assessment(&mine), Assessment::Unsupported);
    assert!(!w.gov().unwrap().rules().unwrap().evidence.self_support);
    // Not vacuous: an inference along a fixed line (a whole task from a step of it) is recorded
    // and counts, at the strength of what it cites.
    let other = Task::whole(TaskName::lit("task2"));
    let step = recorded(&w.act(&MONITOR, attest(About::Supports(Prop::Done(other.step(DEPLOY))), Method::Probe, "seen")).unwrap());
    assert_eq!(evidence_refusal(&w.act(&VENDOR, infer(About::Supports(Prop::Done(other.clone())), step)).unwrap()), None);
    assert_eq!(w.epi().unwrap().assessment(&Prop::Done(other)), Assessment::Accepted);

    // The demonstrator: the culpable agent cannot undo the refutation with one act (rt04).
    let (mut w, o) = run(Cast::default(), false, |_| {}).unwrap();
    let before = w.adj().unwrap().findings(&o.c1);
    assert_eq!(w.norms().unwrap().status(&o.c1), Some(Status::Violated));
    assert!(before.contains(&(Finding::Culpable, AGENT)));
    let ev: Vec<_> = w.epi().unwrap().evidence().cloned().collect();
    let probe = ev.iter().find(|e| e.about == About::Rebuts(deployed())).unwrap().id;
    let ack = ev.iter().find(|e| e.source == ALICE && e.method == Method::Ack).unwrap().id;
    assert_eq!(evidence_refusal(&w.act(&AGENT, infer(About::Undercuts(probe), ack)).unwrap()), Some(EvidenceError::InferenceMustSupport));
    assert_eq!(evidence_refusal(&w.act(&AGENT, infer(About::Supports(deployed()), ack)).unwrap()), Some(EvidenceError::DoesNotFollow));
    assert_eq!(w.epi().unwrap().assessment(&deployed()), Assessment::Refuted);
    assert_eq!(w.norms().unwrap().status(&o.c1), Some(Status::Violated));
    assert_eq!(w.adj().unwrap().findings(&o.c1), before);

    // A debtor trusted for a method cannot satisfy its own commitment on its own word, even
    // without ever filing a claim (rt23).
    let mut w = plain();
    let x = fact("alice-did-it");
    let c = created(&w.act(&ALICE, Act::Commit(offer(VENDOR, Condition::Holds(x.clone()), 5, &[Tag::Task]))).unwrap());
    assert_eq!(evidence_refusal(&w.act(&ALICE, attest(About::Supports(x.clone()), Method::Ack, "trust me")).unwrap()), None);
    assert_eq!(w.epi().unwrap().assessment(&x), Assessment::Unsupported);
    assert_eq!(w.norms().unwrap().status(&c), Some(Status::Detached));
}

// 2. A unit outside a boundary forges the inner institution's events
#[test]
fn fable_02_only_a_boundary_emits_scoped_events_and_a_forged_one_is_dropped() {
    let steal = Transfer { from: ENG, to: QA, resource: CREDIT, amount: Amount(100), memo: Memo::None };
    let p = fact("inner-work-done");
    let forged_ok = Event::Scoped { entity: VENDOR.0, inner: Box::new(Event::Ok { by: ENG, via: CapId::Own(ENG), act: Act::Transfer(steal) }) };
    let forged_assessment = Event::Scoped {
        entity: VENDOR.0,
        inner: Box::new(Event::Assessed {
            prop: p.clone(),
            assessment: Assessment::Accepted,
            since: Some(Time(0)),
            basis: Vec::new(),
            was: Assessment::Unsupported,
            rules: Version(1),
        }),
    };

    // Composition refuses a non-boundary unit that declares it emits scoped events.
    let rogue: Box<dyn Unit<Event>> = Box::new(Cell(Rogue { emits: vec![forged_ok.clone(), forged_assessment.clone()] }));
    assert_eq!(org(vec![vendor_entity(), rogue]).err(), Some(Fault::EmitsInsideBoundary(MALLORY)));

    // And should such an event arise anyway, the boundary drops it: its actor is not inside.
    let mut w = vendor_org();
    w.scope = vec![VENDOR.0];
    let c = created(&w.act(&ENG, Act::Commit(offer(ENVOY, Condition::Holds(p.clone()), 9, &[Tag::Task]))).unwrap());
    assert_eq!(w.norms().unwrap().status(&c), Some(Status::Detached));
    // From outside it is not even a stimulus...
    assert_eq!(w.rt.inject_as(Path::of(&MALLORY), forged_ok.clone()).err(), Some(Fault::NotAStimulus(Kind::Scoped)));
    // ...and delivered past that check (as an outer unit's emission would be), it changes nothing inside.
    for forged in [forged_ok, forged_assessment] {
        let caused = w.rt.inject_unchecked(Path::of(&MALLORY), forged).unwrap();
        assert!(caused.iter().skip(1).all(|e| e.event == Event::Settle), "nothing reacted: {caused:?}");
    }
    assert_eq!(w.ledger().unwrap().balance(&ENG, &CREDIT), Amount(100));
    assert_eq!(w.ledger().unwrap().balance(&QA, &CREDIT), Amount(0));
    assert_eq!(w.epi().unwrap().assessment(&p), Assessment::Unsupported);
    assert_eq!(w.norms().unwrap().status(&c), Some(Status::Detached));
}

// ================================================================ fable: high

// 3. The runtime takes any event kind as an external stimulus, from anyone
#[test]
fn fable_03_only_stimuli_are_taken_from_outside() {
    let mut w = plain();
    // (a) A non-member cannot move Alice's money by submitting the institution's own word.
    let forged = Event::Ok { by: ALICE, via: CapId::Own(ALICE), act: pay(ALICE, VENDOR, 10) };
    assert_eq!(not_a_stimulus(w.inject(&nobody(), forged.clone())), Some(Kind::Ok));
    assert_eq!(w.ledger().unwrap().balance(&VENDOR, &CREDIT), Amount(0));
    // (b) A forged failure does not refund a budget that was really spent.
    let cap = delegated(&w.act_with(&ALICE, &CapId::Own(ALICE), budget(AGENT, ALICE, 50, true)).unwrap()).unwrap();
    assert_eq!(denial(&w.act_with(&AGENT, &cap, pay(ALICE, AGENT, 40)).unwrap()), None);
    assert_eq!(denial(&w.act_with(&AGENT, &cap, pay(ALICE, AGENT, 40)).unwrap()), Some(DenyReason::BudgetExceeded(cap.clone())));
    let spent = Transfer { from: ALICE, to: AGENT, resource: CREDIT, amount: Amount(40), memo: Memo::None };
    let failed = Event::TransferFailed { transfer: spent, via: cap.clone(), reason: LedgerError::InsufficientFunds };
    assert_eq!(not_a_stimulus(w.inject(&nobody(), failed)), Some(Kind::TransferFailed));
    assert_eq!(denial(&w.act_with(&AGENT, &cap, pay(ALICE, AGENT, 40)).unwrap()), Some(DenyReason::BudgetExceeded(cap.clone())));
    assert_eq!(w.ledger().unwrap().balance(&AGENT, &CREDIT), Amount(40), "40 moved under a 50 budget, not 80");
    assert!(w.inst().unwrap().audit().is_empty());
    // (c) A forged assessment does not satisfy a commitment; a forged verdict imposes nothing.
    let p = fact("p");
    let c = created(&w.act(&VENDOR, Act::Commit(offer(ALICE, Condition::Holds(p.clone()), 5, &[Tag::Task]))).unwrap());
    let assessed =
        Event::Assessed { prop: p, assessment: Assessment::Accepted, since: Some(Time(0)), basis: Vec::new(), was: Assessment::Unsupported, rules: Version(1) };
    assert_eq!(not_a_stimulus(w.inject(&nobody(), assessed)), Some(Kind::Assessed));
    assert_eq!(w.norms().unwrap().status(&c), Some(Status::Detached));
    // (e) A stored log in which such an event stands as a stimulus is not a valid history.
    let mut tampered = w.fork();
    let _ = tampered.rt.inject_unchecked(Path::of(&nobody()), forged).unwrap();
    assert_eq!(tampered.ledger().unwrap().balance(&VENDOR, &CREDIT), Amount(10), "the harness door does deliver it");
    let dir = scratch("fable03");
    let path = dir.join("log.jsonl");
    save(&tampered, &path).unwrap();
    assert!(matches!(restore(Box::new(org(Vec::new()).unwrap()), &path), Err(AdapterError::World(Error::Fault(Fault::NotAStimulus(Kind::Ok))))));
    // The honest log restores.
    save(&w, &path).unwrap();
    assert_eq!(restore(Box::new(org(Vec::new()).unwrap()), &path).unwrap().rt.log().head(), w.rt.log().head());
    std::fs::remove_dir_all(dir).unwrap();
}

// 3 (d). "`Tick(u64::MAX)` -> every expiring capability is dead for good (the clock is `max`)"
#[test]
fn fable_03d_a_non_member_cannot_move_the_clock() {
    let mut w = plain();
    let cap = delegated(&w.act_with(&ALICE, &CapId::Own(ALICE), budget(AGENT, ALICE, 50, true)).unwrap()).unwrap();
    let _ = w.inject(&nobody(), Event::Tick(Time(u64::MAX)));
    assert_eq!(denial(&w.act_with(&AGENT, &cap, pay(ALICE, AGENT, 1)).unwrap()), None, "the capability (good until t=20) still works at t=0");
}

// 4. `Signal` is unauthenticated, untimed and last-write-wins; it decides culpability
#[test]
fn fable_04_notice_counts_only_from_the_answerable_party_first_and_before_the_claim() {
    // After the verdict, nobody's word (nor the agent's own, nor a second word from the
    // service) erases "it was notified of the failure" (rt05).
    let (mut w, o) = run(Cast::default(), false, |_| {}).unwrap();
    let before = w.adj().unwrap().findings(&o.c1);
    assert!(before.contains(&(Finding::Culpable, AGENT)) && before.contains(&(Finding::Liable, VENDOR)));
    let duty = w.one(&VENDOR, Tag::Remediation).unwrap().status;
    assert_ne!(duty, Some(Status::Void));
    for sender in [nobody(), AGENT.0, DEPLOYER.0] {
        let _ = w.inject(&sender, Event::Signal { to: AGENT, prop: deployed(), value: true }).unwrap();
        assert_eq!(w.adj().unwrap().findings(&o.c1), before, "after a signal from {sender}");
        assert_eq!(w.one(&VENDOR, Tag::Remediation).unwrap().status, duty);
    }

    // With a lying service nobody is at fault; a notification after the report does not
    // manufacture "it knew" -- not from a stranger, and not from the service itself (rt06).
    let cast = Cast { service_truthful: false, ..Cast::default() };
    let (mut w, o) = run(cast, false, |_| {}).unwrap();
    let before = w.adj().unwrap().findings(&o.c1);
    assert!(!before.iter().any(|(f, _)| matches!(f, Finding::Culpable | Finding::Liable)), "{before:?}");
    for sender in [nobody(), DEPLOYER.0] {
        let _ = w.inject(&sender, Event::Signal { to: AGENT, prop: deployed(), value: false }).unwrap();
        assert_eq!(w.adj().unwrap().findings(&o.c1), before, "after a signal from {sender}");
    }
}

// 5. Liability is escaped, or imposed on others, through fields the debtor writes itself
#[test]
fn fable_05_a_subcontract_is_owed_to_the_parents_debtor_by_another_and_a_refund_is_what_was_paid() {
    // rt07: a debtor cannot hang a commitment of its own under its own and fail it on purpose.
    let mut w = plain();
    let main = created(&w.act(&VENDOR, Act::Commit(offer(ALICE, Condition::Holds(fact("job")), 5, &[Tag::Task]))).unwrap());
    let mut sub = offer(VENDOR, Condition::Holds(fact("whatever")), 5, &[]);
    sub.parent = Some(main);
    assert_eq!(norm_refusal(&w.act(&VENDOR, Act::Commit(sub.clone())).unwrap()), Some(NormError::NotASubcontract));
    // Nor can a third party hang one there that is not owed to the parent's debtor.
    sub.creditor = ALICE;
    assert_eq!(norm_refusal(&w.act(&DEPLOYER, Act::Commit(sub)).unwrap()), Some(NormError::NotASubcontract));
    let _ = w.tick(6).unwrap();
    assert_eq!(w.adj().unwrap().findings(&main), BTreeSet::from([(Finding::Accountable, VENDOR), (Finding::Liable, VENDOR)]));
    assert_eq!(w.one(&VENDOR, Tag::Remediation).unwrap().status, Some(Status::Detached));

    // rt08: terms are the debtor's own statement. Where nothing was paid, nothing is refunded.
    let mut w = plain();
    let mut o = offer(DEPLOYER, Condition::Holds(fact("never")), 1, &[Tag::Task, Tag::Paid]);
    o.terms = Some(Terms { price: Amount(1_000_000), resource: CREDIT, payer: DEPLOYER });
    let c = created(&w.act(&AGENT, Act::Commit(o)).unwrap());
    let mut paid = w.fork();
    let _ = w.tick(2).unwrap();
    assert!(w.adj().unwrap().findings(&c).contains(&(Finding::Liable, VENDOR)));
    assert!(w.tagged(Tag::Refund).unwrap().is_empty(), "nobody owes a refund of money never paid");
    let skipped = w.rt.log().entries().iter().any(|e| matches!(&e.event, Event::NormSkipped { of, reason: SkipReason::NothingWasPaid, .. } if *of == c));
    assert!(skipped);
    // Where the payer did pay part of the stated price, that part is what is owed back.
    assert_eq!(denial(&paid.act_with(&DEPLOYER, &CapId::Own(DEPLOYER), pay(DEPLOYER, AGENT, 25)).unwrap()), None);
    let _ = paid.tick(2).unwrap();
    let refund = paid.one(&VENDOR, Tag::Refund).unwrap();
    assert_eq!(refund.terms().cons, Condition::Paid { from: VENDOR, to: DEPLOYER, resource: CREDIT, at_least: Amount(25) });
    assert_eq!(refund.status, Some(Status::Detached));
}

// 5 (residual). "Any third party can attach the 'sub-commitment' too."
#[test]
fn fable_05b_an_unsolicited_subcontract_from_a_third_party_does_not_excuse_the_debtor() {
    let mut w = plain();
    let main = created(&w.act(&VENDOR, Act::Commit(offer(ALICE, Condition::Holds(fact("job")), 5, &[Tag::Task]))).unwrap());
    // The vendor asked nobody for anything. DEPLOYER volunteers, names the vendor as creditor
    // and the vendor's duty as parent, and walks away at once.
    let mut sub = offer(VENDOR, Condition::Holds(fact("whatever")), 5, &[]);
    sub.parent = Some(main);
    let sub = created(&w.act(&DEPLOYER, Act::Commit(sub)).unwrap());
    let _ = w.act(&DEPLOYER, Act::Cancel { id: sub }).unwrap();
    let _ = w.tick(6).unwrap();
    assert!(w.adj().unwrap().findings(&main).contains(&(Finding::Liable, VENDOR)), "{:?}", w.adj().unwrap().findings(&main));
    assert_eq!(w.one(&VENDOR, Tag::Remediation).unwrap().status, Some(Status::Detached));
}

// 6. A violated maintenance commitment becomes satisfied when *more* refutation arrives late
#[test]
fn fable_06_more_refutation_after_the_deadline_leaves_a_violated_maintenance_commitment_violated() {
    use rigor::gitlog::{found, ADMISSIONS, REGISTRAR};
    const NEWCOMER: ActorId = ActorId::lit("newcomer");
    let project = rigor::gitlog::PROJECT;
    let mut w = found().unwrap();
    let m = Member { id: NEWCOMER, kind: MemberKind::Human, principal: None, role: None };
    let _ = w.act_with(&REGISTRAR, &CapId::Charter(ADMISSIONS), Act::Admit(m)).unwrap();
    let b = Prop::Sound(ChangeId(Name::lit("bbbb")));
    let k = created(&w.act(&NEWCOMER, Act::Commit(offer(project.clone(), Condition::Unrefuted(b.clone()), 10, &[Tag::Change]))).unwrap());
    let _ = w.tick(5).unwrap();
    let _ = w.act(&project, attest(About::Rebuts(b.clone()), Method::Reversal, "undo b")).unwrap();
    let _ = w.tick(11).unwrap();
    assert_eq!(w.norms().unwrap().status(&k), Some(Status::Violated));
    assert!(w.adj().unwrap().findings(&k).contains(&(Finding::Accountable, NEWCOMER)));
    let _ = w.act(&project, attest(About::Rebuts(b.clone()), Method::Reversal, "and it is still broken")).unwrap();
    assert_eq!(w.epi().unwrap().standing(&b).basis.len(), 2, "refuted twice over");
    assert_eq!(w.epi().unwrap().assessment(&b), Assessment::Refuted);
    assert_eq!(w.norms().unwrap().status(&k), Some(Status::Violated));
    assert!(w.adj().unwrap().findings(&k).contains(&(Finding::Accountable, NEWCOMER)));
}

// ---------------------------------------------------------------- 7. the steward

fn cargo(root: &std::path::Path, args: &[&str]) -> bool {
    Command::new("cargo").args(args).current_dir(root).output().unwrap().status.success()
}

/// A real, tiny cargo project laid out as the steward expects: code under `app/`, with a
/// fixture the test reads at compile time.
fn tiny_repo(name: &str, lib: &str) -> PathBuf {
    let root = scratch(name);
    std::fs::create_dir_all(root.join("app")).unwrap();
    std::fs::create_dir_all(root.join("proofs")).unwrap();
    let manifest = "[package]\nname = \"tiny\"\nversion = \"0.1.0\"\nedition = \"2021\"\n[lib]\npath = \"app/lib.rs\"\n[workspace]\n";
    std::fs::write(root.join("Cargo.toml"), manifest).unwrap();
    std::fs::write(root.join("rustfmt.toml"), "").unwrap();
    std::fs::write(root.join("proofs/t.lean"), "").unwrap();
    std::fs::write(root.join("app/lib.rs"), lib).unwrap();
    std::fs::write(root.join("app/expected.txt"), "1\n").unwrap();
    assert!(cargo(&root, &["generate-lockfile"]));
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
/// The undertaking that the tests held on that day, and how it stands.
fn tests_held_on(s: &Steward, day: u64) -> (CommitmentId, Status) {
    let cons = Condition::Holds(Prop::HeldOn { requirement: Check::Tests.requirement(), day });
    let found: Vec<_> = s.commitments(Tag::Upkeep).unwrap().into_iter().filter(|(c, _)| c.terms.cons == cons).collect();
    assert_eq!(found.len(), 1, "one undertaking for day {day}");
    (found[0].0.id, found[0].1)
}
/// Whether a duty to put right that undertaking exists.
fn duty_to_put_right(s: &Steward, lapsed: CommitmentId) -> bool {
    s.commitments(Tag::Remediation).unwrap().iter().any(|(c, _)| c.terms.basis.as_ref().is_some_and(|b| b.of == lapsed))
}

// 7a. The steward records falsehoods and exits 0 on failing tests: `steward accept tests-pass`
#[test]
fn fable_07a_the_steward_takes_nobodys_word_that_the_tests_pass() {
    let root = tiny_repo("fable07a", LIB_BROKEN);
    assert!(!cargo(&root, &["test", "--quiet"]), "the real tests fail");
    let log = root.join(".rigor/steward.jsonl");
    let mut s = Steward::open(&root, &log, 100).unwrap();
    // `accept` only accepts roadmap items: a self-claim is not one, before or after a run.
    assert!(!s.accept(Check::Tests.requirement(), 100).unwrap());
    // Even the project's own acknowledgement, given directly, does not stand in for a run.
    let _ = s.world.act(&PROJECT, attest(About::Supports(Check::Tests.prop()), Method::Ack, "accepted by the project")).unwrap();
    assert!(!s.evidenced(Check::Tests).unwrap());
    assert_eq!(tests_outcome(&s.run(100, false, &only_tests()).unwrap()), Outcome::RanAndFailed, "the check was really run");
    assert!(!s.accept(Check::Tests.requirement(), 100).unwrap());
    assert!(!s.conformant(&only_tests()).unwrap(), "exit code is not 0");
    assert_ne!(s.world.epi().unwrap().assessment(&Check::Tests.prop()), Assessment::Accepted);
    // And it stays that way on later days.
    let mut s = Steward::open(&root, &log, 101).unwrap();
    assert_eq!(tests_outcome(&s.run(101, false, &only_tests()).unwrap()), Outcome::RanAndFailed);
    assert!(!s.conformant(&only_tests()).unwrap());
    std::fs::remove_dir_all(root).unwrap();
}

// 7b. The steward records falsehoods and exits 0 on failing tests: fixtures are not watched
#[test]
fn fable_07b_the_steward_notices_a_change_to_a_test_fixture() {
    let root = tiny_repo("fable07b", LIB_OK);
    let log = root.join(".rigor/steward.jsonl");
    let mut s = Steward::open(&root, &log, 100).unwrap();
    assert_eq!(tests_outcome(&s.run(100, false, &only_tests()).unwrap()), Outcome::RanAndPassed);
    assert!(s.conformant(&only_tests()).unwrap());
    std::fs::write(root.join("app/expected.txt"), "2\n").unwrap();
    assert!(!cargo(&root, &["test", "--quiet"]), "the real tests now fail");
    let mut s = Steward::open(&root, &log, 101).unwrap();
    assert_eq!(tests_outcome(&s.run(101, false, &only_tests()).unwrap()), Outcome::RanAndFailed);
    assert!(!s.conformant(&only_tests()).unwrap());
    std::fs::remove_dir_all(root).unwrap();
}

// 7c. The steward records falsehoods: any edit turns every earlier good day into a violation
#[test]
fn fable_07c_an_ordinary_edit_leaves_past_good_days_good() {
    let root = tiny_repo("fable07c", LIB_OK);
    let log = root.join(".rigor/steward.jsonl");
    let mut s = Steward::open(&root, &log, 100).unwrap();
    assert_eq!(tests_outcome(&s.run(100, false, &only_tests()).unwrap()), Outcome::RanAndPassed);
    assert_eq!(tests_held_on(&s, 100).1, Status::Satisfied);
    // Day 101: a harmless edit. The tests are re-run and pass.
    std::fs::write(root.join("app/lib.rs"), format!("{LIB_OK}// a comment\n")).unwrap();
    let mut s = Steward::open(&root, &log, 101).unwrap();
    assert_eq!(tests_outcome(&s.run(101, false, &only_tests()).unwrap()), Outcome::RanAndPassed);
    assert!(s.conformant(&only_tests()).unwrap());
    let (day100, status) = tests_held_on(&s, 100);
    assert_eq!(status, Status::Satisfied, "day 100, on which the tests passed, still reads as satisfied");
    assert!(!s.world.adj().unwrap().findings(&day100).contains(&(Finding::Accountable, MAINTAINER)));
    assert!(!duty_to_put_right(&s, day100));
    assert_eq!(tests_held_on(&s, 101).1, Status::Satisfied);
    std::fs::remove_dir_all(root).unwrap();
}

// 7d. The steward records falsehoods: a bad day disappears
#[test]
fn fable_07d_a_failed_day_stays_on_the_record_and_a_cut_record_is_noticed() {
    let root = tiny_repo("fable07d", LIB_OK);
    let log = root.join(".rigor/steward.jsonl");
    let mut s = Steward::open(&root, &log, 100).unwrap();
    assert_eq!(tests_outcome(&s.run(100, false, &only_tests()).unwrap()), Outcome::RanAndPassed);
    let day100_lines = std::fs::read_to_string(&log).unwrap().lines().count();
    std::fs::write(root.join("app/lib.rs"), LIB_BROKEN).unwrap();
    let mut s = Steward::open(&root, &log, 101).unwrap();
    assert_eq!(tests_outcome(&s.run(101, false, &only_tests()).unwrap()), Outcome::RanAndFailed);
    let with_bad_day = std::fs::read_to_string(&log).unwrap();
    // Day 102, still broken: day 101 is a violation on the record; day 100 is not.
    let mut s = Steward::open(&root, &log, 102).unwrap();
    assert_eq!(tests_outcome(&s.run(102, false, &only_tests()).unwrap()), Outcome::RanAndFailed);
    assert_eq!((tests_held_on(&s, 100).1, tests_held_on(&s, 101).1), (Status::Satisfied, Status::Violated));
    // (a) Revert the file: today is in order again, and day 101 is still the day the tests failed.
    std::fs::write(root.join("app/lib.rs"), LIB_OK).unwrap();
    let mut s = Steward::open(&root, &log, 102).unwrap();
    assert_eq!(tests_outcome(&s.run(102, false, &only_tests()).unwrap()), Outcome::StillEvidenced);
    assert!(s.conformant(&only_tests()).unwrap());
    let (day101, status) = tests_held_on(&s, 101);
    assert_eq!(status, Status::Violated);
    assert!(s.world.adj().unwrap().findings(&day101).contains(&(Finding::Accountable, MAINTAINER)));
    assert!(duty_to_put_right(&s, day101));
    assert_eq!(tests_held_on(&s, 100).1, Status::Satisfied);
    // (b) Cut the log back to the end of day 100: the record no longer reaches its anchor.
    let kept: String = with_bad_day.lines().take(day100_lines).map(|l| format!("{l}\n")).collect();
    std::fs::write(&log, kept).unwrap();
    let mut s = Steward::open(&root, &log, 102).unwrap();
    assert!(matches!(s.anchor_problem, Some(AnchorProblem::Shorter { anchored, found }) if found < anchored), "{:?}", s.anchor_problem);
    // Running again does not paper over it.
    assert_eq!(tests_outcome(&s.run(102, false, &only_tests()).unwrap()), Outcome::StillEvidenced);
    assert!(!s.conformant(&only_tests()).unwrap());
    assert!(Steward::open(&root, &log, 102).unwrap().anchor_problem.is_some());
    std::fs::remove_dir_all(root).unwrap();
}

// ================================================================ fable: medium

// 8. One transfer discharges every matching debt; the memo is never read
#[test]
fn fable_08_one_transfer_settles_one_debt_and_a_refund_settles_the_one_it_names() {
    let mut w = plain();
    let owe = |d| offer(DEPLOYER, Condition::Paid { from: ALICE, to: DEPLOYER, resource: CREDIT, at_least: Amount(30) }, d, &[Tag::Payment]);
    let ids: Vec<_> = (5..8).map(|d| created(&w.act(&ALICE, Act::Commit(owe(d))).unwrap())).collect();
    let send = |w: &mut World, memo| {
        let t = Transfer { from: ALICE, to: DEPLOYER, resource: CREDIT, amount: Amount(30), memo };
        assert_eq!(denial(&w.act_with(&ALICE, &CapId::Own(ALICE), Act::Transfer(t)).unwrap()), None);
    };
    let open = |w: &World| -> Vec<CommitmentId> { ids.iter().copied().filter(|c| w.norms().unwrap().status(c) == Some(Status::Detached)).collect() };
    let settled = |w: &World| ids.iter().filter(|c| w.norms().unwrap().status(c) == Some(Status::Satisfied)).count();
    send(&mut w, Memo::Payment(Task::whole(TASK1)));
    assert_eq!((settled(&w), open(&w).len()), (1, 2), "three debts of 30, one payment of 30");
    // A payment that names the debt it settles settles that one, not the oldest.
    let named = *open(&w).last().unwrap();
    send(&mut w, Memo::Refund(named));
    assert_eq!(w.norms().unwrap().status(&named), Some(Status::Satisfied));
    assert_eq!((settled(&w), open(&w).len()), (2, 1));
    // A payment naming a debt already settled settles no other.
    send(&mut w, Memo::Refund(named));
    assert_eq!((settled(&w), open(&w).len()), (2, 1));
    assert_eq!(w.ledger().unwrap().balance(&ALICE, &CREDIT), Amount(10));
}

// 9. Evidence that becomes admissible late is stamped early: a missed deadline is cured
#[test]
fn fable_09_evidence_counts_from_when_it_became_admissible() {
    let mut w = plain();
    let art = ArtifactId::lit("src");
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
    assert_eq!(w.epi().unwrap().assessment(&p), Assessment::Accepted, "it holds again");
    assert_eq!(w.norms().unwrap().status(&day2), Some(Status::Violated), "day 2 did not hold on day 2");
    assert_eq!(w.epi().unwrap().standing(&p).since, Some(Time(3)), "it counts from the day it began to count again");
    assert_eq!(w.one(&VENDOR, Tag::Remediation).unwrap().status, Some(Status::Detached));
    assert!(w.adj().unwrap().findings(&day2).contains(&(Finding::Accountable, VENDOR)));
}

// 10. Quorum counts approvals from actors without, or no longer with, the approve power
#[test]
fn fable_10_a_quorum_is_counted_in_approvals_by_current_holders_of_the_power() {
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
        plain_with(c)
    };
    let proposal = |entries: &[Entry<Event>]| {
        entries.iter().find_map(|e| match &e.event {
            Event::Proposed { id, .. } => Some(*id),
            _ => None,
        })
    };
    let approvals = |entries: &[Entry<Event>]| {
        entries.iter().find_map(|e| match &e.event {
            Event::Approved { approvals, .. } => Some(approvals.clone()),
            _ => None,
        })
    };
    let change = || Act::Propose { amendments: vec![Amendment::SetRank(Method::Probe, None)] };
    let (reflect, alice, auditor) = (CapId::Charter(REFLECT), CapId::Charter(GOV_ALICE), CapId::Charter(GOV_AUDITOR));

    // Quorum 2: the proposer's proposing is not an approval; two holders of the power must approve.
    let mut w = founded(2);
    let id = proposal(&w.act_with(&REFLECTOR, &reflect, change()).unwrap()).unwrap();
    assert_eq!(denial(&w.act_with(&REFLECTOR, &reflect, Act::Approve { id }).unwrap()), Some(DenyReason::ActionNotGranted));
    assert_eq!(approvals(&w.act_with(&ALICE, &alice, Act::Approve { id }).unwrap()), Some(BTreeSet::from([ALICE])));
    assert_eq!(w.gov().unwrap().version(), Version(1), "one approval is not two");
    let _ = w.act_with(&AUDITOR, &auditor, Act::Approve { id }).unwrap();
    assert_eq!(w.gov().unwrap().version(), Version(2));
    assert_eq!(w.gov().unwrap().history()[1].by, BTreeSet::from([ALICE, AUDITOR]));

    // Quorum 1: a charter to propose and nothing else enacts nothing by itself.
    let mut w = founded(1);
    let id = proposal(&w.act_with(&REFLECTOR, &reflect, change()).unwrap()).unwrap();
    assert_eq!(w.gov().unwrap().version(), Version(1));
    let _ = w.act_with(&ALICE, &alice, Act::Approve { id }).unwrap();
    assert_eq!(w.gov().unwrap().history()[1].by, BTreeSet::from([ALICE]));

    // Quorum 2: an expelled member's approval is removed, so the remaining one is not a quorum.
    let mut w = founded(2);
    let id = proposal(&w.act_with(&ALICE, &alice, change()).unwrap()).unwrap();
    let _ = w.act_with(&ALICE, &alice, Act::Approve { id }).unwrap();
    let _ = w.act_with(&AUDITOR, &CapId::Charter(MEMBERSHIP), Act::Expel { member: ALICE }).unwrap();
    assert_eq!(approvals(&w.act_with(&AUDITOR, &auditor, Act::Approve { id }).unwrap()), Some(BTreeSet::from([AUDITOR])));
    assert_eq!(w.gov().unwrap().version(), Version(1));
}

// 11. An attested snapshot can be re-pointed; anyone can attest a forged one
#[test]
fn fable_11_attestation_names_the_whole_snapshot_and_nobody_infers_one() {
    let dir = scratch("fable11");
    let (mut w, _) = run(Cast::default(), true, |_| {}).unwrap();
    let fresh = || root(Cast::default()).unwrap();
    let good = dir.join("good.snapshot");
    let snap = save_snapshot(&w, &good).unwrap();
    assert!(audit_snapshot(&mut w, fresh(), &snap, &AUDITOR).unwrap());
    let resumed = resume_attested(fresh(), &good, &w).unwrap();
    assert_eq!((resumed.rt.log().next_seq(), resumed.rt.log().head()), (snap.seq, snap.head));
    // (a) The same state pointed at another history is a different snapshot: nobody attested it.
    let mut moved = snap.clone();
    moved.seq = 3;
    moved.head = Digest(0xDEAD_BEEF);
    assert_ne!(faithful(&moved).unwrap(), faithful(&snap).unwrap());
    let path = dir.join("moved.snapshot");
    std::fs::write(&path, bincode::serialize(&moved).unwrap()).unwrap();
    assert!(matches!(resume_attested(fresh(), &path, &w), Err(AdapterError::SnapshotNotAttested)));
    // (b) A forged state that nobody audited cannot be vouched for by "inference" from an audit.
    let mut forged = snap.clone();
    let end = forged.state.len() - 1;
    forged.state[end] ^= 1;
    forged.digest = rigor::waist::digest_bytes(&forged.state);
    let path = dir.join("forged.snapshot");
    std::fs::write(&path, bincode::serialize(&forged).unwrap()).unwrap();
    let audit = w.epi().unwrap().evidence().find(|e| e.about == About::Supports(faithful(&snap).unwrap())).unwrap().id;
    let vouch = infer(About::Supports(faithful(&forged).unwrap()), audit);
    assert_eq!(evidence_refusal(&w.act(&VENDOR, vouch).unwrap()), Some(EvidenceError::DoesNotFollow));
    assert!(matches!(resume_attested(fresh(), &path, &w), Err(AdapterError::SnapshotNotAttested)));
    std::fs::remove_dir_all(dir).unwrap();
}

// 12. A truncated log is a valid log
#[test]
fn fable_12_a_record_cut_short_or_removed_does_not_reach_its_anchor() {
    let root = scratch("fable12");
    std::fs::create_dir_all(root.join("waist")).unwrap();
    std::fs::create_dir_all(root.join("proofs")).unwrap();
    for f in ["waist/lib.rs", "proofs/t.lean", "Cargo.toml", "Cargo.lock", "rustfmt.toml"] {
        std::fs::write(root.join(f), f).unwrap();
    }
    let log = root.join(".rigor/steward.jsonl");
    let all: BTreeSet<Check> = Check::ALL.into_iter().collect();
    let mut s = Steward::open(&root, &log, 100).unwrap();
    let _ = s.run(100, false, &all).unwrap();
    let (events_day_100, lines_day_100) = (s.world.rt.log().next_seq(), std::fs::read_to_string(&log).unwrap().lines().count());
    let mut s = Steward::open(&root, &log, 101).unwrap();
    assert_eq!(s.anchor_problem, None);
    let _ = s.run(101, false, &all).unwrap();
    let anchored = s.world.rt.log().next_seq();
    assert!(anchored > events_day_100);
    // Cut the tail off. The log is still a valid chain (it restores), and the steward says what is missing.
    let text = std::fs::read_to_string(&log).unwrap();
    let kept: String = text.lines().take(lines_day_100).map(|l| format!("{l}\n")).collect();
    std::fs::write(&log, kept).unwrap();
    let mut s = Steward::open(&root, &log, 102).unwrap();
    assert!(s.previous_log_rejected.is_none() && s.migrated_from.is_none());
    assert_eq!(s.anchor_problem, Some(AnchorProblem::Shorter { anchored, found: events_day_100 }));
    // It is not conformant whatever is skipped, and carrying on does not move the anchor.
    let _ = s.run(102, false, &all).unwrap();
    assert!(!s.conformant(&all).unwrap());
    assert!(Steward::open(&root, &log, 102).unwrap().anchor_problem.is_some());
    // Removing the record altogether does not quietly found a new one either.
    std::fs::remove_file(&log).unwrap();
    let s = Steward::open(&root, &log, 102).unwrap();
    assert!(matches!(s.anchor_problem, Some(AnchorProblem::Shorter { anchored: a, found }) if a == anchored && found < events_day_100));
    assert!(!s.conformant(&all).unwrap());
    std::fs::remove_dir_all(root).unwrap();
}

// 12 (residual). The original reproduction: `restore` by itself, with no anchor to measure against.
#[test]
fn fable_12b_restore_by_itself_refuses_a_log_cut_short() {
    let dir = scratch("fable12b");
    let path = dir.join("log.jsonl");
    let (w, o) = run(Cast::default(), false, |_| {}).unwrap();
    assert_eq!(w.norms().unwrap().status(&o.c1), Some(Status::Violated));
    save(&w, &path).unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    // Cut where a stimulus begins (the tick at which the refutation arrives), so that what is
    // left is a complete history of an earlier time: one line per entry after the header.
    let entries = w.rt.log().entries();
    let header = text.lines().count() - entries.len();
    let cut = entries.iter().position(|e| e.cause.is_none() && e.event == Event::Tick(Time(6))).unwrap();
    let kept: String = text.lines().take(header + cut).map(|l| format!("{l}\n")).collect();
    std::fs::write(&path, kept).unwrap();
    let back = restore(root(Cast::default()).unwrap(), &path);
    std::fs::remove_dir_all(dir).unwrap();
    assert!(back.is_err(), "a history missing its end was taken for the whole");
}

// 13. A budget limits only the resources it names
#[test]
fn fable_13_a_budget_covers_only_the_resources_it_names() {
    let gold = Resource::lit("gold");
    let mut c = constitution(rules());
    c.balances.push((ALICE, gold.clone(), Amount(1_000_000)));
    let mut w = plain_with(c);
    let cap = delegated(&w.act_with(&ALICE, &CapId::Own(ALICE), budget(AGENT, ALICE, 50, true)).unwrap()).unwrap();
    for amount in [1_000_000, 1] {
        let t = Transfer { from: ALICE, to: AGENT, resource: gold.clone(), amount: Amount(amount), memo: Memo::None };
        assert_eq!(denial(&w.act_with(&AGENT, &cap, Act::Transfer(t)).unwrap()), Some(DenyReason::BudgetExceeded(cap.clone())));
    }
    assert_eq!(w.ledger().unwrap().balance(&AGENT, &gold), Amount(0));
    assert_eq!(w.ledger().unwrap().balance(&ALICE, &gold), Amount(1_000_000));
    // The resource it does name is still good, up to its limit.
    assert_eq!(denial(&w.act_with(&AGENT, &cap, pay(ALICE, AGENT, 50)).unwrap()), None);
    assert_eq!(w.ledger().unwrap().balance(&AGENT, &CREDIT), Amount(50));
    assert!(w.inst().unwrap().audit().is_empty());
}

// ================================================================ fable: low

// 14. A repudiated secondary duty is not voided with its basis
#[test]
fn fable_14_a_repudiated_secondary_duty_is_voided_with_its_basis() {
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
    assert_eq!(w.norms().unwrap().status(&rem), Some(Status::Violated), "repudiated");
    let _ = w.act(&AUDITOR, Act::Retract { evidence: no }).unwrap();
    assert_eq!(w.norms().unwrap().status(&main), Some(Status::Satisfied), "the violation never was");
    assert!(w.adj().unwrap().findings(&main).is_empty());
    assert_eq!(w.norms().unwrap().status(&rem), Some(Status::Void));
    assert!(w.adj().unwrap().findings(&rem).is_empty());
}

// 15. Any inner member silences the envoy
#[test]
fn fable_15_the_envoy_tracks_only_the_designated_workers_commitment() {
    let mut w = vendor_org();
    let wanted = fact("vendor-delivers");
    let duty = created(&w.act(&VENDOR, Act::Commit(offer(ALICE, Condition::Holds(wanted.clone()), 20, &[Tag::Task]))).unwrap());
    let task = Envoy::task_for(duty).unwrap();
    w.scope = vec![VENDOR.0];
    let done = Condition::Holds(Prop::Done(task.clone()));
    // INTERN is not the worker. It "undertakes" the job and repudiates it.
    let spoiler = created(&w.act(&INTERN, Act::Commit(offer(ENVOY, done.clone(), 3, &[]))).unwrap());
    let _ = w.act(&INTERN, Act::Cancel { id: spoiler }).unwrap();
    assert_eq!(w.norms().unwrap().status(&spoiler), Some(Status::Violated));
    // The real worker then does the job, to the inner institution's satisfaction.
    let real = created(&w.act(&ENG, Act::Commit(offer(ENVOY, done, 9, &[Tag::Task]))).unwrap());
    let _ = w.act(&QA, attest(About::Supports(Prop::Done(task)), Method::Test, "inner checks pass")).unwrap();
    assert_eq!(w.norms().unwrap().status(&real), Some(Status::Satisfied));
    w.scope = Vec::new();
    let outward: Vec<_> = w.rt.log().entries().iter().filter(|e| e.actor.only() == Some(&VENDOR.0)).map(|e| e.event.clone()).collect();
    assert!(!outward.iter().any(|e| matches!(e, Event::Signal { value: false, .. })), "no failure was reported outward");
    assert!(outward.iter().any(|e| matches!(e, Event::Do { act: Act::Claim { prop }, .. } if *prop == wanted)), "the worker's success was claimed");
    assert!(outward.iter().any(|e| matches!(e, Event::Signal { to, prop, value: true } if *to == ALICE && *prop == wanted)));
}

// 15 (residual). The original reproduction, in which the spoiler is the inner institution's only trusted tester.
#[test]
fn fable_15b_a_spoiler_who_is_the_trusted_tester_does_not_silence_the_workers_success() {
    let mut w = vendor_org();
    let wanted = fact("vendor-delivers");
    let duty = created(&w.act(&VENDOR, Act::Commit(offer(ALICE, Condition::Holds(wanted.clone()), 20, &[Tag::Task]))).unwrap());
    let task = Envoy::task_for(duty).unwrap();
    w.scope = vec![VENDOR.0];
    let done = Condition::Holds(Prop::Done(task.clone()));
    let spoiler = created(&w.act(&QA, Act::Commit(offer(ENVOY, done.clone(), 3, &[]))).unwrap());
    let _ = w.act(&QA, Act::Cancel { id: spoiler }).unwrap();
    let real = created(&w.act(&ENG, Act::Commit(offer(ENVOY, done, 9, &[Tag::Task]))).unwrap());
    let _ = w.act(&QA, attest(About::Supports(Prop::Done(task)), Method::Test, "inner checks pass")).unwrap();
    assert_eq!(w.norms().unwrap().status(&real), Some(Status::Satisfied));
    w.scope = Vec::new();
    let claimed =
        w.rt.log()
            .entries()
            .iter()
            .any(|e| e.actor.only() == Some(&VENDOR.0) && matches!(&e.event, Event::Do { act: Act::Claim { prop }, .. } if *prop == wanted));
    assert!(claimed);
}

// 16. The ledger destroys value and reports conservation
#[test]
fn fable_16_a_credit_that_does_not_fit_is_refused_and_nothing_moves() {
    let mut c = constitution(rules());
    c.balances = vec![(ALICE, CREDIT, Amount(u64::MAX)), (DEPLOYER, CREDIT, Amount(40))];
    let mut w = plain_with(c);
    let entries = w.act_with(&DEPLOYER, &CapId::Own(DEPLOYER), pay(DEPLOYER, ALICE, 40)).unwrap();
    let refused = entries.iter().find_map(|e| match &e.event {
        Event::TransferFailed { reason, .. } => Some(*reason),
        _ => None,
    });
    assert_eq!(refused, Some(LedgerError::Overflow));
    assert!(!entries.iter().any(|e| matches!(e.event, Event::Transferred { .. })));
    let l = w.ledger().unwrap();
    assert_eq!((l.balance(&ALICE, &CREDIT), l.balance(&DEPLOYER, &CREDIT)), (Amount(u64::MAX), Amount(40)), "nothing vanished");
    assert!(l.conserved());
}

// ================================================================ fable: unconfirmed

// Unconfirmed: "`Envoy::task_for` ... prints 40 of 64 bits; two outer duties could share a task."
#[test]
fn fable_u2_two_outer_duties_never_share_an_inner_task() {
    let base = 0x0123_4567_89AB_CDEF_u64;
    let task = |d: u64| Envoy::task_for(CommitmentId(Digest(d))).unwrap();
    for bit in 0..64 {
        assert_ne!(task(base), task(base ^ (1 << bit)), "digests differing in bit {bit}");
    }
}

// Unconfirmed: "`upcast` patches only a top-level `Genesis`; a schema-2 log of a nested world ... `Unmigratable`."
#[test]
fn fable_u3_an_old_founding_of_a_nested_organisation_is_brought_up_to_date() {
    let current = Event::Scoped { entity: VENDOR.0, inner: Box::new(Event::Genesis(Box::new(constitution(rules())))) };
    // The same event as schema 2 wrote it: charters without `delegable`, rules without a period.
    let mut old = serde_json::to_value(&current).unwrap();
    let founding = old.pointer_mut("/Scoped/inner/Genesis").unwrap();
    for charter in founding.get_mut("charters").unwrap().as_array_mut().unwrap() {
        assert!(charter.as_object_mut().unwrap().remove("delegable").is_some());
    }
    assert!(founding.pointer_mut("/rules/norms").unwrap().as_object_mut().unwrap().remove("maintenance_period").is_some());
    assert!(serde_json::from_value::<Event>(old.clone()).is_err(), "as written, it has no current form");
    assert_eq!(serde_json::from_value::<Event>(upcast(2, old)).unwrap(), current);
}

// ================================================================ fable: tried and held up

fn task_prop(i: usize) -> Prop {
    let whole = Task::whole(if i.is_multiple_of(2) { TaskName::lit("t0") } else { TaskName::lit("t1") });
    Prop::Done(match i / 2 {
        0 => whole,
        1 => whole.step(TaskName::lit("s0")),
        _ => whole.step(TaskName::lit("s1")),
    })
}

// Tried and held up: "Incremental `revise` versus `full()` ... found no disagreement" (redteam_fuzz.rs)
#[test]
fn fable_held_incremental_assessment_equals_from_scratch_under_a_wider_mix() {
    let sources = [DEPLOYER, MONITOR, ALICE, AUDITOR, VENDOR];
    let methods = [Method::Receipt, Method::Probe, Method::Ack, Method::Audit, Method::Test, Method::Inference, Method::Hearsay, Method::Reversal];
    let arts = [ArtifactId::lit("f"), ArtifactId::lit("g")];
    let env = Path::of(&Name::lit("env"));
    let mut inferences = 0u64;
    for seed in 0..120u64 {
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
        let alone: Box<dyn Unit<Event>> = Box::new(Par::new(vec![Box::new(Cell(Epistemic::default()))], &open_kinds()).unwrap());
        let mut rt: Runtime<Event> = Runtime::new(alone);
        // A unit exercised alone is fed what its neighbours would have produced.
        let _ = rt.inject_unchecked(env.clone(), rc(&r, version)).unwrap();
        let mut now = 0u64;
        for step in 0..80 {
            let epi = rt.leaf::<Epistemic>(&[EPI]).unwrap();
            let ids: Vec<EvidenceId> = epi.evidence().map(|e| e.id).collect();
            let by = rng.pick(&sources).clone();
            let ok = |act: Act| Event::Ok { by: by.clone(), via: CapId::Base(by.clone()), act };
            let roll = rng.below(100);
            let event = if roll < 50 || ids.is_empty() {
                let n = (*rng.pick(&[0usize, 0, 1, 1, 2, 3])).min(ids.len());
                let mut derived_from: BTreeSet<EvidenceId> = (0..n).map(|_| *rng.pick(&ids)).collect();
                let mut method = *rng.pick(&methods);
                let mut about = if !ids.is_empty() && rng.percent(35) {
                    About::Undercuts(*rng.pick(&ids))
                } else if rng.percent(50) {
                    About::Supports(task_prop(rng.below(6)))
                } else {
                    About::Rebuts(task_prop(rng.below(6)))
                };
                if method == Method::Inference {
                    // Mostly a well-formed inference (a whole task from support for a step of
                    // it); sometimes left ill-formed, which must be refused without effect.
                    let steps: Vec<(EvidenceId, Prop)> = epi
                        .evidence()
                        .filter_map(|e| match &e.about {
                            About::Supports(Prop::Done(t)) => t.parent().map(|whole| (e.id, Prop::Done(whole))),
                            _ => None,
                        })
                        .collect();
                    if !steps.is_empty() && rng.percent(80) {
                        let (premise, conclusion) = rng.pick(&steps).clone();
                        derived_from = BTreeSet::from([premise]);
                        about = About::Supports(conclusion);
                        inferences += 1;
                    } else if derived_from.is_empty() {
                        method = Method::Ack;
                    }
                }
                let artifacts: BTreeMap<ArtifactId, ContentDigest> = if rng.percent(25) {
                    std::iter::once((rng.pick(&arts).clone(), ContentDigest(Digest(rng.below_u64(2))))).collect()
                } else {
                    BTreeMap::new()
                };
                ok(Act::Attest(Attestation { about, method, derived_from, artifacts, content: Content::Note(format!("{step}")) }))
            } else if roll < 62 {
                ok(Act::Claim { prop: task_prop(rng.below(6)) })
            } else if roll < 74 {
                let target = epi.evidence().nth(rng.below(ids.len())).unwrap();
                Event::Ok { by: target.source.clone(), via: CapId::Base(target.source.clone()), act: Act::Retract { evidence: target.id } }
            } else if roll < 84 {
                Event::ArtifactChanged { artifact: rng.pick(&arts).clone(), digest: ContentDigest(Digest(rng.below_u64(2))) }
            } else if roll < 96 {
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
            let _ = rt.inject_unchecked(env.clone(), event.clone()).unwrap();
            let epi = rt.leaf::<Epistemic>(&[EPI]).unwrap();
            for (prop, standing) in epi.full() {
                assert_eq!(epi.standing(&prop), standing, "seed {seed} step {step} {prop} after {event:?}");
            }
        }
    }
    assert!(inferences > 100, "the mix exercised well-formed inferences: {inferences}");
}

// ================================================================ astra

fn maintenance() -> (World, Prop, CommitmentId) {
    let mut w = world(Cast::default(), rules()).unwrap();
    let p = fact("maintenance");
    let id = created(&w.act(&AGENT, Act::Commit(offer(ALICE, Condition::Unrefuted(p.clone()), 10, &[Tag::Task]))).unwrap());
    (w, p, id)
}

// High — Corroborating a refutation discharges a violated maintenance duty
#[test]
fn astra_01_corroborating_a_refutation_leaves_the_maintenance_duty_violated() {
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

// High — The adjudicator drops an auditor's accountability until the auditor becomes the creditor
#[test]
fn astra_02_a_rule_about_a_role_reaches_its_holder_whoever_the_family_mentions() {
    let mut rules = rules();
    let (party, id) = (Term::Var(Var(0)), Term::Var(Var(1)));
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
    let id = created(&w.act(&AGENT, Act::Commit(offer(ALICE, Condition::Holds(fact("deliver")), 1, &[Tag::Payment]))).unwrap());
    let _ = w.tick(2).unwrap();
    let finding = (Finding::Accountable, AUDITOR);
    let before = w.adj().unwrap().findings(&id).contains(&finding);
    let _ = w.act(&ALICE, Act::Assign { id, to: AUDITOR }).unwrap();
    let after = w.adj().unwrap().findings(&id).contains(&finding);
    assert_eq!((before, after), (true, true));
}

// High — A repudiated secondary duty survives withdrawal of its sole basis
#[test]
fn astra_03_a_repudiated_secondary_duty_is_void_once_its_sole_basis_is_withdrawn() {
    let (mut w, p, id) = maintenance();
    let evidence = recorded(&w.act(&MONITOR, attest(About::Rebuts(p), Method::Probe, "first")).unwrap());
    assert_eq!(w.norms().unwrap().status(&id), Some(Status::Violated));
    let child = w.one(&AUDITOR, Tag::Audit).unwrap().commitment.id;
    let _ = w.act(&AUDITOR, Act::Cancel { id: child }).unwrap();
    assert_eq!(w.norms().unwrap().status(&child), Some(Status::Violated));
    let _ = w.act(&MONITOR, Act::Retract { evidence }).unwrap();
    assert_eq!(w.norms().unwrap().status(&id), Some(Status::Detached));
    assert_eq!(w.norms().unwrap().status(&child), Some(Status::Void));
}

/// Astra's own probe: a crate whose one test reads a text fixture; day 100 passes; then either
/// only the fixture changes (so cargo fails) or the source is replaced by another passing test.
fn astra_two_days(name: &str, fixture_changes: bool) -> (Steward, bool, Vec<(Check, Outcome)>) {
    let root = tiny_repo(name, "#[test]\nfn fixture() {\n    assert_eq!(include_str!(\"expected.txt\"), \"1\\n\");\n}\n");
    let log = root.join(".rigor/steward.jsonl");
    let mut s = Steward::open(&root, &log, 100).unwrap();
    assert!(s.run(100, false, &only_tests()).unwrap().contains(&(Check::Tests, Outcome::RanAndPassed)));
    assert_eq!(tests_held_on(&s, 100).1, Status::Satisfied);
    if fixture_changes {
        std::fs::write(root.join("app/expected.txt"), "changed").unwrap();
    } else {
        std::fs::write(root.join("app/lib.rs"), "#[test]\nfn still_passes() {}\n").unwrap();
    }
    let actual = cargo(&root, &["test", "--quiet"]);
    let mut s = Steward::open(&root, &log, 101).unwrap();
    let outcomes = s.run(101, false, &only_tests()).unwrap();
    std::fs::remove_dir_all(root).unwrap();
    (s, actual, outcomes)
}

// High — The steward preserves passing test evidence after a test fixture changes and Cargo fails
#[test]
fn astra_04_the_steward_agrees_with_cargo_after_a_fixture_only_change() {
    let (s, actual, outcomes) = astra_two_days("astra04", true);
    assert!(!actual, "cargo fails");
    assert!(outcomes.contains(&(Check::Tests, Outcome::RanAndFailed)), "{outcomes:?}");
    assert_eq!(s.conformant(&only_tests()).unwrap(), actual);
    assert_eq!(s.world.epi().unwrap().assessment(&Check::Tests.prop()), Assessment::Refuted);
}

// Medium — Two passing steward runs manufacture a historical violation
#[test]
fn astra_05_two_passing_runs_leave_both_days_satisfied() {
    let (s, actual, outcomes) = astra_two_days("astra05", false);
    assert!(actual, "cargo passes");
    assert!(outcomes.contains(&(Check::Tests, Outcome::RanAndPassed)), "{outcomes:?}");
    let (day100, status) = tests_held_on(&s, 100);
    assert_eq!(status, Status::Satisfied);
    assert!(!duty_to_put_right(&s, day100), "no spurious remediation duty");
    assert_eq!(tests_held_on(&s, 101).1, Status::Satisfied);
}

// Medium — A successful transfer destroys one credit while the conservation check passes
#[test]
fn astra_06_the_ledgers_total_is_what_was_minted_summed_in_a_wider_type() {
    let mut c = constitution(rules());
    c.balances = vec![(ALICE, CREDIT, Amount(1)), (AGENT, CREDIT, Amount(u64::MAX))];
    let mut w = plain_with(c);
    let events = w.act_with(&ALICE, &CapId::Own(ALICE), pay(ALICE, AGENT, 1)).unwrap();
    // The credit does not fit, so the transfer is refused outright rather than half carried out.
    assert!(events.iter().any(|e| matches!(e.event, Event::TransferFailed { reason: LedgerError::Overflow, .. })));
    assert!(!events.iter().any(|e| matches!(e.event, Event::Transferred { .. })));
    let l = w.ledger().unwrap();
    assert!(l.conserved());
    let total = u128::from(l.balance(&ALICE, &CREDIT).0) + u128::from(l.balance(&AGENT, &CREDIT).0);
    assert_eq!(total, 1 + u128::from(u64::MAX));
    // And a founding balance sheet that does not fit one account is not a founding at all.
    let mut c = constitution(rules());
    c.balances = vec![(ALICE, CREDIT, Amount(u64::MAX)), (ALICE, CREDIT, Amount(1))];
    let mut w = World::new(Box::new(org(Vec::new()).unwrap()));
    let founding = w.inject(&CONSTITUTION, Event::Genesis(Box::new(c)));
    assert!(matches!(founding, Err(Error::Fault(Fault::Unit { failure: rigor::model::Failure::BalanceSheetOverflows, .. }))));
}

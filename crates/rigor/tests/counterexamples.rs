//! Negative results and adversarial cases, kept as executable facts.
//!
//! Each test pins down something the foundation does NOT give you, or an attack it must (and
//! does) withstand. See docs/03-status-ledger.md.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::indexing_slicing)]

use rigor::impact::{cone, what_if, Change, Judgment};
use rigor::kernel::{Cell, Digest, Emit, Entry, Envelope, Leaf, Name, Par, Path, Runtime, Unit, Who};
use rigor::logic::datalog::{Atom, Literal, Rule, RuleError, Term, Var};
use rigor::model::{
    About, Act, Amendment, Assessment, Attestation, CapId, Condition, ConsExpr, Content, DenyReason, Event, EvidenceError, GovError, Kind, Method, NormError,
    Offer, PartyExpr, Pred, Prop, RTerm, Role, RuleId, RulePath, RulesError, Status, Tag, Template, Time, Version, ViolationTemplate,
};
use rigor::model::{Fault, LeafError};
use rigor::org::open_kinds;
use rigor::scenario::{attest, constitution, root, rules, run, world, Cast, AGENT, ALICE, AUDITOR, DEPLOYER, GOV_ALICE, GOV_AUDITOR, MONITOR, VENDOR};
use rigor::world::{World, CONSTITUTION};
use std::any::Any;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::hash::Hasher;

fn fact(n: &'static str) -> Prop {
    Prop::Fact(Name::new(n).unwrap())
}
fn msg(n: &'static str) -> Event {
    Event::Signal { to: ALICE, prop: fact(n), value: true }
}

/// Forwards: on a signal about `listens` (or on a tick if `listens` is None), emits signals
/// about each of `emits`, and remembers what it saw.
#[derive(Clone, Debug, Hash, serde::Serialize, serde::Deserialize)]
struct Fwd {
    name: Name,
    listens: Option<Prop>,
    emits: Vec<Prop>,
    seen: Vec<Prop>,
}

impl Leaf<Event> for Fwd {
    fn name(&self) -> Name {
        self.name.clone()
    }
    fn inputs(&self) -> BTreeSet<Kind> {
        [Kind::Signal, Kind::Tick].into_iter().collect()
    }
    fn outputs(&self) -> BTreeSet<Kind> {
        std::iter::once(Kind::Signal).collect()
    }
    fn on(&mut self, env: &Envelope<'_, Event>) -> Result<Vec<Event>, LeafError> {
        let hit = match (env.event, &self.listens) {
            (Event::Tick(_), None) => true,
            (Event::Signal { prop, .. }, Some(p)) if prop == p => {
                self.seen.push(prop.clone());
                true
            }
            _ => false,
        };
        Ok(if hit { self.emits.iter().map(|p| Event::Signal { to: ALICE, prop: p.clone(), value: true }).collect() } else { Vec::new() })
    }
}

/// Records every signal it is sent.
#[derive(Clone, Debug, Hash, serde::Serialize, serde::Deserialize)]
struct Rec {
    seen: Vec<Prop>,
}
impl Leaf<Event> for Rec {
    fn name(&self) -> Name {
        Name::lit("s")
    }
    fn inputs(&self) -> BTreeSet<Kind> {
        std::iter::once(Kind::Signal).collect()
    }
    fn outputs(&self) -> BTreeSet<Kind> {
        BTreeSet::new()
    }
    fn on(&mut self, env: &Envelope<'_, Event>) -> Result<Vec<Event>, LeafError> {
        if let Event::Signal { prop, .. } = env.event {
            if *prop != fact("m1") {
                self.seen.push(prop.clone());
            }
        }
        Ok(Vec::new())
    }
}

fn fwd(name: &'static str, listens: Option<&'static str>, emits: &[&'static str]) -> Box<dyn Unit<Event>> {
    Box::new(Cell(Fwd { name: Name::new(name).unwrap(), listens: listens.map(fact), emits: emits.iter().map(|e| fact(e)).collect(), seen: Vec::new() }))
}
/// p: tick -> m1.   q: m1 -> m2.   r: tick -> m3.   s records the order of m2 and m3.
type U = Box<dyn Unit<Event>>;
fn pqr() -> (U, U, U, U) {
    (fwd("p", None, &["m1"]), fwd("q", Some("m1"), &["m2"]), fwd("r", None, &["m3"]), Box::new(Cell(Rec { seen: Vec::new() })))
}

/// The tempting alternative: a composite that runs its internal feedback to completion.
#[derive(Debug)]
struct Rtc(Par<Event>);

impl Unit<Event> for Rtc {
    fn names(&self) -> Vec<Name> {
        self.0.names()
    }
    fn inputs(&self) -> BTreeSet<Kind> {
        self.0.inputs()
    }
    fn outputs(&self) -> BTreeSet<Kind> {
        self.0.outputs()
    }
    fn spies(&self) -> Vec<Name> {
        Vec::new()
    }
    fn step(&mut self, env: &Envelope<'_, Event>) -> Result<Vec<Emit<Event>>, Fault> {
        let mut out = self.0.step(env)?;
        let mut q: VecDeque<Emit<Event>> = out.iter().cloned().collect();
        while let Some(o) = q.pop_front() {
            let entry = Entry { seq: 0, id: Digest(0), actor: o.actor.clone(), cause: None, src: o.src, event: o.event };
            let more = self.0.step(&Envelope { who: Who::Here(o.actor), entry: &entry, event: &entry.event })?;
            q.extend(more.iter().cloned());
            out.extend(more);
        }
        Ok(out)
    }
    fn fork(&self) -> Box<dyn Unit<Event>> {
        self.0.fork()
    }
    fn hash_state(&self, h: &mut dyn Hasher) {
        self.0.hash_state(h);
    }
    fn save(&self) -> Result<Vec<u8>, rigor::kernel::EncodingError> {
        self.0.save()
    }
    fn load(&mut self, bytes: &[u8]) -> Result<(), rigor::kernel::EncodingError> {
        self.0.load(bytes)
    }
    fn find(&self, path: &[Name]) -> Option<&dyn Any> {
        self.0.find(path)
    }
}

fn seen_by_s(rt: &Runtime<Event>) -> Vec<Prop> {
    rt.leaf::<Rec>(&[Name::lit("s")]).unwrap().seen.clone()
}
fn tick(root: Box<dyn Unit<Event>>) -> Runtime<Event> {
    let mut rt = Runtime::new(root);
    let _ = rt.inject(&Name::lit("clock"), Event::Tick(Time(1))).unwrap();
    rt
}

#[test]
fn run_to_completion_composition_is_not_associative() {
    // Why Par delivers one event at a time and leaves all feedback to the single runtime.
    let open = open_kinds();
    let (p, q, r, s) = pqr();
    let nested = tick(Box::new(Par::new(vec![Box::new(Rtc(Par::new(vec![p, q], &open).unwrap())), r, s], &open).unwrap()));
    let (p, q, r, s) = pqr();
    let flat = tick(Box::new(Par::new(vec![p, q, r, s], &open).unwrap()));
    assert_eq!(seen_by_s(&flat), vec![fact("m3"), fact("m2")]);
    // Same units, different bracketing: the inner loop finishes before r is heard...
    assert_eq!(seen_by_s(&nested)[..2], [fact("m2"), fact("m3")]);
    assert_ne!(seen_by_s(&nested), seen_by_s(&flat));
}

#[test]
fn par_is_associative_on_the_same_units() {
    let open = open_kinds();
    let mut heads = Vec::new();
    for shape in 0..3 {
        let (p, q, r, s) = pqr();
        let root: Box<dyn Unit<Event>> = match shape {
            0 => Box::new(Par::new(vec![p, q, r, s], &open).unwrap()),
            1 => Box::new(Par::new(vec![Box::new(Par::new(vec![p, q], &open).unwrap()), r, s], &open).unwrap()),
            _ => Box::new(Par::new(vec![s, Box::new(Par::new(vec![r, Box::new(Par::new(vec![q, p], &open).unwrap())], &open).unwrap())], &open).unwrap()),
        };
        heads.push(tick(root).log().head());
    }
    assert!(heads[0] == heads[1] && heads[1] == heads[2]);
}

#[test]
fn liveness_is_not_compositional() {
    // Each unit answers whenever it is asked: a liveness guarantee, given its assumption.
    // Composed, each waits for the other and nothing ever happens. Circular assume-guarantee
    // reasoning is unsound for liveness; only safety survives composition unconditionally.
    let open = open_kinds();
    let a = || fwd("a", Some("b-asks"), &["a-asks"]);
    let b = || fwd("b", Some("a-asks"), &["b-asks"]);
    let mut alone = Runtime::new(Box::new(Par::new(vec![a()], &open).unwrap()));
    let entries = alone.inject(&Name::lit("env"), msg("b-asks")).unwrap();
    assert_eq!(entries.iter().map(|e| e.event.clone()).collect::<Vec<_>>(), vec![msg("b-asks"), msg("a-asks")]); // a is live
    let mut both = Runtime::new(Box::new(Par::new(vec![a(), b()], &open).unwrap()));
    assert_eq!(both.inject(&Name::lit("env"), msg("start")).unwrap().len(), 1); // deadlock: only the stimulus itself
}

// ---------------------------------------------------------------- evidence

fn founded() -> World {
    world(Cast::default(), rules()).unwrap()
}
fn assessment(w: &World, p: &Prop) -> Assessment {
    w.epi().unwrap().assessment(p)
}
fn promise(w: &mut World, p: &Prop) {
    let o = Offer {
        creditor: ALICE,
        ant: None,
        cons: Condition::Holds(p.clone()),
        deadline: Time(3),
        tags: std::iter::once(Tag::Conformance).collect(),
        parent: None,
        terms: None,
    };
    let _ = w.act(&VENDOR, Act::Commit(o)).unwrap();
}
fn status(w: &World) -> Status {
    w.one(&VENDOR, Tag::Conformance).unwrap().status.unwrap()
}

#[test]
fn no_self_certification() {
    let mut w = founded();
    let x = fact("x");
    let _ = w.act(&ALICE, Act::Claim { prop: x.clone() }).unwrap();
    let entries = w.act(&ALICE, attest(About::Supports(x.clone()), Method::Ack, "trust me")).unwrap();
    assert_eq!(assessment(&w, &x), Assessment::Unsupported);
    // ...and laundering it through an inference from her own word does not help:
    let own = entries
        .iter()
        .find_map(|e| match &e.event {
            Event::EvidenceRecorded(ev) => Some(ev.id),
            _ => None,
        })
        .unwrap();
    let inference = Attestation {
        about: About::Supports(x.clone()),
        method: Method::Inference,
        derived_from: std::iter::once(own).collect(),
        artifacts: BTreeMap::new(),
        content: Content::Note("so it is done".to_owned()),
    };
    let _ = w.act(&ALICE, Act::Attest(inference)).unwrap();
    assert_eq!(assessment(&w, &x), Assessment::Unsupported);
    // ...while the very same acknowledgement from her about someone else's claim counts:
    let y = fact("y");
    let _ = w.act(&VENDOR, Act::Claim { prop: y.clone() }).unwrap();
    let _ = w.act(&ALICE, attest(About::Supports(y.clone()), Method::Ack, "seen")).unwrap();
    assert_eq!(assessment(&w, &y), Assessment::Accepted);
    // ...and the rule that forbids self-support cannot be amended away:
    let entries = w.act_with(&ALICE, &CapId::Charter(GOV_ALICE), Act::Propose { amendments: vec![Amendment::SetSelfSupport(true)] }).unwrap();
    assert!(entries.iter().any(|e| matches!(&e.event, Event::GovRejected { reason: GovError::Entrenched(RulePath::SelfSupport), .. })));
    // ...nor can the entrenchment itself be lifted:
    let entries = w.act_with(&ALICE, &CapId::Charter(GOV_ALICE), Act::Propose { amendments: vec![Amendment::Disentrench(RulePath::SelfSupport)] }).unwrap();
    assert!(entries.iter().any(|e| matches!(&e.event, Event::GovRejected { reason: GovError::Entrenched(RulePath::Entrenched), .. })));
}

#[test]
fn untrusted_sources_and_unranked_methods_carry_no_weight() {
    let mut w = founded();
    let q = fact("q");
    let _ = w.act(&VENDOR, attest(About::Supports(q.clone()), Method::Probe, "x")).unwrap(); // not a trusted prober
    let _ = w.act(&MONITOR, attest(About::Supports(q.clone()), Method::Hearsay, "x")).unwrap(); // method has no rank
    assert_eq!(assessment(&w, &q), Assessment::Unsupported);
    let ungrounded = attest(About::Supports(q.clone()), Method::Inference, "obviously");
    let entries = w.act(&MONITOR, ungrounded).unwrap();
    assert!(entries.iter().any(|e| matches!(&e.event, Event::EvidenceRejected { reason: EvidenceError::UngroundedInference, .. })));
}

#[test]
fn equal_evidence_stays_contested_no_forced_decision() {
    let mut w = founded();
    let q = fact("q");
    promise(&mut w, &q);
    let _ = w.act(&MONITOR, attest(About::Supports(q.clone()), Method::Probe, "up")).unwrap();
    assert_eq!(status(&w), Status::Satisfied);
    let _ = w.act(&MONITOR, attest(About::Rebuts(q.clone()), Method::Probe, "down")).unwrap();
    assert_eq!(assessment(&w, &q), Assessment::Contested);
    assert_eq!(status(&w), Status::Detached); // undecided is not satisfied
    let _ = w.tick(4).unwrap();
    assert_eq!(status(&w), Status::Violated); // ...and the debtor bears the doubt
}

#[test]
fn late_evidence_does_not_cure_a_missed_deadline() {
    let mut w = founded();
    let q = fact("q");
    promise(&mut w, &q);
    let _ = w.tick(5).unwrap();
    let _ = w.act(&MONITOR, attest(About::Supports(q.clone()), Method::Probe, "up now")).unwrap();
    assert_eq!(assessment(&w, &q), Assessment::Accepted);
    assert_eq!(status(&w), Status::Violated);
}

#[test]
fn retracting_the_refutation_reinstates_everything_except_what_already_happened() {
    // Revision runs in both directions. When the monitor withdraws its probe, the claim is
    // accepted again, the violations and verdicts are withdrawn and the duties they created are
    // voided. The refund that was already paid is NOT undone: effects are irreversible; only
    // judgments are revisable.
    let (mut w, o) = run(Cast::default(), false, |_| {}).unwrap();
    let probe = o
        .refutation
        .iter()
        .find_map(|e| match &e.event {
            Event::EvidenceRecorded(ev) => Some(ev.id),
            _ => None,
        })
        .unwrap();
    assert_eq!(w.one(&DEPLOYER, Tag::Refund).unwrap().status, Some(Status::Satisfied));
    let not_yours = w.act(&AGENT, Act::Retract { evidence: probe }).unwrap();
    assert!(not_yours.iter().any(|e| matches!(&e.event, Event::EvidenceRejected { reason: EvidenceError::NotTheSource, .. })));
    let _ = w.act(&MONITOR, Act::Retract { evidence: probe }).unwrap();
    assert_eq!(w.norms().unwrap().status(&o.c1), Some(Status::Satisfied));
    assert_eq!(w.norms().unwrap().status(&o.c2), Some(Status::Satisfied));
    assert!(w.adj().unwrap().findings(&o.c1).is_empty());
    assert_eq!(w.one(&DEPLOYER, Tag::Refund).unwrap().status, Some(Status::Void));
    let moved = w.rt.log().entries().iter().filter(|e| matches!(e.event, Event::Transferred { .. })).count();
    assert_eq!(moved, 2); // the money stays moved
}

// ---------------------------------------------------------------- governance

fn propose(amendments: Vec<Amendment>) -> GovError {
    let mut w = founded();
    let entries = w.act_with(&ALICE, &CapId::Charter(GOV_ALICE), Act::Propose { amendments }).unwrap();
    entries
        .iter()
        .find_map(|e| match &e.event {
            Event::GovRejected { reason, .. } => Some(reason.clone()),
            _ => None,
        })
        .unwrap()
}

#[test]
fn obligation_regress_is_rejected_statically() {
    // "Violating an audit duty creates an audit duty" would regress forever.
    let looped = ViolationTemplate {
        tag: Tag::Audit,
        create: Template {
            debtor: PartyExpr::RoleHolder(Role::Auditor),
            creditor: PartyExpr::Creditor,
            cons: ConsExpr::Audited,
            within: 1,
            tags: std::iter::once(Tag::Audit).collect(),
        },
    };
    assert_eq!(propose(vec![Amendment::AddViolationTemplate(looped)]), GovError::Invalid(RulesError::TemplateCycle(Tag::Audit)));
}

#[test]
fn rules_without_a_unique_meaning_are_rejected() {
    const X: RTerm = Term::Var(Var(0));
    const C: RTerm = Term::Var(Var(1));
    let r = |id: &'static str, head: Pred, negated: Pred| Rule {
        id: RuleId(Name::new(id).unwrap()),
        head: Atom { pred: head, args: vec![X, C] },
        body: vec![Literal::Pos(Atom { pred: Pred::Debtor, args: vec![C, X] }), Literal::Neg(Atom { pred: negated, args: vec![X, C] })],
    };
    // liable unless excused; excused unless liable: no unique model.
    let circular = vec![r("x1", Pred::Liable, Pred::Excused), r("x2", Pred::Excused, Pred::Liable)];
    assert_eq!(propose(vec![Amendment::AddResponsibilityRules(circular)]), GovError::Invalid(RulesError::Responsibility(RuleError::NotStratifiable)));
    // Ill-sorted: Debtor(commitment, actor) written with its arguments swapped, so the head would
    // call a commitment liable for an actor. Caught where the two uses of a variable disagree.
    let swapped = Rule {
        id: RuleId::lit("x3"),
        head: Atom { pred: Pred::Liable, args: vec![X, C] },
        body: vec![Literal::Pos(Atom { pred: Pred::Debtor, args: vec![X, C] })],
    };
    assert_eq!(
        propose(vec![Amendment::AddResponsibilityRules(vec![swapped])]),
        GovError::Invalid(RulesError::Responsibility(RuleError::IllSorted { rule: RuleId::lit("x3"), pred: Pred::Liable }))
    );
    assert_eq!(propose(vec![Amendment::SetQuorum(0)]), GovError::Invalid(RulesError::QuorumZero));
    assert_eq!(propose(Vec::new()), GovError::EmptyProposal);
}

#[test]
fn quorum_and_authority() {
    let mut w = founded();
    let (ga, gu) = (CapId::Charter(GOV_ALICE), CapId::Charter(GOV_AUDITOR));
    let lower = || Act::Propose { amendments: vec![Amendment::SetQuorum(1)] };
    assert_eq!(rigor::scenario::denial(&w.act(&AGENT, lower()).unwrap()), Some(DenyReason::ActionNotGranted));
    assert_eq!(rigor::scenario::denial(&w.act_with(&AGENT, &ga, lower()).unwrap()), Some(DenyReason::NotHolder));
    let entries = w.act_with(&ALICE, &ga, lower()).unwrap();
    assert_eq!(w.gov().unwrap().version(), Version(1)); // one voice is not a quorum of two
    let id = entries
        .iter()
        .find_map(|e| match &e.event {
            Event::Proposed { id, .. } => Some(*id),
            _ => None,
        })
        .unwrap();
    let _ = w.act_with(&ALICE, &ga, Act::Approve { id }).unwrap(); // voting twice does not help
    assert_eq!(w.gov().unwrap().version(), Version(1));
    let _ = w.act_with(&AUDITOR, &gu, Act::Approve { id }).unwrap(); // the rules amend their own amendment rule
    assert_eq!((w.gov().unwrap().version(), w.gov().unwrap().rules().unwrap().governance.quorum), (Version(2), 1));
    // A proposal made against version 1 cannot be enacted against version 2.
    let stale = w.act_with(&AUDITOR, &gu, Act::Approve { id }).unwrap();
    assert!(stale.iter().any(|e| matches!(&e.event, Event::GovRejected { reason: GovError::NoSuchOpenProposal, .. })));
}

#[test]
fn an_invalid_constitution_is_refused() {
    let mut r = rules();
    r.governance.quorum = 0;
    let mut rt = Runtime::new(root(Cast::default()).unwrap());
    let err = rt.inject(&CONSTITUTION, Event::Genesis(Box::new(constitution(r)))).unwrap_err();
    assert_eq!(err, Fault::Unit { unit: rigor::units::governance::GOV, failure: rigor::model::Failure::InvalidConstitution(RulesError::QuorumZero) });
}

// ---------------------------------------------------------------- norms

#[test]
fn nobody_can_bind_or_release_another() {
    let mut w = founded();
    let q = fact("q");
    promise(&mut w, &q);
    let id = w.one(&VENDOR, Tag::Conformance).unwrap().commitment.id;
    let refused = |entries: &[Entry<Event>]| entries.iter().any(|e| matches!(&e.event, Event::NormRejected { reason: NormError::NotWithinTheActorsPower, .. }));
    assert!(refused(&w.act(&VENDOR, Act::Release { id }).unwrap())); // the debtor cannot release itself
    assert!(refused(&w.act(&AGENT, Act::Release { id }).unwrap())); // nor can a third party
    assert!(refused(&w.act(&AGENT, Act::Assign { id, to: AGENT }).unwrap()));
    assert!(refused(&w.act(&ALICE, Act::Cancel { id }).unwrap())); // the creditor cannot cancel for the debtor
    let _ = w.act(&ALICE, Act::Assign { id, to: AUDITOR }).unwrap(); // the creditor may assign her claim
    assert_eq!(w.one(&VENDOR, Tag::Conformance).unwrap().terms().creditor, AUDITOR);
    let _ = w.act(&VENDOR, Act::Cancel { id }).unwrap(); // repudiating a live duty is a breach
    assert_eq!(status(&w), Status::Violated);
    // There is no act by which one actor commits another: `Commit` has no debtor field.
}

#[test]
fn contrary_to_duty_without_paradox() {
    // Chisholm's scenario: the primary duty is violated AND the secondary duty is in force.
    // In standard deontic logic this is inconsistent; with commitments as objects it is just
    // two records with different statuses.
    let (w, o) = run(Cast::default(), false, |_| {}).unwrap();
    let rem = w.one(&VENDOR, Tag::Remediation).unwrap();
    assert_eq!(rem.terms().basis.as_ref().unwrap().of, o.c1);
    assert_eq!(rem.status, Some(Status::Satisfied));
    assert_eq!(w.norms().unwrap().status(&o.c1), Some(Status::Violated)); // remediation does not erase the violation
}

// ---------------------------------------------------------------- impact

#[test]
fn exact_impact_lies_inside_the_static_cone() {
    let (w, _) = run(Cast::default(), false, |_| {}).unwrap();
    let stimuli = w.rt.stimuli();
    let i = stimuli.iter().position(|(a, _)| *a == Path::of(&MONITOR.0)).unwrap();
    let pre = World { rt: Runtime::replay(root(Cast::default()).unwrap(), &stimuli[..i]).unwrap(), scope: Vec::new() };
    let head = pre.rt.log().head();
    let deployed = Prop::Done(rigor::model::Task::whole(rigor::scenario::TASK1).step(rigor::actors::DEPLOY));
    let bound = cone(&pre, &Change::Prop(deployed.clone())).unwrap();
    let exact = what_if(&pre, &MONITOR.0, stimuli[i].1.clone()).unwrap();
    let mut after = pre.fork();
    let _ = after.inject(&MONITOR.0, stimuli[i].1.clone()).unwrap();
    assert!(exact.len() >= 6);
    for (j, (before, _)) in &exact {
        match (j, before) {
            (_, Some(_)) | (Judgment::Verdict(_), None) => assert!(bound.contains(j), "{j:?} changed outside the cone"),
            // New commitments hang off something inside the cone.
            (Judgment::Commitment(c), None) => {
                let of = after.norms().unwrap().record(c).unwrap().terms().basis.as_ref().unwrap().of;
                assert!(bound.contains(&Judgment::Commitment(of)));
            }
            (Judgment::Prop(_) | Judgment::Cap(_), None) => panic!("unexpected new judgment {j:?}"),
        }
    }
    assert_eq!(pre.rt.log().head(), head, "a what-if never touches the real world");
    // And the cone is far from "everything": an unrelated commitment is outside it.
    let mut other = founded();
    promise(&mut other, &fact("unrelated"));
    assert!(cone(&other, &Change::Prop(deployed)).unwrap().iter().all(|j| !matches!(j, Judgment::Commitment(_))));
}

#[test]
fn capability_cone() {
    let (w, o) = run(Cast::default(), false, |_| {}).unwrap();
    let expect: BTreeSet<Judgment> = [Judgment::Cap(o.cap.clone()), Judgment::Cap(o.subcap.clone())].into();
    assert_eq!(cone(&w, &Change::Cap(o.cap)).unwrap(), expect);
}

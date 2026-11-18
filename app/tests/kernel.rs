//! The kernel's laws: log integrity, determinism, composition, boundaries, interfaces.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::indexing_slicing)]

mod common;

use common::{scenario_parts, Rng};
use rigor::logic::argue::{grounded, grounded_reference, Label};
use rigor::logic::datalog::{self, Atom, Fact, Literal, Rule, RuleError, Signature, Sorted, Term, Var};
use rigor::model::{Act, CapId, DenyReason, Event, Kind, Prop};
use rigor::model::{Fault, LeafError};
use rigor::org::{open_kinds, org};
use rigor::scenario::{constitution, pay, root, rules, run, Cast, AGENT, ALICE, DEPLOYER};
use rigor::units::{adjudicator::ADJ, epistemic::EPI, governance::GOV, institution::INST, ledger::LEDGER, norms::NORMS};
use rigor::waist::{state_digest, Cell, Entity, Envelope, Leaf, Log, Name, Par, Path, Runtime, Unit};
use rigor::world::CONSTITUTION;
use std::collections::BTreeSet;

fn story() -> rigor::world::World {
    run(Cast::default(), true, |_| {}).unwrap().0
}

#[test]
fn chain_detects_tampering() {
    let mut w = story();
    assert!(w.rt.log().verify().is_ok());
    w.rt.log_mut().tamper(10, |e| e.actor = Path::of(&Name::lit("mallory")));
    assert!(w.rt.log().verify().is_err());
}

#[test]
fn log_survives_a_round_trip_through_json() {
    let w = story();
    let mut bytes = Vec::new();
    w.rt.log().save(&mut bytes).unwrap();
    let back: Log<Event> = Log::load(bytes.as_slice()).unwrap();
    assert_eq!(back.entries(), w.rt.log().entries());
}

#[test]
fn replay_is_deterministic() {
    let w = story();
    let again = Runtime::replay(root(Cast::default()).unwrap(), &w.rt.stimuli()).unwrap();
    assert_eq!(again.log().head(), w.rt.log().head());
    assert_eq!(again.digest(), w.rt.digest());
}

#[test]
fn par_is_associative_and_commutative() {
    // Any bracketing and ordering of the same units yields the identical event log.
    let w = story();
    let stimuli = w.rt.stimuli();
    let mut rng = Rng::new(7);
    for _ in 0..8 {
        let mut us = scenario_parts(Cast::default());
        rng.shuffle(&mut us);
        let k = 1 + rng.below(us.len() - 2);
        let right = us.split_off(k);
        let j = rng.below(k);
        let mid = us.split_off(j);
        let open = open_kinds();
        let mut left: Vec<Box<dyn Unit<Event>>> = mid;
        if !us.is_empty() {
            left.push(Box::new(Par::new(us, &open).unwrap()));
        }
        let nested = Par::new(vec![Box::new(Par::new(left, &open).unwrap()), Box::new(Par::new(right, &open).unwrap())], &open).unwrap();
        let rt = Runtime::replay(Box::new(nested), &stimuli).unwrap();
        assert_eq!(rt.log().head(), w.rt.log().head());
    }
}

#[test]
fn entity_scoping_is_functorial() {
    // A whole organisation nested one and two boundaries deep behaves exactly as it does
    // standalone, up to the wrapping: the same system at every scale.
    let w = story();
    let (acme, holding) = (Name::lit("acme"), Name::lit("holding"));
    for depth in [1usize, 2] {
        let none = BTreeSet::new;
        let mut unit: Box<dyn Unit<Event>> = Box::new(Entity::new(acme.clone(), root(Cast::default()).unwrap(), none(), none()));
        let mut prefix = vec![acme.clone()];
        if depth == 2 {
            unit = Box::new(Entity::new(holding.clone(), unit, none(), none()));
            prefix.insert(0, holding.clone());
        }
        let wrap = |mut ev: Event, mut actor: Path| {
            for n in prefix.iter().rev() {
                ev = Event::Scoped { entity: n.clone(), inner: Box::new(ev) };
                actor = actor.inside(n);
            }
            (actor, ev)
        };
        let mut rt = Runtime::new(unit);
        for (actor, ev) in w.rt.stimuli() {
            let (a, e) = wrap(ev, actor);
            let _ = rt.inject_as(a, e).unwrap();
        }
        let flat = w.rt.log().entries();
        let nested = rt.log().entries();
        assert_eq!(flat.len(), nested.len());
        for (f, n) in flat.iter().zip(nested) {
            if f.event == Event::Settle {
                assert_eq!(n.event, Event::Settle);
            } else {
                let (a, e) = wrap(f.event.clone(), f.actor.clone());
                assert_eq!((&n.actor, &n.event), (&a, &e));
            }
        }
    }
}

#[test]
fn a_composite_can_be_substituted_by_an_atomic_unit() {
    // Replace the two-part service by a stateless monolith with the same boundary behaviour:
    // every other unit ends in exactly the same state, though the traces differ.
    let w = story();
    let mono =
        rigor::world::World { rt: Runtime::replay(root(Cast { monolith: true, ..Cast::default() }).unwrap(), &w.rt.stimuli()).unwrap(), scope: Vec::new() };
    assert_eq!(state_digest(w.inst().unwrap()), state_digest(mono.inst().unwrap()));
    assert_eq!(state_digest(w.ledger().unwrap()), state_digest(mono.ledger().unwrap()));
    assert_eq!(state_digest(w.epi().unwrap()), state_digest(mono.epi().unwrap()));
    assert_eq!(state_digest(w.norms().unwrap()), state_digest(mono.norms().unwrap()));
    assert_eq!(w.adj().unwrap().verdicts().collect::<Vec<_>>(), mono.adj().unwrap().verdicts().collect::<Vec<_>>());
    assert_eq!(state_digest(w.gov().unwrap()), state_digest(mono.gov().unwrap()));
    let agent = |x: &rigor::world::World| state_digest(x.rt.leaf::<rigor::actors::Agent>(&[AGENT.0]).unwrap());
    assert_eq!(agent(&w), agent(&mono));
    assert_ne!(w.rt.log().len(), mono.rt.log().len());
    // Typed access is by name AND type: asking a unit for the wrong type yields nothing.
    for n in [INST, LEDGER, EPI, NORMS, ADJ, GOV] {
        assert_eq!(w.rt.leaf::<rigor::units::ledger::Ledger>(std::slice::from_ref(&n)).is_some(), n == LEDGER);
    }
}

#[derive(Clone, Debug, Hash, serde::Serialize, serde::Deserialize)]
struct Probe {
    name: Name,
    listens: Kind,
    emits: Kind,
    actually: Option<Event>,
}

impl Leaf<Event> for Probe {
    fn name(&self) -> Name {
        self.name.clone()
    }
    fn inputs(&self) -> BTreeSet<Kind> {
        std::iter::once(self.listens).collect()
    }
    fn outputs(&self) -> BTreeSet<Kind> {
        std::iter::once(self.emits).collect()
    }
    fn on(&mut self, _: &Envelope<'_, Event>) -> Result<Vec<Event>, LeafError> {
        Ok(self.actually.clone().into_iter().collect())
    }
}

fn probe(name: &'static str, listens: Kind, emits: Kind, actually: Option<Event>) -> Box<dyn Unit<Event>> {
    Box::new(Cell(Probe { name: Name::new(name).unwrap(), listens, emits, actually }))
}

#[test]
fn boundary_hides_internals() {
    let w = story();
    let internal: Vec<_> =
        w.rt.log()
            .entries()
            .iter()
            .filter(|e| matches!(&e.event, Event::Scoped { inner, .. } if matches!(**inner, Event::JobQueued(_) | Event::JobFailed(_))))
            .collect();
    assert!(!internal.is_empty());
    assert!(internal.iter().all(|e| e.actor.0.len() == 2 && e.actor.0[0] == DEPLOYER.0));
    // What leaves the boundary is attributed to the entity, never to a part of it.
    let exported: Vec<_> = w.rt.log().entries().iter().filter(|e| e.src.0.len() == 2 && matches!(e.event, Event::Do { .. })).collect();
    assert!(!exported.is_empty() && exported.iter().all(|e| e.actor == Path::of(&DEPLOYER.0)));
    // Nobody outside may listen to scoped events.
    let spy = probe("spy", Kind::Scoped, Kind::Signal, None);
    assert!(matches!(org(vec![spy]), Err(Fault::ListensInsideBoundary(_))));
}

#[test]
fn interface_enforcement() {
    let open = open_kinds();
    let two: Vec<Box<dyn Unit<Event>>> = vec![probe("a", Kind::Tick, Kind::Signal, None), probe("a", Kind::Tick, Kind::Signal, None)];
    assert!(matches!(Par::new(two, &open), Err(Fault::DuplicateName(_))));
    // A unit that declares it emits institutional events cannot be composed with the institution.
    assert!(matches!(org(vec![probe("rogue", Kind::Tick, Kind::Ok, None)]), Err(Fault::OutputClash { kind: Kind::Ok, .. })));
    // A unit that emits what it did not declare is stopped at run time.
    let liar = probe("liar", Kind::Tick, Kind::Signal, Some(Event::Settle));
    let mut rt = Runtime::new(Box::new(Par::new(vec![liar], &open).unwrap()));
    assert!(matches!(rt.inject(&Name::lit("clock"), Event::Tick(rigor::model::Time(1))), Err(Fault::Undeclared { kind: Kind::Settle, .. })));
}

#[test]
fn a_unit_cannot_impersonate() {
    // There is no "actor" field for a unit to fill in: the kernel stamps it. A unit that is not
    // a member is denied even when it presents someone else's capability.
    let forged = Event::Do { act: pay(ALICE, AGENT, 5), cap: Some(CapId::Own(ALICE)) };
    let mallory = probe("mallory", Kind::Tick, Kind::Do, Some(forged));
    let mut rt = Runtime::new(Box::new(org(vec![mallory]).unwrap()));
    let _ = rt.inject(&CONSTITUTION, Event::Genesis(Box::new(constitution(rules())))).unwrap();
    let entries = rt.inject(&Name::lit("clock"), Event::Tick(rigor::model::Time(1))).unwrap();
    let denials: Vec<_> = entries
        .iter()
        .filter_map(|e| match &e.event {
            Event::Denied { reason, .. } => Some(reason.clone()),
            _ => None,
        })
        .collect();
    assert_eq!(denials, vec![DenyReason::NotAMember]);
}

#[test]
fn divergence_is_detected_not_hidden() {
    let sig = Event::Signal { to: ALICE, prop: Prop::Fact(Name::lit("ping")), value: true };
    let parts = vec![probe("ping", Kind::Signal, Kind::Signal, Some(sig.clone())), probe("pong", Kind::Signal, Kind::Signal, Some(sig.clone()))];
    let mut rt = Runtime::new(Box::new(Par::new(parts, &open_kinds()).unwrap())).with_fuel(200);
    assert_eq!(rt.inject(&Name::lit("env"), sig).unwrap_err(), Fault::Divergence { fuel: 200 });
}

#[test]
fn names_are_validated() {
    assert!(Name::new("alice").is_ok());
    for bad in ["", "Alice", "a b", "a/b", "done:task"] {
        assert!(Name::new(bad).is_err(), "{bad:?}");
    }
    assert!(serde_json::from_str::<Name>("\"not valid\"").is_err());
    assert!(serde_json::from_str::<Act>("{\"Frobnicate\":{}}").is_err());
}

// ---------------------------------------------------------------- reasoners

#[test]
fn grounded_matches_the_textbook_definition_on_random_graphs() {
    let mut rng = Rng::new(1);
    for _ in 0..300 {
        let n = 1 + rng.below(8);
        let nodes: BTreeSet<usize> = (0..n).collect();
        let edges = rng.below(2 * n);
        let attacks: BTreeSet<(usize, usize)> = (0..edges).map(|_| (rng.below(n), rng.below(n))).collect();
        assert_eq!(grounded(&nodes, &attacks), grounded_reference(&nodes, &attacks));
    }
}

#[test]
fn reinstatement_and_stalemate() {
    let l = grounded(&[1, 2, 3].into(), &[(1, 2), (2, 3)].into());
    assert_eq!((l[&1], l[&2], l[&3]), (Label::In, Label::Out, Label::In));
    let l = grounded(&[1, 2].into(), &[(1, 2), (2, 1)].into());
    assert_eq!((l[&1], l[&2]), (Label::Undecided, Label::Undecided));
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum P {
    Edge,
    Node,
    Reach,
    Isolated,
    Flag,
    Other,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum S {
    Num,
    Text,
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum K {
    N(u8),
    T(&'static str),
}
impl Signature for P {
    type Sort = S;
    fn signature(self) -> &'static [S] {
        match self {
            Self::Edge | Self::Reach => &[S::Num, S::Num],
            Self::Node | Self::Isolated | Self::Flag | Self::Other => &[S::Num],
        }
    }
}
impl Sorted<S> for K {
    fn sort(&self) -> S {
        match self {
            Self::N(_) => S::Num,
            Self::T(_) => S::Text,
        }
    }
}
type R = Rule<u8, P, K>;
const X: Term<K> = Term::Var(Var(0));
const Y: Term<K> = Term::Var(Var(1));
const Z: Term<K> = Term::Var(Var(2));
fn pos(pred: P, args: &[Term<K>]) -> Literal<P, K> {
    Literal::Pos(Atom { pred, args: args.to_vec() })
}
fn neg(pred: P, args: &[Term<K>]) -> Literal<P, K> {
    Literal::Neg(Atom { pred, args: args.to_vec() })
}
fn rule(id: u8, pred: P, head: &[Term<K>], body: Vec<Literal<P, K>>) -> R {
    Rule { id, head: Atom { pred, args: head.to_vec() }, body }
}
fn f(pred: P, args: &[u8]) -> Fact<P, K> {
    Fact { pred, args: args.iter().map(|n| K::N(*n)).collect() }
}

#[test]
fn datalog_negation_and_proof() {
    let rules = vec![
        rule(1, P::Reach, &[X, Y], vec![pos(P::Edge, &[X, Y])]),
        rule(2, P::Reach, &[X, Z], vec![pos(P::Reach, &[X, Y]), pos(P::Edge, &[Y, Z])]),
        rule(3, P::Isolated, &[X], vec![pos(P::Node, &[X]), neg(P::Reach, &[Term::Const(K::N(1)), X])]),
    ];
    let m = datalog::solve(&rules, [f(P::Edge, &[1, 2]), f(P::Edge, &[2, 3]), f(P::Node, &[3]), f(P::Node, &[4])]).unwrap();
    assert!(m.holds(&f(P::Reach, &[1, 3])));
    assert_eq!(m.of(P::Isolated).cloned().collect::<Vec<_>>(), vec![f(P::Isolated, &[4])]);
    assert_eq!(m.assumptions(&f(P::Isolated, &[4])), [f(P::Reach, &[1, 4])].into());
    assert!(matches!(m.proof(&f(P::Reach, &[1, 3])), datalog::Proof::Derived { rule: 2, .. }));
}

#[test]
fn datalog_rejects_what_has_no_unique_meaning() {
    // p :- not q.  q :- not p.   (no unique model)
    let cyc =
        vec![rule(1, P::Flag, &[X], vec![pos(P::Node, &[X]), neg(P::Other, &[X])]), rule(2, P::Other, &[X], vec![pos(P::Node, &[X]), neg(P::Flag, &[X])])];
    assert_eq!(datalog::check(&cyc), Err(RuleError::NotStratifiable));
    assert_eq!(datalog::check(&[rule(1, P::Flag, &[X], vec![neg(P::Other, &[X])])]), Err(RuleError::UnsafeHead { rule: 1 }));
    assert_eq!(datalog::check(&[rule(1, P::Reach, &[X, Y], vec![pos(P::Node, &[X])])]), Err(RuleError::UnsafeHead { rule: 1 }));
    assert_eq!(datalog::check(&[rule(1, P::Flag, &[X], vec![pos(P::Node, &[X, X])])]), Err(RuleError::Arity { rule: 1, pred: P::Node }));
    // Ill-sorted: a text constant where a number is required.
    assert_eq!(
        datalog::check(&[rule(1, P::Flag, &[X], vec![pos(P::Edge, &[X, Term::Const(K::T("two"))])])]),
        Err(RuleError::IllSorted { rule: 1, pred: P::Edge })
    );
    let dup = vec![rule(1, P::Flag, &[X], vec![pos(P::Node, &[X])]), rule(1, P::Other, &[X], vec![pos(P::Node, &[X])])];
    assert_eq!(datalog::check(&dup), Err(RuleError::DuplicateId(1)));
}

// ---------------------------------------------------------------- the runtime against the proved model

/// A line-by-line transcription of the Lean model in proofs/Rigor.lean: `Unit`, `par`, `drive`.
/// States and events are small numbers; a unit is its initial state and its step table.
mod lean_model {
    pub type Table = Vec<Vec<(u8, Vec<u8>)>>; // table[state][event] = (next state, outputs)

    #[derive(Clone)]
    pub enum Unit {
        Leaf(Table),
        Par(Box<Unit>, Box<Unit>),
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum State {
        Leaf(u8),
        Par(Box<State>, Box<State>),
    }

    impl Unit {
        pub fn init(&self) -> State {
            match self {
                Self::Leaf(_) => State::Leaf(0),
                Self::Par(a, b) => State::Par(Box::new(a.init()), Box::new(b.init())),
            }
        }
        // def par ... step := fun s e => let a := A.step s.1 e; let b := B.step s.2 e; ((a.1, b.1), a.2 ++ b.2)
        pub fn step(&self, s: &State, e: u8) -> (State, Vec<u8>) {
            match (self, s) {
                (Self::Leaf(t), State::Leaf(x)) => {
                    let (next, out) = t[usize::from(*x)][usize::from(e)].clone();
                    (State::Leaf(next), out)
                }
                (Self::Par(a, b), State::Par(sa, sb)) => {
                    let (na, mut oa) = a.step(sa, e);
                    let (nb, ob) = b.step(sb, e);
                    oa.extend(ob);
                    (State::Par(Box::new(na), Box::new(nb)), oa)
                }
                _ => panic!("state shape follows unit shape"),
            }
        }
    }

    // def drive : Nat → U.S → List E → List E
    //   | 0, _, _ => [] | _ + 1, _, [] => [] | n + 1, s, e :: q => e :: drive U n (U.step s e).1 (q ++ (U.step s e).2)
    pub fn drive(u: &Unit, fuel: usize, mut s: State, mut q: std::collections::VecDeque<u8>) -> Vec<u8> {
        let mut log = Vec::new();
        for _ in 0..fuel {
            let Some(e) = q.pop_front() else { break };
            let (next, out) = u.step(&s, e);
            log.push(e);
            q.extend(out);
            s = next;
        }
        log
    }
}

/// The same step table as a real unit of the real kernel.
#[derive(Clone, Debug, Hash, serde::Serialize, serde::Deserialize)]
struct Tabled {
    name: Name,
    state: u8,
    table: lean_model::Table,
}

fn symbol(n: u8) -> Prop {
    Prop::Fact(Name::new(&format!("e{n}")).unwrap())
}

impl Leaf<Event> for Tabled {
    fn name(&self) -> Name {
        self.name.clone()
    }
    fn inputs(&self) -> BTreeSet<Kind> {
        std::iter::once(Kind::Signal).collect()
    }
    fn outputs(&self) -> BTreeSet<Kind> {
        std::iter::once(Kind::Signal).collect()
    }
    fn on(&mut self, env: &Envelope<'_, Event>) -> Result<Vec<Event>, LeafError> {
        let Event::Signal { prop, .. } = env.event else { return Ok(Vec::new()) };
        let e = (0..5u8).find(|n| symbol(*n) == *prop).unwrap();
        let (next, out) = self.table[usize::from(self.state)][usize::from(e)].clone();
        self.state = next;
        Ok(out.into_iter().map(|n| Event::Signal { to: ALICE, prop: symbol(n), value: true }).collect())
    }
}

#[test]
fn the_runtime_agrees_with_a_transcription_of_the_proved_model() {
    // 400 random triples of units, composed and driven two ways: by the real kernel (Par,
    // Runtime, the queue, the log) and by a direct transcription of the Lean definitions that
    // theorems T1-T9 are about. The logs must be identical, event for event, including when the
    // cascade is cut off by fuel. This is evidence that the code realises the model; it is a
    // test of a transcription, not a proof.
    let mut rng = Rng::new(11);
    let (states, events) = (3usize, 5usize);
    let mut diverged = 0;
    for case in 0..400 {
        let tables: Vec<lean_model::Table> = (0..3)
            .map(|_| {
                (0..states)
                    .map(|_| {
                        (0..events)
                            .map(|_| {
                                (
                                    u8::try_from(rng.below(states)).unwrap(),
                                    (0..rng.below(3).saturating_sub(usize::from(rng.percent(55)))).map(|_| u8::try_from(rng.below(events)).unwrap()).collect(),
                                )
                            })
                            .collect()
                    })
                    .collect()
            })
            .collect();
        let first = u8::try_from(rng.below(events)).unwrap();
        let fuel = 60;

        // The model: (a ∥ b) ∥ c.
        let leaf = |i: usize| Box::new(lean_model::Unit::Leaf(tables[i].clone()));
        let model = lean_model::Unit::Par(Box::new(lean_model::Unit::Par(leaf(0), leaf(1))), leaf(2));
        let expected = lean_model::drive(&model, fuel, model.init(), std::iter::once(first).collect());

        // The real thing, bracketed the same way.
        let open = open_kinds();
        let unit =
            |i: usize, n: &'static str| -> Box<dyn Unit<Event>> { Box::new(Cell(Tabled { name: Name::new(n).unwrap(), state: 0, table: tables[i].clone() })) };
        let inner = Par::new(vec![unit(0, "a"), unit(1, "b")], &open).unwrap();
        let root = Par::new(vec![Box::new(inner), unit(2, "c")], &open).unwrap();
        let mut rt = Runtime::new(Box::new(root)).with_fuel(u64::try_from(fuel).unwrap());
        let outcome = rt.inject(&Name::lit("env"), Event::Signal { to: ALICE, prop: symbol(first), value: true });
        diverged += usize::from(outcome.is_err());
        let actual: Vec<u8> = rt
            .log()
            .entries()
            .iter()
            .map(|e| match &e.event {
                Event::Signal { prop, .. } => (0..5u8).find(|n| symbol(*n) == *prop).unwrap(),
                other => panic!("unexpected {other:?}"),
            })
            .collect();
        assert_eq!(actual, expected, "case {case}");
    }
    assert!(diverged > 20 && diverged < 380, "both terminating and fuel-limited cascades must be covered: {diverged}");
}

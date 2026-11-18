# rigor

A small, executable foundation for modelling and governing systems made of humans, AI agents, services and
organisations — with authority, obligations, evidence and rule changes handled by the same compositional
semantics at every scale.

This is a research prototype. It runs, it is tested, a core of it is machine-proved, and its limits are
written down. It is not production software.

The implementation is Rust (`crates/rigor`), strictly typed: event kinds, acts, propositions, statuses,
findings, rule paths, templates and refusal reasons are closed types; identifiers are distinct newtypes;
nothing is decided by inspecting a string. `unsafe`, `unwrap`, `expect`, `panic!`, indexing and `as` casts
are denied by lint in library code.

## Run it

```sh
cargo run --bin scenario                      # the end-to-end demonstrator, narrated
cargo run --bin steward                       # the project checks itself and records the evidence
cargo test                                    # 59 tests + 8 doctests; includes 4,000 real commits
cargo clippy --all-targets                    # the strict lint set in Cargo.toml; must be clean
cargo run --release --bin measure             # regenerates docs/04-measurements.md
cargo run --release --bin stress -- <bare git repo>...   # real histories through the whole system
lean proofs/Rigor.lean                        # 7 machine-checked theorems about the model
```

Dependencies: `serde`, `serde_json`, `sha2`. Lean 4 is needed only to re-check the proofs.

## The idea in one screen

**Kernel.** An append-only hash-chained event log, and *units*: deterministic open transition systems,
generic in the event type. Units compose in parallel (`Par`) and can be wrapped in a boundary (`Entity`) that
makes a whole system look like one actor. A composite is a unit. That is the only recursion principle.

**Everything else is an ordinary unit**, built from established theory:

| Unit | Does | Borrowed from |
|---|---|---|
| `inst` | An attempt *counts as* an act only with a valid, attenuated capability chain | count-as rules, object capabilities |
| `ledger` | Moves resources; conserves totals | resource algebras |
| `epi` | Assesses claims from admissible evidence; revises when evidence, artifacts or rules change | grounded argumentation |
| `norms` | Commitments whose status is a function of current facts; secondary duties on violation | social commitments |
| `adj` | Derives who is accountable, culpable, liable — with a proof and its assumptions | sorted, stratified Datalog |
| `gov` | Rules are versioned data, amended only by the rules' own procedure | rules-as-data, entrenchment |

Humans, AI agents and services are not different kinds of thing in the kernel. Their differences are
attributes that only the *rules* read (for example: an AI agent can be at fault, but liability lifts to its
principal).

## What the demonstrator shows

Alice delegates a task and a 50-credit budget to an AI agent. The agent subcontracts a deployment service and
pays it 30. The service fails and says so; the agent reports success anyway, citing the service's receipt.

1. The claim is **accepted**; both commitments read *satisfied*; Alice's promise to pay becomes due.
2. A monitor's probe **refutes** the deployment. In one cascade: the receipt is defeated, the agent's inference
   falls with it, both commitments flip to *violated*, and Alice's duty to pay disappears.
3. Rules create an **audit** duty, a **remediation** duty on the agent's vendor, and a **refund** duty on the
   service — which pays.
4. The verdict: the agent is *accountable* and *culpable* (it was notified of the failure and claimed
   success), the service *contributed*, the vendor is *liable*. Each finding carries its derivation.
5. Over-delegation, overspending and use of a revoked capability (including a sub-delegated one) are **denied**.
6. An entrenched rule cannot be amended; a valid amendment needs a quorum and produces rule version 2.

## It governs itself

`cargo run --bin steward` treats this repository as a member of its own model. "The tests pass", "the lints
are clean", "the formatting is canonical" and "the proofs check" are propositions; the only evidence for them
is really running the checks, bound to digests of the real files.

- Unchanged files mean earlier evidence still counts and nothing is re-run; a changed file makes the evidence
  about it stale.
- A day on which a claim did not hold stays in the log as a violated commitment, and creates a duty to put it
  right within a week, discharged only by evidence that the requirement holds again.
- The roadmap's next items are commitments in the same world, each with a review-by day, closed only by the
  project's acceptance.
- Its log survives restarts and format changes: it has already migrated its own log once.

A **reflector** unit goes one step further for rules: it records how each method of evidence fares and, past
a threshold, proposes lowering that method's rank through ordinary governance. It cannot enact anything.

None of this writes code. It notices, records, proposes and holds to account. `scripts/pre-push` is an opt-in
git hook that runs the steward (`git config core.hooksPath scripts`).

## It has met real data

Three public git histories — git, Node.js, Kubernetes; 276,548 commits, 3.7 million events — were run through
the whole system, with reverts as real refutations, newcomers admitted by sponsors under delegated authority,
a rule amendment applied to each record, and the three projects nested as separate organisations in one
world. Its conclusions matched an independent oracle throughout. The work exposed a 20-fold scaling defect,
unbounded restart cost, a log format that could not evolve, and several missing concepts; those were fixed.
What it still does not handle is listed in `docs/06-real-data.md`. A 4,000-commit real slice is part of
`cargo test`.

## Where to read

| File | Content |
|---|---|
| `docs/01-prior-art.md` | Comparison of 24 families of formalisms; what was reused; the counterexample behind each addition |
| `docs/02-semantics.md` | The formal model, the typing discipline, and answers to the ten research questions |
| `docs/03-status-ledger.md` | **Every claim, marked proved / compiler-enforced / model-checked / tested / assumed / unresolved**; negative results; trusted assumptions |
| `docs/04-measurements.md` | Trusted-base size, encoding overhead, scaling, state-space growth (release build) |
| `docs/05-roadmap.md` | What it lacks: address next / deferred / parked, with reasons and triggers |
| `docs/06-real-data.md` | Stress against three real git histories (276,548 commits): results, what broke, what remains |

## Layout

```
crates/rigor/src/
  kernel/      base.rs log.rs unit.rs runtime.rs   the trusted substrate (generic in the event type)
               explore.rs                          bounded model checker
  logic/       argue.rs datalog.rs                 the two reasoners (generic)
  model/       ids.rs event.rs rules.rs            the closed, typed vocabulary
  units/       institution ledger epistemic norms adjudicator governance reflector
  org.rs actors.rs scenario.rs world.rs impact.rs adapters.rs synth.rs
  gitlog.rs steward.rs                             real-data adapter; self-stewardship
  bin/         scenario.rs measure.rs stress.rs steward.rs
crates/rigor/tests/   kernel scenario properties counterexamples adapters realworld
proofs/Rigor.lean
```

## The most important caveats

- The Lean theorems are about a mathematical model. Nothing proves that the Rust code realises that model;
  tests are the only link.
- The system never establishes that a claim is *true*. It computes an assessment from evidence supplied by
  sources it was told to trust.
- There is one log and one clock. Distribution, partitions and confidentiality are not addressed.
- Real data has exercised admission, delegation, commitments, evidence, adjudication, governance and nesting,
  but not resource transfers or spending limits, and a snapshot is trusted rather than attested.
- Liveness is not established for anything, and is shown not to compose.
- Strict typing closes the vocabulary: adding a new kind of proposition, tag or predicate means changing an
  enum and recompiling. That is a deliberate trade, recorded as U16 in the ledger.

The full list is in `docs/03-status-ledger.md`.

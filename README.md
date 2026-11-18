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
cargo run --bin steward                       # the project checks and examines itself; writes the dashboard
cargo run --bin steward -- cycle 2            # the same, plus research on 2 open questions via two models
cargo run --bin steward -- can                # what it can do, what it cannot, and what is lacking
cargo run --bin scenario                      # the original end-to-end demonstrator, narrated
cargo test                                    # 82 tests + 8 doctests; includes seven slices of real data
cargo clippy --all-targets                    # the strict lint set in Cargo.toml; must be clean
cargo run --release --bin measure             # regenerates docs/04-measurements.md
cargo run --release --bin stress -- <bare git repo>...   # real histories through the whole system
lean proofs/Rigor.lean                        # 9 machine-checked theorems about the model
```

Model calls go through the Vercel AI Gateway and need a credential (`AI_GATEWAY_API_KEY`, or a key file) and
a budget granted by the owner; without them the system says it cannot research and carries on with the rest.

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
- Its log survives restarts and format changes: it has migrated its own log twice, once across a real
  change in the shape of events (after first losing track of it; see N18 in the ledger).

A **reflector** unit goes one step further, for the rules themselves. It records how each method of evidence
fares and how late refutations arrive, and past a threshold it proposes a change — a lower rank for a method,
a longer maintenance period — through ordinary governance. It cannot enact anything. On the real histories
it lengthened each project's period two or three times, differently for each, and proposed no rank change.

None of this writes code. It notices, records, proposes and holds to account. `scripts/pre-push` is an opt-in
git hook that runs the steward (`git config core.hooksPath scripts`).

## It has met real data

- **Three public git histories** — git, Node.js, Kubernetes; 276,548 commits, 4.3 million events — with
  reverts as refutations, 128,112 recorded reviews as evidence, newcomers admitted by sponsors under delegated
  authority, the rules revised from the record, and the three projects nested as separate organisations in
  one world. Conclusions matched an independent oracle throughout.
- **Real transfers of value**: 3,101 real Bitcoin transactions, each signed input treated as a delegation
  limited to its amount and bound to its transaction. None was wrongly refused, every replay was refused,
  and every balance equals plain bookkeeping.
- **A real package registry**: 13,754 crate releases and 910 yanks. A release rests on what it depends on,
  and a yank is a retraction: 308 releases that were never yanked are not held fit because they rest on one
  that was. Matches an independent oracle.
- **Real forge reviews**: a sample of 52 pull requests, approvals for and standing objections against.

Separately, on a constructed case, **two institutions interact**: an organisation behind a federated boundary
is held to a duty by the outer institution, does the work under its own, and can only *claim* the result
outward.

The work exposed two scaling defects (one 20-fold, one 300-fold), unbounded restart cost and memory, a log
format that could not evolve, and several missing concepts; those were fixed. What it still does not handle is listed in
`docs/06-real-data.md`. Slices of all of this data are part of `cargo test`.

## Where to read

| File | Content |
|---|---|
| `docs/01-prior-art.md` | Comparison of 24 families of formalisms; what was reused; the counterexample behind each addition |
| `docs/02-semantics.md` | The formal model, the typing discipline, and answers to the ten research questions |
| `docs/03-status-ledger.md` | **Every claim, marked proved / compiler-enforced / model-checked / tested / assumed / unresolved**; negative results; trusted assumptions |
| `docs/04-measurements.md` | Trusted-base size, encoding overhead, scaling, state-space growth (release build) |
| `docs/05-roadmap.md` | What it lacks: address next / deferred / parked, with reasons and triggers |
| `docs/07-running-itself.md` | How it runs itself: members, one turn, its account of itself, change control, the owner's commands |
| `docs/06-real-data.md` | Stress against real data (git histories, Bitcoin blocks, a package registry, forge reviews): results, what broke, what remains |

## Layout

The dependency direction is enforced by Cargo: nothing below depends on anything above it.

```
crates/kernel/         names, hash-chained log, units, composition, runtime, snapshots   (knows no institution)
crates/logic/          grounded argumentation; sorted, stratified Datalog               (knows no institution)
crates/institution/    the typed vocabulary and the institutional units; performs no I/O
crates/adapters/host/      files, processes, stored logs, snapshots, migration, re-founding
crates/adapters/git/       git histories
crates/adapters/chain/     blocks of transfers
crates/adapters/registry/  a package registry
crates/adapters/forge/     a forge's public event archive
crates/mind/           language models through an AI gateway
crates/demo/           the demonstrator scenario, its actors, synthetic worlds
crates/steward/        the system looking after itself: checks, self-model, research, change control, dashboard
crates/rigor/          a facade re-exporting all of it; the binaries; the tests
proofs/Rigor.lean
research/              prior research this project builds on (drafts and red-team rounds)
docs/research/         answers the system obtained itself, admitted through its own change control
redteam/               the most recent independent attack reports
```

## The most important caveats

- The Lean theorems are about a mathematical model. Nothing proves that the Rust code realises that model;
  tests are the only link.
- The system never establishes that a claim is *true*. It computes an assessment from evidence supplied by
  sources it was told to trust.
- There is one log and one clock. Distribution, partitions and confidentiality are not addressed.
- Real data has exercised every unit on three kinds of record, but through mappings that are
  interpretations (a revert is a refutation; a signature is a bounded delegation; a yank is a retraction),
  checked against oracles written by the same author.
- Liveness is not established for anything, and is shown not to compose.
- Strict typing closes the vocabulary: adding a new kind of proposition, tag or predicate means changing an
  enum and recompiling. That is a deliberate trade, recorded as U16 in the ledger.

The full list is in `docs/03-status-ledger.md`.

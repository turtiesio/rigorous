# rigor

A small, executable foundation for modelling and governing systems made of humans, AI agents, services and
organisations — with authority, obligations, evidence and rule changes handled by the same compositional
semantics at every scale.

This is a research prototype. It runs, it is tested, a core of it is machine-proved, and its limits are
written down. It is not production software.

## Run it

Requires Python 3.12 (standard library only). Lean 4 is needed only to re-check the proofs.

```sh
python -m rigor.scenario                      # the end-to-end demonstrator, narrated
python -m unittest discover -s tests -t .     # 49 tests: properties, model checks, counterexamples
lean proofs/Rigor.lean                        # 7 machine-checked theorems about the model
python -m bench.measure                       # regenerates docs/04-measurements.md (~20 s)
```

## The idea in one screen

**Kernel (227 lines).** An append-only hash-chained event log, and *units*: deterministic open transition
systems. Units compose in parallel (`Par`) and can be wrapped in a boundary (`Entity`) that makes a whole
system look like one actor. A composite is a unit. That is the only recursion principle.

**Everything else is an ordinary unit**, built from established theory:

| Unit | Does | Borrowed from |
|---|---|---|
| `inst` | An attempt *counts as* an act only with a valid, attenuated capability chain | count-as rules, object capabilities |
| `ledger` | Moves resources; conserves totals | resource algebras |
| `epi` | Assesses claims from admissible evidence; revises when evidence, artifacts or rules change | grounded argumentation |
| `norms` | Commitments whose status is a function of current facts; secondary duties on violation | social commitments |
| `adj` | Derives who is accountable, culpable, liable — with a proof and its assumptions | stratified Datalog |
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

## Where to read

| File | Content |
|---|---|
| `docs/01-prior-art.md` | Comparison of 23 families of formalisms; what was reused; the counterexample behind each addition |
| `docs/02-semantics.md` | The formal model, and answers to the ten research questions |
| `docs/03-status-ledger.md` | **Every claim, marked proved / model-checked / tested / assumed / unresolved**; negative results; trusted assumptions |
| `docs/04-measurements.md` | Trusted-base size, encoding overhead, scaling, state-space growth |
| `docs/05-roadmap.md` | What to do next, ordered by which assumption it removes |

## Layout

```
rigor/kernel/    event.py unit.py            the trusted substrate
                 argue.py datalog.py         the two reasoners
                 explore.py                  bounded model checker
rigor/norms/     institution ledger epistemic commitments adjudicator governance org
rigor/           actors.py scenario.py impact.py adapters.py synth.py
proofs/Rigor.lean
tests/           test_kernel test_scenario test_properties test_counterexamples test_adapters
bench/measure.py
```

## The most important caveats

- The Lean theorems are about a mathematical model. Nothing proves the Python code implements that model;
  only tests connect them.
- The system never establishes that a claim is *true*. It computes an assessment from evidence supplied by
  sources it was told to trust.
- There is one log and one clock. Distribution, partitions and confidentiality are not addressed.
- Liveness is not established for anything, and is shown not to compose.

The full list is in `docs/03-status-ledger.md` §4–§6.

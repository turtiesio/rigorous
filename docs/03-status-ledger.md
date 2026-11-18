# Status ledger

Every claim this repository makes, with how it is supported. Five statuses, kept strictly apart:

- **Proved** — machine-checked in Lean 4 (`proofs/Rigor.lean`). These are theorems about the *mathematical
  model* in `02-semantics.md`. They say nothing directly about the Python code.
- **Model-checked** — exhaustive exploration of a *finite instance* to a fixed point.
- **Tested** — seeded randomised or scenario tests of the Python implementation. Evidence, not proof.
- **Assumed** — taken on trust from outside the system.
- **Unresolved** — not established here.

Run `python -m unittest discover -s tests -t .` (49 tests) and `lean proofs/Rigor.lean`.

## 1. Proved (about the model)

| Id | Statement | Lean name | Axioms used |
|---|---|---|---|
| T1 | Invariants of the parts, each inductive on its own unit, hold jointly on the parallel composite | `inv_par`, `inv_run` | none |
| T2 | Refinement (forward simulation with equal outputs) of each part gives refinement of the composite | `sim_par` | none |
| T3 | Refinement implies equal output traces on every input sequence | `sim_trace` | none |
| T4 | `(A ∥ B) ∥ C` and `A ∥ (B ∥ C)` simulate each other with identical outputs | `par_assoc` | propext |
| T5 | Two events outside each other's footprint commute on the composite state | `commute` | none |
| T6 | The capability at the end of any attenuating chain grants no action and no limit beyond the root's | `no_amplification` | none |
| T7 | An enabled transfer between distinct holders preserves the total | `transfer_conserves` | propext, Quot.sound |

**Gap between model and code.** The Lean model has binary `par` over a single event type and no scheduler:
it omits the feedback queue, `settle`, `Entity`, kind patterns, and all six institutional units. The Python
code is connected to these theorems only by the tests below. That correspondence is **unresolved** (U1).

## 2. Model-checked (finite instances)

| Id | Statement | Instance | Test |
|---|---|---|---|
| M1 | Absorbing statuses are never left; `satisfied` is undone only by reassessment or release; nothing is pending after its deadline; a debtor cannot release itself | one commitment; 12 stimuli; explored to a fixed point | `test_model_check_single_commitment` |
| M2 | Mutually independent stimuli reach one state under every ordering | 5 stimuli (120 orders) in tests; 6 (720) in the measurements | `test_independent_stimuli_commute` |
| M3 | The explorer reports a shortest counterexample for a false invariant | seeded bug | `test_model_checker_finds_seeded_bug` |

## 3. Tested (implementation)

| Id | Statement | Extent | Test |
|---|---|---|---|
| P1 | Any bracketing and ordering of the same units produces the identical log | 8 random bracketings of the full scenario | `test_par_is_associative_and_commutative` |
| P2 | An organisation nested 1 or 2 boundaries deep produces the standalone log up to prefix | full scenario | `test_entity_scoping_is_functorial` |
| P3 | A two-part service can be replaced by a monolith leaving every other unit's state identical | full scenario | `test_composite_can_be_substituted_by_an_atomic_unit` |
| P4 | Replay reproduces the log bit-for-bit; any edit to a stored event is detected | full scenario | `test_replay_is_deterministic`, `test_log_persistence_and_tamper_detection` |
| P5 | Under random delegate/transfer/revoke/tick attempts: attenuation holds, no chain overspends, revoked authority never acts, only the owner's line moves funds, credit is conserved | 25 seeds × 120 steps | `test_random_delegation_trees_never_amplify_authority` |
| P6 | Incremental (component-local) assessment equals from-scratch assessment after every event | 40 seeds × 60 steps | `test_incremental_assessment_equals_from_scratch` |
| P7 | The grounded labelling algorithm equals the textbook least fixed point | 300 random graphs ≤ 8 nodes | `test_matches_textbook_definition_on_random_graphs` |
| P8 | Work caused by one refutation does not depend on world size | N = 20 vs 200 exactly; timed to N = 5000 | `test_revision_work_is_independent_of_world_size`, measurements §3 |
| P9 | Judgments changed by a stimulus lie inside the static cone of the change | **one** point of one scenario | `test_exact_impact_lies_inside_the_static_cone` |
| P10 | The nine demonstrations of the brief | scenario and four variants | `tests/test_scenario.py` |
| P11 | Evidence bound to a file digest goes stale when the file changes, using a real subprocess | one loop | `tests/test_adapters.py` |

## 4. Assumed (the trusted-assumption inventory)

| Id | Assumption | What breaks if it is false |
|---|---|---|
| A-GENESIS | The genesis event (members, root capabilities, rules) is the intended constitution | Everything; it is the only trust anchor |
| A-AUTH | An external stimulus really comes from the actor it names | All authority checks |
| A-ORDER | One total order of events exists (a single log) | Revocation, budgets, deadlines: see N3 |
| A-CLOCK | `tick` events are monotone and reflect real time | Deadlines, expiry |
| A-HASH | SHA-256 is collision resistant; note ids are **truncated** (64-bit event ids, 40-bit object ids) | Log integrity; identity of commitments/evidence at large scale |
| A-TRUST | The `trusted` table names sources that report honestly for each method | Every assessment |
| A-ADAPTER | Adapters report what they observed (file digests, exit codes) | Conformance evidence |
| A-UNIT | Each unit's transition function is deterministic and touches only its own state | Replay, composition laws. Python does not enforce this |
| A-RULES | The rule set expresses what the institution means | Verdicts are sound for the rules, not for justice |
| A-OBS | "Notified" means a message was delivered, not that anyone understood it | The `knew` finding |
| A-TCB | The ~1,170 lines of kernel, reasoners and institutional units are correct | Everything not independently proved |

## 5. Unresolved

| Id | Open item |
|---|---|
| U1 | No proof connects the Lean model to the Python code |
| U2 | Termination of a cascade is not guaranteed for arbitrary units (only a fuel bound). Acyclic templates are *intended* to bound chains of secondary duties; that is not proved |
| U3 | Soundness of the static impact cone is tested at one point only |
| U4 | Only the adjudicator waits for quiescence. Any other unit that reads several facts at once can still see a half-propagated state |
| U5 | Commutativity of `Par` is tested, not proved. There is no static independence analysis and no partial-order reduction; independence is demonstrated per case |
| U6 | No liveness property of any kind is established |
| U7 | Refinement relations must be supplied by hand; there is no automatic refinement checker |
| U8 | Distributed execution, log merging, and operation under partition are not addressed |
| U9 | A nested organisation acting *as a member* of an outer institution (inner verdicts becoming outer evidence) is not built; nesting is tested for behavioural isomorphism only |
| U10 | Uncertainty is ordinal (method ranks). No probabilities, no quantitative confidence |
| U11 | Confidentiality: boundaries stop units *listening*, but the log is global and readable |
| U12 | Dynamic restructuring: members and units are fixed at genesis / build time. No admission, expulsion or re-wiring at run time |
| U13 | Goals and requirements exist only as propositions and commitment consequents; intentions are not modelled |
| U14 | Propositions are opaque names: no entailment between them except explicit `derived_from`, and no time-indexed facts (redoing a task needs a new proposition) |
| U15 | Rules of non-retroactivity are per family and coarse; evidence-rule changes are always retroactive |

## 6. Negative results and counterexamples (executable)

| Id | Result | Exact assumptions | Test |
|---|---|---|---|
| N1 | Composition that runs internal feedback to completion is **not associative** | order-sensitive listener; FIFO feedback | `test_run_to_completion_composition_is_not_associative` |
| N2 | **Liveness is not compositional**: two units that each respond when asked deadlock when each waits for the other | circular assume–guarantee, no fairness or initial move | `test_liveness_is_not_compositional` |
| N3 | **Use and revocation of one capability do not commute**; their order is itself a fact that must be agreed | same capability | `test_dependent_stimuli_do_not` |
| N4 | **No self-certification**, and the rule cannot be amended away | `self_support` entrenched | `test_no_self_certification` |
| N5 | **Equal evidence stays contested**; the system does not decide, and the debtor bears the doubt at the deadline | equal ranks | `test_equal_evidence_stays_contested_no_forced_decision` |
| N6 | **Late evidence does not cure a missed deadline** | evidence time > deadline | `test_late_evidence_does_not_cure_a_missed_deadline` |
| N7 | **Effects are irreversible; only judgments are revisable.** A refund paid on a verdict stays paid when the verdict is withdrawn | — | `test_retracting_the_refutation_reinstates…` |
| N8 | **Rules can have gaps the system cannot see.** Under v1, when the service lies, nobody is at fault for the agent's false report | rule set v1 | `test_rule_change_closes_a_responsibility_gap` |
| N9 | **A passed test is not correctness**: a real passing test run is outranked by an observation of production | ranks test < probe | `test_evidence_is_bound_to_the_artifact_it_was_about` |
| N10 | **Regressing obligations** and **rules without a unique model** are rejected before enactment | — | `TestGovernanceCounterexamples` |
| N11 | **Judging mid-cascade produces spurious verdicts.** Found during development: a liability was created and voided within one cascade. Fixed by `settle`; see U4 for what remains | — | history; `test_rule_change_closes…` asserts two verdicts, not three |
| N12 | **Global state grows as 3ⁿ** for n independent commitments even in the smallest instance | measurements §5 | `bench/measure.py` |
| N13 | **Adjudication cost is superlinear in family size** (naive Datalog joins): 1 ms at depth 5, ~30 ms at depth 100 | measurements §4 | `bench/measure.py` |

Known theorems that bound what is achievable, cited and **not** re-proved here: Rice's theorem (no non-trivial
behavioural property of arbitrary unit code is decidable — hence rules are restricted to stratified Datalog);
the CAP theorem (under partition one cannot have both availability and the single order A-ORDER requires);
and Gödel/Löb-style limits (the system offers no argument for the soundness of its own trusted base, A-TCB).

## 7. Classification of properties

| Property | Preserved by composition | Statically checked | Enforced at run time | Observable only | Rests on external trust | Not established |
|---|---|---|---|---|---|---|
| Unit-local safety invariants | yes (T1) | per unit | | | A-UNIT | |
| Refinement / substitution of a part | yes (T2, T3) | relation supplied by hand | | | | automation (U7) |
| One owner per institutional event kind | yes | at composition time | | | | |
| No amplification of authority | yes (T6; P5) | | at delegation | | A-GENESIS | |
| Revocation and budgets | | | yes, relative to log order | | A-ORDER | under partition (U8) |
| Resource conservation | yes (T7; P5) | | by the ledger | | | |
| Log integrity | | | | tampering is detected, not prevented | A-HASH | |
| Fulfilment of commitments | | | | violations are detected, not prevented | A-CLOCK | |
| Truth of a claim | | | | | A-TRUST, A-ADAPTER | never established |
| Uniqueness of an assessment | n/a | by construction (grounded) | | | | |
| Verdict follows from the rules | n/a | unique model (stratified) | | | A-RULES | that the rules are just |
| Well-formed rule set | n/a | at every amendment | | | | |
| Confluence | for independent events (T5; M2) | | | | | in general (U5) |
| Termination of a cascade | | | fuel bound only | | | U2 |
| Liveness | **no** (N2) | | | | | U6 |
| Confidentiality | | | | | | U11 |

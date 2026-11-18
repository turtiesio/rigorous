# Status ledger

GENERATED from the records in `claims/src/book/` by `cargo run --bin claims -- --write`. Do not edit:
a test fails if this file differs from what the records say, or if a record cites a test or a theorem
that does not exist.

Every claim this repository makes, with how it is supported. The kinds of support are kept strictly apart:

- **Proved** — machine-checked in Lean 4 (`proofs/Rigor.lean`). These are theorems about the *mathematical
  model* in `02-semantics.md`. They say nothing directly about the Rust code.
- **Compiler-enforced** — the Rust type checker or a denied lint rejects the program. Holds for every
  execution, with no test needed.
- **Model-checked** — exhaustive exploration of a *finite instance* to a fixed point.
- **Tested** — seeded randomised or scenario tests. Evidence, not proof.
- **Measured** — run on external data; reproducible, but not part of the test suite.
- **Assumed** — taken on trust from outside the system.
- **Unresolved** — not established here.

Section 7 places each property in the six classes: preserved by composition, statically checked, enforced
at run time, observable only, resting on external trust, not established.

Run `cargo test`, `cargo clippy --all-targets` and `lean proofs/Rigor.lean` — or `cargo run --bin steward`,
which runs all of them and records the evidence.

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
| T8 | Refinement is preserved by the scheduler: two units related by a simulation produce the same log from the same stimuli, with every output fed back as a later input | `sim_drive` | propext |
| T9 | Under the scheduler, the bracketing of a composition cannot be observed in the log | `drive_assoc` | propext |

**Gap between model and code.** The Lean model has binary `par` over a single event type and a FIFO
scheduler with feedback (T8, T9). It still omits `settle`, the ordering of simultaneous outputs, `Entity`,
kinds, and all the institutional units. The Rust code
is connected to these theorems only by the tests below. That correspondence is **unresolved** (U1).

## 1a. Compiler-enforced

Each line is something that cannot be written, not something that is checked when it happens.

| Id | What cannot be expressed | How |
|---|---|---|
| C1 | An event of an unknown kind; a unit interface naming a kind that does not exist | `Event` and `Kind` are closed enums; interfaces are `BTreeSet<Kind>` |
| C2 | Confusing a commitment id, evidence id, capability id, proposal id, actor, resource, amount, time or version with one another | distinct newtypes; doctest `compile_fail` |
| C3 | An invalid name literal | `Name::lit` is evaluated at compile time; doctest `compile_fail` |
| C4 | Committing another actor | `Offer` has no debtor field; the debtor is the kernel-stamped actor; doctest `compile_fail` |
| C5 | A unit claiming to be someone else | events carry no actor field; the kernel assigns it from the emitter's position |
| C6 | Evidence that bears on both a proposition and another piece of evidence, or on neither | `About` is an enum with exactly three cases |
| C7 | A template that builds a proposition or a party by string substitution | `PartyExpr`, `ConsExpr` are enums; propositions are structured (`Prop::Remediated(CommitmentId)`) |
| C8 | An amendment to a path that does not exist | `Amendment` and `RulePath` are closed enums |
| C9 | A refusal without an enumerated reason | `DenyReason`, `NormError`, `EvidenceError`, `GovError`, `RulesError`, `Failure` |
| C10 | A unit reading or writing another unit's state | each unit owns its state; no shared references; `unsafe` is forbidden |
| C11 | Library code that can panic by `unwrap`, `expect`, `panic!`, indexing, or lose data by an `as` cast | denied lints (`Cargo.toml`); `cargo clippy --all-targets` is clean |
| C12 | Non-deterministic iteration order inside a unit | only ordered maps and sets are used (no `HashMap`); checked by inspection and by replay tests, not by a lint |

Not compiler-enforced, and checked instead when rules are proposed: a responsibility rule that is ill-sorted
(arguments of the wrong kind), unsafe, or unstratifiable — because rules are run-time data
(`rules_without_a_unique_meaning_are_rejected`).

Where strings remain, and why they are not "stringly": the free text of `Content::Note` and the program name
in `Content::Command` (documentation that nothing interprets); the diagnostic text of a JSON encoding or
parsing failure; the statements and notes of the claim records (prose for people; what a record cites is
checked to exist); and the JSON serialisation boundary itself, where an unknown variant or invalid name is
rejected on the way in.

## 2. Model-checked (finite instances)

| Id | Statement | Instance | Test |
|---|---|---|---|
| M1 | Absorbing statuses are never left; `satisfied` is undone only by reassessment or release; nothing is pending after its deadline; a debtor cannot release itself | one commitment; 12 stimuli; explored to a fixed point | `model_check_single_commitment` |
| M2 | Mutually independent stimuli reach one state under every ordering | 5 stimuli (120 orders) in tests; 6 (720) in the measurements | `independent_stimuli_commute` |
| M3 | The explorer reports a shortest counterexample for a false invariant | seeded bug | `model_checker_finds_a_seeded_bug` |
| M4 | The ledger alone, whatever it is told and in whatever order: balances always sum to what was minted; a refused order moves nothing; every order is answered exactly once; only known accounts hold balances. Checked on the concept by itself, with no system assembled around it | 3 accounts (one never opened), 2 resources, 12 inputs including zero, more-than-held and the largest number; every reachable state | `the_laws_hold_in_every_reachable_state_of_a_small_world` |

## 3. Tested

| Id | Statement | Extent | Test |
|---|---|---|---|
| P1 | Any bracketing and ordering of the same units produces the identical log | 8 random bracketings of the full scenario | `par_is_associative_and_commutative` |
| P2 | An organisation nested 1 or 2 boundaries deep produces the standalone log up to prefix | full scenario | `entity_scoping_is_functorial` |
| P3 | A two-part service can be replaced by a monolith leaving every other unit's state identical | full scenario | `a_composite_can_be_substituted_by_an_atomic_unit` |
| P4 | Replay reproduces the log bit-for-bit; any edit to a stored event is detected | full scenario | `replay_is_deterministic`, `log_persistence_and_tamper_detection` |
| P5 | Under random delegate/transfer/revoke/tick attempts: attenuation holds, no chain overspends, revoked authority never acts, only the owner's line moves funds, credit is conserved | 25 seeds × 120 steps | `random_delegation_trees_never_amplify_authority` |
| P6 | Incremental (component-local) assessment equals from-scratch assessment after every event | 40 seeds × 60 steps | `incremental_assessment_equals_from_scratch` |
| P7 | The grounded labelling algorithm equals the textbook least fixed point | 300 random graphs ≤ 8 nodes | `grounded_matches_the_textbook_definition_on_random_graphs` |
| P8 | Work caused by one refutation does not depend on world size | N = 20 vs 200 exactly; timed to N = 5000 | `revision_work_is_independent_of_world_size`, measurements §3 |
| P9 | Judgments changed by a stimulus lie inside the static cone of the change | one point of one scenario (see P23 for the property test) | `exact_impact_lies_inside_the_static_cone` |
| P10 | The nine demonstrations of the brief | scenario and four variants | `app/tests/scenario.rs` |
| P11 | Evidence bound to a file digest goes stale when the file changes, using a real subprocess | one loop | `app/tests/adapters.rs` |
| P12 | Ill-sorted responsibility rules are rejected before enactment | three malformed rule sets | `rules_without_a_unique_meaning_are_rejected` |
| P13 | Members can be admitted at run time only by a charter-holder; an admitted member has the same standing as a founding one | constructed | `members_can_be_admitted_at_run_time_only_by_charter` |
| P14 | A maintenance commitment is violated exactly while its proposition stands refuted within the period, is discharged at the end otherwise, and is revised when the refutation is undone | constructed; plus exhaustive exploration of one commitment | `a_maintenance_commitment_is_about_not_being_refuted`, `model_check_maintenance_commitment` |
| P15 | A real git history built with real `git` (reverts, a revert of a revert, a self-revert, an automated committer) is ingested and the conclusions equal an independent oracle | 6 commits | `a_real_git_history_with_reverts_and_reverts_of_reverts` |
| P16 | The steward's record survives restarts, records unevidenced days as violations with findings, closes roadmap items only on the project's acceptance, and sets aside an edited log | constructed repository | `the_steward_keeps_an_honest_record_across_runs` |
| P17 | Judging does no work while nothing is violated, whatever the number of members | 600 members | `judging_costs_nothing_until_something_is_violated_whatever_the_membership` |
| P18 | **Real data**: 4,000 consecutive real commits (84 resolvable reverts, 6 reverts of reverts, 270 sponsored admissions) agree with the oracle and keep every invariant | fixture checked into the repository | `four_thousand_real_commits_agree_with_the_oracle_and_keep_every_invariant` |
| P19 | A world resumed from a snapshot file and continued, compacting as it goes, equals the uninterrupted one in state, chain head and conclusions; a corrupt or ill-fitting snapshot is refused | the same fixture | `a_snapshot_resumes_exactly_and_memory_need_not_grow_with_history` |
| P20 | Two real histories as nested organisations, interleaved commit by commit, each conclude exactly what they conclude alone | the same fixture, halved | `nested_organisations_on_real_data_behave_exactly_as_they_do_alone` |
| P21 | A log in an older format is verified from its stored bytes and migrated to the same judgments; an edited one is refused before interpretation | scenario log | `a_log_in_an_older_format_is_verified_from_its_bytes_and_migrated` |
| P22 | The reflector proposes lowering a method's rank once its record crosses the threshold, cannot approve its own proposal, cannot touch entrenched rules, and existing assessments are revised when a human approves | constructed | `the_reflector_proposes_a_rule_change_from_the_record_and_cannot_enact_it_alone` |
| P24 | **Real transfers**: 22 real Bitcoin blocks (938 transactions) settle with none wrongly refused, every replay and every off-purpose use refused, every budget spent exactly, balances equal to bookkeeping | fixture in the repository | `real_bitcoin_blocks_settle_with_every_budget_enforced` |
| P25 | A capability bound to a purpose can be used and delegated only for that purpose | constructed | `a_capability_can_be_bound_to_one_purpose` |
| P26 | Expulsion ends a member's power to act and revokes what they delegated, while their obligations remain; a role can be moved, and duties created afterwards fall on the new holder | constructed | `a_member_can_be_expelled_and_roles_reassigned_by_charter` |
| P27 | A snapshot is resumed only if an audit by replay is accepted evidence; a forged but self-consistent snapshot is caught by the audit | scenario | `a_snapshot_is_resumed_only_once_someone_trusted_has_shown_it_faithful` |
| P28 | Review trailers enter as evidence; the reflector proposes a maintenance period from observed refutation delays, and it takes effect only on approval | 1,500 real commits; constructed | `review_trailers_are_evidence_and_the_period_is_calibrated_from_the_record`, `the_reflector_learns_the_maintenance_period_from_how_late_refutations_arrive` |
| P29 | An event that changed shape between schemas is upgraded, not lost; changing the stored form of events without bumping the schema fails a test | constructed; fixed sample | `an_event_that_changed_shape_is_upgraded_not_lost`, `changing_the_vocabulary_without_bumping_the_schema_fails_here` |
| P30 | **A real package registry**: 3,679 real releases (282 yanked): the fit set equals an independent oracle; releases resting on a yanked one lose support; revision work stays local | fixture in the repository | `a_real_registry_with_yanks_agrees_with_the_oracle` |
| P31 | **Real forge reviews**: approvals support, standing change-requests rebut; objected changes are not accepted | 935 real commits, 52 real pull requests | `real_pull_request_reviews_enter_as_evidence_for_and_against` |
| P32 | **Two institutions interacting**: an outer duty becomes inner work; the inner result crosses only as a claim; inner evidence and identities do not cross; each institution judges at its own level | constructed | `federation::a_duty_laid_on_an_organisation_becomes_work_inside_it_and_its_report_is_only_a_claim`, `federation::each_institution_judges_at_its_own_level_when_the_inner_work_fails` |
| P33 | **It knows what it cannot do**: each ability's needs are probed for real; what is lacking is named; its account of itself is recorded as evidence and revised when the facts or the owner's decisions change; re-examining with nothing changed adds nothing | constructed repository, no credential | `it_knows_what_it_cannot_do_and_exactly_what_is_lacking` |
| P34 | **The model budget is enforced by the institution's own ledger**: nothing is spent before the owner grants a budget, nothing beyond it, and neither the service account nor a model can take or grant one | constructed | `the_model_budget_is_enforced_by_the_institutions_own_ledger` |
| P35 | **Controlled change**: applied only when verified by every check, approved by the owner and assessed sound; the proposer's word counts for nothing; a failed verification and the owner's rejection are never outweighed; what is applied is exactly what was verified; an edit applies only to the content it was written against and may not discard most of a file; code and proofs are out of reach | constructed repository, checks injected | four tests in `app/tests/office.rs` |
| P36 | A project founded under an older constitution is re-founded with its history, in the open | constructed | `a_project_founded_under_an_older_constitution_is_refounded_with_its_history` |
| P37 | The dashboard puts what needs the owner first and escapes outside text | constructed | `the_dashboard_puts_what_needs_the_owner_first` |
| P38 | **The runtime agrees with a transcription of the proved model**: 400 random triples of units driven by the real kernel and by a line-by-line transcription of the Lean `Unit`/`par`/`drive` give identical logs, including when cut off by fuel | random step tables | `the_runtime_agrees_with_a_transcription_of_the_proved_model` |
| P39 | **Real forge reviews at scale, with retraction**: 6,000 real happenings; superseded and dismissed reviews are withdrawn; every assessment equals the oracle | fixture in the repository | `real_forge_reviews_with_superseded_and_dismissed_reviews_agree_with_the_oracle` |
| P23 | Every existing judgment a stimulus changes lies inside the static cone of the change (closes U3) | 360 random cases, over 100 non-trivial | `exact_impact_always_lies_inside_the_static_cone` |
| P40 | **The code keeps its shape**: the waist depends on no other crate and stays within a line budget; a concept depends on the waist and the reasoners and on no other concept; the assembly does not reach the outside world; no file grows past its limit, and the files already too long may only shrink | every manifest and source file of the repository | `claims/tests/honest.rs` |
| P41 | **No claim outlives its witness**: every test and every theorem a record cites exists by that name; every claim a property rests on is in the book; identifiers are unique | all records | `everything_a_claim_cites_exists` |
| P42 | **The status document says what the records say**: it is generated from them, and a test fails if the file differs | `docs/03-status-ledger.md` | `the_status_document_says_exactly_what_the_records_say` |
| P43 | **A concept in its own words is a part of the assembled system**: the ledger, written against its own vocabulary in its own crate, replaces the unit that was written against the system's events, and every test and oracle agrees as before | the whole suite, including 938 real transactions | the suite; in particular the transfer and delegation tests |

## 3a. Measured on external data (reproducible with network access; not part of `cargo test`)

| Id | Statement | Extent | How |
|---|---|---|---|
| R1 | On three public git histories: the set of changes held refuted equals an independent oracle; incremental assessment equals from-scratch; every act was authorised; no capability amplifies its parent; the log verifies and replays bit-for-bit | 276,548 commits, 4,303,196 events, 13,109 members admitted (7,782 under delegated authority), 128,112 reviews, 1,185 reverts of which 44 revert a revert | `cargo run --release --bin stress`; `06-real-data.md` |
| R2 | A world resumed from a half-way snapshot and continued equals the uninterrupted one; resume takes under 1 s where replay takes 9–14 s | the same three histories | same |
| R3 | The rules revised from the record: each project's maintenance period was lengthened two or three times by reflector proposal and approval, differently per project; a final amendment re-assesses each record consistently | the same three histories | same |
| R4 | Three histories as separately bounded organisations in one world conclude what the oracle says; with compaction the run holds 32,287 of 4,303,196 entries and peaks at 1.28 GB | one hash chain, verified stretch by stretch | `stress --federate-only` |
| R5 | Real transfers: 3,101 real Bitcoin transactions settled with no wrongful refusal, 3,015 of 3,015 replays and off-purpose uses refused, every budget spent exactly, balances equal to bookkeeping | 86 blocks, 7,895 accounts | `stress --chain`; also P24 |
| R7 | A real package registry: of 12,844 releases never yanked, 308 are not held fit because they rest on a yanked release; the fit set equals an independent oracle | 397 crates, 13,754 releases, 910 yanks | `stress --registry`; also P30 |
| R8 | A week of a forge's public event archive: every reviewed change's assessment equals an independent oracle; nothing refused | 169 hourly files; 212,434 happenings; 68,522 people admitted; 12,676 approvals, 2,173 change requests, 1,581 dismissals, 743 reviews withdrawn; 15,419 reviewed changes; 1,976,364 events | `stress --forge`; also P39 |
| R9 | The system ran itself with two real models through a gateway: 6 research questions answered and accepted (4 only after a rejected first draft), 4 change requests proposed, of which the owner rejected 3 and 3 were applied; total spend about 2 cents of a 60-cent budget | 2026-11-18 | `cargo run --bin steward -- cycle`; `.rigor/`; `07-running-itself.md` |
| R6 | The steward, on this repository: caught and recorded a lint failure; re-ran only what had changed; migrated its own log across two schema changes, the second a real change of event shape; and holds six roadmap items as closed on the project's acceptance | 2026-11-18 | `cargo run --bin steward`; `.rigor/` |

**No longer reproducible.** An earlier Python prototype of the same semantics was kept alongside the Rust
code until the two were shown to agree on five scenario variants (commitments, statuses, assessments,
verdicts, rule version, event count) and on every shared measurement count. It has since been removed, so
that agreement cannot be re-run from this repository; it is recorded here as history, not as a current claim.

## 4. Assumed (the trusted-assumption inventory)

| Id | Assumption | What breaks if it is false |
|---|---|---|
| A-GENESIS | The genesis event (members, root capabilities, rules) is the intended constitution | Everything; it is the only trust anchor |
| A-AUTH | An external stimulus really comes from the actor it names | All authority checks |
| A-ORDER | One total order of events exists (a single log) | Revocation, budgets, deadlines: see N3 |
| A-CLOCK | `tick` events are monotone and reflect real time | Deadlines, expiry |
| A-HASH | SHA-256 is collision resistant; note ids are **truncated** to 64 bits | Log integrity; identity of commitments/evidence at large scale |
| A-TRUST | The `trusted` table names sources that report honestly for each method | Every assessment |
| A-ADAPTER | Adapters report what they observed (file digests, exit codes, commit metadata), and the mapping they apply means what it claims (a revert is a refutation; a distinct committer is an acknowledgement) | Conformance evidence; every conclusion drawn from real data |
| A-UNIT | Each unit's transition function is deterministic and touches only its own state | Replay, composition laws. State-locality is compiler-enforced (C10); purity is not: a unit could still read a clock or a file |
| A-RULES | The rule set expresses what the institution means | Verdicts are sound for the rules, not for justice |
| A-OBS | "Notified" means a message was delivered, not that anyone understood it | The `knew` finding |
| A-MODEL | Language models are trusted for nothing: what one says becomes a claim or a proposal by the member that asked, and counts only if another trusted party acknowledges it. The reviewing model IS such a party for research answers and change reviews, so two models agreeing can be wrong together | Research answers; change reviews (not verification, not approval) |
| A-OWNER | Acts recorded as the owner's were made by the owner or on their instruction. In the first session they were made by an assistant acting for the owner | Budgets, decisions, approvals |
| A-SNAPSHOT | For a plain resume: the snapshot file is the state its log produces. For an attested resume: the auditor who replayed is trustworthy | Everything after a resume |
| A-TCB | The waist (budgeted at 1,500 lines, P40), the reasoners, the vocabulary, the concepts and their wiring are correct | Everything not independently proved |
| A-TOOLS | The Rust compiler, `serde`, `serde_json`, `sha2`, and the Lean kernel are correct | Everything marked compiler-enforced or proved |

## 5. Unresolved

| Id | Open item |
|---|---|
| U1 | No proof connects the Lean model to the Rust code. A differential test against a transcription of the model does (P38), for `Par` and the scheduler only |
| U2 | Termination of a cascade is not guaranteed for arbitrary units (only a fuel bound). Acyclic templates are *intended* to bound chains of secondary duties; that is not proved |
| U3 | Soundness of the static impact cone is property-tested (P23), not proved |
| U4 | Only the adjudicator waits for quiescence. Any other unit that reads several facts at once can still see a half-propagated state |
| U5 | Commutativity of `Par` is tested, not proved. There is no static independence analysis and no partial-order reduction; independence is demonstrated per case |
| U6 | No liveness property of any kind is established |
| U7 | Refinement relations must be supplied by hand; there is no automatic refinement checker |
| U8 | Distributed execution, log merging, and operation under partition are not addressed |
| U9 | One organisation acting as a member of another is built and tested (P32) on a constructed case only; on real data organisations have only run side by side |
| U10 | Uncertainty is ordinal (method ranks). No probabilities, no quantitative confidence |
| U11 | Confidentiality: boundaries stop units *listening*, but the log is global and readable |
| U12 | Members can be admitted, expelled and given roles at run time; units are still fixed at build time |
| U13 | Goals and requirements exist only as propositions and commitment consequents; intentions are not modelled |
| U14 | Propositions are opaque names: no entailment between them except explicit `derived_from`, and no time-indexed facts (redoing a task needs a new proposition) |
| U15 | Rules of non-retroactivity are per family and coarse; evidence-rule changes are always retroactive |
| U16 | The typed vocabulary is closed: a new kind of proposition, tag, method or predicate means editing an enum and recompiling. The kernel and reasoners are generic, so a second domain can bring its own event type, but two domains cannot yet be *combined* without a common enum. One concept (value) now states its own vocabulary and is wired in by translation (P43); the others still speak the shared one, and propositions are still one closed type |
| U18 | A snapshot can be audited and resumed only on accepted evidence (P27), but the audit is a full replay by a trusted auditor: cost is not avoided, only moved, and plain `resume` still exists |
| U19 | Migration replays old stimuli under today's units, so derived events may differ from the old log's; only the stimuli and the old chain head are carried over. Upgrade steps are written by hand for each change of shape; a guard test fails when the stored form of events changes without a schema bump, but nothing checks that a hand-written step preserves meaning |
| U20 | Backdated records get their period from arrival: a policy, not a model of event time versus record time |
| U21 | Identity merging is only what the project's own mailmap says |
| U22 | Three kinds of real record have been tried (version-control histories, a ledger window of 86 blocks, a registry of 397 packages). Each adapter is an interpretation, and each oracle was written by the same author |
| U24 | The system changes only research notes and generated tests. It cannot change its own code, proofs or rules under control, and there is deliberately no way to apply anything without the owner |
| U25 | The dashboard is a static page rewritten each run; it could not be rendered for inspection in this environment (the preview tool failed), so its appearance is unverified |
| U23 | The reflector's thresholds are fixed parameters and it knows two kinds of improvement (a method's rank; the maintenance period). It shows that rules can be revised from the record, not that the revision is right. In the stress runs the project's approval was automatic |
| U17 | In-memory state digests (used only by the model checker) rely on `std::hash`, which is not stable across compiler versions; they are never persisted or compared across builds |

## 6. Negative results and counterexamples (executable)

| Id | Result | Exact assumptions | Test |
|---|---|---|---|
| N1 | Composition that runs internal feedback to completion is **not associative** | order-sensitive listener; FIFO feedback | `run_to_completion_composition_is_not_associative` |
| N2 | **Liveness is not compositional**: two units that each respond when asked deadlock when each waits for the other | circular assume–guarantee, no fairness or initial move | `liveness_is_not_compositional` |
| N3 | **Use and revocation of one capability do not commute**; their order is itself a fact that must be agreed | same capability | `dependent_stimuli_do_not_commute` |
| N4 | **No self-certification**, and the rule cannot be amended away | `self_support` entrenched | `no_self_certification` |
| N5 | **Equal evidence stays contested**; the system does not decide, and the debtor bears the doubt at the deadline | equal ranks | `equal_evidence_stays_contested_no_forced_decision` |
| N6 | **Late evidence does not cure a missed deadline** | evidence time > deadline | `late_evidence_does_not_cure_a_missed_deadline` |
| N7 | **Effects are irreversible; only judgments are revisable.** A refund paid on a verdict stays paid when the verdict is withdrawn | — | `retracting_the_refutation_reinstates_everything_except_what_already_happened` |
| N8 | **Rules can have gaps the system cannot see.** Under v1, when the service lies, nobody is at fault for the agent's false report | rule set v1 | `rule_change_closes_a_responsibility_gap` |
| N9 | **A passed test is not correctness**: a real passing test run is outranked by an observation of production | ranks test < probe | `evidence_is_bound_to_the_artifact_it_was_about` |
| N10 | **Regressing obligations** and **rules without a unique model** are rejected before enactment | — | `obligation_regress_is_rejected_statically`, `rules_without_a_unique_meaning_are_rejected` |
| N11 | **Judging mid-cascade produces spurious verdicts.** Found during development: a liability was created and voided within one cascade. Fixed by `settle`; see U4 for what remains | — | `main_scenario` asserts one verdict per cascade |
| N12 | **Global state grows as 3ⁿ** for n independent commitments even in the smallest instance | measurements §5 | `measure` |
| N13 | **Adjudication cost is superlinear in family size** (naive Datalog joins): 0.2 ms at depth 5, about 4 ms at depth 100 | measurements §4 | `measure` |
| N15 | **Judging every family on every cascade does not scale.** Found on real data: throughput fell 20-fold over 61,537 commits because each judgment enumerated every member. Fixed; pinned by P17 | first stress run | `06-real-data.md` |
| N16 | **A version-control history does not contain the evidence of review.** 5% of Kubernetes commits show an independent acknowledgement in git metadata | git metadata only | `06-real-data.md` |
| N17 | **Real records are not well-formed.** 11% of declared reverts cannot be resolved to one earlier commit; 578 commits have clocks behind ones already seen | three histories | `06-real-data.md` |
| N19 | **Revising evidence over the whole connected component does not scale** when derivation connects everything: 297 s for a real registry of 13,754 releases. Revision is now downstream-local (1 s); the from-scratch reference is kept and still agrees | real registry | `06-real-data.md` |
| N20 | **A reviewer model and the checks both passed a destructive edit.** A model asked to revise a document returned only the changed paragraph; the edit would have replaced the file with one line. Caught by the owner's inspection; now refused structurally | first day of real use | `07-running-itself.md` |
| N21 | **Two models agreeing is not evidence of correctness.** The research reviewer accepted an answer whose change request the same model, asked as a change reviewer, then rejected for over-claiming | first day of real use | `docs/research/README.md` |
| N22 | **The project's checks do not cover what its documents say.** Every documentation change passes verification trivially. Since then the status ledger is generated from checked records (P41, P42); the other documents are still uncovered | by construction | on the roadmap |
| N18 | **A vocabulary change silently orphaned a stored log.** Adding fields to events made the steward's own log unparseable; it set the log aside and began a new one, losing its record of closed items. Found on the next run. Fixed by bumping the schema, writing the upgrade step, recovering the log by migration, and adding the guard test (P29) | this repository, 2026-11-18 | `.rigor/` |
| N14 | **Nobody is outside the model.** When the service works, the scripted Alice never pays what she promised, and the same rules find her accountable and liable. Not a bug | scenario, working service | `working_service_everything_is_satisfied` |

Known theorems that bound what is achievable, cited and **not** re-proved here: Rice's theorem (no non-trivial
behavioural property of arbitrary unit code is decidable — hence rules are restricted to stratified Datalog);
the CAP theorem (under partition one cannot have both availability and the single order A-ORDER requires);
and Gödel/Löb-style limits (the system offers no argument for the soundness of its own trusted base, A-TCB).

## 7. Classification of properties

| Property | Preserved by composition | Statically checked | Enforced at run time | Observable only | Rests on external trust | Not established |
|---|---|---|---|---|---|---|
| Unit-local safety invariants | yes (T1) | per unit |  |  | A-UNIT |  |
| Refinement / substitution of a part | yes (T2, T3) | relation supplied by hand |  |  |  | automation (U7) |
| One owner per institutional event kind | yes | at composition time |  |  |  |  |
| Well-formedness of events, acts, ids, templates, amendments | yes | **by the Rust compiler** (C1–C9) | at the JSON boundary |  | A-TOOLS |  |
| State-locality of units | yes | **by the Rust compiler** (C10) |  |  |  | purity (A-UNIT) |
| No amplification of authority | yes (T6; P5) |  | at delegation |  | A-GENESIS |  |
| Revocation and budgets |  |  | yes, relative to log order |  | A-ORDER | under partition (U8) |
| Resource conservation | yes (T7; P5) | model-checked on the ledger alone (M4) | by the ledger |  |  |  |
| Log integrity |  |  |  | tampering is detected, not prevented | A-HASH |  |
| Fulfilment of commitments |  |  |  | violations are detected, not prevented | A-CLOCK |  |
| Truth of a claim |  |  |  |  | A-TRUST, A-ADAPTER | never established |
| Uniqueness of an assessment | n/a | by construction (grounded) |  |  |  |  |
| Verdict follows from the rules | n/a | unique model (stratified) |  |  | A-RULES | that the rules are just |
| Well-formed rule set | n/a | at every amendment |  |  |  |  |
| Confluence | for independent events (T5; M2) |  |  |  |  | in general (U5) |
| Termination of a cascade |  |  | fuel bound only |  |  | U2 |
| Liveness | **no** (N2) |  |  |  |  | U6 |
| Confidentiality |  |  |  |  |  | U11 |

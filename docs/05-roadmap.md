# Roadmap

What the system lacks, sorted by what to do about it. Ids refer to `03-status-ledger.md`. The list was
re-derived after stressing the system with real data (`06-real-data.md`); several priorities moved because
of what that run broke.

The six items under **Address next** are not only written here: they are commitments in the steward's own
world, each with a review-by day (`crates/rigor/src/steward.rs`, shown by `cargo run --bin steward`). An item
is closed only when the project accepts it; an item that passes its day unresolved becomes a violated
commitment on the record.

## Done since the real-data stress began

| What | Why it was needed | Evidence |
|---|---|---|
| Members admitted at run time, by a delegable charter | A real history introduces thousands of people continuously | `members_can_be_admitted_at_run_time_only_by_charter`; 12,866 real admissions, 7,803 under delegated authority |
| Maintenance obligations ("must not come to be refuted until …") | Real undertakings are often about keeping something true | `a_maintenance_commitment_is_about_not_being_refuted`, `model_check_maintenance_commitment` |
| Judging cost independent of membership | Throughput collapsed 20-fold on the first real run | `06-real-data.md`; pinned by a test |
| **Snapshots, resume, compaction** (was item 1) | Restart cost and memory grew with history | `a_snapshot_resumes_exactly_and_memory_need_not_grow_with_history`; resumed in under 1 s on all three histories |
| **Log schema migration** (was item 2) | The steward's own log could not be read after a format change | `a_log_in_an_older_format_is_verified_from_its_bytes_and_migrated`; the steward migrated its real log |
| **Impact-cone soundness, property-tested** (was item 3) | The bound was checked at one point | `exact_impact_always_lies_inside_the_static_cone`: 360 random cases |
| Policy for backdated records; mailmap identities; merges ingested | 10 refused commitments; 11% unresolved reverts; duplicate members | `messy_history_text_is_typed_once_at_the_boundary`; unresolved reverts now 4% |
| Nesting, delegation and governance on real data | They were tested only on constructed scenarios | `06-real-data.md`; `nested_organisations_on_real_data_behave_exactly_as_they_do_alone` |
| Real data inside `cargo test` | The stress figures were not reproducible from the repository alone | 4,000 real commits as a pseudonymous fixture; three tests use them |
| A reflector that proposes rule changes from the record | "Improves itself" had no mechanism | `the_reflector_proposes_a_rule_change_from_the_record_and_cannot_enact_it_alone` |
| The project stewards itself; a lapse creates a duty to put it right | Its claims about itself were asserted, not evidenced | `the_steward_keeps_an_honest_record_across_runs`; `steward` binary |

The three items marked "was item N" were closed through the steward itself (`steward accept <item>`): they
now stand in its log as satisfied commitments.

## Closed in the last round (through the steward)

| What | Evidence |
|---|---|
| Snapshots are attested, not trusted: an auditor replays, attests, and resume requires that evidence to be accepted | `a_snapshot_is_resumed_only_once_someone_trusted_has_shown_it_faithful` |
| Real data for transfers and budgets: 938 real Bitcoin transactions, none wrongly refused, every replay refused, balances equal to bookkeeping | `real_bitcoin_blocks_settle_with_every_budget_enforced`; `06-real-data.md` Part 2 |
| Expulsion and role change, by charter | `a_member_can_be_expelled_and_roles_reassigned_by_charter` |
| Review evidence from commit trailers (128,112 real reviews) | `review_trailers_are_evidence_and_the_period_is_calibrated_from_the_record` |
| The maintenance period is a rule the system revises from its record | `the_reflector_learns_the_maintenance_period_from_how_late_refutations_arrive`; `06-real-data.md` |
| Capabilities bound to one purpose | `a_capability_can_be_bound_to_one_purpose` |
| Memory bounded in long runs (4.7 GB → 1.28 GB for 4.3 million events) | `stress --federate-only` |

## Address next (ordered; each is a steward commitment with a review-by day)

| # | Item | What is wrong today | Done when | Review within |
|---|---|---|---|---|
| 1 | **Forge adapter** | Projects that record review in pull requests show no review evidence at all (Kubernetes: 0) | Reviews and check runs enter as evidence bound to commit digests | 60 days |
| 2 | **Organisations interacting across boundaries** (U9) | Nested organisations have only ever run side by side | One organisation acts as a member of another, with identities and evidence translated at the boundary | 75 days |
| 3 | **A second kind of real record** | Only version-control histories and a ledger have been tried | A real dataset of another kind (contracts, incidents, procurement) with an independent check | 75 days |
| 4 | **Open the vocabulary by type parameters** (U16) | Each new domain so far has added variants to shared enums | Units are generic in `Prop`/`Tag`/`Pred` | 90 days |
| 5 | **Verify the kernel against the Lean model** (U1) | Nothing proves the Rust kernel realises the model the theorems are about | T1–T5 proved of the Rust `Par`/`Runtime` | 180 days |

Items 4 and 5 are large and are deliberately not attempted piecemeal: a partial version of either would add
risk to the trusted base without removing an assumption.

## Deferred (worth doing; not blocking; each with the trigger that would promote it)

| Item | Why deferred | Promote when |
|---|---|---|
| Separate event time from record time | Backdated records now get a period from arrival, which is a policy, not a model of two clocks | A domain where late records are normal (accounting, incident reports) |
| Identity merging beyond the project's own mailmap | Affects attribution, not correctness of the machinery | The forge adapter lands (item 2), which supplies stable account ids |
| The remaining unresolvable references (4% of reverts) | Needs forge data | Item 2 |
| Re-wiring of units at run time | No case has needed it | A scenario needing it |
| Semi-naive, incremental Datalog (N13) | Families in real data are tiny; cost is superlinear only for deep delegation chains | Chains deeper than ~100 in practice |
| Partial-order reduction and declared footprints (U5) | The explorer is used on small instances only | Model checking a composite of more than ~6 units |
| Automatic refinement checking (U7) | Substitution is tested case by case | A second implementation of any unit |
| Termination ranking (U2), strata for all multi-fact units (U4) | No divergence or glitch observed since `settle`; fuel bound catches divergence | A unit other than the adjudicator that reads several facts at once |
| Purity sandbox for units (A-UNIT) | Units are written in-repo and reviewed | Units supplied by a third party |
| Time-indexed propositions and entailment (U14) | Worked around by fresh propositions | A domain with recurring obligations over the same fact |
| Federation: a nested organisation acting as a member of an outer one (U9) | Nesting is behaviourally correct; no use case yet | Two organisations sharing a process |

## Parked (deliberately not pursued now, with the reason)

| Item | Reason |
|---|---|
| Distribution, operation under partition, merging logs (U8) | A different problem with its own literature (consensus, CALM). N3 shows revocation and budgets need agreement on order; nothing here is a step towards that. Revisit only for a multi-organisation deployment |
| Quantitative or probabilistic evidence (U10) | Ordinal ranks were sufficient for everything tried, including the real data. No counterexample demands more, and the brief asks not to add concepts without one |
| Confidentiality (U11) | Requires a different log architecture (per-entity logs, selective disclosure). Out of scope until federation is |
| Liveness guarantees (U6) | Shown not to compose (N2). Deadlines turn the liveness properties that matter into safety properties, which is the approach taken |
| Modelling goals and intentions (U13) | Commitments and propositions have covered every scenario; no gap demonstrated |
| The system rewriting its own code | Out of scope. "Self-improving" here means three concrete things: it detects and records its own lapses and creates duties to put them right; it holds its roadmap to deadlines; and its reflector proposes changes to its own rules from the record, which take effect only through governance. It does not write code |

## Rule for changing this list

If an extension requires a change to `crates/rigor/src/kernel/`, record the counterexample that forced it in
`01-prior-art.md` §3 before making the change. The real-data work changed the kernel three times, none of
them a change of semantics: interfaces computed once (performance); state save/load, snapshots and
compaction (restart cost); and chain verification from stored bytes (migration).

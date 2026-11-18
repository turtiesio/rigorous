# Draft 11 adversarial review — Astra, round 10

One confirmed false documentary statement; no new wrong model outcome found within the stated scope. This is a bounded search, not a proof. The new counterfactual construction preserves a dependence on forbidden intervals when reading the actual history, although it correctly calls InstAL to decide counterfactual attempts.

Reproduce with `python3 -B -u /root/mic/model/attack_contract_astra_r10.py`. The [script](/root/mic/model/attack_contract_astra_r10.py) has separately selectable `suites`, `attack`, `mutations`, `record`, and `regression` phases. Mutants are supplied by intercepting source reads in memory. Existing files were not edited; only the two requested review files were created, apart from the compiler's ordinary temporary workspace. No subagents were started. The other round-10 report was not read.

## A. Confirmed wrong outcomes or false statements inside scope

### A1 — MEDIUM — K8: “It holds no interval” is false of the new verdict

**Material:** written after round 9. **Kind:** false documentary statement, not a demonstrated wrong allocation. One root defect: the description omits the interval-based actual-history reader. The matching sentence in the module docstring is the same defect, not an additional item.

**Runnable scenario:** `attack`, specifically its `INTERVALS` and `GATE_CALLS` probe. Two units, budgets 2 and 1, work 3 and 1, no after-time. The second unit's authority bans delivery from input arrival until two days later. Instrument `gated.gate` in memory, leaving its returned results unchanged, and call `norm_rerun.verdict`.

The actual input to the second unit arrives at 3, its request at 4 is refused, and it emits at 5. The verdict constructs a `gated.Unit` holding `closed=[(3,5)]`. `first_refusal`, `counted_refusal`, `work`, `after`, and `late` consult this interval through `gate`. These are used to choose players and each coalition's moves. This is actual execution, not an inference merely from an unused import.

Copied output from the real run:

```text
INTERVALS [('u0', [], Fraction(3, 1), Fraction(0, 1), True), ('u1', [(Fraction(3, 1), Fraction(5, 1))], Fraction(1, 1), Fraction(0, 1), False)]
GATE_CALLS 27 NONEMPTY 27
VERDICT {'loss': Fraction(2, 1), 'under_prohibition': Fraction(1, 1), 'shares': {'u0': Fraction(1, 1)}}
```

The returned split is correct: compliance moves the downstream input from 3 to 2, its ban from [3,5) to [2,4), and final receipt from 5 to 4. Against total budget 3, one day remains under prohibition and one is attributable to u0. The false outcome here is the document's description of the implementation, not these amounts.

**Correct description and minimal repair:** “Counterfactual delivery decisions are made by InstAL rather than by replaying fixed intervals. The actual trace's forbidden intervals are still read to recover work, after-time, and lateness through `gated.Unit`.” Narrow the module docstring too. There is no need to change allocation code to repair this statement. If an interval-free implementation is intended, replace the actual-history reader as well and test it independently.

The narrower claim that no prohibition/exemption rule is reimplemented in the counterfactual evaluator survives. Looking up a fact exported by InstAL is not independently deriving that fact. Thus A1 does not by itself establish stop condition 3, nor warrant HIGH or CRITICAL severity.

## B. Design objections and evidence limits

1. **Authority policy is external input, but still a causal assumption.** `policy(u, ready)` is supplied by the caller. It constructs issue/withdraw events; it does not itself decide permission, exemption, conflict or emission. Calling it a model of authorities supplied from outside both layers is accurate for this API. Its counterfactual truth is not certified by InstAL or by the actual log. Different policies can fit the same observed history. The document now acknowledges this dependency, so it is not another A item.
2. **Scope matters to the retry controller.** With one ban, duty permission issued at readiness, and one zero-cost delivery effect, the first refusal can only be inside the ban. The next relevant authority act is withdrawal. Hence `later[0] + after` agrees with the stated convention here. A refusal before the ban is issued needs another cause, such as missing permission or funds; the supplied scenario constructor does not represent that case. Extending the policy to arbitrary authority actions would require revisiting retry scheduling and work inference. The function's general-sounding docstring is not evidence of support for such an extension.
3. **Reproduction assertion is weaker than full-history reproduction.** The last assertion compares clipped final losses, not all intermediate receipts, refusals or events. Equal zero losses could hide different final receipts. This is a test-quality objection: the independent checks here compare intermediate data too, and no such failure was found in the supported policy family.
4. **Caching and cost.** The cache keys include the unit sequence and all sorted, timed event entries. Changing ban times changes the key; repeating an identical history reuses it. The cache returns mutable traces and assumes the fixed scenario/compiler configuration does not change during the process. No stale result was found under ordinary calls. “One solver run per attempt” is a cost model/upper bound, not an exact count of invocations with caching. Unique coalitions double, but the Shapley implementation enumerates permutations and repeats coalition evaluation; that extra Python work is not captured by the solver count.
5. **Evidence remains finite.** The 12 contingent random chains in the maintained test check conservation and player selection largely by construction. This review adds an independent arithmetic oracle for the limited policy, but does not prove arbitrary policy composition. The original two-way random corpus is thin; the documented 3 excused units and 3 deferred deadlines are honest qualifications.
6. **Historical residual objections remain.** Actual log completeness is one-sided; no general online deadline scheduler exists. Fixed-later duty issuance remains an ambiguity in the broader earlier interface, although this new constructor explicitly issues at readiness. The existing qualifications about conventions, witness order and formal proof are not repaired by running more examples.

## C. Stated limits, not new findings

- **K4 unreachable amendment:** the regression rerun prints `occurred(refuse(step),unit)`, `occurred(refuse(finish),unit)`, then `occurred(missed(o),unit)`. This is the explicitly open wrong outcome; not A.
- **K1:** one shared sequence; differing clocks/per-unit orders are untested. Delivery at its deadline must precede the deadline. The code does not enforce that whole convention. These disclosed restrictions are not promoted into findings.
- **K8:** work inferred from attempts, removal of after-time on compliance, refusal-based excuse, and residual loss assigned under prohibition are conventions. Premature/artificial attempts, absent refusal reasons, funds, two duties per hand-over, no attempt while waiting, trees, repeated/overlapping bans and same-time withdraw/reissue remain outside the demonstrated construction. The new norm rerun covers one way and input-relative or fixed one-ban schedules; it does not establish two-way or arbitrary-policy support. Deadline placement from completed histories, policy provenance, and who ultimately bears prohibition loss remain open.
- **K12:** directly observed `relayed(p)` still bypasses a blocked upstream relay, and directly observed `relaywithdrawn(p)` can lift the prohibition. The route regression reproduces both. Passing on is a static initial fact; selective withdrawal, changing consent, rank asymmetry, vocabulary translation and leased context remain stated limits or objections.
- **K0/K2 and other clauses:** adapter trust and admitted coverage gaps, the partial stratification argument rather than a compiled-program proof, assumed witness agreement, creation during execution, and distributed/repeated ledger grants remain unresolved. This review does not silently treat them as guaranteed features.

## Execution and boundary attacks

The independent oracle computes each unit's attempt, refusal and receipt directly from one interval generated from the candidate readiness, then calculates coalition losses and Shapley shares independently. It does not use `gated.Unit`, `gated.gate` or `gated.rerun` to produce expectations. Actual work, after-time and player sets are separately checked against the resulting trace reader.

The corpus includes ban start exactly at the attempt, withdrawal exactly at the attempt, already-ended bans, work finishing before a future ban, positive after-time, zero work, zero budgets, and a zero-duration ban. Each hand-selected boundary case is run with both fixed and input-relative timing. Seeded additional chains have two or three units and ties between upstream delivery, downstream duty issue, downstream ban issue and requests. Every coalition is sent through `play`, which calls the real InstAL compiler and solver. Fixed-timetable verdicts are additionally compared with `gated.verdict` using a canonical log.

Copied final execution output:

```text
ORACLE 66 chains 156 coalitions; all receipts, refusals, work, after, late and shares agree
CACHE 2 2 2
CACHE 4 4 4
CACHE 2 2 4
MUTANT UNW2-own detected True AssertionError:
MUTANT UNW3-exemption detected True AssertionError:
MUTANT UNW4-watched detected True AssertionError:
ANSWER_SETS 896 [1]
```

There are 33 fixed and 33 input-relative chains in the independent corpus. All eleven maintained commands passed. The separate `regression` invocation finished with `ANSWER_SETS 7 [1]`: 903 successful solver calls across these executions, including mutated-rule runs. The final script's default invocation includes that regression phase too. The failed mutant assertions are expected detections, not unmodified-model failures.

The cache probe changes a one-unit ban end from 2 to 4, then repeats 2. Correct receipts are 2, 4, 2; the repeat adds no solver call. This tests both history distinction and ordinary cache reuse, without mutating cached outputs.

All ten previously maintained commands, plus the new `test_contingent.py`, are run. The route regression also reruns three-hop routes with each intermediate declining, inherited routes, prohibited route shapes, and the disclosed injection limit. Every successful solver call is checked for one answer set; rejection scenarios are not counted as solver nonuniqueness.

### Mutation checks

The exact three wrong rule forms from round 9 are applied to `unit.ial` only in memory:

| Mutant | Wrong meaning | Result |
|---|---|---|
| UNW2 | Use `forbidden` instead of `forbiddenabove`: own prohibition counts | New own-prohibition assertion fails |
| UNW3 | Derive forbidden matching authority prohibition without exemption check | New exemption assertion fails |
| UNW4 | Use any in-force authority prohibition, including watched | New watched-prohibition assertion fails |

The authority-enforced positive control passes unmutated. The four added cases therefore fix the reported coverage gap, rather than merely increasing the scenario count.

## Round-9 status

| Round-9 item / objection | Status in draft 11 | Basis |
|---|---|---|
| Fable A1: three rounds of agreement when the first had a disagreement | Fixed | Stop condition 5 now names rounds 7–9 and explicitly identifies round 6's failure |
| Astra list A | No item to fix | Round 9 count was zero |
| Astra main objection: failure of fixed intervals does not prove every interface impossible | Fixed in text; supported by the new construction | K8 distinguishes the failed interface from general impossibility; the two counterexamples now get the stated splits through InstAL |
| Fable contingent-ban counterexample | Fixed for the new evaluator; still a stated limit of the old evaluator | Both named cases pass, while fixed intervals retain their documented wrong splits outside their scope |
| Three undetected `noway` mutants | Fixed and pinned | All three fail the new maintained assertions under in-memory mutation |
| `noway` in another unit's institution | Moved to an explicitly stated limit; not changed in code | “A wrinkle” restricts the meaningful projection to the owing unit |
| Thin two-way corpus | Acknowledged, not strengthened by these four semantic probes | 3 excused, 3 deferred, one interval/conflict difference are stated |
| Fixed-later duty issue / deadline scheduling / incomplete log interface | Still objections or stated limits | No general repair claimed by the new one-way controller |

No round-9 list-A item remains unfixed. No item was reclassified to force the round-10 count upward or downward.

## Documentary and numerical audit

| Draft-11 number or assertion | Verification |
|---|---|
| Two named contingent cases | Run: shares/upstream and prohibition loss are (1,1) and (1,3); fixed versions are (0,2) and (4,0) |
| 12 fixed and 12 contingent random chains | Run of `test_contingent.py`; fixed comparisons pass, 6 contingent cases differ or are refused by fixed intervals |
| 175 solver runs | Exact clean-cache maintained-suite output |
| 99 and 287 coalition histories; 161 counterfactual | Astra and Fable round-9 reports, respectively; 161 belongs to Fable's 287 |
| 386 | 99 + 287; historical independent two-way histories, not this round's new runs |
| 556 | 155 + 401 in rounds 7 and 8, as documented in both round-9 audits |
| 192 tie-heavy chains | Fable round-9 TIE-ENUM record: 1128 solver calls, no disagreement |
| 1752 | Fable round-9 successful-run record, each with one answer set; not a claim of 1752 new calls here |
| 3 excused units, 3 deferred deadlines | Fable round-9 DOC8/DOC9: `STAT {'units': 56, 'late': 26, 'excused': 3, 'deferred': 3, ...}`; the four added cases do not change that random generator |
| 74 scenarios / 25 rejections | Maintained suite output and prior audit; 16 non-inertial fluents and 17 dependencies reproduced |
| 61/51 one-way, 63/37 two-way coalition/changed counts | Maintained suite output |
| Historical finding sequences | Reports give Fable 21,15,6,5,6,3,3,1,1 and Astra 8,6,3,2,2,1,2,3,0 |
| First jointly model-error-free round and nothing above LOW | Round-9 reports support it; round 8 still had Astra's HIGH model finding |
| Wrong model outcomes in rounds 7,8,9: 1,1,0 | Reports support the distinction from documentary findings |
| New draft-9 correction table | All three rows accurately describe the earlier issue and the response, subject to A1's separate overstatement about the new reader |

The new contingent-failure paragraph accurately states both counterexamples and their limits. “No fluent of the actual trace” should be read as no fixed realized interval supplies the counterfactual schedule; it is not an impossibility theorem about an interface exporting a conditional model. That is also what the following paragraph says. The status warning that the new material was unreviewed was true when written; this report supplies the next review.

## Stop conditions

1. **No trace or multiple traces:** not met in these runs. K2's proof sketch remains only a sketch.
2. **Repair requires changed InstAL semantics:** not met. A1 needs a wording repair, not a compiler change.
3. **K8 requires restating norms inside the verdict:** not established. Fixed actual intervals fail for contingent schedules; solver-backed counterfactual evaluation gives the correct tested results. The policy is an external causal input, and the actual reader still uses intervals (A1), but neither fact proves prohibition/exemption rules must be duplicated in the verdict. The document's narrow “not met on these cases” verdict follows; a general guarantee would not.
4. **K11 cannot be made by events alone:** not met in the maintained sequential ledger tests; broader grant semantics remain open.
5. **Two consecutive rounds with neither count falling:** not met through round 9. At least one fell in rounds 6, 8 and 9, so rounds 5 and 7 do not form a consecutive pair. This review's count rises from Astra's 0 to 1; the independent other round-10 result was not consulted. Even if neither falls in round 10, round 9 was a falling round. Scope changes still prevent interpreting raw counts as controlled convergence evidence.

List A by age: **1 new, 0 older**. By kind: **0 wrong model outcomes, 1 false documentary statement**. Variants and repeated wording are not counted separately.

CONFIRMED WRONG OUTCOMES (list A): 1
OF WHICH IN NEW MATERIAL: 1
CRITICAL: 0  HIGH: 0  MEDIUM: 1  LOW: 0

# Draft 6 adversarial review — Astra

Reproduce with `python3 -B -u /root/mic/model/attack_contract_astra_r6.py`. The [script](/root/mic/model/attack_contract_astra_r6.py) runs the nine maintained checks, ordered timed histories through the real compiler and solver, intermediate coalitions, validation probes, route probes, and mutations in memory. It uses earlier scenario helpers and reads earlier mutation definitions as data; it does not run an earlier tester's whole script. No existing model or report was edited. The supported harness creates its ordinary compiler scratch files. I did not read Fable's round-6 report or start another agent.

**One independent list-A defect, HIGH.** Two manifestations of lost event order are counted once. Astra's count falls from two to one, so stop condition 5 is **not met this round**, independently of the other tester's count. This is a bounded review, not a proof of correctness.

The defect is classified as **material that existed in round 5**: reducing ordered trace positions to scalar times and testing only authority-before-request ties were already present. The new refusal filter inherits that information loss. Its new wording and the rejected-receipt manifestation are not additional root defects. Numeric-representation and malformed-input objections remain in B, as in Astra rounds 4 and 5; disclosed wrong outcomes remain in C.

## A. Confirmed wrong outcomes

### A1 — HIGH — K1/K8: equal timestamps erase whether the authority had acted yet

Runnable scenario: `ordered_ties()` in the script. One unit, one duty, one delivery effect, one authority prohibition, budget 1, ready at 0. Authorities act independently of the unit. The effect costs 1. The certified log is:

| Certified time | Ordered event | Result |
|---|---|---|
| 0 | `issue(o)` | Duty in force |
| 1 | `request(e)` | Refused for want of funds; no authority prohibition yet |
| 1 | `issue(p)` | Authority prohibition begins **after** the refusal in the same log |
| 2 | `withdraw(p)` | Prohibition ends |
| 2 | `grant(e)` | Funds granted |
| 2 | `request(e)` | Delivery |

This is not a different order across units or clock skew. There is one ordered log, exactly as K1 requires. K8 specifies the performance/deadline tie but does not require every authority act to precede every request with the same timestamp. `history()` imposes that extra ordering on its generated tests; neither the scope nor the input evidence imposes it on actual histories.

Copied output:

```
TIE actual refused-step prestate forbidden False
TIE refusals [Fraction(1, 1)] forbidden [(Fraction(1, 1), Fraction(2, 1))] receipt [Fraction(2, 1)]
TIE counted 1 verdict {'loss': Fraction(1, 1), 'under_prohibition': Fraction(1, 1), 'shares': {}}
TIE correct deadline events ['occurred(missed(o),unit)']
```

**Wrong outcome:** `counted_refusal` returns 1 and the verdict has no player. The reader extracted `[1,2)` from a prohibition that began at a later trace position than the refusal. Membership of scalar time 1 cannot establish that the prohibition applied when the request was refused. The test's deadline algorithm consequently puts delivery before the moved deadline and excuses the unit.

**Correct outcome:** this refusal must not count. At the ordinary deadline, after the authority's act at time 1, the duty is in conflict; send its deferred deadline at time 2 before delivery. InstAL then derives `occurred(missed(o),unit)`. The verdict must include the unit as a late player. This does not imply a positive monetary share: under the declared counterfactual policy that share can be zero. The confirmed error is the lateness decision and its connection to InstAL, not a fabricated amount claim.

A second manifestation uses the same ordered times but a zero-cost effect. Issue the duty at 0; deliver at 1; **then** issue the prohibition at 1; withdraw at 2. InstAL accepts the delivery. The verdict rejects valid evidence:

```
TIE valid receipt [Fraction(1, 1)] forbidden [(Fraction(1, 1), Fraction(2, 1))]
TIE valid evidence rejected the evidence for unit is not a chain this model covers
```

**Minimal repair:** preserve ordered positions at interval boundaries and at refusals/receipts, for example a certified `(time, log-position)` key in addition to physical durations. Evaluate actual refusal eligibility against the pre-event norm state. Specify a compatible tie rule for synthetic counterfactual requests; apply it in both the lookup and the InstAL replay. Add the two histories above. Alternatively, explicitly restrict and validate the interface to a canonical authority-before-request ordering, acknowledging that this narrows K1/K8. Do not silently reorder the certified log. This needs no change to InstAL semantics.

This is a surviving boundary variant of Astra round-5 A1/Fable A2, not another count for each refusal, receipt, deadline and documentation symptom.

## B. Design objections and evidence qualifications

1. **Numeric representation remains narrower than the prose.** `validation()` accepts `Fraction(5,2)` but prints `VALIDATION decimal AssertionError the cost of e is not a finite number of zero or more` for `Decimal('2.5')`. The implementation accepts Python `numbers.Real`, excluding Decimal. Say which numeric types are accepted, or normalize finite Decimal values. This was already an Astra B objection about numeric representation; it is not promoted into A this round. Consequently the draft-5 repair table's unqualified “accepted” is too broad for both of Fable's numeric examples.
2. **The default tie generator is an assumption, not independent evidence about all ordered histories.** It sorts authority acts/grants first, deferred deadlines second, requests third, ordinary deadlines last. That is a useful test convention but masks A1. Zero budgets and issuing duties at readiness work under that convention. The arithmetic identity between `work > budget` and the chosen lateness rule does not independently validate the convention.
3. **Counterfactual controllers still embody policy.** Work ends at a counted refusal or otherwise at receipt; compliance truncates that work to budget; grants in the replay are supplied at the constructed receipt. Solver agreement verifies that constructed controller/history. It does not establish that these are the only plausible actions a complying unit could take. The draft now mostly states this limitation accurately.
4. **Stop condition 3 remains open.** `gate()` applies prohibition intervals to hypothetical delivery times. Calling this “no norm rule” while admitting it is refusal by lookup requires the narrow distinction between evaluating norm syntax and applying a precomputed policy. There is no impossibility proof for the general composition, nor a general success proof. Preserving order for A1 does not itself require copying InstAL semantics into the verdict.
5. **Coverage claims are bounded.** Both forms of the ownership mutant and the foreign-match mutant are detected. Route mutation gaps persist; see the results below. A passing suite is not a compiled-program uniqueness proof or a specification of the adapter. The earlier inventory of other validation mutants was not exhaustively rerun here.

## C. Disclosed limits, not counted again

- **K4 unreachable amendment:** rerun `old.disclosed_open()`. Both requests are refused, then `occurred(missed(o),unit)` is derived although the declared amendment way is unreachable. This is the expressly open wrong outcome.
- **K8:** multiple ways, several bans/recurring conflicts, trees, faulty evidence, differing clocks, two duties per hand-over, runtime deadline placement, waiting without an attempt, and allocation of prohibition loss remain the document's open or unmaintained cases. Direct gated probes for touching, overlapping and separated intervals succeed, but do not upgrade those cases into maintained composition coverage.
- **K12:** direct observed `relayed(p)` bypasses a declining hop; direct observed `relaywithdrawn(p)` removes the prohibition. Both reproduce. They are the stated log-trust limit, not an authority-only bypass. Passing on is static; changing consent, selective withdrawal refusal, translation, leased context and route-rank policy remain open. A same-origin exemption attempted alongside the origin's direct edge is rejected under the declared route restriction, so that rejected probe is not evidence of another exemption defect.
- **K11:** amounts are external to grants; delayed, duplicated and reordered grants remain outside the sequential connection. Maintained ledger tests pass.
- **K2:** the derived-fluent dependency check is partial. No additional proof about bridges or compiled-program uniqueness follows from these runs.

## Round-5 list-A status, both testers

| Prior finding | Status | Evidence in this review |
|---|---|---|
| Astra A1: unrelated refusal becomes suspension/no loss | Partly fixed | Ordinary unfunded-before-ban case now records a miss and preserves actual loss; equal-time ordered variant survives as A1 |
| Astra A2: fixed-length comparator tests a different rule | Fixed | Revised whole-budget counterexample and revised comparator pass; 523 reproduced |
| Fable A1: dawdling/unfunded evidence loses actual loss | Fixed for the reported cases | Dawdler has loss 4, prohibition loss 1, share 3; unfunded-only case has loss/share 4; invalid receipt/readiness rejected |
| Fable A2: refusal reason/readiness unchecked | Partly fixed | Pre-ready refusal no longer counts, ordinary unrelated refusal no longer counts; ordered tie A1 remains |
| Fable A3: actual conflict intervals omit earlier prohibition | Fixed for the reported case | Exact DR history now charges upstream 3/2 and records 3/2 under prohibition; actual duty conflict empty but `forbiddenabove` interval retained; counterfactual InstAL agrees |
| Fable A4: first counterexample/501 against wrong rule | Fixed | Fixed-length charges 1 versus re-run share 0 in replacement example; 523 reproduced |
| Fable A5: adapter validation | Partly fixed | Unknown keys/events/norms/effects/procedures rejected; Fraction accepted; Decimal remains B; malformed shapes/reserved identifiers moved to an explicitly stated limit |
| Fable A6: ownership repair claim false | Fixed | Original M1, broader M1b and foreign-match M6 all detected by the maintained suite |

No prior item is relabelled as a new independent defect merely because another manifestation is demonstrated. Eight tester rows are not eight distinct roots; the table intentionally preserves each tester's own entries.

## Executed checks and numerical audit

All nine maintained commands pass. The norm suite reports **72 scenarios, 23 rejections**. A separate AST-based execution runs just the 23 rejection cases and prints their exceptions: all fire for the reason named by their scenario. Isolated malformed-shape probes produce the documented parser/attribute errors, rather than being credited as successful semantic checks.

The conflict suite's actual output includes:

```
40 random chains with bans
10 late deliveries excused by a refusal that counts, 3 units that delivered later than the ban's end, 18 refusals for want of funds, 5 attempts before readiness
... in all 40; in 34 of them that history differs from the actual one
```

The gated suite runs **300** no-ban chains and **2000** ban chains and reports **523** differing fixed-length shares. Thus the table's **2300** arithmetic checks is accurate. The ordinary connection runs 25 chains; plain verdict runs 300; evidence and ledger checks also pass. Counts are tests with overlapping purposes, not independent formal guarantees.

Additional coalition output:

```
COALITIONS 112 histories PASS including intermediate coalitions, budgets zero, readiness-issued duties
```

These are all subsets on 24 deterministic chains of one to three units. Delivery times are constructed with a separate integer-grid gate oracle, then each history is submitted to InstAL and compared for every emission/refusal. There was no intermediate-coalition discrepancy in this subclass. This extends the maintained all-complying check; it does not repair A1 or establish universal coalition equivalence.

The exact earlier delayed-readiness case now prints:

```
ROUND5 DR FIX {'loss': Fraction(3, 1), 'under_prohibition': Fraction(3, 2), 'shares': {'u0': Fraction(3, 2)}}
```

Its actual second unit has forbidden interval `[5/2,9/2)` and no conflict interval; its counterfactual refuses at 3 and delivers at 9/2. This confirms why `forbiddenabove`, rather than actual-duty conflict, is the right trace quantity for that example.

Boundary runs cover refusal at readiness, deadline and ban start; delivery at ban end; ban starting/ending at deadline; several refusals; zero budgets; early delivery; late refusal; dawdling; unfunded refusal; pre-ready refusal; and ban before duty issuance. Under the maintained tie convention InstAL's misses equal gated players throughout. In particular a no-refusal delivery at conflict end is a player with zero share, whereas a timely counted refusal excuses it. Touching/overlapping direct gated cases choose refusal 1; separated intervals choose the qualifying later refusal 3.

Three-authority routes with either intermediate declining do not carry the prohibition through. Created-unit inheritance and withdrawal work. Cycles, repeated self-hops and a route alongside a direct origin edge are rejected. All successful unmutated solver runs return exactly one answer set; validation failures are not zero-answer-set counterexamples.

## Mutation results

The original ownership-only mutation M1 is detected by `occurred(refuse(am),v)` in the strengthened ownership scenario. M6 (foreign effects matching another unit's norm) is detected by the missing expected conflict. Route mutations N1, N2, N8, N9, N10, N12 and N13 are detected: incorrect route rank, ignoring a refusal to relay, missing issue/withdrawal powers, unconditional issue relaying, and a missing withdrawal bridge.

All eight previously reported route gaps survive the 72-scenario suite again: N3 first-intermediate carrier; N4 first-hop-only route; N5 retained origin power; N6 omitted route-origin validation; N7 omitted ranked-authority validation; N11 unconditional withdrawal relaying; N14 omitted created-unit inheritance; N15 every authority may relay. These are coverage gaps, not eight additional observed wrong outcomes in the unmodified program. The historical attribution in K12 is supported by this fresh run as well.

The broader M1b ownership/funding/amendment mutation is also detected (`occurred(emit(am),v)`). The three advertised K8 comparisons each failed under an in-memory interface mutation:

| Mutated quantity | Observed failure |
|---|---|
| Conflict reader returns no intervals | Trial 3, `missed == expected`, line 157 |
| Forbidden reader discards intervals for units with no refusals | Trial 7, trace-fed verdict versus generator-fed verdict, line 165 |
| Counterfactual history omits the authority ban events | Trial 3, expected counterfactual emissions, line 179 |

These corrupt the readings/history supplied to the comparisons, rather than changing the assertions. They show the comparisons can detect faults in their inputs. They do not make the generator independent of its controller or establish the policy's correctness. The main run finished with `404` successful unmutated solver calls recorded and answer-set counts `[1]`; supplementary rejection-reason, delayed-readiness, isolated-validation and ordered-tie runs were executed separately and also had one answer set whenever they reached the solver. The final script includes those supplementary scenarios in its normal entry point, so its combined count is correspondingly higher.

## Document audit and stop conditions

The new status correctly transcribes the earlier records: Fable 21,15,6,5,6 and Astra 8,6,3,2,2. The six-of-eight new-material and four-HIGH statements about round 5 agree with those reports. Scope changes still prevent interpreting those numbers as a controlled convergence measurement.

K0's scenario/rejection counts and claimed ownership repairs check out. Its warning about rejection tests accepting any `AssertionError` is accurate; this run additionally checked the reasons. Its broad numeric wording needs the B1 qualification. K8's stated new refusal rule is clear, but its interface does not retain enough order to implement it on A1. The draft-5 repair table is accurate for the ordinary repaired examples and overstates completeness of refusal qualification at tied times. Its property-by-construction and offline-deadline corrections are accurate. The 40/34 distinction is accurate; stop condition 3's shorthand “40 histories that did not happen” should be read as 40 counterfactual evaluations, only 34 with different histories, not 40 distinct changed histories. These qualifications are grouped with the relevant root or B objection, not counted as extra textual defects.

1. **No trace or several:** not met in successful solver runs. No semantic uniqueness counterexample found.
2. **Repair needs changed InstAL semantics:** not met. A1 is an ordered-evidence/interface defect and admits a local repair.
3. **K8 impossible without restating norms inside verdict:** open, not demonstrated met. Narrow lookup/replay evidence is real, but A1 disproves unrestricted adequacy of the current interface. Multiple ways remain a separate disclosed open problem.
4. **K11 impossible by events alone:** not met in the tested sequential form; asynchronous limits remain.
5. **Two consecutive rounds with neither count falling:** **not met**. This review has one list-A root versus Astra's two in round 5. Therefore round 6 cannot be the second qualifying flat/increasing round, whatever the other independent tester reports. This does not certify convergence or a successful general composition.

CONFIRMED WRONG OUTCOMES (list A): 1
OF WHICH IN NEW MATERIAL: 0
CRITICAL: 0  HIGH: 1  MEDIUM: 0  LOW: 0

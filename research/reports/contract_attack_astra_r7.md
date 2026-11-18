# Draft 7 adversarial review — Astra

Reproduce with `python3 -B -u /root/mic/model/attack_contract_astra_r7.py`. The [script](/root/mic/model/attack_contract_astra_r7.py) runs the maintained suites, timed boundary cases, additional coalitions, the negative-share example through InstAL, adapter probes, and mutations in memory. It loads definitions from earlier Astra scripts without running their main programs. No existing source or report was edited. The supported InstAL harness produces its normal temporary compiler files. No subagents were used; Fable's round-7 report was not read.

**Two independent list-A defects: one MEDIUM comparison-model error and one LOW stale numerical claim. No new wrong outcome was found in the in-scope replay verdict or its lateness decision.** Both defective source statements existed in round 6. The expanded generator makes them newly visible; that does not make their source newly written. Thus the material split is zero new, two existing. This is a bounded review, not a correctness proof.

## A. Confirmed wrong outcomes and false statements

### A1 — MEDIUM — K8: the purported draft-5 comparator silently uses draft 7's stricter excuse rule

Runnable scenario: `comparator()`.

One unit, ready at 0, budget 3, one delivery duty issued at 0, one authority ban `[1,4)`. It requests at 1, is refused, and delivers at 5. There is one way, one duty, one ban; authorities act independently. This is inside the stated scope, and after-times are explicitly part of the new tests.

`test_gated.fixed_extension()` calls `u.counted_refusal`. That property requires delivery exactly when the ban ends. Its docstring calls this “Draft 5's stated rule”, and K8 says the comparison is against that rule. But draft 5 states: “A duty's clock stops from the unit's first refused attempt while the duty is in conflict, if that attempt was made by the deadline, until the conflict ends.” It does not make the suspension conditional on immediate delivery at the end.

Copied from the run:

```
COMPARATOR maintained {'miss': Fraction(2, 1), 'shares': {'u': Fraction(2, 1)}, 'sole_repairs': ['u'], 'findings': {'u': Fraction(2, 1)}, 'pays_for_link_without_finding': [], 'baseline_holds': True}
COMPARATOR draft5 {'miss': Fraction(0, 1), 'shares': {}, 'sole_repairs': [], 'findings': {}, 'pays_for_link_without_finding': [], 'baseline_holds': True}
COMPARATOR rerun {'loss': Fraction(2, 1), 'under_prohibition': Fraction(1, 1), 'shares': {'u': Fraction(1, 1)}}
```

InstAL on this history gives refusal at 1, forbidden/conflict interval `[1,4)`, delivery at 5, and `missed True` with draft 7's deferred deadline at 4 before requests. The replay verdict correctly gives one day to the unit and one under prohibition. The error is in the separately labelled historical comparison, not in that replay or in the current lateness rule.

**Correct historical result:** suspension is `4 - 1 = 3`; adjusted allowance is `3 + 3 = 6`; delivery at 5 produces no miss and no fixed-length share. The maintained comparator instead suspends nothing and charges 2.

**Minimal repair:** implement the historical comparator independently: select the first ready refusal inside the interval and no later than the original deadline, then extend by interval-end minus refusal. Do not use the current `counted_refusal`. Add this dawdling example with expected historical miss 0. Alternatively rename and document the comparator as a different rule; that would withdraw the historical comparison claim.

The in-memory corrected comparator passes the entire maintained arithmetic suite. That suite therefore does not distinguish the two rules on this outcome. The two named fixed-length counterexamples remain valid; this does not overturn the claim that fixed lengths can fail.

This is a residual of the comparison-rule defect identified in Astra round-5 A2/Fable A4. It is one root, not separate counts for the docstring, the prose, and the resulting wrong share.

### A2 — LOW — K8: the current 2,000-chain count is 563, not 523

Runnable scenario: `suites()` → `test_gated`, and `comparator()`.

K8's current “Amounts” section says: “On 2000 random chains with bans the fixed length gave different shares from a re-run in 523.” Actual maintained output:

```
ok   2000 random chains with closed intervals: every re-run reproduces the actual receipt, only late units have shares, and shares sum to the loss less the loss under prohibition alone
     a share is negative in 1 of them; the fixed length gives different shares in 563
COMPARATOR counts document 523; maintained 563 draft5 563
```

**Correct result and minimal repair:** say 563 for the current deterministic corpus, ideally generated from its output. Keep 523 only where explicitly reporting the historical earlier corpus. The “what was wrong in draft 5” historical row can retain its historical number with a date/corpus qualification.

This is independent of A1: correcting A1 in memory still gives 563. Conversely changing the sentence to 563 does not correct A1's one-unit wrong result. The two require separate repairs. The stale sentence was present in round 6, when it matched that round's generator.

## B. Design objections and claims stronger than their evidence

- **The replay controller remains a convention.** The maintained counterfactual histories are constructed from `Unit.work` and `Unit.after`, then submitted to InstAL. This checks whether the controller's scheduled requests are emitted/refused under those norms. It does not independently recover the intended controller from arbitrary conduct. The document now acknowledges this. My additional integer-grid coalition oracle checks a narrower subclass independently of `gate()`.
- **Canonical ordering is an interface precondition.** `canonical_order` handles its documented two-kind alphabet correctly on tested ties and descending times. It is not called by `verdict`, which receives no full ordered log. The maintained history builder calls it. Deploying the interface requires the evidence producer to call it too. Unknown third kinds are outside its documented alphabet; interposing one is not counted as an in-scope bypass. Grants precede requests in the builder; ordinary deadlines follow requests and deferred deadlines precede them. Those deadline distinctions must not be erased by classifying all events as authority acts.
- **Adapter shape policy remains uneven.** An explicitly declared authority with an empty charter but no originating norm is rejected: `power of b names an authority that issues nothing or a norm that does not exist`. This is stronger than checking that names exist. Tuple and set `authorities` are accepted despite the validation message saying “list”. These remain interface-design/shape objections, consistently with Astra's earlier classification, rather than being promoted to list A this round.
- **The negative-share example uses several bans.** The arithmetic example is real, and its coalition receipts can be checked through InstAL. It is not evidence that the maintained one-ban composition generator covers recurring bans. Dropping one interval from each of its two multi-ban units removes the negative share in the four simplifications tried. That does not establish impossibility of negative shares with one ban.
- **The broad historical status sentence overstates the pattern.** “Material that existed before a round holds up” should be qualified as a tendency. The same document later identifies two round-6 findings in older material, and Astra's round-6 report classified its equal-time defect as older. The detailed records are more precise than that summary. I treat this qualitative generalization as B, not another independent numerical finding.
- **The coverage inventory is historical evidence, not newly verified coverage.** This review rechecks the repairs listed below, including both ownership forms and foreign matching. It does not rerun every route and validation mutant in the union from rounds 3–6. Restoring the omitted inventory is supported by the earlier reports; the fact that it is restored does not repair those gaps. No maintained test checks the report's wording or its historical counts.

## C. Explicitly stated limits, kept out of A

- K4's unreachable amendment way remains a way: the old disclosed scenario still records a miss although the authority blocks reaching the amendment procedure. It is expressly open.
- K8's conventions about attempts, free work, conditional after-time, compliant late units, claimant-borne prohibition loss, and refusal without a recorded reason are policy choices. A refusal under a ban can excuse even when funding was also missing; a unit that never attempted remains late.
- Several bans, recurring or overlapping conflicts, multiple ways/duties, trees, faulty evidence, differing clocks, and online deadline placement remain listed limits. Direct multi-ban probes here exercise first-refusal selection, touching and overlapping intervals, and an after-time covering a second interval. They are not promoted to defects merely for departing from a stronger unclaimed multi-ban controller.
- K12's observed `relayed`/`relaywithdrawn` injection remains the stated log-trust limit. Static consent, selective withdrawal policy, rank asymmetry, translation and leased context remain open. Direct routes plus an origin edge and repeated-authority routes are intentionally rejected.
- K0's unread self-declared attributes, Decimal rejection, malformed shapes/reserved identifiers, and explicitly recorded mutation gaps remain disclosed limitations. K2's partial dependency check remains a sketch rather than a compiled-program uniqueness proof.
- K11 remains a sequential ledger connection, not a result about late, duplicated or reordered grants. F2 agreement remains assumed.

## Repairs from both testers, rounds 5 and 6

Statuses distinguish functional repair from whether maintained tests pin it. Overlapping tester entries are preserved as rows, not counted as independent roots.

| Prior list-A item | Status | Executed evidence and regression check |
|---|---|---|
| Astra r5 A1: unrelated refusal becomes suspension | Fixed within the current canonical-order scope | Unfunded-before-ban and unfunded-only timed histories produce misses; removing the forbidden-refusal filter is tested as a mutation. Noncanonical order is now an explicit restriction. |
| Astra r5 A2: wrong fixed-length comparison | Partly fixed | Both replacement named counterexamples work; dawdling historical comparison remains wrong (A1); current count is stale (A2). Correcting the comparator in memory leaves maintained arithmetic tests passing. |
| Fable r5 A1: re-run loses actual loss outside old conventions | Fixed for reported cases | Dawdler has loss 4, prohibition loss 1, unit share 3; unfunded-only loss/share 4; invalid receipt/readiness rejected. Reverting work/after separation is mutation-tested. |
| Fable r5 A2: refusal eligibility/readiness | Fixed within the declared scope | Readiness, deadline, ban-start ties; earlier and unrelated refusals; repeated refusals tested against InstAL. Dropping readiness and forbidden membership is mutation-tested. |
| Fable r5 A3: conflict intervals hide bans before readiness | Fixed for one way; generalization remains a stated limit | Delayed-readiness case: upstream 3/2 and prohibition 3/2. Mutation supplies conflict intervals instead of forbidden intervals. |
| Fable r5 A4: first counterexample and comparison count | Partly fixed | Named counterexample repaired; residual comparator/count issues in A. Historical text is not pinned by a maintained document test. |
| Fable r5 A5: adapter validation | Partly fixed; malformed-shape and numeric portions moved to stated limits | Unknown scenario/event/norm inputs and fractional-cost regression mutations; new adapter cases and all maintained rejection reasons inspected. No claim that every malformed shape is now handled. |
| Fable r5 A6: ownership mutation claim | Fixed | Both ownership mutations and foreign matching are detected by maintained tests. The ownership-only mutation fails on the foreign amendment refusal. |
| Astra r6 A1: equal-time order erased | Fixed by narrowing the accepted interface | Requests before equal-time authority acts rejected; canonical ties accepted. Removing the ordering rejection is mutation-tested. The rejected old histories are now outside the declared convention. |
| Fable r6 A1: ban waiting replayed as work | Fixed for reported in-scope cases | Named expected amounts 15/4 and 1/4, and 3/4, pass through InstAL. Reverting work to counted-refusal time and omitting after-time are mutation-tested. |
| Fable r6 A2: norm keys and charter names silently accepted | Fixed for reported silent-name cases; stated-limit portions moved to C | Misspelt norm key, unknown charter norm and unknown charter authority rejected. Reverting norm-key and charter checks is mutation-tested. Unread self-declared attributes and Decimal are now stated limits. |
| Fable r6 A3: 40 different histories where only 34 differed | Fixed | Current random corpus reports 71 coalition evaluations, 43 differing histories. Reverting the prose in memory has no maintained document assertion to fail. |

The four named cases contribute ten coalition evaluations in addition to the random corpus's 71. “71” and “43” should be read with their explicit random-corpus qualification, not as the totals for all named plus random evaluations.

## Numerical and document audit

All nine maintained commands pass. The exact outputs support:

| Claim | Run result |
|---|---|
| Adapter suite | 74 scenarios, 25 rejection cases |
| Plain norm/verdict wiring | 25 random chains |
| Named conflict cases | Four; every coalition evaluated |
| Random conflict chains | 40; 15 excused deliveries, five dawdlers, 11 unfunded refusals, six pre-readiness attempts |
| Random conflict coalitions | 71, with 43 histories differing from actual |
| No-ban arithmetic | 300 chains |
| Ban arithmetic | 2,000 chains |
| Negative-share count | One |
| Fixed-length differing shares | 563, not 523 |
| Plain verdict | 15 named cases and 300 random chains |
| Evidence | 400 plus 300 random chains |
| Ledger | Six named checks and 2,000 random histories |

The two fixed-length counterexamples and the discontinuity example still pass. The named round-6 repairs give the amounts the draft reports. K0's two new rejection cases reject for the intended reasons, not incidental parser failures; all 25 rejection messages were printed separately. `authorities` as a string is rejected by its shape check, but there is no maintained rejection scenario pinning that new check.

K1 correctly states the new restriction. K8 correctly separates late status from work/after-time, identifies the offline deadline placement, and limits what its constructed replays establish. Its historical fixed-length comparison and numerical sentence need A1/A2. The claims table's 2,300 arithmetic chains and current random-corpus 71/43 figures are supported. The draft-6 repair table's replay cases, ordering restriction, validation changes and count replacement are supported by the runs and prior reports, with the pinning qualifications above. The restored K0 inventory corresponds to earlier reported gaps; this review does not claim a new exhaustive mutation audit of it.

## Negative shares, checked by hand and by InstAL

For the printed example, budgets are `(3,4,2,4)` and actual receipts are `(7/2,19/2,13,39/2)`. Actual loss is `13/2`; everyone complying leaves loss 2. All 16 coalition final receipts matched InstAL using a separately written interval controller. Summing marginal contributions over the 24 permutations gives:

```
independent shares {'u0': Fraction(-1, 24), 'u1': Fraction(55, 24), 'u2': Fraction(15, 8), 'u3': Fraction(3, 8)}
```

The negative share can be checked without trusting either Shapley implementation. For u0, every coalition marginal is zero except when the preceding set is `{u2,u3}`: its loss is `5/2`, whereas adding u0 changes it to 2. That set has Shapley weight `2! 1! / 4! = 1/12`, so u0's share is `(-1/2)/12 = -1/24`. The four shares sum to `9/2 = 13/2 - 2`. The example is correct under the declared replay conventions; its multiple bans remain the coverage qualification in B/C.

## Mutation results

The readiness, forbidden-refusal, work-to-counted-refusal, omitted-after-time and disabled-equal-order mutations are detected by `test_gated.py`. The norm-key, charter-power, scenario-key, event-norm, integer-only-cost, both ownership and foreign-match mutations are detected by `test_instal.py`. Replacing forbidden intervals with conflict intervals also fails the maintained composition test (AssertionError at line 120). The obsolete any-refusal historical comparator mutation survives; the historical-comparator repair is not pinned. The new authorities-shape check can be removed without failing that suite. This is a coverage gap, not evidence that the unmodified adapter accepts the string probe.

## Stop conditions

1. **No trace or several:** not met in the successful unmodified solver runs. Rejected malformed inputs are not zero-model semantic counterexamples. Route probes with a declining intermediate do not yield an authority-only bypass.
2. **Repair requires changing InstAL semantics:** not met. A1 needs a comparison-helper repair; A2 needs a reporting correction. Neither changes InstAL.
3. **K8 requires restating norm rules in the verdict:** remains open. The narrow interval lookup and supplied work/after controller agree on the tested histories. This is not a general composition proof. Several ways remain unresolved. Neither A finding demonstrates that the condition is met.
4. **K11 cannot be made by events alone:** not met in the tested sequential form; all maintained connected and standalone checks pass.
5. **Two consecutive rounds with neither tester's count falling:** not met. Astra's count rises from one to two this round, but round 6 fell from two to one, so rounds 6–7 cannot be two consecutive non-falling rounds regardless of Fable's round-7 count. I did not consult that report. Counts are not controlled convergence evidence as scopes and stated limits change.

CONFIRMED WRONG OUTCOMES (list A): 2
OF WHICH IN NEW MATERIAL: 0
CRITICAL: 0  HIGH: 0  MEDIUM: 1  LOW: 1

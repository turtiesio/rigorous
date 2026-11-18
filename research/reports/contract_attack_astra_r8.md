# Draft 8 adversarial review — Astra

Run [attack_contract_astra_r8.py](/root/mic/model/attack_contract_astra_r8.py) with `python3 -B -u /root/mic/model/attack_contract_astra_r8.py`. Mutations execute in memory; the supported InstAL harness uses its normal temporary compiler workspace. No existing source or report was edited, no subagents were started, and Fable's round-8 report was not read. Earlier report counts are checked against the reports, not inferred from this draft's characterization of them.

## A. Confirmed wrong outcomes or false statements

### A1 — HIGH — K1/K8: the original deadline can precede issue at the same certified time

Runnable scenario: `probes()`, `deadline before issue`, its reversed-order control, and `ZERO DUE`.

One unit, one duty `o`, one delivering effect `e`, readiness 0, budget 0, receipt 2. Authority `a` issues the duty at readiness. No ban is needed (the one-ban scope includes zero bans). There are no refusals or contingent authority acts. Send:

```
time 0: deadline(o)
time 0: issue(o)
time 2: request(e)
```

K8 rule 3 says to place this non-excused unit's deadline at its original time: 0. It does not require that the duty's issue precede that deadline. The authority act precedes the request; the cross-unit ordering condition is vacuous. Thus both stated equal-time requirements hold. The maintained log projection drops deadlines; `canonical_order` accepts the projected log. Adding a deadline kind does not supply a constraint either.

Actual solver output includes:

```
deadline(o): occurred(deadline(o),unit), no occurred(missed(o),unit)
issue(o): occurred(issued(o),a)
request(e): occurred(emit(e),unit)
ZERO DUE verdict {'loss': Fraction(2, 1), 'under_prohibition': Fraction(0, 1), 'shares': {'unit': Fraction(2, 1)}} late True
```

Reversing only the first two entries, at the same times, produces `occurred(missed(o),unit)` at the deadline. Both runs have exactly one answer set. InstAL correctly evaluates each ordered history: at the first deadline there is no duty in force. The composition rule loses that distinction and does not make its promised lateness record.

**Correct outcome:** the zero-budget duty issued at readiness and delivered two days later must be recorded as missed, consistently with `Unit.late`. Require issue before the original deadline at a tie, after any on-time performance as applicable. Carry positions for deadline placement, or enforce this additional ordering in the evidence interface. Add both event orders as tests. Excluding zero budgets would be a further scope restriction, not the present contract: the verdict accepts zero, the earlier Fable review and this review exercise it, and only negative budgets are rejected.

This is a missing boundary ordering, not an assertion that InstAL mis-evaluates a deadline for an unissued duty. The disclosed absence of an online deadline scheduler does not fix the literal offline rule either. It is also not the recurring-duty limit: the duty is issued exactly once. I classify its root as **older material**: the original-time branch and loss of deadline positions predate round 8. Adding another order check does not make that residual new, consistent with Astra's earlier source-age convention.

### A2 — LOW — Stop condition 5 record: Astra's new-material count is misstated

Runnable scenario: `audit()`.

Draft 8 says: “one tester put two of three in material written after round 6, the other both of two.” The first half matches Fable. The second contradicts Astra's report:

```
RECORD contract_attack_astra_r7.md ['CONFIRMED WRONG OUTCOMES (list A): 2', 'OF WHICH IN NEW MATERIAL: 0', 'CRITICAL: 0  HIGH: 0  MEDIUM: 1  LOW: 1']
RECORD contract_attack_fable_r7.md ['CONFIRMED WRONG OUTCOMES (list A): 3', 'OF WHICH IN NEW MATERIAL: 2', 'CRITICAL: 0  HIGH: 1  MEDIUM: 1  LOW: 1']
```

Astra explicitly explains that its comparator and stale numerical sentence existed in round 6. **Repair:** say “the other neither of two.” This is a false factual attribution in new prose. It does not change the overall round-7 totals or the non-falling-count test.

### A3 — LOW — Stop condition 5 qualification: the alternative charter count is four, not five

Runnable scenario: `audit()`.

The new prose says the other tester noted “a rejected empty charter could be counted as a fifth.” Fable's actual round-7 statement, printed by the script, is:

```
If the charter check (PWa) is read as a wrong rejection and not as a design choice, it is 4.
```

That report has three A findings; adding its one charter objection makes four. Astra's two cannot produce five either. **Repair:** replace “a fifth” by “a fourth.” Keep the actual count at three, as Fable did. This is a separate incorrect numerical statement: repairing the material-age attribution in A2 does not repair this arithmetic, or vice versa. Both are new material. The charter objection itself remains B; I have not promoted it to create this finding.

## B. Design objections and evidence limits

- No valid log *within both literal K1 ordering restrictions* was found that `canonical_order` rejects. It does reject an upstream delivery, downstream attempt, then upstream retry at one time, even though readiness is causally respected. That is stronger than the motivating requirement, but K1 expressly requires all upstream requests before downstream requests, so it is not another A finding.
- The log parameter is not authenticated or cross-checked against the Unit evidence. Supplying an empty or unrelated log defeats the purpose of its check, but faulty evidence is explicitly excluded. Unknown event kinds and missing entries are interface-validation questions, not a demonstrated valid certified-history bypass.
- A grant is a ledger event, not an authority act. The maintained projection classifies it as `authority`; a request/grant/request sequence is therefore rejected by that projection. K11 already excludes reordered grants; no additional in-scope verdict error is inferred from this.
- A test with reversed interval endpoints exercises invalid intervals, but does not pin rejection of equal endpoints. “Each has a test” is defensible if “empty interval” includes the reversed interval tested; it is not evidence that both sides of `b <= a` are pinned. Similarly, choosing the last rather than first qualifying refusal needs a multi-refusal test.
- The maintained coalition histories still use the verdict's work/after readings and gate convention. Successful comparisons establish the scheduled controller's behavior under InstAL, not uniqueness of that controller as an interpretation of arbitrary conduct. The extra grid in this review broadens boundary coverage but also uses that composition helper; it is not represented as a new independent 155-coalition oracle.
- The phrase “one was a wrong outcome” in the status narrative distinguishes the primary replay error from the separately identified comparator error. Read literally over *all* executable models there were two, but the paragraph expressly mentions the comparison helper. I retain this semantic qualification in B instead of manufacturing another count.
- Mutation checks of prose have no maintained assertion to fail. Corrected numerical and historical claims can regress without a code test noticing.

## C. Stated limits, not additional A findings

K4's unreachable amendment way still records a miss; the script reruns it. K8's conditional after-time controller, earliest compliance when only after-time is excessive, claimant-borne prohibition loss, refusal without reason, and waiting without attempting are declared conventions. Several bans, overlapping/recurring conflicts, several ways or duties, trees, faulty evidence, differing clocks and online deadline placement remain open. Withdrawal and reissue of a duty concern the separately excluded repeatedly triggered duties; ban withdrawal/reissue was probed for ordering without claiming a multi-ban theorem.

K12's direct relayed-event injection remains the disclosed log-trust limit. Static relay consent, selective withdrawal, route rank asymmetry, translation and leased context remain unanswered. K0's malformed shapes, unread attributes, Decimal rejection and published mutation gaps remain disclosed. K2's dependency checker is still only a partial check. K11's sequential ledger tests do not establish correctness for repeated, late or reordered grants. F2 agreement remains assumed.

## Round-7 list-A status, both testers

| Prior finding | Status | Reason |
|---|---|---|
| Astra A1: historical comparator uses current excuse rule | Fixed | Independently written historical comparator and the dawdler regression distinguish the rules. |
| Astra A2: 523 versus 563 | Fixed | Draft now reports 563 for the 2,000-chain corpus. |
| Fable A1: downstream attempt before equal-time hand-over excuses | Fixed by the stated ordering restriction | Verdict calls the checker, and reversing chain order is rejected. The newly found duty/deadline tie is a different pair, not a failure of this specific repair. |
| Fable A2: excused unit's deadline before its delivery | Fixed | Rule 3 now says receipt, after request, for excused units, matching the helper. The original-time branch has the distinct A1 boundary omission. |
| Fable A3: 523 versus 563 | Fixed | Same numerical repair as Astra A2; not independently counted here. |

The negative-share scope correction, previously uncalled checker, coalition selection and empty-charter entry in draft 8's table were round-7 B objections, not additional A items. Their inclusion in that table must not retroactively increase either tester's A count. Explicit empty authority charters now run; numeric/scoping corrections are assessed below.

## Numerical audit, mutation evidence and stopping decision

All nine maintained commands passed. The observed counts are:

| Draft claim | Verification |
|---|---|
| 74 scenarios, 25 rejections | 74 from the suite; all 25 rejection reasons printed separately and match the intended checks. |
| Four named conflict cases | All four pass, with all their coalitions; separate from the random corpus. |
| 40 random conflict chains | Pass; current draw has 5 excused deliveries, 4 after-time dawdlers, 11 unfunded refusals and 8 pre-readiness attempts. |
| 61 random-corpus coalitions; 51 differing histories | Exactly the maintained output. These counts exclude the named cases. |
| 2,000 multi-ban arithmetic chains | Pass; fixed-length differences 563; one negative-share example. |
| 4,000 at-most-one-ban chains | Pass; fixed-length differences 746; zero negative shares. |
| 300 no-ban arithmetic chains | Pass, equal to the plain verdict. Total gated arithmetic coverage is 6,300. |
| 155 coalitions; 83 differing histories | Both numbers appear in Fable r7's independently generated 64-chain result. Historical attribution verified; not claimed as 155 additional new runs here. |
| 388 solver runs | Fable r7 reports 388 with one answer set each. Attribution verified. |
| 60,000 one-ban chains without negative shares | Fable r7 explicitly reports zero of 60,000, and separately five negatives in its 60,000 multi-ban corpus. These are historical corpora, not the current 4,000. |

The printed negative-share example still has `u0=-1/24`, `u1=55/24`, `u2=15/8`, `u3=3/8` and recurring bans. The scope correction is justified. It does not prove anything about an arbitrary multi-ban composition. The new fixed-length comparator's dawdler case passes with historical miss zero and current replay assigning one day each to the unit and prohibition.

The status paragraph's total series match both reports: Astra 8, 6, 3, 2, 2, 1, 2; Fable 21, 15, 6, 5, 6, 3, 3. Round 7 is a non-falling round, preceded by a falling round. The erroneous material split and alternative count are A2/A3, not changes to those totals.

The additional 18-chain grid covers zero/positive budgets, zero/positive work, refusal at readiness, after-time, duties issued initially or at readiness, and every coalition selected by its late players. InstAL and the replay agree on the generated histories; the missed sets match the independently computed grid expectations. This does not negate A1: the helper always puts issue before deadline at equal times, a stronger ordering than the text specifies.

K12 probes cover a three-authority route, refusal by either intermediate, withdrawal, direct relayed-event injection, cycle/direct-edge rejection and a routed created unit. No new authority-only bypass or multiple/no-answer-set scenario was found. Direct relayed-event injection reproduces the already disclosed limit. The adapter accepts the newly allowed empty charter and rejects the unknown norm key, charter norm and charter authority; all maintained rejection reasons were inspected.

**Round-6/7 regression mutations run in memory.** Removing the verdict's order-check call, either tie rejection, readiness filter, forbidden-refusal filter, work/after separation, or after-time replay is detected by `test_gated.py`. In particular:

```
MUTANT verdict order call DETECTED test_gated.py SystemExit a request recorded before an authority's act of the same time is rejected: accepted
MUTANT chain tie DETECTED test_gated.py SystemExit an attempt recorded before the hand-over of the same time is rejected: accepted
MUTANT work old rule DETECTED test_gated.py AssertionError a unit refused after its deadline is not charged again for the wait
```

This fixes the previous gap where only a separately invoked order checker was tested. It does not cover the deadline/issue ordering in A1. The interval-equality and first-refusal selection mutations are additional coverage probes, not reversions of newly claimed functional repairs.

The norm-key and charter-name validation regressions fail `test_instal.py`. Reverting the independent historical comparator fails its new dawdler assertion. Replacing forbidden intervals by actual duty-conflict intervals fails `test_conflict_chain.py`. Thus these specific round-6/7 functional repairs are pinned.

Reinstating the old rejection of an explicitly declared authority with an empty charter **survives** `test_instal.py`, despite the direct probe showing the functional repair works. Changing the invalid-interval predicate from `b <= a` to `b < a` **survives both** gated suites. This is an equality-boundary gap, not a new failure in the unmutated verdict.

Eight further coalition histories for three specified controllers were built without `Unit.work`, `Unit.after` or `gate()` in the scheduling calculation. These include the after-deadline refusal, an after-time within budget, and the ban before actual readiness. All final emissions match the independently scheduled receipt and the gated rerun. This is small independent confirmation; no claim of exhaustive replay correctness follows.

Selecting the last qualifying refusal instead of the first also **survives both** gated suites. Allowing the second coalition to be the entire late-player set **survives** the composition suite. The latter is a test-generator repair that is functionally present but has no regression assertion enforcing the proper-subset requirement. Neither survivor is evidence of a new wrong unmutated output.

```
MUTANT empty interval equality SURVIVED
MUTANT last refusal SURVIVED
MUTANT empty authority charter repair SURVIVED
MUTATION allow whole late set as second coalition SURVIVED
```

Text-only reversions of the old 523 count, the old excused-deadline wording, or the old scope characterization have no maintained document checks. Their corrected status is established by the current runs and comparison with the text, not by calling them pinned. The main script can be run by phase (for example `probes`, `mutations`, or `coalition_mutation`) to reproduce these checks separately.

## Stop conditions

1. **No trace or several:** not met by these runs. Every successful unmutated solver call returned exactly one answer set. Adapter rejections are not zero-answer-set semantic counterexamples.
2. **Repair needs changing InstAL semantics:** not met. A1 needs a deadline ordering precondition and its enforcement/test; InstAL evaluates the supplied histories correctly. A2/A3 need prose corrections.
3. **K8 needs norm rules restated inside the verdict:** still open, not established by this attack. Deadline position is another fact/order requirement, not a norm rule. The tested narrow replay controller agrees with the supplied norm program. Several ways, counterfactual conjunctions and online deadline placement remain unresolved. The cited 51/83 counts are accurately attributed, but do not prove the general claim.
4. **K11 cannot be made by events alone:** not met in the sequential tested form. Both ledger suites pass; concurrent grant delivery is still excluded.
5. **Two consecutive rounds in which neither count falls:** Astra's half of the round-8 condition is satisfied: **3 versus 2** in round 7. Round 7 was already a joint non-falling round. Therefore the condition is met **if Fable's independent round-8 A count is at least 3**; it is not met if that count is below 3. This review cannot choose between those results without that independent count, and deliberately has not read Fable's round-8 report. Nothing in the historical-report corrections changes the threshold.

There are three independent roots: **one wrong composition outcome and two false document statements**. One is in older material; two are in prose written after round 7. The older-model classification follows Astra's prior treatment of residual source defects rather than dating every defect anew when a surrounding paragraph changes. The two false statements assert different quantities, can be independently repaired, and neither is a variant of the deadline defect. Disclosed limits and mutation survivors have not been promoted into A to affect convergence.

CONFIRMED WRONG OUTCOMES (list A): 3
OF WHICH IN NEW MATERIAL: 2
CRITICAL: 0  HIGH: 1  MEDIUM: 0  LOW: 2

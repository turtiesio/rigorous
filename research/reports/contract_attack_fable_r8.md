# Attack on the composition contract, round 8 (Fable)

Target: [Composition contract.md](/root/mic/reports/Composition contract.md), draft 8, and the models in /root/mic/model as changed after round 7.

Every scenario is in [attack_contract_fable_r8.py](/root/mic/model/attack_contract_fable_r8.py):

```
python3 /root/mic/model/attack_contract_fable_r8.py            # about twenty-five minutes on eight cores
python3 /root/mic/model/attack_contract_fable_r8.py --fast     # skips the mutation checks and the instrumented test, about five minutes
```

The "derived" lines below are copied from that run. Identifiers such as DOC6 are the labels the script prints. No existing file was changed; mutation checks work on copies. The nine documented checks all pass. I did not read the other tester's round-8 report.

## Summary

- **Count: 1 confirmed (list A), against 3, 3, 6, 5, 6, 15 and 21 in the earlier rounds. My count fell.** Counted by finding, as in rounds 2 to 7, false statements in the document included. The one finding is a false statement, rated low, in text written after round 7. I found no wrong outcome of the models inside the stated scope.
- **All five of round 7's findings are repaired.** Four are held by a test or are a matter of text that is now right. The fifth, equal times, is repaired by exclusion: the log that gave the wrong outcome is now refused by the verdict itself, and a test fails if either ordering or the call is removed.
- **I looked for a third ordering and did not find one in what the verdict reads.** Over every log of two units on four certified times with one ban per unit (125260 readings), a log in the required order is never refused and reading times never differs from reading positions. There is a reason: the two orderings fix the position of every entry at one time except among acts of authorities, and the forbidden intervals come from the trace's states, which already hold that order.
- **The nearest thing to a third ordering is in the placing of a deadline, and it sits under two listed limits.** One ban withdrawn and re-issued at one certified time: where the deferred deadline stands among the acts of authorities at that time decides whether InstAL records a miss. I list it and do not count it (T3-WR, list C).
- **The re-run verdict held again.** 160 chains from round 7's wider generator with new seeds, every coalition through InstAL (401 coalitions, 210 of them histories that did not happen): no disagreement. The same generator without InstAL on 40000 more chains (99891 coalitions): none, and no valid evidence refused.
- **What the equal-times repair is worth is narrower than it reads.** Nothing ties the log to the evidence; the tests build their logs already sorted, so the check is exercised by three hand-written logs; and the helper that builds "the log the verdict is given" labels a ledger grant an act of an authority, so harmless evidence gets no verdict (list B).
- **K12 and K2 hold.** 840 InstAL runs in the script, one answer set each.

The script's last full run printed: `1 scenarios marked A-DEFECT, 26 B-objection, 54 C-listed, 118 as specified, 0 not reproduced`.

## List A: confirmed wrong outcomes or false statements inside the stated scope

### A1. LOW. Stop condition 5: where the round-7 findings were

New material (the paragraph was written after round 7).

- **Statement.** Stop conditions, the record for condition 5: "Where the round-7 findings were: one tester put two of three in material written after round 6, the other both of two."
- **Scenario DOC6.** The other tester's round-7 report, its count lines and its summary.
- **Derived.** `the document has the sentence: True || astra_r7: ['CONFIRMED WRONG OUTCOMES (list A): 2', 'OF WHICH IN NEW MATERIAL: 0', 'CRITICAL: 0  HIGH: 0  MEDIUM: 1  LOW: 1'] || astra_r7, summary: Thus the material split is zero new, two existing`
- **Correct statement.** The other tester put neither of its two in material written after round 6 ("Both defective source statements existed in round 6"). The record for round 7 is two of five in new material and three in older material. The sentence says four of five.
- **Why it counts.** These lines are the document's evidence on where defects arise, which bears on what a falling count means. Draft 7's status text stated it as a pattern ("material that existed before a round holds up"); draft 8 dropped the pattern and kept the record, and the record is wrong in the direction of the dropped pattern. By the standard of rounds 4 to 7 a misreported count of the testers' record is a false statement (round 4: the status line miscounted round 3; round 6: "40 histories that did not happen"; round 7: "523").
- **Repair.** "One tester put two of three in material written after round 6, the other neither of two."

## List B: design objections and claims stronger than their evidence

| Id | Clause | Objection | Derived |
|---|---|---|---|
| TIE-OMIT, TCO | K1 "The verdict takes the log and rejects one in another order"; "What was wrong in draft 7" row 6 | Nothing ties the log to the evidence. Round 7's evidence with a log that leaves the early attempt out, or with no log, passes the order check and the refusal excuses. test_gated.py builds each log from the evidence, already sorted into the required order, so on its 6300 random chains the check cannot fire; test_conflict_chain.py passes a history it has sorted itself, and passes when it hands the verdict an empty log. The check is exercised by three hand-written logs. A faulty evidence layer is listed as open, so this is a remark on what the repair is worth | `{}, u1 late: False \|\| {}, u1 late: False \|\| {}, u1 late: False`; `test_conflict_chain.py: passes` |
| GR1 | K1 "acts of authorities before requests"; K11 "The ledger sits outside" | No ban and no authority: a costed delivery asked for at 1 without funds and refused, granted at 1, asked for again at 1 and made. The helper that builds "the log the verdict is given" labels a grant an act of an authority, so the verdict refuses evidence on which reading times is harmless. That grants come before requests at one time is in a docstring of the test, not in the contract. Request, grant in answer, request again is the ordinary sequence under K11 | `receipt=1, refusals=['1'], forbidden=[] ... gated: ValueError: at time 1 a request precedes an act of an authority \|\| with the grant under its own kind: {}` |
| DUTY-F | K8 scope "Acts of authorities do not depend on what units do"; the third named case; "half with every duty in force from the start" | In half of the random chains and in the third named case the principal's issue of the duty is placed at the unit's readiness and moved with it on every re-run. That is an act of an authority that depends on what units do. Held at its actual time, InstAL refuses the unit at 4.5 for want of a permission and the chain ends at 5, where the re-run says 4.5 (the upstream unit would owe 1 and not 1.5). The intended reading is a duty that arises on receipt; the contract should say so, because a duty issued at a fixed time is a second gate on delivery that the verdict is not given | `issue moved to 2: ... u1: refused at ['3'], delivered at ['9/2'] \|\| issue held at 5: ... u1: refused at ['3', '9/2'], delivered at ['5'] \|\| gated.py: ... kept [] -> final receipt 9/2` |
| DUTY-T | K1's first ordering | When a duty's issue and the delivery it answers share a certified time, the log in the order of cause (delivery, issue, request) is refused, although nothing the verdict reads depends on it. F2 is narrowed further than the two defects needed | `cause first: ValueError: at time 3 a request precedes an act of an authority \| issue first: {}` |
| GN, GO, GEI2 | K8 "It refuses ... Each has a test" | Each has a test, and five of the seven rejections are pinned (both orderings, a receipt before readiness, a negative budget, an interval that ends before it starts). With the readiness check or the forbidden-receipt check deleted, both suites still pass: the catch-all on the re-run refuses those two cases. An interval of no length can be accepted without a test failing | `test_gated.py + test_conflict_chain.py: passes` each |
| HPO, HP2, HA | "What was wrong in draft 7" last row: "Accepted" | True (PWa). No scenario holds it: with the charter check put back as it was in draft 7 the suite passes. The check without its test of the authority, and no check that `authorities` is a list, still pass as in round 7 | `test_instal.py: passes` each |
| PWc, PWb | K0, the charter check | Unchanged from round 7: a charter that gives one authority another's norm is accepted and does nothing; a charter given as the string 'ap' is read as norms a and p | `issue(p) -> ['ultravires(p),state']; request(ord_x) -> ['emit(ord_x),unit']` |
| GL, GCT, GCN | K8 "its first attempt made once ready and refused under a ban"; canonical_order | Still unpinned: the last such attempt taken in place of the first. New and unpinned: a log out of order of time accepted; ties compared between neighbours only | `passes` each |
| UA, UF, TF | K8 rule 3; K11; "half with every duty in force from the start" | Still undetected: InstAL recording a miss for a deadline in conflict; InstAL keeping a grant after a refusal; every random chain with its duties in force from the start | `test_conflict_chain.py: passes` each |
| DOC7 | Status "Three of the five were errors in this document or in a comparison helper ... One was a wrong outcome" | Three and one do not make five: four of the five findings were errors in the text or the helper, or three of four if the stale number is taken once. The sentence explains itself ("reported by both") and the later one, "of five findings in round 7, one was a wrong outcome of the models", is right, so I do not count it | by hand |
| Stop 3 | Stop condition 3 "the verdict layer needs two things"; K8 "What it is given" | The verdict now takes a third thing, the log, and K8's list of what it is given does not mention it. It is a fact from F2 and no norm rule, so it does not move the condition | see TIE1 |
| G-CV, G-DAW, L3-cf | K8 declared conventions | Unchanged from round 7: a complying unit is the latest possible when its work was too long and the earliest possible when its after-time was; a unit's idle time after a refusal is paid for mostly by whoever made the refusal possible; "what its log shows" and the convention are different histories when a unit has an after-time | `shares={'u0': '2'}, loss=2, under_prohibition=0`; `shares={'u0': '9/4', 'u1': '7/4'}`; `receipt=13/4 ... gated.py's re-run: 3` |
| AU2, NKf, EFb | K0 | Unchanged: a frozenset of authorities is refused where a set is accepted; `ultra_vires='no'` marks the norm; a misspelt key inside an effect changes the outcome silently | `request(e) -> ['emit(e),unit'] \|\| control: ... request(e) -> ['refuse(e),unit']` |

## List C: limits the document already lists

| Id | Clause | Limit | Derived |
|---|---|---|---|
| I1, I1c | K4 | An unreachable way counts as a way. Unchanged and stated | `deadline(o) -> ['missed(o),unit']`; one effect fewer: `deadline(o) -> ['conflict(o),unit']` |
| T3-WR | K4 "a conflict that ends and starts again [is] not in the maintained tests"; K8 open "several bans on one duty"; K8 rule 3 "when the conflict ends and before any request" | One ban, 1 to 6, withdrawn and re-issued at certified time 3. The unit, due at 2, never attempts and delivers at 6. By times delivery was forbidden without a break; the unit is late and a player. Placed as the test's six lines place it (the first conflict's end, after the acts of authorities of that time), the deferred deadline finds the duty in conflict again and InstAL records no miss. Placed between the withdrawal and the re-issue, it records one. A position, not a time, decides the record | `conflict=['1..3', '3..6'], counted=None, deadline_sent=3(before requests) \| InstAL missed=[] \| gated: players=['u0']` against `InstAL missed=['u0'] \| gated: players=['u0']` |
| TIE1c, G-RDY | K8 "Not tested by anything" | An attempt at the certified time of the input, logged after it, excuses; the timing of a unit's first log entry decides what the unit upstream pays | `counted=3, deadline_sent=6 \| InstAL missed=[] \| gated: players=[]`; `shares={'u0': '6'}` against `shares={'u0': '0'}, loss=6, under_prohibition=6` |
| D5, LI3 | K8 open | A unit that waits without attempting is late and can be charged for the ban | `InstAL missed=['u0'] \| gated: players=['u0'], shares={'u0': '0'}, loss=3, under_prohibition=3` |
| MB1, MB2, MB3, TB, NEG1, NEG-I | K8 open "several bans on one duty"; "A share can be negative, outside this scope" | Two forbidden intervals on one unit: only the first refusal is read; an after-time can hold a wait under a second ban; a share can be negative. All as round 7 found, and now all stated as outside the scope | `players=['u0'], shares={'u0': '3'} ... after=['9/2']; counted=['None']`; `'u0': '-1/24'` |
| CB3 | K8 "Not tested by anything" | A unit with no funds, refused under a ban and funded as it ends, is excused | `counted=1, deadline_sent=4 \| InstAL missed=[]` |
| GM, UC | K8 "a tester found no evidence on which that check fires"; K8 rule 3 "A tester changed InstAL so that no duty is ever in conflict and the test still passed" | Both as stated: the re-run check can be deleted, and InstAL with no conflicts passes | `passes` each |
| SD1, SD2, W2e, PWe, AU4, NKb | K0 | An unread self-declared attribute; a Decimal cost; malformed shapes that end in unhandled errors | `AssertionError: the cost of e is not a finite number of zero or more`; `AttributeError`, `TypeError`, `KeyError` |
| RW1, RW2 | K12 "the log" | A relayed withdrawal, or a relayed issue below a declining authority, observed directly takes effect | `relayed(p) -> ['issued(p),group']; request(ord_x) -> ['refuse(ord_x),unit']` |
| N3 to N15 (8), M11 to M15, VT, VI, VCR2, VPR, NR5 to NR7, VC2 to VC4, VCY2, V2C, NE3 to NE5, NA, VE | K12 status; K0 "The list below" | All 30 wrong versions still pass, as stated | `passes` each |
| K8 open | K8 | Several ways per duty; who bears the loss under prohibition; two duties per hand-over; trees; no layer places a deadline | not run |

## Priorities 1 to 6

1. **Equal times, a third time.**
   - Round 7's two logs and round 6's are refused by the verdict (TIE1, TIE2, AST1): `gated: ValueError: at time 3 a request of u1 precedes a request of u0, which is before it`; `gated: ValueError: at time 1 a request precedes an act of an authority`.
   - ENUM, every log of two units on certified times 0 to 3, at most one ban per unit, up to three requests per unit, five pairs of budgets, read by position and by gated.py from times: `logs in the required order: 125260; refused by the verdict: 0; lateness, first refusal, excusing refusal or work read from times differs from the reading by position: 0 || the same events with every tie the other way round: 58730; refused: 58730; readings differ: 5360; differ and accepted: 0`.
   - Named ties in the required order agree with InstAL: an authority's act, a hand-over and the receiver's attempt at one time (T3-AH); a ban issued and withdrawn at one time; a ban withdrawn and re-issued at one time with a refusal before or at that time (T3-IW); two requests of one unit at one time (in ENUM).
   - `canonical_order` now compares every pair of entries with one time, so a third kind of entry no longer hides a pair (CO1, case 12). It refused no log that K1 allows in any run. Three things it refuses beyond what the two defects needed: a request before a grant when the grant is labelled as the test labels it (GR1), a delivery before the issue of the duty it triggers (DUTY-T), and a chain unit's request before that of a unit not in the chain (CO1, case 9).
   - What is outside the argument: the three things the verdict is not given. Grants (GR1), the issue of a duty (DUTY-F), and where a deferred deadline stands among acts of authorities at one time (T3-WR). Only the last changes a record, and it is under two listed limits.
2. **gated.py inside its scope.**
   - RND, 160 chains, one ban per unit at most, times on a half-day grid, through InstAL: `{'chains': 160, 'units': 309, 'late': 205, 'excused': 32, 'refused_after_deadline': 64, 'after_time': 63, 'before_ready': 21, 'unfunded': 29, 'several': 39, 'zero_budget': 54, 'zero_work': 47, 'early_unit': 72, 'coalitions': 401, 'counterfactual_histories': 210}; disagreements: 0`. Each coalition's history is laid out from the generator's record, not from gated.py's `work` and `after`.
   - RND-L, the same generator without InstAL, on the record InstAL agreed with: `{'chains': 40000, 'with_equal_times': 36675, 'coalitions': 99891, 'counterfactual_histories': 52925, 'valid_evidence_refused': 0, 'players_differ': 0, 'rerun_differs': 0}`.
   - Evidence wrongly rejected: none found with the log labelled by two kinds. Wrongly accepted: none in the required order. G7, 60000 chains of arbitrary evidence: `accepted 16031; rejected by the checks on receipts, readiness, budgets and intervals 43969; rejected because the re-run does not reproduce the receipt 0`.
   - No negative share with one interval per unit: `the test's generator, at most one: negative in 0 of 4000 ... fresh, at most one interval per unit: negative in 0 of 60000 || fresh, up to two: negative in 5 of 60000`. The argument of round 7 stands and I rechecked it by hand: with one interval every re-run receipt is at most the actual one, so a unit refused on the actual run never arrives past its ban.
3. **Who is late.** With deadlines placed by rule 3 as the text now words it, written in the script from the text and not copied from the test, InstAL's `missed`, `Unit.late` and the generator's own rule agree on all 160 chains of RND and on every boundary case (D1 to D9, H1, E1, P1, P2, LI1 to LI3, CB1, RULE3). The only disagreement I could build is T3-WR.
4. **Are the repairs pinned?** See the tables below.
5. **K12, K2 and the adapter.** No defect. All of round 7's route scenarios give the same results (R3a to RXB): routes of three and four with each intermediate declining, norms routed in opposite directions, a routed permission, created units to depth two, a declined routed exemption. `840 runs: min 1, max 1`. The relaxed charter check accepts round 7's empty charter (PWa), now refuses an entry for a unit, which round 7 found accepted and ignored (PWd), and refuses an entry for an authority that is neither declared nor the origin of a norm (PWa2). Each of the suite's 25 rejections still fires for the reason in its name (REJ).
6. **The document.**
   - As run: 74 scenarios and 25 rejections; 4 named cases; 40 chains, 61 coalitions, 51 differing histories, all distinct, and all 21 second coalitions a non-empty proper part of the late units (DOC4); 25 chains; 300, 2000 and 4000 chains; 563 and 746; one negative share in the 2000 and none in the 4000 (DOC1, DOC3, CE4).
   - As the earlier reports have them: the record 21, 15, 6, 5, 6, 3, 3 and 8, 6, 3, 2, 2, 1, 2; 155 coalitions, 83 histories, 64 chains; 388 runs; 60000 chains (DOC2).
   - True by run: K1's two orderings and "the verdict takes the log and rejects one in another order"; rule 3 as reworded (RULE3, RND); "the comparator is draft 5's rule written out on its own" for the sentence of draft 5 it implements, with a case that tells it from draft 7's (CMP, mutant TCMP); "I undid two earlier repairs in turn ... and a test failed each time" (TA, GA); "Detected, by the testers' runs" for all ten I re-ran; every row of "What was wrong in draft 7".
   - False: where the round-7 findings were (A1).
   - Stronger than its evidence or loosely worded: "Three of the five"; "each has a test"; "the verdict layer needs two things" (list B). "For the first time one tester rated nothing high" is true if high means high or above: that tester's round 4 had no HIGH and one CRITICAL.

## Round 7's list A, both testers, and the repairs of rounds 6 and 7

"Pinned" means that I undid the repair in a copy and a maintained test failed.

| Finding | Status | Evidence from this run |
|---|---|---|
| R7 Fable A1 (HIGH). An attempt logged before the input of the same certified time is read as made once ready | Moved to a stated limit, and the limit is enforced and pinned. K1 requires the second ordering; the verdict refuses the log | TIE1, TIE2. Mutants GCO2 and GCALL fail: `test_gated.py: an attempt recorded before the hand-over of the same time is rejected: accepted`. Not tied to the evidence (TIE-OMIT) |
| R7 Fable A2 (MEDIUM). Rule 3 as worded recorded a miss for every excused unit | Fixed | RULE3: `draft 8: ... counted=2, deadline_sent=5 \| InstAL missed=[]`; `draft 7: ... deadline_sent=5(before requests) \| InstAL missed=['u0']`. Mutant TW8 fails |
| R7 Fable A3 (LOW). "523" | Fixed | CE4: `different shares in 563 \| ... different shares in 746 \|\| the document says 563 and 746: True` |
| R7 Astra A1 (MEDIUM). The comparator labelled as draft 5's rule used the current rule | Fixed, pinned | CMP: `the test's comparator: miss=0, shares={} \|\| draft 7's comparator: miss=2, shares={'u': '2'}`. Mutant TCMP fails on case 3f |
| R7 Astra A2 (LOW). "523" | Fixed | CE4 |

Of the five: four fixed, one moved to a stated limit with the limit enforced. None partly fixed, none not fixed, none no longer applicable.

Round 7's list-B items that the draft answers:

| Item | Status | Evidence |
|---|---|---|
| The negative share given as a counterexample within scope | Fixed in the text | NEG2 |
| The order check called only by the test | Fixed: the verdict calls it, pinned (GCALL). Still on logs the tests have sorted (TCO) | `test_conflict_chain.py: passes` with an empty log |
| The second coalition was often the actual history | Fixed | DOC4: `second coalitions: 21 of which a non-empty proper part of the late units: 21` |
| The verdict's rejections had no tests | Partly fixed: five of seven pinned, two held only by the catch-all (GN, GO) | see list B |
| A refusal exactly at readiness ignored (GK) | Now pinned | `test_gated.py: AssertionError: the same attempt recorded after the hand-over is accepted and excuses` |
| An explicit empty charter rejected (PWa) | Fixed, not pinned (HPO) | `issue(p) -> ['issued(p),group', 'issued(p),state']` |

The earlier repairs, undone again:

| Repair | Status | Evidence |
|---|---|---|
| R6 Fable A1. Work and wait read separately | Pinned | GW fails: `a unit refused after its deadline is not charged again for the wait` |
| R6 Astra A1. Acts of authorities before requests | Pinned | GCO1 fails: `a request recorded before an authority's act of the same time is rejected: accepted` |
| R6 Fable A2. A key inside a norm; a charter naming nothing | Pinned | HK, HP, HP3 fail |
| R5. Forbidden intervals, not conflict intervals; the unit was ready; delivered at the ban's end; a refusal under a ban | Pinned | TA, GA, GC, GIN fail |

## Stop conditions

1. **A scenario has no trace or several.** Not met. 840 runs, one answer set each.
2. **A repair that needs a change to InstAL's semantics.** Not met. A1 is repaired in the text.
3. **K8 cannot be made without restating the norm rules inside the verdict layer.** Open, as the document says. For one way and one ban per duty the narrow case now stands on more than the document claims: 401 coalitions through InstAL here, besides its 51 and round 7's 83, and no third ordering over a small alphabet. Three remarks. The verdict needs a third input, the order of the log, which is a fact from F2 and no norm rule. A duty issued at a fixed time would be a fourth, a gate on delivery that is not a ban (DUTY-F); the tests avoid it by moving the issue with the unit. And nothing new bears on several ways per duty; neither option has been built.
4. **K11 cannot be made by events alone.** Not met, in the simple form only. Touched only by CB1, CB3, D8, GR1 and mutant UF.
5. **The defects are not converging.** Not met on my count. It needs two consecutive rounds in which neither tester's count falls. Round 7 was the first; in round 8 mine fell from 3 to 1, which is enough whatever the other count did. My record is now 21, 15, 6, 5, 6, 3, 3, 1. Three cautions.
   - The count is by the standard of rounds 2 to 7 and I moved nothing to change it. Two items could be read otherwise, as two could in round 7. T3-WR, read as inside the scope, would be a wrong record of who is late and a second finding; I kept it in list C because round 7 put two forbidden intervals on one duty there (TB, MB1) and K4 names a conflict that ends and starts again. GR1, read as a wrong rejection, would be a third; I kept it in list B because round 7 put a rejection that follows a declared convention there (PWa, CO1). With both counted the number would be 3 and would not have fallen. With either one it falls.
   - For the first time I found no wrong outcome of the models inside the scope, and nothing rated above low. The one finding is a sentence about the testers' own record.
   - The scope did not change this round. It was narrowed in draft 8's text in one respect that matters to the count: the negative share and several bans are now stated as outside it. That statement moved nothing out of my round-7 list A, where neither appeared.

List A by where and what: 1 in material written after round 7, 0 in older material; 0 wrong outcomes of the models, 1 false statement in the document.

CONFIRMED WRONG OUTCOMES (list A): 1
OF WHICH IN NEW MATERIAL: 1
CRITICAL: 0  HIGH: 0  MEDIUM: 0  LOW: 1

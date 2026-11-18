# Attack on the composition contract, round 9 (Fable)

Target: [Composition contract.md](/root/mic/reports/Composition contract.md), draft 9, and the models in /root/mic/model as changed after round 8.

Every scenario is in [attack_contract_fable_r9.py](/root/mic/model/attack_contract_fable_r9.py):

```
python3 /root/mic/model/attack_contract_fable_r9.py            # about thirty minutes on eight cores
python3 /root/mic/model/attack_contract_fable_r9.py --fast     # skips the mutation checks and the instrumented tests, about six minutes
```

The "derived" lines below are copied from that run. Identifiers such as SC3 are the labels the script prints. No existing file was changed; mutation checks work on copies. The ten documented checks all pass. I did not read the other tester's round-9 report.

## Summary

- **Count: 1 confirmed (list A), against 1, 3, 3, 6, 5, 6, 15 and 21 in the earlier rounds. My count did not fall.** Counted by finding, as in rounds 2 to 8, false statements in the document included. The one finding is a false statement about the testers' record, rated low, in text written after round 8. I found no wrong outcome of the models inside the stated scope.
- **The two-ways material held.** On 120 chains from a generator written for this round (units that dawdle after the reopening, retry inside the shut interval, attempt before readiness, have no budget or no work; bans with equal ends), the intervals of `noway` are the times both ways were banned, InstAL's `missed` equals the verdict's late units, and the re-run equals InstAL on all 287 coalitions, 161 of them histories that did not happen. The history of each coalition is laid out from the generator's record, not by the test's `play` and not from gated.py.
- **The two new lines change nothing else.** All 49 scenarios of test_instal.py that run give the same trace, state by state, with and without them, once the two new fluents are set aside. One answer set each.
- **The case stop condition 3 names does what the document says it would.** A ban that starts when a unit's input arrives: the fluent read from the actual trace is the wrong interval on the history in which the upstream unit complies, the re-run ends a day later than InstAL, and a day of loss moves from the late unit to "under prohibition". It is outside K8's scope and I do not count it.
- **Equal times: no fourth ordering in what K1 covers.** 192 chains on whole certified times, each run with the acts of authorities of one time in one order and in the reverse: no disagreement. The nearest thing is a delivery at exactly its deadline with the deadline recorded first, which K8 excludes in words and K1's list and the check do not (list B).
- **All of round 8's findings are repaired**, three in the text and one by exclusion with the exclusion enforced and pinned.
- **What the new material's evidence is worth is narrower than it reads.** In the test's 24 random chains, three units are excused, three deadlines are deferred, and the difference the new fluent exists for shows in one unit. `noway` also holds, permanently, for every other unit's duty where patterns tell units apart. Three wrong versions of the new rule pass the suite (list B).
- **K12 and K2 hold.** 1752 InstAL runs in the script, one answer set each.

The script's last full run printed: `1 A-DEFECT; 29 B-objection; 44 C-listed; 88 as specified; 0 NOT REPRODUCED`.

## List A: confirmed wrong outcomes or false statements inside the stated scope

### A1. LOW. Stop condition 5: what the last three rounds show

New material (the paragraph was written after round 8).

- **Statement.** Stop conditions, the record for condition 5: "What the last three rounds do show is narrower: no new defect in routes, in the norm layer's rules, in the ledger or in the evidence clause; the re-run verdict agreeing with InstAL wherever the testers compared them; and one recurring defect, the reading of times where order matters, found in a new place each round."
- **Scenario DOC5.** The document reports eight rounds, so the last three are rounds 6, 7 and 8. My round-6 report, its count lines and the two lines that say what was compared.
- **Derived.** `the document has the sentence: True; it reports eight rounds: True; fable_r6: ['CONFIRMED WRONG OUTCOMES (list A): 3', 'OF WHICH IN NEW MATERIAL: 2', 'CRITICAL: 0  HIGH: 1  MEDIUM: 1  LOW: 1'] || fable_r6: ["- **The amounts are wrong when a refusal does not count (HIGH).** gated.py then takes the unit's work to run to its receipt, which includes the time it waited under the ban. That wait is carried to every re-run as if it were work.", '- A coalition other than "every unit complies" whose re-run disagrees with InstAL: L1 and T49. "Every unit complies" itself: L3.'] || the status text of the same draft: True`
- **Correct statement.** In round 6 the comparison of the re-run with InstAL was that round's one high finding. On three histories the re-run disagreed with InstAL run on the history concerned; a unit half a day late was charged two days where InstAL gave a quarter. The document's own table "What was wrong in draft 6" records it. The re-run has agreed with InstAL in rounds 7 and 8, and the status text of the same draft says so correctly: "has now held for two rounds".
- **Why it counts.** The paragraph is the document's statement of what the record is worth, written to replace a count it calls a weak test. Of its three items, the one about the re-run verdict is false for a third of the period, and false in the direction of the reassurance. By the standard of rounds 4 to 8 a misreport of the testers' record is a false statement (round 4: the status line miscounted round 3; round 6: "40 histories that did not happen"; round 7: "523"; round 8: "the other both of two"). The other two items are true of rounds 6 to 8 as far as I can check them.
- **Repair.** "The last two rounds" for that item, or name round 6's finding.

## List B: design objections and claims stronger than their evidence

| Id | Clause | Objection | Derived |
|---|---|---|---|
| NW-OTHER, NW-CREATED | K8 two ways "They derive, for each duty, whether every declared way of meeting it is forbidden by an authority" | The adapter writes `isoblige` for every duty in every unit, and `matches` for a unit's own effects against every norm whatever its target. Where a duty's pattern tells units apart, `noway` holds in every other unit from the first event to the last, with no prohibition anywhere: a duty with no way in that institution has every way forbidden, vacuously. Where one pattern serves every duty, as in both test files, `noway(recv_u0)` in u1 follows u1's bans, and in a created unit it follows the inherited ban for a duty that K6 says is not copied. `conflicted` does neither, because it asks for the duty to be in force. Nothing reads these atoms; the tests read a unit's own duty only. Repair: a fact for the duties that target the unit, in the body of both rules | `no prohibition exists: states 0..4; noway(recv_u0) holds in u1 at [1, 2, 3, 4]; noway(recv_u1) holds in u0 at [1, 2, 3, 4]; noway(recv_u0) in u0 at []; conflicted(recv_u0) in u1 at []`; `noway(recv_u0) in the created unit c at [2]` |
| TIE-PD | K1 "So the contract requires, at equal certified times" and "The verdict takes the log, rejects one in another order"; K8 "orders a performance at exactly its deadline before the deadline" | A delivery at exactly its deadline with the deadline recorded first. The log satisfies both orderings K1 lists; InstAL records a miss and the verdict finds nobody late. K8 states the order that avoids it, and rule 3 states the opposite order for a deferred deadline, so the text taken together is right. But the order is not in K1's list and `canonical_order` cannot check it: the right place of a deadline among the requests of its time depends on whether it was deferred, which a kind and a time do not say. This is the recurring defect one step outside what K1 says it covers | `deadline then request: ... deadline(o) -> ['missed(o),unit']; request(e) -> ['emit(e),unit'] \| request then deadline: ... deadline(o) -> [] \| the verdict on the first log: shares={}, loss=0, under_prohibition=0, late=False` |
| DOC8, DOC9 | K8 two ways "Tested"; stop condition 3 "The re-run agrees with InstAL on 63 coalitions" | The numbers are as run. Behind them, in the 24 random chains: 56 units, 3 excused by a refusal, 3 deadlines deferred by a conflict, 7 of the 37 counterfactual histories in which some unit met both ways shut, and one unit in which the conflict intervals differ from `noway`, which is the difference the new fluent exists for. No unit delivers later than the reopening, retries, attempts before readiness or lacks funds: `attempts` takes an after-time and is never given one. And two of the "three named cases with amounts worked by hand" assert amounts; the first asserts an interval and a receipt | `STAT {'units': 56, 'late': 26, 'excused': 3, 'deferred': 3, 'refused_both': 6, 'cf': 37, 'cf_shut': 7}`; `named calls of through(): 3; of which assert the verdict's amounts: 2` |
| UNW2, UNW3, UNW4 | K8 two ways "forbidden by an authority"; K4 rules 3 and 4; K5 | Three wrong versions of the new rule pass test_two_ways.py: a unit's own prohibition counting in `noway`, an exempted way still counting as forbidden, a watched prohibition counting. The test has none of the three. The wrong versions that it does catch: `noway` asking for the duty to be in force, stopping at discharge, or holding when any one way is forbidden (UNW1, UNW6, UNW5) | `test_two_ways.py: passes` each |
| DUTY-F | K8 scope, restated: "The issue of a unit's duty may be tied to the arrival of its input, and then moves with it on a re-run" | Round 8's objection, answered in part. The scope now allows the issue to move. It does not exclude a duty issued at a fixed time later than the start, and from the actual history the two cannot be told apart. Held at its time, the issue is a gate on delivery that is neither a ban nor in `noway` | `re-run with u0 complying ends at 3 \|\| InstAL, issue moved to u1's new readiness: u1 delivers at 3 \|\| InstAL, issue held at 5: u1 refused at ['3', '3'], delivers at 5` |
| LOG, GLC-F, GLC-K | K1 "rejects one that lacks an entry for a refusal or receipt in the evidence"; "What was wrong in draft 8" row 4 | Round 8's two objections are answered and the answer is pinned for refusals (GLC, GLC-R fail). What remains: the check is one-sided, so the log may hold a request the evidence leaves out, and the same excused unit is then recorded as late; entries are matched as a set, so one request stands for two refusals of one time; a kind the function does not know is not ordered; the order is required across units, where it cannot matter. With receipts not checked, or with an entry of any kind accepted, both suites pass | `the evidence leaves out a refusal the log holds: shares={'u1': '0'}, loss=2, under_prohibition=2`; `kinds misspelt ...: None`; `test_gated.py + test_conflict_chain.py: passes` each |
| TCOP, TWO, TLG, TWE | "What was wrong in draft 8" row 1 "the test checks the order of the history it sends"; row 4 "a grant is its own kind" | True. The checks hold nothing by themselves: both tests sort their histories into the required order before checking it, so with the check removed they pass. What pins round 8's defect is the named case (TDI fails). With a grant labelled an act of an authority again in test_conflict_chain.py the test passes, because its histories never put a grant after a request of its time. With every two-ways chain issued from the start, test_two_ways.py passes | `passes` each |
| Stop 3 | Stop condition 3 "Not met on the two cases tried" | Supported for the two cases. But the way the second was met changes what the condition tests. The verdict's need was moved into the norm layer as a fluent made for it, which works for any obstacle that is a function of acts of authorities. The condition can now be met only by an obstacle that depends on what units did, and the first such case fails as predicted (SC3). The description of what would meet it does not say whether a solver run per coalition, the other course draft 8 named, would count as meeting it | see SC3 |
| DOC6, DOC11, DOC12 | Status "All three false statements were in the paragraph of draft 8 that reported round 7"; stop condition 5 "two of its out-of-scope items"; status "histories the testers built independently" | Loose, not counted. The two sentences stood in two adjacent paragraphs. Of the two items, one was a listed limit and one a design objection. Both figures, 155 and 401, are one tester's, as K8 says | `paragraph of 'could be counted as a fifth': 58; paragraph of 'the other both of two': 59` |
| GCT, GCN, GL, GEI2, UA, UF, TF, HP2, HPO, HA | K8, K0 | Unchanged from round 8: a log out of order of time accepted; ties compared between neighbours only; the last qualifying refusal taken for the first; an interval of no length; InstAL recording a miss in conflict; InstAL keeping a grant after a refusal; every random chain issued from the start; three weaker versions of the charter check | `passes` each |

## List C: limits the document already lists

| Id | Clause | Limit | Derived |
|---|---|---|---|
| SC3 | Stop condition 3 "A prohibition triggered by a unit's own earlier act would be such a case. It is outside K8's scope and has not been tried"; K8 scope | Now tried. Both ways of u1 are shut for two days from the arrival of its input; u0 is a day late. See priority 1 | `gated: late=['u0'], shares={'u0': '0'}, loss=2, under_prohibition=2; re-run with u0 complying ends at 5 \|\| InstAL on that history, the ban following the arrival: noway=['2..4'], u1 refused at ['3', '3'], delivers at 4` |
| I1, NW-UNREACH | K4 "An unreachable way counts as a way" | The open wrong outcome, unchanged, and it carries over to the new fluent: the unit cannot deliver and `noway` is false | `deadline(o) -> ['missed(o),unit']`; `request(am) -> ['refuse(am),u0'] noway=False; deadline(recv_u0) -> ['missed(recv_u0),u0'] noway=False` |
| NW-EXEMPT, NW-OWNEX, NW-FUNDS, NW-THREE | K8 two ways "More than two ways, ways with different owners, exemptions, and funds are untried" | With three ways the conjunction holds. An exemption opens a way and `noway` follows it. A way without funds leaves the unit unable to deliver and `noway` false. A unit ranked above the authority sets a ban aside by its own exemption, so `noway` then changes on an act of a unit and "depends only on acts of authorities" needs that case excluded | `ownissue(ex, officer) -> ['ownissued(ex),u0'] noway=False conflicted=False; request(road_u0) -> ['emit(road_u0),u0']` |
| T3-WR | K8 open "A ban withdrawn and issued again at one certified time" | As stated. One thing the text does not say: the only placing on which InstAL records the miss is the one K1's new ordering forbids | `Deadline after both acts of time 3: ['conflict(recv_u0),u0'], canonical_order: None \|\| deadline between them: ['missed(recv_u0),u0'], canonical_order: ValueError: at time 3 a deadline precedes an act of an authority` |
| GM, GN, GO, UC | K8 "a tester found no evidence on which that check fires ... two of the rejections still pass their tests when deleted"; rule 3 "A tester changed InstAL so that no duty is ever in conflict" | All as stated: exactly two of the five deleted rejections pass | `passes` each |
| RW1, RW2 | K12 "the log" | A relayed withdrawal, or a relayed issue below a declining authority, observed directly takes effect | `relayed(p) -> ['issued(p),group']; request(ord_x) -> ['refuse(ord_x),unit']` |
| N3 to N15 (8), M11 to M15, VT, VI, VCR2, VPR, NR5 to NR7, VC2 to VC4, VCY2, V2C, NE3 to NE5, NA, VE | K12 status; K0 "The list below" | All 30 wrong versions still pass, as stated | `passes` each |
| K8 open, not run | K8 | Conventions; a unit that waits without attempting; who bears the loss under prohibition; two duties per hand-over; trees; several bans per duty; no layer places a deadline | |

## Priorities 1 to 5

1. **The two-ways material and stop condition 3.**
   - *The rule.* `noway` holds exactly while every declared way is forbidden by an authority, before the duty is issued and after it is discharged (NW-BOTH, NW-BEFORE, NW-ONE). A watched prohibition does not count (NW-WATCH). The unit's own prohibition does not count, so a unit that shut its own second way cannot deliver, has `noway` false and is recorded as missing, which is rule 4 as stated (NW-OWN). Under an exemption a way is open (NW-EXEMPT). It also holds where it means nothing, for other units' duties (NW-OTHER, list B). The adapter refuses a duty with no declared way of its own unit, so that case arises only there.
   - *The intervals, who is late, every coalition.* TW-RND: `{'units': 306, 'late': 119, 'excused': 11, 'shut': 133, 'differs': 79, 'after': 18, 'coalitions': 287, 'changed': 161, 'cf_shut': 53, 'runs': 527}; disagreements: 0`. In 79 units the conflict intervals differ from `noway` (the test has one); in 53 counterfactual histories some unit met both ways shut (the test has 7); 18 units delivered later than the reopening (the test has none). Deadlines are placed from rule 3's text.
   - *Other outcomes.* NW-DIFF: `calls of the adapter: 74; rejected in both: 25; run in both: 49; answer sets per run: [1]; states that differ once someway and noway are set aside: 0`. STRAT: `16 non-inertial fluents, 17 dependencies, no cycle`, which I also counted by hand.
   - *"Evaluated by InstAL and handed over as a fact; the verdict layer holds no norm rule beyond the lookup it already had."* True. gated.py is the same for one way and two, and knows nothing of ways. Handing it the conflict intervals in place of `noway` fails the test even with the assertions on the intervals themselves taken out, on the comparison of a re-run with InstAL (TWC, TWC2). Two qualifications. A road refusal while rail is open reaches the verdict among the refusals and is set aside by the same lookup, correctly. And the placing of deadlines in the test still reads the conflict intervals, as K8 says under "open".
   - *The case the document names (SC3).* An authority shuts both of u1's ways for two days from the arrival of its input. u0 has a budget of 2 and delivers at 3. Actual: `noway(recv_u1)=['3..5'], u1 refused at ['4', '4'] and delivers at 5`. The verdict: `late=['u0'], shares={'u0': '0'}, loss=2, under_prohibition=2; re-run with u0 complying ends at 5`. InstAL on the history in which u0 complies: `noway=['2..4'], u1 refused at ['3', '3'], delivers at 4`. So the loss with u0 complying is one day, not two; u0 owes one day and one arises under prohibition. The re-run is right only for a ban held at its actual time (`delivers at 5`), and becomes right if the interval is moved with the arrival (`4`), which is the norm layer's trigger rule applied in the verdict layer. No fluent of the actual trace holds `2..4`.
   - *Is the verdict on stop condition 3 supported?* For the two cases, yes. "Open beyond them" is right and, one step beyond them, the condition is met unless a solver run per coalition is accepted. The description of what would meet it is right. Two additions. The case is not exotic: the tests already move one act of an authority with a unit's act, the issue of a duty at readiness, and that is harmless only because the test's history builder moves it and a duty not in force is never the binding obstacle there (DUTY-F). And the second case was met by exporting a fluent made for the verdict, which any static obstacle allows; the document says as much ("a new kind of obstacle to delivery would need a new exported fluent"), and it means the condition now tests one thing only.
2. **Equal times after the third ordering.**
   - TIE-ENUM: `chains: 192; with two acts of authorities at one time: 192; late units: 144; excused: 52; InstAL runs: 1128; disagreements between InstAL's missed, the verdict's late units and the generator's rule, between the re-run and InstAL, or between the two orders: 0`. Two bans can make a state with no way open between a withdrawal and an issue of one time; no deadline or request can stand in it, and the reading drops an interval of no length.
   - Round 8's case is refused and its mirror accepted (R8-AST1): `canonical_order: ValueError: at time 0 a deadline precedes an act of an authority || issue first: ... deadline(o) -> ['missed(o),unit'] ... canonical_order: None`.
   - No valid log was refused: none of the 312 chains' logs in TW-RND and TIE-ENUM, with attempts before readiness and several requests of one unit at one time. What `canonical_order` and `log_covers` refuse beyond need or accept without checking is in list B (LOG).
   - A wrong record of who is late on a log in K1's two orders exists only with the deadline before a delivery of its own time (TIE-PD), which K8 excludes in words.
3. **Round 8's items.** See the tables below.
4. **The document.**
   - As run: 74 scenarios and 25 rejections; 5 named cases, 40 chains, 61 coalitions and 51; 3 named cases, 24 chains, 63 coalitions and 37; 16 derived fluents (DOC1, DOC7, DOC8, STRAT).
   - As the reports have them: the record 21, 15, 6, 5, 6, 3, 3, 1 and 8, 6, 3, 2, 2, 1, 2, 3; 60, 87, 94, 89, 71, 182, 388 and 840 runs; 155 coalitions from 64 chains with 83; 401 from 160; 556; 125260 (DOC2, DOC3).
   - True by run: the status text apart from the loose phrases in list B; K1's three orderings, "rejects one that lacks an entry" and "a grant ... may fall anywhere"; K8's scope as far as it goes; every sentence of the two-ways paragraph; every row of "What was wrong in draft 8"; "neither count fell in round 5 or in round 7; in rounds 6 and 8 at least one fell"; "two of the rejections still pass their tests when deleted".
   - False: what the last three rounds show about the re-run verdict (A1).
5. **K12, K2 and the adapter.** No defect. Round 8's 22 route scenarios give the same results (routes of three and four with each intermediate declining, opposite directions, a routed permission, created units to depth two, a declined routed exemption). An explicit empty charter is accepted (PWa). `1752 runs: min 1, max 1`.

## Round 8's list A, both testers, and the earlier repairs

"Pinned" means that I undid the repair in a copy and a maintained test failed.

| Finding | Status | Evidence from this run |
|---|---|---|
| R8 Astra A1 (HIGH). A deadline sent before the issue of its own duty at the same certified time records no miss | Moved to a stated limit, and the limit is enforced and pinned. K1 requires the order; `canonical_order` refuses the log; the test has the case | R8-AST1. GCO3 fails: `test_gated.py: a deadline recorded before an authority's act of the same time: accepted`. TDI fails: `AssertionError: {'missed': set(), ...` |
| R8 Fable A1 (LOW). "The other both of two" | Fixed | DOC-R8: `'of its five findings, two were in material written after round 6, not four as draft 8 said': True` |
| R8 Astra A2 (LOW). The same sentence | Fixed | DOC-R8 |
| R8 Astra A3 (LOW). "Could be counted as a fifth" | Fixed | DOC-R8: `'would have made its count four, not five': True` |

Of the four (three distinct): three fixed, one moved to a stated limit with the limit enforced. None partly fixed, none not fixed, none no longer applicable.

Round 8's list-B items that the draft answers:

| Item | Status | Evidence |
|---|---|---|
| Nothing tied the log to the evidence (TIE-OMIT, TCO) | Fixed for the entries the evidence holds, pinned for refusals (GLC, GLC-R, TCO, TWL fail); not for receipts, and one-sided (GLC-F, LOG) | `empty log: ValueError: the log has no request of u0 at 3` |
| A grant labelled an act of an authority (GR1) | Fixed, pinned in gated.py (GGR fails); the label in test_conflict_chain.py is not pinned (TLG) | `request, grant, request at one time: None` |
| "Acts of authorities do not depend on what units do" (DUTY-F) | Partly fixed: the scope allows the issue to move; a duty issued at a fixed later time is not excluded | DUTY-F |
| A delivery before the issue of the duty it triggers is refused (DUTY-T) | Not fixed; a stated consequence of K1's first ordering | not run again |
| "Each has a test" (GN, GO) | Moved to a stated limit: the text now says two rejections pass when deleted | GN, GO pass; GRB, GNB, GEI fail |
| "The verdict layer needs two things" | Not fixed: K8's "What it is given" still does not name the log | by reading |

The earlier repairs, undone again:

| Repair | Status | Evidence |
|---|---|---|
| R7 Fable A1. A request before a request of an earlier unit | Pinned | GCO2 fails |
| R7 Fable A2. Rule 3's first case | Pinned, and in the two-ways test as well | TW8, TWX fail |
| R7 Astra A1. The comparator | Pinned | TCMP fails |
| R6 Fable A1. Work and wait read separately | Pinned | GW fails |
| R6 Astra A1. Acts of authorities before requests | Pinned | GCO1 fails |
| R6 Fable A2. A key inside a norm; a charter naming nothing | Pinned | HK, HP, HP3 fail |
| R5. Forbidden intervals, not conflict intervals; the unit was ready; delivered at the ban's end; a refusal under a ban | Pinned | TA, GA, GC, GIN fail |
| K0 "Detected, by the testers' runs" | All ten detected | M1, M1b, M6, NR1 to NR4, N8, N10, N9 fail |

## Stop conditions

1. **A scenario has no trace or several.** Not met. 1752 runs, one answer set each, the two new fluents included.
2. **A repair that needs a change to InstAL's semantics.** Not met. A1 is repaired in the text.
3. **K8 cannot be made without restating the norm rules inside the verdict layer.** The document's verdict, "not met on the two cases tried; open beyond them", is supported. For two ways it now stands on 287 coalitions through InstAL here besides the test's 63, with no change to any other trace. Beyond the two cases it is met by the first case tried (SC3): a prohibition that follows a unit's act cannot be read from the actual trace, and the repair is the trigger rule in the verdict layer or a solver run per coalition. That case is outside the scope by the scope's own sentence on prohibitions. So the reading I would give is: not met inside the scope, and the scope's edge is where it would be met.
4. **K11 cannot be made by events alone.** Not met, in the simple form only. Touched here only by NW-FUNDS and mutant UF.
5. **The defects are not converging.** My count did not fall: 1 in round 8, 1 in round 9. My record is now 21, 15, 6, 5, 6, 3, 3, 1, 1. The condition needs two consecutive rounds in which neither tester's count falls. Round 8 was not one, because mine fell. Whether round 9 is one depends on the other tester's count, which I have not seen. Three cautions.
   - The count is by the standard of rounds 2 to 8 and I moved nothing to change it. One item could be read otherwise. TIE-PD, read as inside K1's claim and not as excluded by K8's sentence, would be a wrong record of who is late and a second finding. I kept it in list B because the document states the order that avoids it, and round 8 kept T3-WR out on the same kind of ground. NW-OTHER is a fluent that holds where it means nothing; nothing reads it, so it is no outcome.
   - For the second round running I found no wrong outcome of the models inside the scope and nothing above low. The one finding is again a sentence about the testers' record, in a paragraph written to qualify that record. Round 8's was the same.
   - The scope changed this round in two ways that bear on the count. Two ways per duty were brought in, and held. A ban withdrawn and issued again at one time, and prohibitions that depend on what units do, were written out. Neither moved anything out of my round-8 list A.

List A by where and what: 1 in material written after round 8, 0 in older material; 0 wrong outcomes of the models, 1 false statement in the document.

CONFIRMED WRONG OUTCOMES (list A): 1
OF WHICH IN NEW MATERIAL: 1
CRITICAL: 0  HIGH: 0  MEDIUM: 0  LOW: 1

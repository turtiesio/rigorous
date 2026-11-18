# Attack on the composition contract, round 6 (Fable)

Target: [Composition contract.md](/root/mic/reports/Composition contract.md), draft 6, and the models in /root/mic/model as changed after round 5.

Every scenario is in [attack_contract_fable_r6.py](/root/mic/model/attack_contract_fable_r6.py):

```
python3 /root/mic/model/attack_contract_fable_r6.py            # about twenty minutes on eight cores
python3 /root/mic/model/attack_contract_fable_r6.py --fast     # skips the mutation checks, about six minutes
```

The "derived" lines below are copied from that run. Identifiers such as L1 are the labels the script prints. No existing file was changed; mutation checks work on copies. The nine documented checks all pass. I did not read the other tester's round-6 report.

## Summary

- **Count: 3 confirmed (list A), against 6, 5, 6, 15 and 21 in the earlier rounds. My count fell.** Counted by finding, as in rounds 2 to 5. Two of the three are in material written after round 5, including the only high one. One is in material that existed in round 5.
- **Who is late now holds (K8 rules 1 and 2).** On 40 chains from a wider generator than the test's, and on every boundary I tried, the rule, InstAL's `missed` and gated.py's players agree. Round 5's three high findings in K8 are repaired.
- **The amounts are wrong when a refusal does not count (HIGH).** gated.py then takes the unit's work to run to its receipt, which includes the time it waited under the ban. That wait is carried to every re-run as if it were work. A unit half a day late is charged two days where InstAL, run on the history in question, gives a quarter. A unit within its budget makes two thirds of an upstream unit's loss appear to arise under prohibition.
- **The adapter still changes outcomes silently on misspelt input it does not check (MEDIUM).** Round 5's four cases are rejected now. A misspelt key inside a norm, a charter that names a norm or an authority that does not exist, and a self-declared attribute that nothing reads are accepted.
- **Stop condition 3 says 40 histories that did not happen; six of them did (LOW).**
- **K12 holds.** No way round a declining authority by acts of authorities. **K2 holds.** 182 InstAL runs, one answer set each.
- **Several repairs are correct and held in place by nothing.** The test passes with the conflict intervals handed to the verdict again (round 5's A3), with the readiness condition deleted (round 5's A2), and with InstAL changed so that no duty is ever in conflict.

The script marks 16 scenarios as A-DEFECT. They group into the three findings below.

## List A: confirmed wrong outcomes or false statements inside the stated scope

### A1. HIGH. K8: a unit's work time contains its wait under a ban whenever its refusal does not count

New material (`Unit.counted_refusal` and `Unit.work` in gated.py, written after round 5).

- **Statements.** K8: "gated.py answers 'what if these units had complied' by running the chain again." "The loss that remains when every unit complies is recorded as arising under prohibition." "A length fixed from the actual run assumes a unit's delay is the same whatever the others do. A prohibition holds in absolute time, so that is false."
- **Scope.** A chain of two, one duty per unit, one way per duty, one ban, acts of authorities fixed. All as K8 states.
- **Scenario L1.** u0: budget 2, delivers at 4. u1: budget 1, ready at 4, due at 5. A ban on u1 from 4.5 to 7. u1 attempts at 5.5, is refused, delivers at 7.
- **Derived (L1, gated.py alone).** `players=['u0', 'u1'], shares={'u0': '2', 'u1': '2'}, loss=4, under_prohibition=0; actual loss=4; work=['4', '3']; counted=['None', 'None'] || kept [] -> final receipt 3; kept ['u0'] -> final receipt 7; kept ['u1'] -> final receipt 7; kept ['u0', 'u1'] -> final receipt 7`
- **Derived (L1-act, the same through InstAL).** `u1: ready=4, due=5, receipt=7, refusals=['11/2'], forbidden=['9/2..7'], conflict=['9/2..7'], counted=None, deadline_sent=7(before requests) | InstAL missed=['u0', 'u1'] | gated: players=['u0', 'u1'], shares={'u0': '2', 'u1': '2'}, loss=4, under_prohibition=0; actual_loss=4`
- **Derived (L1-cf, InstAL on the history "u0 complies, u1 as it was").** `u0: ready=0, due=2, receipt=2 ... | u1: ready=2, due=3, receipt=7/2, refusals=[], forbidden=['9/2..7'], conflict=[] ... | InstAL missed=['u1'] | gated: players=['u1'], shares={'u1': '1/2'}, loss=1/2, under_prohibition=0; actual_loss=1/2`
- **Correct outcome.** u0 pays 3.75 and u1 0.25. Had u0 complied, u1 would have had its input at 2, attempted a day and a half later at 3.5, a day before the ban, and delivered: half a day of loss, not four. InstAL says so, and so does gated.py when that history is given to it as evidence. Its re-run for the same coalition says 7.
- **Why.** u1's refusal came after its deadline, so it does not count, and `work` then ends at the receipt: 7 - 4 = 3 days, of which a day and a half is the wait from the refusal to the end of the ban. The re-run moves those three days to a start at 2 and lands in the ban again. Whether a refusal excuses the unit and where the unit's work ended are decided by one expression. Round 5's code, as that round's report describes it, ended work at the first refusal whatever its time; the condition "by the deadline" was added to both at once.
- **Scenario L3,** the same defect in the "every unit complies" re-run. u0: budget 2, delivers at 3. u1: budget 3.5, ready at 3. A ban on u1 from 3.5 to 6. u1 is refused at 4 and delivers at 6.25, within its budget.
- **Derived (L3).** `players=['u0'], shares={'u0': '1/4'}, loss=3/4, under_prohibition=1/2; actual loss=3/4; work=['3', '13/4']; counted=['None', 'None'] || kept [] -> final receipt 6; kept ['u0'] -> final receipt 25/4`
- **Derived (L3-cf, InstAL on "every unit complies", u1 doing from its new readiness what its log shows).** `u1: ready=2, due=11/2, receipt=13/4, refusals=[], forbidden=['7/2..6'], conflict=[] ... | InstAL missed=[] | gated: players=[], shares={}, loss=0, under_prohibition=0; actual_loss=0`
- **Correct outcome.** u0 pays 0.75 and nothing arises under prohibition. u1 was not late and is not a player. Because it delivered a quarter of a day after the ban ended, its refusal does not count and two days of waiting become work. Two thirds of the loss u0 caused is moved to the claimant. K8 calls this convention "harsh on a unit that lost half a day to a ban and then took two". Here that unit pays nothing and the harshness falls elsewhere.
- **Scenario L2,** L3 with u1's budget 2: `shares={'u0': '1/8', 'u1': '1/8'}, loss=9/4, under_prohibition=2`. Two of 2.25 days are recorded as arising under prohibition; with u0 on time there is no ban in u1's way.
- **Scenario T49,** chain 49 of the test's own generator with its seed (the test stops at 39). `gated: players=['u0', 'u1'], shares={'u0': '9/4', 'u1': '9/4'}, loss=9/2`. InstAL on "u0 complies, u1 as it was": `u1: ready=3, due=7, receipt=8`, a loss of one day. Correct: u0 4, u1 0.5.
- **Why the tests did not see it.**
  - K8 lists a coalition other than "everyone complies" as not tested. That is accurate, and it is where L1 and T49 are.
  - The "everyone complies" comparison cannot see L3. The test builds the history it gives InstAL from gated.py's own work times (`first = ready + min(gu.work, gu.budget)`). L3-cft runs that history: u1 "first attempts" at 5.25 and InstAL agrees with the re-run. The comparison checks `gate()` against InstAL's refusals. It checks nothing about what a unit would have done.
  - GEN: `the test's 40 chains: units whose work, as gated.py takes it, includes a wait under a ban: 11 (refused after the deadline and delivered at the ban's end: 8; delivered later than the ban's end: 3) ... shares that change if a unit refused after its deadline is taken to have worked until that refusal: 0 of 40 || the same generator continued to 4000 chains: 640 hold such a unit, shares change in 128; the first is chain 49`.
  - Mutant GL replaces `work` by the repaired version for units refused after their deadline. Both suites pass. The tests cannot tell the two apart.
- **A note on classification.** "A unit's work ends at a refusal that counts, and otherwise at its receipt" is a declared convention, and K8 says the conventions are untested. I count this as a wrong outcome and not as an objection for three reasons. The amounts contradict InstAL on the histories the re-run claims to stand for, which is the test I applied to round 5's A3. The convention is the fixed length K8 itself proves wrong two paragraphs earlier. And the open item K8 does list, a unit that waits without attempting, is a case with no log entry; here the log holds the attempt and the verdict is given it. If the author reads that open item as covering every late unit, this finding moves to list C and the count is 2.
- **Repair.** Separate the two questions. A refusal excuses under rule 1 as now. A unit's work ends at its first refusal made at or after readiness inside a forbidden interval, whatever the deadline, and the time from the end of that interval to the receipt is further work done after the gate. Then compare at least one coalition re-run with InstAL on a history built from the logged attempts, not from the verdict's own work times. No norm rule is needed in the verdict layer for this.

### A2. MEDIUM. K0: misspelt input the adapter does not check still changes outcomes silently

Material that existed in round 5. Round 5's A5, partly fixed.

- **Statement.** K0: "It rejects input it would otherwise have to guess about." "It rejects a scenario key it does not know ... and an event that names a norm, effect or procedure that does not exist. Other malformed shapes ... still end in an unhandled error and not in a message."
- **What is fixed.** A misspelt norm or event name in an event, a misspelt scenario key, a fractional cost (W4u, W4v, W16, W16b, W2d). Four of round 5's unhandled errors now give a message (W1, W1c, W4, W4e). The other eleven are now a stated limit (list C).
- **Scenario UV1.** A norm marked `ultra_vire` for `ultra_vires`.
- **Derived.** `issue(a) -> ['issued(a),ministry']; issue(p) -> ['issued(p),ministry']; request(ord_x) -> ['refuse(ord_x),unit']`. Control UVc: `issue(p) -> ['ultravires(p),ministry']; request(ord_x) -> ['emit(ord_x),unit']`.
- **Scenario PW2.** A charter `power={"ministry": {"a", "pp"}}` for a norm called p.
- **Derived.** `issue(p) -> ['ultravires(p),ministry']; request(ord_x) -> ['emit(ord_x),unit']`
- **Scenario PW1.** A charter for `ministri`: `issue(a) -> ['ultravires(a),ministry']; issue(p) -> ['ultravires(p),ministry']; request(ord_x) -> ['refuse(ord_x),unit']`
- **Scenarios SD1, SD2.** `self_declared=["vendr"]`, and `self_declared="vendor"` given as a string: `request(ord_x) -> ['refuse(ord_x),unit']`. Control SDc: `request(ord_x) -> ['breach(ord_x,p),unit', 'emit(ord_x),unit']`.
- **Scenario W2e.** A cost of `Decimal('2.5')`: `AssertionError: the cost of e is not a finite number of zero or more`. The message is false. On its own this would be low.
- **Correct outcome.** Rejected with a message, as the same slips are when made in an event or a scenario key. In each case the scenario says one thing (beyond the charter; within the charter; watched) and the adapter runs another. These are neither unhandled errors nor messages, so the last sentence quoted above does not cover them.
- **Repair.** Check the keys of each norm against the six it may have. Check that `power` names authorities and norms that exist, and that `self_declared` is a list of attributes some pattern reads. Accept any `numbers.Real` or `Decimal`.

### A3. LOW. Stop condition 3 and the claims table: "40 histories that did not happen"

New material (the rewritten stop condition 3 and the K8 row of the claims table).

- **Statement.** Stop condition 3: "its re-run agrees with InstAL on 40 histories that did not happen." Claims table: "40 chains with bans through InstAL, each with one counterfactual history."
- **Scenario DOC4,** the test's generator replayed with its seed.
- **Derived.** `'every unit complies' histories with the same receipts as the actual run: 6, of which with the same attempts too: 4`. The test prints the same: `in all 40; in 34 of them that history differs from the actual one`.
- **What is false.** In six of the 40 every unit was within budget or excused, and the history given to InstAL is the one that happened. K8 has it right ("in 34 that history differs"). The sentence that decides stop condition 3 does not.
- **Repair.** Write 34 in both places.

## List B: design objections and claims stronger than their evidence

| Id | Clause | Objection | Derived |
|---|---|---|---|
| TA | "What was wrong in draft 5", K8 row 3; K8 "the trace's readings" | With the verdict handed the conflict intervals again, which was round 5's A3, the test passes on all 40 chains, the 20 with late duties included. The repair is right (DR) and the test does not hold it | `test_conflict_chain.py: passes` |
| GA, G-PRE | K8 rule 1 "the unit was ready"; test_gated.py "an attempt logged before the unit was ready is not counted" | With the readiness condition deleted both suites pass. In the test's own case the unit delivers three days after the ban ended, so the refusal fails the last condition either way. G-PRE is a case that would tell | `test_gated.py + test_conflict_chain.py: passes` |
| UA, UC | K8 rule 2 "a duty in conflict at its deadline falls due when the conflict ends"; K8 "the units InstAL records as missing" | With InstAL changed so that a deadline in conflict records a miss, or so that no duty is ever in conflict, the test passes. Who is late is "delivered after the deadline with no counted refusal", with or without the exception. The exception only undoes K4's rule 4, and the comparison is not evidence about the norm layer's conflict rule | `test_conflict_chain.py: passes` each |
| G7, GM, GN, GO | K8 "It refuses evidence it cannot reproduce: ... or a re-run that does not end at the actual receipt" | The third refusal cannot fire once the first two pass: 60000 chains of arbitrary evidence, none. Removing it changes nothing. Removing either of the others also passes both suites (the first because the third then catches the test's one case; the second because nothing tests it) | `rejected because the re-run does not reproduce the receipt 0`; `passes` each |
| G-RDY | K8 rule 1 "the unit was ready"; "A unit's work ends at a refusal that counts" | A request costs nothing and the norm layer does not ask whether there was anything to deliver. A unit that logs its attempt at the instant its input arrives has zero work; logged two days later the same conduct gives the opposite split. The unit is excused either way and decides what the late unit upstream pays | `attempt logged at 3: shares={'u0': '6'}, loss=6, under_prohibition=0 ... \|\| attempt logged at 5: shares={'u0': '0'}, loss=6, under_prohibition=6` |
| CB3 | K8 rule 1 | A unit with no funds at all, refused under the ban and funded at the instant it ends, has a refusal that counts. The trace carries no reason for a refusal | `counted=1, deadline_sent=4 \| InstAL missed=[]` |
| CB2 | K8 rule 1, K11 "A refusal drops the grant" | A funded unit refused in time loses its grant by K11's rule. If the ledger grants again half a day after the ban ends, the refusal no longer counts and the unit is recorded as missing | `counted=None, deadline_sent=4(before requests) \| InstAL missed=['u0'] \| gated: players=['u0'], shares={'u0': '1/2'}` |
| GEN | K8 "It draws what round 5 showed the old generator never drew" | True for the three kinds named. The 40 chains hold none on which A1 changes a share; the generator's first is its 50th | see A1 |
| DOC6 | Status "given only facts that do not depend on what the units do"; K8 "the same on every re-run" | True of the intervals. Not of the work time, which is also held fixed | `Unit.work for u1 in L1: 3 (receipt 7 less readiness 4); the same unit, had its input arrived at 2: 3/2` |
| NE3, NE4, NE5, NA | K0, the checks written after round 5 | Four wrong versions pass: no check of effects in events; none of `ownissue`; only `issue` checked among the events that name a norm; no identifier check on authorities. The other four are covered (NK, NE1, NE2, NC fail) | `test_instal.py: passes` each |
| M11 to M15, VT, VI | K0 "The current position ... Not detected" | The list is not complete. Draft 5 listed five rules of K4 and K5 and two rejections as undetected. All seven are still undetected and draft 6's list has dropped them. K0 names three route versions; K12 says eight, and eight pass (list C) | `passes` each |
| GK, UF, TF | K8 | Also undetected: a refusal exactly at readiness not counting; InstAL keeping a grant after a refusal; every duty issued at the start | `passes` each |
| G6d | K8 "It refuses evidence it cannot reproduce" | A negative budget, an interval that ends before it starts and a refusal after the receipt are accepted. No verdict I found is changed by them | see the run |
| W18, Vb6, RC4, AU1 | K0 | As in round 5: a rank demanded where no exemption exists; `blocked` takes a set and not a frozenset; a direct norm with no edge to its target is accepted and binds nobody. New: `authorities` given as a string is read as its letters | `AssertionError` twice; `issue(p) -> ['issued(p),ministry']; request(ov) -> ['emit(ov),v']` |
| Stop 3 | Stop condition 3 "which are facts and the same on every re-run" | The sentence rests on the intervals. A1 shows the verdict also needs each unit's own time free of any wait, on every re-run. That comes from the log and needs no norm rule, so it does not move the condition, but it is a second thing the narrow case depends on | see A1 |

## List C: limits the document already lists

| Id | Clause | Limit | Derived |
|---|---|---|---|
| I1, I1c | K4 | An unreachable way counts as a way. Unchanged and stated | `deadline(o) -> ['missed(o),unit']`; one effect fewer: `deadline(o) -> ['conflict(o),unit']` |
| D5, LI3 | K8 open | A unit that waits without attempting is late | `deadline_sent=6(before requests) \| InstAL missed=['u0'] \| gated: players=['u0'], shares={'u0': '0'}` |
| G6c, TB | K8 open "several bans on one duty" | A unit refused under a first ban, which does not deliver when it ends and is refused again under a second, is excused. Round 5's negative length is gone | `counted=5/2, deadline_sent=6 \| InstAL missed=[]` |
| RW1, RW2 | K12 "the log" | A relayed withdrawal, or a relayed issue below a declining authority, observed directly takes effect | `relayed(p) -> ['issued(p),group']; request(ord_x) -> ['refuse(ord_x),unit']` |
| N3, N4, N5, N6, N7, N11, N14, N15 | K12 status, K0 | The eight wrong versions of the route code still pass | `passes` each |
| NR5, NR6, NR7, VC2, VC3, VC4, VCY2, V2C, VE, VCR2, VPR | K0 "Not detected" | All eleven still pass, as stated | `passes` each |
| W1d, W3, W3g, W3e, W7, W8, W9d, W9e, W13b, W13c, Vr2, Vp | K0 "Other malformed shapes ... still end in an unhandled error" | Twelve unhandled errors. Eleven were in round 5's list A and are moved here by the new sentence; W1d is a new probe | `InstalParserSyntaxError`, `InstalParserLexerError`, `TypeError`, `KeyError`, `AttributeError`, `ValueError`, `IndexError` |
| K8 open | K8 | Several ways per duty; who bears the loss under prohibition; two duties per hand-over; trees; no layer places a deadline | not run |

## Priorities 1 to 6

1. **gated.py inside its scope.**
   - Which refusals count is right on every boundary: a refusal exactly at readiness, at the deadline and at an interval's start counts; one at the instant the interval ends does not; zero budgets, an early unit, a chain of one and an empty chain behave (G6, D1 to D4z).
   - I found no evidence it rejects that it should accept. Each rejection in G6b is for the stated reason.
   - It accepts evidence whose re-run is not what the units would have done: A1.
   - A coalition other than "every unit complies" whose re-run disagrees with InstAL: L1 and T49. "Every unit complies" itself: L3.
   - Round 5's five scenarios against gated.py are repaired (G1 to G5). The two counterexamples and the continuity counterexample are right by hand and against draft 5's stated rule written independently; 523 reproduces (CE1 to CE4).
2. **Who is late, and test_conflict_chain.py.**
   - RND, 40 chains from a wider generator through InstAL: `{'chains': 40, 'units': 81, 'late': 48, 'excused': 7, 'before_ready': 5, 'unfunded': 11, 'several': 12, 'at_readiness': 9}; disagreements between the rule, InstAL's missed and gated.py's players, or evidence gated.py refuses: 0`.
   - The reading of refusals, forbidden intervals and conflict intervals is aligned with the trace. A duty issued at readiness gives a conflict that starts at readiness and a forbidden interval that starts with the ban (LI1); a ban that ends at the instant of issue leaves no conflict (LI2); an exemption ends the forbidden interval (X1); a watched ban gives none (WP).
   - Each of the three comparisons can fail. The first fails under GB, TB and TD. The second fails under UE (`AssertionError: 0`, the trial number). The third fails under GF2, a wrong `gate()` run against test_conflict_chain.py alone: InstAL refuses at the instant a ban starts where the re-run delivers.
   - What the comparisons cannot see is in list B: TA, GA, UA, UC.
3. **Round-5 list A.** See the table below.
4. **The round-5 adapter changes.**
   - Valid input rejected: a Decimal cost (A2). Events written with spaces are accepted and take effect (W4s). Authorities called `min_a` and `ministry2` are accepted (W1f).
   - Invalid input accepted: A2.
   - A rejection for the wrong reason: none in the suite. Each of its 23 rejections fires for the reason in its name (REJ). Of six more in W4t, one message is off: `issue(p, a)` is reported as naming no norm.
   - Both ownership mutations and the foreign-match mutation fail the suite (M1, M1b, M6). The route mutations are as K12 and K0 say.
5. **K12 and K2.** No defect. Routes of three with either intermediate declining; two norms on one route with one declined; two norms routed in opposite directions with each authority declining the other's; a charter power over another authority's norm; issue twice, withdraw and issue again; created units to depth two; a declined routed exemption (R3a to RXB). 182 runs, one answer set each.
6. **The document.** 72 scenarios and 23 rejections, 25 chains, 300 and 2000 chains, 523, 40 chains and 34 are as the runs give them (DOC1, DOC3). The record for stop condition 5 and the account of where the round-5 findings were are as the testers wrote them (DOC2). "That form now fails the suite, which I ran" is true (M1). The draft-5 table has a row for every round-5 finding of both testers. False: A3. Incomplete: K0's list of what is not detected (M11 to M15, VT, VI). Stronger than its evidence: the K8 row "half the test chains issue duties at readiness; the re-run is checked against InstAL on a counterfactual history", given TA and L3-cft.

## The round-5 list A, re-run

| Round-5 finding | Status | Evidence from this run |
|---|---|---|
| Fable A1. The re-run verdict loses the loss off its conventions | Fixed | G1 to G5, H1, H1b, E1c: the shares and the loss under prohibition account for the loss, and InstAL's `missed` equals the players. The repair is where A1 of this round comes from |
| Fable A2. The suspension rule counts refusals the text excludes | Fixed | E1: `counted=None ... InstAL missed=['u0']`. P1: `InstAL missed=['u1'] \| gated: players=['u1'], shares={'u1': '2'}`. Not held by a test (GA) |
| Fable A3. Conflict intervals of the actual run used for the re-run | Fixed for one way per duty; several ways moved to a stated limit | DR: `forbidden=['5/2..9/2'], conflict=[] ... shares={'u0': '3/2'}, loss=3, under_prohibition=3/2`. Not held by the test (TA) |
| Fable A4. The first counterexample and the 501 | Fixed | CE1: `draft 5's rule: shares={'u1': '1'}, miss=1`. CE4: `523 differ` |
| Fable A5. Validation incomplete | Partly fixed | Five cases fixed, four unhandled errors now messages, eleven moved to a stated limit, new silent cases (A2) |
| Fable A6. "Both now fail the suite" | Fixed | M1: `fails (test_instal.py: AssertionError: one unit cannot emit another's amendment: step 1 has occurred(refuse...`. The fourth rejection has a scenario (VU fails) |
| Astra A1. A refusal outside conflict becomes a suspension | Fixed | E1, as above |
| Astra A2. The fixed-length comparison tests a different rule | Fixed | CE1, CE4 |

Of the eight: seven fixed, one of them with part moved to a stated limit; one partly fixed; none not fixed; none no longer applicable.

## Stop conditions

1. **A scenario has no trace or several.** Not met. 182 runs, one answer set each.
2. **A repair that needs a change to InstAL's semantics.** Not met. The repairs above are in gated.py, the adapter and the text. In A1 InstAL's output is the correct one.
3. **K8 cannot be made without restating the norm rules inside the verdict layer.** Open, as the document says, and its account of what stands and what does not is fair with two corrections.
   - The number is 34, not 40 (A3).
   - Those 34 histories are built from the verdict's own work times, so the agreement shows that the interval lookup matches InstAL's refusals at those times. It does not show that the re-run is what the units would have done (A1, L3-cft).
   - A1 can be repaired from the log alone. It is not evidence that the condition is met.
   - Nothing new bears on several ways per duty. Neither option has been built.
4. **K11 cannot be made by events alone.** Not met, in the simple form only. Touched only by CB1 to CB3 and the control Z1m.
5. **The defects are not converging.** Not met on my count, whatever the other tester reports: 21, 15, 6, 5, 6, 3. The condition needs neither count to fall, and mine fell. Three cautions.
   - If A1 is read as a declared convention, or A3 as too small to count, the count is 2. On no reading of the borderline items does it reach 6.
   - Eleven scenarios left list A because the document now states the limit, not because the adapter changed. They were inside one finding, which is still counted (A2), so this did not move the number.
   - In material that existed in round 5 the count is 1, a residue. The other 2, with the only high one, are in what was written after round 5. This is the fourth round running in which the new material holds most of the defects. What fell this round is their number, from three to one, and their reach: who is late now holds and only the amounts are wrong.

CONFIRMED WRONG OUTCOMES (list A): 3
OF WHICH IN NEW MATERIAL: 2
CRITICAL: 0  HIGH: 1  MEDIUM: 1  LOW: 1

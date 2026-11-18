# Attack on the composition contract, round 7 (Fable)

Target: [Composition contract.md](/root/mic/reports/Composition contract.md), draft 7, and the models in /root/mic/model as changed after round 6.

Every scenario is in [attack_contract_fable_r7.py](/root/mic/model/attack_contract_fable_r7.py):

```
python3 /root/mic/model/attack_contract_fable_r7.py            # about twenty minutes on eight cores
python3 /root/mic/model/attack_contract_fable_r7.py --fast     # skips the mutation checks and the instrumented test, about three minutes
```

The "derived" lines below are copied from that run. Identifiers such as TIE1 are the labels the script prints. No existing file was changed; mutation checks work on copies. The nine documented checks all pass. I did not read the other tester's round-7 report.

## Summary

- **Count: 3 confirmed (list A), against 3, 6, 5, 6, 15 and 21 in the earlier rounds. My count did not fall.** Counted by finding, as in rounds 2 to 6. One is high, in material that existed in round 6. Two are in material written after round 6, one medium and one low, and both are in the text, not the code.
- **The verdict by re-run now holds inside its scope and conventions, away from equal times.** I could not break it. On 64 chains from a wider generator, every coalition's re-run (155 in all, 83 of them histories that did not happen) agrees with InstAL run on a history laid out from the generator's own record of what each unit did. Round 6's high finding is repaired, the amounts are the ones InstAL gave, and the repair is held by a test.
- **Equal times are repaired for one pair of events and not for the other (HIGH).** K1 now puts acts of authorities before requests. It says nothing about a unit's request and the arrival of its input at one certified time. An attempt logged before the input arrived is read as made "once ready", excuses the unit, and moves six days of loss from one place to another on the position of two log entries.
- **Rule 3 of "who is late", as rewritten, records a miss for every unit rule 1 excuses (MEDIUM).** The test does something the text does not say. Draft 6's wording was right.
- **"523" is stale (LOW).** The generator changed; the test prints 563.
- **The negative share is real for the code and outside the scope the clause covers.** It needs two bans on one unit, which K8 lists as open. With one ban per duty no share can be negative, by a short argument and on 60000 chains.
- **K12 holds.** No way round a declining authority by acts of authorities, on routes of three and four. **K2 holds.** 388 InstAL runs, one answer set each.
- **Both repairs the document says are pinned are pinned.** Each of gated.py's five rejections, two of them new, and two parts of the new adapter checks can be deleted without a test failing.

The script marks 4 scenarios as A-DEFECT. They group into the three findings below.

## List A: confirmed wrong outcomes or false statements inside the stated scope

### A1. HIGH. K1, K8: an attempt logged before the input of the same certified time is read as made once ready

Material that existed in round 6 (the reading `ready <= t` in gated.py). The new equal-times paragraph of K1 and `canonical_order` were written to close this class of defect and do not cover it.

- **Statements.** K8 rule 1: "A refusal excuses if the unit was ready, an authority forbade delivery at that moment, the refusal came by the unit's deadline, and the unit delivered at the moment the prohibition stopped applying." K1: "At equal certified times, acts of authorities come before requests ... The code now rejects a log in that order." The test's fourth named case: "an attempt before readiness does not excuse a unit that then delivers at the ban's end".
- **Scope.** A chain of two, one duty per unit, one way per duty, one ban, acts of authorities fixed, one shared sequence of events. The log is in K1's canonical order: no request precedes an act of an authority.
- **Scenario TIE1.** u0: budget 3, delivers at 3, on time. u1: budget 1, so due at 4. A ban on u1 from 1 to 6. u1's one attempt is logged at certified time 3, before u0's delivery of the same time. u1 delivers at 6.
- **Derived (TIE1).** `u1: ready=3, due=4, receipt=6, refusals=['3'], forbidden=['1..6'], conflict=['1..6'], counted=3, deadline_sent=6 | InstAL missed=[] | gated: players=[], shares={}, loss=2, under_prohibition=2; actual_loss=2 || canonical_order on this log: accepted`
- **Derived (TIE1c, the same attempt a tenth of a day earlier).** `u1: ready=3, due=4, receipt=6, refusals=['29/10'], forbidden=['1..6'], conflict=['1..6'], counted=None, deadline_sent=6(before requests) | InstAL missed=['u1'] | gated: players=['u1'], shares={'u1': '0'}, loss=2, under_prohibition=2; actual_loss=2`
- **Correct outcome.** u1 is late, InstAL records the miss and u1 is a player, as in TIE1c. When u1 asked it had no input: its request comes before the hand-over in the one ordered log. The reading compares times, the two are equal, and the refusal excuses.
- **Scenario TIE2,** the amounts. u0: budget 2, delivers at 3, a day late. A ban on u1 from 2.5 to 10. u1 (budget 2) logs its one attempt at time 3 before u0's delivery and delivers at 10.
- **Derived (TIE2).** `u1: ready=3, due=5, receipt=10, refusals=['3'], forbidden=['5/2..10'], conflict=['5/2..10'], counted=3, deadline_sent=10 | InstAL missed=['u0'] | gated: players=['u0'], shares={'u0': '6'}, loss=6, under_prohibition=0; actual_loss=6`
- **Derived (the same attempt at 2.9).** `InstAL missed=['u0', 'u1'] | gated: players=['u0', 'u1'], shares={'u0': '0', 'u1': '0'}, loss=6, under_prohibition=6; actual_loss=6`
- **Correct outcome.** The second. Read as made at readiness, the attempt gives u1 no work at all, so a complying u0 would have let it through before the ban, and u0 pays all six days. Read as what the log says, u1 is late and the six days arise under prohibition. Six days move on the position of two entries with one certified time.
- **Why the repair of round 6 does not reach it.** `canonical_order` knows two kinds of entry, a request and an act of an authority, and compares neighbours. Two requests at one time pass (CO1). The hand-over that makes a unit ready is a request of the unit before it.
- **A note on classification.** One could declare that an attempt bearing the certified time of the input counts as made at readiness. K8 already says such an attempt "has no work at all", for an attempt made the instant the input arrives. Nothing says so for an attempt recorded before it, and K1's stated reason for a rule at equal times is exactly that "an interval read from times alone cannot say which came first". If the author takes that reading, this moves to list B and the count is 2.
- **Repair.** Either extend K1's order (at equal times a hand-over comes before the receiving unit's requests), give `canonical_order` a third kind, and reject the log otherwise; or pass the verdict only the refusals that follow the receipt of the input in the unit's own log, which F2 already orders. Then call the check where the evidence is built, not only in the test (see AST1 in list B). No norm rule is needed.

### A2. MEDIUM. K8 rule 3, as rewritten, records a miss for every unit that rule 1 excuses

New material (the rewritten "Who is late").

- **Statement.** K8 rule 3: "For InstAL to record that, the deadline is sent at the original time, unless the duty is in conflict then, in which case it is sent when the conflict ends and before any request."
- **Scenario RULE3.** One unit, budget 3. A ban from 1 to 5. Refused at 2, delivers at 5. Rule 1 excuses it. Its deadline of 3 falls inside the conflict, as it must for any excused unit that delivers after its deadline.
- **Derived, the deadline placed as rule 3 words it.** `u0: ready=0, due=3, receipt=5, refusals=['2'], forbidden=['1..5'], conflict=['1..5'], counted=2, deadline_sent=5(before requests) | InstAL missed=['u0'] | gated: players=[], shares={}, loss=2, under_prohibition=2; actual_loss=2`
- **Derived, as the test places it.** `counted=2, deadline_sent=5 | InstAL missed=[] | gated: players=[]`
- **Derived, at the original time.** `counted=2, deadline_sent=3 | InstAL missed=[] | gated: players=[]`
- **What is wrong.** Followed as written, the rule moves the excused unit's deadline to the end of the conflict and puts it before the request that delivers at that instant, so InstAL records a miss. The rule is there "for InstAL to record" rule 2 and it records the opposite of rule 1. The test places the deadline a third way that the text does not mention: if a refusal excuses, at the receipt and after requests. With the test changed to follow the text (mutant TW) it fails on its third chain: `AssertionError: (2, ['u0', 'u2', 'u3'], ['u0'], ...`.
- **Why it counts.** K8 says no layer's code places the deadline, so this sentence is the only specification an evidence layer would have. Draft 6 had it right ("A unit with a refusal that counts is not late. Otherwise the deadline stands, except ..."); the rewrite lost the "otherwise". The verdict's players are not affected.
- **Repair.** Say what the test does, or say the simpler thing the third run shows: for an excused unit send the deadline at its original time, where K4's rule 4 records a conflict and no miss. Move it to the conflict's end, before requests, only when no refusal excuses.

### A3. LOW. K8: "the fixed length gave different shares from a re-run in 523"

The sentence existed in round 6 and was true then. It became false through the rewrite of test_gated.py's generator after round 6, so I count it with the new material.

- **Statement.** K8: "On 2000 random chains with bans the fixed length gave different shares from a re-run in 523."
- **Scenario CE4.** test_gated.py as it stands.
- **Derived.** `test_gated.py prints: a share is negative in 1 of them; the fixed length gives different shares in 563 || the document says 523: True`
- **Repair.** Write 563. This is the third round running with a count not re-run after the code under it changed (501 in round 5, 40 in round 6).

## List B: design objections and claims stronger than their evidence

| Id | Clause | Objection | Derived |
|---|---|---|---|
| NEG1, NEG2, NEG-I | K8 "A share can be negative ... a counterexample to the property in K10 that no share is negative, once bans exist"; claims table | The printed chain is right by hand and InstAL agrees with the re-run on the two histories that matter. But the credit needs two forbidden intervals on one unit, which K8 lists as open, and it comes from the after-time (a unit that is refused dawdles afterwards; one that is not refused does not), not from being carried past a ban. With one interval per unit no share is negative: a unit refused on the actual run never arrives later than it did, a unit not refused has no after-time, so delivery is non-decreasing in arrival over the arrivals a coalition can produce and the loss is monotone in the coalition. "A unit whose lateness carried the next unit past a ban reduced the loss" cannot happen with one ban | `negative shares in chains [(256, 'a unit with two intervals')]; chains holding a unit with two intervals: 929 of 2000 \|\| at most one interval per unit: negative in 0 of 60000 \|\| up to two: negative in 5 of 60000` |
| G-DAW, L3-cf | K8 convention "if forbidden then, it waits and delivers after its after-time"; "gets nothing for the refusal" | A unit's idle time after a refusal exists only on re-runs in which it is refused. u0 is a day late; u1 is refused in time, then sits for 3.5 days after the ban ended. u0 pays 2.25 days and u1 1.75. Declared, and it is what a literal replay of the log gives (the attempt would have been the delivery). It also means "what its log shows" and the convention are different histories whenever a unit has an after-time (L3-cf: 3.25 against 3), and the test compares the re-run only with the second | `shares={'u0': '9/4', 'u1': '7/4'}, loss=4 ... kept ['u1'] -> final receipt 3` |
| G-CV | K8 convention "takes its budget if its work was longer and has no after-time" | A complying unit is the latest possible when its work was too long and the earliest possible when its after-time was. u0, half a day late through its after-time, is taken to deliver a day before its deadline when it complies; that lets u1 through before a ban and u0 pays two days. Delivering at its deadline, the same two days arise under prohibition | `players=['u0'], shares={'u0': '2'}, loss=2, under_prohibition=0 ... kept [] -> final receipt 3` |
| AST1, CO1, TCO | K1 "The code now rejects a log in that order"; draft-6 table row 2 | The other tester's round-6 log, given to the reading and the verdict, gives the same wrong result as in round 6. What is new is a function that refuses the log when called on it. gated.verdict cannot call it (a Unit carries times, not a log). Its only caller is the test, on a list the test has just sorted into that order; with the call deleted the test passes. Any kind of entry other than the two it knows hides a pair from it | `counted=1, deadline_sent=2 \| InstAL missed=[] ... \|\| canonical_order: ValueError: at time 1 a request precedes an act of an authority`; `test_conflict_chain.py: passes` |
| DOC4 | K8 "the coalition 'everyone complies' and one other"; draft-6 table "one further coalition per chain" | The numbers are as run: 71, 43, all 43 distinct. Of the 31 further coalitions, 18 are the set of every late unit, which is the actual history again. A coalition that is neither was run for 13 chains of 40 | `the second coalition is every late unit, which is the actual history by definition: 18 \| histories that differ from the actual one: 43 \| distinct among them: 43` |
| GL, GK | K8 "its first attempt made once ready and refused under a ban" | With the last such attempt taken in place of the first, and with a refusal exactly at readiness ignored, both suites pass. No maintained case has two refusals under a ban once ready | `test_gated.py + test_conflict_chain.py: passes` each |
| GM, GN, GO, GNB, GEI, G7 | K8 "It refuses a receipt inside a forbidden interval, readiness that does not follow the previous receipt, a negative budget, an empty interval, and a re-run that does not end at the actual receipt" | Each of the five can be deleted and both suites pass. The two new ones have no test. The last still cannot fire on any evidence once the others pass (60000 chains, none) | `passes` each; `rejected because the re-run does not reproduce the receipt 0` |
| UA, UF, TF | K8 rule 3; K11; "half with every duty in force from the start" | Still undetected: InstAL recording a miss for a deadline in conflict; InstAL keeping a grant after a refusal; every random chain with its duties in force from the start | `test_conflict_chain.py: passes` each |
| DOC5 | K8 "Round 6 found this rule, InstAL's record and the verdict's players in agreement on every boundary case tried"; status "none in the rule for who is late" | True of my round-6 runs. The other tester's one finding in round 6 was a boundary case in which the verdict had no player and InstAL, with the deadline rightly placed, recorded a miss. K1 of this draft says so | `returns 1 and the verdict has no player` |
| Stop 3 | Stop condition 3 "the verdict layer needs two things" | A1 is a third: where a refusal stands in the log relative to the hand-over at the same time. K1 names the alternative ("carry log positions as well as times") and says it is not done. It is a fact from the log and no norm rule, so it does not move the condition | see A1 |
| PWa | K0, the new charter check | A charter that lists an authority with nothing to issue, with an empty set, is rejected; left out, the same scenario runs. The message says why, so this is the check as written. It is input that needs no guess, K0 describes the check only as "names a norm that does not exist", and a charter written out one line per authority cannot be run | `AssertionError: power of group names an authority that issues nothing or a norm that does not exist \|\| without that entry: ... request(ord_x) -> ['refuse(ord_x),unit']` |
| PWc, PWb, PWd | K0, the same | Accepted: a charter that gives one authority another's norm (it does nothing, and the owner, left out, may then issue nothing); a charter given as the string 'ap' for norms a and p; an entry for a unit | `issue(p) -> ['ultravires(p),state']; request(ord_x) -> ['emit(ord_x),unit']` |
| HP2, HA | K0, the checks written after round 6 | Two wrong versions pass: the charter check without its test of the authority; no check that `authorities` is a list. The other three fail the suite (HK, HP, HP3). They are not in K0's list, which says it is the union of rounds 4 to 6 | `test_instal.py: passes` each |
| AU2, NKf, EFb | K0 | `authorities` as a frozenset is rejected where a set is accepted. `ultra_vires='no'` marks the norm. Keys inside an effect cannot be checked, so 'cots' for 'cost' still changes the outcome silently | `request(e) -> ['emit(e),unit'] \|\| control: ... request(e) -> ['refuse(e),unit']` |

## List C: limits the document already lists

| Id | Clause | Limit | Derived |
|---|---|---|---|
| I1, I1c | K4 | An unreachable way counts as a way. Unchanged and stated | `deadline(o) -> ['missed(o),unit']`; one effect fewer: `deadline(o) -> ['conflict(o),unit']` |
| D5, LI3 | K8 open | A unit that waits without attempting is late and can be charged for the ban | `deadline_sent=6(before requests) \| InstAL missed=['u0'] \| gated: players=['u0'], shares={'u0': '0'}` |
| MB1, TB | K8 open "several bans on one duty" | Changed since round 6. A unit refused under a first ban, which does not deliver when it ends and is refused again in time under a second, is now late. By rule 1 as worded the second refusal excuses. The after-time of 4.5 days holds 3.5 days of waiting under the second ban, so "waiting for a ban to end is never work" does not hold here | `players=['u0'], shares={'u0': '3'}, loss=3 ... after=['9/2']; counted=['None']`; `counted=None, deadline_sent=6(before requests) \| InstAL missed=['u0']` |
| MB2, MB3 | K8 open, the same | A unit refused under one interval and delivering at the end of a later one; an after-time that lands in a second interval on a re-run; touching and overlapping intervals. No error; nothing checks the amounts | `kept [] -> final receipt 4` |
| G-RDY | K8 "Not tested by anything" | The timing of a unit's first log entry decides what the unit upstream pays | `attempt logged at 3: players=['u0'], shares={'u0': '6'}, loss=6, under_prohibition=0 ... attempt logged at 5: players=['u0'], shares={'u0': '0'}, loss=6, under_prohibition=6` |
| CB3 | K8 "Not tested by anything" | A unit with no funds, refused under a ban and funded as it ends, is excused | `counted=1, deadline_sent=4 \| InstAL missed=[]` |
| UC | K8 rule 3 | With InstAL changed so that no duty is ever in conflict the test passes, as stated | `test_conflict_chain.py: passes` |
| SD1, SD2, W2e | K0 | An unread self-declared attribute; a Decimal cost. Moved here from round 6's list A by the new sentences | `request(ord_x) -> ['refuse(ord_x),unit']`; `AssertionError: the cost of e is not a finite number of zero or more` |
| PWe, AU4, NKb | K0 "Other malformed shapes ... still end in an unhandled error" | A charter that is not a mapping; `authorities` None or a number; a norm with no kind or pattern | `AttributeError`, `TypeError`, `KeyError` |
| RW1, RW2 | K12 "the log" | A relayed withdrawal, or a relayed issue below a declining authority, observed directly takes effect | `relayed(p) -> ['issued(p),group']; request(ord_x) -> ['refuse(ord_x),unit']` |
| N3, N4, N5, N6, N7, N11, N14, N15 | K12 status, K0 | The eight wrong versions of the route code still pass | `passes` each |
| M11 to M15, VT, VI, VCR2, VPR, NR5, NR6, NR7, VC2, VC3, VC4, VCY2, V2C, NE3, NE4, NE5, NA, VE | K0 "The list below" | All 22 still pass, as stated. The list is complete against my runs this time | `passes` each |
| K8 open | K8 | Several ways per duty; who bears the loss under prohibition; two duties per hand-over; trees; no layer places a deadline | not run |

## Priorities 1 to 6

1. **gated.py inside its scope.**
   - The named amounts are right by hand (L1, L2, L3, T49, DR-g) and are the ones InstAL gave in round 6.
   - RND, 64 chains with one ban per unit at most, through InstAL: `{'chains': 64, 'units': 129, 'late': 83, 'excused': 14, 'refused_after_deadline': 35, 'after_time': 35, 'before_ready': 6, 'unfunded': 7, 'several': 17, 'zero_budget': 22, 'zero_work': 19, 'early_unit': 40, 'coalitions': 155, 'counterfactual_histories': 83}; disagreements: 0`. Each coalition's history is laid out from the generator's record (first attempt, refusals, time after the ban), not from gated.py's `work` and `after`, and every coalition is run, not two.
   - Boundaries: a refusal exactly at readiness, at the deadline and at an interval's start excuses; one at the instant the interval ends does not; zero budgets, zero work, an early unit, a chain of one and an empty chain behave (G6, D1 to D9).
   - Evidence wrongly rejected: none found. Wrongly accepted: A1, where the evidence cannot say what the log says.
   - `canonical_order`: it rejects no log that K1 allows. It accepts the log of A1, and any pair hidden behind a third kind of entry (CO1).
   - The negative-share example is right by hand and by InstAL, and outside the scope (list B).
2. **Who is late.** With deadlines placed as the test places them, InstAL's `missed`, `Unit.late` and K8's rules 1 and 2 agree on every history I ran with distinct times (RND, D1 to D9, H1, E1, P1, P2, LI1 to LI3, CB1). They disagree with rule 1 at equal times (A1). With deadlines placed as rule 3 words it, InstAL disagrees with both (A2). With several bans the code and rule 1 as worded disagree (MB1, listed).
3. **Are the repairs pinned?** See the table below. Both that the document names are. Round 6's A1 is. The call to `canonical_order` is not.
4. **The round-6 adapter changes.**
   - Valid input rejected: an empty charter for an authority with nothing to issue (PWa), by design. A valid norm with all six keys is accepted (NKd).
   - Invalid input accepted: a charter over another authority's norm, a charter as a string (PWc, PWb).
   - A rejection for the wrong reason: none. Each of the suite's 25 rejections fires for the reason in its name (REJ), and the two new scenarios fail when their check is removed (HK, HP).
5. **K12 and K2.** No defect. Routes of three and four with each intermediate declining in turn; a declining authority below which every later authority issues, withdraws and reissues norms of its own; two norms on one route with one declined; norms routed in opposite directions with each authority declining the other's; a routed permission; created units to depth two; a declined routed exemption (R3a to RXB). 388 runs, one answer set each.
6. **The document.**
   - As run: 74 scenarios and 25 rejections; 4 named cases; 40 chains, 71 coalitions, 43 differing histories, all distinct; 25 chains; 300 and 2000 chains; one negative share (DOC1, DOC3, DOC4). The record for stop condition 5 and "two of the four in material written after round 5" are as the testers wrote them (DOC2).
   - "I undid two earlier repairs in turn ... and a test failed each time": true (TA, GA). "Detected, by the testers' runs": true for all ten I re-ran (M1, M1b, M6, NR1 to NR4, N8, N9, N10). K0's list of what is not detected: all 22 still undetected, none missing that an earlier round reported.
   - False: rule 3 of "who is late" (A2); 523 (A3).
   - Stronger than its evidence: the negative share as a counterexample within K8; "one other" coalition; "agreement on every boundary case tried"; "the code now rejects a log in that order" (list B).

## The round-5 and round-6 lists A, re-run

"Pinned" means that I undid the repair in a copy and a maintained test failed.

| Finding | Status | Evidence from this run |
|---|---|---|
| R5 Fable A1. The re-run verdict loses the loss off its conventions | Fixed, pinned | G1-G5. Mutant GC fails: `test_gated.py: AssertionError: refused in time, then delivers long after the ban ended` |
| R5 Fable A2. The suspension rule counts refusals the text excludes | Fixed, pinned. Not fixed at equal times (A1) | P1, P2, E1. Mutant GA fails: `test_gated.py: AssertionError: an attempt before readiness does not excuse, even when delivery follows at the ban's end`. GIN fails |
| R5 Fable A3. Conflict intervals of the actual run used for the re-run | Fixed for one way per duty, pinned; several ways are a stated limit | DR. Mutant TA fails: `test_conflict_chain.py: AssertionError` |
| R5 Fable A4. The first counterexample and the 501 | Partly fixed | CE1: the counterexamples are right against the stated rule. The count is stale again (A3) |
| R5 Fable A5. Validation incomplete | Fixed for what was reported; the rest moved to stated limits | UV1, PW1, PW2 rejected; SD1, W2e and the unhandled errors are stated |
| R5 Fable A6. "Both now fail the suite" | Fixed | M1 and M1b fail: `one unit cannot emit another's amendment` |
| R5 Astra A1. A refusal outside conflict becomes a suspension | Fixed, pinned | E1. Mutant GIN fails: `test_gated.py: ValueError: the re-run does not reproduce the actual receipt` |
| R5 Astra A2. The fixed-length comparison tests a different rule | Fixed | CE1 |
| R6 Fable A1. A unit's work contains its wait under a ban | Fixed, pinned | L1 `shares={'u0': '15/4', 'u1': '1/4'}`, L2, L3, T49 `shares={'u0': '4', 'u1': '1/2'}`; L1-cf: InstAL 3.5, re-run 3.5. Mutant GW fails: `a unit refused after its deadline is not charged again for the wait` |
| R6 Fable A2. Misspelt input changes outcomes silently | Fixed for a key inside a norm and a charter naming nothing, both pinned (HK, HP fail); an unread self-declared attribute and a Decimal cost moved to stated limits | UV1, PW1, PW2, SD1, SD2, W2e |
| R6 Fable A3. "40 histories that did not happen" | Fixed | DOC4: 71 coalitions, 43 differing, as printed and as written |
| R6 Astra A1. Equal times erase whether the authority had acted | Moved to a stated limit (K1 requires the order of F2); partly fixed. The function that checks the order is pinned (GCO fails) and the call to it is not (TCO passes); the reading is unchanged (AST1); the other pair of events at one time is not covered (A1) | AST1, CO1, TIE1 |

Of the twelve: nine fixed, of which three with a part moved to a stated limit; one fixed away from equal times and not at them; one partly fixed; one moved to a stated limit and partly fixed. None no longer applicable.

## Stop conditions

1. **A scenario has no trace or several.** Not met. 388 runs, one answer set each.
2. **A repair that needs a change to InstAL's semantics.** Not met. A1 is repaired in the evidence or in K1, A2 and A3 in the text. In A1 and A2 InstAL does what its rules say on the events it is given.
3. **K8 cannot be made without restating the norm rules inside the verdict layer.** Open, as the document says, and its account now matches what I can run, with one addition.
   - The numbers are right: 43 histories that did not happen, from 71 coalitions. My own 83, from every coalition on 64 other chains, agree.
   - "Two things" is three. The verdict also needs to know where a refusal stands relative to the hand-over at the same certified time (A1). Like the other two it comes from the log and is no norm rule.
   - Nothing new bears on several ways per duty. Neither option has been built.
   - For one way and one ban per duty, with distinct times, I now think the narrow case stands. Several bans on one duty are untested and the code and rule 1 already disagree there (MB1).
4. **K11 cannot be made by events alone.** Not met, in the simple form only. Touched only by CB1, CB3, D8 and mutant UF.
5. **The defects are not converging.** Not met. It needs two consecutive rounds in which neither count falls, and in round 6 both fell. My record is now 21, 15, 6, 5, 6, 3, 3: this round my count did not fall. If the other tester's did not fall either, this is the first of the two rounds. Three cautions.
   - If A1 is read as a convention the count is 2 and it fell. If the charter check (PWa) is read as a wrong rejection and not as a design choice, it is 4. I have kept both where round 6's standard puts them.
   - What changed is where the defects are. For the first time since the split was first counted, in round 4, the code written since the last round has none I could find: both findings in new material are in the text. The high one is a residue of the other tester's round-6 finding in older code.
   - The scope did not change this round, so this count is comparable with round 6's.

CONFIRMED WRONG OUTCOMES (list A): 3
OF WHICH IN NEW MATERIAL: 2
CRITICAL: 0  HIGH: 1  MEDIUM: 1  LOW: 1

# Attack on the composition contract, round 4 (Fable)

Target: [Composition contract.md](/root/mic/reports/Composition contract.md), draft 4, and the models in /root/mic/model as changed after round 3.

Every scenario is in [attack_contract_fable_r4.py](/root/mic/model/attack_contract_fable_r4.py):

```
python3 /root/mic/model/attack_contract_fable_r4.py            # about four minutes on eight cores
python3 /root/mic/model/attack_contract_fable_r4.py --fast     # skips the mutation checks
```

The "derived" lines below are copied from that run. Identifiers such as S1 are the labels the script prints. No existing file was changed; mutation checks work on copies. The eight documented checks all pass. I did not read the other tester's round-4 report.

## Summary

- **The new suspension rule excuses lateness the ban did not cause (HIGH).** A ban that starts and ends before the unit tries to deliver moves the unit's deadline by the ban's length. Of the 24 hand-overs the test reports as "delayed only by a ban", 12 were not delayed by the ban at all, and 3 more would have been late without it.
- **The test of K8 runs neither the verdict layer nor the evidence layer (HIGH).** "The verdict's residuals agree" is one line of the test script. `verdict.py` has no input for a length. Stop condition 3 is declared not met on this test.
- **The oracle is the rule written a second time.** For one ban the two are the same inequality; they agree on 200,000 random histories without InstAL being run. This is a list-B point, but it is why the test could not see the first finding.
- **K12 routes work as InstAL cross-institution generation.** Removing the generation power or the bridge rule stops the norm. Length three, withdrawal, created units and loops behave. The defects are in the adapter's handling of route input (MEDIUM): a route declared beside the origin's own edge silently disables that edge, a misspelt `blocked` fails open, and three inputs raise unhandled errors.
- **The round-3 validation fix is real but partial (MEDIUM).** All eight round-3 inputs are now rejected. Authority names are still unchecked, and two accepted inputs change outcomes silently.
- **One sentence of K0 is false (LOW).** Two of "the six changes that went undetected in round 3" are still undetected.
- **K2 holds.** 89 InstAL runs, one answer set each, including routes that loop.
- **Count: 5 confirmed (list A), against 6, 15 and 21 in the earlier rounds.** Three of the five are in material added after round 3.

The script marks 31 scenarios as A-DEFECT. They group into the five findings below. I count by finding, as in rounds 2 and 3.

## List A: confirmed wrong outcomes or false statements inside the stated scope

### A1. HIGH. K8: a ban that never delayed the unit excuses the unit's own lateness

New material.

- **Scenario S1.** One unit, ready at 0, three days allowed, due at 3. A ban on delivery from 0.5 to 1.5. The unit makes no attempt during the ban and delivers at 4.
- **Derived.** `u0: ready=0, due=3, intended=4, receipt=4, in_conflict=['1/2..3/2'], suspended=1, deadline_sent=4, InstAL=no miss, test_oracle=not late, had_there_been_no_ban=late, residual=0 | timeline 1/2: issue(ban0_u0); 3/2: withdraw(ban0_u0); 4: request(h_u0); 4: deadline(recv_u0)`
- **Control S1c,** the same without the ban. `in_conflict=[], suspended=0, deadline_sent=3, InstAL=missed`
- **Scenario S2,** a draw from the test itself (seed 23, trial 4, u0): ban from 2 to 3.5, due at 3, intended and delivered at 4.
- **Derived.** `in_conflict=['2..7/2'], suspended=3/2, deadline_sent=9/2, InstAL=no miss, test_oracle=not late, had_there_been_no_ban=late, residual=-1/2`
- **Scenario S3,** the tie rule. The ban starts at the instant of the deadline and lasts two days. The unit had not delivered and intended to deliver 1.5 days late.
- **Derived.** `in_conflict=['3..5'], suspended=2, deadline_sent=5, InstAL=no miss ... residual=0`. Control S3c, the ban a quarter of a day later: `InstAL=missed ... residual=2`
- **Scenarios S4 and S8.** A ban that starts before the duty is ready (the clip to readiness works, and the outcome is S1's). A ban issued and withdrawn with no event between (the interval is read correctly, and the outcome is S1's).
- **Scenario S0,** the test's 30 chains replayed with its seed, without InstAL.
- **Derived.** `units=93, banned=52, late=25, counted_as_delayed_only_by_a_ban=24, of_which_delivered_when_intended=12, of_which_intended_after_the_deadline_anyway=3, of_which_on_time_but_for_the_ban=9, ban_starts_before_ready=0, ban_starts_at_deadline=8, ban_ends_at_deadline=7, ban_covers_window=1`
- **Scenario C3.** The author's test with one change: a unit is late if it intended to deliver after its deadline.
- **Derived.** `test_conflict_chain.py: fails (AssertionError: (1, [], ['u0'], {'u0': {'ready': Fraction(0, 1), 'due': Fraction(2, 1), 'receipt': Fraction(4, 1), ...`
- **Correct outcome.** S1, S2, S3, S4 and S8: missed. In each the unit delivered when it meant to, nothing it asked for was refused, and it was late by its own slip. In S1 and S8 the duty was not even in conflict at its deadline, so K4 rule 5 says missed and K8 says not.
- **Why this is in list A although the rule is implemented as written.**
  - The test's own words set the standard. Its generator calls the slip "the unit's own doing, and its link's". Its last line says the units recorded as missing "are exactly those late on their own account". Fifteen units late on their own account are not recorded.
  - The document's sentence is false. "24 hand-overs delayed only by a ban were not recorded as missed": 9 were.
  - The excuse is handed on. In S2 the verdict is given a residual of minus half a day for a unit that slipped a day. An early unit offsets other units' lateness in the shares.
  - Any ban, however short and however early, buys its own length. S3 shows the same at the tie: a ban ordered before the deadline at the same instant excuses 1.5 days of the unit's own lateness, and a quarter of a day later it excuses none.
- **What the test did not draw.** A ban that starts before the duty is ready never occurs in 93 units. Removing the clip to readiness passes the test (C2).
- **Repair.** Stop the clock only for what the log shows the ban cost: from the unit's first refused attempt while in conflict to the end of the conflict. At the least, count only a conflict in force at the original deadline. Both are changes to the evidence rule, not to InstAL. Replace the oracle by one that uses `intended`, and correct the 24.

### A2. HIGH. K8, stop condition 3: the verdict layer and the evidence layer are not run

New material.

- **Statements.** K8: "The evidence layer moves the deadline by that length. The verdict layer is told only the length." "The verdict's residuals agree." Stop condition 3: "the verdict layer is told only a length of time."
- **Scenario VL.**
- **Derived.** `test_conflict_chain.py imports verdict.py: False; imports evidence.py: False; its imports: ['import random', 'from fractions import Fraction as F', 'from harness import run']; verdict.py has any of 'suspension', 'suspended', 'conflict', 'prohibit': False; evidence.py has any of 'suspension', 'suspended', 'conflict', 'prohibit', 'deadline': False; the line the document calls the verdict's residuals: ['residual = {u: spec[u]["receipt"] - spec[u]["due"] - suspended[u] for u in units}']`
- **What is false.** The test script reads the conflict intervals from the trace, moves the deadline, and computes "residual" itself. No function of `verdict.py` or `evidence.py` is called. `verdict.py` has no parameter that could receive a length. Nothing is tested about the verdict: no miss, no shares.
- **Severity.** The scale gives HIGH to glue code doing what is attributed to a formalism. Here it is a test script, which is not even in the trusted base, so in the composed system nobody does it.
- **What happens when the verdict layer is told a length (X1, X2; list B, given here because it bears on the same sentences).** Two units, three days each, claimant's deadline 6. u0 is two days late. u1 is banned from 4.5 to 9. In X1 u1 would have met the ban anyway. In X2 u1 meets it only because u0 was late.
  - X1: `given to the verdict layer: ['u0: InstAL missed, suspended 0, residual 2', 'u1: InstAL no miss, suspended 4, residual -3'] | verdict.py, claimant's deadline 6: miss 3, shares {'u0': '2'}, baseline_holds False | verdict.py, deadline moved by the total suspension: miss 0, shares {'u0': '0'}, baseline_holds True | re-running the chain: shares {'u0': '0'}, miss 3, miss with every player compliant 3, final receipt 9`
  - X2: the same up to `re-running the chain: shares {'u0': '3'}, miss 3, miss with every player compliant 0, final receipt 9`
  - The verdict layer gets identical input in both and so answers the same. u0 caused none of the three days in X1 and all of them in X2. With the claimant's deadline the shares do not sum to the miss (K10). With the deadline moved nobody pays in X2 for a loss u0 caused.
- **Repair.** Give `verdict.py` the input and call it from the test, with assertions on miss and shares. State where the length goes (the unit's allowance and the claimant's deadline, or neither). Say what the counterfactual convention of K10 does with a ban that a compliant unit would have missed.

### A3. MEDIUM. K12, K0: the adapter's handling of route input

New material.

- **Scenario T3.** State has its own edge to the unit and issues a prohibition within its charter. A route for the same norm through the ministry is also declared, and the ministry declines it.
- **Derived.** `issue(a) -> ['issued(a),group']; issue(p) -> ['issued(p),state']; request(ord_x) -> ['emit(ord_x),unit']`. Control T3c, same edges and no route: `request(ord_x) -> ['refuse(ord_x),unit']`
- **Scenario R4.** The same with an exemption: state outranks the ministry and has its own edge, and a route through group is declared.
- **Derived.** `issue(x) -> ['issued(x),group', 'issued(x),state']; request(rep_r1) -> ['refuse(rep_r1),unit']`
- **Scenarios T4, T4b, T4c.** `blocked` names a misspelt authority; names a norm that does not exist; is given as a string.
- **Derived.** T4 and T4b: `issue(p) -> ['issued(p),ministry', 'issued(p),state']; request(ord_x) -> ['refuse(ord_x),unit']`. T4c, where blocking `pq` also blocks `p`: `issue(p) -> ['issued(p),state']; issue(pq) -> ['issued(pq),state']; request(ord_x) -> ['emit(ord_x),unit']; request(ord_y) -> ['emit(ord_y),unit']`
- **Scenarios T5, T5h, R8.** An empty route; a route from the origin to itself; a routed exemption through an authority with no rank.
- **Derived.** `IndexError: list index out of range` | `InstalBridgeTypeError: Source institution is the same as sink institution.` | `KeyError: 'agency'`
- **Scenario DOC5.** `K0 mentions 'relay': False; K0 says exemptions are decided 'from the ranks of their origins': True; K0 says bridge powers are 'one for each authority edge': True; harness.py writes mayrelay: True`
- **Correct outcome.**
  - T3, R4: the norm takes effect over the origin's own edge, or the scenario is rejected. K12 is about an authority "with no edge of its own". K0 says there is a bridge power "for each authority edge", and here the edge carries none. An intermediate vetoes a norm it has no say over, and an exemption loses rank by a route it did not need.
  - T4, T4b, T4c: rejected. The scenario says the norm is not passed on and it is passed on. This is round 3's misspelt edge in the new input.
  - T5, T5h, R8: rejected with a message.
  - DOC5: K0 is the list of what the trusted adapter writes. It should name the relay permissions, the powers between authorities, and the rank taken from the route.
- **Repair.** In the adapter: reject a route whose origin has an edge to the target (or write both powers); check that `blocked` is a mapping from authorities on a route to lists of routed norms; require at least two distinct authorities; require a rank for every authority on a route. Four assertions and three sentences in K0.

### A4. MEDIUM. K0: validation is still incomplete

Material that existed in round 3. Round 3's A3, partly fixed.

- **What is fixed.** V1 to V5d, the eight round-3 inputs, are each rejected with a message.
- **Scenario W9.** A pattern value written as a string and not a set: a prohibition on disclosure to `r10`.
- **Derived.** `request(rep_r1) -> ['refuse(rep_r1),unit']; request(rep_r2) -> ['emit(rep_r2),unit']`. Matching is by substring, so disclosure to `r1` is forbidden.
- **Scenario W13.** Edges given as lists and not tuples.
- **Derived.** `ownissue(a,officer) -> []; issue(p) -> ['issued(p),ministry']; request(rep_r1) -> ['emit(rep_r1),unit']`. Control W13c with a tuple: `refuse(rep_r1)`. The edge check passes and the membership test that writes the power fails, so the prohibition is inert.
- **Scenario W2c.** A cost of NaN. `request(rep_r1) -> ['emit(rep_r1),unit']`. K0 says it rejects "a cost that is not a number".
- **Scenarios W1, W1c, W3, W3g, W3e, W4, W4e, W7, W8.** Unhandled errors where a rejection is due.
- **Derived.** A capital letter in an origin's name: `InstalParserSyntaxError: Syntax error at 'Ministry'`. A listed authority `Board`: the same. A norm called `not`: `InstalParserSyntaxError`. A unit called `institution`: `InstalParserSyntaxError`. A norm id `é`: `UnicodeDecodeError`. An event with no closing bracket: `RuntimeError`. An empty event: `IndexError`. A rank that is not a number: `TypeError`. A norm with no pattern: `KeyError: 'pattern'`.
- **Correct outcome.** Each rejected with a message. The identifier check added after round 3 covers norms, effects, units and procedures and omits authorities, which is where round 3's capital letter can still be put.
- **Repair.** Check authority names with the same test and exclude InstAL's reserved words; require each pattern value to be a set; normalise edges to tuples; reject NaN; check each norm has kind, origin and pattern; check each event against `name(args)`.
- **Related, not counted.** A Fraction cost is rejected although it is a number (W2d). A rank is still demanded where no exemption exists (W18). A misspelt scenario key such as `rout` is ignored (W16). A negative cost is accepted as free (W2b, Z2).

### A5. LOW. K0: "the six changes that went undetected in round 3 now each have a scenario"

Material that existed in round 3. Round 3's A4, partly fixed.

- **Scenarios M1, M2, M4, M6, M8, M9,** the six mutants of round 3, each run against the current suites.
- **Derived.**

| Mutant | Change | Checks run | Result |
|---|---|---|---|
| M1 | every unit owns every effect | all four InstAL suites | passes |
| M6 | a norm also matches the effects of other units | all four InstAL suites | passes |
| M2 | an equal rank counts as outranking | test_instal | fails |
| M4 | exemptions are not inherited by created units | test_instal | fails |
| M8 | the rejection "an effect with no owner" removed | test_instal | fails |
| M9 | the rejection "a norm with no target" removed | test_instal | fails |

- **What is false.** Four of the six have a scenario. M1 and M6 do not. The scenario added for M6, "a duty's conflict does not depend on another unit's effects", gives the other unit an effect that matches both the duty and the prohibition, so the wrong fact changes nothing.
- **Repair.** For M6, give the other unit an effect that matches the duty and not the prohibition. For M1, a request by a unit for another unit's permitted effect, which must do nothing. Or say four.

## List B: design objections and claims stronger than their evidence

| Id | Clause | Objection | Derived |
|---|---|---|---|
| ID | K8, claims table "against an independent oracle" | For one ban the oracle (receipt - ready - overlap > budget) and the rule (receipt > due + overlap) are the same inequality; when the ban starts after the deadline both say late. The test can fail only if the conflict interval in the trace is not the ban the generator drew. It shows InstAL's `conflicted` follows the ban. It cannot show the rule is right, and it did not (A1) | `200000 random single-ban histories ... rule by arithmetic and oracle differ in 0` |
| X1, X2 | K8, stop condition 3 | A length is not enough for shares. See A2 | see A2 |
| RR1 | K4, K8 | The second deadline exists as a rule in K8 and as lines of a test. No layer's code sends it, and it can be placed only after the conflict has ended, so the evidence layer cannot place a deadline when it falls due | `deadline(o) -> ['conflict(o),unit']; withdraw(p) -> ['withdrawn(p),ministry']; request(rep_r1) -> ['emit(rep_r1),unit']` |
| S5, S6 | K8 | A ban over the whole window gives the whole window again, although the unit could prepare throughout. A ban that ends at the deadline gives its full length afterwards | `in_conflict=['0..3'], suspended=3, deadline_sent=6, InstAL=no miss` |
| C2 | K8 | The clip of a suspension to readiness is untested: removing it passes | `test_conflict_chain.py: passes` |
| K8 | K8 | The test has one duty per unit. K8 gives each unit two, and says nothing about the response duty of a unit whose delivery is banned | none |
| K12 | K12 "only if the intermediate passes it on" | Passing on is not an act. It is an initial fact the adapter writes from `blocked`, fixed for the run. An intermediate cannot begin or stop relaying, or relay the issue and decline the withdrawal | N8 control fails as it should |
| R1 | K12, K4 rule 3 | State's exemption, relayed by group, against group's own prohibition has no effect, although the prohibition's issuer chose to pass it on and state outranks it. Over a direct edge it works | `request(rep_r1) -> ['refuse(rep_r1),unit']`; control `['emit(rep_r1),unit']` |
| R3 | K12 | A routed exemption takes the lowest rank on its route; a routed prohibition keeps its origin's. No reason is given | `issue(p) -> ['issued(p),group', 'issued(p),supra']; issue(x) -> ['issued(x),ministry']; request(rep_r1) -> ['refuse(rep_r1),unit']` |
| T5e, R6b | K12, K6 | A route whose last authority has no edge to the target, or whose target is a created unit, is accepted; the norm is relayed and binds nobody | `issue(p) -> ['issued(p),regulator', 'issued(p),state']; request(ord_x) -> ['emit(ord_x),unit']` |
| T5f, T5g | K12, K2 | A route may visit an authority twice or return to its origin. This is a loop of positive dependencies across institutions. It has one answer set here; `check_stratified.py` does not look at it | one answer set each |
| N3, N4, N5, N6, N7, N11, N14, N15 | K12 "tested on five scenarios" | Eight wrong versions of the route code pass: the carrier is the first intermediate; only the first hop is written; the origin keeps its own power; both route rejections removed; a declining intermediate still relays the withdrawal; a routed norm is not inherited; every authority may relay. No scenario has a route of three, a route with a created unit, a declined withdrawal, or a rejected route | `test_instal.py: passes` each |
| M11 to M15 | K4 rule 5, K4 rules 3 and 4, K5 | Round 3's five untested rules are still untested | `passes` each |
| VC, VU, VT, VI | K0 | Four rejections have no scenario: a cost that is not a number, a unit that is also an authority, a target that is not a unit, identifiers. K0 no longer claims they do | `test_instal.py: passes` each |
| W6 | K0, K6 | A creation cycle, and a unit that creates itself, are accepted | `request(d_v) -> ['refuse(d_v),v']; request(d_u) -> ['refuse(d_u),unit']` |
| DOC4 | K2 | "60 and 87 further runs by the testers in the two rounds": both numbers are one tester's, from rounds 1 and 2. Round 3's 94 is not there | `the document mentions 94: False` |
| DOC6 | Status | "Round 3 found one wrong outcome in its replacement." K4's own heading says one remains and one was repaired | both sentences present |

## List C: limits the document already lists

| Id | Clause | Limit | Derived |
|---|---|---|---|
| I1, I1c, I2 | K4 | An unreachable way counts as a way. Unchanged from round 3 and stated as open | `request(am_v_b) -> ['refuse(am_v_b),unit']; deadline(o) -> ['missed(o),unit']`; one effect fewer: `deadline(o) -> ['conflict(o),unit']` |
| T1 | K12 | The relayed event observed directly takes effect though the origin never issued | `relayed(p) -> ['issued(p),ministry']; request(ord_x) -> ['refuse(ord_x),unit']` |
| T1b | K12, K3 | The same after the origin's issue was against its charter. The charter check is in the origin's institution, so the observed event goes round it | `issue(p) -> ['ultravires(p),state']; request(ord_x) -> ['emit(ord_x),unit']; relayed(p) -> ['issued(p),ministry']; request(ord_x) -> ['refuse(ord_x),unit']` |
| T2 | K12, K3 | The relayed withdrawal observed directly ends a prohibition its issuer did not withdraw. The stated limit names the issue only, and K3's "only the issuer can end a prohibition" is not qualified | `relaywithdrawn(p) -> ['withdrawn(p),ministry']; request(ord_x) -> ['emit(ord_x),unit']` |
| T6d | K12 | On a route of three with the middle authority declining, one observed `relayed(p)` reaches every authority's institution and enters below the one that declined. Observed events carry no addressee, so the limit is wider than "the intermediate" suggests | `relayed(p) -> ['issued(p),ministry']; request(ord_x) -> ['refuse(ord_x),unit']` |
| S7, S7b, S7c | K8, K4 | Two bans on one duty, apart, overlapping, and with the second starting between the original and the moved deadline. The code compares with the moved deadline; the text does not say which | `in_conflict=['1..2', '7/2..5'], suspended=5/2, deadline_sent=11/2, InstAL=no miss` |

## Priorities 1 to 6

1. **The suspension rule and its test.**
   - The code that reads conflict intervals is correct on every case I gave it: adjacent steps, a start before readiness, a zero length, an end by exemption and not by withdrawal (S8, S4, S9, S11).
   - The tie rule the document reports (C1) is pinned by the test. The tie rule's consequence is S3.
   - The oracle is independent of both layers only in the sense that neither computes it. It is the rule restated (ID).
   - The verdict does not receive a length. It receives nothing (A2).
   - The rule is right in the one case it was written for, an attempt refused in time (S13).
2. **Routes.**
   - A routed norm does not take effect without the intermediate, except by a relayed event in the log, which is the stated limit (T6b, T6c, R9, T1).
   - An authority off the route cannot inject or withdraw by its own acts. Anyone who can put `relayed` or `relaywithdrawn` in the log can (T1, T2, T6d).
   - A route and a direct edge for the same norm disagree, and the route wins silently (A3).
   - A routed exemption never gets a higher rank than it should (R5; the mutants N1 and N2 that would give it one fail the suite). It can get a lower one than it should (R4, R1).
   - Withdrawal by the origin reaches the unit and its created units (T0, T6, R6). Issue and withdrawal in any order and number give one trace (T8).
   - It is InstAL's cross-institution generation. With the generation power or the bridge rule removed the suite fails (N9, N10, N13). The adapter writes powers and permissions, never the norm's state.
3. **Round-3 fixes.** Validation: A4. Mutation coverage: A5 and the B rows. Zero cost: fixed; a history of free, paid, free, refused, free gives `emitted ['f1', 'b1', 'f2', 'f3'], used 60, total 60`, and the control mutant fails the suites (Z1, Z1m).
4. **Round-3 list A.** See the table below.
5. **K2.** 89 runs, one answer set each. Routes of three, routes both ways between two authorities, a route returning to its origin, relayed events observed directly.
6. **The document.** Scenario counts match (57; K3 5; K4 20; K6 4; K7 7; K12 5). The record for stop condition 5 is as the testers wrote it. The draft-3 table has a row for each of the nine round-3 findings; two "Now" entries overstate ("Rejected now", A4; "Six scenarios added", A5). Described as tested and not tested: the verdict's residuals (A2), "24 delayed only by a ban" (A1), "an independent oracle" (ID).

## The round-3 list A, re-run

| Round-3 finding | Status | Evidence from this run |
|---|---|---|
| Fable A1. A unit with no reachable way is recorded as missing | Moved to a stated limit | I1 reproduces: `deadline(o) -> ['missed(o),unit']`. K4 states it as open. K0's rejection now reads "owned by its target", which I2 satisfies |
| Fable A2. Nothing sends a deadline after a conflict | Partly fixed | A rule is stated in K8. It lives in a test script, not in a layer (RR1, A2), and it excuses too much (A1) |
| Fable A3. The adapter accepts malformed input | Partly fixed | V1 to V5d all rejected. Authority names, pattern values, edge types and NaN remain (A4); new holes in route input (A3) |
| Fable A4. K0's status sentence false for four facts and two rejections | Partly fixed | The sentence is narrowed. Four of the six mutants are now detected. The new sentence is false for M1 and M6 (A5) |
| Fable A5. The record of rounds 1 and 2 understated | Fixed | Counts given as reported; stop condition 2 lists repaired, withdrawn, moved and open |
| Fable A6. What InstAL's literature does about conflicts | Fixed | The text now says the cited paper detects and later work revises |
| Astra A1. A zero-cost purchase breaks the connected test | Fixed | Z1; control Z1m fails |
| Astra A2. Coverage of equal rank and of inherited exemptions | Fixed | M2 and M4 fail the suite |
| Astra A3. "An effect with no owner is always rejected" | Fixed | K0 now says "with several" |

Of the nine: five fixed, three partly fixed, one moved to a stated limit, none not fixed, none no longer applicable.

## Stop conditions

1. **A scenario has no trace or several.** Not met. 89 runs, one answer set each. The inputs that raise errors fail before solving.
2. **A repair that needs a change to InstAL's semantics.** Not met. Every repair above is in the adapter, the evidence rule, a test or the text.
3. **K8 cannot be made without restating the norm rules inside the verdict layer.** The document's "not met" is not supported, and the evidence leans further toward met than in round 3.
   - The verdict layer was not run with a ban in play (A2).
   - The rule that was tested gives wrong outcomes on 15 of the 24 cases the test itself counts as successes (A1).
   - When `verdict.py` is given residuals and a length, it cannot tell a ban the late unit caused its successor to meet from one the successor would have met anyway (X1, X2). To tell them apart it must place the ban on each counterfactual timeline, which is rule 4 of K4 inside the verdict.
   - This is not yet a demonstration that it cannot be done. K10's convention keeps residuals fixed in counterfactuals and could be extended to keep suspensions fixed. The document would then have to accept X2: nobody pays for three days that one unit caused.
4. **K11 cannot be made by events alone.** Not met, in the simple form only. The zero-cost fix holds.
5. **The defects are not converging.** Not met on my count: 21, 15, 6, 5. Three cautions.
   - The fall is by one.
   - In material that existed in round 3 the count is 2, both residues of round-3 findings. The other 3, with both HIGHs, are in the two things added in one pass without review. New material is producing defects at the rate old material is losing them.
   - The condition also needs the other tester's count.

CONFIRMED WRONG OUTCOMES (list A): 5
OF WHICH IN NEW MATERIAL: 3
CRITICAL: 0  HIGH: 2  MEDIUM: 2  LOW: 1

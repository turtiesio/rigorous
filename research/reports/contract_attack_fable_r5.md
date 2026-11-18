# Attack on the composition contract, round 5 (Fable)

Target: [Composition contract.md](/root/mic/reports/Composition contract.md), draft 5, and the models in /root/mic/model as changed after round 4.

Every scenario is in [attack_contract_fable_r5.py](/root/mic/model/attack_contract_fable_r5.py):

```
python3 /root/mic/model/attack_contract_fable_r5.py            # about ten minutes on eight cores
python3 /root/mic/model/attack_contract_fable_r5.py --fast     # skips the mutation checks, about two minutes
```

The "derived" lines below are copied from that run. Identifiers such as G1 are the labels the script prints. No existing file was changed; mutation checks work on copies. The nine documented checks all pass. I did not read the other tester's round-5 report.

## Summary

- **The re-run verdict is right only on histories that follow its own conventions, and it does not check that they do (HIGH).** A unit that is refused once and then delivers late on its own account has no share, and the loss it caused is reported as zero. InstAL records the same unit as missing. The four properties K8 reports as tested hold by construction on the chains the test builds.
- **The suspension rule as coded is not the rule as written (HIGH).** The code takes the unit's first refusal of any kind. A refusal for want of funds before the ban began, or an attempt logged before the unit had anything to deliver, stops the clock to the end of the ban.
- **The conflict intervals handed to the verdict are facts about the actual run, and the re-run needs them for a different run (HIGH).** If a duty comes into force when its input arrives, a ban that came and went before that leaves no trace, and the late unit upstream is charged twice what InstAL itself gives when the history is run again.
- **The first counterexample to a fixed length, and the count of 501, are against a rule the document does not state (LOW).** Under K8's own rule the first counterexample charges nothing.
- **K12 holds with its new restrictions.** I found no way round a declining authority by acts of authorities, on routes of two or three, with created units, or with withdrawal. The new rejections fire for the stated reasons.
- **Validation is harder than in round 4 and still incomplete (MEDIUM).** A misspelt norm in an event, and a misspelt scenario key, change outcomes silently. The nine unhandled errors of round 4 are all still there.
- **One sentence of K0 is again false (LOW).** One of the two coverage repairs still does not detect the change as round 3 defined it.
- **K2 holds.** 71 InstAL runs, one answer set each.
- **Count: 6 confirmed (list A), against 5, 6, 15 and 21 in the earlier rounds.** Four of the six are in material written after round 4. My count did not fall.

The script marks 35 scenarios as A-DEFECT. They group into the six findings below. I count by finding, as in rounds 2 to 4.

## List A: confirmed wrong outcomes or false statements inside the stated scope

### A1. HIGH. K8: the re-run verdict loses the loss when a unit does not behave as the conventions assume

New material (gated.py).

- **Statements.** K8: "the re-run reproduces the actual receipt; no share is negative; no unit within its budget pays; shares sum to the loss less the loss under prohibition." "Its players are the units InstAL records as missing." "With no bans it equals the plain verdict."
- **Scenario H1,** through InstAL. One unit, ready at 0, due at 3. A ban from 0.5 to 1.5. The unit is refused at 1 and delivers at 4.
- **Derived.** `u0: ready=0, due=3, receipt=4, conflict=['1/2..3/2'], first_refusal=1, suspended=1/2, deadline_sent=7/2 | InstAL missed=['u0'] | gated: players=[], shares={}, loss=0, under_prohibition=0, reproduces_actual=False; actual_loss=1 | verdict.py with lengths: players=['u0'], miss=1/2, shares={'u0': '1/2'}`
- **Scenario H1b,** the same, delivered at 3.4.
- **Derived.** `InstAL missed=[] | gated: players=[], shares={}, loss=0, under_prohibition=0, reproduces_actual=False; actual_loss=2/5`
- **Scenario E1c,** through InstAL, no ban at all. The unit is refused at 1 for want of funds and delivers at 7.
- **Derived.** `conflict=[], first_refusal=1, suspended=0, deadline_sent=3 | InstAL missed=['u0'] | gated: players=[], shares={}, loss=0, under_prohibition=0, reproduces_actual=False; actual_loss=4 | verdict.py with lengths: players=['u0'], miss=4, shares={'u0': '4'}`
- **Scenarios G1 to G5,** gated.py called directly.
  - G2, a refusal at 1 before a ban from 2 to 6, delivered at 7: `players=[], shares={}, loss=0, under_prohibition=0, reproduces_actual=False; actual loss=4`
  - G3, nothing closed, one refusal: `loss=0 ... | verdict.py: miss=4, shares={'u0': '4'}`
  - G4, a refusal logged before the unit was ready: `players=[], shares={}, loss=0, under_prohibition=0, reproduces_actual=False; actual loss=2`
  - G5, one unit a day late and the next refused and then idle: `players=['u0'], shares={'u0': '1/2'}, loss=1/2, under_prohibition=0, reproduces_actual=False; actual loss=4`
- **Correct outcome.** H1: u0 is a player and pays, as the norm layer says. E1c and G3: four days, all u0's, as verdict.py says. In every case the shares and the loss under prohibition should account for the actual loss.
- **Why.** Convention 1 ends a unit's work at its first refused attempt, whatever the reason and whenever it was. Convention 2 says a refused unit delivers when the interval ends. When the unit in fact delivers later, the re-run stops at the earlier time, the unit's work is within its budget, and the difference belongs to nobody. The verdict returns its amounts with `reproduces_actual` False. Nothing reads the flag: test_conflict_chain.py asserts it only on the generator's chains.
- **Why the tests did not see it.**
  - Scenario G7: 5000 chains drawn inside the conventions, with zero budgets, zero work, empty, early and touching intervals. `failures: 0`. The properties hold because `build()` in the test and `rerun()` in the model call the same `gate()`, players are defined as work over budget, the sum is Shapley efficiency, and the receipt is monotone in each work time. The 2000-chain test cannot fail.
  - Scenario GEN, the generator of test_conflict_chain.py replayed with its seed: `units=126, refused=19, of which refused by the deadline=10; delivered later than the end of the ban that refused them=0, refused outside a ban=0`. A refused unit always delivers at the instant the ban ends.
- **Repair.** Take a unit's work as the time to its first refusal plus the time from the end of that conflict to its receipt. Count a refusal only if it falls inside a closed interval and after readiness. Refuse to return amounts when the re-run does not reproduce the receipt. Draw units that dawdle after a ban in both generators.

### A2. HIGH. K8: the suspension rule in the code counts refusals the text excludes

New material (the new suspension rule).

- **Statement.** K8: "A duty's clock stops from the unit's first refused attempt while the duty is in conflict, if that attempt was made by the deadline, until the conflict ends. A ban the unit never ran into stops nothing."
- **Scenario E1.** One unit, due at 3. Its delivery costs something. It asks at 1 with no funds and is refused. A ban runs from 2 to 6. It is funded and delivers at 7.
- **Derived.** `u0: ready=0, due=3, receipt=7, conflict=['2..6'], first_refusal=1, suspended=5, deadline_sent=8 | InstAL missed=[] | gated: players=[], shares={}, loss=0, under_prohibition=0, reproduces_actual=False; actual_loss=4 | verdict.py with lengths: players=[], miss=0, shares={}`
- **Control E1c,** no ban: `InstAL missed=['u0']`.
- **Scenario P1.** Two units. u1 is ready at 3 and due at 5. A ban on u1 runs from 0 to 4. u1 logs an attempt at 0.5, before its input exists, and delivers at 7.
- **Derived.** `u1: ready=3, due=5, receipt=7, conflict=['0..4'], first_refusal=1/2, suspended=7/2, deadline_sent=17/2 | InstAL missed=[] | gated: players=[], shares={}, loss=0, under_prohibition=0, reproduces_actual=False; actual_loss=2 | verdict.py with lengths: players=[], miss=0, shares={}`
- **Control P1c,** no early attempt: `InstAL missed=['u1'] | gated: players=['u1'], shares={'u1': '2'}`.
- **Correct outcome.** E1: missed. No attempt was refused while the duty was in conflict, the duty was in conflict at its deadline, so by K8 the deadline falls due at 6 and the unit delivered at 7. K11 says an unfunded duty is the unit's miss. P1: missed. At most one day of the ban fell in u1's time, and u1 was two days late.
- **Why.** The code reads `first_refusal` as the first `refuse` in the trace, with no test that the duty was in conflict then or that the unit was ready. The length is the end of the conflict less that time. Round 4 noted that the clip of a suspension to readiness was untested; the new rule has no clip. All three verdicts agree in both scenarios, and all three are wrong, so comparing them cannot find it.
- **Repair.** Count a refusal only at a step where `conflicted` holds, and stop the clock from the later of the refusal and readiness. Two conditions in the test's reading code, and a generator that draws other refusals.

### A3. HIGH. K8: conflict intervals of the actual run are not the intervals the re-run needs

New material (gated.py and the code that feeds it from the trace).

- **Statement.** K8: "What the verdict layer is given. Certified times, budgets, each unit's first refused attempt, and the intervals during which each duty was in conflict. These are facts from the log and the norm layer's trace." "The verdict computed from the trace's intervals and refusals equals the one computed from the generator's bans."
- **Scenario DR.** u0 has two days and delivers at 5. u1 has one day; its duty is issued when its input arrives, at 5; it delivers at 6. A ban on u1's delivery ran from 2.5 to 4.5.
- **Derived.** `actual run: u0: ready=0, due=2, receipt=5, conflict=[] ... | u1: ready=5, due=6, receipt=6, conflict=[], first_refusal=None ... | InstAL missed=['u0'] | gated: players=['u0'], shares={'u0': '3'}, loss=3, under_prohibition=0, reproduces_actual=True; actual_loss=3 ... || the same verdict given the ban itself: shares={'u0': '3/2'}, loss=3, under_prohibition=3/2 || InstAL run again with u0 on time: u1 conflict=['5/2..9/2'], first_refusal=3, receipt=9/2`
- **Correct outcome.** u0 pays 1.5 days and 1.5 days arise under prohibition. Had u0 complied, u1 would have been ready at 2, refused at 3 and delivered at 4.5. InstAL, run on that history, says so.
- **Why.** `conflicted` holds only while the duty is in force and not yet done. On the actual run u1's duty was not in force during the ban, so the trace shows nothing. Whether a duty is in conflict is a fact about one history; the re-run asks about another. The test issues every duty before anything happens, which is the one arrangement in which the two coincide.
- **Scope.** Chains, one duty per unit, a ban on delivery: all as K8 states. Nothing in the document says a duty must be in force before its unit is ready, and a duty that arises on receipt of the input is the ordinary reading of "a unit owes two things for a hand-over".
- **Repair.** Hand over the times at which each way was forbidden by an authority (`forbiddenabove` is in the trace). With more than one way the verdict layer must then combine them, which is rule 4 of K4 inside the verdict. The alternative is to run each counterfactual through the norm layer. Either bears on stop condition 3.

### A4. LOW. K8: the first counterexample and the 501 are against a rule that is not K8's

New material. Listed in the claims table under "proved here".

- **Statement.** K8: "A ban absorbs lateness ... The fixed length charges the first unit a day." "On 2000 random chains with bans the fixed length gave different shares from a re-run in 501."
- **Scenario CE1,** the chain in test_gated.py, by hand and under three rules.
- **Derived.** `re-run: shares={'u1': '0'}, loss=2, under_prohibition=2 | K8's rule (clock stopped from the refusal): shares={'u1': '0'}, miss=0 | test_gated.py's fixed_extension: shares={'u1': '1'}, miss=1`
- **Scenario CE4,** the test's 2000 chains with its seed.
- **Derived.** `re-run against fixed_extension: 501 differ; re-run against K8's rule: 530 differ; K8's rule against fixed_extension: 357 differ`
- **What is false.** The second unit is refused at 4, before its deadline of 5, so K8's rule stops its clock for two days and verdict.py charges the first unit nothing. The day is charged by `fixed_extension`, which moves a deadline to the end of a conflict that covers it. Its docstring calls it "draft 5's first rule". It is in neither draft 4 nor K8.
- **What stands.** The conclusion. CE1b, the same chain with the second unit working its whole budget: `K8's rule (clock stopped from the refusal): shares={'u1': '1'}, miss=1`. The second counterexample (CE2) and the continuity counterexample (CE3) are right by hand and under K8's rule.
- **Repair.** Run the comparison against the rule K8 states, use CE1b, and report 530.

### A5. MEDIUM. K0: validation is still incomplete

Material that existed in round 4. Round 4's A4, partly fixed.

- **What is fixed.** A pattern value that is a string, edges given as lists, and a cost of NaN (W9, W13, W2c).
- **Scenario W4u.** An event names a norm that does not exist: `issue(pp)` for `issue(p)`.
- **Derived.** `issue(a) -> ['issued(a),ministry']; issue(pp) -> []; request(ord_x) -> ['emit(ord_x),unit']`. W4v, `isue(p)`: the same.
- **Scenario W16b.** The key `blocked` written as `block`.
- **Derived.** `issue(p) -> ['issued(p),ministry', 'issued(p),state']; request(ord_x) -> ['refuse(ord_x),unit']`. W16, `routes` written as `rout`: the route is ignored and the order goes through.
- **Scenarios W1, W1c, W3, W3g, W3e, W4, W4e, W7, W8,** round 4's nine. All still raise: `InstalParserSyntaxError` (four), `InstalParserLexerError`, `RuntimeError`, `IndexError`, `TypeError`, `KeyError`.
- **Scenarios W9d, W9e, W13b, W13c, Vr2, Vp,** new: a pattern that is not a mapping, an effect attribute that is a list, an edge of three, edges as a mapping, routes as a list, no procedures. `AttributeError`, `TypeError`, `ValueError`, `ValueError`, `AttributeError`, `IndexError`.
- **Scenarios W2d, W2e.** A cost of `Fraction(5, 2)` or `Decimal('2.5')`: `AssertionError: the cost of e is not a finite number`.
- **Correct outcome.** W4u, W4v, W16, W16b: rejected. The scenario says a prohibition is issued, or a norm declined, and neither happens. This is round 3's misspelt kind of norm in the inputs that are still unchecked. The others: rejected with a message. W2d: accepted, or rejected with a true message; every other model in the directory computes in Fractions.
- **Repair.** Check each event against the known event names and identifiers; reject unknown scenario keys; apply the identifier check to authorities and exclude InstAL's reserved words; check the shape of norms, edges, routes and procedures; accept any rational cost.

### A6. LOW. K0: "Both now fail the suite; I applied each change and ran it"

Material that existed in round 4. Round 4's A5, partly fixed.

- **Scenario M1,** the change as round 3 and round 4 defined it: every unit is written as owning every effect.
- **Derived.** `test_instal.py + test_ledger_link.py + test_end_to_end.py + test_conflict_chain.py: passes`
- **Scenario M1b,** the other tester's form: the owner test replaced by False, which also writes the funds and amendment facts for every unit.
- **Derived.** `fails (test_instal.py: AssertionError: one unit cannot emit another's amendment: step 1 has occurred(emit(am),v))`
- **Scenario M6.** `fails`.
- **What is false.** The new scenario detects a foreign amendment fact. A wrong ownership fact alone still passes all four suites, because the scenario forbids an emission by the other unit and not a refusal.
- **Also.** K0's list of what is "still undetected, by the testers' round-4 runs" names two rejections. My round-4 run reported four. One has a scenario now. The fourth, a unit that is also an authority, is still undetected (VU: `passes`) and is not in the list.
- **Repair.** Assert that the other unit neither emits nor refuses. Add the fourth rejection to the list or give it a scenario.

## List B: design objections and claims stronger than their evidence

| Id | Clause | Objection | Derived |
|---|---|---|---|
| GEN | K8 "The oracle uses something neither layer sees" | On the histories the generator can draw, "missed under the rule" reduces to "intended after the deadline" by arithmetic: refused in time means delivered at the end, which the moved deadline always allows. The oracle is independent in what it reads and not in what it can contradict | see A1 |
| K4, K5, K7 | K8 "from the unit's first refused attempt", "while the duty is in conflict" | Three wrong versions of the rule pass the test: the clock stopped for the whole conflict (draft 4's length); any refusal counted and the clock stopped to the receipt; a conflict after the deadline counted. The test checks who is late and never a length. "By the deadline", the tie orders and the later deadline are covered (K1, K2, K3, K6 fail) | `test_conflict_chain.py: passes` each |
| G7 | K8 "Tested" | The four properties on 2000 chains hold by construction (A1). All seven wrong versions of gated.py I tried fail its tests, so the code is pinned to its conventions; the conventions are what is untested | `failures: 0` |
| D5 | K8 | On one history the two verdicts the test computes give different amounts, and the test compares only their players | `gated: players=['u0'], shares={'u0': '0'} ... verdict.py with lengths: players=['u0'], miss=3, shares={'u0': '3'}` |
| G9 | K8 convention 3 | A complying unit is taken to deliver at the last instant. A ban that starts at that instant excuses half a day of the unit's own lateness; a thousandth later it excuses none. The document gives the discontinuity for arrivals only | `ban from 2: shares={'u0': '0'} ... under_prohibition=5 \|\| ban from 2.001: shares={'u0': '5'}` |
| DOC6 | K8 "No norm rule is evaluated in the verdict layer"; stop condition 3 | `gate()` decides, for a time at which nothing was attempted, that an attempt would have been refused and when it would first be accepted. That is the norm layer's refusal rule applied to a counterfactual request by lookup. A3 is where the lookup and the norm layer differ | `if a <= t < b: t, moved = b, True` against `request(E) generates refuse(E) if owns(E), not allowed(E)` |
| DOC5 | K8 "The evidence layer turns them into one length per unit and places the deadline" | evidence.py does none of it; six lines of the test script do. K4 and K8's open list say "no layer's code sends the later deadline", so the sentence contradicts the document as well. Round 4's A2, half repaired | `evidence.py mentions any of deadline, suspend, conflict, refus: False; test_conflict_chain.py imports evidence.py: False` |
| DOC7 | "What was wrong in draft 4", row for K0 | The row lists the three inputs that changed outcomes and not the nine that raised unhandled errors. All nine still do and the document says nothing of them | `the document mentions 'reserved': False` |
| VE | K0, the scenario "edges given as lists work like edges given as pairs" | It passes when they do not work. With the normalisation removed, the permission loses its power as well as the prohibition, and the order is refused by default. This is a third coverage scenario that does not detect its change | `test_instal.py: passes` |
| NR5, NR6, NR7, VC2, VC3, VC4, VCY2, V2C | K0, K12, the checks written after round 4 | Eight wrong versions pass: no check that a route has two authorities; no type checks on `blocked`; the origin may decline; a negative cost, an infinite cost and a cost of any type accepted; only direct self-creation rejected; two creators accepted. The four route restrictions themselves are covered (NR1 to NR4 fail) | `passes` each |
| VU, VCR2, VPR | K0 | Three rejections K0 lists have no scenario: a unit that is also an authority, a created unit that is not a unit, an origin with no rank | `passes` each |
| RC4 | K0, K6 | A route that ends with no edge to its target is now rejected. A direct norm with no edge to its target is still accepted and binds nobody | `issue(p) -> ['issued(p),ministry']; request(ov) -> ['emit(ov),v']` |
| W6e, W18, Vb6 | K0 | A creator that lists a unit twice is told the unit "has two creators". A rank is demanded where no exemption exists. `blocked` accepts a set and not a frozenset | `AssertionError` each |
| Stop 3 | Stop condition 3 | The condition is worded so that being "told when it was" forbidden never meets it. A3 shows the verdict must be told when delivery would have been forbidden on another history, which only the norm rules can say | see A3 |

## List C: limits the document already lists

| Id | Clause | Limit | Derived |
|---|---|---|---|
| I1, I1c | K4 | An unreachable way counts as a way. Unchanged and stated | `deadline(o) -> ['missed(o),unit']`; one effect fewer: `deadline(o) -> ['conflict(o),unit']` |
| G8, D5 | K8 open | A unit that waits without attempting. The size on one chain: 2.5 days of a five-day loss move from the late unit to a unit that finished in time and logged nothing | `no attempt logged: shares={'u0': '5/2', 'u1': '5/2'} \|\| attempt logged at 4.6: shares={'u0': '5'}` |
| TB | K4 "a conflict that ends and starts again" | Stated as untested. The outcome is wrong and worse than in round 4: a unit refused before its deadline in a second ban is recorded as missing, because the end is taken from the first ban and the length is negative | `conflict=['1/2..1', '2..6'], first_refusal=5/2, suspended=-3/2, deadline_sent=3/2 \| InstAL missed=['u0'] \| gated: players=[]` |
| TB3, TB4 | K4, the same | Two bans that touch; a second ban over the moved deadline. The moved deadline records a conflict and nothing follows | `InstAL missed=[]` with `verdict.py with lengths: players=['u0']` |
| RW1, RW2 | K12 "the log" | A relayed withdrawal, or a relayed issue below a declining authority, observed directly takes effect | `relayed(p) -> ['issued(p),group']; request(ord_x) -> ['refuse(ord_x),unit']` |
| N3 to N15 | K12 status | The eight wrong versions of the route code still pass | `passes` each |
| M11 to M15 | K0 | The five rules of K4 and K5 are still untested | `passes` each |
| VT, VI | K0 | Two rejections still without a scenario | `passes` each |
| K8 open | K8 | Who bears the loss under prohibition; no layer sends the later deadline; two duties per unit | not run |

## Priorities 1 to 7

1. **gated.py.**
   - Inside its conventions the code is right on every case named: two intervals, touching intervals, an interval before readiness, an early unit, zero budgets, a chain of one, an empty chain (G6, G7).
   - A share is never negative, because the receipt is monotone in each work time.
   - The re-run does not reproduce the receipt, and shares do not account for the loss, as soon as a refused unit is late for another reason (A1).
   - The two counterexamples and the continuity counterexample are right by hand. The first is not a counterexample to K8's rule (A4).
   - "No norm rule is evaluated in the verdict layer" is nearly true and not quite (DOC6, A3).
2. **The suspension rule and its test.**
   - The tie orders behave as declared: an attempt refused exactly at the deadline is excused, a ban ending exactly at the deadline changes nothing, an attempt at the instant a ban starts is refused (D1, D2, D3).
   - A unit that attempts twice, and a conflict ended by an exemption, are read correctly (D6, X1). Round 4's three scenarios are fixed (S1, S2, S3).
   - A refusal for another reason, and a refusal before readiness, stop the clock (A2). Two bans give a negative length (TB).
   - InstAL's `missed` and the re-run verdict's players disagree in H1 and E1c. InstAL is right in both.
3. **K12.** No defect found. Routes of three, declining at either position, created units to depth two, withdrawal, norms routed in opposite directions, a routed duty, rank over three authorities (R3a to RK). An intermediate's own edge carries nothing for a routed norm (R3d). The origin's edge to a created unit, or to a creator, does not carry a declined norm (RC5, RC6). The rejections reject for the stated reason (REJ, RD1, RC3).
4. **Round-4 fixes.** Validation: A5 and the B rows. Coverage: A6, VE and the B rows. Each of the suite's 19 rejections fires for the reason in its name.
5. **Round-4 list A.** See the table below.
6. **K2.** 71 runs, one answer set each.
7. **The document.** 67 scenarios and 19 rejections are right, as are the counts for K3 to K7, K9 to K12 and the numbers quoted from the tests (DOC1, DOC3, DOC4). The record for stop condition 5 is as the testers wrote it, and "three of five and one of two" is right (DOC2). The draft-4 table has a row for every round-4 finding; one "Now" entry overstates (DOC7). Described as proved and not: the first counterexample (A4). Described as done by a layer and done by a test: the length and the deadline (DOC5).

## The round-4 list A, re-run

| Round-4 finding | Status | Evidence from this run |
|---|---|---|
| Fable A1. A ban the unit never ran into excuses its lateness | Fixed | S1, S2, S3: `InstAL missed=['u0']`. The replacement has A2 |
| Fable A2. The verdict and evidence layers are not run | Partly fixed | verdict.py and gated.py are called and their shares asserted to sum. evidence.py is still not involved (DOC5), and amounts are never compared (D5) |
| Fable A3. The adapter's handling of route input | Fixed | T3, Vr3 to Vr6, Vb1 to Vb3 rejected with a message; K0 lists the route facts (DOC9) |
| Fable A4. Validation incomplete | Partly fixed | Three silent cases fixed; nine unhandled errors remain; new silent cases (A5) |
| Fable A5. Two of the six coverage repairs do not detect their change | Partly fixed | M6 fails the suite. M1 as defined passes (A6) |
| Astra A1. A route that returns to an authority goes round one that declined | Fixed | AC: `AssertionError: route of p visits an authority twice`; with the check removed the suite fails (NR1) |
| Astra A2. The same two coverage repairs | Fixed in the form that tester used | M1b and M6 fail the suite |

Of the seven: three fixed, one fixed in one form, three partly fixed, none not fixed, none moved to a stated limit, none no longer applicable.

## Stop conditions

1. **A scenario has no trace or several.** Not met. 71 runs, one answer set each.
2. **A repair that needs a change to InstAL's semantics.** Not met. Every repair above is in gated.py, the test's reading code, the adapter or the text. In A1, A2 and A3 InstAL's own output is the correct one.
3. **K8 cannot be made without restating the norm rules inside the verdict layer.** The document's "not met on the case tried" is accurate about the case tried and says less than it appears to.
   - The case tried is one in which every duty is in force before anything happens and every refused unit delivers the instant its ban ends. Outside it the re-run verdict is wrong in three ways (A1, A2, A3).
   - A1 and A2 can be repaired without norm rules in the verdict layer.
   - A3 cannot be repaired by handing over more of the same facts. The verdict needs to know when delivery would have been forbidden on a history that did not happen. With one way per duty that is an interval per way. With several it is rule 4 of K4, evaluated in the verdict layer or by running the norm layer again.
   - So the condition is not shown to be met, and "not met" is not shown either. The honest status is open, with the evidence leaning toward met for anything beyond one way per duty.
4. **K11 cannot be made by events alone.** Not met, in the simple form only. Not re-attacked beyond the control Z1m.
5. **The defects are not converging.** On my count the condition's first half is now satisfied for one round: 21, 15, 6, 5, 6. It needs the other tester's count, and a second round, to be met.
   - In material that existed in round 4 the count is 2, both residues of round-4 findings.
   - The other 4, with all three HIGHs, are in the two things written in one pass after round 4. This is the third round running in which the new material holds most of the defects, and the first in which it holds enough to stop the count falling.

CONFIRMED WRONG OUTCOMES (list A): 6
OF WHICH IN NEW MATERIAL: 4
CRITICAL: 0  HIGH: 3  MEDIUM: 1  LOW: 2

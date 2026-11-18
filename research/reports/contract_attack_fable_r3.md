# Attack on the composition contract, round 3 (Fable)

Target: [Composition contract.md](/root/mic/reports/Composition contract.md), draft 3, and the models in /root/mic/model as changed after round 2.

Every scenario is in [attack_contract_fable_r3.py](/root/mic/model/attack_contract_fable_r3.py):

```
python3 /root/mic/model/attack_contract_fable_r3.py            # about three minutes
python3 /root/mic/model/attack_contract_fable_r3.py --fast     # skips the mutation checks and the random chains
```

The "derived" lines below are copied from that run. Identifiers such as I1 are the labels the script prints. No existing file was changed. The seven documented checks all pass. I did not read the other tester's round-3 report.

## Summary

- **The new K4 does what its six rules say.** I ran 25 scenarios against them (exemptions before, after, withdrawn, re-issued, equal-ranked, own, inherited; conflict and lateness in every order I could think of; discharge) and found no case where the encoding departs from the letter of a rule.
- **The letter of rule 4 gives one wrong outcome, and it is a defect that round 1 found and draft 2 had repaired (HIGH).** A unit whose every path to a duty is forbidden by an authority is recorded as missing the duty, because a way that can never be taken still counts as a way.
- **Round 2's "never late afterwards" is half repaired (MEDIUM).** The institution can record the miss at a second deadline event. Nothing sends one, and the K8 timeline places each deadline once.
- **The adapter accepts input that changes outcomes silently (MEDIUM).** A misspelt norm kind or edge makes a prohibition inert; a creation edge can contradict an authority edge.
- **Three statements in the document are false (LOW each).** K0's status sentence, the record of rounds 1 and 2, and what K4 says about InstAL's literature.
- **K8's agreement is arithmetic, as the document now says.** I could not make the layers disagree inside K8's scope, and they cannot. Outside it, under one prohibition, they disagree at once.
- **K2 holds.** All 94 InstAL runs gave exactly one answer set.
- **Count: 6 confirmed (list A), against 15 in round 2 and 21 in round 1.** Four of the six are carried over or reintroduced; two are new.

The script marks 17 scenarios as A-DEFECT. They group into the six findings below. I count by finding, as in round 2.

## List A: confirmed wrong outcomes or false statements inside the stated scope

### A1. HIGH. K4 rule 4, K7: every path to the duty is forbidden by an authority, and the unit is recorded as missing it

- **Scenario I1.** Procedure in force: officer. The regulator obliges an amendment to `board`. The ministry forbids every amendment that uses `officer`. Declared effects: `am_o_b` (officer to board), `am_o_v` (officer to vote), `am_v_b` (vote to board). Events `issue(o), issue(p), request(am_o_b), request(am_o_v), request(am_v_b), deadline(o)`.
- **Derived.** `three amendments declared: issue(o) -> []; issue(p) -> []; request(am_o_b) -> ['refuse(am_o_b),unit']; request(am_o_v) -> ['refuse(am_o_v),unit']; request(am_v_b) -> ['refuse(am_v_b),unit']; deadline(o) -> ['missed(o),unit'] | control, same norms, am_v_b not declared: ... request(am_o_b) -> ['refuse(am_o_b),unit']; request(am_o_v) -> ['refuse(am_o_v),unit']; deadline(o) -> ['conflict(o),unit']`
- **Scenario I2.** A duty to amend to `board` whose only declared effect is `am_v_b`. No prohibition at all.
- **Derived.** `issue(o) -> []; request(am_v_b) -> ['refuse(am_v_b),unit']; deadline(o) -> ['missed(o),unit']`
- **Correct outcome.**
  - I1: a conflict and no miss. The ministry's prohibition stops the direct way and the only step toward the other way. The unit tried all three and was refused three times. Nothing it could have done would have met the duty.
  - I2: the adapter should reject the scenario. K0 says it rejects "a duty with no declared way its target can take".
- **Cause.** `openway(O)` holds for any matching effect that no authority's prohibition names. `procfail` is not consulted. The adapter's assertion checks only who owns the effect.
- **Why this is in list A although rule 4 is implemented as written.** Rule 4 says "every declared way of meeting it is forbidden by an authority", and `am_v_b` is declared and not named by a prohibition. The outcome is still wrong, for three reasons.
  - It is round 1's H5 ("a way that cannot be taken still counts as a way"), rated HIGH then and accepted as a defect. Draft 2 repaired it by making feasibility count. Draft 3 removed that repair together with the exception rule, and the table of draft-2 defects does not say the earlier defect is back.
  - The outcome turns on a declaration that changes nothing the unit can do. Declaring one effect fewer gives the conflict.
  - K7's sentence covers a unit that "amends away its only way". This unit amended nothing. K11's limit covers funds. Neither covers this.
- **Repair.** Count a way as open only if it can be reached: for an amendment, its procedure is in force or can be reached by declared amendments that no authority forbids. To avoid round 2's A2, keep an inertial fluent that records that the duty had an open, reachable way when it came into force, so a unit that destroys such a way is still recorded as missing. Both changes are in `unit.ial`. In the adapter, make the "no declared way" assertion use the same test. At the least, rule 4 and K7 must say that this unit is recorded as missing.

### A2. MEDIUM. K4 rules 4 and 5: a duty in conflict at its deadline is late afterwards only if the deadline is sent again, and nothing sends it

Round 2's A4, partly fixed.

- **Scenario R1.** Regulator duty, ministry prohibition. Events `issue(o), issue(p), deadline(o), withdraw(p), request(rep_r1)`.
- **Derived.** `issue(o) -> []; issue(p) -> []; deadline(o) -> ['conflict(o),unit']; withdraw(p) -> []; request(rep_r1) -> ['emit(rep_r1),unit']`
- **Control R2,** the documented test: the same with `deadline(o)` sent twice more after the withdrawal.
- **Derived.** `... deadline(o) -> ['conflict(o),unit']; withdraw(p) -> []; deadline(o) -> ['missed(o),unit']; deadline(o) -> []; request(rep_r1) -> ['emit(rep_r1),unit']; misses recorded: 1`
- **Scenario R3,** the construction of `test_end_to_end.py`. u1's receipt is due at 4 and happens at 6. A lower authority forbids delivery from 3 to 5.
- **Derived.** `timeline ['3: deadline(resp_u1)', '4: deadline(recv_u1)', '5: withdraw(ban)', '6: request(h_u1)']; InstAL: conflict(recv) ['u1'], missed(recv) [], emitted ['u0', 'u1']; verdict: players ['u1'], shares {'u1': '2'}, miss 2`
- **Correct outcome.** Once the conflict ends the duty is owed and not in conflict, so by rule 5 it can be late. R1 and R3 record no lateness however long the unit then waits.
- **What is and is not repaired.** The institution no longer sets `pastdue` at a deadline in conflict, so a later deadline event records the miss. But a duty has one deadline. No clause says who sends a second one, or for what time. The only code that places deadlines places each once.
- **Scope.** R1 is a K4 scenario and is inside the scope. R3's disagreement with the verdict falls under K8's listed open item "units under prohibitions at the same time"; it is here to show that the composed system never produces the second deadline that K4's own test relies on. This is not the listed "act window before a deadline": the conflict covers the deadline and ends after it.
- **Repair.** State in K4 and K8 that when a conflict ends, the evidence layer sends the duty a fresh deadline, and say how it is placed (for example the end of the conflict plus the time the unit had left). Add that event to the K8 timeline and a scenario with a single original deadline.

### A3. MEDIUM. K0: the adapter accepts input that should be rejected, and the outcome changes silently

- **Scenario V1.** A prohibition written with kind `forbid`.
- **Derived.** `issue(p) -> []; issue(a) -> []; request(rep_r1) -> ['emit(rep_r1),unit']`
- **Scenario V2.** An edge `("ministri", "unit")`.
- **Derived.** `issue(p) -> []; issue(a) -> []; request(rep_r1) -> ['emit(rep_r1),unit']`
- **Scenario V3.** `created={"unit": ["sub"]}`, an edge `("sub", "unit")`, and a prohibition whose origin is `sub`.
- **Derived.** `issue(o) -> []; issue(p) -> []; request(rep_r1) -> ['refuse(rep_r1),unit']; deadline(o) -> ['conflict(o),unit']`
- **Scenario V4.** A permission that carries `against="p"`.
- **Derived.** `issue(p) -> []; issue(a) -> []; request(rep_r1) -> ['refuse(rep_r1),unit']`
- **Scenario V5.** Four inputs that raise an unhandled error where a rejection is due.
- **Derived.** `an amendment effect with no 'used' or 'to': KeyError: 'used' | a cost that is not a number: TypeError: '>' not supported between instances of 'str' and 'int' | a unit also listed as an authority: InstalParserNotDeclaredError: ... | an identifier with a capital letter: RuntimeError: <string>:1:17: error: unexpected token: P`
- **Correct outcome.** Each is rejected with a message.
  - V1 and V2: a misspelt prohibition or edge fails open. The adapter writes `initially(isforbid(p), unit)` and InstAL ignores the undeclared fluent.
  - V3: a unit puts itself in conflict through something it created. Rule 4's "prohibitions the unit issued for itself do not count" is one indirection deep. "Units under common control" is listed as not covered; an adapter that accepts a creation edge contradicting an authority edge is not.
  - V5: `test_instal.py`'s own `rejected` helper catches only `AssertionError`.
- **Repair.** Six assertions in the adapter: the kind is one of the four; each edge runs from an origin or listed authority to a unit of the run; `created` names only units, and no edge runs from a unit's subtree to it; `against` appears only on an exemption; an amendment has `used` and `to` among the procedures; authorities and units are disjoint.
- **Related, not counted.** V6: a rank is demanded for every origin although rank is used only for exemptions. This is rejected input that could be accepted.

### A4. LOW. K0: the status sentence is false for four facts and two rejections

Round 2's A9, not fixed, and wider now that the sentence also claims the rejections.

- **Statement.** "Each fact has at least one scenario that fails if the adapter gets it wrong, and each rejection has a scenario."
- **Scenarios M1, M2, M4, M6, M8, M9.** Each changes one line in a copy of the adapter and runs the documented checks.
- **Derived.**

| Mutant | Change | Checks run | Result |
|---|---|---|---|
| M1 | every unit owns every effect | test_instal, test_ledger_link, test_end_to_end | passes |
| M2 | an equal rank counts as outranking | test_instal, test_ledger_link | passes |
| M4 | exemptions are not inherited by created units | test_instal, test_ledger_link | passes |
| M6 | a norm also matches the effects of other units | all three | passes |
| M8 | the rejection "an effect with no owner" removed | all three | passes |
| M9 | the rejection "a norm with no target" removed | all three | passes |

- **Controls.** M3 (same-origin exemption disabled), M5 (permissions not inherited) and M10 (the edges rejection removed) fail the checks, as they should.
- **Repair.** Six scenarios: two units where only ownership separates an outcome; an equal-ranked exemption; an exemption inherited by a created unit; a duty whose conflict depends on another unit's effect not matching; the two rejections. Or weaken the sentence.

### A5. LOW. The record of rounds 1 and 2 is understated

Round 2's A11, not fixed.

- **Scenario DOC1.**
- **Derived.** `says 'Round 1 found eleven': True, while its own record of condition 5 says 21 and eight: True; says 'every defect so far was repaired in the unit institution or the adapter': True; rows in the draft-2 table: 12; round-2 list-A findings: Fable 15, Astra 6; Fable findings with no row: ['A9 (K0 status sentence)', "A10 (K9 'enumerates')", 'A11 (record of round 1)', 'A13 (own norm cannot be repealed)', 'A14 (free purchase refused)']; empty history credited to: ['Astra'] (also Fable A15: True)`
- **What is false.**
  - "Round 1 found eleven wrong outcomes or false statements." Eleven is the number of rows in draft 2's table. The same document's record for condition 5 gives 21 and eight.
  - Stop condition 2: "every defect so far was repaired in the unit institution or the adapter." The exception rule was withdrawn, not repaired; K4 calls it "a rule that could not be made right". A13 was moved to "not covered". A9 and A11 are still open (A4 and this finding).
- **What is accurate.** The twelve rows of "What was wrong in draft 2" each describe a real round-2 finding. One attribution is incomplete: both testers reported the empty history. The table is not said to be complete, and it is not: five of my fifteen have no row, and three of those five are still open.
- **Repair.** Give the counts the testers gave. Replace "every defect was repaired" with a list: repaired, withdrawn, moved to a limit, open.

### A6. LOW. K4: what InstAL's literature does about conflicts

- **Statement.** "InstAL has no priority among institutions, and its literature detects conflicts between them and leaves the resolution to the parties." And: "I have not checked these sources beyond what the reviewers reported."
- **Scenario DOC2.**
- **Derived.** `sentence in the document: True; 'I have not checked these sources beyond what the reviewers reported': True; round 1 (Fable L5) reported the opposite: True`
- **What is false.** The cited paper is about detection only ([Li et al.](https://researchportal.bath.ac.uk/en/publications/legal-conflict-detection-in-interacting-legal-systems/), 2013). The same authors, in the same language, resolve the conflicts they detect by revising the institutions with inductive logic programming: Li's Bath thesis, [Normative conflict detection and resolution in cooperating institutions](https://researchportal.bath.ac.uk/en/studentTheses/normative-conflict-detection-and-resolution-in-cooperating-instit/), and the paper on [revising secondary legislation](https://researchportal.bath.ac.uk/en/publications/a-model-based-approach-to-the-automatic-revision-of-secondary-leg/). Round 1 said so from memory; I checked it online this round.
- **The other two sources are as described.** Cedar refuses by default and lets a forbid override a permit. [arXiv 1212.0079](https://arxiv.org/abs/1212.0079) (Governatori, Olivieri, Rotolo, Scannapieco) treats permissive norms as explicit exceptions to opposing obligations. "The only thing rank is used for here is deciding whether an exemption works" is true of the code.
- **Repair.** "The cited paper detects conflicts. Later work by the same group resolves them by revising the institutions. This contract neither revises nor ranks."

## List B: design objections and claims stronger than their evidence

| Id | Clause | Objection | Derived |
|---|---|---|---|
| W1, W2 | K5 against K4 rule 4 | The same norms give a conflict when the prohibition is enforced and a miss or a breach when it is watched. K5 says so, so it is as specified. But the unit is excused where the prohibition can stop it and blamed where it cannot, and this conflict is not detected at all | `watched: ... deadline(o) -> ['missed(o),unit']; request(pay_h) -> ['breach(pay_h,p),unit', 'emit(pay_h),unit'] | enforced: ... deadline(o) -> ['conflict(o),unit']; request(pay_h) -> ['refuse(pay_h),unit']` |
| I5 | K7 against K4 rule 4 | A unit amends away its lawful way and an authority then forbids that way too. K7 says missed; rule 4 says conflict; the encoding follows rule 4. The document does not say which sentence wins | `request(am_o_v) -> ['emit(am_o_v),unit']; issue(q) -> []; deadline(o) -> ['conflict(o),unit']` |
| M11, M12, M15 | K4 rule 5 | "Recorded once", the clearing of `pastdue` and `done` on re-issue, and discharge by a late performance are untested: removing each passes every check. The test named "a missed duty is a finding, recorded once" does not look at the second deadline | `passes` |
| M13 | K4 rules 3, 4 | No scenario has an exempted duty reach its deadline, so "an exempted way is an open way" is untested | `passes` |
| M14 | K5 | "A watched prohibition never puts a duty in conflict" is untested: the scenario with that name discharges the duty before the deadline | `passes` |
| M7, M16 | K0, K11 | Which norms are a unit's own, and a grant funding only the owner's institution: no check depends on either (round 2, M3 and M10) | `passes` |
| W5 | K5, claims table | "From the literature: InstAL's power and permission." The unit institution grants every permission and power at the start and never changes them. Prevention is the fluent `allowed`; which prohibitions are watched is a fact the adapter writes from `self_declared`, which K0 does not list. The row was in draft 2 too | `request(rep_r1) -> ['refuse(rep_r1),unit']; InstAL's own perm(emit(rep_r1)) holds at the step of the refusal: True` |
| DOC3 | K4 | "The contract now follows InstAL's own approach." A conflict here is recorded only at a deadline event, only for a duty, inside one unit institution. Li et al. compare two institutions' verdicts on one event | see R4, W1 |
| R8 | K4 rule 5 | Re-issuing a missed duty that is still owed records the miss again. "Once" holds per issue (round 2, P3) | `misses recorded: 3` |
| R9 | K4 rule 1 | Default refusal binds only a unit that does not use its own procedure: an own duty or permission permits the act, each time it is re-issued | `request(rep_r1) -> ['refuse(rep_r1),unit']; ownissue(o,officer) -> []; request(rep_r1) -> ['emit(rep_r1),unit']; ...` |
| X5 | K0, K4 rule 3 | The rank table is not checked against the edges. A unit ranked above its authority exempts itself (round 2, G3) | `ownissue(x,officer) -> []; request(rep_r1) -> ['emit(rep_r1),unit']` |
| X6, X8 | K4 rule 3, K3 | Any authority that outranks the unit sets aside the unit's own prohibition. An exemption is not tied to a duty and does not end when the duty is discharged, so one "explicit act" is a standing partial repeal by a third party | `request(rep_x) -> ['emit(rep_x),unit']; request(rep_r2) -> ['emit(rep_r2),unit']` |
| K6-2 | K6, K4 rule 3 | A created unit does what its creator may not, through an exemption from an authority with no edge to the creator. Rank is one table for the run, not a relation among authorities over the same unit | `request(ox_u) -> ['refuse(ox_u),unit']; request(ox_v) -> ['emit(ox_v),v']` |
| K6-4 | K6, K3 | An authority over the creator cannot address the created unit alone. The norm is issued and binds nobody, silently | `issue(a) -> []; issue(p) -> []; request(ox_v) -> ['emit(ox_v),v']` |
| K7-1, Z3 | K5, K0 | The adapter reads `kind`, `used`, `to`, `cost` and `of` as facts whether or not the scenario lists them as self-declared. A unit that declares its own cost declares the purchase free | `request(am_o_v) -> ['breach(am_o_v,p),unit', 'emit(am_o_v),unit']`; `request(b) -> ['emit(b),unit']` |
| V6, V7 | K0 | A rank is demanded when no exemption exists. An effect that omits the attribute a prohibition reads is not matched (round 2, G1) | `AssertionError: no priority for the origin of a`; `request(rep) -> ['emit(rep),unit']` |
| N1, N3 | K9.5 | Unchanged from round 2. With two lumps, resolving one asks A, B and C for 1 each against a final 7/12. If the evidence never arrives, a reporting unit stays charged with a hidden deviator's share, so "the loss stays where it fell" is not true of a lump that cancels | `present ACF: pays {'A': '1', 'B': '1', 'C': '1'}, suspense 0, provisional True`; `final {'B': '1/2', 'C': '1/2'}; A and B missing: pays {'C': '1'}` |
| N6 | K9 | The grid check runs no model code and is one of the "11 checks". Three stale "old rule" labels remain in `evidence.py` and `test_evidence.py` | `the grid loop calls the model: False` |

Three further objections without a scenario of their own:

- **K8.** "The evidence layer places the two deadlines." No evidence-layer code does. The test script places them with `verdict.receipt` and `verdict.handover`, so the norm layer's input is computed by the verdict layer. The receipt is certified by the receiver's witnesses (K9.1) and is modelled as the sender's own emission.
- **K8, two duties.** Giving the norm layer a response duty and a receipt duty copies the verdict's split between own lateness and link lateness into the norm layer. That is the reverse of what stop condition 3 names, and it is why the two now agree.
- **Stop condition 5.** "K4 has now been replaced, so its count starts again." The condition counts a tester's findings on the scope. It has no count per clause to restart.

## List C: limits the document already lists

| Id | Clause | Limit | Derived |
|---|---|---|---|
| I4 | K4, K11 | One way forbidden by an authority and the other not funded is a miss | `deadline(o) -> ['missed(o),unit']` |
| R4 | K4 | A conflict that starts and ends between deadlines leaves no record; the unit, refused while it lasted, is then recorded as missing (act window) | `request(rep_r1) -> ['refuse(rep_r1),unit']; withdraw(p) -> []; deadline(o) -> ['missed(o),unit']` |
| W4 | K5 | An exemption that reads a self-declared attribute lets the unit declare itself out of an enforced prohibition | `request(pay_t) -> ['emit(pay_t),unit']; request(pay_h) -> ['refuse(pay_h),unit']` |
| K6-3 | K6 | A creator's own prohibition does not follow into the unit it creates | `request(ox_u) -> ['refuse(ox_u),unit']; request(ox_v) -> ['emit(ox_v),v']` |
| K1-2 | K1 | Clocks that differ. With the receiver's clock half a day ahead, on-time units are recorded late. With the bound added to the allowance, a link a quarter of a day slow is not recorded while the verdict charges it. The bound is in no file | `InstAL late receipts ['u0', 'u1'], verdict players [] | ... InstAL late receipts [], verdict players ['u0'], shares {'u0': '1/4'}` |
| E2 | K8 | A hand-over under a prohibition: the norm layer records a conflict and refuses a receipt that happened; the verdict charges the unit 2 | `conflict(recv) ['u1'], missed(recv) [], hand-overs emitted ['u0'], refused ['u1']; verdict: players ['u1'], shares {'u1': '2'}` |
| Z2 | K11 | A grant outstanding when the duty is discharged another way stays until a later refusal | `funded(b1) after the discharge: True` |

## Priorities 1 to 6, clause by clause

- **K4, exemptions.** Issued before the prohibition, withdrawn, re-issued, from an equal-ranked origin, from the unit itself at rank 9, covering one of two prohibitions on the same way, inherited by a created unit: all as the rules say (X1 to X4, X7, K6-1). An exemption from the prohibition's own issuer is covered by the documented test and by control M3. An exemption against a duty, against itself, or with no target is rejected (V8).
- **K4, conflict.** Not recorded for a duty not in force, discharged or withdrawn (R6). A duty cannot be discharged while in conflict, because every way is then refused. A unit cannot put itself in conflict by its own norms, amendments or emissions; the one route I found is the adapter's (V3).
- **K4, lateness and discharge.** One miss per issue; missed, then conflict, then free records one of each (R5). One emission discharges every duty in force and none issued later (G1). A discharged duty permits neither of its ways (G2).
- **K1.** As restated, the clause is consistent with the model. The tie rule K8 relies on is the test script's, not F2's (K1-1). The clock bound is named and nowhere implemented (K1-2).
- **K2.** 94 runs, one answer set each, including repeated issues and withdrawals, a 35-step history, ill-typed, wrong-arity and institutional events observed directly, and an amendment to the procedure in force. Institutional events observed directly have no effect.
- **K5.** An exemption from a lower authority does not cover a watched prohibition, and the breach is recorded (W3). The objections are W1, W2, W5 and M14.
- **K6.** A created unit inherits prohibitions, permissions and exemptions, and loses them on withdrawal (K6-1). It can do what its creator may not in three ways: its creator's own prohibition does not follow (listed), an authority with an edge to it alone exempts it (K6-2), and it permits itself (as any unit can).
- **K7.** After an amendment an own rule takes effect only by the new procedure (K7-2). An own exemption lifts an own prohibition, which is a repeal in all but name (R7).
- **K8.** On 20 random chains with ranges far wider than the test's (receipt before hand-over, response before readiness, no allowance) the layers disagree in 0. They cannot disagree: "missed the response duty" is own lateness above zero and "missed the receipt duty" is own plus link above zero, and those are the two inequalities the verdict evaluates, on times computed by the verdict's own functions. A real test needs three things: a timeline built from per-unit logs by code that does not import `verdict.py`; a history in which the norm layer has something of its own to say, such as E2 or R3; and clocks that differ within the bound (K1-2).
- **K9.** Exactness holds on 2000 further random chains with the first and last units included (N5). The two proofs are sound as qualified. The four counterexamples in the table reproduce.
- **K11.** A free purchase is now emitted (Z1). The loop in `test_ledger_link.py` cannot run that history, because it asserts that every emission had a grant. Nothing else new.

## The round-2 list A, re-run

| Round-2 finding | Status | Evidence from this run |
|---|---|---|
| A1. Watched prohibition yields where an enforced one would not | Fixed | `request(pay_h) -> ['breach(pay_h,p),unit', 'emit(pay_h),unit']`. They still differ the other way (B: W1) |
| A2. The unit decides which ways are feasible | Fixed; the opposite defect is back | `request(am_v_b) -> ['refuse(am_v_b),unit']; deadline(o) -> ['missed(o),unit']`; `request(sup) -> ['refuse(sup),unit']; deadline(o) -> ['missed(o),unit']`. See A1 |
| A3. A duty performed while overridden is not discharged | No longer applicable | Nothing is performed while forbidden. `withdraw(p) -> []; request(rep_r1) -> ['emit(rep_r1),unit']; deadline(lo) -> []` |
| A4. A duty overridden at its deadline can never be late | Partly fixed | A second deadline event records the miss; nothing sends one. See A2 |
| A5. K8: layers disagree when a unit is late and its link fast | Fixed | `late responses ['u0'], late receipts []; verdict findings ['u0'], players []`; the generator now draws fast links |
| A6. K1 denied the cross-unit order K8 needs | Partly fixed | Fixed in the text. `a clock bound appears in test_end_to_end.py or evidence.py: False` |
| A7. Order of the unit list and default edges decide who is bound | Fixed | `AssertionError: norm p needs a target`; `AssertionError: with several units the authority edges must be given`. Edge endpoints are still unchecked (A3) |
| A8. A creator's norm for a created unit binds the creator | Fixed | `AssertionError: own norm p cannot target another unit`. A creator still cannot bind what it creates; now listed |
| A9. K0's status sentence false for two facts | Not fixed | Now four facts and two rejections. See A4 |
| A10. K9: "the test enumerates the allocations" | Fixed | The sentence now says what the loop does. The loop still runs no model code (B: N6) |
| A11. The record of round 1 is understated | Not fixed | "Round 1 found eleven"; "every defect so far was repaired". See A5 |
| A12. A duty whose only way belongs to another unit | Fixed | `AssertionError: obligation o has no declared way its target can take`. The same hole remains for a way nobody can take (A1, I2) |
| A13. A unit's own norm can never be repealed | Not fixed | Now listed as not covered. An own exemption works as a repeal (R7) |
| A14. A purchase that costs nothing is refused for ever | Fixed | `request(b0) -> ['emit(b0),unit']` |
| A15. The empty history has no trace | No longer applicable | Rejected by design, and K0 says so |

Of the fifteen: eight fixed, two partly fixed, three not fixed, two no longer applicable.

The other tester's six, from the same run: the watched exception (fixed), the infeasible prohibited effect that suppressed a miss (fixed: `deadline(o) -> ['missed(o),unit']; ... misses recorded: 1`), ownership in the declared-way check (fixed), the documented form of events (fixed: rejected with a message, and the documentation now says strings), the round-1 counts (fixed), and the comparison rule's label (fixed in the function named; three stale labels remain).

**Did dropping the inferred exceptions reintroduce anything?** One thing: round 1's H5 (A1 above). Round 1's H6, that the same act is treated differently under a watched and an enforced prohibition, also holds again, in the opposite direction and now by the stated design (B: W1). The other round-1 repairs still hold in the documented checks, which pass: a deadline does not lift a norm, a late hand-over is accepted, depth-two inheritance, the owner check, and the release of a refused reservation.

## The claims table

- **Counts that match.** K3 5; K4 18 (9 + 5 + 4); K5 3; K6 3; K7 7; 43 norm scenarios in all; K8 25; K9 11 and 700; K10 15 and 300; K11 6, 2000 and 5.
- **K0 "a scenario per fact and per rejection".** False (A4).
- **K4 "18".** Parts of rules 3, 4 and 5 have no scenario that depends on them (B: M11, M12, M13, M15).
- **K5 "3".** The clause's last sentence is untested (B: M14). The literature column is not what the encoding uses (B: W5).
- **K6 "3, depth two".** None has an inherited exemption (A4, M4).
- **K8 "25 chains, wiring only".** Accurate.
- **K9 "11 checks".** Ten run model code (B: N6).

## Stop conditions

1. **A scenario has no trace or several.** Not met. 94 runs, one answer set each. The four inputs that raise errors (V5) fail in the adapter or InstAL's parser before solving.
2. **A repair that needs a change to InstAL's semantics.** Not met as worded. Every repair above is in `unit.ial`, the adapter, a test or the text. But the record is wrong (A5): the exception rule was not repaired, it was withdrawn after two attempts. Whether a priority rule can be made right without changing InstAL was the question this condition was there to answer, and it has been set aside rather than answered. The document should say that.
3. **K8 cannot be made without restating the norm rules inside the verdict layer.** Undecided, and the evidence now leans toward met. Inside K8's scope the agreement is arithmetic and can decide nothing. One step outside it, under a single prohibition, the norm layer excuses a unit and the verdict charges it the whole miss (E2, R3). To make them agree the verdict has to know which lateness fell inside a conflict, which is rule 4 restated in the verdict layer.
4. **K11 cannot be made by events alone.** Not met, in the simple form only, as the document says. Nothing new.
5. **The defects are not converging.** Not met on my count: 21, then 15, then 6. Two cautions. The scope is not quite the same, since the clause that held most of round 2's defects was replaced by a simpler one. And four of the six are old: A1 is round 1's H5, A2 is round 2's A4, A4 is round 2's A9, A5 is round 2's A11. The condition also needs the other tester's count.

CONFIRMED WRONG OUTCOMES (list A): 6
CRITICAL: 0  HIGH: 1  MEDIUM: 2  LOW: 3

# Attack on the composition contract, round 2 (Fable)

Target: [Composition contract.md](/root/mic/reports/Composition contract.md), draft 2, and the models in /root/mic/model as changed after round 1.

Every scenario is in [attack_contract_fable_r2.py](/root/mic/model/attack_contract_fable_r2.py):

```
python3 /root/mic/model/attack_contract_fable_r2.py            # about 90 seconds
python3 /root/mic/model/attack_contract_fable_r2.py --fast     # skips the mutation and repair checks
```

The "derived" lines below are copied from that run. Identifiers such as W1 are the labels the script prints. No existing file was changed. The seven documented checks all still pass.

## Summary

- **The round-1 repairs hold on the scenarios they were written for.** Of the 21 distinct round-1 findings, 11 are fixed, 4 partly fixed and 6 not fixed. Three of the six are now listed as limits.
- **Two of the repairs are wrong in a new way (both HIGH).**
  - The exception test added for watched prohibitions is weaker than the one for enforced prohibitions, so K5's statement is false.
  - "A way that cannot be taken is not a way" lets the unit decide which ways can be taken, so a unit can buy itself an exception or avoid a recorded miss.
- **The `pastdue` repair has two holes (both MEDIUM).** A duty performed while overridden is not discharged and is later recorded as missed. A duty overridden at its deadline can never be late afterwards.
- **K8's test agrees with the verdict only because its generator never draws a fast link.** With one allowed, 3 of 25 chains disagree.
- **K1 and K8 contradict each other** about whether an order across two units' logs is needed.
- **K2 holds.** All 87 InstAL runs gave exactly one answer set.
- **Count: 15 confirmed (list A), against 21 in round 1.** Five of the 15 are carried over unrepaired.

The script marks 23 scenarios as A-DEFECT; they group into the 15 findings below. I count by finding.

## List A: confirmed wrong outcomes or false statements inside the stated scope

### A1. HIGH. K5 with K4: a watched prohibition yields where an enforced one would not

- **Scenario W1.** The regulator obliges a payment (`kind=pay`). The lower-ranked ministry forbids payments for hospitality; `purpose` is self-declared, so the prohibition is watched. Effects `pay_h` (hospitality) and `pay_t` (travel, forbidden by nothing). Events `issue(p), issue(o), request(pay_h)`.
- **Derived.** `watched: issue(p); issue(o); request(pay_h) -> ['emit(pay_h),unit'] | enforced: issue(p); issue(o); request(pay_h) -> ['refuse(pay_h),unit']`
- **Scenario W2.** A supra duty to pay. The state forbids hospitality (watched). The group forbids vendor t (enforced). `pay_h` is touched by the state's ban, `pay_t` by the group's.
- **Derived.** `state's prohibition watched: ... request(pay_t) -> ['refuse(pay_t),unit']; request(pay_h) -> ['emit(pay_h),unit'] | both enforced: ... request(pay_t) -> ['emit(pay_t),unit']; request(pay_h) -> ['refuse(pay_h),unit']`
- **Correct outcome.** K5 records a breach "except where an enforced prohibition would have given way to a higher duty". In W1 an enforced prohibition does not give way, because the duty has a free way. So `breach(pay_h,p)` must be recorded. In W2 rule 4 says the lowest prohibition yields: the group's ban should yield and the state's should hold. Instead the unit is forced onto the act the higher authority forbids, and nothing is recorded.
- **Cause.** `wexcepted` has no "cheaper way" test, and `touchedge` reads only enforced prohibitions, so a way touched by a watched prohibition counts as free.
- **Repair.** Compute `touchedge` over all prohibitions in force, and give `wexcepted` the same `not cheaperway` condition. Applied to a copy (script, REPAIR): the 35 scenarios pass, W1 records the breach, and in W2 the group's ban yields.
- **Why not CRITICAL.** K5's sentence is right as a specification; the encoding does not implement it.

### A2. HIGH. K4 rules 2 to 4: the unit decides which ways are feasible

Rule 2 is implemented as written. The outcomes are wrong because feasibility is under the unit's control.

- **Scenario F1.** Procedure in force: officer. The regulator obliges an amendment to `board`. The ministry (lower) forbids amendments that use `vote`. Lawful way: `am_o_b`. Events `issue(o), issue(p), request(am_o_v), request(am_v_b)`.
- **Derived.** `control, lawful way taken: ... request(am_o_b) -> ['emit(am_o_b),unit'] | unit first amends officer->vote: ... request(am_o_v) -> ['emit(am_o_v),unit']; request(am_v_b) -> ['emit(am_v_b),unit']`
- **Scenario F2.** The same with the duty from the group, below the ministry.
- **Derived.** `control, unit does nothing: ... deadline(o) -> ['missed(o),unit'] | unit first amends officer->vote: ... request(am_o_v) -> ['emit(am_o_v),unit']; deadline(o) -> []`
- **Scenario F3.** A regulator duty met by a purchase `b1` (costed, lawful) or a supply `sup` from vendor x (forbidden by the ministry).
- **Derived.** `no grant asked: ... request(sup) -> ['emit(sup),unit'] | grant outstanding: ... grant(b1); request(sup) -> ['refuse(sup),unit']; request(b1) -> ['emit(b1),unit']`
- **Scenario F4.** The same with a group duty and a deadline.
- **Derived.** `no grant asked: ... deadline(o) -> [] | grant outstanding at the deadline: ... grant(b1); deadline(o) -> ['missed(o),unit']`
- **Correct outcome.**
  - F1: the forbidden amendment by vote should be refused. The duty had a lawful way and the unit removed it by an act that met no duty.
  - F2: the duty should be recorded as missed. A unit that sits still is recorded; one that destroys its only lawful way is not.
  - F3 and F4: a purchase under the cap is a way whether or not the unit has yet asked for rights. In the turn-taking loop an effect is funded only between its grant and its request, so a costed way is "infeasible" at almost every step.
- **Repair.**
  - Funds: make a costed way infeasible only after the ledger has refused it (an explicit event), not while no grant is outstanding.
  - Amendments: a way that was feasible when the duty came into force and that the unit's own emission made infeasible must still count, or that emission must be a miss of the duty.
  - Neither is a one-line change. Both stay inside the unit institution.

### A3. MEDIUM. K4 rule 5: a duty performed while overridden is not discharged, and is then missed

- **Scenario D1.** Supra duty `hi` and group duty `lo`, both to disclose to r1. The ministry forbids disclosure. Events `issue(hi), issue(lo), issue(p), request(rep_r1), withdraw(p), deadline(lo)`.
- **Derived.** `with the prohibition: ... request(rep_r1) -> ['emit(rep_r1),unit']; withdraw(p); deadline(lo) -> ['missed(lo),unit'] | control without it: ... request(rep_r1) -> ['emit(rep_r1),unit']; deadline(lo) -> []`
- **Correct outcome.** The unit disclosed to r1, which is what `lo` requires. No miss.
- **Cause.** `emit(E) initiates done(O) if liveobl(O), ...`. An overridden duty is not live.
- **Repair.** `emit(E) initiates done(O) if obliges(O), matches(E, O);`. On a copy the 35 scenarios pass and D1 gives no miss.
- **Why MEDIUM.** The document lists "several obligations at once" as untested. That is a statement about coverage; the outcome is still wrong under rule 5.

### A4. MEDIUM. K4 rule 5: a duty overridden at its deadline can never be late afterwards

- **Scenario P1.** Regulator duty, state prohibition. Events `issue(o), issue(p), deadline(o), withdraw(p), deadline(o), deadline(o), request(rep_r1)`.
- **Derived.** `issue(o); issue(p); deadline(o) -> []; withdraw(p); deadline(o) -> []; deadline(o) -> []; request(rep_r1) -> ['emit(rep_r1),unit']`
- **Scenario P2.** The same reached through F2: the unit's own amendment causes the override.
- **Derived.** `... request(am_o_v) -> ['emit(am_o_v),unit']; deadline(o) -> []; withdraw(p); deadline(o) -> []; deadline(o) -> []`
- **Correct outcome.** Once the override ends the duty is owed, and some deadline must be able to record a miss. As encoded it is owed for ever with no deadline.
- **Cause.** `due(O)` sets `pastdue` whether or not a miss was recorded, and `not pastdue(O)` then blocks every later miss.
- **Repair.** `deadline(O) generates due(O) if obliges(O), not overridden(O);`, with the clock sending the deadline again when an override ends. On a copy the 35 scenarios pass and P1 records the miss at the later deadline.
- **Not the listed limit.** "An act window before a deadline" covers an override that ends just before the deadline. This is one that ends after it.

### A5. MEDIUM. K8: the layers also disagree the other way, and the test cannot see it

- **Scenario E1.** u0 late by 1/2 itself, its link fast by 1/2.
- **Derived.** `InstAL missed []; verdict findings {'u0': '1/2'}, shares {}`
- **Scenario E2.** The 25 chains of `test_end_to_end.py`, same seed, with `link` drawn from {−1/2, 0, 0, 1/2}.
- **Derived.** `missed != findings + link-only payers in 3 of 25 chains`
- **What is false.** K8 lists one mismatch: a violation in the norm layer and no finding in the verdict. The reverse also holds. Round 1 reported both (K8-3); draft 2 kept one. "The units InstAL reports as having missed are exactly the units the verdict names" passes because the test never draws a negative link residual.
- **Repair.** Let the generator draw fast links and state both mismatches. To remove them, give the norm layer two duties per hand-over (respond within the bound; be received within bound plus allowance), or drop the own-or-link split from the verdict's findings.

### A6. MEDIUM. K1 against K8: K8 needs the order across units that K1 says is not needed

- **Scenario O1.** u0 owes a hand-over to u1. Two sequences: receipt then deadline, deadline then receipt.
- **Derived.** `receipt first: ... request(h_u0) -> ['emit(h_u0),u0']; deadline(o_u0) -> [] | deadline first: ... deadline(o_u0) -> ['missed(o_u0),u0']; request(h_u0) -> ['emit(h_u0),u0']`
- **Why both agree on every unit's own order.** By K8 and K9.1 the receipt is certified by u1's witnesses in u1's log. The deadline is a time in u0's log. Each log holds one of the two events.
- **What is false.** K1: "No order across units is needed, and none can be certified when clocks differ." K8 builds its sequence by comparing a time from u1's witnesses with a time from u0's. If the receipt instead enters u0's order when the acknowledgement arrives, which is K1's rule for an authority's act, a hand-over received in time with a slow acknowledgement is recorded as missed.
- **Repair.** Say in K1 that a duty discharged by another unit's receipt needs the two logs' times to be comparable, and take from F2 a bound on clock difference that K8 adds to the allowance.

### A7. MEDIUM. K0: the order of the unit list and the default edges decide who is bound

- **Scenario T1.** The ministry forbids vendor x. `unit` created `v`. The same data with the units listed as `("unit","v")` and as `("v","unit")`.
- **Derived.** `units listed as ('unit', 'v'): ... request(ox_u) -> ['refuse(ox_u),unit'] | units listed as ('v', 'unit'): ... request(ox_u) -> ['emit(ox_u),unit']`
- **Scenario T2.** A prohibition with `target="v"` and no `edges` given.
- **Derived.** `issue(p); ownissue(vp,officer); request(ox_v) -> ['emit(ox_v),v']`
- **Correct outcome.** T1: the creator is bound in both. T2: an error, or the norm binds v. As it stands the issue succeeds in the ministry and binds nobody.
- **Cause.** A norm with no target goes to `sink_units[0]`; the default edge set is `{(authority, sink_units[0])}`.
- **Repair.** With more than one unit, require an explicit target and explicit edges, and reject a norm whose target has no edge unless the scenario marks it as a no-edge test.

### A8. MEDIUM. K0, K6: a creator's norm for the unit it created binds the creator instead

- **Scenario T3.** A prohibition with origin `unit` and `target="v"`. Events `issue(a), ownissue(p,officer), request(ox_v), request(ox_u)`.
- **Derived.** `issue(a); ownissue(p,officer); request(ox_v) -> ['emit(ox_v),v']; request(ox_u) -> ['refuse(ox_u),unit']`
- **Correct outcome.** v is bound and `unit` is not, or the input is rejected.
- **Cause.** The adapter removes units from the set of authorities, writes no bridge from a creator, and ignores `target` for a unit's own norms.
- **Repair.** Write a bridge for a creator when a norm of its origin names a created unit as target, or assert that an own norm has no target.

### A9. LOW. K0: the status sentence is false for two of the five facts

- **Statement.** "Each of these has at least one scenario that fails if the adapter gets it wrong."
- **Scenario M1.** The adapter writes `owns` for every unit.
- **Derived.** `test_instal.py + test_ledger_link.py + test_end_to_end.py: passes`
- **Scenario M2.** The adapter writes norms of equal priority as strictly higher.
- **Derived.** `test_instal.py + test_ledger_link.py + test_end_to_end.py: passes`
- **Controls.** M4 ("same or higher" written as strictly higher) and M5 (inheritance one level deep) fail the tests, as they should.
- **Repair.** Add a tie scenario under rule 4, and a scenario where only `owns` separates two units. Or weaken the sentence.

### A10. LOW. K9: "The test enumerates the allocations" (round 1 L4, not repaired)

- **Scenario X1.**
- **Derived.** `the loop in test_evidence.py calls no function of evidence.py, and the sentence is still in the document: True`
- **Repair.** Delete the sentence and count 10 checks. The proof stands without it.

### A11. LOW. The record of round 1 is understated

- **Scenario X2.**
- **Derived.** `rows in 'What was wrong in draft 1': 11; distinct round-1 findings re-run in part 1: 21; of those still reproducible and unrepaired: ['Fable M2', 'Fable M4', 'Fable M5', 'Fable L3', 'Fable L4', 'Fable L6']`
- **What is false.**
  - "They found eleven wrong outcomes or false statements ... This draft corrects them." Stop condition 5 of the same document counts 21 and 8.
  - Stop condition 2: "every defect so far was repaired in the institution or the adapter." Three were not repaired (L3, L4, L6) and three were turned into listed limits (M2, M4, M5).
  - Three rows credit one tester with what both reported: the grant in two institutions (also Fable M6), the charter input (also Fable L2), the refused late hand-over (also Astra D8).
  - "Each of the testers' 60 further runs": the 60 were one tester's.
- **Repair.** List all 21 with a status each.

### A12. LOW. K4, K0: a duty whose only declared way belongs to another unit (round 1 L6, not repaired)

- **Derived.** `issue(o); request(ox_u) -> ['refuse(ox_u),unit']; deadline(o) -> ['missed(o),v']`
- **Correct outcome.** The adapter should reject the scenario. v is recorded as missing a duty it has no effect to meet.
- **Repair.** In the "no declared way" assertion, look only at effects owned by the unit the duty binds.

### A13. LOW. K7: a unit's own norm can never be repealed (round 1 L3, not repaired)

- **Derived.** `ownissue(self,officer); request(am_o_v) -> ['refuse(am_o_v),unit']; ownissue(self,officer); request(am_o_v) -> ['refuse(am_o_v),unit']`
- **Repair.** An `ownwithdraw(N, P)` event, by the procedure in force.

### A14. LOW. K11: a purchase that costs nothing is refused for ever

- **Scenario Z1.** The loop of `test_ledger_link.py`, cap 100, purchases b0 (cost 0) and b1 (60).
- **Derived.** `emitted ['b1']`
- **Cause.** The adapter writes `needsfunds` for any effect with a `cost` key; `Ledger.issue` refuses an amount of 0.
- **Repair.** Write `needsfunds` only when the cost is positive.

### A15. LOW. K2: the empty history has no trace through the adapter (round 1 L1, not repaired)

- **Derived.** `RuntimeError: <string>:2:2: error: syntax error, unexpected <EOF>`
- **Repair.** Return the initial state when there are no events. This is an adapter fault and does not meet stop condition 1.

## List B: design objections and claims stronger than their evidence

| Id | Clause | Objection | Derived |
|---|---|---|---|
| M6, M7 | K4 rule 5 | "Records the miss once" and the clearing of `pastdue` on re-issue are untested: removing either line passes every check | `passes` |
| M8 | K4 rule 2 | "A costed effect with no rights reserved is not a way" is untested: removing the line passes every check | `passes` |
| M9 | K5 | "A higher duty": a watched prohibition that yields to a duty of any rank passes every check | `passes` |
| M10 | K11 | "A grant that funds only the unit owning the effect": a grant that funds every unit passes every check, because the owner test on `request` does the work | `passes` |
| M3 | K0 | K0's list is incomplete. The adapter also decides which norms are a unit's own, which are watched (K5's whole test), which effects are costed or amendments, each norm's target, and the first procedure. Letting every unit issue every unit's own norms passes every check | `passes` |
| P3 | K4 rule 5 | Re-issuing an undischarged, missed duty records a second miss for one undone act | `misses recorded: 2` |
| P4 | K4 rules 4, 5 | After the deadline the exception and the permission stand with no limit of time. An unperformed duty is a standing licence for one emission against every lower prohibition | `deadline(o) -> ['missed(o),unit']; ... request(rep_r1) -> ['emit(rep_r1),unit']` |
| G4 | K4, K0 | Norms of one origin tie and the tie goes to the prohibition. An authority cannot oblige an exception to its own ban; the duty is silently overridden | `request(rep_r1) -> ['refuse(rep_r1),unit']; deadline(o) -> []` |
| G1 | K0, K5 | Matching is fail-open: an effect that omits the attribute a prohibition reads is not matched. K5 asks who certifies a value, not who certifies presence | `request(ord) -> ['emit(ord),unit']` |
| G2 | K6 | A creator's own prohibition does not follow into the unit it creates, so a unit gets round its own rule | `request(ox_u) -> ['refuse(ox_u),unit']; request(ox_v) -> ['emit(ox_v),v']` |
| G3 | K0 | The priority table is not checked against the edges. A unit ranked above its authority excepts itself with its own duty | `request(rep_r1) -> ['emit(rep_r1),unit']` |
| G5 | K0 | With one unit, an effect owned by someone else is accepted and its request is neither emitted nor refused | `request(ord) -> []` |
| Q1 | K1, K11 | A cap shared by two units needs an order across units at the ledger. K1 holds to the letter because grants sit in each unit's log | `hears unit first: ... emit(buy_u),unit ... refuse(buy_v),v | hears v first: ... refuse(buy_u),unit ... emit(buy_v),v` |
| N1 | K9.5 | With two lumps, resolving one asks A, B and C for 1 each against a final 7/12, with nothing in suspense. "Collected on account" is safe only if everything is refundable until the last lump closes. The clause does not say who holds the money | `present ACF: pays {'A': '1', 'B': '1', 'C': '1'}, suspense 0, provisional True` |
| N2 | K9.5 | More evidence does not move a provisional amount monotonically toward the final one | `3162 unit-cases: ... moved AWAY from the final share in 29; ... exceeded the final share in 330` |
| N3 | K9 | "If the evidence never arrives ... the loss stays where it fell" is not true of a lump that cancels: a reporting unit carries the hidden deviator's share on account, and no rule ends "provisional" | `final {'B': '1/2', 'C': '1/2'}; A and B missing: pays {'C': '1'}` |
| N4 | K9.5 | "Any lump open" is sufficient for "provisional", not necessary: a verdict whose miss is zero is still provisional. Evidence that contradicts an acknowledgement cannot be expressed in `evidence.py` | `pays {}, miss 0, ... provisional True` |

Two further objections without a scenario:

- **K8 and stop condition 3.** `test_end_to_end.py` still places each deadline by calling `verdict.receipt`. The norm layer's deadlines are therefore computed by the verdict layer's code.
- **K2.** `check_stratified.py` still misses an indented cyclic rule (part 1, L1). The document now disclaims it correctly, so this is no longer a false statement.

## List C: limits the document already lists

| Id | Clause | Limit | Derived |
|---|---|---|---|
| Q2 | K11 | A late grant, or a grant whose request never comes, leaves the effect funded and the reservation counted | `funded(b1) at the end: True` |
| Q3 | K11 | A replayed grant buys a second emission | `grant(b1); request(b1) -> ['emit(b1),unit']` twice |
| Q4 | K11 | A grant dropped by a refusal for want of a permission that arrives a step later | `request(b1) -> ['refuse(b1),unit']; issue(a); request(b1) -> ['refuse(b1),unit']` |
| Q5 | K4, K11 | A duty to spend with no grant is missed | `deadline(o) -> ['missed(o),unit']` |
| R1-M4 | K4 | Two duties that one emission would meet earn two exceptions (round 1 M4; now "several obligations at once") | `request(e2) -> ['emit(e2),unit']; request(e1) -> ['emit(e1),unit']` |
| Fable M2 | K9 | The receiver alone can open a lump | `pays {}, suspense 2, undetermined [['A', 'B']]` |
| Fable M5 | K11 | A grant carries no amount | `Ledger.use(60) accepted: False` |
| Fable M7 | K1 | Per-unit orders that differ are not tested; the encoding uses one sequence | merge A refuses, merge B emits |

## The round-1 defects

| Round-1 finding | Status | Evidence from this run |
|---|---|---|
| Fable H1, Astra D8: late hand-over refused | Fixed | `missed ['u0'], emitted ['u0', 'u1']` |
| Fable H2: agreement test never used the verdict; layers name different units | Partly fixed | Test now calls `verdict()`. One mismatch is stated; the other is not (A5) |
| Fable H3: a deadline event lifts any norm | Fixed | `deadline(p) -> []; request(ord_x) -> ['refuse(ord_x),unit']` |
| Fable H4, Astra D2: depth-two unit unbound | Fixed | `request(ord_x_w) -> ['refuse(ord_x_w),w']` |
| Fable H5: an infeasible way counted as a way | Fixed for those scenarios; the fix opens A2 | higher duty emits `am_o_v`; lower duty not missed |
| Fable H6: watched prohibition recorded a breach | Fixed for that scenario; the fix is too wide (A1) | `request(pay_h) -> ['emit(pay_h),unit']` |
| Fable H7, Astra D5: cancelling lump not recorded | Fixed | `undetermined [['A', 'B']], provisional True` |
| Fable H8, Astra D4: rights never released on refusal | Fixed inside the turn-taking loop | `emitted ['b2']; ledger counts 60 of 100` |
| Fable M1, Astra D6: comparison with the time bar | Partly fixed | Text corrected. `test_evidence.py` still uses `old_bar()` as the comparator |
| Fable M2: receiver opens a lump | Not fixed; now a listed limit | `pays {}, suspense 2` |
| Fable M3: exactness for shares only | Fixed (text) | Both qualifications are in the clause |
| Fable M4: two duties, two exceptions | Not fixed; now listed as open | `emit(e2)` then `emit(e1)` |
| Fable M5: grant has no amount or end | Not fixed; now a listed limit | `Ledger.use(60) accepted: False` |
| Fable M6, Astra D3: effect with no owner in every unit | Fixed | `AssertionError: effect ord_y needs an owner` |
| Fable M7: K1 per-unit order | Partly fixed | Restated and disclosed. Encoding unchanged. New contradiction with K8 (A6) |
| Fable L1, Astra D7: checker; empty history | Partly fixed | Text corrected. Checker still says `no cycle` for an indented cycle. Empty history still raises (A15) |
| Fable L2, Astra D1: charter input ignored | Fixed | `issue(allowp) -> ['ultravires(allowp),ministry']`. An issue event still carries no issuer |
| Fable L3: own norm cannot be repealed | Not fixed | A13 |
| Fable L4: the limit's test is empty | Not fixed | A10 |
| Fable L5: counts | Fixed | 15, 19 and 35 match the files. The new table is wrong (A11) |
| Fable L6: duty whose way belongs to another unit | Not fixed | A12 |

The round-1 script itself now stops at K6-2 with `AssertionError: effect ord_x_w needs an owner`. That is the owner check doing its job on a scenario that omitted an owner.

## The claims table

- **Counts that match.** K2 35; K3 3; K4 19, nine of them from earlier defects; K5 3; K6 3; K7 6; K8 25; K10 15 and 300; K11 6, 2000 and 5.
- **K0 "one scenario per fact it writes".** False for ownership and for ties in rank (A9), and the list of facts is incomplete (B, M3).
- **K4 "19".** Rules 2 and 5 each have a part that no scenario depends on (B: M6, M7, M8).
- **K5 "3".** None separates a higher duty from any duty (B, M9), and none has a free way (A1).
- **K8 "25 chains, wiring only".** Fair as worded. The generator hides one mismatch (A5).
- **K9 "11 checks".** Ten; one calls no model code (A10). The two proofs are sound as now qualified. I found no counterexample to exactness in the model.
- **K11 "5 connected".** The scenario for "funds only the unit owning the effect" does not depend on the grant rule (B, M10).

## Stop conditions

1. **A scenario has no trace or several.** Not met. 87 runs, one answer set each, including institutional events observed directly, ill-typed and unknown events. The empty history fails in the adapter, not in InstAL (A15).
2. **A repair that needs a change to InstAL's semantics.** Not met. Every repair above is in `unit.ial`, the adapter, a test or the text. The sentence "every defect so far was repaired" is false (A11).
3. **K8 cannot be made without restating the norm rules inside the verdict layer.** Still undecided, and the present test cannot decide it. The layers disagree in both directions (A5), the deadlines are computed by the verdict's own function, and K8 needs a cross-log order that K1 denies (A6). The repair I can see goes the other way: two duties per hand-over in the norm layer, which restates the verdict's own-or-link split there.
4. **K11 cannot be made by events alone.** Not met, in the simple form only, as the document says. Nothing new was learned. A2 shows that funding now feeds the priority rule, so "the ledger sits outside" no longer holds for K4.
5. **The defects are not converging.** By the restated condition, my count fell: 15 confirmed against 21 in round 1 on the same scope. Ten of the 15 are new and five are carried over. Three of the new ones (A1, A2, A4) are in lines written to repair round 1. The condition also needs the other tester's count.

CONFIRMED WRONG OUTCOMES (list A): 15
CRITICAL: 0  HIGH: 2  MEDIUM: 6  LOW: 7

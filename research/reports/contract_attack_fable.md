# Attack on the composition contract (Fable)

Target: [Composition contract.md](/root/mic/reports/Composition contract.md), draft 1, and the models in /root/mic/model.

Every scenario below is in [attack_contract_fable.py](/root/mic/model/attack_contract_fable.py). Run it with:

```
python3 /root/mic/model/attack_contract_fable.py
```

It prints 46 scenarios: 30 marked DEFECT and 16 "as specified". The "derived" lines quoted here are copied from that run. Scenario identifiers such as K8-1 are the labels the script prints. No existing file was changed; the documented checks (`test_instal.py`, `check_stratified.py`, `test_end_to_end.py`, `test_ledger_link.py`, `test_evidence.py`, `test_verdict.py`, `test_ledger.py`) all still pass.

## Summary

- **No CRITICAL defect.** K2 held on all 60 InstAL runs (exactly one answer set each). The two proofs in K9 are sound within the model's assumptions.
- **Eight HIGH defects.** The most damaging are in K8: the end-to-end test agrees by arithmetic and never consults the verdict, and the norm layer refuses a late hand-over that the verdict layer counts as received.
- **Seven MEDIUM and six LOW.**
- **Stop condition 5 is met on the raw count** (21 defects against eight in the previous round). On the norm layer alone the count is eight to ten against eight, so it is at best not falling. The other four stop conditions are not met, but conditions 3 and 4 have not passed the tests the document says they passed.

Three classes are kept apart for each finding:

- **Encoding**: `unit.ial`, the harness or a test is wrong; the clause as written is fine.
- **Clause**: the clause as stated is wrong or incomplete.
- **Listed limit**: the document already says so. These are not counted.

## HIGH

### H1. K8: a late hand-over is refused by the norm layer and counted as received by the verdict layer (encoding)

- **Scenario** K8-1 (and K4-9 for the cause). Two units, u0 late by one day, built exactly as `test_end_to_end.py` builds its chains.
- **Derived.** `timeline [('2', 'deadline(o_u0)'), ('3', 'request(h_u0)'), ('5', 'request(h_u1)'), ('5', 'deadline(o_u1)')]; InstAL: missed ['u0'], emitted ['u1'], refused ['u0']; verdict: u1 becomes ready at u0's receipt, shares {'u0': '1'}`
- **How often.** In 20 of the 25 chains of the existing test, at least one late hand-over is refused (K8-2).
- **Correct outcome.** A late delivery is late, not forbidden. The norm layer should emit `h_u0` and record `missed(o_u0)`. As it stands, u0's delivery never takes effect in the norm layer, yet u1 delivers a result built on it and the verdict computes u1's readiness from it.
- **Cause.** `due(O) terminates inforce(O)` removes the obligation at its deadline, and the obligation was the only permission for the hand-over.
- **Repair.** Keep the obligation in force after `missed`, or record the miss in a separate fluent and leave the permission. Add "every hand-over in the timeline is emitted" to the end-to-end assertions.

### H2. K8: the agreement is guaranteed by arithmetic, and it fails against what the verdict actually returns (encoding of the test; the claim in the clause)

- **Scenario** K8-2, K8-3.
- **Derived.** `missed == {own+link > 0} read off the INPUT specs, no verdict call: 25/25; missed != verdict()['findings']: 6/25; missed != units with a positive share: 1/25`
- **Why it is arithmetic.** The test sets the receipt at `ready + bound + allowance + residual` and the deadline at `ready + bound + allowance`, from the same `ready`. "Request before deadline" is then the same statement as "residual not positive". The set `late` is read from `Event.residual`, an input field. `verdict()` is called only for `set(v["shares"]) <= late`, which holds by the definition of a player.
- **What the test does establish.** That the InstAL encoding turns "request before deadline" into "not missed". That is a test of K4, not of K8.
- **Where the layers disagree** (K8-3): `u0 slow, link fast: InstAL missed [], verdict findings {'u0': '1'}; u0 on time, link slow: InstAL missed ['u0'], verdict findings {}, pays_for_link_without_finding ['u0']`. The norm layer records a violation by a unit against which the verdict layer says there is no finding, and the reverse.
- **Consequence for the document.** "Conditions 3 and 4 have each passed a first, simple test" is not supported for condition 3. The deadline placement in the test also repeats the verdict layer's formula (`ready + bound + allowance` from `verdict.receipt`), which is the restating that stop condition 3 is about.
- **Repair.** Derive the deadline events from contract data that the verdict code does not also compute; feed InstAL's violation events into the verdict as K8 says ("Needs from F1"); compare `missed` with a named output of `verdict()`; decide whether a duty is about the unit's own response or about receipt, and make both layers use the same one.

### H3. K3/K4: a clock event lifts any norm; no authority's power is involved (encoding)

- **Scenario** K3-1, K3-2.
- **Derived.** `request -> ['refuse(ord_x),unit']; deadline(p) ; request -> ['emit(ord_x),unit']` and `request -> ['emit(ord_x),unit']; deadline(allowp); request -> ['refuse(ord_x),unit']`
- **Correct outcome.** Only the issuing authority, across its bridge, should end a prohibition or permission.
- **Cause.** `deadline(O) generates due(O)` is unconditional, `due(O) terminates inforce(O)` applies to every norm, and the unit holds `pow(deadline(Norm))` for all norms. Whoever supplies deadline events (the evidence layer, in K8) can repeal prohibitions.
- **Repair.** `deadline(O) generates due(O) if isoblige(O), inforce(O);` and add a regression scenario.

### H4. K6: a unit created by a created unit escapes (encoding, in the harness)

- **Scenario** K6-1. The ministry forbids vendor x to `unit`; `unit` creates `v`; `v` creates `w`; each has its own permission to order.
- **Derived.** `v: ['refuse(ord_x_v),v']   w: ['emit(ord_x_w),w']`
- **Correct outcome.** K6 gives a created unit a bridge from each of its creator's authorities. The ministry is one of v's authorities, so w must be bound.
- **Cause.** `harness.py` line 50 grants inherited powers only if `u in created[target]`, one level deep. The inheritance is done by the harness writing power facts; InstAL only applies them.
- **Repair.** Take the transitive closure of `created` when writing bridge powers, and add a depth-two scenario. This is the delegation chain the contract is about, and both existing K6 tests have depth one.

### H5. K4/K7: a way that cannot be taken still counts as a way (encoding, and rule 2 and 3 as stated)

- **Scenario** K4-1 and K4-2. Procedure in force: officer. Ways to meet a duty to amend: `am_o_v` (uses officer) and `am_v_b` (uses vote, not in force). A prohibition covers amendments that use officer.
- **Derived, duty above the prohibition (K4-1).** `request(am_o_v) -> ['refuse(am_o_v),unit']; request(am_v_b) -> ['refuse(am_v_b),unit']; deadline -> ['missed(o),unit']`
- **Derived, duty below the prohibition (K4-2).** The same three outcomes.
- **Correct outcome.** In K4-1 the higher duty should earn an exception for `am_o_v` (rule 3). In K4-2 the duty should be overridden and not reported as missed (rule 2). In both the unit is recorded as violating a duty it had no lawful way to meet.
- **Cause.** `hasway` and `cheaperway` test only prohibitions. They ignore `procfail` and `unfunded`.
- **Repair.** Define a way as feasible (`not procfail(E), not unfunded(E)`) and use that in `hasway`, `blockedway` and `cheaperway`. The check for a cycle must be re-run, since `unfunded` and `procfail` then feed `liveobl`.
- **Listed limit, same cause.** A duty to spend with no grant is also missed (K4-3); K11 lists it as untested.

### H6. K5 with K4: the same act is lawful under an enforced prohibition and a violation under a watched one (clause and encoding)

- **Scenario** K5-1. The regulator obliges a payment; the lower-ranked ministry forbids it.
- **Derived.** `watched: ['breach(pay_h,p),unit', 'emit(pay_h),unit']   enforced: ['emit(pay_h),unit']`
- **Correct outcome.** Rule 3 of K4 makes the prohibition give way to the higher duty. A watched prohibition is the weaker form and should not record a breach where the enforced form would have yielded.
- **Cause.** `emit(E) generates breach(E, N) if watches(N), matches(E, N)` has no exception test. K5 as written says the same ("a matching emission goes through and is recorded as a violation"), so the clause needs the qualification too.
- **Repair.** Compute `excepted` for watched prohibitions as well and record a breach only when not excepted.
- **Related, as specified (K5-2).** A self-declared attribute in an obligation's pattern lets the unit choose which emission earns the exception from an enforced prohibition. K5 is silent on obligations and permissions that read self-declared attributes.

### H7. K9: suspense does not hold the lump's share, some lumps are not recorded at all, and determined units pay wrong amounts with finality (clause 4 and encoding)

- **Scenario** K9-1. Residuals A −2, B +1, C +2; A and B missing.
  - **Derived.** `full evidence {'B': '1/2', 'C': '1/2'}; A and B missing: pays {'C': '1'}, suspense 0, undetermined []`
  - B deviated and pays nothing, permanently: no lump is listed, so nothing is reopened when the evidence arrives. C, which reported, pays B's half day.
- **Scenario** K9-2, the document's own counterexample.
  - **Derived.** `full evidence {'A': '1/2', 'B': '1/2'}; B and C missing: pays {'A': '1'}, suspense 0, undetermined []`
  - The document reports that A pays the whole day. It does not report that B's half day is neither charged nor held, and that clause 4 ("held in suspense ... reopened for that lump") does nothing here.
- **Scenario** K9-3. Residuals 1, 1, 1, slack 1; A and B missing.
  - **Derived.** `full evidence {'A': '2/3', 'B': '2/3', 'C': '2/3'}; A and B missing: pays {'C': '1/2'}, suspense 3/2`
  - A reporting unit pays less because its neighbours did not report. `test_evidence.py` names this property (P3) in its header and never tests it. The suspense, 3/2, is not what A and B owe, 4/3.
- **How often** (K9-4). `19000 histories: a determined unit pays less in 502 unit-cases, more in 1264; a deviator sits in an unrecorded lump in 576 histories`
- **Why.** The lump is treated as one Shapley player. Its share and everyone else's then differ from the full-evidence values whenever the lump mixes signs or the slack bites. A lump with a non-positive sum is not a player, so `with_missing` records no suspense and no `undetermined` entry for it.
- **What is wrong in the clause.** "Charged to nobody" is false when the lump's sum is not positive: the share moves to reporting units. "Reopened for that lump" is not enough: the shares of units outside the lump must be reopened too, or the shares no longer sum to the miss.
- **Repair.** List every lump of more than one unit as undetermined, whatever its sum. Mark the whole verdict provisional while any lump is open, and recompute all shares when evidence arrives. State in the clause that only non-deviators are protected by "never charge on a reporting failure".

### H8. K11: rights reserved for a purchase that the norm layer then refuses are never released (encoding of the connection)

- **Scenario** K11-1. Cap 100. b1 (60, vendor x, forbidden) then b2 (60, vendor y, lawful), using the loop of `test_ledger_link.play`.
- **Derived.** `b1 refused at [4], b2 refused at [5], emitted: none; ledger counts 60 of 100, really used 0; funded(b1) still holds: True`
- **Correct outcome.** b2 is lawful and within the cap and should go through.
- **Cause.** The ledger reserves before the norm check, and no event tells it of a refusal. The grant also stays in the institution, so the forbidden purchase is pre-funded for whenever the prohibition lapses.
- **Repair.** On `refuse(E)` with `funded(E)`, terminate `funded(E)` and emit an event the ledger answers with expire and settle. Or check norms first and ask the ledger only for effects that are otherwise allowed. Either can be done with events, so stop condition 4 is not met.

## MEDIUM

### M1. K9: the comparison with the time bar credits the suspense rule with what acknowledgements do (clause, table)

- **Scenario** K9-9.
- **Derived.** `row 1: old_bar {'U2': '3/2'} | bar WITH acknowledgements {'U3': '1', 'U2': '1/2'} | suspense {'U3': '1', 'U2': '1/2'}`; row 2 likewise identical for the bar with acknowledgements and for suspense.
- On both rows of the table the bar and the suspense rule give the same answer once both have double recording. The table compares a rule without acknowledgements against a rule with them.
- `old_bar()` in `evidence.py` is also harsher than the rule it stands for. It merges a missing unit with its successor although the successor's own log fixes that boundary, and it treats the sender's whole residual as link. The rule in `verdict.py` (`absorb_missing`) gives, for row 2 with U3 slow itself: `{'U1': '1/2', 'U3': '1'}`, which is the truth.
- **Repair.** Compare three columns: old bar, bar with acknowledgements, suspense. The rules differ only on lumps, and there the honest summary is "the bar charges the missing units, suspense charges nobody or the wrong unit (H7)".

### M2. K9: the receiver alone can open a lump (clause)

- **Scenario** K9-5.
- **Derived.** `B reports: {'B': '2'}; B missing: {'B': '2'}; B missing and B's acknowledgement to A lost: pays {}, suspense 2, undetermined [['A', 'B']]`
- The summary and the acknowledgement both come from the receiver's side. A late receiver that withholds both pays nothing, indefinitely, and puts its compliant sender A in an undetermined lump. The document states the loss ("one missing and an acknowledgement lost") but not that one party controls both events, which makes not reporting the better strategy for that party.
- **Repair.** Either take from F2 that an acknowledgement by honest witnesses always arrives, and say so in K9, so that this lump cannot occur; or treat "summary missing and own acknowledgement missing" as attributable to the receiver.

### M3. K9: exactness covers shares, not findings, and rests on two unstated boundary assumptions (clause, proof)

- **Scenario** K9-8.
- **Derived.** `U3 slow itself: shares {'U1': '1/2', 'U3': '1'}, findings {'U1': '1/2', 'U3': '1'}; U3's link slow: shares {'U1': '1/2', 'U3': '1'}, findings {'U1': '1/2'}`
- With U3's summary missing, both histories leave the same evidence, so the findings that K10's verdict returns (`findings`, `pays_for_link_without_finding`) cannot be recovered. "The verdict equals the verdict on full evidence" is true of shares only. `evidence.py` has one residual per unit and cannot express the difference.
- The proof says "every boundary time is known". It needs the start of the first unit and the final receipt to be known as well (K9-7). `evidence.py` says the claimant knows the final receipt; the clause and the proof do not, and nothing says who certifies the start.
- "Every acknowledgement arrived" is sufficient and not necessary (K9-6); the clause claims only sufficiency, so this is not a defect.
- **Repair.** State exactness for shares; add the two boundary assumptions; say that the split between a unit and its link is lost when the sender is missing.

### M4. K4: exceptions are tested one duty at a time; the rule is not one of the published procedures (clause)

- **Scenario** K4-4. o1 (state) requires e1; o2 (regulator) requires e1 or e2; a ministry prohibition covers both.
- **Derived.** `request(e2) -> ['emit(e2),unit']; request(e1) -> ['emit(e1),unit']; request(e1) -> ['refuse(e1),unit']`
- Two forbidden disclosures are let through where e1 alone meets both duties. This is the pattern of the "n+1 disclosures" attack that K6 was written to close, reached with duties instead of created units.
- **Which published procedure is it?** Liao et al. give three: Greedy, Reduction and Optimization (the last after Brewka and Eiter, and Hansen). For norms without conditions they come to the same thing: go down the priority order and keep each norm that is consistent with those kept. The encoding is not that procedure, on either reading of a prohibition:
  - Prohibition as one norm: in the first scenario of `test_instal.py` Optimization drops the whole ministry prohibition, so `rep_x` would be allowed. The encoding refuses it.
  - Prohibition as one norm per effect: in K4-4 Optimization keeps the ban on e2. The encoding lets e2 through.
  - Ties (K4-5): Optimization gives two outcomes for a duty and a prohibition of equal rank. The encoding always prefers the prohibition.
  - The one-time exception that lapses on discharge has no counterpart in those static procedures.
- For a single duty with prohibitions read per effect, the outcomes I ran agree with Optimization ("the lowest prohibition yields"). I did not prove that. The literature comparison is from memory of the paper and was not re-checked online.
- **Repair.** Either say that the rule is a per-duty, per-effect variant of Optimization and state the two differences, or test "cheaper way" against all live duties jointly.

### M5. K11: a grant has no amount, no cap and no end; the test harness keeps them in step (encoding)

- **Scenario** K11-2. `grant, cap 100 -> 50, lease expired, request: emitted at [3]; Ledger.use accepted: False`. The institution emits a purchase that the ledger refuses to book. No event withdraws a grant.
- **Scenario** K11-3. `ledger reserves 1 for b1, which costs 60: emitted at [3]; Ledger.use(60) accepted: False`. The event `grant(E)` carries no amount. `test_ledger_link.py` keeps cost and reservation equal by reading both from one Python dictionary, and it calls `Ledger.use` itself after reading the trace. That part of "connected by events alone" is done by the harness.
- **Listed limit.** A double grant (K11-5) is listed.
- **Repair.** An event that ends a grant (expiry, cap change), and an amount in the grant checked against the effect's cost inside the institution or by the ledger on a `spent(E)` event.

### M6. K6/K11: observed events are global; units are kept apart only by an optional attribute the scenario writes (encoding)

- **Scenario** K6-3. `['emit(ord_y),unit', 'emit(ord_y),v']`: one request for an effect with no `of` attribute is emitted by every unit institution.
- **Scenario** K11-4. `cap 60, one grant of 60: emitted in unit at [3], in v at [3]`: one grant, two purchases, 120 spent under a cap of 60.
- The harness does not reject an effect without an owner when there is more than one unit. `request`, `grant`, `deadline` and `ownissue` are all seen by every unit; the separation is in the `matches` and `needsfunds` facts the harness writes.
- **Repair.** Require `of` when there are several units, or give the events a unit argument.

### M7. K1: the clause asks for an order per unit; InstAL needs one order over all units and authorities (clause)

- **Scenario** K1-1. `ministry log [issue(allowp), issue(p)], unit log [request(ord_x)]: merge A -> ['refuse(ord_x),unit'], merge B -> ['emit(ord_x),unit']`
- K1 takes from F2 "a total order on a unit's events". InstAL in the mode used admits one observed event per step for all institutions together, so the order between an authority's issue and a unit's request decides the outcome, and those events are in different logs. K8 mentions that the evidence layer builds "the one ordered sequence" from certified times, but K1 does not ask F2 for it and nothing says how ties across logs are broken.
- **Repair.** State in K1 that F2 must yield a single total order across the logs of a unit and of all its authorities (in a synchronous network: certified time plus a fixed tie-break), and that K2's "one outcome per history" is relative to that merged order.

## LOW

- **L1. K2, the checker** (K2-3, K2-2). `check_stratified.py` anchors rule heads at column 0. An indented cyclic rule, or one sharing a line with another rule, passes: `the same rule indented: checker '19 non-inertial fluents, 27 dependencies, no cycle', InstAL 'InstalParserTypeError: Failed occurs check ...'`. InstAL's own parser rejects all three variants, so the script is weaker than, and redundant with, the tool. It reports 19 non-inertial fluents; the document says eighteen. The empty history raises a parse error in the harness instead of giving one trace.
- **L2. K3, the harness interface** (K3-3). The docstring of `run` documents a `power` key and events as `(institution, event)` pairs. Neither is implemented: `power={'ministry': set()}` changes nothing. An issue event carries no issuer, so "one authority issues another's norm" cannot be written, and K3's charter test is a flag on the norm (`ultra_vires`). The no-edge test itself is genuine: the bridge lacks `ipow` and InstAL does the refusing (K3-4).
- **L3. K7, own norms cannot be repealed** (K7-1). `request -> ['refuse(am_o_v),unit']; second ownissue; request -> ['refuse(am_o_v),unit']`. One own prohibition on amendment freezes the charter for good. Related, as specified (K6-2): a creator's own prohibition does not follow into a created unit, which gives a unit a way round its own rule.
- **L4. K9, the test of the limit** (K9-10). The loop in `test_evidence.py` asserts that no number is both 0 and at least 2; it calls no code of the model. "The test enumerates the allocations" should be dropped; the proof in the text stands on its own. The header of that file lists P2 and P3 as properties "tested"; P3 is not tested and is false (H7), and P2 is given up by the clause itself.
- **L5. Counts and attributions.** `test_verdict.py` has 15 cases, not 14. The 27 norm scenarios do not split as 2 + 11 + 2 + 2 + 6 (that is 23). "InstAL ... its literature detects conflicts between them and does not resolve them": from memory, the same group published conflict resolution by revising the institutions with inductive logic programming (Li, Balke, De Vos, Padget, Satoh; Li's Bath thesis), so "does not resolve by priority" would be accurate. Not re-checked online.
- **L6. K4, the "declared way" guard** (K4-10). The harness accepts a duty whose only matching effect belongs to another unit: `in v: request -> ['refuse(ord_x),v']; deadline -> ['missed(o),v']`. The assertion should look at the effects of the unit the duty binds.

## Checked and found as specified

- K2: undeclared effects and undeclared event names still give one answer set (K2-1). All 60 InstAL runs gave exactly one (K2-4).
- K4: a tie goes to the prohibition (K4-5); withdrawal ends the exception and each re-issue buys a new one (K4-6); a duty issued after its deadline is owed with no deadline (K4-8). The last two are choices the document should state.
- K4-7: a duty overridden until the step before its deadline is then missed. The missing act window is listed under "What this does not cover".
- K6-4: a created unit gets no exception from its creator's duty.
- K8-4: a unit with a large negative residual hands over before its input exists and both layers accept it. A modelling oddity in `verdict.py`, not counted.

## Verdict on each clause

| Clause | Verdict |
|---|---|
| K1 order | Incomplete as stated: needs one order across logs, not one per unit (M7). |
| K2 one outcome | Holds on everything run. The argument is right for the reason given, and InstAL enforces the acyclicity itself; `check_stratified.py` adds nothing and can be fooled (L1). The gap the document names (no proof against the formal semantics) remains. |
| K3 edges as powers | The no-edge case is InstAL's power semantics. The clause is breached from the side: a clock event ends any norm (H3). Issuer identity is not modelled (L2). |
| K4 priority and exceptions | Wrong outcomes when a way is infeasible (H5); not jointly minimal and not one of the published procedures (M4); guard on declared ways is too weak (L6). |
| K5 prevented or detected | Inconsistent with K4: a watched prohibition records a breach where an enforced one yields (H6). |
| K6 created units | False at depth two in the encoding (H4); separation of units rests on scenario data (M6). |
| K7 amendment | The six tested behaviours hold. Duties met by amendment inherit H5. Own norms cannot be repealed (L3). |
| K8 norms to verdict | Not established. The test is true by arithmetic (H2) and the layers disagree on whether a late hand-over happened (H1) and on who has a finding (H2). |
| K9 missing evidence | Both proofs are sound under the model's assumptions. The suspense rule does not do what clause 4 says (H7); the table is unfair (M1); the receiver controls the lump (M2); exactness is for shares only (M3); the limit's test is empty (L4). |
| K10 verdict | Not attacked beyond running its tests, which pass. The count is 15, not 14 (L5). Its findings output is what K8 and K9 fail to connect to (H2, M3). |
| K11 ledger | The three connected histories pass. Rights leak on refusal (H8); grants have no amount or end (M5); one grant funds every unit (M6). |

## The claims table

- **K8 "25 random chains".** Listed as tested. The property tested is not the one claimed (H2).
- **K9 "8 checks".** One of the eight is empty (L4). "Exactness" and "the limit" are fairly listed as proved, with the qualifications of M3.
- **K2 "argued".** Fair.
- **K3 "2", K5 "2", K6 "2", K7 "6".** The scenarios exist and pass. K6's two are both depth one.
- **K4 "11".** The count does not match the file (L5).
- **K11 "3 histories connected".** They pass; what connects them is partly the harness (M5).

## Stop conditions

1. **A scenario with no trace or several.** Not met. 60 runs, one answer set each, including unrecognised events.
2. **A contradiction whose repair needs a change to InstAL's semantics.** Not met. Every repair above is in `unit.ial`, the templates or the harness. Two things come close and should be watched: global observed events (M6, M7) are InstAL's composite mode, repairable by adding a unit argument to events; creation during a run is listed as open.
3. **K8 needs the norm rules restated in the verdict layer.** Not shown either way. The document's statement that it "passed a first, simple test" is not supported (H2), and the test's deadline placement already repeats the verdict layer's formula. A real test has to come before this condition can be called passed.
4. **K11 cannot be made by events alone.** Not met. The repairs for H8 and M5 are further events. The claim that the first test passed "by events alone" overstates it: amounts and the use of rights go through Python.
5. **More defects than the previous round.** Met on the raw count: 21 here against eight in the previous round, by one tester. The comparison is not like for like, since K8, K9 and K11 are new surface. Restricted to the norm layer (H3, H4, H5, H6, M4, M6, L3, L6, with L1 and L2 on its tooling) the count is eight to ten against eight. That is not convergence. The count should be merged with the other tester's before the condition is ruled on.

## Novelty

- **Overclaimed: nothing found.** The document claims none, and I found no place where it implies otherwise.
- **Under-attributed.** These are from memory and were not re-checked online:
  - K5 is the standard distinction between regimentation and enforcement in normative multi-agent systems (Jones and Sergot; Grossi, Aldewereld and Dignum). The document derives it from InstAL's power and permission, which is the mechanism, not the source.
  - K9's "reporting failure is not fault", with acknowledgements and a held state until evidence arrives, is close to PeerReview (Haeberlen, Kouznetsov, Druschel, 2007), where an unresponsive node is "suspected" and only proven misbehaviour "exposed". It should be named beside double-entry records.
  - K11's grant before effect is a reserve-then-commit protocol, which is escrow as already cited; no further attribution needed.
  - K4 should say that it is not Greedy, Reduction or Optimization as published, with the separating scenarios of M4.

CRITICAL: 0  HIGH: 8  MEDIUM: 7  LOW: 6

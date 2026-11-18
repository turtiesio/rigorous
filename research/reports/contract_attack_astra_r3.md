# Adversarial review of composition contract, draft 3

Reviewed 2026-11-17. Reproduce with `cd /root/mic && python3 -B model/attack_contract_astra_r3.py`. The [script](/root/mic/model/attack_contract_astra_r3.py) runs all seven maintained checks, the additional scenarios below, and three adapter mutations in memory. It does not edit the adapter or institutions. The existing harness creates its ordinary temporary solver inputs. Historical assertions about inferred exceptions are not used as current expectations.

Counting follows Astra round 2: independent root defects, including reproducible false documentation statements, counted once; controls, variants, objections and disclosed limitations are not additional defects. No counterexample to any of the six current K4 rules was found. This is a bounded testing result, not a proof of correctness.

## A. Confirmed wrong outcomes or false statements

### A1 — HIGH — K11: zero-cost repair breaks the connected executor

Runnable scenario: `A1 zero cost connected`: call the existing `test_ledger_link.play(100, [('free', 0), ('paid', 60)])`. It issues the buy permission and requests the free purchase without a grant, since `Ledger.issue` rejects zero. InstAL correctly emits the free effect. The connected executor then fails rather than accepting it and proceeding to the paid purchase.

Copied run output (the original run used the provisional label A2):

```
PROBE A2 zero cost connected
AssertionError
```

The separating maintained norm scenario passes: a zero-cost effect emits without a grant. The connection still executes `assert granted and L.use(e, cost)` for every emission. Thus round-2 Fable A14 was repaired in the institution adapter but not in the executable connection. This is a synchronous, ordered history, not the disclosed late/replayed-grant limit.

Correct outcome: emitted effects `['free', 'paid']`, ledger used amount 60. Minimal repair: for a zero-cost emission, append it without issuing or consuming ledger rights; retain the reservation/use check for positive costs. Add this history to the connected checks.

### A2 — MEDIUM — K0: claimed coverage of adapter facts is false

Runnable audit: `coverage mutations (in-memory harness source only)`. Mutate exemption eligibility from `po < pn` to `po <= pn`, keeping same-origin eligibility. This incorrectly allows a foreign authority of equal rank to exempt a prohibition. Separately remove exemptions from inherited bridge powers. Neither error is detected by the maintained norm, end-to-end or connected ledger suites. This contradicts “each fact has at least one scenario that fails if the adapter gets it wrong.” Grouped as one false coverage claim, not one defect per mutation.

Copied mutation output:

```
equal rank wrongly exempts ['test_instal.py:PASS', 'test_end_to_end.py:PASS', 'test_ledger_link.py:PASS']
no exemption inheritance ['test_instal.py:PASS', 'test_end_to_end.py:PASS', 'test_ledger_link.py:PASS']
own exemption disabled ['test_instal.py:AssertionError', 'test_end_to_end.py:PASS', 'test_ledger_link.py:PASS']
```

The new separating scenarios show what must be tested:

```
PROBE exemption origin c
answer_sets 1
4 ['occurred(refuse(e),unit)']
5 ['occurred(conflict(o),unit)']
```

Here b and c both rank 1; b issues p, c issues x against p, and a issues duty o. Correct: c's exemption does not work. For inherited exemption, a's x, b's p and a's permission all target the creator; with two descendants, all three units may emit. Dropping inherited x would wrongly refuse the descendants.

Minimal repair: add those separating scenarios to maintained coverage, or narrow K0's coverage assertion. The current adapter's rank comparison itself is correct. A control that removes same-origin exemption eligibility does fail a maintained scenario, so the mutation method is capable of finding existing coverage.

### A3 — LOW — K0: owner rejection statement contradicts the supported default

Runnable scenario: `single unit implicit owner`: one unit; a permits any effect; e has `kind='buy'` and no `of`; events `issue(a), request(e)`.

Copied output:

```
PROBE single unit implicit owner
answer_sets 1
2 ['occurred(emit(e),unit)']
```

K0 says an effect with no owner is rejected. The adapter instead defaults ownership to the sole unit. This is also how many maintained scenarios are written. The outcome is sensible but the unconditional rejection statement is false. Correct outcome under the text as written would be rejection; the preferable minimal repair is to state that ownership is mandatory only with multiple units, and document the sole-unit default. Counted as a documentation defect, not a false InstAL guarantee.

## B. Design objections and evidence strength

- **K0 schema boundary.** `undeclared amendment destination` accepts an effect naming a procedure absent from `procs`; the request is refused and officer remains in force. The adapter should reject unknown procedure references before grounding, rather than silently dropping their typed facts. There is no declared schema promising this rejection, so this is B, not another A. Defaults also determine the initial procedure, own-norm identity, watched/enforced classification, cost gating and the built-in amendment permission; K0's five-item fact list is incomplete. I found no valid input wrongly rejected under the current stated restrictions: empty and tuple histories are now explicitly outside the interface, and a child-targeted own norm is explicitly unsupported.
- **K4 scope and agency.** Conflict counts declared ways, not feasible ways. A duty with one authority-blocked way and another self-blocked or unfunded way is missed, correctly under the new wording. A unit can be practically unable to perform without being in conflict. Conversely, all declared ways can be authority-blocked even when independently infeasible. Do not silently reinstate the old feasibility rule in interpreting these results.
- **K4 lifecycle.** Reissuing a still-undischarged duty resets its miss state. This makes sense as a new activation, but the identity of recurring commitments remains unspecified. A deadline during conflict reports conflict without setting `pastdue`; another deadline after conflict ends can report a miss. The model does not schedule that second deadline itself.
- **K6 is inheritance of norms, not behavioral equivalence.** An authority can target an exemption directly at a child. The child then emits while the creator refuses; this is permitted by the explicit authority edge and targeted exemption, not a breach of K6. Likewise matching patterns can distinguish units by attributes. A child cannot issue a rule against its creator through `ownissue`; that cross-target input is rejected. A creator's own exemption cannot be withdrawn by `withdraw`: own-rule repeal is expressly outside K7.
- **K8 remains arithmetic.** The 25-chain suite now draws fast links and uses separate response and receipt duties. Its deadlines and requests are generated using `verdict.receipt` and `verdict.handover` themselves. Two norm misses are therefore two ordered comparisons of exactly the quantities used by verdict findings and players. To test composition independently, consume certified send/receive records from a separate event producer, independently construct deadlines from the contract, and test activation, norm changes and the handling of refused hand-overs. In particular, distinguish a request from a successful certified performance. No scoped disagreement was found in the maintained two-duty experiment; absence of disagreement there is not independent validation.
- **K9 settlement.** The cancelling-lump probe returns full shares b=1/2,c=1/2 but provisional pays c=1, suspense=0, undetermined `[a,b]`. This is correctly disclosed, not a new counterexample. The text's statement that the loss “stays where it fell” needs a settlement rule if provisional money has already been collected. Timestamp authenticity and contradictions cannot be represented by the residual/presence API. No counterexample to the qualified exactness or indistinguishability proof was found.
- **K2 evidence.** The checker reports 14 derived fluents and 15 dependencies, without a cycle. All successful original-encoding runs examined had one answer set. This does not certify the compiled bridges or arbitrary accepted event strings. Parser failures are not zero-answer-set results.

## C. Limits already listed in draft 3

Differing per-unit orders and clocks (K1); the incomplete uniqueness proof (K2); the finite declared-effect universe (K4); self-declared attributes on duties, permissions and exemptions (K5); dynamic creation, subtree caps and creator-owned rules for children (K6); own-rule repeal (K7); trees, prohibitions during hand-over, faulty evidence and differing clocks (K8); indefinite withholding and missing acknowledgements (K9); grants without amounts and late, repeated or reordered grants (K11). These are not counted again. A duty without funding can be missed by design.

## Current K4 rule-by-rule attack results

| Rule | Executed attacks and result |
|---|---|
| 1 Default refusal / duty permission | Broad exemption without a separate permission refuses f; duty permits e; after discharge the second request refuses. Amendment retains its independent charter permission. No defect found. |
| 2 Forbid overrides permit | Both authority rank directions in maintained tests refuse and report conflict; partial bans refuse the blocked effect but leave a miss possible. No defect found. |
| 3 Explicit exemption | Before-p issuance works; withdrawal restores conflict; reissue restores emission; same-origin works; equal-ranked foreign and lower-ranked own exemptions fail; own exemption against own prohibition works. Broad exemption does not create permission. Naming another unit's prohibition has no effect on the unit where that prohibition is absent. No defect found. |
| 4 Conflict | Partial authority ban plus own ban or missing funds gives a miss. Conflict deadline then withdrawal then two deadlines gives one miss; discharge/inactive duty gives no spurious conflict. A request cannot discharge an actually conflicted duty because all its owned matching effects are forbidden. Removing conflict with an exemption permits discharge. No defect found. |
| 5 Lateness | Conflict then miss then repeated deadline records only one miss; late request emits. A discharged and reissued duty becomes owed and can be missed again. No defect found. |
| 6 Discharge | One emission sets done for two matching duties; neither deadline misses, and a second emission lacks duty permission. No defect found. |

Additional K7 attacks: an officer-to-officer amendment preserves officer (the suspected simultaneous initiate/terminate defect did not reproduce); an undeclared destination refuses; maintained procedure-change, self-prohibition and amendment-duty cases pass. K5 watched emissions record a breach without an exemption and none with one; a watched prohibition does not cause conflict. K6 depth-two exemption and permission inheritance work in reverse unit-list order.

Representative transition output:

```
PROBE conflict ends between deadlines
answer_sets 1
3 ['occurred(conflict(o),unit)']
5 ['occurred(missed(o),unit)']
7 ['occurred(emit(e),unit)']

PROBE two duties one emission
answer_sets 1
3 ['occurred(emit(e),unit)']
6 ['occurred(refuse(e),unit)']
final ['holdsat(done(o),unit)', 'holdsat(done(q),unit)', 'holdsat(proc(officer),unit)']
```

## Every round-2 list-A item

“Fixed” means the reported behavior or false claim has been repaired under draft 3, not that every possible variant has been proved safe. “No longer applicable” means the obligation has been withdrawn or its boundary explicitly changed.

| Tester / item | Status | Current evidence / qualification |
|---|---|---|
| Astra A1 watched cheaper-way exception | No longer applicable | Inferred exceptions removed. Watched duty emission now records breach unless explicitly exempted. |
| Astra A2 infeasible way suppresses miss | No longer applicable | Feasibility no longer defines conflict; partial authority ban/unfunded probe misses. |
| Astra A3 foreign declared way / bad owner | Fixed | Owner checked even with one unit; owned matching effect required for a duty. Rejection probes pass. |
| Astra A4 tuple / empty history | Fixed | Actual string interface documented and validated; empty histories explicitly rejected. |
| Astra A5 historical count attribution | Fixed | 21 versus eight and eight versus eight now quoted accurately; scope qualification retained. |
| Astra A6 earlier comparison-rule attribution | Fixed | `bar_rule` name/docstring and contract expressly distinguish textual comparator from earlier executable model. |
| Fable A1 watched exception | No longer applicable | Same as Astra A1. |
| Fable A2 manufacturing infeasibility | No longer applicable | Own bans and infeasibility do not shrink the declared-way universe. |
| Fable A3 discharge while overridden | Fixed | One emission discharges both duties; no overridden duty state remains. |
| Fable A4 conflict deadline prevents later miss | Fixed | Conflict deadline does not set pastdue; next deadline after withdrawal misses once. |
| Fable A5 fast-link K8 mismatch | Fixed | Separate send/receipt duties; 25 chains include fast links and compare both sets. |
| Fable A6 K1 denies cross-unit clocks | Fixed | Cross-unit timing premise now explicit; independent clocks remain untested. |
| Fable A7 list order / default edges | Fixed | Multiple units require explicit edges and external norm targets; reversed depth-two order passes. |
| Fable A8 creator norm binds wrong unit | Fixed | Cross-target own norm rejected. Child governance by own norms remains outside scope. |
| Fable A9 untested adapter facts | Partly fixed | Ownership separation now tested; equal ranks remain undetected by maintained tests. Current A2. |
| Fable A10 empty allocation test claim | No longer applicable | Document now says a grid checks the argument on one pair. That algebraic check really runs; it is not model validation. |
| Fable A11 historical record | Partly fixed | Counts, 60/87 attribution and narrowed claims improved. Opening “eleven” remains a compressed historical selection; no complete disposition ledger in the contract. |
| Fable A12 foreign-only duty way | Fixed | Same as Astra A3. |
| Fable A13 own repeal | No longer applicable | Explicit K7 exclusion, not implemented. |
| Fable A14 zero-cost purchase | Partly fixed | Norm layer allows it; connected helper now aborts. Current A1. |
| Fable A15 empty history | No longer applicable | Explicit rejection is the new interface contract. |

## Document and literature audit

All seven maintained suites pass: 43 norm scenarios (37 solver scenarios and six rejection checks), 25 connected chains, four connected ledger histories, 11 evidence checks including 400+300 random chains, 15 verdict cases plus 300 random chains, and six ledger checks plus 2000 histories. K3's five, K4's eighteen, K5's three, K6's three and K7's seven match their named scenario groups. These are overlapping clause coverage, not additional independent totals. K11's fifth connected scenario is the standalone norm-layer grant/free-effect scenario, not a fifth `play` history; it does not cover A1. K0's universal coverage claim fails A2. K9's eleven includes the algebraic allocation-grid argument; the revised text accurately describes its limited nature.

The “what was wrong in draft 2” rows accurately describe the named prior findings and their authors. It is a selection, not an exhaustive list: it omits the free-purchase, coverage and allocation-test findings, among others. The opening “eleven wrong outcomes or false statements” is not a reliable exhaustive count of round 1: the named reports used eight counted findings and 21 findings under different scopes. Treat it as the historical selected defect groups, or replace it with an explicit deduplicated ledger. This ambiguity is not counted as another A after the numerical stopping record itself was corrected.

The stop-5 record correctly gives the authors' raw counts and says their scopes differ. Six is below eight; fifteen is below 21. That establishes a numerical fall, not a controlled convergence experiment. Saying K4 “starts again” is an extra policy decision, not an implication of the stated stop condition. Retain the full-scope record as well as a separate K4 record, so changing the contract cannot silently reset the stopping test.

The checked primary-source descriptions support the three narrow precedents: [Li et al.](https://researchportal.bath.ac.uk/en/publications/legal-conflict-detection-in-interacting-legal-systems/) describe detecting conflicts in interacting legal specifications; [Cedar authorization](https://docs.cedarpolicy.com/auth/authorization.html) explicitly uses default deny and forbid-overrides-permit; [Governatori et al.](https://arxiv.org/abs/1212.0079) discuss explicit permissions derogating from prohibitions. They do not establish that this exact all-declared-ways conflict predicate, target-specific exemption relation and origin-rank rule are InstAL's own semantics. “Follows InstAL's own approach” is defensible as an approach to detecting instead of resolving conflict, not a semantic equivalence result. I verified those source descriptions, not an equivalence proof or the universal assertion that the entire InstAL literature leaves resolution to parties. No novelty claim is made, and these citations do not prove composition.

## Stop conditions

| Condition | Verdict |
|---|---|
| 1 No trace or several | Not met in successful solver runs examined. Explicit validation rejections and the connected executor assertion are not semantic nonuniqueness. |
| 2 Repair needs changed InstAL semantics | Not met. A1 requires a connection fix; A2 requires tests or a weaker claim; A3 requires truthful default documentation. |
| 3 K8 requires restating norm rules in verdict | Undecided, not demonstrated met. Two-duty wiring passes; it does not independently test hard composition. |
| 4 K11 cannot be made by events alone | Not established. A1 admits a local zero-cost branch; no impossibility has been shown. |
| 5 Two consecutive same-scope rounds with neither tester's count falling | Not met by this review. Astra falls from six to three under the same independent-defect counting convention. K4's changed semantics constrain what that improvement means. The other tester's round-3 result is not available here; this review does not invent it. |

CONFIRMED WRONG OUTCOMES (list A): 3
CRITICAL: 0  HIGH: 1  MEDIUM: 1  LOW: 1

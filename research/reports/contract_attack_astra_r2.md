# Adversarial review of composition contract, draft 2

Reviewed 2026-11-17. The fixes materially improve the model, but the watched exception is still wrong and infeasible effects still influence whether a duty is overridden. Six independent findings are counted below; two contradict explicit clause guarantees. No change to InstAL semantics has been shown necessary.

Reproduction: `cd /root/mic && python3 -B model/attack_contract_astra_r2.py`. The [script](/root/mic/model/attack_contract_astra_r2.py) runs all seven maintained checks, replays both original attack scripts against the current modules, and runs the additional probes named below. Old assertions are evaluated without enforcing historical expectations. Rejected scenarios are printed as errors; downstream historical messages after a failed assignment can contain stale variables and are **not evidence**. Historical DEFECT labels and explanatory prose are also not the round-2 assessment. Fable's source mutation experiments are replaced by in-memory source interception, so existing files are not changed. The harness still creates its ordinary temporary solver inputs.

All seven maintained suites passed: 35 norm scenarios, the dependency checker, 25 connected chains, four ledger histories, evidence tests including 700 random chains, 15 verdict cases plus 300 random chains, and six ledger cases plus 2000 histories. Every successfully solved original-encoding scenario examined had one answer set. Parser errors are distinguished from zero-answer-set solver results.

## A. Confirmed wrong outcomes and false statements

### A1 — CRITICAL — K5/K4: watched exception ignores a cheaper lawful way

Runnable probe: `A1 cheaper alternative watched=True`, with the otherwise identical enforced control. Authority a's duty o requires any buy; lower b prohibits vendor x. Effects x and y are buys from vendors x and y. Events: `issue(o), issue(p), request(x), deadline(o)`. Only vendor is self-declared in the watched case; the duty reads certified kind, so this is not the excluded self-declared-duty case.

Copied output:

```
PROBE A1 cheaper alternative watched=False
answer_sets 1
3 ['occurred(refuse(x),unit)']
4 ['occurred(missed(o),unit)']
PROBE A1 cheaper alternative watched=True
answer_sets 1
3 ['occurred(emit(x),unit)']
```

There is no `breach(x,p)` in the watched run. Correct: emission is allowed, discharges o, **and records the breach**, because enforced p would not yield while y is available. K5 expressly uses that counterfactual. `wexcepted` checks only that a matching higher duty exists, unlike `excepted`, which checks `cheaperway`. Minimal repair: compute the relevant hypothetical enforced-prohibition exception for watched norms, including cheaper alternatives and their priority. Simply adding the current `cheaperway` predicate is insufficient because its definition requires `prohibits(N)`, which watched N does not satisfy. The original single-way fix passes, but its general guarantee is false.

### A2 — CRITICAL — K4: an infeasible prohibited effect suppresses a miss

Runnable probe: `A2 amendment control`. Duty o (b, lower) requires amendment. Both declared amendments require board; officer is in force. x changes to vote; y changes to board. Higher a prohibits changes to vote. Events: `issue(o), issue(p), deadline(o), withdraw(p), deadline(o)`.

Copied output:

```
PROBE A2 amendment control
answer_sets 1
final ['holdsat(inforce(o),unit)', 'holdsat(pastdue(o),unit)', 'holdsat(proc(officer),unit)']
```

No `missed(o)` occurs. K4.2 excludes both effects from the set of ways; consequently no actual way is blocked by p. K4.3's additional requirement that a same-or-higher prohibition blocks the duty cannot be supplied by x, which is infeasible independently of that prohibition. Correct: record the miss on the first deadline, once. This is not a duty outranking a spending cap: the separating case uses only charter procedures. The costed companion probe shows the same defect and later succeeds after funding.

Cause: `hasway` excludes infeasible effects, but `blockedway` and therefore `someblocked` do not. Minimal repair: restrict `blockedway` to feasible effects too. `pastdue` then freezes the incorrect exemption: removing the irrelevant ban and sending another deadline does not recover the missed finding. The current original lower-duty regression still passes because its prohibited officer amendment really is feasible; this new case is different.

### A3 — HIGH — K0/K4: declared-way validation ignores ownership

Runnable probes: `A3 duty with only foreign effect` and `single sink mismatched owner`; original Fable K4-10 reproduces the first. Duty o targets v; the only matching buy x belongs to unit. Events: `issue(o), request(x), deadline(o)`.

Copied output:

```
2 ['occurred(refuse(x),unit)']
3 ['occurred(missed(o),v)']
```

The input passes the adapter's asserted “obligation has a declared way” guard, although v cannot perform any declared matching effect. With one sink, even explicit owner v outside the sink set is accepted:

```
PROBE single sink mismatched owner
answer_sets 1
3 ['occurred(missed(o),unit)']
```

Correct adapter outcome: reject these malformed specifications before solving. This is a validation defect, not a claim that all impossible duties must be forgiven. Minimal repair: validate every explicit owner even for one sink, resolve the default owner once, and check obligation matches within the target owner's effects. The new multi-unit owner check fixes duplicate emissions but not this original guard defect.

### A4 — HIGH — K0/K2: documented histories cannot be executed

Runnable probes: `documented tuple events` and `A4 empty history`. Norm p permits buy x. The first passes the documented event representation `[('a','issue(p)'), ('unit','request(x)')]`; the second passes `[]`.

Copied output:

```
PROBE documented tuple events
RuntimeError <string>:1:26: error: unexpected token:
PROBE A4 empty history
RuntimeError <string>:2:2: error: syntax error, unexpected <EOF>
```

Correct: either support institution/event pairs, or document and validate the actual unqualified string representation; an empty history should yield the initial trace. The adapter interpolates Python tuples into ASP, and the empty query reaches an invalid parser path. Minimal repair: explicit input-schema validation/documentation and a supported initial-state query path. These are adapter failures, **not evidence that InstAL has zero or multiple answer sets**, so they do not establish stop condition 1 under its semantic interpretation. They remain uncorrected parts of Fable L1/L2. Grouped as one interface defect, not counted twice.

### A5 — MEDIUM — stop condition 5: historical counts are misattributed

Runnable audit: `COUNT AUDIT` reads the two original reports and draft 2. Draft 2 says “21 against 22 for one, 8 against 8 for the other” and concludes one same-scope nonconvergent round already happened. The actual Fable report says:

> 21 here against eight in the previous round, by one tester.

Astra's report says:

> Eight counted findings versus eight prior reported defects

The evidence in the named round-1 reviews does not supply the claimed 22. Further, Fable explicitly says the scope expanded; its norm-layer-only comparison is eight to ten against eight, not an established same-scope count of 21. Correct: report 21 versus 8 and 8 versus 8 as those authors' raw statements, alongside their comparability qualifications; do not manufacture the first qualifying round for the new stopping rule. Minimal repair: a common scope and deduplication ledger before applying the two-round rule. The script prints the source passages so this is reproducible without relying on memory.

### A6 — MEDIUM — K9 model attribution: `old_bar` still is not the prior executable rule

Runnable replay: Astra D6 and Fable K9-9. A has own lateness 1, B 0, C 1/2; B is missing. Copied Astra output:

```
{'actual_previous_model': {'C': Fraction(1, 2), 'A': Fraction(1, 1)},
 'comparison_old_bar': ({'B': Fraction(3, 2)}, Fraction(3, 2))}
```

Correct provenance: the prior executable `verdict(absorb_missing(...))` does not charge B in this case. Yet `evidence.old_bar` still labels itself the first group-B draft's rule, and the maintained tests still use it to demonstrate what “old bar” does. Minimal repair: rename this as a deliberately different comparator, or implement/call the prior rule faithfully. Draft 2 correctly credits acknowledgements, so its replacement comparison table is improved; the executable attribution remains false. This is not a failure of the revised exactness proof.

## B. Design objections and claims stronger than their evidence

These are not included in list A's count.

* **K1:** the old interleavings still differ, but they move the authority's receipt across the unit's request and therefore do not preserve the revised unit log. They are no longer a counterexample to K1. The new independent-request interleavings emit x in unit and y in v in either order. I found no pair satisfying the revised receipt-inclusive premise with differing outcomes. The API still cannot represent independently delayed receipts of one authority event to different units; the document now admits that coverage gap.
* **K4:** Fable K4-4 still emits e2 then e1, although e1 alone could meet both duties. This is not joint minimization, but K4 never promises joint minimization and explicitly says several obligations are open. Same-origin ties and two own duties under an equal-priority own prohibition refuse x without misses, consistent with the stated priority rule. A perpetual overdue exception is intended until discharge; late performance uses it once. Withdrawal ends it; discharge followed by another deadline does not reopen it. Reissue deliberately resets done/pastdue and produces another miss; recurring activation identity remains unspecified.
* **K7:** an own prohibition against amendment still cannot be repealed by any implemented own-withdraw event. That is a design trap, not a refutation of the stated six amendment cases or a promise that every charter is escapable.
* **K8:** besides the explicitly acknowledged slow-link mismatch, Fable K8-3 still gives `InstAL missed [], verdict findings {'u0': '1'}` for own lateness +1 offset by link residual -1. Thus disagreement goes both ways: a fast link masks own failure in the receipt duty. The 25 tests use nonnegative link residuals and still pass. Their “what is tested” statement is accurate for those tests, not a universal equivalence theorem. Stop condition 3 is not proved met: separate send and receipt obligations could preserve the distinction without putting norm resolution into the verdict. A repair must be executed before claiming composition.
* **K9:** rerunning the whole verdict on successive evidence sets correctly keeps all shares provisional while any lump remains. The script tests both splitting a lump and resolving only one of two separate lumps. This is sufficient for the pure function's accounting; no settlement/refund protocol implements revising amounts already collected. Contradictory acknowledgement timestamps cannot be represented: the API receives a residual chain, summary-presence set and lost-ack set, not signed timestamp records. This is an evidence-validation gap, not a counterexample under consistent certified evidence. The qualified exactness argument remains sound with known endpoints, delivered acknowledgements and nonadjacent missing summaries. It still cannot recover own/link findings.
* **K2:** the checker ignores generated-event cycles and loses indented or multiline derived dependencies; the in-memory cycle probes compare it with InstAL's own occurs check. Draft 2 now correctly calls this a necessary check, not a certificate. No uniqueness proof for the compiled whole exists here.
* **Test evidence:** the allocation enumeration in K9 checks the two-history impossibility argument algebraically, not the evidence interpreter. “Enumerates the allocations” is literally true for the finite test domain; it is weak software validation, not an independent proof of the general theorem. The test_evidence header still lists P2/P3 as tested properties despite deliberate counterexamples. It should say which are refuted and which hold conditionally.

## C. Limits already acknowledged by the document

Not counted again: asynchronous/replayed grants, absent grant amounts, no cross-layer cancellation event, duties outranking caps, fixed institutions, creation during a run, subtree caps, routes through intermediate authorities, self-declared attributes in duties/permissions, repeated triggers, act windows, changing commitments, evidence trees and faulty evidence, broader verdict domains, and the lack of a whole-program uniqueness proof.

The grant probes substantiate the limits: two grants before one request collapse to one funded fluent; a second request is refused. Replaying a grant after spending enables another emission in the original Astra scenario. A grant with no request leaves `holdsat(funded(x),unit)` indefinitely. The old expired-lease and underfunded-reservation scenarios still emit when `Ledger.use` rejects. These histories break synchronization assumptions explicitly excluded by K11. In the current turn-taking loop, a refusal clears funded and expires/settles the reservation; the following affordable allowed purchase succeeds. Nothing in those runs establishes that a dropped grant should have been retained under that loop's stated semantics.

## Round-1 defect disposition

“Fixed” can mean corrected executable behavior or an explicitly corrected/narrowed claim; the reason identifies which. All original scenario code was replayed except the on-disk mutation mechanism, replaced by in-memory probes. Rejected ownerless or out-of-domain scenarios are validation successes, not stale-output successes.

| Original defect | Status | Current evidence / remaining issue |
|---|---|---|
| Astra D1; Fable L2 charter input | Partly fixed | `power` now honored; tuple-event documentation still false (A4). |
| Astra D2; Fable H4 inheritance | Fixed | Grandchild now refuses; explicit targets with two creators and reversed sink order also refuse. |
| Astra D3; Fable M6 duplicate spending/global effects | Fixed | Multi-unit ownerless inputs rejected; maintained owner-scoped grant test passes. Other ownership validation is A3. |
| Astra D4; Fable H8 reservation leak | Fixed in the stated loop | Old scripts omit the newly required feedback; maintained loop releases rights and emits b2. |
| Astra D5; Fable H7 invisible lumps/finality | Fixed | Zero/negative lumps retained; provisional until all lumps resolved; 19,000-history replay finds zero hidden unlisted deviators. |
| Astra D6; Fable M1 comparator | Partly fixed | Draft credits acknowledgements; executable historical comparator remains mislabeled (A6). |
| Astra D7; Fable L1 checker | Partly fixed | Scope disclaimer corrected; checker still weak and empty query still fails (A4). |
| Astra D8; Fable H1 late receipt | Fixed | Missed then emitted, including both late deliveries; no late refusal in 25-chain replay. |
| Fable H2 K8 arithmetic/circular claim | Partly fixed | Calls verdict now and admits wiring-only evidence; semantic mismatch remains acknowledged or B above. |
| Fable H3 deadline lifts any norm | Fixed | Both prohibition and permission survive their deadline events. |
| Fable H5 infeasible alternative | Partly fixed | Original two amendment cases pass; infeasible blockedway still affects overriding (A2). |
| Fable H6 watched breach | Partly fixed | Original one-way case fixed; cheaper-way case wrong (A1). |
| Fable M2 receiver opens lump | Fixed as disclosure | Behavior unchanged; now explicitly admitted. |
| Fable M3 exactness/findings/endpoints | Fixed as statement | Shares-only theorem and both endpoints now stated. |
| Fable M4 multiple duties/literature relation | Fixed as qualification | Behavior unchanged; no claim of equivalence to a named selection procedure remains. |
| Fable M5 amountless/endless grants | Fixed as qualification | K11 now names turn-taking and excluded asynchronous cases; protocol still limited. |
| Fable M7 global order | Partly fixed | Revised receipt-inclusive premise defeats old counterexample; independent receipt orders still not implemented. |
| Fable L3 own repeal | Not fixed | No ownwithdraw; classified B because no universal repeal guarantee is stated. |
| Fable L4 limit test/header | Partly fixed | Counterexamples added; stale header remains; finite enumeration not implementation validation. |
| Fable L5 counts/attributions | Partly fixed | 15 verdict cases and narrower InstAL claims corrected; new historical comparison error A5. |
| Fable L6 foreign declared way | Not fixed | A3 reproduces it. |

## Claims table and attribution audit

The named suite counts 35, 25, 15/300, 6/2000 and 11/700 agree with actual test output. K11's five connected checks are four histories plus the owner-grant norm scenario. K4's nineteen is a coverage grouping, not nineteen disjoint additional runs; clause coverage overlaps, so table entries must not be summed as independent scenario totals. The checker now reports 21 non-inertial fluents and 32 dependencies. The historical “60 further runs” comes from Fable's successful solver runs, not all histories submitted: the known empty-query exception was excluded. Reading “every scenario has exactly one outcome” as including parser failures would be inaccurate.

The eleven-row draft-1 table is a selection of defects, not a complete disposition of both reports. Its listed authors match the reports for the named defects. Its claim that this draft “corrects them” is too broad for H5/H6 and for the unimplemented differing receipt orders. The linked formalisms are not represented as new work; no new literature equivalence or attribution is asserted by this review. I have not independently verified the publications or the claim that exactly two vendor lines changed. Those are unverified provenance claims, not confirmed falsehoods. A6 is independently reproducible executable misattribution.

## Stop conditions

| Condition | Verdict |
|---|---|
| 1. No trace/several | No solver counterexample found. Empty/documented-tuple histories fail before solving (A4); a literal end-to-end “every history has a trace” claim fails, but this does not demonstrate semantic nonuniqueness. |
| 2. Repair needs InstAL semantic change | Not met by this review. The confirmed defects have institution/adapter/documentation repairs. |
| 3. K8 needs norms restated in verdict | Undecided, not established met. Two directions of mismatch exist, but no impossibility argument rules out distinct send/receipt duties and events. |
| 4. K11 cannot work by events alone | Not established. Current scoped loop passes; broader protocol limitations do not prove impossibility. |
| 5. Two same-scope rounds without either tester's count falling | Not established. This review counts six versus Astra's eight, although the severity taxonomy changed. The historical basis for asserting one qualifying prior round is wrong (A5), and the other round-2 tester's count is unavailable. Do not claim convergence from this count alone. |

CONFIRMED WRONG OUTCOMES (list A): 6
CRITICAL: 2  HIGH: 2  MEDIUM: 2  LOW: 0

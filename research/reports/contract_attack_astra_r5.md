# Draft 5: adversarial review

Reproduce with `python3 -B -u /root/mic/model/attack_contract_astra_r5.py`. The [script](/root/mic/model/attack_contract_astra_r5.py) runs maintained suites, explicit timed histories through the real compiler and solver, the round-4 route probes, validation probes, and in-memory adapter mutations. It imports the existing round-4 scenario helpers; it does not edit their source. The suspension-reader statements are extracted from the maintained test and executed on new traces, rather than independently reimplemented. All solver calls request all answer sets. Compiler scratch files are incidental to running the existing harness.

Counting follows Astra rounds 2–4: independent root defects, including false statements; variants are not additional findings. **Two list-A items, both in material written after round 4; none in older material.** This is a bounded review, not an assertion that no other defect exists. In particular, malformed-input exceptions are retained in B, consistently with the earlier Astra review, rather than promoted merely to raise the count.

## A. Confirmed wrong outcomes or false statements

### A1 — HIGH — K8/K11: a refusal outside conflict becomes a conflict suspension

Run `timings()`, label `unfunded before later conflict`. One unit starts at 0, budget 3. Its duty permits the sole effect, which costs 1. It requests at 1 without a grant; a prohibition runs from 2 to 4; at 4 it receives a grant and delivers. There is **no request during the conflict**. There is one duty and one ban, not the expressly excluded recurring-conflict or two-duty case. Unfunded refusal is part of K11, which explicitly says an unfunded duty is the unit's miss.

Copied output:

```
TIMED unfunded before later conflict models 1
  1 request(e) ['occurred(refuse(e),unit)']
  4 request(e) ['occurred(emit(e),unit)']
 reader {'unit': ([(Fraction(2, 1), Fraction(4, 1))], Fraction(1, 1))} suspension {'unit': Fraction(3, 1)} deadline 6
 verdict {'loss': Fraction(0, 1), 'under_prohibition': Fraction(0, 1), 'shares': {}, 'reproduces_actual': False} InstAL missed False
```

The reader selects the first `refuse` without checking that the duty was in conflict at that attempt. It pairs that refusal at 1 with the end of a different cause of refusal at 4. The purported three-day suspension even exceeds the two-day conflict. The re-run takes work=1, gates time 1 through [2,4), and returns 1 instead of the certified receipt 4. It reports loss zero although receipt minus the original deadline is one.

**Correct outcome:** there is no qualifying conflict refusal, hence no three-day clock suspension. Under the test's declared tie convention for a deadline that fell due during conflict, the deadline is sent at 4 before the request and records `missed(o)`. The script also runs that corrected event order as `corrected deadline at conflict end before receipt`. A verdict may need a convention for the intervening funding wait; it cannot honestly present a receipt at 1 as the actual receipt at 4. At minimum it must reject this unsupported input instead of returning an ordinary zero-loss verdict with a false reproduction flag.

**Minimal repair:** extract a refusal together with its reason and membership in a specific conflict interval. Only a refusal during that interval may start its suspension. Do not feed an unrelated refusal to `gated.Unit.work`. Validate the re-run's evidence preconditions and refuse to finalize when reproduction fails. If all non-conflict refusals are intentionally excluded from K8, state that additional composition limit and enforce it at the boundary. This repair changes the adapter/evidence connection, not InstAL semantics. The clock rule as written is not itself disproved; its claimed executable implementation is wrong on this accepted history.

### A2 — MEDIUM — K8: the fixed-length comparison tests a different rule

Run `fixed_audit()`. `test_gated.fixed_extension` sets extension to `gate(due, closed)-due`. K8's revised rule instead stops from the timely refused attempt. They are different lengths. The first advertised counterexample and the 501-difference statistic use the former.

Copied output, retaining the relevant fields:

```
FIXED absorbs test helper {'miss': Fraction(1, 1), 'shares': {'u1': Fraction(1, 1)}, ...}
K8 extensions [Fraction(0, 1), Fraction(2, 1)]
K8 actual rule {'miss': Fraction(0, 1), 'shares': {'u1': Fraction(0, 1)}, ...}
gated {'loss': Fraction(2, 1), 'under_prohibition': Fraction(2, 1), 'shares': {'u1': Fraction(0, 1)}, 'reproduces_actual': True}
```

**Hand calculation:** u1 has budget 2 and takes 3. u2 has budget 2, starts at 3, attempts at 4, and meets [2,6). Its deadline is 5. The rule now stated suspends from 4 to 6, length **2**. With the test's convention of also moving the claimant deadline, total allowance is 2+2+2=6, exactly the final receipt. u1 pays zero, as it does on re-run. The helper uses length **1**, from deadline 5 to end 6, and therefore charges u1 one. “The fixed length charges the first unit a day” is not true of the first-refusal fixed length introduced immediately above it. The helper's own docstring mislabels its deadline-to-end rule as draft 5's first rule.

**Correct outcome/minimal repair:** identify exactly which fixed rule each experiment tests. Change the comparator to the stated first-refusal extension and rerun the statistics, or label it expressly as a different, hypothetical deadline-based rule. Withdraw the first example as a counterexample to the revised suspension-length rule. Do not withdraw the narrower conclusion that fixed lengths can fail: the second example still proves that.

For the second example, u1 takes 3 against budget 2; u2 takes 2, meeting [5,9). Actual refusal is at deadline 5, extension 4. The fixed calculation has deadline 8, receipt 9, and charges u1 **1**. With compliance, u1 ends at 2 and u2 at 4, avoiding the ban; actual loss against deadline 4 is **5**, all caused by u1 under the declared re-run convention. Both executable comparisons give these results. This remains a valid counterexample.

## B. Design objections and evidence limits

1. **Stop condition 3 is not settled by passing the re-run tests.** `gate()` evaluates membership of a counterfactual arrival time in conflict intervals and delays it to their end. This is a model of enforcement/response behavior. It does not re-evaluate pattern matching, permissions, prohibition ranks, exemptions, or the all-ways conflict predicate. In that narrow sense “no norm rule is evaluated” is defensible. In the broader sense that the verdict knows nothing about prohibited delivery, it is false: closed intervals are its central transition condition. The document should maintain this distinction. It also assumes that the supplied absolute intervals remain correct under every intervention on upstream work. Neither the tests nor the unit-derived-fluent checker proves that assumption for arbitrary institutional histories. A1 demonstrates a concrete boundary failure, not impossibility of a sound boundary.
2. **The oracle is independently supplied, not an independently justified notion of fault.** For the generated one-ban controller, intention after the original deadline is just work beyond budget. The re-run player predicate is the same convention expressed through the first refusal. The generator ensures immediate delivery at the end. The test genuinely checks InstAL wiring and trace extraction for that class; it cannot validate the convention or establish that all accepted evidence belongs to it. Draft 5 acknowledges most of this accurately.
3. **Algebraic properties are stronger than empirical coverage, but conditional.** For finite well-formed intervals, `gate` is nondecreasing. Replacing work by `min(work,budget)` cannot make the final receipt later. Thus coalition marginals are nonnegative. Summing marginal contributions telescopes to loss(all)-loss(empty), and averaging preserves that identity. Unique unit names and consistent chronological evidence are preconditions, not checked by `gated.py`. No negative share or charge to an actually within-budget unit was found on conforming histories. Generating actual receipts with the same gate being tested makes `reproduces_actual` largely constructional.
4. **Validation remains uneven.** The script obtains `KeyError 'pattern'`, `InstalParserSyntaxError Syntax error at 'Board'`, and `IndexError list index out of range` for missing pattern, capitalized authority, and empty event. A finite nonnegative `Fraction(1,2)` cost is rejected as “not a finite number”; the supported numeric representation is narrower than the wording. These are the old validation surface, not new silently changed norm outcomes. A blocked map `{'ghost': []}` is accepted, but declares no refusal and changes no result. Do not count that harmless empty entry as a blocked-hop bypass. An initial capital-authority probe accidentally removed other authorities and failed on an unrelated edge; the final script preserves them and reaches the intended parser failure. This illustrates why “rejected for any reason” is weak evidence.
5. **Routes are still weakly tested by the maintained suite.** The adversarial three-authority route obeys both blocked intermediates; an unblocked issue and withdrawal reach the target and a created child. The repeated-authority exploit is now rejected. Direct-edge-plus-route and a route returning to its origin are rejected as specified. No authority-act-only bypass was found. These restrictions narrow the supported input; rejection of a direct-and-indirect route is a declared design choice, not an incorrect implementation. The route rank objection remains unresolved.

## C. Limits already stated, not counted again

- **K4 unreachable way:** `old.disclosed_open()` still runs the amendment path that cannot be reached. It records a miss rather than conflict. This is expressly the open wrong outcome; it is not an additional A item.
- **K8 waiting without attempting:** the single-unit budget-3 receipt-5 case with ban [1,5) has player `u` but share zero because all loss remains under prohibition. In other timings waiting can be charged. The distinction between actual readiness and an unlogged attempt is the stated policy limit.
- **K8 repeated conflicts/controller deviation:** in `two separated conflicts`, attempts are at 1,4,6, bans [1,2), [4,6). The reader chooses the first end, moves deadline to 4, and that deadline records conflict. The re-run delivers at 2, not actual 6. This controller did not deliver at the first end as the declared convention requires, and recurring conflicts are already listed as unmaintained. It is not another A item. It does show why blindly accepting an arbitrary first-refusal timestamp is unsafe.
- **K8 deadlines:** all later deadlines are still calculated offline by the test. No evidence-layer runtime schedules them. The two-duty response/receipt case, exemption ending conflict for only one way, differing clocks, trees, and responsibility for prohibition loss remain open.
- **K12 log injection:** directly observed `relayed(p)` enters below a blocked intermediary; directly observed `relaywithdrawn(p)` ends the norm. Both are reproduced by the old route helper and expressly disclosed now. Static `mayrelay` cannot represent changing consent or a selectively declined withdrawal. Translation, leased context and rank-policy objections remain open.
- **K11:** grants carry no amounts, and concurrent/reordered grant delivery remains outside the connected tests. A1 does not require concurrent or reordered grants: it uses a single ordinary later grant.

## Timing, arithmetic and route checks

The timed probes use authority acts before requests and ordinary deadlines after requests at equal times. Deadlines deferred because conflict covered the original due time precede the request at conflict end.

| Probe | Observed result |
|---|---|
| Refusal exactly at deadline 3, conflict [2,5) | Suspension 2, deadline 5, emits at 5, no miss; re-run reproduces |
| Conflict ends exactly at deadline 3 | Withdrawal then delivery at 3; no suspension or miss |
| Two refused attempts at 2 and 3 during [1,5) | Uses first refusal 2; deadline 6; receipt 5; no miss |
| Touching bans [1,4), [4,6) | Trace reader yields [1,6); receipt 6 reproduced |
| Exemption ends the entire one-way conflict at 4 | Reader ends interval at 4 rather than scheduled withdrawal 8; receipt reproduced |
| No attempt until conflict ends at 5, original deadline 3 | Deadline at 5 before request; InstAL miss; gated player has zero share |
| One-unit zero budget, attempt 0, conflict [0,2) | Loss 2 entirely under prohibition; no players; reproduction true |
| Early unit, no conflict | Loss 0, no players |
| Closed interval begins before readiness | Receipt reproduced; within-budget unit pays nothing |

The continuity example is correct by hand: budgets 2 and 1 give claimant deadline 3. If u1 takes 2.999, u2 ends at 3.999 just before [4,9), so u1's share is 0.999. At u1 work 3, u2 attempts at 4, receipt becomes 9 and the share is 6. The jump is 5.001 across a work difference of 0.001; as the difference tends to zero, the jump tends to 5.

Three-authority blocked-route probes emit on the first request. Their *second* request can refuse because the first discharged the only permitting duty, not because the blocked prohibition slipped through. This matters when reading abbreviated traces. The created-child control uses a permission rather than a one-shot duty and gives refuse before withdrawal, emit after it.

## Every round-4 list-A item

| Prior item | Status in draft 5 | Evidence and qualification |
|---|---|---|
| Astra A1 repeated authority bypasses blocked hop | Fixed | Cycle and blocked-cycle inputs rejected; simple three-authority blocked route does not deliver norm |
| Astra A2 two coverage repairs ineffective | Fixed | Ownership and foreign-match mutants are checked again below; the new scenarios distinguish their results |
| Fable A1 ban never encountered excuses own lateness; false 24 count | Fixed for its one-ban controller | Revised rule requires an attempt; maintained intention oracle/count audited; A1 here is a different non-conflict-refusal association bug |
| Fable A2 test does not call verdict/evidence; fixed length unsupported | Partly fixed | Both verdicts now called; amounts model exists; deadline placement remains test glue and expressly open; comparator mismatch A2 remains |
| Fable A3 route/direct-edge and route validation | Fixed for reported cases | Direct-edge routes, cycles, invalid end and invalid blocked membership rejected; restrictions documented |
| Fable A4 adapter validation | Partly fixed | Sets, finite nonnegative float/int cost, list edges and creation-cycle handling repaired; old malformed-input exceptions/numeric representation remain B |
| Fable A5 six coverage repairs claim | Fixed | Two offending coverage checks replaced; claim narrowed to tested mutations rather than blanket coverage |

Of the seven prior entries: five fixed, two partly fixed. No prior list-A entry is silently treated as a new A merely because it remains a stated limit or objection.

## Mutation results

Each mutant is applied in memory to the adapter and the complete maintained 67-scenario norm suite is run. No production file is replaced. Actual output:

```
MUTATION ownership DETECTED AssertionError
MUTATION foreign matches DETECTED AssertionError
MUTATION first hop only SURVIVED
MUTATION carrier first intermediate SURVIVED
MUTATION routed inheritance removed SURVIVED
MUTATION remove blocking DETECTED AssertionError
MUTATION remove relay powers DETECTED AssertionError
MUTATION remove withdrawal powers DETECTED AssertionError
```

This confirms both disputed round-4 coverage repairs. It also independently confirms three surviving route gaps. I have not rerun every one of the earlier eight route mutants, and do not claim the entire old eight-item inventory remains unchanged after the new restrictions. The document correctly attributes that inventory to the round-4 run. The explicit three-authority and created-child probes in this review cover scenarios missing from the maintained suite.

## Document and test-count audit

All **nine** maintained commands exited zero. The norm suite reports 67 scenarios, of which 19 are rejection checks and 48 reach the solver. Every solver history in the maintained runs and every successful adversarial solver call here had one answer set. Inputs rejected before solving are not zero-answer-set examples. No proof over the compiled program was established by this review.

The connection counts reproduce: 25 ordinary chains; 40 conflict chains; **8** hand-overs late only because of a ban excused; **36** units with their own lateness not excused. The trace-fed gated verdict differs from the fixed-length verdict in **17** of those chains. Gated tests run 300 no-ban chains and 2000 with bans; the helper reports **501** differences, with the comparator qualification in A2. Evidence checks include 400+300 random chains; plain verdict checks include 300; ledger checks include 2000 random histories. The named 11 evidence, 15 verdict and six standalone ledger checks and five connected ledger histories plus the norm scenario agree with the code/output. Clause scenario groups overlap and must not be added as disjoint additional tests.

The “what was wrong in draft 4” table covers both testers' seven list-A entries with merged coverage rows and adds their textual/connection objections. Its named set/edge/cost repairs are real; it is not evidence that all malformed input is now cleanly validated. The verdict really is called. The cycle restriction works. The oracle/count description is corrected. The strongest unresolved qualification is A2: the two advertised amount counterexamples do not both test the newly stated fixed-length rule.

The historical record is correctly transcribed: **Fable 21,15,6,5; Astra 8,6,3,2**. Round 1 counted broader finding inventories, not the later precisely delimited list A; the document now says so. The “new material” figures in round 4 are also accurate: Fable 3/5 and Astra 1/2. No new K9 proof defect was established; rerunning its checks is not a re-proof of the composition. The literature claims were not independently re-audited in this round, and no novelty conclusion is inferred from these runs.

## Stop conditions

1. **Zero or several traces:** not met in this review. All successfully compiled/solved histories had one answer set. The dependency checker remains a necessary partial check, not a uniqueness certificate.
2. **Repair requires changing InstAL semantics:** not met. A1 can be repaired in evidence extraction and boundary validation; A2 in the comparator and its description. Neither identifies a solver-semantics impossibility.
3. **K8 cannot be made without restating norm rules in the verdict:** not demonstrated met, but the positive wording should remain strictly conditional. The code reuses supplied conflict intervals and a delivery controller rather than recomputing norms. That shows a possible interface for the generated class. It does not establish adequacy of the interface for all certified histories, and A1 is a counterexample to its unchecked use. Calling this a universal success would overstate the evidence; calling it an impossibility would also overstate it.
4. **K11 cannot be made by events alone:** not met in the tested sequential form. The maintained ledger connection passes; its explicitly listed asynchronous limitations remain.
5. **Two consecutive same-scope rounds where neither tester's count falls:** not established. Astra is now **8,6,3,2,2**, so this round's count is flat and both current findings concern newly written material. Round 4 fell for both testers. This does not supply two consecutive qualifying rounds, regardless of the other round-5 result. Scope changes still prevent these raw counts from measuring controlled convergence.

CONFIRMED WRONG OUTCOMES (list A): 2
OF WHICH IN NEW MATERIAL: 2
CRITICAL: 0  HIGH: 1  MEDIUM: 1  LOW: 0

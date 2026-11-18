# Adversarial review of composition contract, draft 4

Reproduce with `python3 -B -u /root/mic/model/attack_contract_astra_r4.py`. The [script](/root/mic/model/attack_contract_astra_r4.py) contains the maintained-suite runs, route attacks, explicit timed histories, validation probes, historical-count audit, zero-cost replay, and in-memory mutations. No existing source was edited. InstAL generates its normal temporary compiler/solver inputs. Tests use all answer sets, not the first answer set only.

Counting follows Astra rounds 2 and 3: one item per independent root defect, including reproducible false statements about coverage; variants and mutations are not additional items. There are **two list-A items: one in new K12 material, one in the older K0 coverage claim**. No wrong suspension outcome was confirmed. The unimplemented verdict connection is kept in B, rather than treated as an impossibility proof or another wrong outcome.

## A. Confirmed wrong outcomes or false statements

### A1 — CRITICAL — K12/K0: a repeated authority bypasses a blocked hop

**Runnable scenario:** `routes()`, label `route cycle blocked b`. The adapter accepts `routes={'p':['a','b','a','c']}` and `blocked={'b':['p']}`. Authority a originates prohibition p; b supplies duty o; c has the final edge to unit. Effect e meets both norms. Events are `issue(o), issue(p), request(e), withdraw(p), request(e)`. There is no directly observed relayed event and no origin-to-unit edge.

**Copied real output:**

```
route cycle blocked b models 1
1 ['occurred(issued(o),b)']
2 ['occurred(issued(p),a)', 'occurred(issued(p),c)']
3 ['occurred(refuse(e),unit)']
4 ['occurred(relaywithdrawn(p),b)', 'occurred(relaywithdrawn(p),c)', 'occurred(withdrawn(p),a)', 'occurred(withdrawn(p),c)']
5 ['occurred(emit(e),unit)']
```

**Correct outcome and reason:** reject this route as invalid, or enforce its hop order. If accepted as a route, b's refusal must stop p, and the first request must emit. K12 says the norm takes effect below only if the intermediate passes it on. Here b never issues p, but c does and the unit refuses. The route data format does not require distinct authorities, and the adapter validates this input successfully. This is not the expressly disclosed global-relay injection limit.

**Cause:** the adapter compiles a route into a set of authority-to-authority powers. Both a→b and a→c are powered for the same `issued(p)` event. The origin's initial issue therefore also fires the last hop; nothing distinguishes the first visit to a from the second visit. InstAL correctly executes the graph it was given. The adapter has lost the order required by the route.

**Minimal repair:** require a simple route: nonempty list, at least two entries, and `len(set(path)) == len(path)`, with a useful validation error. If repeated authorities are intended to be supported, use hop-specific relay events and authorize progression only from the preceding hop. Add this blocked-cycle scenario to the maintained suite. CRITICAL follows the requested definition: the clause's explicit relay guarantee is false on accepted input. This does not falsify InstAL's semantics or uniqueness.

### A2 — MEDIUM — K0: two of the six claimed coverage repairs still miss their mutations

**Runnable scenarios:** `mutations()`, especially `all units own all effects` and `foreign matches`. These reproduce Fable round-3 M1 and M6, respectively. Changes are compiled in memory; original files remain intact.

**Copied real output:**

```
MUTATION all units own all effects SURVIVED
MUTATION missing owner accepted SystemExit
MUTATION missing target accepted SystemExit
MUTATION equal rank AssertionError
MUTATION exemption inheritance AssertionError
MUTATION foreign matches SURVIVED
```

The first mutation replaces `if owner[e] != u:` with `if False:`. The second changes `matches(e, n) and owner[e] == u` to `matches(e, n)`. Both pass the 57-scenario norm suite. The script also runs the end-to-end and connected-ledger suites on these survivors.

**Separating scenario and copied output:** two units; duty o in unit matches any effect; its owned effect e has tag local, and authority b prohibits that tag. The other unit v owns f with tag foreign. The duty cannot be met using f. Events: issue o, issue p, deadline o, request f.

```
MUTANT SEPARATING foreign matches models 1
3 ['occurred(missed(o),unit)']
4 ['occurred(refuse(f),v)']
ORIGINAL SEPARATING foreign matches models 1
3 ['occurred(conflict(o),unit)']
4 ['occurred(refuse(f),v)']
```

For the ownership mutation the same scenario emits an extra refusal in the non-owner:

```
MUTANT SEPARATING all units own all effects models 1
4 ['occurred(refuse(f),unit)', 'occurred(refuse(f),v)']
ORIGINAL SEPARATING all units own all effects models 1
4 ['occurred(refuse(f),v)']
```

**Correct outcome and reason:** the maintained coverage scenarios should fail these changes if they cover the six specific faults claimed repaired. The unmutated implementation produces the correct outcomes above. This is a coverage-statement defect, not a claim that the installed adapter currently misassigns ownership.

K0 appropriately drops the earlier universal coverage claim. However, it specifically says the six undetected changes now each have a scenario and calls that evidence of coverage “for those lines.” Two newly supplied scenarios do not distinguish the faulty lines. The new foreign-effect scenario prohibits both units' disclosure effects, so adding the foreign effect to the duty's matches does not create an open way. The ownership scenario checks emission isolation but overlooks a spurious refusal.

**Minimal repair:** use different prohibition-matching attributes for the owned and foreign effects; assert that a non-owner produces neither emit nor refuse for another unit's request. Alternatively state that four of the six mutations are detected and two remain uncovered. Counted once, as in prior reviews.

## B. Design objections and claims stronger than their evidence

### K8: an independent norm-layer check, but no executable verdict connection

The maintained test passes unchanged:

```
ok   30 random chains with bans: the units recorded as missing are exactly those late on their own account; 24 hand-overs delayed only by a ban were not recorded as missed
```

Its imports, extracted by `audits()`, are:

```
SUSPENSION IMPORTS ['import random', 'from fractions import Fraction as F', 'from harness import run']
```

Neither `verdict.py` nor `evidence.py` is called. The “verdict layer” is this local dictionary:

```
residual = {u: spec[u]['receipt'] - spec[u]['due'] - suspended[u] for u in units}
```

Thus no actual verdict function receives a suspension length, and no Shapley shares, aggregate loss, missing-evidence handling, or two-duty response/receipt connection are tested with suspension. Calling this a test of the executable verdict connection overstates its evidence. A residual-only surrogate can reasonably represent the intended boundary, so I classify this as B rather than manufacture a second wrong outcome from the missing call.

The oracle is independent of InstAL's conflict trace: it subtracts physical-ban overlap from independently supplied times. The interval reader instead uses `holdsat(conflicted(...))`. That separation is real and can catch broken norm encoding. It is not independent of the test's arithmetic conventions: both use the same generated ready/receipt times, and the same `overlap` helper. Agreement of the local residual's sign is consequently much weaker than validating the complete verdict implementation. The test compares **signs**, not numerical residual equality.

An explicit illustration is `postdeadline ban`: ready 0, due 4, receipt 9, ban [5,8). The K8 reader correctly grants no suspension because the ban starts after expiry; its residual is 5. The physical-time oracle's residual would be 2. Both classify the unit as late, so the maintained comparison cannot distinguish those amounts. This is consistent with the stated deadline rule, not a new wrong outcome. Before connecting loss sharing, decide which amount the verdict is meant to use and test it.

The test performs two complete offline runs: first it learns conflict intervals from the whole history, then it inserts shifted deadlines and reruns. This is not an implemented online timer. An unclosed conflict leaves `start` outstanding and is not added to `total`; there is no completed receipt history in the maintained generator to exercise that case. Do not infer online scheduling correctness from the finite closed-ban tests.

### Timing probes and tie audit

`timings()` copies the maintained trace reader and supplies explicit histories; its separate oracle computes the union of forbidden intervals, clipped at readiness, receipt, and any exemption. These are the executed results:

| History | Ready / due / receipt | Ban(s) | Suspension | Shifted deadline | InstAL miss / oracle late |
|---|---|---|---:|---:|---|
| Starts before ready | 5 / 9 / 10 | [1,7) | 2 | 11 | false / false |
| Ends at deadline | 0 / 4 / 5 | [1,4) | 3 | 7 | false / false |
| Starts at deadline | 0 / 4 / 7 | [4,6) | 2 | 6 | true / true |
| Covers whole window | 0 / 4 / 7 | [0,6) | 6 | 10 | false / false |
| Late input | 8 / 12 / 14 | [3,10) | 2 | 14 | false / false |
| Consecutive issue/withdraw, no intervening event | 0 / 4 / 6 | [1,3) | 2 | 6 | false / false |
| Zero duration | 0 / 4 / 5 | [2,2) | 0 | 4 | true / true |
| Two overlapping bans | 0 / 4 / 8 | [1,5), [3,7) | 6 | 10 | false / false |
| Two separated bans | 0 / 4 / 8 | [1,3), [4,6) | 4 | 8 | false / false |
| Exemption while ban remains issued | 0 / 4 / 5 | [1,9), exemption at 3 | 2 | 6 | false / false |

All have one answer set. Trace state i is paired with the time of the event that produced that state. Initial duty issues are assigned time zero; readiness clipping prevents pre-input suspension from being charged. Sorted tuples put authority acts before requests before deadlines, with the event string as a further tie-breaker. A performance at the shifted deadline therefore discharges first. An exemption ends the conflict interval at issuance, not at withdrawal of the underlying ban. The tested overlapping and returning conflicts are extra probes, not evidence that the document already promised those cases.

### Other route and adapter observations

- **Ordinary routes work through InstAL:** a→b→c refuses while p is issued, then emits after the origin withdraws p. Blocking either b or c prevents the initial prohibition. A created child inherits the routed prohibition and loses it on withdrawal. All these solver runs have one answer set. The adapter supplies powers and static `mayrelay` facts; the observed issued/withdrawn events are generated by InstAL, not written as outcomes by the adapter.
- **Route/direct-edge precedence is unspecified:** adding an a→unit edge while retaining a blocked route still leaves p ineffective. `carrier[n]` selects exactly one carrier, so the extra direct edge does not supply an alternate delivery path. This is a modeling choice needing documentation, not a reproduced violation of K3's necessary-power condition.
- **Same-origin exemption precedence is ambiguous:** a originates p and routes its own exemption through low-ranked c. The request emits despite c being below a. K4 explicitly permits the prohibition's own issuer to exempt it, while K12 speaks of the lowest route rank. The code preserves the same-issuer exception. Clarify that exception rather than count an outcome permitted by K4 as an independent A.
- **Validation remains incomplete:** an empty route raises `IndexError`; a self-hop raises `InstalBridgeTypeError`; a routed exemption through an authority with no rank raises `KeyError`. A fractional cost is rejected as “not a number,” while NaN is accepted and treated as requiring no funds. Define a finite, nonnegative numeric domain and validate all route entries/ranks before file generation. These newly probed malformed/domain-edge inputs are B; the repeated-route authorization failure itself is counted in A1.
- The new route mutations removing blocking, ignoring route rank, removing withdrawal-hop power, removing relay facts, and removing issuance-hop power all fail a maintained assertion. This is positive evidence for the five new route scenarios, not a certificate for arbitrary routes.

## C. Limits already stated, not counted again

- **K4's unreachable-way wrong outcome remains reproducible.** `disclosed_open()` starts with officer, a duty to amend to board, a declared vote→board amendment, and an authority prohibition on the officer→vote step. Output:

  ```
  3 ['occurred(refuse(step),unit)']
  4 ['occurred(refuse(finish),unit)']
  5 ['occurred(missed(o),unit)']
  ```

  No reachable way exists, but the declared unreachable effect prevents conflict. This is precisely the expressly open wrong outcome, not another A.
- **K12's directly observed relay vulnerability:** with b blocked, observing `relayed(p)` produces `issued(p)` at c and refusal at the unit. With an ordinary active route, observing `relaywithdrawn(p)` ends the prohibition without `withdrawn(p)` at origin a, and the request emits. Global observations do not identify a sending authority. This is the admitted trusted-log boundary, unlike A1's bypass using only normal issue/withdraw events. Directly observing institutional `issued(p)` or `withdrawn(p)` did not inject their institutional effects.
- Differing per-unit orders/clocks, dynamic creation, creator-owned child rules, subtree caps, self-declared attributes on exemptions, own-rule repeal, translation and leased context, recurring commitments, trees in evidence, faulty evidence, and delayed/replayed ledger grants remain listed exclusions.
- Multiple overlapping or returning conflicts remain disclosed as untested by the maintained suite, even though the additional finite probes here pass.

## Every round-3 list-A item

| Tester / item | Disposition in draft 4 | Evidence |
|---|---|---|
| Astra A1 zero-cost connected executor | Fixed | `play(100,[('free',0),('paid',60)])` returns both effects; ledger really used is 60; maintained connected test asserts it. |
| Astra A2 adapter coverage claim | Partly fixed | Equal-rank and inherited-exemption mutations now fail; universal claim narrowed. Specific six-repair claim still fails current A2. |
| Astra A3 single-unit owner default text | Fixed | K0 expressly defaults a sole unit; mandatory ownership limited to multiple units. |
| Fable A1 unreachable way | Moved to stated limit/open wrong outcome | Current C reproduces it; not repaired. |
| Fable A2 no post-conflict deadline | Partly fixed | A suspension rule now exists and the finite offline reader schedules shifted deadlines correctly in these probes. No production evidence/timer connection or actual suspended verdict call exists. |
| Fable A3 malformed adapter inputs | Fixed for the reported cases | Unknown kind, invalid edge, invalid amendment procedures and invalid norm `against` are rejected by maintained tests; string cost rejected by the new probe; unit/authority overlap and invalid created-unit references have explicit assertions. New route/domain validation gaps are separately described above. |
| Fable A4 coverage statement | Partly fixed | Four of six mutations killed, two survive; same current A2, not counted twice. |
| Fable A5 understated historical record | Fixed | Opening and stop-condition record now use 21/15/6 and 8/6/3; text distinguishes repaired, withdrawn, excluded and open. |
| Fable A6 conflict-literature statement | Fixed as a text correction | Draft 4 now explicitly says later work resolves conflicts by revising institutions, matching the correction requested in round 3. No new literature-equivalence claim is established by this audit. |

## Document and test-count audit

All eight maintained commands exited zero. The norm suite reports **57 scenarios**, including 12 rejection checks and 45 solver histories. The derived-fluent checker reports 14 fluents and 15 dependencies without a cycle. The connection suites report 25 ordinary chains and 30 banned chains; the latter reproduces 24 hand-overs delayed only by bans. Evidence has 11 checks including 400+300 random chains; verdict has 15 named cases plus 300 random chains; ledger has six named checks plus 2000 random histories. There are five connected ledger histories plus the separate norm grant/free-effect scenario, consistent with K11's wording.

K3's five, K4's twenty, K5's three, K6's four, K7's seven and K12's five match the scenario groupings, including additional coverage cases counted under their relevant clauses. These are coverage groupings, not disjoint solver-run totals. The six mutation-repair coverage claim is the exception discussed in A2.

The “what was wrong in draft 3” table accounts for the nine prior list-A entries in eight rows because the coverage findings are merged. Its text/default, zero-cost, malformed-input, historical-record and literature corrections are accurate for their named cases. The coverage repair is incomplete; the post-conflict scheduling row establishes a tested rule and offline calculation, not a deployed composition of evidence and verdict. The explicitly open K4 row is honest.

The record **21, 15, 6 and 8, 6, 3** agrees with the earlier report counts; the first-round reports use finding inventories rather than the later exact `CONFIRMED WRONG OUTCOMES` footer. The script prints the later numerical footers. Falling raw counts are not controlled convergence evidence because exclusions and semantics changed, but the recorded numbers themselves are correct. No new proof defect in K9 was found; this review reran its checks rather than claiming an independent proof of the complete system.

## Stop conditions

| Condition | Verdict |
|---|---|
| 1: no trace or several | Not triggered by a completed solver run here. Every original-encoding probe that reached solving had exactly one answer set, including cyclic cross-generation. Parser/validation exceptions are not zero-answer-set runs. The unit-only dependency checker still cannot prove this universally. |
| 2: repair requires changing InstAL semantics | Not triggered. Rejecting repeated authorities or introducing hop-specific protocol events repairs A1 without changing InstAL. A2 needs separating assertions or narrower text. |
| 3: K8 requires restating norm rules inside verdict | Not demonstrated triggered, but draft 4's “not met on the one hard case tried” must be read narrowly. The norm/oracle comparison passes; the actual verdict implementation was not tried with suspension. A duration-only boundary is plausible, and no impossibility was shown. Integration and loss-sharing semantics remain unestablished. |
| 4: K11 cannot work by events alone | Not demonstrated. The synchronous connection passes, including the repaired free purchase. Its admitted asynchronous limits remain. |
| 5: two consecutive same-scope rounds with neither tester's count falling | Not triggered by this review. Astra's full count falls from 3 to 2; the old-material subtotal is 1, with 1 additional defect in K12. The other tester's current result is not assumed. Previously recorded counts are accurate, with scope caveats. |

CONFIRMED WRONG OUTCOMES (list A): 2
OF WHICH IN NEW MATERIAL: 1
CRITICAL: 1  HIGH: 0  MEDIUM: 1  LOW: 0

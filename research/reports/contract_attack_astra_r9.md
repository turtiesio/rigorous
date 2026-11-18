# Draft 9 adversarial review — Astra

No confirmed wrong outcome or false statement inside the stated scope was found. This is a bounded negative search, not a proof of the composition contract. The strongest objection concerns the proposed interpretation of stop condition 3, below.

Run [attack_contract_astra_r9.py](/root/mic/model/attack_contract_astra_r9.py) with `python3 -B -u /root/mic/model/attack_contract_astra_r9.py`. Individual phases can be named on the command line. Mutations are in memory; InstAL uses its normal temporary compilation workspace. Only this report and that script are persistent review outputs. No existing source was edited, no subagents were started, and the other tester's round-9 report was not read. Earlier route and adapter probes are reused explicitly through the round-8 script; the new two-way controller is independent of the maintained test's `play`, `attempts`, and `history` helpers.

## A. Confirmed wrong outcomes or false statements inside scope

None. Counts use the same root-defect convention as Astra rounds 2–8, including reproducible false documentary statements. A limit or a design objection has not been promoted to make this count nonzero.

Material split: **0 new, 0 older**. Kind split: **0 wrong model outcomes, 0 false documentary statements**.

## B. Design objections and claims stronger than their evidence

### B1. Stop condition 3: failure of fixed intervals is not impossibility of composition

The strongest probe is `ties`: an authority starts a three-day prohibition one day after the upstream delivery. Actual upstream work is 3 against budget 2; the downstream budget and work are both 1. Actual ban: [4,7); actual final delivery: 7. When the upstream unit complies, the same authority policy produces [3,6), and InstAL delivers at 6. Replaying only the actual interval predicts 3.

Copied output:

```
CONTINGENT fixed actual facts counterfactual 3 policy replay InstAL 6
```

The two real InstAL runs emit `e0` at 3 and `road` at 7, and `e0` at 2 and `road` at 6 respectively. Both have one answer set. The script supplies the contingent authority acts explicitly; it does **not** claim that the existing institution automatically implements this policy.

This supports the stated restriction to exogenous prohibition times. It does not establish that K8 *cannot* be made without restating norm rules in the verdict: calling the norm layer on each counterfactual could keep those rules in InstAL, or the interface could export a conditional model. The proposed example establishes inadequacy of a history-invariant interval interface, not an impossibility theorem about every interface. Suggested wording: “would defeat the fixed-interval construction; whether it meets the stronger stop condition remains open.” This is a design/evidence objection, not an additional in-scope wrong outcome. The contingent-ban failure itself is C.

### B2. `noway` is institution-relative and does not mean generic inability to deliver

`fluent` runs the following real traces:

| Probe | Derived result |
|---|---|
| Ordinary authority ban matching both ways | `holdsat(noway(o),unit)` after issue and through both refusals; absent after withdrawal |
| Watched ban | No `noway`; `occurred(emit(road),unit)` and `occurred(breach(road,p),unit)` |
| Unit's own prohibition | No `noway`, although both requests are refused |
| Effective exemption | `noway` holds after ban issue and disappears after exemption issue; road emits |
| Duty with no declared matching way | Adapter rejects: `obligation o has no declared way its target can take` |
| Duty targeted at `unit`, with another institution `v` owning no matching effect | `holdsat(noway(o),v)` persists even after the actual target's ban is withdrawn |

The last case follows from the adapter putting `isoblige` in every unit, with matches filtered by local ownership. Universal absence of a local way is vacuously true in the foreign institution. The current consumer selects `noway(recv_u)` in institution `u`, so this does not produce a wrong verdict. Guard the fluent with duty ownership, or document that only the target's projection is meaningful. It is not evidence that InstAL confuses which unit owes the duty.

Likewise, permission, procedure reachability, funds, own bans, and completion can prevent emission without `noway`. Within the stated two-way controller and its restrictions, the exported interval has the needed meaning. “Cannot deliver” should retain those qualifications. Funds and exemptions are explicitly untried in this composition, own prohibitions are deliberately excluded from authority conflict, and reachability is the declared K4 defect.

### B3. Passing tests do not establish an interface theorem

The conjunction really is evaluated by InstAL. `someway` derives an existential open authority-unforbidden match; `noway` negates that result. The verdict contains an interval lookup, not the per-effect prohibition or exemption rules. Adding an exported predicate whose only consumer is the verdict is coupling through an interface, as the text acknowledges.

The independent run strengthens the example evidence but does not prove arbitrary histories sound. Equal-time ordering remains a convention; log coverage checks presence of `(time, unit)` requests, not authenticity, multiplicity, or which effect caused which refusal. Unknown kinds are not a typed interface. These are validation/design concerns, not a demonstrated accepted, correctly certified in-scope history with a wrong verdict.

The stop-condition explanation says the verdict “needs two things” while the API additionally requires chain parameters and the certified log. In context those two things are the norm/work information relevant to the restatement question. It should not be read as an exhaustive API specification; the earlier review already treated this wording as B.

## C. Limits already stated, not additional findings

- The contingent-ban case above changes authority times with conduct and is explicitly outside K8.
- K4's unreachable amendment remains wrong and disclosed. The rerun prints `occurred(refuse(step),unit)`, `occurred(refuse(finish),unit)`, then `occurred(missed(o),unit)`. Nothing in `noway` repairs feasibility.
- K8's declared counterfactual work/after-time conventions, claimant-borne prohibition loss, instantaneous attempts at readiness, refusals without reasons, and waiting without attempting remain conventions or stated limitations. Negative shares with several bans, recurrent bans, trees, two duties per hand-over, heterogeneous ways, funds and exemptions in the two-way composition, faulty evidence, and differing clocks are not promoted into A.
- The test still places deadlines from a finished history. No online scheduler has been implemented. K1's common global sequence is not a test of differing local orders or bounded clock disagreement.
- K12's directly observed relayed issue and withdrawal still bypass the intended provenance. The rerun shows `relayed(p)` leading to `occurred(issued(p),c)` and a refusal; a directly supplied `relaywithdrawn(p)` produces withdrawal and emission. The text explicitly puts this in the log-trust limit. Static relay consent, rank asymmetry, translation, and leased context remain open.
- K2's dependency check is partial, F2 is assumed, and K11's sequential grant tests do not establish correctness with late, repeated or reordered grants. K0's documented malformed shapes and mutation-coverage gaps remain limits.

## Independent two-way attack and trace comparison

`independent` builds 36 two-unit chains, including zero budgets and half-day ties. Each duty has road and rail and one independently sampled issue/withdrawal interval per way. It constructs requests directly by trying road, then rail, and retrying at the earliest currently active ban's end. It does not use the maintained generator or `gated.gate` to construct histories. For every coalition it constructs and solves the counterfactual history, including duty issue at readiness.

Copied output:

```
INDEPENDENT 36 two-unit chains, coalitions 99 intervals / missed / rerun all agree
UNCHANGED 74 scenarios (25 rejections), 49 successful complete traces identical after projection
K2 tracked runs 269 answer set counts [1]
```

The trace comparison runs the entire 74-scenario suite twice. The second pass removes only the two deriving rules in memory. Every successful trace's occurred events and all other held fluents agree at every step; only `someway` and `noway` are projected out. Rejections still pass. Thus “nothing else in the institution uses the new fluent” and “the other 74 scenarios are unchanged” are supported by both source inspection and execution.

No empty or multiple answer set was encountered. Rejected malformed input is an adapter rejection, not a solver trace count of zero.

`ties` independently verifies the repaired zero-budget boundary:

```
TIE bad rejected at time 0 a deadline precedes an act of an authority
```

Issue, then deadline, then delivery produces `occurred(missed(o),unit)`. A costed request/refusal, grant, request/emission at one certified time is accepted. I found no valid log within the literal K1 conventions wrongly rejected, nor a wrong lateness reading within them.

## Every round-8 list-A item, both testers

| Prior item | Status in draft 9 | Reproduction / repair check |
|---|---|---|
| Astra A1, HIGH: deadline before same-time duty issue | Fixed by narrowing the permitted order and enforcing it | Independent bad-order rejection and good-order miss above. Undoing the deadline branch fails the maintained test. |
| Astra A2, LOW: round-7 new-material attribution | Fixed | Earlier reports say Astra 0 new of 2 and Fable 2 new of 3; draft now says 2 of 5, not 4. |
| Astra A3, LOW: alternative charter count five rather than four | Fixed | Fable's round-7 alternative is four; draft now explicitly says four, not five. |
| Fable A1, LOW: same round-7 material-age attribution | Fixed; same root as Astra A2 | Same documentary audit. Not counted twice as an independent defect. |

None is partly fixed, unfixed, or no longer applicable. The ordering repair excludes the old wrong history explicitly, rather than making InstAL interpret that history differently.

Mutation output:

```
MUTATION deadline ordering [('test_gated.py', False, "SystemExit: a deadline recorded before an authority's act of the same time: accepted")]
MUTATION coverage call [('test_gated.py', False, 'SystemExit: a log that leaves out an attempt the evidence holds: accepted')]
MUTATION coverage check [('test_gated.py', False, 'SystemExit: a log that leaves out an attempt the evidence holds: accepted')]
```

Deleting the `noway` rule or replacing it by `noway(O) when conflicted(O)` both fail the first named two-way case: actual closed intervals `[]`, expected `[(3,4)]`. That case delivers before the later conjunction; duty-conflict intervals therefore disappear, but the exported obstacle must persist independently of discharge as well as issue.

The two numerical prose repairs are separately undone in memory by `prose`; the maintained arithmetic test remains green. These textual repairs are checked against reports, not pinned by model tests. The grant classification and log/evidence coverage were round-8 B objections, not extra A items; both are now addressed, with coverage pinned as above.

## Documentary and numerical audit

All ten maintained commands listed in draft 9 completed successfully across the executed phases. The standalone two-way run completed; a redundant second invocation was stopped. The evidence, plain verdict, gated verdict, connected ledger and standalone ledger checks all passed. The gated arithmetic reproduced 563 and 746 fixed-length disagreements, one negative-share case in the two-ban corpus and none in the one-ban corpus.

| Claim | Evidence |
|---|---|
| 74 scenarios, 25 rejections | Suite execution; also 49 successful full-trace comparisons. The adapter audit prints each of the 25 rejection reasons. |
| 5 named one-way cases, 40 random chains | Maintained conflict suite passes; includes the zero-budget issue/deadline tie. |
| 61 coalitions, 51 changed histories | Exact maintained one-way random-corpus output. |
| 16 derived fluents | Dependency checker: `16 non-inertial fluents, 17 dependencies, no cycle`. |
| 3 named two-way cases, 24 random chains | Maintained two-way test passes all; independent controller tested separately. |
| 63 coalitions, 37 changed histories | Exact maintained output; random corpus only, not the named cases. |
| 155 and 401 independent coalitions, hence 556 | Earlier Fable reports; 155 + 401 = 556. These are historical claims, not 556 new runs in this review. |
| 840 further solver runs | Round-8 Fable report says `840 runs: min 1, max 1`. |
| 125260 small ordered logs | Round-8 Fable report's ENUM output; no assertion that this enumeration covers the new deadline ordering. |
| Historical defect counts | Both eight-round sequences match the reports. The draft does not swap their names: it uses “one” and “the other.” |

The new status paragraph counts three *reported false-statement findings*, two concerning the same sentence, alongside one model outcome. There are only two distinct false-statement roots. Its explicit duplicate qualification makes this a defensible account of four report entries, not a claim of three independent documentary defects. The new draft-8 correction table correctly attributes the deadline defect to Astra, the material-age attribution to both, and the alternative count to Astra. It labels the coverage/grant and duty-arrival issues as objections. The new K1 ordering, scope exception for arrival-triggered duty issue, and the two-way paragraph match their implementations. The qualification that the scope is not an online deadline implementation remains necessary.

The conclusion about convergence through round 8 is correct: rounds 5 and 7 each had neither count falling, but they were not consecutive. At least one count fell in rounds 6 and 8. Round 9 also changes scope by adding two ways, so it should not be silently treated as an identical-scope convergence experiment. My A count falls from 3 to 0 regardless.

## Stop conditions

1. **No trace or several:** not met in these runs. The partial stratification argument is not upgraded to a proof.
2. **Repair requiring changed InstAL semantics:** not met. No new in-scope wrong outcome requiring repair was found.
3. **Composition requires restating norms in the verdict:** not met on the two tested constructions; open generally. The current conjunction export works. The contingent-ban example defeats fixed intervals, but the proposed criterion needs the qualification in B1 before it can establish the stronger impossibility claim.
4. **Ledger cannot be composed by events alone:** not met in the tested sequential form. The disclosed distributed-grant questions remain open.
5. **Two consecutive rounds where neither count falls:** not met through round 8, and not triggered by my round-9 count. The other round-9 result is deliberately unknown to this review. No claim of convergence to zero follows.

CONFIRMED WRONG OUTCOMES (list A): 0
OF WHICH IN NEW MATERIAL: 0
CRITICAL: 0  HIGH: 0  MEDIUM: 0  LOW: 0

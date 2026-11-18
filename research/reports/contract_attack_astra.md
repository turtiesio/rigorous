# Adversarial review of the composition contract

Reviewed 2026-11-17. The current composition is not ready to claim the general guarantees of K6 or K11. Two admitted, static scenarios break those guarantees. The failures can be repaired in the encoding and adapter; I found no evidence that InstAL itself must change.

Reproduce all attacks and controls with:

```sh
cd /root/mic
python3 -B model/attack_contract_astra.py
```

The [attack script](/root/mic/model/attack_contract_astra.py) contains the complete data for every new scenario below, prints actual solver atoms and evidence results, and labels counted findings `DEFECT`. Existing files were not edited. The harness necessarily generates temporary compiler inputs as in its normal operation. I ran all seven named test programs plus the stratification checker: all passed, including 27 norm scenarios, 25 end-to-end chains, 400 evidence chains, 300 verdict chains and 2,000 ledger histories. Every new InstAL scenario also returned exactly one answer set. Passing tests do not invalidate the counterexamples below.

## Confirmed findings, most damaging first

### D2 — CRITICAL — K6: inheritance stops at one generation

**Encoding defect; violates a stated guarantee.** This uses three institutions fixed before the run, not dynamic creation or a cap over a subtree. Authority `a` prohibits amendment in `unit`; `unit` creates `v`, and `v` creates `w`. Effect `x` amends `w` from `officer` to `vote`. Events are `issue(p), request(x)`. Full input is the script's `D2` case.

Actual output:

```text
D2 grandchild escapes inherited prohibition DEFECT answer_sets= 1
1 occurred(issued(p),a)
2 occurred(emit(x),w) occurred(refuse(x),unit) occurred(refuse(x),v)
```

Under K6, `v` has `a` as an authority by inheritance, so `w` must receive that authority's prohibition as well and refuse `x`. The harness tests only membership in `created[target]`, where `target` remains the norm's original target. It never closes the creator relation transitively. `w` receives no corresponding bridge power.

**Minimal repair:** compute the static descendant closure for each target, copying permission/prohibition powers through every generation, or explicitly restrict K6 to one generation. Validate creation cycles. Do not copy obligations. The exclusion of “subtree limits” concerns aggregate limits, not this per-unit inherited prohibition.

### D3 — CRITICAL — K11, K1: one grant authorizes two emissions

**Encoding/interface defect; violates single-use resource authorization.** One ledger reserves 10 rights for `x` under cap 10. `unit` and its statically created child `v` inherit a purchase permission. The effect is `{'kind':'buy','cost':10}`; this is accepted input, with no required owner field. Events: `issue(p), grant(x), request(x)`.

```text
D3 one grant spent in two institutions DEFECT answer_sets= 1
1 occurred(issued(p),a)
3 occurred(emit(x),unit) occurred(emit(x),v)
D3 ledger accepts first/second consumption DEFECT [True, False]
```

The same global observation initiates `funded(x)` in both institutions. Both spend it. The ledger can cover only one emission; rejecting the second `use` after the trace is too late. There is one reservation, one ledger and no late or duplicate grant. This is not the explicitly excluded several-shard or replay case.

**Correct outcome:** at most one controller emits against this reservation. **Minimal repair:** require a unique effect owner and enforce that owner for requests, grants, permission matches and emission; preferably include a reservation ID and recipient in the event. Reject ambiguous inputs. Merely checking the ledger after all emissions does not gate the effects. Single-event serialization in K1 does not imply a single institution consumes the event.

### D1 — HIGH — K3: the documented charter input is ignored

**Harness/API defect.** `run` documents `power={origin: set of norm ids it may issue}`. Give `power={'a':set()}`, an ordinary permit `p` from `a`, and `issue(p), request(x)`.

```text
D1 ignored charter power input DEFECT answer_sets= 1
1 occurred(issued(p),a)
2 occurred(emit(x),unit)
```

The charter input says `a` may issue nothing. It should not issue `p` successfully; the effect should lack permission. Instead the harness automatically writes `mayissue(p)` unless the norm carries `ultra_vires=True`; it never reads `power`.

**Minimal repair:** implement the documented mapping and reject absent/contradictory charter data, or remove that API and explicitly document the actual input. This does not refute bridge semantics. My absent-edge control derived `refuse(x)`, and adding authority `b` did not derive `issued(p)` in `b`. InstAL checks the powers it is given; it does not independently establish that the harness supplied the correct charter.

### D7 — MEDIUM — K2: dependency checker is not a whole-program certificate

**Evidence/claim defect, not a reproduced non-unique trace.** The script executes the unmodified checker using an in-memory replacement for its input stream. It changes no source file. It supplies the original source, then the same rules with a newline immediately before `when`, then the source with an extra conditional `generates` rule.

```text
K2 checker actual source as specified 19 non-inertial fluents, 27 dependencies, no cycle
D7 checker loses multiline dependencies DEFECT 19 non-inertial fluents, 0 dependencies, no cycle
D7 checker ignores generates conditions DEFECT 19 non-inertial fluents, 27 dependencies, no cycle
```

The regex requires the literal ` when `; bodies may span lines, but that keyword boundary cannot. The checker never parses `generates ... if`, initiation, termination, bridge rules, or the compiled prelude. The appended rule is a coverage probe, not a claim that it produces several models. Do not count it as stop condition 1.

**Correct outcome:** either parse all dependency-bearing constructs or explicitly say this checks only a formatting-sensitive subset of source fluent rules. **Minimal repair:** use InstAL's AST or analyze the grounded program's signed dependencies. Separately verify event ordering and the transition fragment.

I inspected the compiler's `generates`, `initiates`, `terminates`, and bridge translation and the actual solver prelude in [InstalModel.py](/root/mic/vendor/src/instal-stable-master/instal/models/InstalModel.py). Generated events are guarded by non-occurrence of the source event's violation. Bridge initiation checks `ipow`. Inertia and local/cross initiation affect the next step. Simultaneous initiation and termination deterministically retain the initiated fluent: the self-replacement amendment control retained `proc(officer)`. Observations are broadcast to institutions. None of these dependencies is established by `check_stratified.py`.

For this fixed encoding, static permissions on generating events, fixed observations, acyclic derived fluents and forward-time updates give a plausible stronger uniqueness argument. I found no counterexample to uniqueness. Fluent acyclicity **alone** does not prove all those premises. The contract appropriately calls the formal-semantic proof open, but the checker must not be used as a general uniqueness certificate. The prose also says eighteen fluents; the run has nineteen (not counted separately).

### D8 — MEDIUM — K8: the test supplies receipts for refused effects

**Claim/evidence defect, not a counterexample to the narrow lateness equality.** Independent times: start 0; bounds 2 each; first receipt 3, second receipt 4. The first unit is late; the second responds early relative to its actual late input. This gives the ordered suffix `deadline(o_unit), request(h_unit), request(h_v), deadline(o_v)`.

```text
3 occurred(missed(o_unit),unit)
4 occurred(refuse(h_unit),unit) occurred(refuse(h_unit),v)
5 occurred(emit(h_v),v) occurred(refuse(h_v),unit)
K8 expected/derived as specified (['unit'], ['unit'])
```

The first “receipt” is a request that never emits. Nevertheless its time is supplied as the next unit's actual input. A physical execution in which InstAL gates these hand-overs cannot have that certified receipt on the asserted cause. The existing test calls `receipt` on a preconstructed verdict event graph; it does not construct that graph from successful institutional effects. Its `missed == positive residual` comparison reduces to `receipt > ready + bound + allowance`, with a tie rule. It does genuinely test order and deadline handling, but does not validate a closed execution across the interface.

**Minimal repair:** distinguish an external observed receipt from an attempted controlled emission, specify how late delivery remains possible after an obligation expires, and feed the verdict certified actual receipts linked to those events. Rename the current test as a temporal classification test until that connection exists. Independent tie, early-response/late-input, and two-late-unit tests all agreed on classification. An obligation issued after its sole deadline event produces no `missed`; commitment activation and changes need a policy, which the document already excludes. I do not count that as a separate defect.

### D4 — MEDIUM — K11: refusal has no reservation disposition

**Incomplete interface, not cap overspending.** Cap 10; reserve 10 for `x`; issue both a permit and a prohibition; deliver `grant(x)` then `request(x)`.

```text
4 occurred(refuse(x),unit)
D4 next reservation and remaining funded fluent DEFECT (False, ['holdsat(funded(x),unit)', 'holdsat(unfunded(y),unit)'], 10)
```

The second reservation for `y` fails even though no purchase happened. The ledger's live reservation and InstAL's funded flag remain. There is no cancellation/settlement path in the integration. This can be intended pending authorization, but K11 does not specify that interpretation or eventual release. It is not safe simply to release rights in the ledger while leaving the usable grant in InstAL.

**Correct contract:** say whether refusal preserves a pending request; otherwise cancel the grant in InstAL and expire/settle the reservation before reuse. **Minimal repair:** add a correlated cancellation/release exchange and test both retained and cancelled requests. Severity is MEDIUM because K11 states a safety property, not eventual availability.

### D6 — MEDIUM — K9: the old-rule comparator is not the prior executable rule

**Comparison defect.** Chain A, B, C has bounds 1 each, own residuals 1, 0, 1/2, zero link residuals, deadline 3. B's summary is missing; A and C report. Run both the actual earlier `verdict.absorb_missing` implementation and `evidence.old_bar`:

```text
D6 old time-bar comparison DEFECT {'actual_previous_model': {'C': Fraction(1, 2), 'A': Fraction(1, 1)}, 'comparison_old_bar': ({'B': Fraction(3, 2)}, Fraction(3, 2))}
```

`old_bar` merges across either endpoint's missing summary, combining whole unit residuals, then charges the absent unit. `absorb_missing` moves the inbound link residual to the missing receiver; here there is no link lateness to move. Thus the “compliant missing unit is charged” example is not demonstrated against the prior executable model. This does not vindicate that model: the slow-inbound-link anomaly is real.

**Correct outcome:** describe which earlier rule is being compared and distinguish reconstructed whole-lump settlement from the actual old implementation. **Minimal repair:** run the old implementation on matched histories, or state that `old_bar` is a stronger reconstruction. A new benefit must not be established by quietly changing the baseline algorithm.

### D5 — MEDIUM — K9: unresolved zero/negative lumps disappear from the result index

**Encoding/interface incompleteness.** A=0, B=2, C=-2, D=0; only A,D summaries present. B–C is unknown, so they form a lump. Actual result:

```text
{'pays': {}, 'suspense': Fraction(0, 1), 'undetermined': [], 'miss': Fraction(0, 1), 'reporting_failure': ['B', 'C']}
```

`undetermined` is populated only while iterating Shapley players, which excludes nonpositive lump totals. There really is an unresolved B–C boundary, but the returned reopening index says there are no undetermined groups. Reporting-failure names do not identify missing boundaries, especially with acknowledgements lost. The same omission occurs in the document's stronger-claim example even though resolving B–C changes A's payment.

**Correct outcome:** list every unresolved group independently of its present share; a zero suspense amount is fine. **Minimal repair:** construct `undetermined` from all non-singleton groups before allocating money. There is no persistence or reopening implementation to test here; the defect is not proof that some implemented database permanently loses evidence.

## Clause-by-clause verdict and controls

| Clause | Verdict |
|---|---|
| K1 | Input serialization works as an assumption. One observed event broadcasts to several institutions; D3 disproves treating this as globally single consumption. No witness protocol was implemented or tested. |
| K2 | All actual runs unique; proof remains incomplete. D7 limits the checker. No zero/multiple-answer-set counterexample found. |
| K3 | Bridge-power exclusion works; origin filtering works for unique norm IDs. D1 breaks the documented charter API. Sender authentication is absent, not disproved: event strings carry no actor, so a forged-authority history is not representable as such. |
| K4 | Literal static-pattern rules survive ties, simultaneous duties, withdrawal and reissue controls. “Way” versus executable way needs clarification; see below. No complete priority-procedure equivalence established. |
| K5 | Watch-only prohibitions do not block or override duties in existing tests. Without any positive permission, the watched effect is refused under K4; read “goes through” as “is not prevented by this prohibition,” not an unconditional permission. |
| K6 | Fails for a statically created grandchild, D2. Direct-child copying and not copying obligations pass. |
| K7 | Amendment discharge, procedure changes, old-procedure refusal and own norms pass existing and added controls. A same-procedure amendment retains its procedure. No standalone K7 failure found. |
| K8 | Narrow temporal classification passes. D8 limits the claimed end-to-end connection; no need to duplicate priority rules in the verdict has been demonstrated. |
| K9 | Conditional exactness and information-theoretic limitation survive. D5 affects unresolved-group reporting; D6 affects the comparison baseline. Payment incentives and indefinite suspense are explicitly conceded. |
| K10 | Existing tests pass within the well-formed-contract/baseline assumptions inherited from group B. Shapley allocates marginal loss above the compliant baseline; the well-formed deadline is essential. No new falsification found. Missing-evidence settlement is provisional under K9, not continuity in the discrete reporting mask. |
| K11 | Single-institution effect binding and single consumption pass; global double consumption fails (D3). Refused reservations need a lifecycle (D4). |

### Priority rules and published procedures

K4 is not, as written, an implementation of one of Liao et al.'s named **Greedy, Reduction, or Optimization** procedures. Those select normative conclusions; K4 selects allowed effects with pattern-based alternatives and one-shot discharge. The source explicitly restricts its treatment to regulative obligation detachment, whereas this encoding also has permission, prohibition, amendment and time. See [Liao et al., sections 1–2](https://arxiv.org/html/1709.08034v2). The comparison is an inference from the different definitions, not a proof that no embedding is possible.

The script's temporal separator issues a higher obligation and a lower prohibition matching `x`, then requests `x` twice. It derives `emit(x)` at step 3 and `refuse(x)` at step 4. A static selector on the unchanged issued norm set cannot produce that change without adding a discharge state and transition policy. This separates the procedures as standalone systems; it does **not** establish that K4's static snapshot cannot be represented by one of them. That stronger question remains open. The [cited defeasible-permission paper](https://arxiv.org/abs/1212.0079) explicitly includes permissive exceptions to obligations; K4 instead lets prohibitions defeat explicit permissions regardless of their priority. There is no supplied translation establishing equivalence to that logic either.

A further run exposes an ambiguity rather than a counted violation of the literal pattern policy: an obligation to amend, a lower ban on amendment to `vote`, an available officer-to-vote effect, and an unbanned board-to-board effect while `officer` is the current procedure. Both requests are refused and the obligation is missed. `cheaperway` counts the procedurally impossible board effect as an alternative. If “way” means executable now, this is wrong; if it means any declared matching pattern, this is the encoding's specified abstraction. Repair by defining feasibility explicitly before claiming executable alternatives. A costed but unfunded alternative creates the same conceptual issue; cap/obligation conflicts are already excluded.

### Evidence proof, incentives and boundaries

The exactness proof is valid **provided the initial boundary, final receipt, public bounds and allowances are known**. The last receipt is explicitly assigned to the claimant in `evidence.py`; the first boundary is implicitly assumed fixed. Neither endpoint is parameterized as missing. Interior boundary i is known precisely when its receiver reports, or its sender reports and that sender's acknowledgement is not lost. Note that `acks_lost` uses **sender names**, not receiver names. Under these assumptions, no adjacent missing pair plus all acks delivered is sufficient. The proof must state the initial-boundary assumption; the model accepts exact residuals and cannot test timestamp provenance.

The condition is not necessary for a particular verdict: an irrelevant lost ack changes nothing when the receiver reports; zero-loss chains may have the same zero payments despite a missing pair. For universal reconstruction of every interior residual, knowledge of every boundary is the relevant condition. The two histories with B late versus C late give the same observable lump sum. The impossibility argument is sound for rules depending only on those observations; enumeration is only a finite illustration of that argument.

The document's stronger-claim counterexample is correct: full evidence gives A=1/2, B=1/2; B,C missing gives A=1. Missing neighbours can also **benefit** a fully reporting unit: A=B=C=2, slack 2 gives A=4/3 on full evidence and A=1 with B,C missing. Acks need not be lost for that example. With A=B=2, C=0, B missing and A's ack lost, A's payment falls from 2 to 0 and suspense becomes 4. If B reports, that lost ack has no effect. These are reproduced and labelled `as specified`: K9 expressly sacrifices collection rather than charge on unavailable evidence. The API does not establish that a unit can strategically suppress honest witness acknowledgements; it merely models loss. Nor is there an executable eventual-delivery/reopening mechanism: existing evidence can remain unavailable to the adjudicator indefinitely, as the clause already concedes.

### Ledger limits that are not new defects

A grant for `x` cannot fund distinct `y` in one institution. A second request for `x` without another grant is refused. A grant reserved before a cap falls from 10 to 0 can still spend 10: that is the ledger's explicitly tested grandfathering policy, not a current-cap invariant. Replaying `grant(x)` after consumption allows a second emission; duplicate/late grants are explicitly excluded. These controls do not repair D3, which needs only one grant and one global request.

## Claims table, stop conditions and novelty

The tests named in the table really execute and pass. Their counts are historical coverage, not completeness measures: the actual verdict test prints fifteen named cases, and the checker finds nineteen fluent names. The K9 proof is conditional on fixed endpoints; K2's checker is narrower than a uniqueness proof; K8 supplies its own receipts; the old-rule comparison uses a different algorithm. The harness precomputes matching, priorities, ownership, authority membership and inherited bridge powers. This is not generally precomputing the answer: InstAL still resolves current normative state. But those trusted compilation choices must be part of the contract and tested, as D1–D3 show.

| Stop condition | Review verdict |
|---|---|
| 1: zero/multiple traces while checker passes | **Not met.** All actual histories have one answer set. Checker coverage failure alone does not meet it. |
| 2: wrong outcome needs changed InstAL semantics | **Not met.** The failures admit adapter or institution repairs. The language need not change. Existing tests do not automatically establish the “needs semantic change” part. |
| 3: K8 requires restating norm rules in verdict | **Not met.** D8 identifies an event-meaning mismatch, not an impossibility theorem. A separate receipt event and a clear activation policy may suffice. |
| 4: K11 cannot compose using events alone | **Not met.** Correlated owner/reservation events and cancellation acknowledgements remain available. Current integration is incomplete, not shown impossible. |
| 5: more defects than previous round | **Not met by this review alone.** Eight counted findings versus eight prior reported defects; only three here are HIGH/CRITICAL guarantee/API failures. Do not compare every control or acknowledged limitation as another defect. Another independent review could change the deduplicated round total. |

“No novelty claimed” is accurate as a disclaimer. InstAL powers, state transitions, Shapley allocation and escrow are attributed rather than presented as inventions. The paper references do not certify this composition or its extensions. No evidence here establishes that the entire construction duplicates one named existing system; conversely, temporal extensions and double-recorded provisional settlement are not established novel merely because no equivalence proof is supplied. Keep the disclaimer and replace broad integration language with the exact assumptions actually tested.

CRITICAL: 2  HIGH: 1  MEDIUM: 5  LOW: 0

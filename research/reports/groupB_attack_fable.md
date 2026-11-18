# Adversarial review of "Step 0, group B: evidence, verdicts and the accountability target"

**Reviewed:** [Step 0 group B accountability.md](/root/mic/reports/Step 0 group B accountability.md), against [Step 0 worked on paper.md](/root/mic/reports/Step 0 worked on paper.md), the two earlier attacks and [Integrated foundation plan.md](/root/mic/reports/Integrated foundation plan.md).

**Labels.** "Shown wrong" means I give a history or derivation. "Could not verify" means a literature detail I recall but did not open. "Judgement" is opinion. ‡ marks a source cited from memory.

**How I applied the scale.** The scale's CRITICAL ("a stated result is false") and MEDIUM ("closed by tightening a statement") overlap. I graded by cost of repair: a result that is false as written but survives after a change of definition is HIGH, and one rescued by adding an assumption is MEDIUM. Read literally, B1's supporting premise, B3, B4 and R2 (a) and (d) are false as stated (H1, H2, M1, M2). None is false beyond repair, so I count no CRITICAL.

**What is right.**
- The choice to make the counterfactual a declared convention is coherent, and the document is honest that it is a fiction outside a narrow class.
- B2 holds for findings, and also for membership of minimal repair sets (a unit with zero shortfall is never in a minimal set).
- The membership claim in B5 is correct.
- Dolev–Strong run in fixed slots does deliver the three bulleted log properties for one proposer.
- The earlier attacks' "stale summary" and "certificate completed later by a faulty witness" findings are fixed in principle.
- The worked numbers in B4 are arithmetically right for zero slack, zero margin and no early performance. H1 shows they cannot be read off the closed summaries.

---

## HIGH

### H1. Closed summaries close before any shortfall larger than the margin can be recorded

**Text attacked.** "A summary for a property is closed at a coverage time: the latest deadline in the unit's part of the contract tree, plus the margin. … It lists … the certified time of each input and of the hand-over, or the certified fact that there was none by the coverage time." And B1: "Every variable of the model appears in some closed summary."

**Shown wrong, on the document's own worked run.**

| Unit | Deadline | Coverage time | Actual event | What the closed summary shows |
|---|---|---|---|---|
| U3 | Day 4 | 4 + m | Hand-over on day 4.8 | "No hand-over by coverage time" if m < 0.8 |
| U2 | Day 8 (8.8 in response form) | 8 + m (8.8 + m) | Hand-over on day 9.3 | "No hand-over by coverage time" |
| U1 | Day 9 | 9 + m | Receipt on day 10.3 | "Not received by coverage time" |

A hand-over later than deadline plus margin is by definition after the coverage time. So a closed summary can record only "within the margin" or "not by the coverage time". It never holds a shortfall above the margin, and U1's summary never holds E.

**Consequences.**
- The verdict "two findings, shortfalls 0.8 and 0.5, E = 1.3, loss split 8 to 5" cannot be computed from the closed summaries.
- The model's values, the sum test in B4, proportional loss and the input to B5 do not exist.
- A unit that never hands over has no defined shortfall under any reading, so proportional sharing is undefined for outright non-performance.

**Not a rescue.** Reading "latest deadline" dynamically as receipt time plus response bound changes nothing for magnitudes (U2: 8.8 + m < 9.3). It also makes the coverage time, and so the time bar, depend on the run, which loses the declared bar.

**Repair.**
1. Close summaries at a declared horizon H after the latest static deadline: coverage = deadline + H + margin. Set the bar after the last coverage time.
2. Record a hand-over later than that as "at least H".
3. State how censored values enter the sum test and the loss shares. For example, a censored unit is treated as having the largest shortfall, or takes the residual.
4. State the cost in the D8/D15 row: finality is delayed by H, and shares are approximate beyond it.

### H2. The model's variables are clipped at zero, so it does not reproduce the actual run; B4's formula double-counts slack; B3 and R2 (d) fail when a unit or link is early

**Text attacked.**
- "Its variables are the shortfalls of the guarantees and the excesses of the links" (each "or zero").
- B4: "If the link excesses alone amount to E or more, no set of units repairs it. Otherwise a set of units is a repair set exactly when its shortfalls sum to at least E less the slack the contracts left."
- B3: "has the same answer whether it is computed on the contract model or by re-running the controllers."

**Derivation.** Let d_i be the signed deviation of each unit and link (actual minus bound, negative when early), and s the static slack. Then E = Σd_i − s. Zeroing the positive deviations of a set S gives a miss of E − Σ_S shortfall. So S repairs exactly when its shortfalls sum to at least E. The slack is already inside E.

**(i) Slack is subtracted twice (shown wrong).** Bounds as in the first pass, but the chain deadline is day 10, so slack is 1.

| Event | Time |
|---|---|
| U3 hands over (bound day 4) | 4.8, shortfall 0.8 |
| U2 receives | 5.8 |
| U2 hands over (bound 3 days) | 9.8, shortfall 1.0 |
| U1 receives (deadline 10) | 10.8, E = 0.8 |

B4's threshold is 0.8 − 1 = −0.2, so every set "repairs" a property that failed, including the empty set. The correct minimal repair sets are {U3} and {U2}.

**(ii) The link-excess test is wrong and the clipped model blames the wrong party (shown wrong).** No slack, first-pass bounds.

| Event | Time |
|---|---|
| U3 hands over (bound day 4) | 3.0, one day early |
| Link U3→U2 (allowance 1) | Takes 2.0, excess 1.0 |
| U2 receives | 5.0 |
| U2 hands over (bound 3 days) | 8.5, shortfall 0.5 |
| U1 receives (deadline 9) | 9.5, E = 0.5 |

- By B4, link excess 1.0 ≥ E, so "no set of units repairs it" and the loss goes to the link's bearer, the sender U3.
- By re-running, with U2 at its bound the hand-over is day 8.0 and the receipt day 9.0, so the property holds. {U2} is the unique minimal repair set.
- The outcome under the document's rule is that the only deviator pays nothing, and a unit that was early by exactly the link's excess pays everything.

**(iii) B3 and R2 (d) fail for a contract-shaped unit that is early (shown wrong for the model as literally defined).** Chain U4→U3→U2→U1, links exactly 1 day (allowance 1). U4 has 2 days from an order at day 0, U3 has 2 days, U2 has 3 days, and the deadline is day 10 (no slack). Every unit has a constant offset, so all are contract-shaped.

| Event | Time |
|---|---|
| U4 hands over | 3.0, shortfall 1.0 |
| U3 receives, hands over | 4.0, 6.3 (shortfall 0.3) |
| U2 receives, hands over | 7.3, 9.8 (2.5 days, 0.5 early) |
| U1 receives | 10.8, E = 0.8 |

- Re-running with U4 at its bound: U3 hands over on 5.3, U2 on 8.8, U1 receives on 9.8. The property holds, so {U4} made the difference.
- On the model as defined, U2's variable is 0, so the predicted arrival with U4 zeroed is 10 + 0.3 = 10.3. It fails, and the model's minimal repair set is {U4, U3}.
- The clipped model also "predicts" an actual miss of 1.3 against the real 0.8, so it is not a model of the run.

**Repair.**
1. Make the variables signed deviations, and deem each unit and each link to keep its signed deviation.
2. Restate B4 as: S repairs iff Σ_S shortfall ≥ E.
3. Restate the link test as: no set of units repairs iff E − Σ(all unit shortfalls) > 0.
4. Summaries already carry the times needed once H1 is fixed.

### H3. The link-residual loss rule has a cliff: a small link excess moves the whole loss from proved deviators to a compliant sender

**Text attacked.** "Where a repair set exists the loss falls on the units with findings … Where none exists it falls on the bearer named for the link", together with "lateness on a link is borne by the sender".

**Shown wrong (the rule contradicts the first loss rule and the purpose of B2).** First-pass contracts, no slack.

| Event | Time |
|---|---|
| U3 hands over (bound day 4) | 9.0, shortfall 5.0 |
| U2 receives, hands over | 10.0, 13.0 (compliant) |
| Link U2→U1 (allowance 1) | Takes 1.5, excess 0.5 |
| U1 receives (deadline 9) | 14.5, E = 5.5 |

With every unit's shortfall zeroed the arrival is day 9.5, so no repair set exists. The whole loss falls on the bearer of the slow link, which is its sender U2, a compliant unit. U3, with a finding of five days, bears nothing. If the link takes 1.0 instead, U3 bears everything. An excess just above the margin flips 100% of the loss.

**Repair.** Allocate by contribution and not by case:
- Each unit with a finding bears its shortfall's share of the total positive deviation: shortfall / (Σ shortfalls + Σ link excesses).
- Each link bearer bears the link's share.

In the example U3 bears 5.0/5.5 and the link bearer 0.5/5.5. A Shapley or Chockler–Halpern responsibility share would be the standard alternatives ([Chockler and Halpern](https://arxiv.org/abs/cs/0312038)‡).

### H4. Proportional sharing charges units that are in no minimal repair set, which contradicts the document's own causal part

**Text attacked.** "the loss falls on the units with findings, in proportion to their shortfalls on the tree."

**Shown wrong (chain with slack).** Deadline day 10 (slack 1), first-pass bounds.

| Event | Time |
|---|---|
| U3 hands over | 4.5, shortfall 0.5 |
| U2 receives, hands over | 5.5, 10.5 (5 days, shortfall 2.0) |
| U1 receives | 11.5, E = 1.5 |

- The only minimal repair set is {U2}.
- U3's lateness is neither necessary (with U3 on time the arrival is 11.0) nor sufficient (with U2 compliant the arrival is 9.5).
- U3 still pays 0.5/2.5 = 20%.

This is the "early irrelevant breach" that the plan names as a refuter for R2.

**Same defect on a tree.** U1 responds to the later of input A (via U2) and input B (via U4). A unit that is 0.1 late on a branch that arrived days before the other pays a share, though no repair set contains it.

**Related, from response form.** A unit that receives a day early and hands over inside its absolute deadline still has a "shortfall". Goods arrive on day 4 and are handed over on day 7.8 against "by day 8": that is 3.8 days against a bound of 3. If the property fails on another branch, this unit is charged.

**Repair.** Share the loss only among units that belong to some minimal repair set. On a chain that is: shortfall > 0 and there is a subset T of the others with E − a ≤ ΣT < E. Or use the contribution rule of H3 with the slack credited first, in a declared order. Either way, state that Loss is a function of Causes, which it currently is not.

### H5. A single-proposer log lets a faulty controller, inside its witness bound, launder its own shortfall into link excess

**Text attacked.**
- "the one batch the controller proposed, or 'void' if the controller proposed two."
- "A performance handed to the obligee's witnesses is entered in the obligee's log by the same protocol, with one certified time."
- B2: "This holds while the named unit's own witness bound holds."

**Gap.** A broadcast protocol has one designated sender per instance, and the hash chain needs one sequencer. If that is the controller, the controller decides when an inbound performance is entered, or voids the batch that holds it by equivocating and re-proposes later. The document says the neighbour needs all its witnesses faulty to do this; under this reading it needs none.

**Shown wrong under the "controller proposes" reading.** f = 1, all of U2's witnesses honest, U2's controller faulty.

| Event | Time |
|---|---|
| U3 hands over (bound day 4) | 4.0 |
| Performance reaches all three of U2's witnesses | 5.0 |
| U2's controller proposes the batch holding the receipt (or voids the day-5 batch and re-proposes) | 5.8, the certified receipt time |
| U2 hands over | 8.8, 3.0 after the certified receipt but 3.8 after the real one |
| U1 receives (deadline 9) | 9.8 |

The summaries show U3 with shortfall 0, U2 with shortfall 0, and link U3→U2 with excess 0.8. The verdict has no finding and no repair set, and the loss falls on U3 as sender. The deviator is neither named nor charged.

**The other reading.** If witnesses enter receipts themselves, there are several proposers. A single-sender broadcast then does not order their entries against the controller's batches, and "one certified time" is not defined. The 2f+1 forwarding witnesses of the obligor would each be an outside sender with no delivery bound.

**What would deliver it (known pieces).** The witnesses run synchronous state-machine replication among themselves, with the controller and the obligor's witnesses as clients. One example is [Sync HotStuff](https://eprint.iacr.org/2019/270): minority faults, 2Δ steady-state commit, leader replacement among replicas. Alternatively run one Dolev–Strong instance per fixed slot per input source, and take the earliest non-void slot as the certified time.

**Repair.**
1. Say which of these is meant.
2. Have a witness timestamp an inbound performance on first receipt and forward it within Δ, with the certified time fixed by witness-side agreement that the controller cannot delay.
3. Drop "void" for anything except the controller's own outbound entries.

---

## MEDIUM

### M1. R2 (a) is false unless the property deadline carries the margins of every guarantee and link on the path

**Text attacked.** "Every bound in a contract is read with twice that margin in the unit's favour"; R2 (a); and B4's "the contracts left no slack".

**Shown wrong as stated.** Margin m = 0.1, so the grace is 0.2 per bound. First-pass contracts.

| Event | Time |
|---|---|
| U3 hands over (bound day 4) | 4.15, short 0.15, no finding |
| Link U3→U2 | 1.15, within grace |
| U2 receives, hands over | 5.3, 8.45 (3.15 days, no finding) |
| Link U2→U1 | 1.15 |
| U1 receives | 9.6 > 9 + 0.2 |

The property fails beyond its margin. The verdict has no finding, no link excess, no broken environment assumption and no absent summary.

**The worked run has the same problem.** For any m > 0, either the shortfalls net of grace (0.8 − 2m, 0.5 − 2m) no longer sum to E, so no repair set exists and the verdict reports a necessary link excess that is not there, or the grace is a threshold only, and the case above applies.

**Repair.**
1. Restore the first pass's well-formedness condition in response form: the property's deadline is at least the sum of bounds and allowances on every path plus the grace of every guarantee and link on it.
2. Add it as a hypothesis of R2.
3. Redo B4's example with that slack. It then needs H2's corrected formula.

### M2. B3 is false on trees under the natural reading of "contract-shaped", and under the reading that makes it true it is a restatement of the deeming rule

**Text attacked.** "its response time to an input is the response bound plus an amount that does not depend on when the input arrived. For a tree of contract-shaped controllers …"

**Shown wrong (per-input reading).** Assembler A guarantees hand-over within 3 days of the later of X (from Ux, due day 3) and Y (from Uy, due day 5). Its actual rule is 4 days after X and 3 days after Y, so each offset is constant. The link to U1 takes 1 day and the deadline is day 9.

| Event | Time |
|---|---|
| X arrives (Ux short 2) | 5 |
| Y arrives | 5 |
| A hands over (bound day 8) | max(5+4, 5+3) = 9, shortfall 1 |
| U1 receives | 10, E = 1 |

- Model: with Ux zeroed, the input time is max(3, 5) = 5, A is deemed 1 short, and the arrival is day 10. So {Ux} is not a repair set, and {A} is the only one.
- Re-run: with X on day 3, A hands over at max(7, 8) = 8 and the arrival is day 9. The property holds, so {Ux} made the difference.

**How to make B3 true.** Define contract-shaped as "the signed deviation against the guarantee's own bound expression is the same on the actual run and on every counterfactual run considered". That is the deeming rule assumed true of the controller, so R2 (d) becomes "if the convention is true, verdicts under it are true".

**Judgement: the class is narrow.** It contains fixed-latency processing. It excludes:
- controllers that work to an absolute deadline (deviation falls as input comes later);
- scheduled or batched departures;
- queues;
- every unit whose lateness was caused by late input, which is the case a causal verdict exists to sort out.

**Unstated assumption.** B3 also needs link delays that do not depend on send time.

**Repair.** Adopt the definition above, add the link assumption, and withdraw "the one statement with content".

### M3. The broadcast protocol: what it really gives, its cost, and what per-batch use needs

**Text attacked.** "run a published broadcast protocol for synchronous networks with signatures† on each batch … After a bounded number of rounds every honest witness has reached the same decision".

**The protocol that fits.** [Dolev–Strong](https://decentralizedthoughts.github.io/2019-12-22-dolev-strong/), source opened.

| Aspect | What the source says |
|---|---|
| Problem | Broadcast with a designated sender |
| Tolerance | Any t < n corruptions |
| Rounds | Exactly t + 1, lock-step |
| Guarantees | Agreement; validity for an honest sender; termination |
| Output | ⊥ when the accepted set has zero or more than one value |
| Cost | O(n²t) signatures |

Here the sender is the controller, outside the 2f+1 witnesses. So n = 2f+2 and t = f+1. That gives f+2 rounds of length Δ plus skew for every batch with no early exit, and O(f³) signatures per batch. The t+1 round bound is tight for deterministic protocols ([Lewis-Pye and Roughgarden](https://arxiv.org/pdf/2101.07095)‡, could not verify).

**Corrections to the stated guarantees.**
- The outcome "void" does not mean "the controller proposed two". A silent or late controller also yields ⊥, with no proof of deviation. An equivocating controller helped by f witnesses can still get one value decided.
- Agreement on ⊥ needs instances that start on a common slot clock. An instance triggered by the proposal's arrival has no common round 1.
- A second proposal after the decision must not void the batch. This was the earlier review's point, and it is implicit here; say it.

**Faster protocols do not fit as written (judgement; details could not verify).** Good-case protocols for f < n/2 commit in about Δ when the proposer is honest ([Abraham, Nayak, Ren, Xiang](https://decentralizedthoughts.github.io/2021-03-09-good-case-latency-of-byzantine-broadcast-the-synchronous-case)). They rely on replacing the leader when it is faulty, and a unit with one fixed controller has no replacement. Their commit is also a local timer event, not an f+1 "decision signature" certificate.

**Per-batch soundness.**
- In the classical synchronous model, an honest witness that misses one bound is counted as faulty ([Guo, Pass, Shi](https://eprint.iacr.org/2019/179)‡; Sync HotStuff's "mobile sluggish" model addresses this).
- With f Byzantine witnesses and one honest witness that was slow once, two conflicting f+1 certificates for one slot are possible: the slow witness and f faulty sign ⊥, and the others sign the batch.
- The predecessor condition makes this permanent. A witness that missed a batch "joins" nothing afterwards, so f silent faulty witnesses then block every certificate.
- No catch-up rule is given.

**Repair.**
1. Name Dolev–Strong, or synchronous state-machine replication as in H5.
2. Give the slot schedule and the f+2 rounds.
3. Add catch-up: a witness adopts a batch on seeing its f+1 certificate.
4. State that the bound must hold for every honest witness on every batch, or adopt a sluggish-fault model.

### M4. The time bar: "the summaries then held" is not one set, and the absent-summary rule collides with the finding rule

**Text attacked.** "The verdict is computed … on the closed summaries then held. A unit whose closed summary is absent at the bar receives a suspicion verdict, and the loss is allocated to it."

**(i) No agreed set.** Links to the collector have no bound. Collector witness w1 receives U3's summary at bar − ε, and w2 receives w1's forward at bar − ε + Δ. Honest collector witnesses hold different sets at the bar, so the verdict is not unique.
- Intake must be decided in the collector's log.
- By H5, if the collector's controller sequences that log, a party to the dispute can make any unit "absent".

**(ii) Rule collision (shown).** U3 is short 1.3 by its own summary, and that alone explains E = 1.3. U2 is compliant, and its summary is delayed past the bar. "Loss falls on the units with findings" gives U3 100%. "The loss is allocated to it" gives U2 100%. The document does not say which applies, and with U2's times missing the model has undefined variables, so "where a repair set exists" cannot be evaluated.

**(iii) Other gaps.**
- Two absent units: the split is undefined.
- A deviator can choose absence. A controller that stops proposing means no batch covers the coverage time (unless slots are clocked, M3), so it converts a finding into a suspicion.

**Repair.**
1. Agree the intake set by the witness protocol at bar + Δ.
2. Charge an absent unit only the residual that the present summaries do not explain. Its neighbours' certified times bound its deviation: downstream receipt minus upstream hand-over minus bounds and allowances.
3. Split the residual among several absent units by a declared rule.
4. Use slot-clocked voids, so that silence yields a closed summary showing non-performance.

### M5. The restated R2 is not the plan's R2: the verdict type is the dual one and completeness is dropped

**Text attacked.** "What it gives the plan is a version of R2 that has a truth value"; "Parts (a) to (c) follow from the definitions".

**(i) Verdict type.**
- The plan's R2 asks for "sets of minimally sufficient sets of parties", after Künnemann, Esiyok and Backes. Their a posteriori verdict is explicitly sufficient causation: "all subsets … that are sufficient to cause" the violation, minimal ([paper](https://arxiv.org/abs/1805.10891), definition 1, text opened).
- A repair set is the but-for dual. It is the modified Halpern–Pearl definition (contingencies held at actual values) restricted to the compliant alternative, in a model whose only dependent variable is the outcome ([Halpern 2015](https://arxiv.org/abs/1505.00162); the abstract was opened, the condition is recalled‡). It is also a minimal repair in the sense of consistency-based diagnosis ([Reiter 1987](https://doi.org/10.1016/0004-3702(87)90062-2)‡).
- On the worked run the two differ. The sufficient-cause verdict is {{U3}, {U2}}, since each alone breaks a zero-slack chain. The repair verdict is {{U2, U3}}.
- So the definition is standard, but the document should name the variant and say that it replaces the plan's type.

**(ii) Completeness.** The plan requires that a violation caused by a deviation yields a proved-deviation verdict naming the deviator. Part (a) only promises some entry. H5 and M4 (iii) show a deviator can arrange for the entry to be a link excess or a suspicion.

**(iii) Content of the parts.**
- (b) is the definition of a finding.
- (c) is the definition of the verdict.
- (d) is M2.

**Judgement on the author's assessment.** "Nothing new" is right. "A consistent assembly" is overstated (H1 to H5). The truth value is obtained by weakening R2 until parts (a) to (c) are definitional. Under the plan's own rule on what counts as a result, none of (a) to (d) is one, and the stop rule's "R2 survives" should not be read as met by this form.

**Repair.**
1. State the variant and the duality.
2. Restore a completeness clause, with H5 fixed: within every unit's witness bound, a unit whose real response exceeded its bound by more than the margin gets a finding.

---

## LOW

### L1. B5: the membership claim is correct, but only weakly hard, and enumeration is not hard in the sense implied

- **Correct.** Unit u with shortfall a is in some minimal repair set iff some subset T of the others has E − a ≤ ΣT < E; then prune T greedily. With integers and a = 1 this is subset sum, so the claim stands.
- **Only weakly hard.** Subset sum has a pseudo-polynomial algorithm, and certified times are meaningful only to the margin. At that resolution membership costs O(n·E/m).
- **Enumeration.** Minimal repair sets on a chain are the complements of maximal feasible knapsack solutions. These can be generated in time polynomial in n and the number of sets ([Lawler, Lenstra, Rinnooy Kan 1980](https://ir.cwi.nl/pub/9683/9683D.pdf); the abstract was confirmed, the knapsack section was not opened).
- **What follows.** The real obstacle is output size (up to C(n, n/2) sets), not hardness. The size bound on findings is cruder than necessary.
- **Adequacy.** The proposed output (all shortfalls plus E) is adequate. On a chain it is a complete, succinct encoding of every minimal repair set. "In general" (trees with joins) is not argued.

### L2. B2 is true for findings but narrower than it reads

A compliant unit is never in a finding or a minimal repair set. Its loss, however, is set by its neighbour's certified receipt time (link excess, H3, H5). Which unit with a finding counts as the "sole cause" also depends on other units' numbers. State B2 as being about findings only.

### L3. Time accuracy across units

"Accurate to within the witnesses' delivery bound plus the clock skew" is argued inside one witness set. A link excess subtracts times certified by two witness sets, so it needs a skew bound between units, which is not assumed. With slot-clocked agreement the certified time is a slot index, and the accuracy is the slot length unless timestamps are proposed and validated separately.

### L4. Wording

- "Suspicion … is not named as having deviated" while bearing the whole loss is a distinction with no stated effect.
- "Excess on a link is nobody's deviation" sits beside "borne by the sender". The table should name the bearer once.

---

## Were the earlier findings fixed or moved?

| Earlier finding | Status |
|---|---|
| Stale summaries (fable H1, astra 1) | Fixed in principle by the coverage time, but the coverage time is set too early (H1) |
| Agreement on what was certified | Fixed if Dolev–Strong in slots is meant (M3); asserted, not shown |
| Voiding erases a certified entry | Fixed implicitly, but voiding now gives the controller a delay lever over inbound receipts (H5) |
| Evidence in transit | Honestly converted into a time bar; the non-agreement moved to the collector's intake (M4) |
| "If it had complied" is not one behaviour | Fixed: "at the bound" |
| Causal verdict needs the composition model | Fixed for additive time, with the wrong variables (H2) |
| Result 2's boundary | Kept and stated honestly |

---

## Verdict

| Item | Verdict |
|---|---|
| B1 | True only as a tautology about a function. Its premise ("every variable appears in some closed summary") is false on the worked run (H1). The intake set is not agreed (M4). |
| B2 | True for findings and for membership of minimal repair sets. It does not protect a compliant unit from loss, and the "all witnesses faulty" precondition for a neighbour's attack is wrong under the single-proposer reading (H5). |
| B3 | False as stated: with an early unit under the clipped model (H2 iii), and on trees under the per-input reading (M2). True, and near-tautological, once contract-shaped means constant signed deviation. |
| B4 | The worked numbers are right for zero slack, zero margin and no earliness. The general formula is wrong: slack is double-counted and the link test is wrong (H2). The numbers cannot be read from closed summaries (H1) and are inconsistent with any positive margin (M1). |
| B5 | The membership claim is correct but weakly hard. Enumeration hardness is overstated. The output is adequate (L1). |
| R2 (a) | False without the margin-budget condition (M1). True with it, but much weaker than the plan's completeness clause (M5). |
| R2 (b) | True by definition, so vacuous. |
| R2 (c) | True by definition once H1 and M4 are repaired. The linear size holds only because the sets are omitted. |
| R2 (d) | False as stated (H2, M2). A restatement of the deeming rule after repair. |
| Author's assessment | "Nothing new" is right. "Consistent assembly" and "R2 has a truth value" are overstated. |

CRITICAL: 0  HIGH: 5  MEDIUM: 5  LOW: 4

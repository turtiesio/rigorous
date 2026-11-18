# A composition contract between existing formalisms, draft 4

**Status.** This is the narrowed goal: not a new foundation and not a new rule language, but a statement of what four published formalisms must guarantee to each other so that together they govern a chain of delegating units. Each clause says what is assumed, what is guaranteed, and whether it is proved here, tested by a model, taken from the literature, or open. Nothing in it is claimed as new.

Two independent testers have attacked it three times by running the models (files beginning "contract_attack_" in /root/mic/reports, with their scripts in /root/mic/model). Their counts of confirmed wrong outcomes or false statements fell from round to round: 21, 15 and 6 for one tester, 8, 6 and 3 for the other. After round 2 the rule that inferred exceptions to prohibitions was removed, because each repair of it failed in a new way. Round 3 found one wrong outcome in its replacement, which is recorded below and not repaired. This draft applies the other round-3 corrections and has not itself been reviewed.

Everything executable is in /root/mic/model. Run the checks with:

```
cd /root/mic/model/instal && python3 test_instal.py && python3 check_stratified.py && python3 test_end_to_end.py && python3 test_conflict_chain.py && python3 test_ledger_link.py
cd /root/mic/model && python3 test_evidence.py && python3 test_verdict.py && python3 test_ledger.py
```

## The four formalisms

| | Formalism | Role here | How it is used |
|---|---|---|---|
| F1 | InstAL, the institutional action language, with its bridges between institutions ([Padget et al.](https://purehost.bath.ac.uk/ws/files/148119668/instal.pdf); [source](https://github.com/instsuite/instal-stable)) | The norm layer: what is permitted, forbidden and owed, and who has the power to change that | The published compiler and solver, with two lines changed for a current solver. One institution per unit, one per authority, one bridge per authority edge |
| F2 | Agreement among a unit's witnesses in a synchronous network with signatures (Dolev and Strong) | The evidence layer: one certified, ordered log per unit | Assumed, not implemented |
| F3 | Assume-guarantee contracts in response form, with the Shapley value for sharing a loss | The verdict layer: who pays what when a chain of commitments fails | [verdict.py](/root/mic/model/verdict.py), [evidence.py](/root/mic/model/evidence.py) |
| F4 | The escrow method for limits on totals | The resource layer: a spending cap under concurrent use and changing rules | [ledger.py](/root/mic/model/ledger.py) |

## The clauses

### K0. What the adapter is trusted to do

Between a scenario and InstAL sits a small program, [harness.py](/root/mic/model/instal/harness.py). It is part of the trusted base. It writes, as initial facts:
- which effects match which norms, from their attributes;
- which unit owns each effect;
- which norms each authority's charter lets it issue;
- which exemption may set aside which prohibition, from the ranks of their origins;
- which bridge powers exist: one for each authority edge, and the same prohibitions, permissions and exemptions for every unit created, directly or indirectly, by the unit at the end of the edge.

It rejects input it would otherwise have to guess about. With one unit, effects and norms default to it. With several, every effect must name its owner, every norm its target, and the authority edges must be given. It also rejects: an owner or target that is not a unit; an edge that does not run from an authority to a unit; an unknown kind of norm; a unit's own norm aimed at another unit; a duty with no declared way owned by its target; an exemption that names no prohibition, or any other norm that names one; an amendment without its procedures; a cost that is not a number; a unit that is also an authority; an empty history.

**Status:** tested by 57 scenarios in all. Both testers checked coverage by changing the adapter one line at a time and running the suite; the six changes that went undetected in round 3 now each have a scenario. That is evidence of coverage for those lines and not a claim that every fact is covered. InstAL resolves the state of the norms over time from these facts. It does not check that they are right.

### K1. The order of events

- **Statement.** A unit's institution sees the events of the unit's own log in the log's order, one per step. An act of an authority appears at the point where the unit's witnesses recorded receiving it.
- **Needs from F2:** a total order on each unit's log, final once decided.
- **Where an order across units is needed.** A duty that is discharged when another unit receives something compares a time in that unit's log with a deadline in this one's. That needs the two clocks to agree to within a stated bound, and the bound has to be added to the link allowance. Draft 2 said no order across units was needed, which was false for exactly these duties.
- **Status:** the models run all institutions on one sequence. Per-unit orders that differ, and clocks that differ, are not tested.

### K2. One outcome per history

- **Statement.** For a given initial state and sequence of events, the institutions have exactly one trace.
- **Argument.** In InstAL, inertial fluents at a step are fixed by the previous step and the events that occurred. The unit institution's derived fluents depend on each other without a cycle, so each is fixed by the inertial ones, and the events generated at the step are fixed in turn.
- **Status:** tested. Each scenario that runs has exactly one answer set, as did 60 and 87 further runs by the testers in the two rounds. The argument is a sketch. [check_stratified.py](/root/mic/model/instal/check_stratified.py) checks only the dependencies among derived fluents in the unit institution, not the conditions on events, the bridges, or the compiled program. It is a necessary check and not a certificate.

### K3. An authority edge is a bridge power

- **Statement.** An authority can bring a norm into force in a unit only if the bridge between their institutions holds the power to initiate it. An authority's own charter decides which norms it may issue. Only the issuer can end a prohibition, a permission or an exemption.
- **Status:** InstAL's own semantics of cross-institution power, given the powers the adapter writes. Tested on five scenarios.

### K4. Conflicts are detected, not resolved

InstAL has no priority among institutions. The paper cited below detects conflicts between institutions; later work by the same group resolves them by revising the institutions. This contract neither revises nor ranks. Drafts 1 and 2 added a rule that inferred, from priorities, when a prohibition should give way to a higher duty. Two rounds of testing each found it wrong in new ways: it let a unit choose which way was "feasible" and so earn an exception; a watched prohibition yielded where an enforced one did not; duties performed while overridden were later recorded as missed. That rule is withdrawn. The contract now follows InstAL's own approach.

1. **Default refusal.** An effect is refused unless something permits it. A duty carries the permission for what it requires, until it is discharged.
2. **Forbid overrides permit.** An enforced prohibition refuses a permitted effect, whatever the ranks of the two.
3. **Explicit exemption only.** A prohibition is set aside for an effect only by an exemption in force that names the prohibition and whose pattern covers the effect. An exemption works only if its issuer is the prohibition's issuer or outranks it. An exemption does not itself permit anything.
4. **Conflict.** A duty is in conflict while every declared way of meeting it is forbidden by an authority. Prohibitions the unit issued for itself do not count. While a duty is in conflict its deadline records the conflict, naming the duty, and records no miss.
5. **Lateness.** A duty not in conflict that is still owed at its deadline is recorded as missed, once. It stays owed, and a late performance is accepted.
6. **Discharge.** One emission discharges every duty in force that it meets.

- **Status:** tested on twenty scenarios.
- **One wrong outcome that remains, and one repaired since round 3.**
  - *An unreachable way counts as a way.* A way is any declared effect that no authority forbids. If the only such effect is an amendment by a procedure the unit cannot reach, because an authority forbids every step towards it, the duty is recorded as missed although nothing the unit could do would have met it. Declaring one effect fewer gives a conflict instead. Draft 2 made feasibility count, and that let a unit destroy its own way and escape; neither version is right.
  - *The deadline after a conflict.* Round 3 found that nothing sent a deadline after a conflict ended, so a unit could then wait indefinitely. K8 now states the rule and tests it for one prohibition per duty. Several overlapping conflicts, and a conflict that ends and starts again, are not tested.
- **What this gives up.** A lower authority can block a higher authority's duty until the higher one issues an exemption. Nothing is resolved without an explicit act by an authority. That is a cost in availability, taken to remove a rule that could not be made right.
- **Closest prior work.** Conflict detection between institutions in InstAL's own line of work ([Li et al.](https://researchportal.bath.ac.uk/en/publications/legal-conflict-detection-in-interacting-legal-systems/)); forbid-overrides-permit as in Cedar; explicit derogation by a permissive norm in defeasible deontic logic ([Governatori et al.](https://arxiv.org/abs/1212.0079)). The only thing rank is used for here is deciding whether an exemption works. I have not checked these sources beyond what the reviewers reported.
- **Known limit.** Whether a duty has a way is decided over the effects declared in the scenario.

### K5. Prevented or only detected

- **Statement.** A prohibition is enforced only if every attribute it reads is certified by someone other than the unit. Otherwise it is watched: a matching emission goes through and is recorded as a violation, unless an exemption covers it. A watched prohibition never puts a duty in conflict.
- **Status:** tested on three scenarios.
- **Not covered.** Duties, permissions and exemptions that read attributes the unit declares for itself.

### K6. A created unit

- **Statement.** A created unit is a new institution. Every authority over its creator, or its creator's creator, and so on, holds a bridge to it with the power to impose the same prohibitions, permissions and exemptions. Duties are not copied.
- **Status:** tested on four scenarios, to a depth of two and with the units listed in either order. The inheritance is done by the adapter; InstAL applies it.
- **Open.** InstAL institutions are fixed for a run. Creation during a run, limits on a creator that count its whole subtree, and a creator's own rules for the units it creates are not modelled.

### K7. Amendment

- **Statement.** Changing a unit's charter procedure is an effect like any other. It needs the procedure in force, it can be forbidden, it can discharge a duty, and when emitted it replaces the procedure. A unit's own rules take effect only by the procedure in force. A unit that amends away its only way of meeting a duty has missed the duty.
- **Status:** tested on seven scenarios.
- **Not covered.** Repeal of a unit's own rule. See also the first wrong outcome under K4.

### K8. From the norm layer to the verdict

- **How the layers meet.** The evidence layer holds the certified times and the contract's bounds. A unit owes two things for a hand-over: to respond within its bound, by handing the output to its own witnesses, and to have it received within the bound plus the link allowance. The evidence layer places the two deadlines accordingly and orders a performance at exactly its deadline before the deadline.
- **What is tested.** On 25 random chains, with links fast or slow: every hand-over takes effect in the norm layer, including late ones; the units InstAL reports as missing the first duty are exactly the verdict's findings; the units missing the second are exactly the verdict's players; nobody else has a share.
- **What this is worth.** Both layers read the same bounds and the same certified times, so agreement on who is late follows from arithmetic. The test shows the two are wired to the same data and order ties the same way. It is not independent evidence that they compose.
- **With a prohibition in play.** Here the verdict needs something only the norm layer knows.
  - *The rule.* A duty's clock stops while the duty is in conflict, for a conflict that begins no later than the deadline. The norm layer's trace says when each duty was in conflict. The evidence layer moves the deadline by that length. The verdict layer is told only the length; it knows nothing about prohibitions.
  - *The test.* [test_conflict_chain.py](/root/mic/model/instal/test_conflict_chain.py) builds 30 random chains in which an authority forbids some deliveries for a while and a unit that would have delivered during the ban delivers when it ends. The oracle is computed by neither layer: a unit is late if the time it took, not counting the time delivery was physically forbidden, exceeds its bound plus allowance. The units InstAL records as missing are exactly those, the verdict's residuals agree, and 24 hand-overs delayed only by a ban were not recorded as missed.
  - *What it found on the way.* The first run failed on a ban starting at exactly the deadline, which needed a rule for the tie: an authority's act is ordered before a deadline at the same time.
- **Open.** Trees; several overlapping conflicts; a faulty evidence layer; clocks that differ.

### K9. Missing evidence: reporting failure is not causal responsibility

A rule that charges a unit for whatever its neighbours cannot account for, because its summary is missing, is rejected.

**The clause.**

1. **Every hand-over is recorded twice.** The receiver's witnesses certify the receipt in the receiver's log and acknowledge it to the sender, whose log then also holds it.
2. **A missing summary is a reporting failure and nothing else.** It is listed. It carries no share of the loss.
3. **A unit's residual is computed from whatever evidence fixes the times on both sides of it,** its own or its neighbours'.
4. **Where no evidence fixes a boundary, the units on either side form a lump,** whatever their residuals sum to. Only the sum is observable.
5. **While any lump is open the whole verdict is provisional.** Amounts can be collected on account, but none is final, because what the lump contains changes what every other unit owes, upward or downward. When the evidence arrives every share is recomputed.

**What is proved.**

- *Exactness.* Assume the time the chain started and the time of the final receipt are known. If no two adjacent units are missing and every acknowledgement arrived, the shares equal the shares on full evidence. Proof: each boundary time is in the later unit's log and, by acknowledgement, in the earlier unit's. If at least one of every adjacent pair is present, every boundary time is known, so every residual is. Tested on 400 random chains. This covers shares. It does not recover the split of a missing unit's residual between its own lateness and its link's.
- *A limit.* When two adjacent units are missing, no rule that depends only on the available evidence can both spare every unit that did not deviate and charge every unit that did. Proof: a chain in which the first of the two was late and one in which the second was give the same evidence, so a rule charges each the same in both; sparing the compliant one in each means charging neither. The test checks this on a grid of allocations for one such pair.

**Counterexamples to stronger claims,** each found or sharpened in review.

| Claim one might make | Counterexample |
|---|---|
| A unit whose own evidence is complete pays what it would on full evidence | A late by one day, B late by two, C early by two. On full evidence A pays half a day. With B and C missing A is asked for the whole day |
| A reporting unit is never asked for less | Three units each a day late against a day of slack. With the first two missing the third is asked for half a day and finally owes two thirds |
| The amount in suspense is what the lump's members owe | Same case: one and a half days are in suspense; the two owe one and a third |
| Suspense is what removes a time bar's unfairness | Double recording does that. With acknowledgements and a single missing unit there is no lump |

**What the clause gives up.**

- If the evidence never arrives, a unit that deviated inside a lump pays nothing and the loss stays where it fell.
- A receiver can open a lump by itself, by withholding both its summary and its acknowledgement. The clause protects units that did not deviate from being charged on a reporting failure. It does not stop a deviator from delaying the verdict.
- How often lumps occur is not known.

**A note on the comparison.** The time-bar rule in [evidence.py](/root/mic/model/evidence.py) is the rule as an earlier draft described it in words. It is not the rule the earlier executable model implemented, which was milder. It is there for comparison only.

### K10. The verdict

The properties in [group B](/root/mic/reports/Step 0 group B accountability.md) stand as stated there: the verdict is a function of the evidence, a unit that was not late pays nothing for lateness, the shares sum to the miss, and they vary continuously. Tested on fifteen cases and 300 random chains. The rule rests on a declared convention about counterfactuals and agrees with re-running the controllers only for a narrow class.

### K11. Caps and the ledger

- **How the layers meet.** InstAL's surface language has no arithmetic, so a cap is not an InstAL norm. The ledger sits outside. It reserves rights for one named effect of one unit and sends that unit's institution a grant. The institution refuses an effect that costs something and has no grant. An emission uses the grant up. A refusal drops the grant, and the ledger releases the reservation.
- **Status:** tested on five histories and one scenario.
- **Limits.** A grant carries no amount. The test keeps the ledger and the institution in step by taking turns, so nothing here shows they stay in step when grants are late, repeated or reordered. A duty to spend that the ledger will not fund is recorded as the unit's miss.

### K12. A norm routed through an intermediate authority

- **Statement.** An authority with no edge of its own to a unit can reach it through an intermediate authority that has one. The norm takes effect below only if the intermediate passes it on, and a withdrawal travels the same way. A routed exemption has the rank of the lowest authority on its route.
- **How.** InstAL's own generation of events across a bridge: the origin's act of issuing generates a relayed event in the intermediate's institution, which generates the intermediate's own issue, which initiates the norm in the unit. The adapter writes the cross-institution powers for each hop.
- **Status:** tested on five scenarios, added after round 3 and not reviewed. The relayed event is one the intermediate's institution will also accept if it is observed directly, so the route is only as trustworthy as the log that supplies events.
- **Not covered.** Translation of a norm between vocabularies along a route, and context an intermediate exports under a lease.

## What is and is not established

| Clause | Proved here | Tested | From the literature | Open |
|---|---|---|---|---|
| K0 adapter | | 57 scenarios; coverage checked by mutation in round 3 | | a specification of the adapter |
| K1 order | | one shared order only | F2's guarantee | differing per-unit orders and clocks |
| K2 one outcome | sketch | every run by the author and the testers | InstAL's event semantics | a proof over the compiled program |
| K3 edges as powers | | 5 | InstAL bridges | |
| K4 conflict and exemption | | 20 | conflict detection; forbid overrides permit; derogation | whether detection without resolution is usable |
| K5 prevented or detected | | 3 | InstAL's power and permission | self-declared attributes elsewhere |
| K6 created units | | 4, depth two | | creation during a run; subtree limits |
| K7 amendment | | 7 | | repeal |
| K8 norms to verdict | | 25 chains for wiring; 30 chains with bans against an independent oracle | | trees; overlapping conflicts; a faulty evidence layer |
| K9 missing evidence | exactness of shares; the limit | 11 checks, 700 random chains | | trees; how often lumps occur |
| K10 verdict | | 15 cases, 300 random | | beyond lateness |
| K12 routes | | 5 | InstAL cross-institution generation | translation; leased context |
| K11 ledger | | 6 cases and 2000 random histories alone; 6 connected | escrow | late or repeated grants |

## Stop conditions

The work stops, and the outcome is reported as negative, if any of these occurs.

1. **A scenario has no trace or several.** Not met in any run.
2. **A wrong outcome within scope whose repair needs a change to InstAL's semantics.** Not met. No defect so far has needed a change to InstAL. Not every defect was repaired: most were repaired in the unit institution or the adapter, the inferred-exception rule was withdrawn, some were moved to the list of things not covered, and one is open under K4.
3. **K8 cannot be made without restating the norm rules inside the verdict layer.** Not met on the one hard case tried. A tester showed in round 3 that the layers disagreed as soon as a unit was under a prohibition at its deadline. With the suspension rule of K8 they agree with an independent oracle on 30 random chains, and the verdict layer is told only a length of time. That test was written after round 3 and has not been reviewed.
4. **K11 cannot be made by events alone.** Not met, in a simple form only.
5. **The defects are not converging:** over two consecutive rounds on the same scope, neither tester's count of confirmed wrong outcomes falls.

**The record for condition 5.** These are the testers' own counts. Round 1: 21 findings from one tester and eight from the other, over a scope larger than the round before it. Rounds 2 and 3, where each was asked for a separate count of confirmed wrong outcomes or false statements within scope: 15 then 6, and 6 then 3. Each tester's count fell in each round, so the condition is not met.

## What was wrong in draft 3

| Clause | Defect | Found by | Now |
|---|---|---|---|
| K4, K7 | A unit with no reachable way was recorded as missing its duty | Fable | Open; stated under K4 |
| K4 | A duty in conflict at its deadline is late afterwards only if a second deadline is sent, and nothing sends one | Fable | Rule stated and tested under K8 for one prohibition per duty |
| K0 | The adapter accepted a misspelt kind of norm or edge, and other malformed input, silently | Fable | Rejected now |
| K0 | The claim that every fact and rejection had a scenario was false | both | Six scenarios added; claim narrowed |
| K11 | A purchase costing nothing broke the connected test | Astra | Fixed |
| K0 | The text said an effect with no owner is always rejected | Astra | Text corrected |
| K4 | The text misdescribed what InstAL's literature does about conflicts | Fable | Text corrected |
| Status | "Round 1 found eleven" understated the testers' counts | Fable | Counts given as reported |

## What was wrong in draft 2

| Clause | Defect | Found by |
|---|---|---|
| K4, K5 | A watched prohibition yielded to a duty where an enforced one would not | both |
| K4 | Because a way that could not be taken did not count, a unit could make its lawful way impossible, or not ask for funds, and earn an exception or avoid a miss | both |
| K4 | A duty performed while overridden was not discharged and was later recorded as missed | Fable |
| K4 | A duty overridden at its deadline could never be late afterwards | Fable |
| K8 | The layers also disagreed when a unit was late and its link fast, and the test never drew a fast link | Fable |
| K1 | The clause denied the cross-unit comparison of times that K8 relies on | Fable |
| K0 | The order in which units were listed, and default edges, decided who was bound | Fable |
| K0 | A creator's norm for a unit it created bound the creator instead | Fable |
| K0 | A duty whose only way belonged to another unit was accepted | both |
| K0 | The documented form of events, and an empty history, could not be run | Astra |
| Stop conditions | The counts given for round 1 were not what the testers had written | Astra |
| K9 | The comparison rule was labelled as the earlier model's rule and was not | Astra |

## What this does not cover

Translation of norms along a route; context exported under a lease; commitments and what a rule change does to them; duties triggered repeatedly; an act window before a deadline, and what a counterparty's refusal or silence means; units under common control; trees in the evidence clause; anything not about time in the verdict.

## Novelty

None is claimed. InstAL, its bridges, conflict detection between institutions, forbid-overrides-permit, synchronous agreement, assume-guarantee contracts, the Shapley value and escrow are all published. What the contract adds is small: which authority's exemption works (K4), what a created unit inherits (K6), and double recording with a provisional verdict (K9). I have not compared any of these closely enough with existing work to say that it differs, and not having found an equivalent is not evidence that there is none.

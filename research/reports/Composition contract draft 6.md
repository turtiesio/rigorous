# A composition contract between existing formalisms, draft 6

**Status.** This is the narrowed goal: not a new foundation and not a new rule language, but a statement of what four published formalisms must guarantee to each other so that together they govern a chain of delegating units. Each clause says what is assumed, what is guaranteed, and whether it is proved here, tested by a model, taken from the literature, or open. Nothing in it is claimed as new.

Two independent testers have attacked it five times by running the models (files beginning "contract_attack_" in /root/mic/reports, with their scripts in /root/mic/model). Their counts of confirmed wrong outcomes or false statements were 21, 15, 6, 5 and 6 for one tester, and 8, 6, 3, 2 and 2 for the other. **In round 5 neither count fell.** One more such round meets stop condition 5.

The pattern behind the counts is plain. After round 2 the rule that inferred exceptions to prohibitions was removed, because each repair failed in a new way. Since then the material that existed before each round has held up better each time: round 5 found two residues of earlier findings in it and nothing new, and found no defect in routes. What fails is whatever was written since the last round. In round 4 that was the suspension rule and routes. In round 5 it was the replacement suspension rule and the verdict by re-run, which held all four high findings between the two testers.

This draft narrows both. The verdict by re-run is now given only facts that do not depend on what the units do, it refuses evidence it cannot reproduce, and its re-run is checked against InstAL on histories that did not happen. These changes have not been reviewed, and on the record above they should be presumed to contain errors. [Draft 5](/root/mic/reports/Composition contract draft 5.md) and [draft 4](/root/mic/reports/Composition contract draft 4.md) are kept.

Everything executable is in /root/mic/model. Run the checks with:

```
cd /root/mic/model/instal && python3 test_instal.py && python3 check_stratified.py && python3 test_end_to_end.py && python3 test_conflict_chain.py && python3 test_ledger_link.py
cd /root/mic/model && python3 test_evidence.py && python3 test_verdict.py && python3 test_gated.py && python3 test_ledger.py
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
- which bridge powers exist: one for each authority edge, and the same prohibitions, permissions and exemptions for every unit created, directly or indirectly, by the unit at the end of the edge;
- for a routed norm (K12): the power of each authority on the route to generate the relayed event in the next, which authorities pass the norm on, which authority finally carries it to the unit, and, for a routed exemption, the lowest rank on its route.

It rejects input it would otherwise have to guess about. With one unit, effects and norms default to it. With several, every effect must name its owner, every norm its target, and the authority edges must be given. It also rejects: an owner or target that is not a unit; an edge that does not run from an authority to a unit; an unknown kind of norm; a unit's own norm aimed at another unit; a duty with no declared way owned by its target; an exemption that names no prohibition, or any other norm that names one; an amendment without its procedures; a cost that is not a finite number of zero or more; a unit that is also an authority; an empty history; a pattern value that is not a set; a unit that is among the units it created, or has two creators. For routes it rejects: a route that visits an authority twice, does not start at the norm's origin, names an authority with no rank, or ends at an authority with no edge to the norm's target; a route for a norm whose origin has an edge of its own to the target; and a declared refusal to pass on by an authority that is not on the route. It rejects a scenario key it does not know, an event that is not an event of any institution, and an event that names a norm, effect or procedure that does not exist. Other malformed shapes, such as a pattern that is not a mapping or an authority named with a reserved word of InstAL, still end in an unhandled error and not in a message.

**Status:** tested by 72 scenarios in all, 23 of them rejections. A rejection scenario passes if the adapter refuses the input for any reason, so it does not show that the intended check is the one that fired.

Both testers checked coverage by changing the adapter one line at a time and running the suite. This document has twice claimed that particular changes were detected when they were not. The current position, from the round-5 runs and my own: the two changes at issue in round 4 are detected in both forms the testers used. Not detected: three wrong versions of the route code (only the first hop written; the carrier taken as the first intermediate; a routed norm not inherited by created units), eight wrong versions of the checks added after round 4, the scenario for edges given as lists, and the rejections of a created unit that is not a unit and of an origin with no rank. So the suite is evidence for the lines it has been shown to cover and no more.

InstAL resolves the state of the norms over time from these facts. It does not check that they are right.

### K1. The order of events

- **Statement.** A unit's institution sees the events of the unit's own log in the log's order, one per step. An act of an authority appears at the point where the unit's witnesses recorded receiving it.
- **Needs from F2:** a total order on each unit's log, final once decided.
- **Where an order across units is needed.** A duty that is discharged when another unit receives something compares a time in that unit's log with a deadline in this one's. That needs the two clocks to agree to within a stated bound, and the bound has to be added to the link allowance. Draft 2 said no order across units was needed, which was false for exactly these duties.
- **Status:** the models run all institutions on one sequence. Per-unit orders that differ, and clocks that differ, are not tested.

### K2. One outcome per history

- **Statement.** For a given initial state and sequence of events, the institutions have exactly one trace.
- **Argument.** In InstAL, inertial fluents at a step are fixed by the previous step and the events that occurred. The unit institution's derived fluents depend on each other without a cycle, so each is fixed by the inertial ones, and the events generated at the step are fixed in turn.
- **Status:** tested. Each scenario that runs has exactly one answer set, as did 60, 87, 94, 89 and 71 further runs by one tester over the five rounds, and every run the other reported. The argument is a sketch. [check_stratified.py](/root/mic/model/instal/check_stratified.py) checks only the dependencies among derived fluents in the unit institution, not the conditions on events, the bridges, or the compiled program. It is a necessary check and not a certificate. Routes add dependencies between institutions that it does not look at; the adapter now refuses a route that returns to an authority, which removes the loop a tester built, and nothing checks the rest.

### K3. An authority edge is a bridge power

- **Statement.** An authority can bring a norm into force in a unit only if the bridge between their institutions holds the power to initiate it. An authority's own charter decides which norms it may issue. Only the issuer can end a prohibition, a permission or an exemption. This holds for the acts of authorities. It does not hold against whoever supplies the log: see K12.
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
- **One wrong outcome that remains, and one moved to K8.**
  - *An unreachable way counts as a way.* A way is any declared effect that no authority forbids. If the only such effect is an amendment by a procedure the unit cannot reach, because an authority forbids every step towards it, the duty is recorded as missed although nothing the unit could do would have met it. Declaring one effect fewer gives a conflict instead. Draft 2 made feasibility count, and that let a unit destroy its own way and escape; neither version is right.
  - *The deadline after a conflict.* Round 3 found that nothing sent a deadline after a conflict ended, so a unit could then wait indefinitely. K8 states a rule. The rule of draft 4 was wrong and is replaced there. No layer's code sends the later deadline: the test computes where it goes after the whole history is known. Several overlapping conflicts, and a conflict that ends and starts again, are not in the maintained tests.
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
- **With a prohibition in play.** Here the verdict needs something only the norm layer knows. Three attempts at this have now failed, and the fourth below is unreviewed.
  - *Draft 4:* stop the clock for the whole conflict. A unit was excused by a ban it never ran into.
  - *Draft 5:* stop the clock from the first refused attempt, and give the verdict that length. The code took any refusal, including one for want of funds or one logged before the unit had its input. And a length fixed from the actual run gives wrong amounts, as shown below.
  - *Draft 5's re-run verdict:* right in idea, wrong at its boundary in three ways. It returned a verdict of no loss for a unit that was refused, then delivered long after the ban ended. It read refusals without checking why they were refused. And it was given the times each duty was *in conflict* on the actual run, which is not a fact about the ban: a duty that is not yet in force is not in conflict, so a ban that came and went before a late input arrived left no trace, and the unit that caused the late input was charged for time that would have arisen under the ban anyway.
- **Scope of what follows.** A chain. One duty per unit. One way of delivering per duty. Acts of authorities do not depend on what units do.
- **Who is late.**
  1. A refusal counts if the unit was ready, an authority forbade delivery at that moment, the refusal came by the unit's deadline, and the unit delivered at the moment the prohibition stopped applying. A unit with a refusal that counts is not late.
  2. Otherwise the deadline stands, except that a duty in conflict at its deadline falls due when the conflict ends, and a delivery at that moment is late.
  - A unit refused in time that then delivers later than it could gets nothing for the refusal. That is harsh on a unit that lost half a day to a ban and then took two. It is chosen because it is the same convention the verdict uses: compare with a unit that complied.
- **Amounts: one length per unit is not enough.** A length fixed from the actual run assumes a unit's delay is the same whatever the others do. A prohibition holds in absolute time, so that is false. Two counterexamples against draft 5's stated rule, in [test_gated.py](/root/mic/model/test_gated.py), both checked by hand by the testers in this or an equivalent form:
  - *A ban absorbs lateness.* A first unit is a day late. The second works its whole budget and meets the ban whether or not the first was late, and delivers when it ends. Had the first complied, the final receipt would be the same. The fixed length charges the first unit a day. (Draft 5 gave a different chain here and compared it against a rule it did not state; under the stated rule that chain was no counterexample.)
  - *Lateness pushes a unit into a ban.* A first unit is a day late, and for that reason the second arrives as a four-day ban begins. Had the first complied there would be no loss. The fixed length charges the first unit one day and loses the other four.
  - On 2000 random chains with bans the fixed length gave different shares from a re-run in 523.
- **The verdict by re-run.** [gated.py](/root/mic/model/gated.py) answers "what if these units had complied" by running the chain again.
  - *What it is given.* Certified times, budgets, each unit's refused attempts, and the intervals during which an authority forbade the unit's one way of delivering. The last comes from the norm layer's trace and is a fact about acts of authorities, so it is the same on every re-run. No norm rule is evaluated in the verdict layer. It does hold a lookup that says a delivery attempted during such an interval waits until it ends; a tester rightly calls that the norm layer's refusal applied by table.
  - *Declared conventions.* Which refusals count is as above. A unit's work ends at a refusal that counts, and otherwise at its receipt. A complying unit takes its budget if its work was longer. The loss that remains when every unit complies is recorded as arising under prohibition and is not shared among the units.
  - *It refuses* evidence it cannot reproduce: a receipt inside a forbidden interval, readiness that does not follow the previous receipt, or a re-run that does not end at the actual receipt.
- **What is tested, and what each test could and could not show.**
  - *By construction.* On 2000 random chains no share is negative, no unit within its budget pays, and shares sum to the loss less the loss under prohibition. These follow from the Shapley value and from delivery time not decreasing with work time. A tester confirmed the test cannot fail on chains drawn inside the conventions. It checks the code.
  - *Against InstAL.* [test_conflict_chain.py](/root/mic/model/instal/test_conflict_chain.py) draws 40 chains, half with every duty in force from the start and half with each duty issued when its unit is ready. It draws what round 5 showed the old generator never drew: units that deliver later than the ban's end, refusals for want of funds, attempts before the input exists. Three comparisons can fail. The units InstAL records as missing are compared with a set computed from the generator's bans. The verdict from the trace's readings is compared with the verdict from the generator's bans. And the verdict's re-run of "every unit complies" is compared with InstAL run on that history, refusals included: they agree in all 40, and in 34 that history differs from the actual one.
  - *Not tested by anything.* That the conventions are the right ones. A re-run for a coalition other than "everyone complies" against InstAL.
- **What it costs.** Shares no longer vary continuously with the facts. A unit that arrives a thousandth of a day later, at the start of a five-day ban, sees a share go from under one day to six. The same holds for a ban that starts at the instant a complying unit would deliver. This is a counterexample to the continuity property of K10 once bans exist. The verdict also depends on absolute times, so K1's bound on clock differences matters here.
- **Closest prior work.** Counterfactuals evaluated on a structural model, as in the causal-responsibility literature the survey cites, with the Shapley value over the outcome. I have not checked whether this construction is published; I would expect it to be.
- **Open.**
  - *More than one way per duty.* The verdict would need to know when every way was forbidden at once, on a history that did not happen. That is rule 4 of K4. Either it is evaluated in the verdict layer, or each re-run goes through the norm layer. See stop condition 3.
  - *Placing the deadlines.* Six lines of the test place them, from the finished history. evidence.py does none of it, and nothing places a deadline as it falls due. Draft 5 said the evidence layer does this; it does not.
  - *Who bears the loss under prohibition.* The claimant, by default.
  - *A unit that waits without attempting* is recorded as late and can be charged for the ban.
  - *Two duties per hand-over,* as in the first test: a ban on delivery says nothing about the response.
  - Trees; several bans on one duty; a faulty evidence layer; clocks that differ.

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

The properties in [group B](/root/mic/reports/Step 0 group B accountability.md) stand as stated there: the verdict is a function of the evidence, a unit that was not late pays nothing for lateness, the shares sum to the miss, and they vary continuously. Tested on fifteen cases and 300 random chains. The rule rests on a declared convention about counterfactuals and agrees with re-running the controllers only for a narrow class. That class does not include chains in which delivery is forbidden for a while: see K8, where the verdict is computed by re-running and continuity fails.

### K11. Caps and the ledger

- **How the layers meet.** InstAL's surface language has no arithmetic, so a cap is not an InstAL norm. The ledger sits outside. It reserves rights for one named effect of one unit and sends that unit's institution a grant. The institution refuses an effect that costs something and has no grant. An emission uses the grant up. A refusal drops the grant, and the ledger releases the reservation.
- **Status:** tested on five histories and one scenario.
- **Limits.** A grant carries no amount. The test keeps the ledger and the institution in step by taking turns, so nothing here shows they stay in step when grants are late, repeated or reordered. A duty to spend that the ledger will not fund is recorded as the unit's miss.

### K12. A norm routed through an intermediate authority

- **Statement.** An authority with no edge of its own to a unit can reach it through an intermediate authority that has one. The norm takes effect below only if the intermediate passes it on, and a withdrawal travels the same way. A routed exemption has the rank of the lowest authority on its route.
- **How.** InstAL's own generation of events across a bridge: the origin's act of issuing generates a relayed event in the intermediate's institution, which generates the intermediate's own issue, which initiates the norm in the unit. The adapter writes the cross-institution powers for each hop.
- **Restrictions added after round 4.** A route visits each authority once, and ends at an authority with an edge to the target. A norm whose origin has its own edge to the target has no route. In draft 4 a route that came back to an authority delivered the norm although an authority between had declined it, and a route beside a direct edge silently replaced the edge.
- **Status:** tested on five scenarios and four rejections. A tester ran eight wrong versions of the route code and the suite passed each: among them writing only the first hop, leaving the origin its own power, and letting a declining intermediate relay a withdrawal. There is no scenario with a route of three, with a created unit under a route, or with a declined withdrawal. So this clause is weakly tested.
- **Limits.**
  - *Passing on is not an act.* It is an initial fact, fixed for the run. An intermediate cannot begin or stop relaying, or pass on an issue and decline its withdrawal.
  - *The log.* A relayed issue or a relayed withdrawal that is observed directly takes effect in every authority's institution that would accept it, whether or not the origin acted and even if the origin's act was against its charter. Observed events carry no sender. So a route, and K3's rule that only the issuer ends a norm, are only as trustworthy as the log that supplies events.
- **Objections recorded and not answered.**
  - A routed exemption takes the lowest rank on its route, and a routed prohibition keeps its origin's. No reason is given for the difference.
  - An exemption from a higher authority, passed on by the lower authority whose prohibition it names, has no effect, although over a direct edge it would. The exception for a prohibition's own issuer does apply to a routed exemption, and the statement above does not say so.
- **Not covered.** Translation of a norm between vocabularies along a route, and context an intermediate exports under a lease.

## What is and is not established

| Clause | Proved here | Tested | From the literature | Open |
|---|---|---|---|---|
| K0 adapter | | 72 scenarios; coverage checked by mutation in rounds 3 to 5, with known gaps | | a specification of the adapter |
| K1 order | | one shared order only | F2's guarantee | differing per-unit orders and clocks |
| K2 one outcome | sketch | every run by the author and the testers | InstAL's event semantics | a proof over the compiled program |
| K3 edges as powers | | 5 | InstAL bridges | |
| K4 conflict and exemption | | 20 | conflict detection; forbid overrides permit; derogation | whether detection without resolution is usable |
| K5 prevented or detected | | 3 | InstAL's power and permission | self-declared attributes elsewhere |
| K6 created units | | 4, depth two | | creation during a run; subtree limits |
| K7 amendment | | 7 | | repeal |
| K8 norms to verdict | two counterexamples to a fixed length; one to continuity | 25 chains for wiring; 40 chains with bans through InstAL, each with one counterfactual history; 2300 chains for the re-run's arithmetic | counterfactuals on a structural model; Shapley | several ways per duty; placing deadlines; who bears loss under prohibition; trees |
| K9 missing evidence | exactness of shares; the limit | 11 checks, 700 random chains | | trees; how often lumps occur |
| K10 verdict | | 15 cases, 300 random | | beyond lateness |
| K12 routes | | 5 and 4 rejections; eight wrong versions undetected | InstAL cross-institution generation | relaying as an act; rank along a route; translation; leased context |
| K11 ledger | | 6 cases and 2000 random histories alone; 6 connected | escrow | late or repeated grants |

## Stop conditions

The work stops, and the outcome is reported as negative, if any of these occurs.

1. **A scenario has no trace or several.** Not met in any run.
2. **A wrong outcome within scope whose repair needs a change to InstAL's semantics.** Not met. No defect so far has needed a change to InstAL. Not every defect was repaired: most were repaired in the unit institution or the adapter, the inferred-exception rule was withdrawn, some were moved to the list of things not covered, and one is open under K4.
3. **K8 cannot be made without restating the norm rules inside the verdict layer.** Open, and leaning towards met outside a narrow case. Drafts 4 and 5 each said "not met" on evidence that did not support it. What stands: with one way per duty, the verdict layer needs only the times an authority forbade that way, which are facts and the same on every re-run, and its re-run agrees with InstAL on 40 histories that did not happen. What does not: with several ways per duty the verdict needs the conjunction of K4's rule 4 on counterfactual histories. Evaluating it in the verdict layer meets this condition. Running each counterfactual through the norm layer does not restate a rule, but then the verdict is a function of the norm layer's program and not of the evidence, and its cost is one solver run per coalition. Neither has been built.
4. **K11 cannot be made by events alone.** Not met, in a simple form only.
5. **The defects are not converging:** over two consecutive rounds on the same scope, neither tester's count of confirmed wrong outcomes falls.

**The record for condition 5.** These are the testers' own counts. Round 1: 21 findings from one tester and eight from the other. Rounds 2 to 5, where each was asked for a separate count of confirmed wrong outcomes or false statements within scope: 15, 6, 5 and 6; and 6, 3, 2 and 2. In round 5 neither count fell. That is one round of the two the condition requires. If round 6 does not bring at least one count down, the condition is met and the work stops with a negative result.

Where the round-5 findings were: six of the eight, and all four rated high, in material written after round 4. The other two were residues of round-4 findings in the adapter. Neither tester found a new defect in material that existed at round 4, and neither found a defect in routes. The scope changed between rounds, so the counts are not a controlled measure.

## What was wrong in draft 5

| Clause | Defect | Found by | Now |
|---|---|---|---|
| K8 | The re-run verdict reported no loss for a unit refused in time that delivered long after the ban ended, and for a unit refused for want of funds; nothing read the flag that said the re-run did not reproduce the receipt | both | Such a refusal does not count; evidence that cannot be reproduced is refused |
| K8 | The reading of refusals took any refusal: one for want of funds before a ban, or one logged before the unit was ready | both | A refusal counts only when the unit was ready, delivery was forbidden by an authority, and delivery followed at the end |
| K8 | Conflict intervals from the actual run were used on re-runs; a ban before a duty came into force left no trace | Fable | The verdict is given the times an authority forbade delivery; half the test chains issue duties at readiness; the re-run is checked against InstAL on a counterfactual history |
| K8 | The first counterexample to a fixed length, and the count of 501, were against a rule the document did not state | both | Counterexample replaced; comparison made against the stated rule; 523 |
| K8 | The four properties reported as tested on 2000 chains hold by construction; "the evidence layer places the deadline" was false | Fable | Said so |
| K0 | A misspelt norm in an event, or a misspelt scenario key, changed the outcome silently; a fractional cost was refused | Fable | Rejected; accepted. Other malformed shapes still raise unhandled errors |
| K0 | "Both now fail the suite" was false for one form of the ownership change | Fable | Scenario strengthened; that form now fails the suite, which I ran |

## What was wrong in draft 4

| Clause | Defect | Found by | Now |
|---|---|---|---|
| K8 | A ban that never delayed a unit excused the unit's own lateness; the oracle was the rule restated; "24 delayed only by a ban" was nine | Fable | Rule replaced; oracle uses the unit's intention; not reviewed |
| K8, stop condition 3 | The test did not call the verdict or the evidence code; one length per unit was said to be enough | both | The verdict is called. A length is shown not to be enough and is replaced by a re-run on conflict intervals; not reviewed |
| K12, K0 | A route that returned to an authority went round one that had declined | Astra | Such routes are rejected |
| K12, K0 | A route beside a direct edge replaced it silently; routes and refusals to pass on were not validated | Fable | Rejected |
| K0 | Pattern values, edge types and a cost that is not a finite number were accepted | Fable | Rejected, or normalised for edges |
| K0 | Two of the six coverage repairs did not detect their change | both | Two scenarios replaced; both changes now fail the suite |
| K2, Status | The further runs cited were one tester's and left out round 3; the status line miscounted round 3's findings | Fable | Text corrected |

## What was wrong in draft 3

| Clause | Defect | Found by | Now |
|---|---|---|---|
| K4, K7 | A unit with no reachable way was recorded as missing its duty | Fable | Open; stated under K4 |
| K4 | A duty in conflict at its deadline is late afterwards only if a second deadline is sent, and nothing sends one | Fable | A rule is stated under K8; draft 4's was wrong; still no layer sends the deadline |
| K0 | The adapter accepted a misspelt kind of norm or edge, and other malformed input, silently | Fable | The reported cases are rejected; round 4 found more |
| K0 | The claim that every fact and rejection had a scenario was false | both | Six scenarios added, of which two did not work until round 4; claim narrowed |
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

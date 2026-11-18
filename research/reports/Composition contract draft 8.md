# A composition contract between existing formalisms, draft 8

**Status.** This is the narrowed goal: not a new foundation and not a new rule language, but a statement of what four published formalisms must guarantee to each other so that together they govern a chain of delegating units. Each clause says what is assumed, what is guaranteed, and whether it is proved here, tested by a model, taken from the literature, or open. Nothing in it is claimed as new.

Two independent testers have attacked it seven times by running the models (files beginning "contract_attack_" in /root/mic/reports, with their scripts in /root/mic/model). Their counts of confirmed wrong outcomes or false statements were 21, 15, 6, 5, 6, 3 and 3 for one tester, and 8, 6, 3, 2, 2, 1 and 2 for the other. **In round 7 neither count fell.** Both had fallen in round 6. If neither falls in round 8, stop condition 5 is met.

What the round-7 findings were matters more than their number. For the first time one tester rated nothing high. Three of the five were errors in this document or in a comparison helper: a stale number reported by both, a comparator that did not implement the rule it was labelled with, and a rule whose wording did not match the test. One was a wrong outcome: an attempt logged before the unit's input at the same certified time excused the unit. It is the second defect of that kind, after the one round 6 found, and it has the same cause. The re-run verdict itself held: one tester built histories independently of it for 155 coalitions and InstAL agreed on every one.

This draft applies round 7. The changes are unreviewed. Drafts [7](/root/mic/reports/Composition contract draft 7.md), [6](/root/mic/reports/Composition contract draft 6.md), [5](/root/mic/reports/Composition contract draft 5.md) and [4](/root/mic/reports/Composition contract draft 4.md) are kept.

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

It rejects input it would otherwise have to guess about. With one unit, effects and norms default to it. With several, every effect must name its owner, every norm its target, and the authority edges must be given. It also rejects: an owner or target that is not a unit; an edge that does not run from an authority to a unit; an unknown kind of norm; a unit's own norm aimed at another unit; a duty with no declared way owned by its target; an exemption that names no prohibition, or any other norm that names one; an amendment without its procedures; a cost that is not a finite number of zero or more; a unit that is also an authority; an empty history; a pattern value that is not a set; a unit that is among the units it created, or has two creators. For routes it rejects: a route that visits an authority twice, does not start at the norm's origin, names an authority with no rank, or ends at an authority with no edge to the norm's target; a route for a norm whose origin has an edge of its own to the target; and a declared refusal to pass on by an authority that is not on the route. It rejects a scenario key it does not know, a key inside a norm it does not know, a charter power that names a norm that does not exist, an event that is not an event of any institution, and an event that names a norm, effect or procedure that does not exist. It does not check that an attribute declared as self-declared is read by any norm, and it takes a cost only as a Python real number, so a Decimal is refused. Other malformed shapes, such as a pattern that is not a mapping or an authority named with a reserved word of InstAL, still end in an unhandled error and not in a message.

**Status:** tested by 74 scenarios in all, 25 of them rejections. A rejection scenario passes if the adapter refuses the input for any reason, so it does not show that the intended check is the one that fired.

Both testers checked coverage by changing the adapter one line at a time and running the suite. This document has three times misstated the result, twice by claiming a detection that did not happen and once by dropping gaps from the list. The list below is the union of what the testers reported undetected in rounds 4 to 6 and I have not fixed:
- routes: eight wrong versions in round 4, of which three were re-run and still pass in round 6 (only the first hop written; the carrier taken as the first intermediate; a routed norm not inherited by created units);
- five rules of K4 and K5 first reported in round 3;
- the rejections of a target that is not a unit, of malformed identifiers, of a created unit that is not a unit, and of an origin with no rank;
- eight wrong versions of the checks added after round 4, and four of those added after round 5;
- the scenario for edges given as lists, which passes when they do not work.

Detected, by the testers' runs: both forms of the ownership change, the foreign-match change, the four route restrictions, and removal of blocking, relay powers or withdrawal powers. So the suite is evidence for the lines it has been shown to cover and no more.

InstAL resolves the state of the norms over time from these facts. It does not check that they are right.

### K1. The order of events

- **Statement.** A unit's institution sees the events of the unit's own log in the log's order, one per step. An act of an authority appears at the point where the unit's witnesses recorded receiving it.
- **Needs from F2:** a total order on each unit's log, final once decided.
- **Where an order across units is needed.** A duty that is discharged when another unit receives something compares a time in that unit's log with a deadline in this one's. That needs the two clocks to agree to within a stated bound, and the bound has to be added to the link allowance. Draft 2 said no order across units was needed, which was false for exactly these duties.
- **Equal times.** The verdict of K8 reads times, and times alone cannot say which of two entries came first. Two rounds each found a wrong outcome from this: a refusal for want of funds read as a refusal under a ban that began just after it, and an attempt logged before the unit's input arrived read as made once ready. So the contract requires, at equal certified times: acts of authorities before requests, and a request of one unit before a request of a unit after it in the chain. The verdict takes the log and rejects one in another order. This narrows what F2 may deliver. It is a list of the two orderings found to matter and not a proof that there is no third; the sounder repair is to carry log positions through the verdict, which is not done.
- **Status:** the models run all institutions on one sequence. Per-unit orders that differ, and clocks that differ, are not tested.

### K2. One outcome per history

- **Statement.** For a given initial state and sequence of events, the institutions have exactly one trace.
- **Argument.** In InstAL, inertial fluents at a step are fixed by the previous step and the events that occurred. The unit institution's derived fluents depend on each other without a cycle, so each is fixed by the inertial ones, and the events generated at the step are fixed in turn.
- **Status:** tested. Each scenario that runs has exactly one answer set, as did 60, 87, 94, 89, 71, 182 and 388 further runs by one tester over the seven rounds, and every run the other reported. The argument is a sketch. [check_stratified.py](/root/mic/model/instal/check_stratified.py) checks only the dependencies among derived fluents in the unit institution, not the conditions on events, the bridges, or the compiled program. It is a necessary check and not a certificate. Routes add dependencies between institutions that it does not look at; the adapter now refuses a route that returns to an authority, which removes the loop a tester built, and nothing checks the rest.

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
  1. A refusal excuses if the unit was ready, an authority forbade delivery at that moment, the refusal came by the unit's deadline, and the unit delivered at the moment the prohibition stopped applying.
  2. A unit is late if it delivered after its deadline with no refusal that excuses.
  3. For InstAL to record that, the deadline is placed in one of three ways. For a unit with a refusal that excuses: at its receipt, after the request, where the duty is already discharged. Otherwise at the original time; unless the duty is in conflict then, in which case when the conflict ends and before any request. Draft 7 left out the first case, and followed as written it recorded a miss for every excused unit. The second and third only undo rule 4 of K4, under which a deadline in conflict records no miss. A tester changed InstAL so that no duty is ever in conflict and the test still passed. So agreement here is not evidence about the conflict rule.
  - A unit refused in time that then delivers later than it could gets nothing for the refusal. That is harsh. It is chosen because it is the convention the verdict uses: compare with a unit that complied.
  - Rounds 6 and 7 found this rule, InstAL's record and the verdict's players in agreement on every boundary case tried, apart from the equal-times case now excluded by K1.
- **Amounts: one length per unit is not enough.** A length fixed from the actual run assumes a unit's delay is the same whatever the others do. A prohibition holds in absolute time, so that is false. Two counterexamples against draft 5's stated rule, in [test_gated.py](/root/mic/model/test_gated.py), both checked by hand by the testers in this or an equivalent form:
  - *A ban absorbs lateness.* A first unit is a day late. The second works its whole budget and meets the ban whether or not the first was late, and delivers when it ends. Had the first complied, the final receipt would be the same. The fixed length charges the first unit a day. (Draft 5 gave a different chain here and compared it against a rule it did not state; under the stated rule that chain was no counterexample.)
  - *Lateness pushes a unit into a ban.* A first unit is a day late, and for that reason the second arrives as a four-day ban begins. Had the first complied there would be no loss. The fixed length charges the first unit one day and loses the other four.
  - On 2000 random chains with up to two bans per unit the fixed length gave different shares from a re-run in 563, and on 4000 with at most one in 746. The comparator is draft 5's rule written out on its own. In draft 7 it borrowed the current rule for which refusals excuse, which is stricter, and the number in the text was from an earlier generator.
- **The verdict by re-run.** [gated.py](/root/mic/model/gated.py) answers "what if these units had complied" by running the chain again.
  - *What it is given.* Certified times, budgets, each unit's refused attempts, and the intervals during which an authority forbade the unit's one way of delivering. The last comes from the norm layer's trace and is a fact about acts of authorities, so it is the same on every re-run. No norm rule is evaluated in the verdict layer. It does hold a lookup that says a delivery attempted during such an interval waits until it ends; a tester rightly calls that the norm layer's refusal applied by table.
  - *What a unit did, as distinct from whether it was late.* Draft 6 answered both with one expression, so a unit whose refusal did not excuse it had its wait under the ban counted as work, and that wait was replayed on every re-run. Now a unit's work runs to its first attempt made once ready and refused under a ban, whatever the deadline, and the time from the ban's end to its receipt is kept separately. Waiting for a ban to end is never work.
  - *Declared conventions.* On a re-run a unit attempts after its work; if forbidden then, it waits and delivers after its after-time. A late unit that complies takes its budget if its work was longer and has no after-time. The loss that remains when every late unit complies is recorded as arising under prohibition and is not shared.
  - *It refuses* a log out of the order K1 requires, a receipt inside a forbidden interval or before readiness, readiness that does not follow the previous receipt, a negative budget and an empty interval. Each has a test. It also checks that the re-run ends at the actual receipt; a tester found no evidence on which that check fires once the others pass.
- **What is tested, and what each test could and could not show.**
  - *Named cases with amounts supplied by the testers,* who obtained them by running InstAL on the histories concerned: a unit refused after its deadline (3.75 and 0.25 days, where draft 6 gave 2 and 2); a unit within budget that delivers after the ban's end (nothing moved to the claimant); a ban that came and went before a duty came into force (1.5 days charged and 1.5 under prohibition, where draft 5 gave 3). Each runs through InstAL, and for each every coalition's re-run is compared with InstAL on that history.
  - *Random chains through InstAL.* [test_conflict_chain.py](/root/mic/model/instal/test_conflict_chain.py) draws 40, half with every duty in force from the start. For each: the units InstAL records as missing against a set computed from the generator's bans; the verdict from the trace's readings against the verdict from the generator's bans; and the re-run against InstAL for "everyone complies" and, where two or more units were late, for some but not all of them complying. That is 61 coalitions, of which 51 give a history different from the actual one.
  - *By a tester, independently.* In round 7 one tester built the counterfactual histories without using gated.py, for every coalition of 64 chains from a wider generator: 155 coalitions, 83 of them histories that did not happen. InstAL agreed with the re-run on all.
  - *Repairs that are pinned.* I undid two earlier repairs in turn, handing the verdict conflict intervals and dropping the condition that the unit was ready, and a test failed each time. Round 6 had shown that neither was pinned.
  - *By construction.* On 6000 random chains, 2000 with up to two bans per unit and 4000 with at most one, only late units have shares and shares sum to the loss less the loss under prohibition. With no bans the re-run equals the plain verdict on 300. This checks the code.
  - *Not tested by anything.* That the conventions are the right ones. A unit that logs an attempt the instant its input arrives has no work at all, and the norm layer does not ask whether there was anything to deliver; a tester showed that the timing of that log entry decides what the unit upstream pays. A refusal carries no reason in the trace, so a unit without funds that is refused under a ban and funded as it ends is excused.
- **What it costs.**
  - *A share can be negative, outside this scope.* Arriving during a ban can cost more than arriving after it, so a unit whose lateness carried the next unit past a ban reduced the loss and its share is a credit. The one example found needs two bans on one unit, which is listed as open below. With one ban per duty a tester gave a short argument that no share is negative and found none in 60000 chains; the test here finds none in 4000. Draft 7 presented the example as a counterexample to K10 within scope, which it is not.
  - *Continuity.* Shares no longer vary continuously with the facts. A unit that arrives a thousandth of a day later, at the start of a five-day ban, sees a share go from under one day to six. The same holds for a ban that starts at the instant a complying unit would deliver. This is a counterexample to the continuity property of K10 once bans exist.
  - *Absolute times.* The verdict depends on them, so K1's bound on clock differences and its rule for equal times matter here.
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

The properties in [group B](/root/mic/reports/Step 0 group B accountability.md) stand as stated there: the verdict is a function of the evidence, a unit that was not late pays nothing for lateness, the shares sum to the miss, and they vary continuously. Tested on fifteen cases and 300 random chains. The rule rests on a declared convention about counterfactuals and agrees with re-running the controllers only for a narrow class. That class does not include chains in which delivery is forbidden for a while: see K8, where the verdict is computed by re-running, continuity fails, and with several bans on one unit a share can be negative.

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
| K0 adapter | | 74 scenarios; coverage checked by mutation in rounds 3 to 6, with known gaps | | a specification of the adapter |
| K1 order | | one shared order only | F2's guarantee | differing per-unit orders and clocks |
| K2 one outcome | sketch | every run by the author and the testers | InstAL's event semantics | a proof over the compiled program |
| K3 edges as powers | | 5 | InstAL bridges | |
| K4 conflict and exemption | | 20 | conflict detection; forbid overrides permit; derogation | whether detection without resolution is usable |
| K5 prevented or detected | | 3 | InstAL's power and permission | self-declared attributes elsewhere |
| K6 created units | | 4, depth two | | creation during a run; subtree limits |
| K7 amendment | | 7 | | repeal |
| K8 norms to verdict | counterexamples: two to a fixed length, one to continuity | 25 chains for wiring; 4 named cases and 40 random chains through InstAL, 61 coalitions re-run against it, and 155 by a tester; 6300 chains for the re-run's arithmetic | counterfactuals on a structural model; Shapley | the conventions; several ways per duty; several bans per duty; placing deadlines; who bears loss under prohibition; trees |
| K9 missing evidence | exactness of shares; the limit | 11 checks, 700 random chains | | trees; how often lumps occur |
| K10 verdict | | 15 cases, 300 random | | beyond lateness |
| K12 routes | | 5 and 4 rejections; eight wrong versions undetected | InstAL cross-institution generation | relaying as an act; rank along a route; translation; leased context |
| K11 ledger | | 6 cases and 2000 random histories alone; 6 connected | escrow | late or repeated grants |

## Stop conditions

The work stops, and the outcome is reported as negative, if any of these occurs.

1. **A scenario has no trace or several.** Not met in any run.
2. **A wrong outcome within scope whose repair needs a change to InstAL's semantics.** Not met. No defect so far has needed a change to InstAL. Not every defect was repaired: most were repaired in the unit institution or the adapter, the inferred-exception rule was withdrawn, some were moved to the list of things not covered, and one is open under K4.
3. **K8 cannot be made without restating the norm rules inside the verdict layer.** Open, and leaning towards met outside a narrow case. Drafts 4 and 5 each said "not met" on evidence that did not support it. What stands: with one way per duty, the verdict layer needs two things, both from the log and the trace and neither a norm rule. One is the times an authority forbade that way, which are the same on every re-run. The other is each unit's own time free of any wait under a ban. Both are read from times, which is why K1 has to fix the order of entries at equal times. Its re-run agrees with InstAL on 51 histories that did not happen here, and on 83 more built independently by a tester. What does not stand: with several ways per duty the verdict needs the conjunction of K4's rule 4 on counterfactual histories. Evaluating it in the verdict layer meets this condition. Running each counterfactual through the norm layer does not restate a rule, but then the verdict is a function of the norm layer's program and not of the evidence, at one solver run per coalition. Neither has been built.
4. **K11 cannot be made by events alone.** Not met, in a simple form only.
5. **The defects are not converging:** over two consecutive rounds on the same scope, neither tester's count of confirmed wrong outcomes falls.

**The record for condition 5.** These are the testers' own counts. Round 1: 21 findings from one tester and eight from the other. Rounds 2 to 7, where each was asked for a separate count of confirmed wrong outcomes or false statements within scope: 15, 6, 5, 6, 3 and 3; and 6, 3, 2, 2, 1 and 2. Neither count fell in round 5; both fell in round 6; neither fell in round 7. The condition needs two consecutive rounds and has had one, twice. If round 8 brings neither count down, it is met and the work stops with a negative result.

Two things qualify the round-7 numbers without changing them. Each tester noted a finding that could be read otherwise: one that its high finding becomes a design objection if an attempt at the time of the input is declared to count, the other direction that a rejected empty charter could be counted as a fifth. And the counts mix wrong outcomes with false statements in this document: of five findings in round 7, one was a wrong outcome of the models.

Where the round-7 findings were: one tester put two of three in material written after round 6, the other both of two. The equal-times defect was in older material that the round-6 repair should have covered and did not. The scope changed between rounds, so the counts are not a controlled measure.

## What was wrong in draft 7

| Clause | Defect | Found by | Now |
|---|---|---|---|
| K1, K8 | An attempt logged before the unit's input at the same certified time was read as made once ready, and excused the unit; six days moved between the upstream unit and "under prohibition" on the position of two log entries | Fable | K1 requires a second order at equal times; the verdict takes the log and rejects one out of order |
| K8 rule 3 | As worded, the deadline of an excused unit was placed before its delivery, so InstAL recorded a miss for every unit rule 1 excuses. The test placed it differently | Fable | The text says what the test does |
| K8 | The comparator labelled as draft 5's rule used the current, stricter rule for which refusals excuse | Astra | Written out independently; a case that tells them apart is a test |
| K8 | "523" was from an earlier generator | both | 563, and the count for one ban per unit |
| K8, K10 | The negative share was given as a counterexample within scope; it needs two bans on one unit | Fable | Said so; no negative share with one ban per unit in 4000 chains |
| K8 | The order check was called only by the test, on a list the test had sorted; the verdict's rejections had no tests; the second coalition was often the actual history | Fable | The verdict takes the log; each rejection has a test; the second coalition is a proper part of the late units |
| K0 | An explicit empty charter for an authority that issues nothing was rejected | Fable | Accepted |

## What was wrong in draft 6

| Clause | Defect | Found by | Now |
|---|---|---|---|
| K8 | When a refusal did not excuse, the unit's wait under the ban was counted as its work and replayed on every re-run. A unit half a day late was charged two days; loss an upstream unit caused was recorded as arising under prohibition | Fable | Whether a unit is late and what it did are read separately; the testers' amounts are now test cases; not reviewed |
| K1, K8 | With a request recorded before an authority's act of the same time, a refusal for want of funds was read as a refusal under the ban, and valid evidence was rejected | Astra | K1 states the order at equal times; a log in another order is rejected |
| K8 | The test did not pin two of round 5's repairs, and compared only the coalition "everyone complies", built from the verdict's own work times | Fable | Named cases added; one further coalition per chain and every coalition in the named cases; both repairs shown to be pinned |
| K0 | A misspelt key inside a norm and a charter power naming nothing changed outcomes silently | Fable | Rejected. An unread self-declared attribute and a Decimal cost are stated limits |
| K0 | The list of undetected changes had dropped seven that were still undetected | Fable | Restored |
| Stop condition 3, claims table | "40 histories that did not happen": six were the actual history | Fable | Counts given as run |

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

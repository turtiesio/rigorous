# Step 0, group B: evidence, verdicts and the accountability target

**Status.** This continues [Step 0 worked on paper.md](/root/mic/reports/Step 0 worked on paper.md) and settles the plan's second group of design decisions (D8, D9, D10, D15, D16) with the target they serve, R2. The first draft was attacked once by two independent reviewers ([groupB_attack_astra.md](/root/mic/reports/groupB_attack_astra.md), [groupB_attack_fable.md](/root/mic/reports/groupB_attack_fable.md)). They agreed that making the counterfactual a declared convention is coherent, and that the model, the evidence and the loss rules as first written were wrong: four of the draft's five stated results were false as written. This version is rebuilt on their counterexamples and has not been reviewed again. † marks something recalled from memory.

## Results after review

1. **The plan's R2 is replaced, not proved.** The plan asked for minimal sets of deviating parties that suffice for the failure. What can be computed from boundary evidence is a different object: what each unit's lateness added to the miss, under a declared rule for counterfactuals. By the plan's own stop rule this is a new target, called R2′ here.
2. **R2′ is true by construction, and that is all it is.** Its properties are consistency properties: the verdict is a function of the evidence, a unit that was never late is charged nothing for lateness, the shares add up, and a small change in the facts makes a small change in the shares.
3. **The declared rule matches what the controllers would really have done only for a narrow class:** fixed processing time per output, readiness when all inputs have arrived, no shared resources. Outside it the verdict is a convention.
4. **Evidence after a deadline has to be allowed for.** A summary closed at the deadline cannot say how late anything was. Summaries close at a fixed horizon after the property's deadline, and lateness beyond the horizon is recorded only as "at least that much".
5. **A log entry, once final, is never voided.**

## The model

**One variable per event.** Each hand-over is one event with one certified time. Each output event has a readiness rule, which is the time the last of its named inputs was received, and one response bound. A second bound on the same event is a predicate on the same variable and adds no variable. This also makes the result independent of how many times a guarantee is written down.

**Residuals are signed.** A unit's residual on an output is its actual response time minus its bound: positive when late, negative when early. A link's residual is its actual delay minus its allowance. The actual run is reproduced exactly by the bounds, the allowances and the residuals. The draft kept only the positive part, which did not reproduce the run and blamed the wrong unit whenever someone had been early.

**The declared rule of counterfactual.** In any counterfactual, every output event and every link keeps its signed residual, except those being set to compliance, whose positive residuals become zero. Early performance is kept.

**The miss.** The property has a deadline. The miss E is how far the final certified receipt exceeded it. A well-formed contract tree has bounds and allowances that sum to no more than the deadline on every path; the difference is the slack, and it is already inside E.

## Evidence

**The log.** A unit's witnesses, 2f+1 with at most f faulty and a delivery bound among them, keep the log on a fixed schedule of slots that runs whether or not the controller is active. Each slot is decided once by a published agreement protocol for synchronous networks with signatures. Dolev and Strong's protocol is one that fits: with the controller as an extra, possibly faulty sender it takes f+2 rounds per slot and a number of signatures that grows with the cube of f ([protocol](https://www2.imm.dtu.dk/courses/02220/2015/L12/DolevStrong83.pdf), as the reviewers read it).

- A slot's decision is final. A conflicting proposal that surfaces later is appended as proof that the controller deviated and changes nothing already decided. A slot with no admissible proposal is decided as an empty slot with a successor hash.
- Witnesses enter what they receive from outside themselves, stamped on first receipt and forwarded within the delivery bound. A controller cannot delay or omit an input its witnesses received. In the draft the controller proposed these entries, and a faulty controller could turn its own lateness into apparent lateness of the link.
- After a decision every honest witness broadcasts its signature, so honest witnesses can always assemble the f+1 signatures of a certificate. Faulty witnesses cannot control whether it is available.
- An honest witness that misses a bound counts as faulty for that period. Rejoining needs a transfer of the certified log, which is not specified here.

**Contractual time.** The time of an event is its certified time. Certified times can differ from real times by the witnesses' delivery bound plus the clock skew, and a unit that performs closer to a deadline than that carries the risk. No further margins enter the arithmetic.

**Closed summaries.** Every unit on a property's tree closes its summary at one frontier: the property's deadline plus a declared horizon H. The summary is produced by a fixed extraction rule from the decided log up to the frontier, tied to the property, the contract version and the frontier. It lists the certified time of each input and each output event, or "none by the frontier". An event later than the frontier is therefore known only as later than the frontier.

**Intake.** The witnesses of the unit the property is owed to collect the summaries. They enter each one they receive in their own log, and at a cutoff after the frontier they certify a manifest: which summaries are in, and which units are missing. The verdict is tied to that manifest. A summary that reaches no honest collecting witness before the cutoff is missing.

## The verdict

**Findings.** A finding against a unit is a positive residual on one of its output events, shown by its own closed summary. A unit whose input arrived late and which responded within its bound has no finding.

**Contributions.** The sender bears its outgoing link, so an output event and that link are one player with one signed residual: the unit's own plus the link's. A sender that was early by as much as its link was slow has a residual of zero. The players are those with a positive combined residual, and the missing units. A missing unit's own residual cannot be separated from those of its two links, so the three are one lump, computed from its neighbours' summaries. For any set of players, compute on the model the miss that would have occurred had only those players kept their positive residuals. Each player's share is its average marginal addition to the miss over all orders in which the players could be added. This is the Shapley value of that function.

**Loss.** Each player pays its share. A player whose own residual is not positive, and whose combined residual is positive only because of its link, pays without a finding. A missing unit pays its lump's share. The shares sum to the miss.

**Censoring.** A residual known only as "at least H" is entered as H, and the verdict is marked as a lower bound for that player. An output that never happened is treated the same way; any further penalty for non-performance is outside this rule.

**Sole repairs.** The verdict also lists each unit whose compliance alone would have removed the miss. It does not list all minimal sets of units whose compliance would have done so: deciding whether a given unit belongs to one is as hard as subset sum, which both reviewers confirmed.

## The worked cases

| Case | Facts | Outcome |
|---|---|---|
| The first pass's run | U3 late by 0.8, U2 late by 0.5, no slack, miss 1.3 | Shares 0.8 and 0.5. Neither is a sole repair |
| Slack | U3 late by 0.5, U2 late by 2.0, slack 1, miss 1.5 | U3 alone would have caused no miss and U2 alone a miss of 1.0. Shares: U3 0.25, U2 1.25. U2 is a sole repair. U3 is charged because, given U2's lateness, U3's added half a day to the miss |
| An early unit | U4 late by 1.0, U3 late by 0.3, U2 early by 0.5, miss 0.8 | U2's earliness is kept. U4 is a sole repair. With clipped residuals the draft said U4 and U3 were both needed |
| A slow link and a late unit | U3 late by 5.0, the last link slow by 0.5, no slack, miss 5.5 | Shares 5.0 and 0.5, the second paid by the last link's sender without a finding. The draft put the whole loss on that sender |
| An early sender and a slow link | U3 a day early, its link a day slow, U2 late by 0.5, miss 0.5 | U3's combined residual is zero and it pays nothing. U2 pays 0.5 and is a sole repair. The draft charged U3 everything |
| A late unit on a branch that did not matter | A branch arrives days before the other and one of its units was 0.1 late | Its marginal addition is zero in every order, so its share is zero |
| A compliant sender and a late receiver | Sender's link slow by 1, receiver late by 2, slack 1, miss 2 | Sender 0.5 without a finding, receiver 1.5 |

## Checked by a model

The verdict is implemented in [verdict.py](/root/mic/model/verdict.py), about seventy lines. [test_verdict.py](/root/mic/model/test_verdict.py) runs the cases above and the counterexamples from both reviews, twelve in all, with exact arithmetic; all give the outcomes stated here. Two further cases cover a missing unit and censoring at the horizon. On 300 random chains it also checks three properties: with every positive residual set to zero the deadline is met, the shares sum to the miss, and no share is negative. All hold. The model does not cover the evidence scheme.

One consequence the model made visible: when a receiver's summary is missing, the lateness of the link into it moves from the sender's account to the missing unit's. A compliant sender with a slow link is better off if its receiver fails to report.

## What can be said for it

- **A function of the evidence.** The verdict depends only on the certified manifest, the closed summaries it lists and the public contracts.
- **No finding without the unit's own evidence.** A finding rests on the unit's own closed summary, while its witness bound holds. This protects against being named. It does not protect against paying: a link's bearer pays on two units' evidence, and a missing unit pays on the collector's.
- **Zero for no effect.** A player whose residual changes the miss in no combination pays nothing.
- **No cliffs.** The miss function is continuous in the residuals, and so are the shares.
- **Cost.** Computing the shares directly takes time exponential in the number of players. I do not know a faster method for this function. The plan needs a bound on players per property, with a declared fallback beyond it.

## When the rule is not a fiction

For controllers that take a fixed time per output from the moment their last input arrives, on a tree with fixed link delays and no resource shared between outputs, the model's counterfactuals are what re-running the controllers would give. The argument is an induction over the tree. The class excludes controllers that work to an absolute deadline, batch or queue their work, or schedule several outputs on one resource. It is narrow, and it excludes many of the cases a causal verdict is wanted for. For everything else the verdict is what the declared rule says, and a different rule would give a different verdict.

## The decisions

| | Decision | Choice | What it costs |
|---|---|---|---|
| D8 | Witnesses and logs | 2f+1 witnesses with a delivery bound among them, deciding fixed slots by a published agreement protocol; decisions final; inputs entered by witnesses | A new assumption; f+2 rounds per slot; no rejoining rule |
| D8, D15 | Finality between units | A frontier at deadline plus horizon, then a certified manifest at a cutoff | Lateness beyond the horizon is not measured; a summary that arrives late counts as missing |
| D9 | Form of the claim | The verdict is a function of the manifest, produced within a bound after the cutoff | Nothing is claimed before the cutoff |
| D10 | Discharge | One certified time per event, stamped by the receiver's witnesses on first receipt | Certified time is the contractual time, with its error |
| D15 | Loss | Shares by average marginal addition to the miss, over units, links and missing units | Exponential to compute; a convention |
| D16 | Collection and dissolution | The receiving unit's witnesses collect and certify the manifest; a unit cannot dissolve with a commitment outstanding unless its creator accepts it | If the collecting unit's witness bound fails, the manifest cannot be trusted |

## R2′

For a property whose contracts form a finite acyclic tree with one response bound per output event and readiness when all inputs have arrived, with bounds and allowances summing to no more than the deadline on every path, and while each unit's witness bound holds:

- (a) if the property fails, the shares are defined and sum to the miss, as measured up to the horizon;
- (b) a unit with no positive residual in its own closed summary pays nothing as a unit, though it may pay as the bearer of a link or as a missing unit;
- (c) the verdict is a function of the certified manifest, the summaries and the public contracts;
- (d) for the narrow class above, the verdict agrees with re-running the controllers.

All four follow from the definitions, the agreement protocol and an induction on the tree. None is a new result.

## How this differs from the plan's target

The plan asked for sets of deviating parties that suffice for the failure. Take two units each one day late against a slack of one day. Making either one compliant removes the miss, yet keeping only either one's lateness also gives no miss, so the only set sufficient for the failure is both together. Repairs and sufficient sets are different objects, and the shares above are a third. The plan's stop rule says a statement repaired after a counterexample is a new conjecture. R2′ is that, and the plan's R2 remains without a truth value until a relation between actual and counterfactual runs is fixed for it.

## What this leaves open

- Guarantees that are not about time. A shortfall and a miss have to be defined for each kind, and the miss need not grow with the residuals.
- Joins other than "when all inputs have arrived", and outputs that share a resource.
- Rejoining and replacing witnesses.
- A faster way to compute the shares, or a proof that there is none.
- A penalty for outright non-performance.
- Whether paying on a missing summary, or carrying the timing risk near a deadline, is acceptable to the people the foundation would govern.

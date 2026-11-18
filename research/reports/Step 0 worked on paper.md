# Step 0 worked on paper

**Status.** This is the first execution of the plan in [Integrated foundation plan.md](/root/mic/reports/Integrated foundation plan.md): its design decisions worked against their cases, and its central target (accountability under delegation, R2) attempted on a three-unit chain. The first draft was attacked once by two independent reviewers ([step0_attack_astra.md](/root/mic/reports/step0_attack_astra.md), [step0_attack_fable.md](/root/mic/reports/step0_attack_fable.md)). Neither found a result false beyond repair, and both found several wrong as stated. This version is corrected against those reviews and has not been reviewed again. The arguments are sketches, not mechanised proofs.

## Results after review

1. **A consistent set of decisions has not been demonstrated.** The first draft claimed one. The reviewers found three pairs of choices that contradicted each other or the benchmark, and two places where a choice leaves an open problem. The repairs are recorded below; two questions remain open (how a translated norm gets its priority, and when a verdict is final).
2. **Blame by broken guarantee composes from per-unit summaries,** once the verdict is defined as the set of all such findings on the dependency tree of the failed property. This is reuse of known assume-guarantee reasoning.
3. **Blame by actual difference does not compose from boundary summaries.** The first draft's counterexample had wrong arithmetic. A corrected pair of histories shows the narrower point: boundary events do not determine how a controller would have responded to an input it never received. Under the trace-based definition the plan cites, this does not refute the plan's target.
4. **The witness argument proves less than I claimed.** At most one log head per sequence number can be certified. That does not give one consistent log, agreement on what was certified, or final verdicts.
5. **How much of the candidate is new remains a judgement.** The ingredients are known. What is unproven is that they fit together, which is what a foundation would have to show.

## The decisions

### D1, D2, D7: conflicts, satisfiability, obligations

**Kept.** A norm is a contract: a guarantee with a named assumption. Conflicts are resolved by a priority-ordered selection, and a norm in the effective set stays owed as time passes. A unit that lets its options run out has deviated; a unit whose options were removed by the environment has not.

**Corrected.**

- *The satisfiability test.* The draft required a way of acting that keeps every guarantee "whenever all the assumptions hold". That accepts any set whose assumptions can never all hold together. The test is: one way of acting under which each guarantee holds whenever its own assumption holds.
- *What assumptions may mention.* An assumption such as "some vendor can deliver" cannot appear in a summary, because nothing certifies it. Obligations are restated as obligations to act on certified inputs ("place an order with a permitted vendor by day 8"), with the vendor's delivery a separate contract. An assumption counts as met unless a certified event shows it broken.
- *When the selection is re-run.* Not only when a norm is issued. It is re-run at every change to the effective norms: issue, withdrawal, a change of path or translation, a change to the priority table, a change of authority. The passing of time and a unit's own loss of options do not trigger it. A later prohibition that removes the last compliant plan is a rule change, with the loss on the authority that issued it.

**Cost.** Deciding whether one way of acting exists is harder than checking for contradiction, and this is untouched.

### D3: priority among authorities

**Kept.** A total order, with edges allowed only from higher to lower rank so that each declarer can check acyclicity alone.

**Corrected.** The draft set a unit's level by its depth of creation. That put the ministry (created by the state) below the group (a top-level unit) and so forbade the benchmark's own edge from ministry to group. Rank is separated from creation depth: the anchor assigns each top-level unit a rank from an ordered domain with gaps, a created unit takes a rank strictly below its creator's, and a declarer needs the target's certified rank. Changing the rank of a subtree is a rule change by the anchor.

### D4: which norms bind a unit

**Kept.** All ancestors' norms, each with its origin, each reaching a unit by one named path.

**Open.** A translating authority can pass off its own tightening as the origin's norm and so give it the origin's priority. Three repairs are possible and each has a cost: give a translation the lower of the two priorities, which loses the state's norm again in the case this decision was meant to fix; require a certified exact translation and refuse edges without one; or have the origin sign the translation. This is not settled.

### D5: commitments

**Kept.** Commitments are not norms in the selection, and a commitment hit by a later norm is grandfathered or recorded as frustrated with a named bearer.

**Corrected.** Keeping commitments out of the selection does not remove their constraints. A new commitment has to be checked on admission against the selected norms and the outstanding commitments together, including scarce things other than money such as one machine slot. A norm found to breach its authority's charter is void from its issue for the purpose of verdicts and from the finding for effects; this is the one declared case in which a verdict is revised.

### D6: caps

**Kept.** A cap is a prohibition that takes part in the selection, and spending always needs rights from the ledger.

**Corrected.** Carrying out an override "as a raising of the cap" does not reserve the money: an unrelated commitment can take it first, and deciding the override later is evaluating at the current state, which D1 rules out. An obligation to spend must state a maximum amount. When it takes effect, the ledger reserves that amount, and any raising of a cap is fixed at that moment as the least that admits the reservation. If the amount cannot be reserved under every cap its origin does not outrank, the obligation is left out then.

### D8 and D9: witnesses, logs and verdicts

**What holds.** With 2f+1 witnesses, a delivery bound Δ among them, and the rule "forward the head, wait a little longer than Δ, sign only if no different head for that sequence number has been seen", at most one head per sequence number can reach f+1 signatures. Both reviewers confirmed the timing. One identified it as a published voting rule for synchronous replication, so it is reuse.

**What does not follow.**

- *One log.* Uniqueness per sequence number is not consistency across them. Each head must carry the hash of its predecessor, and a witness signs a head only if it signed that predecessor.
- *Agreement on what was certified.* Honest witnesses can end in different states: one signed, one refused. A faulty witness can then hold back its signature and complete the certificate later, after a verdict. Closing this takes further rounds of forwarding among the witnesses, or a time bar.
- *Fresh summaries.* Two validly signed summaries of different prefixes of the same log can exist, and presenting the older one can move a finding to another unit. A summary needs a certified coverage time, signed only after the deadline it covers plus the bound and the skew.
- *Evidence in transit.* Links between units have no delivery bound, so the adjudicator can wait without limit for a unit's summary and cannot tell a late one from a withheld one.

**Consequence.** The choice the plan listed, between verdicts that are never final and an explicit time bar, is not removed. The draft was wrong to say it was. The verdict claim in D9 is a bounded response measured from the moment every required closed summary is present, and that moment has no bound.

### D10: discharge through witnesses

**Corrected.** A certified receipt time is accurate only to within the delivery bound plus the clock skew, so it cannot settle a sharp deadline. There are three outcomes: certainly in time, certainly late, and too close to call, with the last allocated by a declared rule. Contracts carry that margin. The receipt goes through the same sequence-numbered echo as log heads, so that one time per performance is certified. "Sent by" and "received by" obligations are kept apart.

### D11: the ledger

**What holds.** Both reviewers checked the arithmetic of the four categories and found no error, provided each amount sits in exactly one category and settlement releases only what is shown to be unused. New rights are at most the current cap minus the total, or zero.

**Corrected.** "A raised cap frees the difference at once" is true only when the total is within the old cap. Where rights from an earlier, higher cap are still outstanding, a raise may free nothing. The sharper true statement is that the total never exceeds the cap in force at the most recent issue. Obligation reservations from D6 belong in "committed".

**Standing.** The numeric part is the escrow method, and the rule that a bound can be moved at once in the safe direction and only after settlement in the other is also published. The bound on grandfathered commitments still needs its own argument.

### D12: creation

**What holds, conditionally.** If every finite prefix of every admitted run fits in some large enough pool, a safety property that holds in all pools holds in the limit. This needs runs with only finitely many creations in any bounded time, and bounds that do not depend on the pool size. Until that embedding is shown, pool results are results about bounded populations, as the plan says.

### D13: availability

Measured per fixed graph and workload, with the worst case reported as zero. Unchanged.

## The accountability target on a three-unit chain

### Setting

U1 orders from U2, and U2 orders from U3. Each hand-over takes exactly one day on the link.

- U3 guarantees: hand over for U2 by day 4, assuming U2's order was received by day 1.
- U2 guarantees: send its order to U3 by day 0; hand over for U1 within 3 days of receiving the goods; and hand over by day 8, assuming the goods were received by day 5. All assume U1's order was received by day 0.
- The chain property: U1 receives the goods by day 9, given that U1 ordered in time.

Every assumption is either a named guarantee of another unit or a declared assumption about the environment with a bearer. The contracts are finite and acyclic, and each guarantee's deadline exceeds those it depends on by at least the link allowance plus the certification margin.

A unit's summary has one entry per guarantee: the certified time of its own hand-over or the certified fact that there was none, and the certified times of the receipts its assumptions refer to.

### Result 1: blame by broken guarantee

**Definition.** A finding against a unit is a guarantee that its own certified log shows broken, at a time when its own certified receipts show that guarantee's assumptions met. The verdict for a failed property is the set of all findings on the dependency tree of that property, together with any hand-over made in time and received late (an allocated loss) and any broken environment assumption.

**Claim.** If the chain property fails, the verdict computed from the three summaries contains at least one of those three kinds of entry, and it never contains a finding against a unit whose own certified log shows no breach.

**Argument.** Walk the dependency tree from the failed property. At each guarantee, the owner's summary shows it kept, broken with its assumptions met (a finding), or broken with an assumption unmet, in which case follow that assumption to the guarantee or environment assumption it names. The tree is finite, so the walk ends. Breaches that are not on the tree are reported separately and are not findings for this property.

**Limits found in review.**
- More than one entry can be present. If U3 hands over half a day late and the last link is also slow, the verdict holds both U3's finding and the link loss. The draft's "exactly one" was false.
- Certified times carry a margin, so a compliant unit close to a deadline can be found against unless every deadline is measured with that margin.
- "Never names a compliant unit" holds only while the named unit's own witness bound holds and its log records authenticated inputs.
- This is not a computation of who caused the failure. It is a list of who broke what.

**Standing.** Reuse.

### Result 2: blame by actual difference

**Corrected counterexample.** In the actual run the goods reach U2 on day 5.8 and U2 hands over on day 9.3. Take two rules that U2's controller might follow, both producing that run:

| | U2's rule | If U3 had been on time (goods on day 5) | U3's lateness |
|---|---|---|---|
| History 1 | Take 3 days if the goods arrive by day 5, otherwise 3.5 days | Hand-over on day 8, receipt on day 9, the property holds | Made the difference |
| History 2 | Hand over on day 9.3 whenever the goods arrive | Hand-over on day 9.3, the property fails | Made no difference |

The boundary events are identical, so the summaries are identical, and whether U3's lateness was a cause differs.

**What this shows.** For any definition of blame under which the verdict depends on how a unit would have responded to inputs it did not receive, the verdict is not determined by boundary events. If the controller's rule is public and certified, the two histories are distinguishable without inner logs, so this is not an impossibility in that setting.

**What it does not show.** The plan cites a definition of accountability in which the verdict is computed from the trace, the protocol and a fixed relation between the actual and counterfactual runs. Under that definition the refuter does not fire as stated. The plan left the causal definition to step 0, and this result says only that the choice decides whether the target is true.

**A caveat withdrawn.** The draft said that under a weaker reading the verdict depends only on who broke a guarantee. That is false. Two parties who each send a forbidden bit have broken the same guarantees whether the output is their conjunction or their disjunction, and the minimal sets that suffice for the failure differ. Any causal verdict needs the public model of how the units compose, and local breach labels are not enough.

### Result 3: a conjecture, restated

Under blame by actual difference, the adjudicator can ask each unit's witnesses what the unit would have done with one input changed, and they can answer by replaying its controller.

For this to be a well-posed question it has to be stated for a class: controllers that follow their recorded rule and are deterministic given recorded inputs; a counterfactual that changes one input and holds every other recorded input and every link delay fixed; and an adaptive sequence of queries with a bound on rounds. It is ill-posed for a controller that departed from its rule, since replaying the rule does not reproduce it. For a finite acyclic chain of such controllers the construction is likely routine. The first thing to test is whether the number of queries is polynomial in the number of broken guarantees.

## What this does to the plan

| Target | After this work and its review |
|---|---|
| R1, authority analysis | Untouched and open |
| R2, accountability | True as a list of broken guarantees on the dependency tree, which is reuse. Whether a causal verdict composes depends on the causal definition, which must be fixed before the claim means anything |
| R3, conformance | The step from "complies with each surviving norm" to "complies with the effective set" is trivial. Still open: how large a certificate of the selection each edge needs, how a translation gets its priority, how context is exported between levels, and whether conflicts can be found within a bound, which the choice in D1 and D2 makes harder |
| R4, rule change | The numeric part is the escrow method. The bound on grandfathered commitments is not argued |
| R5, entrenchment | Not attempted |

**Against the stop rules.** The plan continues only if R2 survives and R1 or R3 improves on its baseline. R2 survives in a form that is reuse; R1 and R3 are not yet tested. The paper stage is therefore not finished, and nothing here justifies either stopping or going on to mechanisation.

**What is open, in order of how much depends on it.**

1. The causal definition for R2. Until it is fixed, the central target has no truth value.
2. Finality of verdicts: further forwarding rounds among witnesses, or a time bar.
3. The priority of a translated norm.
4. Whether joint satisfiability can be decided for a grammar large enough for the benchmark.
5. The authority analysis, R1.

## What the two reviews of this document establish

- The single-slot witness lemma, the ledger arithmetic and the limit argument for pools were checked and hold under their stated conditions.
- Every claim of the form "this problem no longer arises" in the first draft was wrong.
- Working cases found errors faster than reviewing the plan's prose did: one pass over this document produced thirteen high findings, most with an explicit small history. That supports doing the rest of step 0 this way, with a review after each group of decisions.

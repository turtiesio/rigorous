# Adversarial review of "Plan for an integrated foundation", version 6 (fable, round 6)

**Method.** I read the current plan and formed findings, then diffed it against version 5, then read the two round-5 reviews to drop repeats and check that the round-5 fixes were applied faithfully. I did not read any round-6 review by another reviewer. Each finding is labelled:

- **verified**: checked against a source fetched in this round.
- **plan-internal**: follows from the plan's own text, by argument or calculation.
- **judgement**: my own assessment.
- **could not verify**: stated from memory, or the source could not be fetched.

**Overall.** No CRITICAL defect. One HIGH defect, in text written in this pass: the rule that an obligation left out of the selection "is not owed", combined with conflicts defined relative to the current state, lets a unit escape any obligation by its own conduct. Twelve MEDIUM and four LOW defects follow. Most MEDIUM items are round-5 suggestions that were copied in without being checked against the rest of the plan. The plan is sound enough to start step 0 if the HIGH item is settled as a design decision before the freeze.

---

## HIGH

### H1. "An obligation left out is not owed" plus state-relative conflicts makes violation escapable by the unit's own conduct

**Text attacked.**
- Effective norms: "A conflict is a minimal set of norms that cannot be jointly satisfied from a given state".
- "A unit's effective norm set at a state is the result of a prioritised selection".
- "because the priority order on norms is total it leaves no conflict undecided".
- "An obligation left out is not owed, and an obligation that selection keeps and the unit misses is a violation under R2 like any other."

**Why it is wrong (plan-internal).** The conflict is evaluated at the current state, and the state includes the unit's own past choices. Two cases follow from the text as written.

1. *Self-induced conflict.* The subsidiary owes a low-priority obligation (buy compute by day 10) and is under a higher-priority prohibition (never buy from vendor V2). Until day 9 it could buy from V1. It waits. On day 9.5 only V2 can deliver in time. From that state the pair cannot be jointly satisfied, so it is a conflict, the selection drops the lower norm, and the obligation "is not owed". No violation exists, so R2 has nothing to classify, not even an allocated loss.
2. *Conflict of one.* An obligation that can no longer be met from the current state is, alone, a minimal set of norms that cannot be jointly satisfied. The selection must decide it, and the only way is to leave it out. Just before any deadline is missed, the obligation is unsatisfiable from the state, so it is dropped and not owed. Read literally, "an obligation that selection keeps and the unit misses" is an empty class, and R2(a), R2(c) and the conclusion of R3 are vacuous for obligations.

Version 5 recorded "the authority that caused each pending conflict". Version 6 keeps blame for the authority side only ("An authority that issues a norm its own charter forbids has deviated") and has no rule for a conflict the constrained unit caused. Round 5 (astra, finding 2) made the general point that a conflict is not evidence that nobody deviated. The edit answered half of it.

**Why this is HIGH and not a step-0 detail.** The repair needs a choice between designs, and each choice changes a target or a cost.

- *Evaluate conflicts when a norm is issued.* Conflicts that arise later are then not decided by the selection, which contradicts "leaves no conflict undecided".
- *Define compliance as staying inside the set of states from which the selected norms remain jointly satisfiable, and treat the step that leaves that set as the violation.* This is the sound reading. It makes conflict detection a question about what the unit can guarantee against its environment, which is a realizability question and not a satisfiability question (see M6). It changes the R3 detection bound and the per-decision cost in step 5.
- *Keep the present rule and add causal attribution for self-induced conflicts.* This imports R2's causal definition into the definition of the effective norm set, which the R3 local check would then depend on.

**Evidence.** The argument uses only the plan's text. The published selection procedures the plan cites work on a fixed context of facts and have no notion of a state changed by the agent's own actions (verified for Liao et al., whose system is a set of literals, conditional norms, a context and a rank function, with no time or state: https://arxiv.org/pdf/1709.08034).

**Change recommended.** Decide before the freeze. My recommendation is the second option:
- "A norm is left out only if it conflicts with higher-priority norms at the state where the later of them takes effect. A unit that moves from a state where its effective norms are jointly satisfiable to one where they are not, when a compliant move was available, has violated the lowest-priority norm of the new conflict at that step."
- "A single obligation that has become unsatisfiable is a violation, not a conflict."
- Add both cases above to variant 10 and to the immediate checks.

---

## MEDIUM

### M1. Invariants "not subject to selection" let a lower-priority cap defeat a higher-priority obligation

**Text attacked.** "Invariants on a resource that are kept by rights allocated in advance, such as a spending cap, are not subject to selection: a commitment that would breach one is refused when it is made." First section: "Where two authorities conflict, the lower-priority norm is overridden".

**Why (plan-internal).** The cap is the ministry's norm (variant 9). Suppose an authority that outranks the ministry obliges a purchase that would exceed the cap. The commitment is refused, so the higher-priority obligation cannot be met. Either the obligation stays in the effective set and a compliant unit violates it, or it is dropped as a conflict of one (H1). In both readings the lower authority has overridden the higher one outside the selection. The effective norm set is then not jointly satisfiable together with the exempt invariants, which is the consistency property the selection was meant to give. The plan also does not say who may declare an exempt invariant. If a unit can cap its own resource, it can make any payment obligation impossible.

**Fidelity.** This is round 5 (fable M2), second option, copied as written. The reviewer did not check it against priorities and neither did the editor.

**Change.** State that exempt invariants form a tier above all norms. Restrict who may declare one to an authority over the resource's owner. Require that a higher-priority obligation which meets a lower-priority cap is recorded as an override of the obligation, with a named bearer, and add the case to variant 10. Narrow the first section's guarantee to "conformance to the resolved set, subject to the exempt invariants".

### M2. R2(a) is now a liveness property, which contradicts the premise of the pool argument, a non-goal and the assume-guarantee premise; the new convergence refuter can hardly fail

**Text attacked.**
- R2(a): "every covered violation caused by a deviation eventually yields a proved-deviation verdict".
- New metatheory: "Every property the plan claims is decided on finite prefixes of a run".
- Non-goals: "No general liveness."
- Controlled: the rule's premises "include that the discharged assumptions are safety properties".
- Assumptions: "every message is eventually delivered".
- R2 refuter: "verdicts that do not converge once all the evidence has been delivered and processed".

**Why.**
- *Plan-internal.* "Eventually yields" is not decided on any finite prefix. In version 5 the verdict had a challenge period, so the round-5 reviewer's premise ("every property the plan claims is a deadline or prevention property") was true when written. The editor made R2 eventual and copied the premise in the same pass. Closure and substitutivity under trace inclusion, which is what the pool argument would establish, do not carry an eventuality.
- *Plan-internal.* "Every message is eventually delivered" is a liveness assumption about the environment. The plan's own stated rule discharges only safety assumptions.
- *Judgement.* If the verdict is a function of the set of evidence received, convergence "once all the evidence has been delivered and processed" holds by construction. The refuter can fire only if the verdict depends on arrival order. It does not test the thing that matters after a fork, which is how long a wrong provisional verdict stands.
- *Could not verify* how the timed I/O automata monograph treats liveness; the search result says only that safety and liveness properties are both defined (https://sws.cs.ru.nl/publications/papers/fvaan/RTSS03.html). The definition of safety by finite prefixes is Alpern and Schneider (from memory): https://www.cs.cornell.edu/fbs/publications/DefLiveness.pdf

**Change.** Restate R2(a) as a bounded response to a delivery event: "within a stated bound after the evidence is logged at the unit that computes the verdict, the verdict names the deviators". That is a safety property in a timed model. Restate the refuter as "two orders of arrival of the same evidence that give different verdicts, or a verdict later than the bound". Then either drop "every message is eventually delivered", which no safety claim uses, or list the eventual claims separately as outside the pool argument and the assume-guarantee rule.

### M3. "Final" is not observable by any party and is unsound when evidence is withheld and released later

**Text attacked.** "it is final only when every link on the forwarding paths has met the bound of the performance envelope. Evidence needed for a provisional verdict is retained until the verdict is final." Step 0: "the performance envelope under which a verdict is final".

**Why (plan-internal).**
1. No party can tell that every link has met its bound for evidence it does not yet know exists. Links between units have no delivery bound, so finality is a property of the run and not something a controller or witness can act on. The retention rule keyed to it therefore never releases anything, and "log growth as a function of activity" in step 5 has no finite bound.
2. Link bounds limit transit time. They do not limit when a faulty holder chooses to send. In variant 8, "only a colluding recipient saw the effect, which must come out as outside coverage". If that recipient later defects and presents its valid receipt after the verdict is "final", either the verdict is revised, so it was not final, or valid proof of a deviation is ignored. If the evidence has been discarded by then, the receipt cannot be checked against the log.

**Fidelity.** Round 5 (fable H1) option 3 kept a challenge period and stated finality inside the envelope. The editor dropped the period and kept the finality clause, which leaves no clock that anyone can read.

**Change.** Either delete "final" (verdicts are never final, retention is unbounded, and step 5 reports that cost), or add an explicit time bar: "evidence presented more than T after the event is inadmissible for revising a verdict; the loss stays where it was allocated". State which. Add the late-defecting colluder to variant 8.

### M4. R3 as worded has a counterexample in the benchmark's own authority graph

**Text attacked.** R3: "Compliance of a unit's rules with one authority is defined relative to that authority's norms that survive selection. The target: a local check at each authority edge ... together with enforcement at the leaf, implies that the leaf's behaviour complies with its effective norm set." Effective norms: "a prioritised selection over the norms of all its authority ancestors. Each norm carries its origin and is counted once, at its origin's priority".

**Why (plan-internal).** The effective set of a leaf contains the norms of every ancestor, but the checks are on edges and each covers only the norms of that edge's authority.

| Element | In the benchmark |
|---|---|
| Ancestor with no direct edge to the leaf | the state, with norm N |
| Intermediate authority | the ministry, which implements N as a more specific norm N' on the subsidiary |
| Second direct authority | the regulator, with norm R that conflicts with N' and not with N |
| Priority (rank, with the anchor's setting among ministry and regulator) | state, then regulator, then ministry |
| Selection at the subsidiary | N kept, R kept, N' dropped |
| Edge check ministry to subsidiary | passes: N' did not survive, so nothing to check |
| Edge check regulator to subsidiary | passes if the subsidiary's rules satisfy R |
| Result | the subsidiary's rules may break N, which is in its effective set; no edge check covered N |

Two further gaps in the same text:
- An implementing norm has the implementer's origin and priority, so it loses to authorities that its source would beat. The plan does not say whether a translation of N carries N's origin or the translator's.
- Variant 10's "ministry norm that reaches the subsidiary ... with different translations" gives two norms with the same origin and the same time of issue. The order "rank, then identifier, then time of issue" does not separate them, so the claim that the order on norms is total fails in a case the plan itself lists.

**Fidelity.** The local-check wording is round 5 (fable M3) and the origin rule is round 5 (fable M5). Each was copied without checking it against the other.

**Change.** Choose one and freeze it:
- (i) only the norms of direct authorities enter a unit's effective set, and an ancestor's norm binds only through each intermediate authority's implementation, as in the multi-level baseline; or
- (ii) the check at an edge covers every ancestor norm routed through that edge, a faithful translation keeps its source's origin, and a path index breaks ties between translations.

Option (ii) raises the per-decision cost in step 5. Add the table above to the immediate checks.

### M5. "Enforcement at the leaf" cannot imply compliance with obligations whose performance depends on the environment

**Text attacked.** R3: "together with enforcement at the leaf, implies that the leaf's behaviour complies with its effective norm set". Controllers: "other components send requests, and only the controller emits the effect". First section: "Humans, learned agents and third-party services are environments here."

**Why.** A controller that mediates requests can refuse an action. It cannot make the officer approve before a deadline or stop the clock. Mechanisms that monitor execution and halt it enforce only safety properties, and later work adds that actions the mechanism can observe and not control (the passage of time is the standard example) limit this further.
- *Verified at the level of lecture notes and abstract:* Schneider, https://www.cs.cornell.edu/courses/cs513/2000SP/NL18.html
- *Could not fetch; reported from a search snippet:* Basin et al., "Enforceable Security Policies Revisited", https://link.springer.com/chapter/10.1007/978-3-642-28641-4_17

So for obligations R3's conclusion is false whenever fulfilment needs an input from a human or agent, unless the theorem carries an assumption about that input.

**Change.** State the conclusion separately: prohibitions and permissions are enforced; for obligations the claim is "every miss of an obligation in the effective set is detected and classified under R2", or compliance holds under a named environment assumption for each obligation.

### M6. The inherited consistency result does not apply to the plan's norms, and "jointly satisfied" hides a choice with a large complexity cost

**Text attacked.** R3 baseline: "consistency and uniqueness of the selection under a total order are inherited from the chosen published procedure (Liao et al.)". Effective norms: "cannot be jointly satisfied from a given state". Step 5: per-decision cost includes "the cost of conflict detection and selection".

**Why.**
- *Verified.* Liao et al. treat a finite set of conditional norms over literals with a fixed context. Consistency means no literal and its negation both appear. There are no deadlines, quantities, states or permissions. Uniqueness under a total order is proved for the Greedy and Optimization procedures (Lemmas 1 and 2); Reduction "may have a unique extension, multiple extensions, or an empty extension" under a total order. Source: https://arxiv.org/pdf/1709.08034. A result about literal consistency cannot be instantiated for "satisfiable from a state of a timed open transition system", so the reuse-ledger entry cannot be written as described. Uniqueness of a greedy sweep over a total order is immediate and needs no citation; consistency in the plan's sense has to be proved.
- *Judgement.* A unit is an open system. "Can be jointly satisfied" means either that some behaviour of the environment allows it or that the unit can ensure it whatever the environment does. The second is what makes an obligation fair to impose, and for temporal specifications it is realizability, which is far more expensive than satisfiability. *Could not verify* the exact class; I recall doubly exponential time for linear temporal logic from Pnueli and Rosner (https://doi.org/10.1145/75277.75293), and the search did not confirm it. The selection is recomputed "at a state", so this cost is paid per decision unless the grammar is very restricted.

**Change.** In the R3 baseline, replace "inherited" by "uniqueness is immediate; consistency for the frozen grammar is to be proved". In step 0, freeze which reading of "jointly satisfied" is used, and add to the R3 refuter "detection within the frozen bound under that reading".

### M7. R4 is false for two cap changes inside one lease period, and for one change it is a two-line invariant

**Text attacked.** R4 refuter: "total spending at any intermediate state exceeds the larger of the old cap and the new cap". R4: "New rights are issued against the new cap minus all three, floored at zero". Step 0: a bound on the grandfathered set "as a function of the lease period and of the commitments and rights outstanding at the change".

**Why (plan-internal, by calculation).**

| Step | Cap | Rights held by the late shard | New rights issued | Spending possible |
|---|---|---|---|---|
| Start | 100 | 100 | | 100 |
| First change | 50 | 100, lease not expired | max(0, 50 − 100) = 0 | 100 |
| Second change, same lease period | 60 | 100, lease not expired | max(0, 60 − 100) = 0 | 100 |

For the second change the old cap is 50 and the new cap is 60. Spending of 100 exceeds the larger of the two, so the refuter fires on a correct ledger. The bound that holds is the largest cap of any version that still has unexpired rights. Variant 9 has only single changes and would not catch this.

For a single change the claim follows at once. Before the change, spending plus commitments plus rights is at most the old cap. The amount issued is at most the new cap minus that sum. So the sum never exceeds the larger cap. The bound on the grandfathered set is "a function of the commitments and rights outstanding", which is satisfied by the outstanding amount itself. With the single ledger, R4 is the escrow method applied to a versioned bound and belongs with the application lemmas, as R6 now does.
- *Verified at the level of a search summary:* O'Neil, "The Escrow Transactional Method", https://doi.org/10.1145/7239.7265
- Already in the reuse list: Balegas et al., https://arxiv.org/abs/1503.09052

**Change.** Restate the bound as "the largest cap among versions with unexpired rights". Add a two-change case to variant 9. Either move R4 to the lemmas, or give it content that can fail: for example a frozen upper limit on how long new spending is blocked after a cap is raised, or a bound on the grandfathered set that is smaller than everything outstanding.

### M8. Discharge by f+1 witness receipts has no timing rule, and the obvious rule allows backdating

**Text attacked.** "receipts from f+1 of them discharge the obligation, timed by their receipt".

**Why (plan-internal).** The f+1 receipts carry f+1 different times. Up to f of the obligee's witnesses may be faulty and may collude with the obligor.
- If the discharge time is the earliest time, or a median, f faulty witnesses can backdate a late performance; the one honest signer's true time is outvoted.
- Only the latest of the f+1 presented times is safe against backdating. With that rule, faulty witnesses can refuse or post-date, so the obligor needs all f+1 honest witnesses, and timeliness is set by the slowest of f+1 unbounded links instead of one.

Two further points are unstated: whether the obligee's controller must then log the performance, and what its replay means if it does not, since its later actions (declaring a breach, clawing back) are correct against its own log and wrong against the witnesses' record.

**Fidelity.** Round 5 (fable L3) asked whether witness time or controller time decides. The edit answered "their receipt" without saying which of f+1.

**Change.** Freeze: "the discharge time is the latest time among the f+1 receipts presented". State that the witnesses then submit the performance to the controller as a recorded input, and that failure to log it within the link bound yields the suspicion verdict already provided. Add a backdating case to variant 8. Count the extra links in the step-5 envelope.

### M9. Ranks: nobody is named to assign them, a new unit has no authority that can declare its first edge, and the anchor's priority table is an unleased dependency of every unit

**Text attacked.** "Every unit receives a rank when it is created, and an authority edge may run only from a higher rank to a lower one". "Authority edges are declared by the anchor or by an authority over the units concerned". "Priority among authorities is a total order derived from rank and then from identifier, which only the anchor can override". Variant 3: units created "in a number that depends on an input from the environment".

**Why (plan-internal).**
1. *Who assigns the rank, and from what set.* Rank fixes priority, so it is a grant of power. If ranks are natural numbers, the depth of an authority chain below the subsidiary is bounded by the subsidiary's rank, which constrains variant 3 ("each of which may create further units"). If they are dense, nothing in the text stops a creator giving a child a rank above the state.
2. *First edge.* A newly created unit has no authority over it. "An authority over the units concerned" therefore cannot declare its first edge, and only the anchor can. Witnesses must also be "appointed by a rule authority or the anchor". Read literally, every creation at run time needs the anchor.
3. *Anchor override.* An override changes the effective norm set of every unit below the affected authorities. It is a rule change made by a component that is not a unit, so R4, R5 and the lease mechanism do not cover it. The availability rule lists "every authority among its ancestors" and omits the anchor's table, which every selection reads.
4. *Rank is permanent.* A unit can never come under a unit of equal or lower rank. The first section mentions acyclicity and not this.

**Fidelity.** Round 5 (fable M9) proposed ranks in one sentence. It was copied without these rules.

**Change.** Add: the creator assigns a rank strictly below its own, or the anchor assigns it; the rank domain is stated; a new unit inherits its first authority edges and its witness appointer from a creation rule; the anchor's priority table is versioned and leased like a rule, and its change is costed in step 5; "rank is fixed for life" is added to the narrowings in the first section.

### M10. The independence assumption for witness appointers is false in the benchmark

**Text attacked.** "That an appointing authority is independent of the disputes its appointees witness is an assumption." Benchmark: "The ministry funds the group by a commitment"; the team and the audit body must "report summaries ... to the ministry"; witnesses are "appointed by a rule authority or the anchor".

**Why (plan-internal).** The ministry is a rule authority over the group and the subsidiary. It is also the counterparty to the funding commitment and the obligee of the reporting duty. If it appoints their witnesses, the assumption fails in the compliant path of variant 1. The other possible appointers are the regulator, which is only over the subsidiary, and the anchor, which is not counted in step 5. Version 5 exercised this in variant 6; version 6 deleted the case and kept the assumption.

**Change.** Name the appointer of each witness set in the benchmark. If it is the anchor, add the anchor's appointment load to step 5. If it is an authority that is party to commitments with the unit, list those commitments as outside the accountability guarantee.

### M11. Whether commitments enter the selection is no longer stated, and R4 and the selection give opposite answers

**Text attacked.** Effective norms: "a prioritised selection over the norms of all its authority ancestors"; "two norms of one authority are ordered by time of issue, later first". R4: "which commitments made under version n must be honoured under version n+1".

**Why (plan-internal).** Version 5 gave "a cap with two commitments" as a conflict of three, so commitments were members of conflict sets. Version 6 removed the example and does not say whether an obligation that arises from a unit's own commitment is a norm in the selection. If it is, its origin is the unit itself, it has the lowest priority, and any later norm of an authority that conflicts with it makes it "not owed"; the counterparty's loss is then not even an allocated loss. If it is not, a later authority norm that conflicts with an outstanding commitment is a conflict the selection does not decide. R4 says the commitment may be grandfathered against the later rule; the selection says later and higher wins.

A related gap: when a norm is later shown to have been issued in breach of its authority's charter, the plan does not say whether obligations that were "not owed" because of it are owed retroactively.

**Change.** State that commitments are not norms; that a norm issued after a commitment is a rule change governed by R4; and that the selection applies only among authority norms. State that removing an invalid norm takes effect from the revision and that losses in between are allocated to the deviating authority.

### M12. The pool argument fails its own criterion for the "useful behaviour" outcomes, and step 1 is still said to be done in the default core

**Text attacked.** "If closure and substitutivity can be proved ... for every pool size and with hypotheses that do not depend on the size, the plan takes that as its core". Useful behaviour: "outcomes that must occur whenever the delivery bounds of a stated performance envelope hold". Step 1: "This stage is done by hand in the default core of step 2."

**Why (judgement).**
- A pool of size n refuses, or cannot perform, the creation numbered n+1. For any n, an environment input in variant 3 that asks for more than n units makes a required outcome fail inside the envelope. The hypothesis "the environment asks for at most n" depends on the size. Prevention properties survive this; required outcomes do not, unless each is restated per prefix ("for runs with at most n creations").
- With R2 eventual, the premise of the argument is false as well (M2).
- If the pool check succeeds, the default is not the core, so paper work done "in the default core" is done in a model the plan then drops.

**Change.** State the pool criterion as: "for every n, every claimed property holds on all prefixes with at most n creations, with no other hypothesis that mentions n". Run the pool check first in step 1, and do the rest of the paper stage in whichever core it selects.

---

## LOW

- **L1. Fork consistency is misnamed (verified).** The plan says "forwarded receipts reveal the fork eventually, which is the guarantee known as fork consistency". In the source, fork consistency means a server that hides one user's operation from another must keep their views forked from then on, and detection happens only "if A and B ever communicate or see each other's future file system operations": https://www.usenix.org/legacy/event/osdi04/tech/full_papers/li_j/li_j.pdf. Eventual revelation comes from the plan's forwarding duty, not from fork consistency. Fail-aware untrusted storage is the closer prior art for eventual detection (abstract level only): https://people.csail.mit.edu/idish/ftp/faust-sicomp.pdf. Reword and remove the dagger once fetched.
- **L2.** Variant 11's third amendment "must be rejected" because the replay period and reference implementation change only with the appointer's consent, but R5's list of entrenched invariants does not contain that rule, so R5's refuter does not cover the case. Add it to the list. Say that the reference implementation is an interpreter of the policy and does not change when the policy is amended, and say what happens when consent is given.
- **L3.** R6 became a lemma and its stop rule was deleted. The lemma can still fail if the frozen summary cannot carry approver and auditor identities within the growth bound. Restore the consequence: constraints across one identity's roles are added to what a positive result does not establish.
- **L4.** "Ordered by time of issue" does not order two norms that different shards of one authority issue within the clock skew, or at the same reading. Add a sequence number from the shard that owns the charter.

---

## Round-5 fixes checked

| Round-5 item | Applied | Faithful and sound |
|---|---|---|
| Challenge period (fable H1, astra 1) | Removed; verdicts eventual | R2(b) now covers provisional verdicts, which is right. New defects: M2, M3 |
| Consent-founded authorities (fable H2) | Removed and listed as not modelled | Faithful |
| Residual rights ledger (fable M1, astra 3) | Yes, with both numeric cases in variant 9 | Sound for one change; M7 |
| Cap as norm (fable M2) | Exempted from selection | Not checked against priorities: M1 |
| R3 local check (fable M3) | Yes | Literal statement fails: M4, M5 |
| Uniqueness (fable M4, astra 4) | Total order; claim moved to reuse | Reuse entry cannot be instantiated as described: M6 |
| Two-path translation (fable M5) | Origin rule and variant case added | Tie between translations remains: M4 |
| Pool criterion (fable M6) | Yes | Premise broken by the eventual R2: M2, M12 |
| R5 dependency (fable M7) | Yes | Faithful. The check is now entrenched. See L2 |
| Availability (fable M8, astra 5) | Yes | Faithful. The anchor's table is missing: M9 |
| Ranks (fable M9) | Yes | Underspecified: M9 |
| R6 (fable M10) | Reclassified; audit law added | Faithful. See L3 |
| Late adjudication (fable M11), pending conflict (astra 2) | Mechanism removed | Authority side handled; unit side is H1 |
| Variant 6 (fable M12) | Clause deleted; assumption stated | Assumption fails in the benchmark: M10 |
| Witness discharge (fable L3) | Yes | Incomplete: M8 |

## Targets against known results

- **R1.** Unchanged this round. Not known true or false until the fragment is frozen. The refuter is real.
- **R2.** Parts (b) and (c) and the composition claim are real and can go either way. Part (a) needs M2. The convergence refuter is close to unfalsifiable. For obligations, coverage is emptied by H1 until that is fixed.
- **R3.** The literal statement has a counterexample (M4) and is false for obligations (M5). With both repaired, the open question, whether the selection can be summarised per edge within a bound, is real.
- **R4.** False for chained changes, immediate for single ones (M7).
- **R5.** Can fail on soundness and on completeness. The dependency on R3 is now stated correctly.

## Gap to the goal

The narrowing is stated honestly. This version narrows further than the first section says in one respect: with ranks fixed for life, a total order from rank, a single root and no consent-founded authorities, "several rule authorities over one unit" reduces to "the highest rank wins unless the anchor says otherwise". That is a strict hierarchy with one exception mechanism held by a component outside the foundation. R6 is now a lemma, R4 is one in substance, and R3 has lost its uniqueness claim. The integrated contribution therefore rests on R2's composition from summaries, on R1, and on what remains of R3. That is still worth testing, and the stop rule already keys on exactly those three. The first section should say that the result, if positive, is about single-rooted ranked hierarchies.

## Order of work

The order is right. Four changes:
1. Decide H1 before the freeze, and with it M1 and M11, since all three are about what the selection ranges over and when it is evaluated.
2. Decide M3 (no finality, or a time bar) before freezing R2, and restate R2(a) as in M2.
3. Run the pool check first in step 1, before any paper work in the default core (M12).
4. Add to the immediate checks, each of which takes under an hour on paper: the conflict of one and the self-induced conflict (H1); the state, ministry and regulator table (M4); two cap changes in one lease period (M7); backdated witness receipts (M8); the receipt released by a colluder after the verdict (M3).

## Verdict

Sound enough to start step 0, on condition that H1 is settled first as a design decision. No target is known impossible, and no central claim about prior art is wrong in a way that invalidates a step. Removing mechanisms did remove the two round-5 HIGH defects, and the fixes for them are faithful. As in every earlier round, the defects are in the newest sentences: the "not owed" rule, the exemption for caps, the finality clause, the eventual restatement of R2 and the rank rule were each adopted in one line and not checked against the selection, the priorities, the fault model or the pool argument. All the MEDIUM items are corrections to statements or benchmark cases that step 0 already schedules.

CRITICAL: 0  HIGH: 1  MEDIUM: 12  LOW: 4

# Plan for an integrated foundation

**Status.** This is a proposal, in its fourth version. It follows from the survey in [Compositional foundations for governed systems.md](/root/mic/reports/Compositional foundations for governed systems.md), which found that small rigorous substrates exist and that nobody has composed them into one foundation with a stated composition law for authority, obligation and liability. Two rounds of independent adversarial review shaped it (files beginning "plan_attack_" in /root/mic/reports). Round 1 found that the theorem targets could not fail. Round 2 found that the threat model made the accountability targets unsatisfiable, that rule authority had been wrongly tied to containment, and that several targets were already known. Round 3 found no critical defect and judged the plan ready to start step 0, with two serious problems in the evidence scheme and the conflict rule. This version fixes those. Citations marked † are from memory and were not fetched; the reviewers checked the others at varying depth.

## What a positive result would and would not establish

A successful run would show that mediated, recorded interactions among units with specified controllers compose with stated guarantees about authority, resources, commitments and rule change, and that the guarantees survive nesting, several rule authorities over one unit, and creation and dissolution of units.

It would not show that a foundation for governing humans, learned agents and institutions exists in the full sense of the original goal. Humans, learned agents and third-party services are environments here. "Self-reflecting" is delivered as rule change checked by the rules in force, with a fixed set of entrenched invariants and a named anchor. "10^10" is delivered as stated growth bounds and a load calculation, not a deployment. Legitimacy, incentives and alignment are outside it. Three narrowings are specific to this design. Rule authority is acyclic, so mutual oversight between units (an auditor that audits the body setting its mandate) is not modelled. Where two authorities conflict, the lower-priority norm is overridden, lawfully and on the record, so the guarantee is conformance to a resolved set of norms and not to every authority. And the results depend on named anchors: an identity issuer, root rules, a time source, and a bound on faulty witnesses. For each theorem that survives, the plan lists the anchors it depends on and reports any theorem that depends on all of them.

Three targets are open research (R1 to R3) and three are narrower open questions (R4 to R6). One piece of metatheory for the core is new work. Everything else is reuse of published results or application lemmas, and the plan reports reuse and novelty separately. A failed target is evidence against this candidate and its encoding, not an impossibility result about the goal.

## Non-goals

- No complete automatic verifier for arbitrary units.
- No unconditional self-certification. Trust anchors are named.
- No optimal decentralised control and no ideal preference aggregation.
- No guarantee about human or learned behaviour outside a mediated interface or a declared envelope.
- No strict Sybil-proofness without an identity anchor. Identity is an assumption of every result.
- No general liveness. Obligations are deadlines in a timed model.
- No confidentiality or privacy guarantee. An observation declaration bounds what is logged, not what a party can learn.
- No incentive compatibility. Sanctions are recorded and executed; whether they deter is not claimed.
- No accountability for effects that only colluding parties ever see.
- No claim about legitimacy.

## Step 0: definitions, assumptions, benchmark and frozen statements

**Unit.** An open transition system with input, output and internal actions, a declared interface, and a clock.

**Principals and roles.** A principal is an identity plus a set of role instances. Each role instance belongs to exactly one unit and each unit has at most one containing unit, so component containment is a tree. A unit can itself be a principal and hold roles in other units. What an identity carries between its own role instances in different units is not mediated.

**Four separate relations.** Membership (which role instances an identity holds). Component containment (a tree). Resource ownership (each resource has exactly one owning unit; other units may hold recorded funding claims on it). Rule authority (which units' rules constrain a unit). Rule authority is a declared directed acyclic relation that is independent of containment, and a unit may have several authorities. Authority edges, the priority among authorities and the powers of adjudicators can be changed only by the anchor or by an authority above all the units concerned, never by the constrained unit.

**Effective norms.** A unit's effective norm set is its authorities' norms resolved by that priority relation. A lower-priority norm is overridden only where it conflicts, and each override is recorded. Where a conflict has no declared priority, the unit refuses the affected actions until a named adjudicator decides, and the refusals count as unavailability. "Valid", "pending" and "rejected" are distinct outcomes.

**Controllers.** Every unit has an atomic controller that owns the effect of every protected action: other components send requests, and only the controller emits the effect. Where load requires it, a unit has a set of atomic controllers (shards). Each shard owns a disjoint set of resources and rules, and a static function assigns every protected action to exactly one shard. An invariant that spans shards is kept only through rights allocated in advance. The assignment is a function of the resource name. One shard owns the charter and the assignment function, and every amendment is a coordinated action installed by lease. An action that needs two shards is a coordinated action and is counted as such in step 5.

**Receipts and witnesses.** Each unit, and each shard, has a witness set of 2f+1 members of which at most f are faulty. Witnesses are appointed by a rule authority or the anchor, never by the unit's own charter. A receipt is accepted only with f+1 witness signatures, so at least one signer is honest and f faulty witnesses cannot block the unit. A recipient has a duty to forward every receipt it relies on to the whole witness set. Witnesses hold the log contents, not only commitments to them, and replay the controller against a reference implementation at a stated period; the time to complete a replay is bounded only under a stated load. Controllers are therefore deterministic given their recorded inputs: requests from humans and learned agents, event order and clock readings are recorded as inputs, and their reasoning is not replayed. Per-unit summaries carry f+1 signatures. Log retention periods are frozen with the other parameters. An external party that cannot check receipts, such as the vendor, is reached through a gateway that is a unit with its own controller and witnesses; what the vendor does behind the gateway is environment.

**Fractal.** Closure, substitutivity under stated hypotheses, and a uniform governance interface: every unit exposes authority, resources, commitments and norms, a log, and a rule-change procedure.

**Controlled.** A list of named properties, each with a formal statement and an assurance record with six fields: mechanisms, assumptions, what is observable, detection time, intervention available, and what is left unprotected. The mechanism is recorded on two separate axes: prevented, certified or detected; and enforced or assumed. A composite's record is derived by rules that are candidates to be checked, not an algebra already known to be sound. An interface assumption is removed only through a sound assume-guarantee rule, which exists for safety guarantees of the form "held until time t" and not for circular promises of eventual action ([Abadi & Lamport](https://lamport.azurewebsites.net/pubs/abadi-conjoining.pdf)). Remaining assumptions and unprotected remainders are united. The composite's observables are what it exports at its boundary after hiding. Detection time is a local bound plus a named delivery term for every link on the path, and that term may be unbounded. Intervention is recorded per observer. Where the observable evidence cannot support a conclusion, or no rule applies, the record is "unestablished". Step 1 exercises the rules on serial paths, alternative paths, a circular pair of deadline promises and an unbounded reporting link, and checks that the derivation gives the same record however the units are grouped.

**Assumptions about faults and the network.**

- Any non-controller component may deviate arbitrarily, and any set may collude.
- Prevention results assume correct controllers.
- Accountability results allow a controller and up to f of its witnesses to deviate, and cover only effects that reach at least one honest party, which then forwards the receipt. An effect seen only by colluders is outside every guarantee.
- Links between a controller and its own witnesses deliver within a stated bound. Vendor links and all links between units, including links to an adjudicator in another unit, have no delivery bound.
- Clocks are synchronised to within a stated skew by an external time source. That source is a trust anchor, since no protocol can synchronise clocks across links with no delivery bound.
- The identity anchor and the root rules are trusted.

**Obligations and verdicts.** An obligation is discharged by a receipt logged at the obligee's controller before the deadline, so a missed deadline is observable locally. There are three kinds of verdict, kept distinct: proved deviation, suspicion (evidence withheld or a party unresponsive, declared by f+1 witnesses), and allocated loss where nobody deviated. Only a proved deviation names a party for the purposes of R2; a suspicion never counts as blame. Lateness caused by an unbounded link is allocated loss borne by the obligor.

**Useful behaviour.** Each benchmark variant lists outcomes that must occur whenever the delivery bounds of a stated performance envelope hold. Refusals outside the envelope are counted as unavailability. A controller that rejects everything inside the envelope fails.

**Smaller.** In steps 1 and 2: definitions and hypotheses counted on paper in a common notation. In step 3: lines of definitions plus axioms and trust anchors in the proof assistant, for the chosen core and the runner-up. Domain definitions, trusted assumptions, reused libraries and proof effort are counted separately. The runtime trust base is inventoried separately. No global minimality is claimed.

**Benchmark family.** A public procurement chain. Containment: a state contains a ministry; a parent group (enterprise) contains a subsidiary (organisation), which contains a team, which contains an approving officer's role (human) and a purchasing agent (learned agent) with its purchasing tool (program). Resources are a budget and purchased compute. A compute vendor is an external service. Rule authority: a supranational body over the state, the ministry over the group, the subsidiary and the audit of ministry-funded purchases through procurement law, the group over the subsidiary through group policy, and an external audit body over the officer's audit role. The vendor is reached through a gateway unit. The ministry funds the group by a commitment. The variants are:

1. The compliant path.
2. A diamond: the officer also holds a role in the audit body, and the budget carries funding claims from two units.
3. Formation: a charter creates the subsidiary, a vote constitutes an authority no delegator held, and one declared amplification (officer approval combined with the agent's budget right).
4. Revocation and claw-back of a partly spent grant, concurrent with a purchase.
5. Creation, a deadline, then dissolution of the subsidiary with a commitment outstanding.
6. Vendor failure with every chain principal compliant, and a disputed delivery decided by a named adjudicating unit.
7. Purchases split to stay under the approval threshold, with the team and the audit body naming the purchase differently.
8. A controller omits a log entry, in four cases: an honest recipient holds a receipt; only a colluding recipient saw the effect, which must come out as outside coverage; a colluding witness countersigns while an honest one audits; and a witness lies in a summary.
9. The ministry lowers the spending cap while purchases are in flight, one controller hears late, and the new cap is below what has already been spent.
10. Group policy and procurement law conflict for the subsidiary, once as two prohibitions and once as an obligation against a prohibition; the subsidiary passes an amendment that satisfies the group and violates procurement law; and three authorities declare priorities that form a cycle.
11. The subsidiary validly amends its own amendment rule while a commitment is outstanding. It then attempts three amendments: one that removes the logging duty, one that keeps the duty and narrows the definition of "protected action", and one that keeps the witness duty and replaces the witnesses.
12. An early breach that is irrelevant: one unit breaks a guarantee the chain property does not depend on, and a later unit causes the violation.

After the core is chosen, the external reviewers write one further variant that the authors have not seen.

**Frozen before any proof attempt, and stated over traces so that they do not presuppose one core.** The grammar of the policy fragment, its restriction rule, whether quantities are bounded, whether principals can be created during the analysis, the semantics of revocation, and the complexity class that counts as success (R1). The causal definition of deviation, the verdict type, and the content of a per-unit summary (R2). The class of mappings between vocabularies and the pairwise compliance predicate (R3). The rule for which commitments survive a rule change, with at least one useful class of changes that it accepts (R4). The list of entrenched invariants and at least one useful self-amendment that must be accepted (R5). A ledger giving, for each reused result, its source, the theorem, and how it is instantiated. Thresholds: the ceiling on "unknown" classifications, the ceiling on coordinated decisions, the number of runs per variant and the pass rate required with language-model agents.

**Exit test.** Two reviewers independently judge ten named systems against each definition. The ten include a law-governed interaction deployment, a Cedar policy store, a capability-token deployment, a decentralised autonomous organisation, a national procurement regime and an open message bus with no governance. Two small constructed examples that are meant to satisfy the definitions are added to the ten, because the survey found no existing system that composes all the structures. Judgements are made clause by clause (each element of the governance interface, each field of the record) for each system. The step passes if chance-corrected agreement over those judgements reaches a level fixed in advance, each clause is judged satisfied by at least two systems and unsatisfied by at least two, and the message bus is rejected.

**Sources to read in full first.** The multi-level compliance paper, the timed I/O automata monograph, the dynamic I/O automata creation theorem, the accountable-log protocol, the compositional-blame and component-causality papers, the composition theorem for assume-guarantee specifications, and the paper cited for conformance hierarchies, whose abstract does not mention them. Several were checked only at abstract level.

## Step 1: paper stage

This stage is done by hand in the default core of step 2. It comes first because it is cheap and decides whether the rest is worth doing.

**Immediate checks.** Five small cases are run against the definitions before anything else: the four cases of variant 8; the late update and the cap below past spending in variant 9; the priority cycle of variant 10; the early irrelevant breach of variant 12; and creation, a deadline and dissolution in the default core (variant 5), with a time limit. Dissolution by an outside decision has to be encoded as a request, because the published creation theorem allows a unit to end only by its own action. If the definitions do not handle a case, step 0 is revised.

**Open research targets.**

| | Target | Test that could refute it | Prior work it must go beyond |
|---|---|---|---|
| R1 | Authority analysis. For a fragment that includes consumable quantities and revocation, and a restriction rule naming which units' rules and amendment procedures are fixed, the questions "can p ever obtain r" and "does role A always contain role B" are decidable within the complexity class frozen in step 0. Amendment is an action in the analysis, so "only after an amendment passing procedure P" is a possible answer | The analysis is trivial (bounded quantities and a fixed set of principals, or everything reachable after an amendment), or undecidable or non-elementary for the fragment the benchmark needs, as counter systems with reset can be | Trust-management analysis, where simple safety is polynomial and containment is harder, under a restriction rule ([Li, Mitchell & Winsborough](https://www.cs.purdue.edu/homes/ninghui/papers/analysis_oakland03.pdf)); reachability for administrative role-based policies ([Sasturkar et al.](https://www3.cs.stonybrook.edu/~stoller/arbac-analysis-TCS.html)); consumable authority in a linear access-control logic ([Bowers et al.](https://www.mpi-sws.org/~dg/papers/NDSS07Final.pdf)) |
| R2 | Accountability under delegation. With the frozen causal definition and verdicts that are sets of minimally sufficient sets of parties: (a) every covered violation caused by a deviation yields a proved-deviation verdict naming the deviators; (b) no compliant party is named; (c) a violation with no deviation is classified as allocated loss with a bearer fixed by a declared rule. The composition claim: the verdict for a chain can be computed from signed per-unit summaries without reading inner logs. A summary has one entry per guarantee, giving whether it was broken and when, the assumptions that guarantee depends on, and the prefix of the boundary trace needed for the counterfactual | Two histories with the same summaries that require different chain verdicts, as in joint causation across two units or variant 12; or summaries that must grow to the size of the inner logs | Protocol accountability ([Künnemann et al.](https://arxiv.org/abs/1805.10891); [Küsters et al.](https://eprint.iacr.org/2010/236)); compositional blame for contracts ([Hvitved et al.](https://dblp.org/rec/journals/jlp/HvitvedKZ12)); blame and causality in component systems ([Gössler & Le Métayer](https://inria.hal.science/hal-00873665); [Caltais et al.](https://arxiv.org/abs/1608.07880)); accountable logs ([PeerReview](https://haeberlen.cis.upenn.edu/papers/peerreview-tr.pdf)); threshold witness cosigning ([Syta et al.](https://arxiv.org/abs/1503.08768)); receipts with replay and individual blame ([Shamis et al.](https://www.usenix.org/conference/nsdi22/presentation/shamis)). The compositional causality result cited here covers only non-communicating systems |
| R3 | Behavioural conformance across vocabularies and several authorities. Each level states norms in its own vocabulary, mappings between levels depend on context read from the lower level's state, and a unit may have several authorities. Under a stated condition on the mappings, pairwise design compliance plus enforcement at the leaf implies that the leaf's behaviour complies with its effective norm set, and that the effective norm set is consistent whenever the declared priorities form a strict partial order that decides every conflicting pair | Pairwise compliance checked in inconsistent contexts while the leaf's behaviour violates the root's norm; variant 10 | Multi-level institutional compliance over a directed acyclic graph of institutions, with a three-level case study to reproduce ([King et al.](https://researchportal.bath.ac.uk/en/publications/automated-multi-level-governance-compliance-checking/)); conformance hierarchies in law-governed interaction ([Minsky](https://arxiv.org/abs/1403.4321)). Transitivity of sound trace mappings on one vocabulary is not counted as a result |

**Narrower open questions.**

| | Target | Test that could refute it |
|---|---|---|
| R4 | Commitments across a rule change. The frozen rule says which commitments made under version n must be honoured under version n+1. The target is that every change is either rejected or installed with an explicit, bounded set of grandfathered commitments, and that after the lease bound the new invariant holds for everything outside that set. An infeasible change (variant 9, a cap below what is already spent) must be rejected or installed with the past spending grandfathered | A change in the frozen accepted class for which a commitment validly made under the old cap breaks the new cap outside the grandfathered set |
| R5 | Entrenchment under self-amendment. For the frozen grammar, the check that an amendment preserves the entrenched invariants (single ownership of resources, log continuity, the witness duty with witnesses appointed from outside, and conformance to the effective norm set) is decidable, and it is complete against amendments that change the meaning of terms the invariants use. Competing amendments are serialised by the shard that owns the charter | Variant 11: an accepted amendment after which an entrenched invariant no longer holds in substance; or a check that rejects the useful self-amendment frozen in step 0 |
| R6 | A constraint that spans one identity's roles. Separation of duty for the officer (not approving and auditing the same purchase) is a norm of procurement law. The ministry, as the authority that owns it, detects a breach by joining receipts from the team and the audit body, keyed by identity, when the two units name the purchase differently and the purchase is split (variant 7) | A violation that the receipts cannot reveal without inner logs or without a shared name for the purchase |

**New metatheory for the core.** Closure and substitutivity for a model with both time and creation is not a published result as far as the plan knows. It is new work, checked first on variant 5.

**Results to reproduce and application lemmas, counted as reuse.**

- Complete mediation: with correct controllers, no protected effect occurs without a recorded derivation.
- Conservation of a resource with a single owner, with funding claims retained in both the nested and the flattened reading.
- The lease argument: after a rule change, a controller that has not heard stops authorising within a time fixed by the lease period and clock skew. Online schema change with leases is the precedent ([Rae et al.](https://research.google/pubs/online-asynchronous-schema-change-in-f1/)).
- Coordination classification with existing tools: invariant-confluence checking and its segmented form ([Whittaker & Hellerstein](https://mwhittaker.github.io/publications/iconfluence_vldbj2020.pdf)), pre-allocated rights ([Balegas et al.](https://arxiv.org/abs/1503.09052)), and conflict analysis with synthesised coordination ([Houshmand & Lesani](https://mohsenlesani.github.io/companion/popl19/POPL19.pdf)). The share of benchmark invariants in each class is reported, with a time limit per invariant and timeouts counted as unknown.
- An assume-guarantee contract for the vendor. Inside the declared envelope the purchase invariants hold; outside it the departure is detected at a named observation point within a stated local bound, and the dispute goes to the adjudicating unit, since exchange between two parties cannot be made fair without a third.

**Other work in this stage.** Derive the assurance records for the three-unit chain. Write the scale argument of step 5 on paper. Send everything to external reviewers.

**Stop rules.**

- If the frozen statement of R2 is false on a three-unit chain and the weakest repair needs inner logs, publish that as a negative result for this candidate and stop.
- The work continues only if R2 survives and at least one of R1 and R3 improves on its stated baseline, as judged by the external reviewers. Otherwise report that the integration is a repackaging and stop.
- If R5 fails, the claim about self-reflection is withdrawn from what a positive result establishes.
- The external reviewers judge whether a surviving statement is definitional. A statement counts as a result only if it is refuted by some benchmark variant when one named hypothesis is dropped, which is a necessary check and not a sufficient one, and it is not an instance of a step-0 definition or of a result in the reuse ledger. A reviewer who says it is must exhibit the instance.

## Step 2: choose the core

**Default hypothesis.** Timed input/output automata with dynamic creation. The untimed model ([Lynch & Tuttle](https://groups.csail.mit.edu/tds/papers/Lynch/CWI89.pdf)) has no deadlines and no creation. Dynamic I/O automata add creation and destruction; their substitutivity theorem for creation has six premises ([Attie & Lynch](https://arxiv.org/abs/1604.06030)). Timed I/O automata add time; the conditions that stop a component blocking time matter for whether a deadline can be met vacuously†. The plan knows of no published theorem for the two together. The combined model therefore carries two groups of proof obligations, and failing to discharge them for variant 5 (creation, deadline, dissolution, with hiding) is a reason to drop this core. Creation by a constitutional step is made an observable action.

**Candidates.** Two concrete encodings compete: the default, and a timed specification theory with refinement, conjunction, composition and quotient ([David et al.](https://homes.cs.aau.dk/~adavid/publications/41-hscc10.pdf)). Law-governed interaction with an institutional monitor is kept as a baseline for expressiveness only, because it assumes trusted controllers and so cannot state R2. One further model, dynamic networks of timed machines†, is screened in the survey of existing work. Polynomial functors, bigraphs and reflective rewriting logic are left out of the bake-off to keep its cost bounded, which means the plan does not test the categorical candidates the survey describes.

**Decision rule, applied in this order.** Can the candidate express variants 2, 3, 5 and 9 under the same observable requirements and environment assumptions? Does it support the frozen statement of R2? Which semantic obligations and trust assumptions does it need, taken from one common inventory so that the count does not depend on how conditions are packaged, with incomparable conditions reported as such? How large is it on paper? Each encoding gets the same time budget, fixed in advance, and an advocate of each candidate reviews its encoding. "Not established within the budget" is an allowed outcome and is not evidence that a candidate is inferior. If neither candidate is eligible the plan stops and reports it; if one is, it is chosen without a runner-up; a tie goes to the candidate with existing mechanisation. Existing mechanisations are surveyed first.

**If the core changes,** the module contracts are reassessed.

## Step 3: mechanise

Mechanise the core, the five structures (authority, resources, commitments and norms, log and time, rule change) and the targets that survived step 1.

**Rule change.** A unit's rules are controller state. An amendment is valid when it passes the unit's own charter procedure, conforms to the unit's effective norm set and passes the entrenchment check of R5. A charter may contain an amendment rule that applies to itself; version n+1 is checked under version n. This gives a history of authorised change and nothing more; what persists is the entrenched set of R5.

**Separability.** Each structure has a local lemma from the reuse list. The hypothesis fails if the proof of a local lemma imports definitions from more than one other structure. R1 to R6 are integration targets, and for them the dependency graph is reported without a pass or fail.

## Step 4: reference controller and benchmark run

- Write an executable controller, witness and policy evaluator for the decidable fragment.
- Test them against the mechanised model on sequences of transitions, concurrent revocation, crash recovery and version change. The model's theorems are proved. The executable's conformance is tested, not proved.
- Run every variant with language-model agents, scripted humans and an unmodified vendor service, for the fixed number of runs, and check the required outcomes against the fixed pass rate. Controller-side outcomes (wrongly blocked, wrongly allowed) are reported separately from whether the agents completed the task.

## Step 5: scale

Containment depth is fixed by the benchmark and fan-out is not bounded, so the load bounds carry the content. Rules flow along the authority relation and not the containment tree, so the bounds are stated in terms of containment depth, the longest authority path, the number of authorities over a unit, policy size, and the vertices and edges of the affected dependency closure. Each bound is first argued on paper in step 1, on cases that include a long authority chain and a wide join of authorities over shallow containment.

- **Per-decision cost.** Proportional to containment depth plus the number of applicable authorities.
- **Load per shard per unit time,** under a stated rate of active principals. It includes authorisation, lease renewal, receipt forwarding, and witness replay and its backlog. A unit whose load grows with population must be shown to shard under the definition in step 0.
- **Summary and log growth.** The size of the per-unit summaries used by R2, and of logs, as functions of activity.
- **Rule change,** as four separate quantities. The time after which no controller authorises under the old rule is finite and is proved from the lease period and clock skew along the longest authority path, with no assumption on delivery; a lease that depends on an authority cannot be renewed once that authority's own lease has expired. Installation time and recovery time are finite only under a stated bounded-delay workload assumption; otherwise they are unbounded and appear as unavailability. Message cost is proportional to the affected dependency closure, under a stated dissemination algorithm. When a cap is lowered, expiry of the rights already allocated stops further spending under them and does not undo spending already done. The new cap holds only if completed spending and grandfathered commitments fit under it and new rights are issued against what remains; otherwise R4 applies. Rate limits that renew are treated separately from cumulative caps.

The workload mix, taken from a real procurement workload if one can be obtained, is fixed before measuring. Simulation measures the constants at increasing sizes, including shared-root and several-authority workloads.

**The scale claim fails if** any unit cannot be sharded at the target workload, any bound exceeds its declared growth, or the coordinated share exceeds its ceiling. Correlated faults, anchor failure and adversarial load are not covered.

## How the plan could fail as a whole

- **The formal model does not match the world.** The defence is the benchmark and review by people who run real procurement, and it is weak.
- **Most consequential actions are unmediated,** or are seen only by parties who collude. Then the foundation governs little.
- **The open targets collapse.** The stop rules fire and the result is negative for this candidate.
- **Trust anchors dominate.** If every guarantee reduces to the honesty of the identity anchor, the root rules and one witness per unit, the foundation is an accounting system for trust. The plan would say which it is.

## Order of work

1. Step 0, including the frozen statements and the source reading. **Decision:** do the definitions pass the exit test?
2. The immediate checks, then the paper stage and its external review. **Decision:** do the stop rules fire?
3. Bake-off. **Decision:** which core, or neither?
4. Mechanisation. **Decision:** are the structures separable?
5. Reference controller and benchmark run.
6. Scale measurements.
7. External adversarial review of the theorems and the evidence.

No effort estimates are given. An estimate becomes possible after the paper stage.

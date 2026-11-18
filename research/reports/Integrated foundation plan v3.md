# Plan for an integrated foundation

**Status.** This is a proposal, in its third version. It follows from the survey in [Compositional foundations for governed systems.md](/root/mic/reports/Compositional foundations for governed systems.md), which found that small rigorous substrates exist and that nobody has composed them into one foundation with a stated composition law for authority, obligation and liability. Two rounds of independent adversarial review shaped it (files beginning "plan_attack_" in /root/mic/reports). Round 1 found that the theorem targets could not fail. Round 2 found that the threat model made the accountability targets unsatisfiable, that rule authority had been wrongly tied to containment, and that several targets were already known. This version fixes those. Citations marked † are from memory and were not fetched; the reviewers checked the others at varying depth.

## What a positive result would and would not establish

A successful run would show that mediated, recorded interactions among units with specified controllers compose with stated guarantees about authority, resources, commitments and rule change, and that the guarantees survive nesting, several rule authorities over one unit, and creation and dissolution of units.

It would not show that a foundation for governing humans, learned agents and institutions exists in the full sense of the original goal. Humans, learned agents and third-party services are environments here. "Self-reflecting" is delivered as rule change checked by the rules in force, with a fixed set of entrenched invariants and a named anchor. "10^10" is delivered as stated growth bounds and a load calculation, not a deployment. Legitimacy, incentives and alignment are outside it.

Three targets are open research (R1 to R3) and three are narrower open questions (R4 to R6). Everything else is reuse of published results, and the plan reports reuse and novelty separately. A failed target is evidence against this candidate and its encoding, not an impossibility result about the goal.

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

**Principals and roles.** A principal is an identity plus a set of role instances. Each role instance belongs to exactly one unit, so component containment is a tree. What an identity carries between its own role instances in different units is not mediated.

**Four separate relations.** Membership (which role instances an identity holds). Component containment (a tree). Resource ownership (each resource has exactly one owning unit; other units may hold recorded funding claims on it). Rule authority (which units' rules constrain a unit). Rule authority is a declared directed acyclic relation that is independent of containment, and a unit may have several authorities.

**Controllers.** Every unit has an atomic controller that owns the effect of every protected action: other components send requests, and only the controller emits the effect. Where load requires it, a unit has a set of atomic controllers (shards). Each shard owns a disjoint set of resources and rules, and a static function assigns every protected action to exactly one shard. An invariant that spans shards is kept only through rights allocated in advance. An action that needs two shards is a coordinated action and is counted as such in step 5.

**Receipts and witnesses.** An effect is usable by any other party only with a receipt countersigned by a witness of the unit. Each unit has at least one honest witness with an active duty: it holds signed commitments to the controller's log and, at a stated period, replays the controller against a reference implementation. Controllers are therefore deterministic.

**Fractal.** Closure, substitutivity under stated hypotheses, and a uniform governance interface: every unit exposes authority, resources, commitments and norms, a log, and a rule-change procedure.

**Controlled.** A list of named properties, each with a formal statement and an assurance record with six fields: mechanisms, assumptions, what is observable, detection time, intervention available, and what is left unprotected. A composite's record is derived field by field. Interface assumptions that a neighbour's guarantee discharges are removed and the rest are united. Unprotected remainders are united. Observables are intersected. Detection times add along a dependency path and take the maximum across independent paths. The mechanism is the least capable present, in the order: blocked at the controller, certificate checked, envelope assumed, logged and adjudicated. Where no rule applies the composite's record is "unestablished". The derivation is conservative, and it is exercised in step 1 on serial and on alternative evidence paths.

**Assumptions about faults and the network.**

- Any non-controller component may deviate arbitrarily, and any set may collude.
- Prevention results assume correct controllers.
- Accountability results allow a controller to deviate, and cover only effects that reach at least one honest party able to contact a witness. An effect seen only by colluders is outside every guarantee.
- Links between a controller, its witnesses and its adjudicator deliver within a stated bound. Vendor links and links between units have no delivery bound.
- Clocks are synchronised to within a stated skew.
- The identity anchor and the root rules are trusted.

**Obligations and verdicts.** An obligation is discharged by a receipt logged at the obligee's controller before the deadline, so a missed deadline is observable locally. There are three kinds of verdict, kept distinct: proved deviation, suspicion (evidence withheld or a party unresponsive), and allocated loss where nobody deviated. Lateness caused by an unbounded link is allocated loss borne by the obligor.

**Useful behaviour.** Each benchmark variant lists outcomes that must occur whenever the delivery bounds of a stated performance envelope hold. Refusals outside the envelope are counted as unavailability. A controller that rejects everything inside the envelope fails.

**Smaller.** In steps 1 and 2: definitions and hypotheses counted on paper in a common notation. In step 3: lines of definitions plus axioms and trust anchors in the proof assistant, for the chosen core and the runner-up. Domain definitions, trusted assumptions, reused libraries and proof effort are counted separately. The runtime trust base is inventoried separately. No global minimality is claimed.

**Benchmark family.** A public procurement chain. Containment: a state contains a ministry; a parent group (enterprise) contains a subsidiary (organisation), which contains a team, which contains an approving officer's role (human) and a purchasing agent (learned agent) with its purchasing tool (program). Resources are a budget and purchased compute. A compute vendor is an external service. Rule authority: a supranational body over the state, the ministry over the group and subsidiary through procurement law, the group over the subsidiary through group policy, and an external audit body over the officer's audit role. The ministry funds the group by a commitment. The variants are:

1. The compliant path.
2. A diamond: the officer also holds a role in the audit body, and the budget carries funding claims from two units.
3. Formation: a charter creates the subsidiary, a vote constitutes an authority no delegator held, and one declared amplification (officer approval combined with the agent's budget right).
4. Revocation and claw-back of a partly spent grant, concurrent with a purchase.
5. Creation, a deadline, then dissolution of the subsidiary with a commitment outstanding.
6. Vendor failure with every chain principal compliant, and a disputed delivery decided by a named adjudicating unit.
7. Purchases split to stay under the approval threshold.
8. A controller omits a log entry, once where an honest recipient holds a receipt and once where only a colluding recipient saw the effect. The second must come out as outside coverage.
9. The ministry lowers the spending cap while purchases are in flight, one controller hears late, and the new cap is below what has already been spent.
10. Group policy and procurement law conflict for the subsidiary, and the subsidiary passes an amendment that satisfies the group and violates procurement law.
11. The subsidiary validly amends its own amendment rule while a commitment is outstanding, then attempts an amendment that removes the logging duty.

After the core is chosen, the external reviewers write one further variant that the authors have not seen.

**Frozen before any proof attempt.** The grammar of the policy fragment and its restriction rule (R1). The causal definition of deviation, the verdict type, and the content of a per-unit summary (R2). The class of mappings between vocabularies and the pairwise compliance predicate (R3). The rule for which commitments survive a rule change (R4). The list of entrenched invariants (R5). Thresholds: the ceiling on "unknown" classifications, the ceiling on coordinated decisions, the number of runs per variant and the pass rate required with language-model agents.

**Exit test.** Two reviewers independently judge ten named systems against each definition. The ten include a law-governed interaction deployment, a Cedar policy store, a capability-token deployment, a decentralised autonomous organisation, a national procurement regime and an open message bus with no governance. The step passes if chance-corrected agreement reaches a level fixed in advance, each definition is judged satisfied by at least two systems and unsatisfied by at least two, and the message bus is rejected.

**Sources to read in full first.** The multi-level compliance paper, the timed I/O automata monograph, the dynamic I/O automata creation theorem, the accountable-log protocol, and the compositional-blame and component-causality papers. Several were checked only at abstract level.

## Step 1: paper stage

This stage is done by hand in the default core of step 2. It comes first because it is cheap and decides whether the rest is worth doing.

**Two immediate checks.** The indistinguishable-execution case of variant 8 and the late-update case of variant 9 are run against the definitions on the first day. If the receipt rule and the lease rule do not handle them, step 0 is revised before anything else.

**Open research targets.**

| | Target | Test that could refute it | Prior work it must go beyond |
|---|---|---|---|
| R1 | Authority analysis. For a fragment that includes consumable quantities and revocation, and a restriction rule naming which units' rules and amendment procedures are fixed, the questions "can p ever obtain r" and "does role A always contain role B" are decidable with a stated bound. Amendment is an action in the analysis, so "only after an amendment passing procedure P" is a possible answer | The analysis is trivial (everything is reachable after an amendment) or undecidable for the fragment the benchmark needs | Trust-management analysis, where simple safety is polynomial and containment is harder, under a restriction rule ([Li, Mitchell & Winsborough](https://www.cs.purdue.edu/homes/ninghui/papers/analysis_oakland03.pdf)); reachability for administrative role-based policies ([Sasturkar et al.](https://www3.cs.stonybrook.edu/~stoller/arbac-analysis-TCS.html)) |
| R2 | Accountability under delegation. With the frozen causal definition and verdicts that are sets of minimally sufficient sets of parties: (a) every covered violation caused by a deviation yields a verdict naming the deviators; (b) no compliant party is named; (c) a violation with no deviation is classified as allocated loss with a bearer fixed by a declared rule. The composition claim: the verdict for a chain can be computed from per-unit summaries of the form "unit U broke its guarantee at its boundary at time t, its assumptions having held until t", without reading inner logs | Two histories with the same summaries that require different chain verdicts, as in joint causation across two units | Protocol accountability ([Künnemann et al.](https://arxiv.org/abs/1805.10891); [Küsters et al.](https://eprint.iacr.org/2010/236)); compositional blame for contracts ([Hvitved et al.](https://dblp.org/rec/journals/jlp/HvitvedKZ12)); blame and causality in component systems ([Gössler & Le Métayer](https://inria.hal.science/hal-00873665); [Caltais et al.](https://arxiv.org/abs/1608.07880)); accountable logs ([PeerReview](https://haeberlen.cis.upenn.edu/papers/peerreview-tr.pdf)) |
| R3 | Behavioural conformance across vocabularies and several authorities. Each level states norms in its own vocabulary, mappings between levels depend on context read from the lower level's state, and a unit may have several authorities. Under a stated condition on the mappings, pairwise design compliance plus enforcement at the leaf implies that the leaf's behaviour complies with every authority above it | Pairwise compliance checked in inconsistent contexts while the leaf's behaviour violates the root's norm; variant 10 | Multi-level institutional compliance over a directed acyclic graph of institutions, with a three-level case study to reproduce ([King et al.](https://researchportal.bath.ac.uk/en/publications/automated-multi-level-governance-compliance-checking/)); conformance hierarchies in law-governed interaction ([Minsky](https://arxiv.org/abs/1403.4321)). Transitivity of sound trace mappings on one vocabulary is not counted as a result |

**Narrower open questions.**

| | Target | Test that could refute it |
|---|---|---|
| R4 | Commitments across a rule change. The frozen rule says which commitments made under version n must be honoured under version n+1. The target is that this rule and the new invariant are jointly satisfiable when rights were allocated in advance, and that an infeasible change (variant 9, a cap below what is already spent) is rejected, explicitly grandfathered, or recorded as an invariant that does not hold | A commitment validly made under the old cap whose honouring breaks the new cap, with no outcome the rule permits |
| R5 | Entrenchment under self-amendment. Every valid sequence of amendments, including amendments to the amendment rule, preserves a named set of entrenched invariants: single ownership of resources, log continuity and the witness duty, and conformance to every declared authority. Competing amendments are serialised by the controller | Variant 11: a valid amendment that removes the logging duty |
| R6 | A constraint that spans one identity's roles. Separation of duty for the officer (not approving and auditing the same purchase) is detected by joining receipts from the team and the audit body, keyed by identity, without either unit reading the other's log | A violation that the two units' receipts cannot reveal without inner logs |

**Results to reproduce, counted as reuse.**

- Closure and substitutivity for the chosen core, with its hypotheses.
- Controller correctness for correct controllers.
- Conservation of a resource with a single owner, with funding claims retained in both the nested and the flattened reading.
- The lease argument: after a rule change, a controller that has not heard stops authorising within a time fixed by the lease period and clock skew. Online schema change with leases is the precedent ([Rae et al.](https://research.google/pubs/online-asynchronous-schema-change-in-f1/)).
- Coordination classification with existing tools: invariant-confluence checking and its segmented form ([Whittaker & Hellerstein](https://mwhittaker.github.io/publications/iconfluence_vldbj2020.pdf)), pre-allocated rights ([Balegas et al.](https://arxiv.org/abs/1503.09052)), and conflict analysis with synthesised coordination ([Houshmand & Lesani](https://mohsenlesani.github.io/companion/popl19/POPL19.pdf)). The share of benchmark invariants in each class is reported, with a time limit per invariant and timeouts counted as unknown.
- An assume-guarantee contract for the vendor. Inside the declared envelope the purchase invariants hold; outside it the departure is detected at a named observation point within a stated local bound, and the dispute goes to the adjudicating unit, since exchange between two parties cannot be made fair without a third.

**Other work in this stage.** Derive the assurance records for the three-unit chain. Write the scale argument of step 5 on paper. Send everything to external reviewers.

**Stop rules.**

- If the frozen statement of R2 is false on a three-unit chain and the weakest repair needs inner logs, publish that as a negative result for this candidate and stop.
- If, for all of R1 to R3, the statement either fails or does not improve on its stated baseline, report that the integration is a repackaging and stop.
- The external reviewers judge whether a surviving statement is definitional. The criterion is that a non-definitional statement is refuted by at least one benchmark variant when one named hypothesis is dropped.

## Step 2: choose the core

**Default hypothesis.** Timed input/output automata with dynamic creation. The untimed model ([Lynch & Tuttle](https://groups.csail.mit.edu/tds/papers/Lynch/CWI89.pdf)) has no deadlines and no creation. Dynamic I/O automata add creation and destruction; their substitutivity theorem for creation has six premises ([Attie & Lynch](https://arxiv.org/abs/1604.06030)). Timed I/O automata add time; the conditions that stop a component blocking time matter for whether a deadline can be met vacuously†. The plan knows of no published theorem for the two together. The combined model therefore carries two groups of proof obligations, and failing to discharge them for variant 5 (creation, deadline, dissolution, with hiding) is a reason to drop this core. Creation by a constitutional step is made an observable action.

**Candidates.** Three concrete encodings: the default; one interface or assume-guarantee contract theory ([interface automata](https://luca.dealfaro.com/papers/01/FSE01.pdf)); and a governance baseline, law-governed interaction with an institutional monitor. Polynomial functors, bigraphs and reflective rewriting logic are left out of the bake-off to keep its cost bounded, which means the plan does not test the categorical candidates the survey describes.

**Decision rule, applied in this order.** Can the candidate express variants 2, 3, 5 and 9 under the same observable requirements and environment assumptions? Does it support the frozen statement of R2? How many hypotheses does it need? How large is it on paper? Each encoding is time-boxed, "not established within the budget" is an allowed outcome, and an advocate of each candidate reviews its encoding. Existing mechanisations are surveyed first.

**If the core changes,** the module contracts are reassessed.

## Step 3: mechanise

Mechanise the core, the five structures (authority, resources, commitments and norms, log and time, rule change) and the targets that survived step 1.

**Rule change.** A unit's rules are controller state. An amendment is valid when it passes the unit's own charter procedure and conforms to every declared rule authority over the unit. Where two authorities conflict, a declared priority order decides; where none is declared, the conflict is recorded and referred to a named adjudicator. A lemma shows this rule always yields an outcome. A charter may contain an amendment rule that applies to itself; version n+1 is checked under version n. This gives a history of authorised change and nothing more; what persists is the entrenched set of R5.

**Separability.** Each structure has a local lemma from the reuse list. The hypothesis fails if the proof of a local lemma imports definitions from more than one other structure. R1 to R6 are integration targets, and for them the dependency graph is reported without a pass or fail.

## Step 4: reference controller and benchmark run

- Write an executable controller, witness and policy evaluator for the decidable fragment.
- Test them against the mechanised model on sequences of transitions, concurrent revocation, crash recovery and version change. The model's theorems are proved. The executable's conformance is tested, not proved.
- Run every variant with language-model agents, scripted humans and an unmodified vendor service, for the fixed number of runs, and check the required outcomes against the fixed pass rate.

## Step 5: scale

Depth is fixed by the benchmark and fan-out is not bounded, so the load bounds carry the content. Each is first argued on paper in step 1.

- **Per-decision cost.** Proportional to depth.
- **Load per shard per unit time,** under a stated rate of active principals. It includes authorisation, lease renewal, and witness and audit traffic. A unit whose load grows with population must be shown to shard under the definition in step 0.
- **Summary and log growth.** The size of the per-unit summaries used by R2, and of logs, as functions of activity.
- **Rule change,** as four separate quantities. The time after which no controller authorises under the old rule is finite and is proved from the lease period and clock skew down the depth of the tree, with no assumption on delivery. Installation time and recovery time are finite only under a stated bounded-delay workload assumption; otherwise they are unbounded and appear as unavailability. Message cost is proportional to the affected subtree. A cap lowered below the sum of rights already allocated holds only after those rights expire, and that interval is recorded.

The workload mix, taken from a real procurement workload if one can be obtained, is fixed before measuring. Simulation measures the constants at increasing sizes, including shared-root and several-authority workloads.

**The scale claim fails if** any unit cannot be sharded at the target workload, any bound exceeds its declared growth, or the coordinated share exceeds its ceiling. Correlated faults, anchor failure and adversarial load are not covered.

## How the plan could fail as a whole

- **The formal model does not match the world.** The defence is the benchmark and review by people who run real procurement, and it is weak.
- **Most consequential actions are unmediated,** or are seen only by parties who collude. Then the foundation governs little.
- **The open targets collapse.** The stop rules fire and the result is negative for this candidate.
- **Trust anchors dominate.** If every guarantee reduces to the honesty of the identity anchor, the root rules and one witness per unit, the foundation is an accounting system for trust. The plan would say which it is.

## Order of work

1. Step 0, including the frozen statements and the source reading. **Decision:** do the definitions pass the exit test?
2. The two immediate checks, then the paper stage and its external review. **Decision:** do the stop rules fire?
3. Bake-off. **Decision:** which core?
4. Mechanisation. **Decision:** are the structures separable?
5. Reference controller and benchmark run.
6. Scale measurements.
7. External adversarial review of the theorems and the evidence.

No effort estimates are given. An estimate becomes possible after the paper stage.

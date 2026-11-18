# Plan for an integrated foundation

**Status.** This is a proposal, in its second version. It follows from the survey in [Compositional foundations for governed systems.md](/root/mic/reports/Compositional foundations for governed systems.md), which found that small rigorous substrates exist and that nobody has composed them into one foundation with a stated composition law for authority, obligation and liability. The first version of this plan was attacked by two independent reviewers ([plan_attack_astra_r1.md](/root/mic/reports/plan_attack_astra_r1.md), [plan_attack_fable_r1.md](/root/mic/reports/plan_attack_fable_r1.md)). Their central finding was that its theorem targets could not fail, so it could not answer its own question. This version restates the targets so that each can fail, and moves the cheapest decisive tests to the front. Citations marked † are from memory and were not fetched.

## What a positive result would and would not establish

A successful run of this plan would show that mediated, recorded interactions among units with specified controllers compose with stated guarantees about authority, resources, commitments and rule change, and that the guarantees survive nesting, overlap and creation and dissolution of units.

It would not show that a foundation for governing humans, learned agents and institutions exists in the full sense of the original goal. Humans, learned agents and third-party services are environments here: the plan proves things about what their controllers let through and about what is recorded, and proves one bounded-envelope result about an unspecified actor. "Self-reflecting" is delivered as rule change checked by the rules in force, ending in a named anchor. "10^10" is delivered as stated growth bounds and a load calculation, not a deployment. Legitimacy, incentives and alignment are outside it.

A failed target is evidence against this candidate and its encoding. It is not an impossibility result about the goal.

## Non-goals

- No complete automatic verifier for arbitrary units.
- No unconditional self-certification. Trust anchors are named.
- No optimal decentralised control and no ideal preference aggregation.
- No guarantee about human or learned behaviour outside a mediated interface or a declared envelope.
- No strict Sybil-proofness without an identity anchor. Identity is an assumption of every result, not a module with its own theorem.
- No general liveness. Obligations are deadlines in a timed model, where missing a deadline is a safety violation.
- No confidentiality or privacy guarantee. An observation declaration is an upper bound on what is logged, not on what a party can learn. Information-flow properties are not preserved by the refinement used here.
- No incentive compatibility. Sanctions are recorded and executed; whether they deter is not claimed.
- No claim about legitimacy.

## Step 0: definitions, threat model and benchmark, fixed before anything else

**Unit.** An open transition system with input, output and internal actions, a declared interface, and a clock.

**Principals and roles.** A principal is an identity plus a set of role instances. Each role instance belongs to exactly one unit, so containment of role instances is a tree. Four relations are kept separate: membership (which role instances an identity holds), component containment (which unit a role instance or sub-unit sits in), resource ownership (which unit owns each resource, exactly one), and rule authority (which unit's rules constrain which). What an identity carries between its own role instances in different units is not mediated, and is covered only by a declared envelope or by audit.

**Boundary controller.** Every unit has a controller that owns the effect of every protected action: other components send requests, and only the controller emits the effect. A controller is either atomic or itself a unit with an atomic controller, so the regress ends. At the upper levels a controller is a logical service that may be sharded, and step 5 has to show that it can be.

**Fractal.** Closure (a composite of units is a unit), substitutivity (a part can be replaced by one that refines it, under stated hypotheses), and a uniform governance interface (every unit exposes authority, resources, commitments and norms, a log, and a rule-change procedure).

**Controlled.** A list of named properties. Each has a formal statement and an assurance record: the mechanisms used (block at the controller, check a supplied certificate, assume an envelope, log and adjudicate), their assumptions, what is observable, how long detection takes, what intervention is available, and what is left unprotected. A composite reports the weakest record among the parts a property depends on.

**Useful behaviour.** Each benchmark variant lists outcomes that must still occur (for example, a compliant purchase completes before its deadline). A controller that rejects everything fails.

**Smaller.** Lines of definitions plus the number and statement of axioms and trust anchors, measured in one proof assistant, plus encoding size on the benchmark. The runtime trust base (parser, storage, clock, recovery) is inventoried separately.

**Threat model.** Any non-controller component may deviate arbitrarily and any set of them may collude. A controller may deviate, provided at least one honest witness per unit holds signed commitments to that controller's log (the accountable-log approach†). The identity anchor and the root rules are trusted. Network delivery is not bounded; clocks are synchronised to within a stated skew.

**Benchmark family.** A public procurement chain, with each of the ten levels mapped to a named unit: budget and compute (resources), the agent's purchasing tool (program), a compute vendor (service, unspecified behaviour), an approving officer (human), a purchasing agent (learned agent), a team, a subsidiary (organisation), its parent group (enterprise), a ministry (government), and the state bound by a supranational spending rule (nation). The variants are:

1. The compliant path.
2. A diamond: the officer also holds a role in an external audit body, and one resource is funded by two units.
3. Formation: a charter creates the subsidiary, a vote constitutes an authority no delegator held, and one declared amplification (officer approval combined with the agent's budget right).
4. Revocation and claw-back of a grant that has been partly spent, concurrent with a purchase.
5. Dissolution of the subsidiary with a commitment outstanding.
6. Vendor failure with every chain principal compliant, and a disputed delivery decided by a named adjudicating unit.
7. Purchases split to stay under the approval threshold.
8. A controller that omits log entries.
9. The ministry lowers the spending cap while purchases are in flight, and one controller receives the change late.
10. Two overlapping units impose conflicting norms on one role-holder.

One variant is held out and used only after the core is chosen.

**Exit test.** Two reviewers independently judge ten named systems (including a law-governed interaction deployment, a Cedar policy store, a capability-token deployment, a decentralised autonomous organisation and a national procurement regime) against each definition. The step passes if they agree on at least four judgements in five; the threshold is a proposal and is fixed before the judging.

## Step 1: paper stage, before any mechanisation

This stage needs only the definitions and the benchmark. It is done first because it is cheap and decides whether the rest is worth doing.

**Research targets.** Each has a formal statement, a candidate counterexample, and a note of what it says beyond "the controller implements its policy".

| | Target | Candidate counterexample | Prior work it must go beyond |
|---|---|---|---|
| R1 | Authority safety analysis: for the policy fragment with attenuating delegation, constitution and declared amplification, "can principal p ever obtain right r" is decidable, with a complexity bound | Unrestricted constitution, which reduces to the undecidable access-matrix problem | Decidability for the RT trust-management family under restricted policy change ([Li, Mitchell & Winsborough](https://www.cs.purdue.edu/homes/ninghui/papers/analysis_oakland03.pdf)) |
| R2 | Accountability under delegation, as three statements over a fixed causal definition of deviation: (a) every violation caused by a covered deviation yields a verdict naming the deviating parties; (b) no party that complied is named; (c) violations with no deviation (rule change, external failure) are classified as such and the loss falls on a bearer fixed by a declared allocation rule. The composition claim: a chain verdict can be computed from per-unit verdicts without reading inner logs | Variant 6, where everyone complied; joint causation across two units where neither unit-level verdict is sufficient | Protocol accountability ([Künnemann et al.](https://arxiv.org/abs/1805.10891); [Küsters et al.](https://eprint.iacr.org/2010/236)); compositional blame for multiparty contracts ([Hvitved et al.](https://dblp.org/rec/journals/jlp/HvitvedKZ12)) |
| R3 | Norm conformance across vocabularies: where each level states its norms in its own vocabulary and adjacent levels are linked by declared "counts as" mappings, pairwise design compliance plus leaf enforcement implies end-to-end compliance, under conditions on the mappings | Two mappings that each preserve compliance and whose composite does not, because an abstraction merges events the root distinguishes | Same-vocabulary prohibitions enforced by construction in law-governed interaction ([Minsky](https://arxiv.org/abs/1403.4321)); two-level compliance checking ([King et al.](https://researchportal.bath.ac.uk/en/publications/automated-multi-level-governance-compliance-checking/)) |
| R4 | Rule change with commitments in flight: under a stated activation protocol (acknowledged epochs or expiring leases, refusing when unsure), every change either preserves each commitment made under the old rules or produces a recorded violation with a named bearer, and named application invariants such as the new cap hold after the effective time | Variant 9: a controller that has not heard of the change authorises under the old cap | Versioned policy; timed common knowledge |
| R5 | Coordination classification: a sound check over a named restricted language of operations, invariants and merge rules, returning one of four answers: needs no coordination; local after pre-allocation of rights; needs an owner or committee, with a witness; unknown | The check is useless if most benchmark invariants come back unknown | Invariant confluence ([Bailis et al.](https://arxiv.org/abs/1402.2237)), undecidable in general ([Whittaker & Hellerstein](https://mwhittaker.github.io/publications/iconfluence_vldbj2020.pdf)); bounded counters ([Balegas et al.](https://arxiv.org/abs/1503.09052)) |
| R6 | Overlap: with one identity in two units and one resource funded by two, the nested and flattened readings agree on identities, effects and balances, and each resource is counted once | Variant 2 | Consolidation accounting; component models with shared sub-components† |
| R7 | Envelope for an unspecified actor: if the vendor stays within a declared rate and price envelope the purchase invariants hold, and a departure from the envelope is detected and attributed within a stated time | A departure that is only detectable after the invariant is already lost | Assume-guarantee contracts; shielding |

**Lemmas to reproduce, not counted as results.** Closure and substitutivity for the chosen core, with its hypotheses. Controller correctness: no action takes effect without a recorded derivation. Conservation of a resource over disjoint ownership. Conformance for same-vocabulary prohibitions.

**Work in this stage.**

- Attempt R2 and R3 by hand on the smallest chain, and R6 on the diamond.
- Write the scale argument of step 5 on paper, including the cap change.
- Send the results to external reviewers before going further.

**Stop rule.** If R2 has no true, non-definitional statement on a three-unit chain, publish that as a negative result for this candidate and stop. If every statement that survives follows from one module plus controller correctness, report that the integration adds nothing and stop.

## Step 2: choose the core by a bake-off

**Default hypothesis.** Timed input/output automata with dynamic creation. The untimed model ([Lynch & Tuttle](https://groups.csail.mit.edu/tds/papers/Lynch/CWI89.pdf)) has no deadlines and no creation. Dynamic I/O automata add creation and destruction, with substitutivity for creation only under technical conditions ([Attie & Lynch](https://arxiv.org/abs/1604.06030)); timed I/O automata add time, with conditions that stop a component blocking time†. The plan knows of no published substitutivity theorem for the two together. Substitutivity therefore carries two explicit hypotheses, and failure to obtain it for creation plus time is a reason to drop this core.

**Candidates, as concrete encodings and not as fields.** The default; interface automata and assume-guarantee contract theories; a governance-specific baseline (law-governed interaction with an institutional monitor); polynomial functors with wiring diagrams; bigraphs; reflective rewriting logic.

**Comparison.** Each candidate encodes benchmark variants 1 to 3, proves closure and substitutivity, and carries the paper proof of R2 on a chain. They are compared on the "smaller" measure and on which hypotheses each needs. An advocate of each candidate reviews its encoding, so that the author of the default does not judge alone. Existing mechanisations of each candidate are surveyed first and are an input to the choice of proof assistant†.

**If the core changes,** the module contracts are reassessed; they are not assumed to carry over.

## Step 3: mechanise

Mechanise the core, the five structures (authority, resources, commitments and norms, observation log and time, rule change) and whichever targets survived step 1.

**Rule change.** A unit's rules are controller state. An amendment is valid when it passes the unit's own charter procedure and conforms to the constraints of the unit that contains it; with role instances in a tree, that unit is unique. A charter may contain one amendment rule that applies to itself, with each amendment checked by the rules in force at the previous version, which keeps the versions well-founded. The chain of containing units ends at a named anchor.

**Separability hypothesis.** The structures are separate modules. It fails if the proof of any structure's target imports definitions from more than one other structure. On failure the plan drops the claim and reports the dependency graph.

## Step 4: reference controller and benchmark run

- Write an executable controller and policy evaluator for the decidable fragment.
- Test it against the mechanised model on sequences of transitions, concurrent revocation, crash recovery and version change. The model's theorems are proved. The executable's conformance is tested, not proved, and an error in it can violate every invariant on an untested execution.
- Run all ten variants with language-model agents, scripted humans and an unmodified vendor service, and check the required outcomes of each.

## Step 5: scale

Three bounds are stated as functions of population N, depth d and fan-out b, and each is first argued on paper in step 1.

- **Per-decision cost.** Acceptable growth is proportional to depth, with depth logarithmic in N.
- **Load per controller per unit time**, under a stated rate of active principals. A controller whose load grows with N must be shown to shard, including its rules, balances and log. Upper-level units have unbounded direct membership, so fan-out is not assumed bounded.
- **Rule-change cost and latency.** A cap change is one operation including its propagation. Acceptable cost is proportional to the affected subtree, with a stated maximum latency, and controllers that have not received the current version refuse; the resulting unavailability is counted.

The workload mix and the largest acceptable share of coordinated decisions are fixed before measuring, taken from a real procurement workload if one can be obtained. Simulation then measures the constants at increasing sizes, including shared-root and overlapping-membership workloads.

**The scale claim fails if** any controller is an unshardable bottleneck at the target workload, or any bound exceeds its declared growth, or the coordinated share exceeds its ceiling. Correlated faults, anchor failure and adversarial load are not covered.

## How the plan could fail as a whole

- **The formal model does not match the world.** Verification moves risk to the specification. The defence is the benchmark and review by people who run real procurement, and it is weak.
- **Most consequential actions are unmediated,** including whatever one person carries between their own roles. Then the foundation governs little.
- **The research targets collapse.** If R1 to R7 fail or turn out definitional, the stop rules fire and the result is negative for this candidate.
- **Trust anchors dominate.** If every guarantee reduces to the honesty of the identity anchor, the root rules and one witness per unit, the foundation is an accounting system for trust. That may still be useful, and the plan would say which it is.

## Order of work

1. Definitions, threat model and benchmark family (step 0). **Decision:** do the definitions pass the exit test?
2. Paper stage (step 1) and external review of it. **Decision:** do the stop rules fire?
3. Bake-off (step 2). **Decision:** which core, and does substitutivity hold for it?
4. Mechanisation (step 3). **Decision:** are the structures separable?
5. Reference controller and benchmark run (step 4).
6. Scale measurements (step 5).
7. External adversarial review of the theorems and the evidence.

No effort estimates are given. An estimate becomes possible after the paper stage.

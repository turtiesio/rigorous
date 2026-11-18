# Plan for an integrated foundation

**Status.** This is a proposal. It follows from the survey in [Compositional foundations for governed systems.md](/root/mic/reports/Compositional foundations for governed systems.md), which found that small rigorous substrates exist, that each is closed under its own composition, and that nobody has composed them into one foundation with a stated composition law for authority, obligation and liability. Every design choice below is a hypothesis to be tested, and the plan says how each one could fail. Citations marked † are from memory and were not fetched while writing this plan.

## What the plan commits to

Build the smallest mechanised calculus in which a unit at any level (resource, program, service, human, agent, team, organisation, enterprise, government, nation) is the same kind of object, carries the same governance interface, and composes under theorems stated property by property. Prove a short list of composition theorems, run one worked example that crosses all ten levels, and measure whether decision cost stays local as the population grows.

The plan does not commit to a deployed system for 10^10 actors. It commits to producing the evidence that would show whether one is buildable.

## Step 0: make the target testable

The survey's reviewers repeatedly found that "controlled", "smaller", "fractal" and "scale" had no test. The first deliverable is a definitions document that fixes them.

- **Unit.** An open transition system with input, output and internal actions and a declared interface.
- **Fractal.** Three checkable statements. Closure: a composite of units is a unit. Substitutivity: a part can be replaced by a part that refines it without rechecking the whole. Uniform governance interface: a composite exposes the same six governance structures (listed in step 2) as its parts.
- **Controlled.** A named list of properties. Each property has a formal statement, one of four enforcement classes, a composition theorem with its side conditions, and an evidence level (proved, tested, or measured).
- **Enforcement classes.** Prevent (a monitor at a boundary blocks the action). Prove (the actor supplies a certificate that a small checker verifies). Bound (a conservative model of an unspecified actor is assumed and its validity is monitored). Account (the action is logged, attributed and sanctioned after the fact; this gives deterrence and not a guarantee).
- **Smaller.** Measured, not asserted: size of the trusted base (definitions, checker, axioms, trust anchors) plus the encoding overhead on the worked example.
- **Scale.** A bounded claim: 10^10 principals under a stated workload and fault model. The test is locality: the cost of one governance decision, and the number of parties that must coordinate for it, do not grow with the total population.

**Exit test.** Two independent reviewers can each say, for any proposed system, whether it meets each definition.

## Non-goals

These follow from results the survey cites, and the plan does not try to get round them.

- No complete automatic verifier for arbitrary units. Guarantees come from restricted languages, supplied certificates, monitors or audit.
- No unconditional self-certification. The foundation names its trust anchors.
- No optimal decentralised control and no ideal preference aggregation. Each unit declares its own decision procedure.
- No guarantee about human or learned behaviour outside a mediated interface or a declared behavioural bound.
- No strict Sybil-proofness without an identity anchor.
- No general liveness. Obligations are deadlines with sanctions.
- No claim about legitimacy. The calculus records who recognised what; it does not decide whether they should have.

## Step 1: choose the semantic core by a bake-off

**Default hypothesis (H1).** The core is input/output automata ([Lynch & Tuttle](https://groups.csail.mit.edu/tds/papers/Lynch/CWI89.pdf)): composition of compatible automata, hiding, and refinement as trace inclusion. The reasons are that it is the smallest established baseline, it separates actions the environment controls from actions the unit controls, and refinement is substitutive under its compatibility conditions. A known cost is input-enabledness: a unit cannot refuse an input, so refusal has to be modelled as an explicit reject output.

**Bake-off.** Encode the worked example (step 3) at small size in four candidates: the I/O automaton or TLA baseline, polynomial functors with wiring diagrams, bigraphs, and reflective rewriting logic (Maude). Compare them on encoding size, trusted assumptions, which of the theorems in step 4 can be stated, and which can be proved with existing metatheory.

**H1 fails if** another candidate states and proves the same theorems with a smaller trusted base, or if nesting with shared membership (a principal who belongs to several units) cannot be expressed in the baseline without ad hoc machinery. In that case the plan switches core and keeps the rest.

## Step 2: six governance structures as modules over the core

Each structure is typed state plus invariants on the core, with its own theorem. The survey's nine-element checklist omitted identity, time, and observation; they are included here.

1. **Identity.** Principals have identifiers issued by a named anchor. The anchor is an assumption, stated per deployment.
2. **Authority.** Three explicit operations: delegation that can only attenuate; constitution, where a recognised procedure creates authority that no delegator held (a vote, a charter); and amplification, where combining two authorities yields a third, declared in advance. Policies are written in a decidable fragment so that a decision can be evaluated locally.
3. **Resources.** Two types kept apart: conserved resources that cannot be copied (budgets, tokens) and persistent ones that can (knowledge, shareable permissions).
4. **Commitments and norms.** A commitment is a directed promise with a lifecycle (created, discharged, cancelled, delegated, violated). A norm is data with a monitor semantics, a deadline and a sanction, not a modal formula.
5. **Observation and time.** Each interface declares what each party can observe. Events are logged with provenance. Clocks are assumed synchronised to within a stated skew, which is what makes deadlines and effective dates meaningful.
6. **Rule change.** The rules a unit enforces are part of its controller's state. Changing them is an action governed by rules held one level up, or by a distinct procedure in the unit's own charter. The stratification is finite and ends in an anchor procedure outside the system. Rules are versioned and take effect at a pre-announced time.

Every composite has a boundary controller: a distinguished component through which every action crossing the boundary passes. The controller is an ordinary unit at the next level. Containment is a graph and not a tree, because one principal can belong to several units.

**H2.** These six can be specified as separate modules whose theorems do not depend on each other's internals. **H2 fails if** the modules interact so much that each theorem needs the whole system (revocation of authority over a conserved resource is the first place to look).

## Step 3: one worked example across all ten levels

A public procurement chain. A national government, bound by a supranational spending rule, grants a budget to an enterprise. The enterprise passes part of it to a subsidiary organisation, which assigns it to a team. A learned agent on the team buys compute from an external service. A human must approve purchases above a threshold. Midway, the government lowers the spending cap, and the change has to propagate.

The example exercises every level, a human-mediated action, a learned component, a third-party service whose behaviour is not specified, delegation across four hops, a conserved resource, a commitment with a deadline, a rule change, and a constraint above the nation. It is small enough to encode by hand and is the fixed benchmark for the bake-off and for every later step.

## Step 4: the theorems to prove

Listed from most to least likely to succeed. Each is a target, not a result. T1, T3 and T7 are integrations of results the survey reports as established; T2 and T6 adapt known techniques; T4 and T5 are the open research.

| | Target | Enforcement class | Prior work to reuse | Main risk |
|---|---|---|---|---|
| T1 | Closure and substitutivity for nesting: composing and hiding gives a unit, and refinement of a part gives refinement of the whole, for safety properties | Prove | I/O automata composition; TLA composition theorem | Shared membership; liveness excluded |
| T2 | Authority confinement: inside a composite, no principal acts with authority that is not derivable from recorded delegations, constitutions and declared amplifications; without revocation this is checkable offline | Prevent | Object-capability confinement proofs†; proof-carrying authorization; take-grant decidability† | The general access-matrix safety problem is undecidable, so the policy fragment must stay restricted |
| T3 | Conservation: balances of a conserved resource consolidate across nested units by addition | Prevent | Double-entry algebra; linear and separation logic | Non-negative shared balances need an owner or a small committee |
| T7 | Coordination classification: a static check sorts each declared invariant into "needs no coordination" or "needs an owner or committee" | Prove | Invariant confluence ([Bailis et al.](https://arxiv.org/abs/1402.2237)) | The check is defined for database transactions; extending it to this calculus is new work |
| T6 | Rule-change safety: a rule change accepted by the stratified checker preserves T2 and T3 | Prove | Supervisory control; reflective logics; versioned policy | Interaction with in-flight commitments made under the old rules |
| T5 | Norm conformance down a hierarchy: if each unit's rules comply with its parent's, the leaf's behaviour complies with the root's, under stated conditions | Prevent and account | Multi-level compliance checking ([King et al.](https://orca.cardiff.ac.uk/id/eprint/130977/1/AS4805164377456641491575454833_content_1.pdf)); law-governed conformance hierarchies | Conflicts between norms of overlapping units; compliance may not be transitive without restrictions |
| T4 | Delegation accountability: if a commitment is violated and the logs are intact, the calculus identifies at least one responsible principal on the delegation chain and never blames a principal who complied | Account | Accountability definitions for protocols ([Künnemann et al.](https://publications.cispa.de/articles/conference_contribution/Automated_Verification_of_Accountability_in_Security_Protocols/24612942/1)); tamper-evident logs† | This is the composition law the survey found missing; it may hold only for tree-shaped delegation |

**Fallbacks.** For T4, restrict to tree-shaped delegation, then to chains. For T5, restrict to norms that only prohibit. If T4 fails even for chains, the plan reports that as a negative result, since it would mean accountability does not compose even in the simplest case.

**Not targeted.** General liveness, incentive compatibility, alignment of learned components, and probabilistic guarantees. Learned and human actors are treated as unspecified environments behind a boundary controller in the first version.

## Step 5: mechanise, then build a reference monitor

- Mechanise the core, the six modules and the theorems in one proof assistant. The choice between Isabelle/HOL, Rocq and Lean is made after the bake-off, on the availability of existing libraries for the chosen core.
- Write an executable boundary controller and policy evaluator for the decidable fragment.
- Tie the executable to the mechanised model by differential testing, the method used for Cedar ([Disselkoen et al.](https://arxiv.org/abs/2407.01688v1)). This leaves the specification and the test oracle as trusted, and the plan says so.
- Run the worked example with real language-model agents, scripted humans and an unmodified external service.

## Step 6: evidence about scale

The claim to test is locality, not deployment at 10^10.

- For each decision type in the worked example, count the parties that must be contacted and the state that must be read. Show by argument over the calculus that neither depends on total population, given bounded delegation depth and bounded unit size.
- Measure constants in simulation at increasing sizes and check that per-decision cost stays flat.
- Report separately the decisions that need coordination (T7's second class), the size of the committee each needs, and how often they occur in the workload.
- State what is not covered: failures of anchors, correlated faults, adversarial load, and anything a simulation cannot show.

**Scale claim fails if** any decision type has cost that grows with population, or if the coordinated class dominates the workload.

## How the plan could fail as a whole

- **The formal model does not match the world.** Verification moves risk to the specification. The plan's only defence is the worked example and review by people who run real procurement, and that is weak.
- **Humans act outside the interface.** The calculus governs mediated actions. If most consequential actions are unmediated, the foundation governs little.
- **Overlapping membership produces norm conflicts with no principled resolution.** The first version requires each unit to declare precedence. That may be unacceptable in practice.
- **The integration adds nothing.** If T1 to T3 and T7 are the only theorems that go through, the result is a tidy repackaging of known work. The plan counts that as a negative result on the survey's open question.
- **Trust anchors dominate.** If every interesting guarantee reduces to "assuming the identity anchor and the root rules are honest", the foundation is an accounting system for trust and not a source of it. That may still be useful, and the plan would say which it is.

## Order of work and decision points

1. Definitions (step 0) and the worked example (step 3). **Decision:** are the definitions testable?
2. Bake-off (step 1). **Decision:** which core?
3. Core plus T1, T2, T3, T7 mechanised. **Decision:** are the modules separable (H2)?
4. T6, T5, T4 with fallbacks. **Decision:** does any delegation or norm composition theorem hold?
5. Reference monitor and the worked example run end to end (step 5).
6. Locality argument and measurements (step 6).
7. External adversarial review of the definitions, the theorems and the evidence.

No effort estimates are given. The survey's only cost data are for single verified artefacts, and they do not transfer to a calculus whose core is not yet chosen. An estimate becomes possible after the bake-off.

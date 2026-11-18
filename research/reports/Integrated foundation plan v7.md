# Plan for an integrated foundation

**Status.** This is a proposal, in its seventh version. It follows from the survey in [Compositional foundations for governed systems.md](/root/mic/reports/Compositional foundations for governed systems.md), which found that small rigorous substrates exist and that nobody has composed them into one foundation with a stated composition law for authority, obligation and liability.

Six rounds of independent adversarial review shaped it (files beginning "plan_attack_" in /root/mic/reports, with earlier versions of the plan beside them). From round 3 onward both reviewers judged the plan sound enough to start its first step. In every round, though, the serious errors were in design details written into the plan between rounds: a witness scheme, a conflict rule, a challenge period, a rule for dropped obligations. Each fix was a one-line mechanism that had not been checked against the rest. This version therefore changes what the plan commits to. It fixes the architecture that has survived review, and it turns each contested mechanism into a design decision for step 0, listing the options, what the reviews showed goes wrong with each, and the small cases that have to pass before the choice is frozen. Citations marked † are from memory and were not fetched; the reviewers checked the others at varying depth.

## What a positive result would and would not establish

A successful run would show that mediated, recorded interactions among units with specified controllers compose with stated guarantees about authority, resources, commitments and rule change, and that the guarantees survive nesting, several rule authorities over one unit, and creation and dissolution of units.

It would not show that a foundation for governing humans, learned agents and institutions exists in the full sense of the original goal.

- Humans, learned agents and third-party services are environments. The plan proves things about what their controllers let through and what is recorded.
- "Self-reflecting" is delivered as rule change checked by the rules in force, with a fixed set of entrenched invariants and a named anchor.
- "10^10" is delivered as stated growth bounds and a load calculation, not a deployment.
- All rule authority descends from one anchor, and the rule authority relation is acyclic. Mutual oversight between units, authorities that units found over themselves by consent, and conflicts of laws with no common superior are not modelled. If the default priority rule of decision D3 is kept, the result is about single-rooted ranked hierarchies.
- Where authorities conflict, a lower-priority norm is overridden on the record, so the guarantee is conformance to a resolved set of norms.
- Witnesses, the anchor, the time source and the identity issuer are components and not units, so the recursion stops at parts the foundation does not govern. For each surviving theorem the plan lists the anchors it depends on.
- Legitimacy, incentives, alignment, confidentiality and general liveness are outside it.

Three targets are open research (R1 to R3) and two are narrower (R4 and R5). Everything else is reuse of published results or application lemmas, and the plan reports reuse and novelty separately. A failed target is evidence against this candidate and its encoding, not an impossibility result about the goal.

## Non-goals

- No complete automatic verifier for arbitrary units.
- No unconditional self-certification. Trust anchors are named.
- No optimal decentralised control and no ideal preference aggregation.
- No guarantee about human or learned behaviour outside a mediated interface or a declared envelope.
- No strict Sybil-proofness without an identity anchor. Identity is an assumption of every result.
- No general liveness. Obligations are deadlines in a timed model.
- No confidentiality or privacy guarantee.
- No incentive compatibility. Sanctions are recorded and executed; whether they deter is not claimed.
- No accountability for effects that only colluding parties ever see.
- No claim about legitimacy.

## The architecture that is fixed

These parts have been stable across the last four rounds of review.

**Unit.** An open transition system with input, output and internal actions, a declared interface, and a clock.

**Principals and roles.** A principal is an identity plus a set of role instances. Each role instance belongs to exactly one unit and each unit has at most one containing unit, so component containment is a tree. A unit can be a principal and hold roles in other units. What an identity carries between its own role instances in different units is not mediated.

**Four separate relations.** Membership. Component containment. Resource ownership: each resource has exactly one owning unit, and other units may hold recorded funding claims on it. Rule authority: a declared acyclic relation, independent of containment, under which a unit may have several authorities.

**Controllers.** Every unit has an atomic controller that owns the effect of every protected action: other components send requests, and only the controller emits the effect. Where load requires it, a unit has several atomic controllers (shards). Each owns a disjoint set of resources and rules, a function of the resource name assigns each protected action to one shard, one shard owns the charter, and an action that needs two shards is counted as a coordinated action.

**Evidence.** Effects carry receipts. Each unit has a set of witnesses, appointed from outside the unit, that hold its log and replay its controller against a reference implementation. Controllers are deterministic given their recorded inputs; requests from humans and learned agents, event order and clock readings are inputs, and their reasoning is not replayed. An external party that cannot check receipts is reached through a gateway unit.

**Verdicts.** Three kinds, kept distinct: proved deviation, suspicion, and allocated loss where nobody deviated. Only a proved deviation names a party. A conflict or a missed deadline is not by itself evidence that nobody deviated.

**Assumptions.** Any non-controller component may deviate and any set may collude. Prevention results assume correct controllers. Accountability results allow a controller and a bounded number of its witnesses to deviate, and cover only effects for which an honest party holds a receipt. Links between a controller and its own witnesses have a delivery bound; links between units and to vendors do not. Clocks are synchronised to a stated skew by an external time source. The identity issuer, the root rules and the time source are trusted.

**Fractal.** Closure, substitutivity under stated hypotheses, and a uniform governance interface: every unit exposes authority, resources, commitments and norms, a log, and a rule-change procedure.

**Controlled.** A list of named properties, each with a formal statement and an assurance record: the mechanism on two axes (prevented, certified or detected; enforced or assumed), assumptions, what is observable, detection time, intervention per observer, and what is left unprotected. A composite's record is derived by candidate rules that step 1 checks. An assumption is removed only by instantiating a sound assume-guarantee rule with all its premises ([Abadi & Lamport](https://lamport.azurewebsites.net/pubs/abadi-conjoining.pdf)), and two promises that are each conditional on the other do not discharge each other. Detection time is a local bound plus a delivery term for each link, which may be unbounded. Where the evidence cannot support a conclusion the record is "unestablished". If the derivation depends on how units are grouped, the claim that assurance records are fractal is withdrawn.

**Useful behaviour.** Each benchmark variant lists outcomes that must occur whenever the delivery bounds of a stated performance envelope hold. A controller that rejects everything inside the envelope fails.

**Smaller.** Definitions and hypotheses counted on paper in a common notation first, then lines of definitions plus axioms and anchors in a proof assistant. Domain definitions, trusted assumptions, reused libraries and proof effort are counted separately, as is the runtime trust base. No global minimality is claimed.

## Step 0: decisions, then frozen statements

Step 0 has two parts, in this order. First the design decisions below are made, each checked on its listed cases and reviewed by the external reviewers. Only then are the formal statements of the targets frozen.

### Design decisions

Each row is an open choice. "Known problems" records what the reviews found; none of the options is asserted to be sound.

| | Decision | Options | Known problems | Cases that must pass |
|---|---|---|---|---|
| D1 | When norm conflicts are evaluated, and what becomes of an obligation that loses | Evaluate when the later norm takes effect; or define compliance as staying among states from which the selected norms remain jointly satisfiable, with the step that leaves them being the violation; or evaluate at the current state and attribute causes | Evaluating at the current state and treating a dropped obligation as not owed lets a unit escape an obligation by waiting until it conflicts or becomes impossible. The second option makes conflict detection a question of what the unit can guarantee against its environment, which costs more | A unit that waits until only a forbidden vendor can deliver; a single obligation that has become impossible |
| D2 | What "jointly satisfiable" means for an open unit | Some behaviour of the environment allows it; or the unit can ensure it whatever the environment does | The published selection procedures work on literals with a fixed context ([Liao et al.](https://arxiv.org/pdf/1709.08034)), so consistency has to be proved for the plan's own norms and cannot be inherited; one of the published procedures does not give a unique result even under a total order | Three obligations of which any two can be met and all three cannot |
| D3 | Priority among authorities | A total order from a rank fixed at creation and then an identifier, adjustable only by the anchor; or a partial order with adjudication of undecided conflicts | Partial order: refusal while a conflict is pending can last without bound, a missed deadline needs a classification, and a late adjudication races with any default. Total order: it is a strict hierarchy; someone must assign ranks from a stated domain; a new unit needs its first authority edge and its witness appointer from a creation rule; the anchor's priority table is a dependency of every unit and must be versioned and leased; two norms of one authority need a sequence number | The regulator obliges what procurement law forbids; two norms from one authority issued within the clock skew; an authority edge from a lower rank to a higher one |
| D4 | Which norms are in a unit's effective set | Only the norms of its direct authorities, with an ancestor's norm binding through each intermediate authority's implementation; or the norms of all ancestors, each keeping its origin | All ancestors with checks on edges: a check on each edge can pass while the unit breaks an ancestor's norm that no edge covered. Two paths can translate one norm differently, so the translations must be required to agree or one path made canonical; counting the norm once does not settle which translation holds. An implementing norm carries the implementer's priority unless stated otherwise | The state's norm, the ministry's implementation of it and a conflicting regulator's norm; a ministry norm reaching the subsidiary directly and through the group with contradictory translations |
| D5 | Whether an obligation from a unit's own commitment is a norm in the selection | No: a later authority norm that conflicts with a commitment is a rule change under R4; or yes | If yes, commitments have the lowest priority and any later conflicting norm makes them vanish with no loss recorded | A commitment made before a conflicting norm is issued; a norm later shown to breach its authority's charter |
| D6 | How a resource invariant such as a spending cap relates to norms | A tier above all norms, declared only by an authority over the resource's owner, with a conflicting higher-priority obligation recorded as overridden; or subject to selection like any norm | Exempt with no further rule: a lower authority's cap defeats a higher authority's obligation, and a unit could cap its own resource to avoid paying. Subject to selection: the cap can be lawfully overridden, which contradicts R4 | A higher-ranked authority obliges a purchase that exceeds the ministry's cap; a second commitment that would exceed the cap |
| D7 | What is claimed for obligations | Every miss of an obligation in the effective set is detected and classified; or compliance under a named assumption about the environment for each obligation | A controller can refuse an action and cannot make a human act or stop the clock, so "enforcement implies compliance" is false for obligations | An approval the officer does not give before the deadline |
| D8 | Witness quorum and forked logs | 2f+1 witnesses with f+1 signatures and verdicts that may be revised; or a delivery bound between a unit's witnesses, who exchange signed log heads before signing; or 3f+1 witnesses with 2f+1 signatures | With f+1 of 2f+1, two valid summaries of different prefixes of a forked log can exist. A fixed challenge period is unsound when links between units have no delivery bound. A verdict that is "final when every link met its bound" cannot be observed by any party, and evidence withheld by a colluder and released later either reopens it or is ignored, so the choice is between no finality with unbounded retention and an explicit time bar. Who appoints each witness set must be named: in the benchmark the ministry is both an authority over the group and its counterparty | A fork shown to two honest witnesses with the exposing receipt delivered late; a colluding recipient that presents its receipt after the verdict; a witness that lies in a summary; a colluding witness that countersigns |
| D9 | Whether accountability is stated as eventual or as a bounded response | "Eventually yields a verdict"; or "within a stated bound after the evidence is logged at the unit that computes the verdict" | The eventual form is a liveness property. It is not decided on finite prefixes, it is not covered by an assume-guarantee rule that discharges only safety assumptions, and a refuter of the form "verdicts do not converge once all evidence has arrived" can hardly fire | Two orders of arrival of the same evidence; a verdict later than the bound |
| D10 | Discharge of an obligation through the obligee's witnesses | Which of the presented receipt times counts; whether the controller must then log the performance | If the earliest or median time counts, faulty witnesses can backdate a late performance; only the latest presented time is safe against that, and it makes timeliness depend on the slowest link | A late performance backdated by f witnesses; an obligee's controller that drops the performance |
| D11 | The ledger for an aggregate invariant across rule versions | Categories for completed spending, outstanding commitments, exercisable rights, and allocations that have expired and are not yet settled | Subtracting only known spending lets a late shard and a new allocation together exceed both caps. Expiry stops further use and does not tell the owner how much was spent, so an expired allocation stays reserved until an authenticated final balance arrives. With two changes inside one lease period the bound is the largest cap among versions that still have unexpired or unsettled rights | Cap 100 to 95 with 10 spent and 90 held by a late shard; cap 100 to 120 with 100 held by a late shard; two changes in one lease period; spending just before expiry reported after it |
| D12 | How creation is modelled | Timed input/output automata with dynamic creation, which needs new metatheory; or a pool of dormant units in a model whose composition theorems are published | A pool result holds for prefixes with at most as many creations as the pool has members. It does not give eventual claims under continuing creation, and a required outcome fails when the environment asks for one more unit than the pool holds. So a pool result is a result about bounded populations | Creation in a number set by an environment input; creation continuing while an earlier event awaits its verdict |
| D13 | The availability model | A joint model of renewal delays across links and time; or measured availability per fixed graph and workload | A per-link probability does not determine availability without the dependence between links. Under eventual delivery alone the worst case is zero | One authority against several independent ones at the same depth |

### Benchmark family

A public procurement chain.

- **Containment.** A state contains a ministry. A parent group (enterprise) contains a subsidiary (organisation), which contains a team, which contains an approving officer's role (human) and a purchasing agent (learned agent) with its purchasing tool (program). Resources are a budget and purchased compute. A compute vendor is an external service behind a gateway unit.
- **Rule authority.** A supranational body over the state; the state over the ministry and over an external audit body; the ministry over the group and the subsidiary through procurement law; the group over the subsidiary through group policy; a regulator in the group's home jurisdiction over the subsidiary, with no edge to or from the ministry; the subsidiary over the team and through it the officer's team role, the agent and the tool; the audit body over the officer's audit role.
- **Commitments and duties.** The ministry funds the group by a commitment. Procurement law and the state's audit law oblige the team and the audit body to report summaries of ministry-funded purchases to the ministry.

The variants are the compliant path and the following, together with every case in the decisions table.

1. A diamond: the officer also holds a role in the audit body, and the budget carries funding claims from two units.
2. Formation: a charter creates the subsidiary under a power granted by the group, with one declared amplification (officer approval combined with the agent's budget right). The subsidiary then creates project units that may create further units.
3. Revocation and claw-back of a partly spent grant, concurrent with a purchase.
4. Creation, a deadline, then dissolution of the subsidiary with a commitment outstanding.
5. Vendor failure with every chain principal compliant, and a disputed delivery decided by a named adjudicating unit.
6. Purchases split to stay under the approval threshold, with the team and the audit body naming the purchase differently.
7. A controller omits a log entry, once where an honest recipient holds a receipt and once where only a colluding recipient saw the effect, which must come out as outside coverage.
8. The ministry changes the spending cap while purchases are in flight and one shard hears late.
9. An amendment by the subsidiary that satisfies the group and violates procurement law; an authority that issues a norm its own charter forbids.
10. The subsidiary validly amends its own amendment rule while a commitment is outstanding, then attempts amendments that remove the logging duty, narrow the definition of "protected action", or change what the witnesses replay.
11. An early breach that is irrelevant: one unit breaks a guarantee the chain property does not depend on, and a later unit causes the violation.

After the core is chosen, the external reviewers write one further variant that the authors have not seen.

### Frozen after the decisions, and stated over traces

- For R1: the grammar of the policy fragment, its restriction rule, whether quantities are bounded, whether principals can be created during the analysis, the semantics of revocation, how quantities and rule syntax are sized, and the complexity class that counts as success for each of the two questions.
- For R2: the causal definition of deviation, the verdict type, the content of a receipt and of a per-unit summary, and a bound on summary growth in terms of activity at the unit's boundary.
- For R3: the class of mappings between vocabularies, the selection procedure, the bound for conflict detection, and the local compliance predicate with the size of the description of the selection it may use.
- For R4 and R5: the rule for which commitments survive a rule change and a class of useful changes it accepts; the entrenched invariants, the meaning of "preserves", and a class of useful self-amendments that must be accepted.
- Thresholds: ceilings on "unknown" classifications and on coordinated decisions, the availability floor, and the number of runs and pass rate with language-model agents.
- A ledger giving, for each reused result, its source, the theorem, and how it is instantiated.

### Exit test

Two reviewers independently judge ten named systems, clause by clause, against each definition. The ten include a law-governed interaction deployment, a Cedar policy store, a capability-token deployment, a decentralised autonomous organisation, a national procurement regime and an open message bus with no governance; two small constructed examples are added for the composite definitions. The step passes if chance-corrected agreement reaches a level fixed in advance, each clause is judged satisfied by at least one existing system and unsatisfied by at least two, the message bus is rejected, and every design decision has passed its cases and the reviewers' check.

### Sources to read in full first

The multi-level compliance paper; the timed and the dynamic input/output automata theorems; the accountable-log protocol; the compositional-blame and component-causality papers; the assume-guarantee composition theorem; prioritised norm selection ([Hansen](https://link.springer.com/doi/10.1007/s10458-007-9016-7); Liao et al.); conflict detection between interacting legal systems ([Li et al.](https://researchportal.bath.ac.uk/en/publications/legal-conflict-detection-in-interacting-legal-systems/)), which may already contain part of R3; fork consistency and fail-aware storage ([Li et al.](https://www.usenix.org/legacy/event/osdi04/tech/full_papers/li_j/li_j.pdf)); and the paper cited for conformance hierarchies, whose abstract does not mention them.

## Step 1: paper stage

Done by hand in whichever core step 0's decision D12 points to. It comes before any mechanisation because it is cheap and decides whether the rest is worth doing.

**Open research targets.** The exact statements are frozen in step 0; what follows is their intent, how each could be refuted, and what each must go beyond.

| | Target | How it could be refuted | Prior work it must go beyond |
|---|---|---|---|
| R1 | Authority analysis. For a fragment with consumable quantities and revocation, and a restriction rule naming which units' rules and amendment procedures are fixed, "can p ever obtain r" and "does role A always contain role B" are decidable within the frozen complexity class. Amendment is an action in the analysis | Everything is reachable after an amendment; the analysis misses the complexity target in the size of the policy as written; or the fragment the benchmark needs is undecidable or non-elementary. Observing only that a finite-state encoding is decidable does not count | Trust-management analysis under a restriction rule ([Li, Mitchell & Winsborough](https://www.cs.purdue.edu/homes/ninghui/papers/analysis_oakland03.pdf)); reachability for administrative role-based policies ([Sasturkar et al.](https://www3.cs.stonybrook.edu/~stoller/arbac-analysis-TCS.html)); consumable authority in a linear access-control logic ([Bowers et al.](https://www.mpi-sws.org/~dg/papers/NDSS07Final.pdf)) |
| R2 | Accountability under delegation. With verdicts that are sets of minimally sufficient sets of parties: every covered violation caused by a deviation yields a proved-deviation verdict naming the deviators, in the form D9 fixes; no proved-deviation verdict ever names a compliant party, including when a neighbour's witness bound has failed; and a violation with no deviation is allocated to a bearer by a declared rule. The composition claim: the verdict for a chain can be computed from signed per-unit summaries, one entry per guarantee with the assumptions it depends on and the boundary trace prefix the counterfactual needs, without reading inner logs | Two histories with the same summaries that require different chain verdicts, as with joint causation or the early irrelevant breach; a compliant party named; summaries that exceed the frozen growth bound | Protocol accountability ([Künnemann et al.](https://arxiv.org/abs/1805.10891); [Küsters et al.](https://eprint.iacr.org/2010/236)); compositional blame for contracts ([Hvitved et al.](https://dblp.org/rec/journals/jlp/HvitvedKZ12)); blame and causality in component systems ([Gössler & Le Métayer](https://inria.hal.science/hal-00873665); [Caltais et al.](https://arxiv.org/abs/1608.07880), whose compositional result covers only non-communicating systems); accountable logs ([PeerReview](https://haeberlen.cis.upenn.edu/papers/peerreview-tr.pdf)); threshold witness cosigning ([Syta et al.](https://arxiv.org/abs/1503.08768)); receipts with replay and individual blame ([Shamis et al.](https://www.usenix.org/conference/nsdi22/presentation/shamis)) |
| R3 | Conformance across vocabularies and several authorities. A local check at each authority edge, each using only a description of the selection within the frozen size, implies the conclusion D7 fixes about the unit's effective norm set. Separately, for the frozen grammar, finding the minimal conflict sets is decidable within the frozen bound under the reading D2 fixes | Two edges that each pass the local check while the unit breaks a norm in its effective set; a minimal conflict that the detector misses or cannot find within the bound | Multi-level institutional compliance over an acyclic graph of institutions, with a three-level case study to reproduce ([King et al.](https://researchportal.bath.ac.uk/en/publications/automated-multi-level-governance-compliance-checking/)); conformance hierarchies in law-governed interaction ([Minsky](https://arxiv.org/abs/1403.4321)). Transitivity of sound trace mappings on one vocabulary, and uniqueness of a greedy selection under a total order, are not counted as results |

**Narrower targets.**

| | Target | How it could be refuted |
|---|---|---|
| R4 | Commitments across a rule change. Every change is either rejected or installed with an explicit set of grandfathered commitments within a frozen bound, and an aggregate invariant holds at every intermediate state in the form D11 fixes. With a single ledger this is close to the escrow method applied to a versioned bound, so it counts as a result only if the frozen statement has content beyond that, such as a limit on how long new spending is blocked after a cap is raised | The grandfathered set exceeds the frozen bound; total spending at an intermediate state exceeds the bound D11 fixes |
| R5 | Entrenchment under self-amendment. For the frozen grammar, the check that an amendment preserves the entrenched invariants terminates, is sound, including against amendments that change the meaning of a term, and is complete for the frozen class of useful self-amendments. The invariants are single ownership of resources, log continuity, the witness duty with outside appointment, the rule that what witnesses replay changes only with their appointer's consent, the obligation to run this check, and conformance to the effective norm set | An accepted amendment after which an entrenched invariant no longer holds; a preserving amendment in the frozen class that is rejected |

**Reused results and application lemmas.**

- Closure and substitutivity for the chosen core, which is published for the pool model and new work for the dynamic one.
- Complete mediation: with correct controllers, no protected effect occurs without a recorded derivation.
- Conservation of a resource with a single owner, with funding claims retained in both the nested and the flattened reading.
- The lease argument: after a rule change, a controller that has not heard stops authorising within a time fixed by the lease period and the clock skew ([Rae et al.](https://research.google/pubs/online-asynchronous-schema-change-in-f1/)).
- Coordination classification with existing tools ([Whittaker & Hellerstein](https://mwhittaker.github.io/publications/iconfluence_vldbj2020.pdf); [Balegas et al.](https://arxiv.org/abs/1503.09052); [Houshmand & Lesani](https://mohsenlesani.github.io/companion/popl19/POPL19.pdf)), reporting the share of benchmark invariants in each class.
- An assume-guarantee contract for the vendor, with departures from the envelope detected at a named observation point and disputes referred to the adjudicating unit.
- Detection of a breach of separation of duty across one identity's two roles from the summaries reported to the ministry (variant 6). If the frozen summary cannot carry what this needs, constraints across one identity's roles are added to what a positive result does not establish.

**Other work in this stage.** Derive the assurance records for a three-unit chain, on serial paths, alternative paths, a circular pair of deadline promises and an unbounded reporting link. Write the scale argument of step 4 on paper. Send everything to the external reviewers.

**Stop rules.**

- If the frozen statement of R2 is false on a three-unit chain and the weakest repair needs inner logs, publish that as a negative result for this candidate and stop.
- The work continues only if R2 survives and at least one of R1 and R3 improves on its stated baseline, as judged by the external reviewers. Otherwise report which criterion failed, keep any result that is new on its own, and stop.
- A statement repaired after a counterexample is reported as a new conjecture, not as the surviving target.
- A statement counts as a result only if it is refuted by some benchmark variant when one named hypothesis is dropped, which is necessary and not sufficient, and it is not an instance of a step-0 definition or of a result in the reuse ledger. A reviewer who says it is must exhibit the instance.
- If R5 is unsound, amendments are limited to a syntactic class that cannot touch the entrenched terms and the claim about self-reflection is withdrawn. If it is sound and incomplete, the claim is narrowed to what the check accepts. Only its conformance part depends on R3.
- If R4 fails, the bound on cap changes in step 4 is withdrawn.

## Step 2: choose the core

**Candidates.** Timed input/output automata with dynamic creation. The untimed model ([Lynch & Tuttle](https://groups.csail.mit.edu/tds/papers/Lynch/CWI89.pdf)) has neither deadlines nor creation; dynamic input/output automata add creation, with a substitutivity theorem that has six premises and allows a unit to end only by its own action ([Attie & Lynch](https://arxiv.org/abs/1604.06030)); timed input/output automata add time†. The plan knows of no published theorem under trace inclusion for the two together. Timed process calculi with creation ([Berger](https://link.springer.com/doi/10.1007/978-3-540-28644-8_8)) and timed transition systems with dynamic creation ([David et al. 2013](https://homes.cs.aau.dk/~kgl/SSFT2015/DYN.pdf)) are screened first. The alternatives are plain timed input/output automata with a pool of dormant units, and a timed specification theory with refinement, conjunction, composition and quotient ([David et al.](https://homes.cs.aau.dk/~adavid/publications/41-hscc10.pdf)), which also needs a pool. Law-governed interaction with an institutional monitor is a baseline for expressiveness only, because it assumes trusted controllers. Polynomial functors, bigraphs and reflective rewriting logic are left out to keep the cost bounded, so the plan does not test the categorical candidates the survey describes.

**Decision rule, in this order.** Can the candidate express the formation, dissolution, diamond and rule-change variants under the same observable requirements and environment assumptions? Does it support the frozen statement of R2? Which semantic obligations and trust assumptions does it need, taken from one common inventory, with incomparable conditions reported as such? How large is it on paper? Each encoding gets the same time budget, fixed in advance, and an advocate of each candidate reviews its encoding. "Not established within the budget" is an allowed outcome and is not evidence that a candidate is inferior. If no candidate is eligible the plan stops and reports it. If a pool model is chosen, every result is reported as a result about bounded populations.

## Step 3: mechanise and run

- Mechanise the core, the five structures (authority, resources, commitments and norms, log and time, rule change) and the targets that survived step 1. Each structure has a local lemma from the reuse list; the separability hypothesis fails if the proof of a local lemma imports definitions from more than one other structure. For R1 to R5 the dependency graph is reported without a pass or fail.
- An amendment is valid when it passes the unit's own charter procedure, conforms to the unit's effective norm set and passes the entrenchment check. A charter may contain an amendment rule that applies to itself; version n+1 is checked under version n.
- Write an executable controller, witness and policy evaluator for the decidable fragment, and test them against the mechanised model on sequences of transitions, concurrent revocation, crash recovery and version change. The model's theorems are proved. The executable's conformance is tested, not proved.
- Run every variant with language-model agents, scripted humans and a vendor behind its gateway, for the fixed number of runs. Controller-side outcomes (wrongly blocked, wrongly allowed) are reported separately from whether the agents completed the task.

## Step 4: scale

Containment depth is fixed by the benchmark and fan-out is not bounded, so the load bounds carry the content. Rules flow along the authority relation, so the bounds are stated in terms of the number of authority ancestors, the longest authority path, policy size, and the vertices and edges of the affected dependency closure. Each is first argued on paper in step 1, on cases that include a long authority chain and a wide join of authorities over shallow containment.

- **Per-decision cost,** in policy size, the number of authority ancestors, the cost of conflict detection and selection under D1 and D2, and the round trip for witness signatures.
- **Load per shard per unit time,** under a stated rate of active principals, including authorisation, lease renewal, receipt forwarding, and witness replay and its backlog. A unit whose load grows with population must be shown to shard. The anchor's load, including witness appointment and any priority table, is counted.
- **Summary, log and evidence growth,** including whatever retention D8 requires.
- **Rule change.** The time after which no controller authorises under the old rule is finite and is proved from the lease period and the clock skew along the longest authority path. Installation and recovery times are finite only under a stated bounded-delay assumption and otherwise appear as unavailability. Message cost is proportional to the affected dependency closure.
- **Availability,** under the model D13 fixes, with the worst case under eventual delivery reported as zero and a floor fixed in advance for the performance envelope.

The workload mix is fixed before measuring. **The scale claim fails if** any unit cannot be sharded at the target workload, any bound exceeds its declared growth, availability falls below its floor, or the coordinated share exceeds its ceiling. Correlated faults, anchor failure and adversarial load are not covered.

## How the plan could fail as a whole

- **The design decisions have no sound combination.** Step 0 then ends with the decisions that could not be reconciled, which is itself a finding about the candidate.
- **The formal model does not match the world.** The defence is the benchmark and review by people who run real procurement, and it is weak.
- **Most consequential actions are unmediated,** or are seen only by parties who collude.
- **The open targets collapse.** The stop rules fire and the result is negative for this candidate.
- **Trust anchors dominate.** If every guarantee reduces to the honesty of the anchors and a bound on faulty witnesses, the foundation is an accounting system for trust, and the plan would say so.

## Order of work

1. Step 0: design decisions with their cases and an external review of them, then the frozen statements, the source reading and the exit test. **Decision:** is there a consistent set of decisions, and do the definitions pass?
2. Paper stage and its external review. **Decision:** do the stop rules fire?
3. Choice of core. **Decision:** which core, or none?
4. Mechanisation, reference controller and benchmark run. **Decision:** are the structures separable?
5. Scale measurements.
6. External adversarial review of the theorems and the evidence.

No effort estimates are given. An estimate becomes possible after the paper stage.

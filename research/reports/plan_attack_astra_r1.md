# Adversarial review of the integrated foundation plan

Reviewed 17 November 2026. I read the plan before consulting the survey, researcher notes and earlier reviews; I ignored the concurrent `attack_*_r4.md` reviews. My initial concerns were the quantifiers in T4/T7, overlapping composition, and the gap between locality and capacity. Primary-source checks below sharpened those concerns. No subagents were used.

**Overall assessment:** a plausible research programme, but not yet a sufficiently discriminating experiment. No CRITICAL finding: I found neither an impossibility of the intended restricted foundation nor an irreparable core contradiction. Five HIGH findings require changes to the specifications and experiments before expensive mechanisation. “Verified wrong” below applies to the particular stated implication, not to the existence of a repaired theorem. Counterexamples and recommendations labelled “my judgement” are this review's reasoning, not claims made by the cited authors.

## 1. HIGH — T4 lacks the assumptions that make nonempty, sound blame possible; its negative-result interpretation is invalid

**Text attacked:** “if a commitment is violated and the logs are intact, the calculus identifies at least one responsible principal on the delegation chain and never blames a principal who complied”; “If T4 fails even for chains ... it would mean accountability does not compose even in the simplest case.”

**Verified wrong:** Log integrity alone does not imply the proposed verdict exists. Nor does failure of this particular specification establish that accountability cannot compose. The cited accountability framework specifies causal deviations relative to a protocol, a violation property, a counterfactual relation and a verdict function; its target is sets of minimally sufficient deviating parties, not merely one name. See Künnemann, Esiyok and Backes, definitions 1–2 and the examples in §II: https://robertkunnemann.weebly.com/uploads/2/9/4/5/29452463/accver-conf.pdf.

**My judgement / counterexample:** A delegates a procurement task to B; both perform every locally required action. An external provider fails or delivery is delayed beyond the deadline. If the chain excludes that provider and communication infrastructure, no chain principal deviated. If instead B's promise makes B responsible regardless of cause, that is a strict allocation of contractual liability, not identification of culpable deviation. Both interpretations are possible; the theorem currently conflates them. Intact logs can also omit the evidence needed to distinguish two possible culprits. Restricting the topology to a chain fixes neither problem.

**Exact change:** Define separately (a) breach, (b) allocated liability and (c) causal misconduct. Specify the observable evidence, authenticated completeness assumptions, admissible failures, colluding sets and adjudicator. For misconduct, require or prove that every covered violation has an observable qualifying deviation; otherwise allow an explicit “unattributable” result. For liability, specify who retains the obligation after delegation, including insolvency and dissolution. State a theorem for this verdict function and these assumptions. Replace the negative-result claim with: “Failure refutes this accountability contract or encoding under these assumptions; it does not establish non-composability in general.” Test the all-compliant failure case before mechanising T4.

## 2. HIGH — T7 classifies the wrong object and promises an unspecified decision procedure

**Text attacked:** “a static check sorts each declared invariant into ‘needs no coordination’ or ‘needs an owner or committee’.”

**Verified wrong:** Invariant confluence is relative to transactions, reachable states and merge semantics, not an invariant alone. Bailis et al.'s theorem additionally specifies availability and convergence. Their §5.3 explicitly says arbitrary-program I-confluence is undecidable. Their counter examples classify the same numeric threshold differently for increment and decrement operations. Thus neither an invariant-only classification nor an unrestricted complete static classifier follows from this source: https://arxiv.org/html/1402.2237, definition 6, theorem 1, §§5.2–5.3.

**My judgement:** Step 2 restricts authorization policies, but does not restrict the transition programs, invariants or merge operators used by T7. Decidable authorization is not the missing decidability result. A conservative checker can always label everything “committee”; then its second answer is not proof that coordination is necessary. Also, necessity of coordination does not itself establish that a *small* committee suffices for every invariant.

**Exact change:** Make the input `(initial states, operations, invariant, merge, availability/fault contract)`. Either choose a fragment with a complete decision procedure or return three outcomes: certified coordination-free, witnessed coordination requirement under the stated model, unknown. Require certificates/counterexamples and a benchmark coverage threshold. Treat committee construction and sizing as separate obligations. Decide the fragment before the bake-off, not after core selection.

**Prior art that changes the design:** Escrow-derived bounded counters can enforce numeric bounds through distributed rights, with local operations while rights suffice and coordination to move rights. Include them as a baseline instead of equating every protected balance with a single owner on every operation: https://arxiv.org/abs/1503.09052. This is not an argument that rights can be replenished or revoked without coordination.

## 3. HIGH — Shared membership is mistaken for a definition of graph composition

**Text attacked:** “Every composite has a boundary controller ... through which every action crossing the boundary passes”; “Containment is a graph and not a tree, because one principal can belong to several units”; T3: “balances ... consolidate across nested units by addition.”

**My judgement / counterexample:** Let P hold one token and belong to A and B. If P is literally a component of both, separately composing A and B duplicates P's state, or requires a shared-component composition operator that the plan has not defined. Adding their reported balances counts the same token twice. If P merely holds two memberships, that problem disappears, but membership is then a relation distinct from semantic containment and asset ownership. The plan must choose. Declaring precedence between norms does not settle identity of shared state, counting, or which controllers own a crossing action.

**Verified source constraint:** Ordinary I/O-automaton product composition has compatibility conditions, including distinct ownership of output actions; it is not automatically a gluing operation for overlapping component graphs. Its input-enabled recipients cannot veto a sender's output. See Lynch and Tuttle, §3.2: https://groups.csail.mit.edu/tds/papers/Lynch/CWI89.pdf. A controller must therefore mediate requests and own the eventual protected effect; attaching it as another input recipient is insufficient.

**Exact change:** Before T1, define separate graphs for principal membership, component ownership, resource ownership and rule authority. Give one operational rule for a cross-domain transaction, with explicitly owned effects and jointly required approvals. State T3 over disjoint ownership, or provide overlap-aware aggregation with a proof that resources are counted exactly once. Prove that nested and flattened interpretations agree on identities, effects and balances. Include a diamond-shaped example with one shared principal and one shared resource in the bake-off. Fail H1/H2 if this cannot be done compositionally; do not treat unspecified graph gluing as ordinary nesting.

## 4. HIGH — The scale exit test can pass a system that cannot support the workload

**Text attacked:** “The test is locality”; “the cost of one governance decision, and the number of parties that must coordinate for it, do not grow with the total population”; “given bounded delegation depth and bounded unit size.”

**My judgement / counterexample:** A root server handles every purchase in constant CPU time, contacts no other party and reads constant state. It passes the locality criterion while its arrival rate grows with the population and its capacity remains fixed. A constant-size committee has the same problem. This is ordinary offered-load arithmetic, not an appeal to a distributed impossibility theorem. A “distinguished component” may be a logical distributed service, but then the plan needs its sharding and capacity argument.

If bounded unit size means bounded fan-out b, and depth is bounded by d, a single rooted hierarchy contains at most `1 + b + ... + b^d` units. Bounds compatible with 10^10 must be supplied. Replicating disconnected procurement chains proves only scale-out of disconnected chains. Bounding delegation depth does not also bound membership count, policy dependencies, log size, root traffic or revocation fan-out.

**Evidence / relevant comparison:** LGI explicitly couples local law evaluation to decentralised per-actor controllers, rather than using locality alone as its architecture: Minsky, §4.1, https://arxiv.org/html/1403.4321. This is a concrete comparison the plan can reuse, not a proof that LGI satisfies the whole goal.

**Exact change:** Retain locality as one necessary metric. Add per-controller offered load, aggregate update traffic, state growth, capacity and latency bounds under a specified active-principal rate. Count a cap change as one operation including propagation, not as arbitrarily many constant-cost deliveries. Include shared-root and overlapping-membership workloads. Specify b, d and the population represented. Add a kill criterion for an unshardable bottleneck at the target workload. Run this architectural check before mechanising all seven theorems. The plan may openly exclude adversarial load and correlated faults; even its benign workload still needs this test.

## 5. HIGH — The fixed benchmark can certify a static access-control hierarchy while omitting construction and structural self-change

**Text attacked:** “one worked example ... is the fixed benchmark ... for every later step”; “run one worked example that crosses all ten levels”; “producing the evidence that would show whether one is buildable.”

**My judgement:** Labelling nodes resource/program/human/nation does not test the different semantic obligations at those levels. The described execution is essentially a delegation tree with a spending-policy update. It need not exercise overlapping membership, formation by constitution, declared amplification, changing interfaces, creation, merging or dissolution of units, or transfer of unresolved liabilities. A successful static encoding therefore leaves major parts of the promised construction and recursive governance untested. This is a missing essential experiment, not a demand to solve excluded human alignment, incentive compatibility or legitimacy.

**Verified missed prior art:** Dynamic I/O automata already support creation, destruction and changing action signatures, alongside composition, hiding and trace-based refinement: Attie and Lynch, https://arxiv.org/abs/1604.06030. Omitting this nearby candidate makes “without ad hoc machinery” an unfair test of the I/O baseline. A generic transition system can encode such behaviour, but the desired compositional metatheory must be evaluated too.

**Exact change:** Make the procurement case a benchmark family fixed before the bake-off: happy path; diamond membership; constitution and amplification; concurrent revocation; creation and dissolution with outstanding commitments; external-provider failure and disputed evidence; and cap-change propagation under load. Add DIOA and a governance-specific implementation baseline (LGI plus an institutional monitor). Compare concrete encodings, not entire fields such as “polynomial functors.” Fix equivalent observable requirements and count proof/encoding effort separately from trusted assumptions. Hold out at least one variant for evaluation after selection. A switch of core must trigger a reassessment of module contracts, not “switches core and keeps the rest” by default.

## 6. MEDIUM — T2 blurs three different checking problems

**Text attacked:** “no principal acts with authority that is not derivable ...; without revocation this is checkable offline”; “Policies are written in a decidable fragment so that a decision can be evaluated locally.”

**Verified distinctions:** HRU concerns possible future acquisition of a right under protection-state transitions; decidability of a current authorization request is a different problem: https://www.cs.purdue.edu/homes/ninghui/readings/Qual2/Harrison-Ruzzo-Ullman76.pdf. Take-grant has a particular linear-time safety algorithm, not an algorithm for arbitrary constitutions and amplifications: https://flint.cs.yale.edu/cs428/doc/liptonTG.pdf. Object-capability authority-safety proofs depend on the language and capability model: https://theory.stanford.edu/~ataly/Papers/sp10.pdf.

**My judgement:** The references are real and their headline claims are correct; the proposed integration is not disproved. But a checker verifying a supplied derivation, a whole-system authority-safety proof, and offline evaluation of all relevant institutional facts are different deliverables. Even without revocation, a constitution may require a quorum or facts not present in the supplied record. Decidability alone bounds neither those dependencies nor their size. “Derivable” also becomes vacuous if constitution permits unrestricted authority creation.

**Exact change:** Separate the three claims. Specify the proof language, immutable facts supplied offline, root/constitution constraints, attenuation order, and a machine-checked preservation theorem for authority transitions. Give explicit dependency and proof-size bounds. Do not cite take-grant as justification until a reduction or embedding preserves its restrictions.

## 7. MEDIUM — T5 can collapse into transitivity by definition or fail to connect compliant rules to actual behaviour

**Text attacked:** “if each unit's rules comply with its parent's, the leaf's behaviour complies with the root's, under stated conditions”; fallback “norms that only prohibit.”

**Verified source scope:** King et al. provide institutional event/state semantics, constitutive abstraction between levels, and soundness/completeness of an AnsProlog translation. That is not directly the claimed behavioural transitivity theorem: https://orca.cardiff.ac.uk/id/eprint/130977/1/AS4805164377456641491575454833_content_1.pdf, §§3–5 and appendix theorems 1–2.

**My judgement:** If compliance means allowed-trace inclusion on one alphabet, transitivity is already a mathematical property of inclusion. If each level interprets events differently, abstraction maps and their compatibility must be defined. Either way, compliant rulebooks alone do not imply that a leaf obeys its own rules. Account-class enforcement explicitly admits violations. Prohibitions do not solve missing observations or enforcement.

**Exact change:** State separately rule compatibility, leaf enforcement, event abstraction and their preservation under composition. Require a commuting abstraction diagram and distinguish prevention of violations from correct detection/sanctioning of them. Use the King case as a translation benchmark. Specify what new result remains beyond ordinary inclusion transitivity. I do not rate this HIGH because T5 is explicitly open and its limited fallback is candid.

## 8. MEDIUM — Synchronized clocks do not implement safe distributed rule activation

**Text attacked:** “Clocks are assumed synchronised to within a stated skew, which is what makes deadlines and effective dates meaningful”; “Rules are versioned and take effect at a pre-announced time”; T6 preserves T2 and T3.

**My judgement / counterexample:** A controller receives a cap reduction and another does not. Both clocks are accurate, but after the effective date the second still authorizes under the old cap. Clock skew says nothing about delivery or acknowledgements. Near the switch, even successful distribution leaves a skew interval that needs semantics. Separately, preserving authority derivability and total tokens does not imply respecting the newly reduced spending cap: a transition can preserve T2/T3 while violating the policy that motivated it.

**Evidence:** The baseline is an asynchronous discrete-event model, not a timed deployment protocol; its specification of input/output actions does not provide bounded network delivery: https://groups.csail.mit.edu/tds/papers/Lynch/CWI89.pdf. The counterexample above is this review's reasoning under that model, not a claim attributed to its authors.

**Exact change:** Specify a timed execution model, time-progress assumption, deadline uncertainty interval, version admissibility, and activation protocol (for example acknowledged epochs or expiring authorization leases with fail-closed behaviour). Make T6 preserve named application invariants as well as T2/T3, and distinguish safe refusal from successful migration. Add the delayed-update trace to the benchmark. The finite external anchor is coherent; I do not object to stratification itself or merely repeat the acknowledged in-flight-commitment risk.

## 9. MEDIUM — Enforcement classes mix assurance mechanisms and promise an unsupported deterrence effect

**Text attacked:** “Each property has ... one of four enforcement classes”; “Bound ... its validity is monitored”; “Account ... this gives deterrence and not a guarantee.”

**My judgement:** These mechanisms overlap. A certificate checker that blocks admission uses both Prove and Prevent, and T5 itself already has two classes. Monitoring an assumed bound may detect a failure only after the protected invariant is lost. Logging and attribution do not by themselves imply an executable sanction or a deterrent effect. Deterrence needs behavioural and payoff assumptions that this version explicitly excludes.

**Evidence / verified scope:** The cited protocol-accountability definition establishes correctness of blame relative to deviations and violations; it does not prove an incentive response to sanctions: https://robertkunnemann.weebly.com/uploads/2/9/4/5/29452463/accver-conf.pdf, §II. Treat the plan's deterrence assertion as **could not verify**, not as a universal impossibility of deterrence.

**Exact change:** Replace exclusive classes with a per-property assurance record: mechanism(s), assumptions, observability, failure-detection latency, intervention powers and residual outcome. Say “may support deterrence under separately stated incentive and sanction-execution assumptions.” For Bound, distinguish an assumed envelope from a proved robust guarantee while the envelope holds, and specify what happens when monitoring detects a breach. Do not claim that adding a monitor validates the assumption in advance.

## 10. MEDIUM — The implementation assurance boundary is understated

**Text attacked:** “Tie the executable to the mechanised model by differential testing ... This leaves the specification and the test oracle as trusted.”

**Verified correction:** The Cedar method is accurately named, but testing model/implementation agreement does not prove implementation refinement. Cedar explicitly distinguishes testing that agreement from formally proving it, and uses property-based tests for unmodelled production components: https://arxiv.org/html/2407.01688v1, §§1 and 4.

**My judgement:** A buggy runtime controller remains able to violate the mechanised invariants on untested executions. The trust/evidence accounting must also include production parsing, serialization, persistence/recovery, clock integration and atomic check/use behaviour where these affect enforcement. Otherwise the “smaller trusted base” comparison understates the deployed boundary. Testing is an entirely reasonable research-stage choice; the error is describing what it establishes too narrowly.

**Exact change:** Say explicitly that model theorems are proved and executable conformance is tested, not proved. Inventory the runtime trust base separately from the proof kernel and axioms. Test transition sequences, concurrent revocation, crash recovery and version changes, not just individual authorization requests. Do not require full verified deployment to pass this research phase.

## Claims checked without finding a defect, and limits of this review

The default I/O-automaton choice is technically credible. Input-enabledness and substitutive trace refinement under compatibility conditions are appropriate claims; an explicit request/reject protocol is a legitimate encoding. The problem is ownership of the actual effect, not inability to express refusal at all. T1 is appropriately described as reuse, so its being established is not a novelty defect.

The classical take-grant and general access-matrix claims are supported by the primary sources above. Object-capability isolation has formal treatments, but their hypotheses must be transported. King et al. and Cedar are genuine and relevant sources. No source checked establishes that an integrated foundation is impossible.

**Could not verify:** “smallest established baseline” has no defined measure or comparative proof. The global absence of previous integrated foundations is also not established by a finite source search. These should remain hypotheses, not strengthened novelty claims. I did not perform an exhaustive review of every categorical, bigraph or Maude variant; the bake-off must identify the actual theories it compares.

I have not charged separate findings for the acknowledged exclusions of general liveness, probabilistic guarantees, legitimacy, incentive compatibility, adversarial load, anchor failures or unmediated human activity. These limits are legitimate for a first kernel. However, completion would establish a restricted foundation for mediated governance under its assumptions, not decide the unrestricted existence question for humans and institutions. A failed encoding, fallback or benchmark is evidence against that candidate, not an impossibility result about the goal.

## Verdict and revised order

Proceed only after revising the experimental contract. First define the distinct graphs, ownership, observations, faults, verdict semantics and nontrivial useful behaviour that must remain possible; otherwise reject-all controllers and tautological liability rules can satisfy safety-shaped targets. Next fix T7's fragment and the benchmark variants, and run the scale bottleneck calculation. Then compare concrete candidate encodings, including dynamic I/O automata and governance-specific baselines. Mechanise the smallest counterexample-bearing slices before committing to the full six-module implementation. Make intermediate proof reviews occur before the final external review.

The plan deserves repair, not abandonment. Its core choice and explicit non-goals are defensible; its biggest problems are insufficient theorem hypotheses and experiments that can pass without resolving the intended integration question. A successful revised programme could deliver meaningful positive evidence for a restricted integrated foundation. The present plan would leave too much of that conclusion dependent on labels, assumptions and untested structure.

CRITICAL: 0  HIGH: 5  MEDIUM: 5  LOW: 0

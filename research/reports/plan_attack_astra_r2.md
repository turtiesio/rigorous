# Adversarial review of the integrated foundation plan, version 2

Reviewed 17 November 2026. I read the current plan and formed an initial view before reading the first-round attacks and the old plan. I then checked primary sources, the survey and relevant researcher notes. No subagents were used. This review targets the rewrite, including corrections that reproduce bad advice from the earlier reviews.

**Assessment:** materially improved, and worth a repaired paper-stage experiment. I found no CRITICAL defect in the restricted research programme. Two HIGH findings concern assumptions that cannot support the promised guarantees as written; five MEDIUM findings concern composition rules, experimental discrimination and fidelity to prior art. An open theorem is not a finding merely because it could fail. The explicit exclusions of legitimacy, alignment, confidentiality, unmediated behaviour and actual planetary deployment are not counted against the plan.

“Verified wrong” identifies a source-checkable correction or an explicit invalid implication. “My judgement” identifies my counterexamples, experimental assessments and proposed repairs. “Could not verify” is used for unresolved literature or interpretation questions, not as evidence of falsity.

## 1. HIGH — The new witness assumption does not support the accountability threat model

**Text attacked:** “A controller may deviate, provided at least one honest witness per unit holds signed commitments to that controller's log”; R2(a), “every violation caused by a covered deviation yields a verdict naming the deviating parties”; “Controller correctness: no action takes effect without a recorded derivation.”

**Verified wrong:** Holding signed log commitments is not sufficient for complete, sound attribution of controller deviations. PeerReview requires substantially more: message commitment and consistency protocols, audit/replay against a reference implementation, challenge/response, and evidence transfer. Its coverage is observable faulty behaviour, not arbitrary concealed effects. It distinguishes exposure from suspicion; with asynchronous communication a correct node can be temporarily suspected. See [PeerReview, technical report, §§3–4](https://haeberlen.cis.upenn.edu/papers/peerreview-tr.pdf). Protocol accountability likewise requires that violations leave evidence usable by the verdict function; that does not follow from the presence of signatures. See [Künnemann et al., §II](https://arxiv.org/pdf/1805.10891).

**My judgement / counterexample:** A controller and a recipient collude. The controller emits a protected effect, omits it from the log, and sends its honest witness commitments to an otherwise valid log. Compare a run where the effect never occurred. The witness has the same observations in both. No verdict computed from those observations can both expose the first execution and avoid accusing the compliant controller in the second. Neither hash integrity nor the controller's exclusive ownership of the output repairs this. Exclusive ownership concentrates the ability to cause the hidden effect in the faulty party.

This is not the excluded case of a human acting outside the interface: the omitted action is a controller-owned effect inside the proposed model. Nor can controller correctness be applied to a controller expressly permitted to deviate arbitrarily. “Covered deviation” can exclude this case, but then the threat model and omission benchmark must explicitly disclose the exclusion. Otherwise the main experiment may fail for an elementary evidence deficit rather than a failure of delegation composition.

**Exact change:** Before R2's paper proof, define a fault-indexed theorem contract. Prevention lemmas require correct controllers or a separate trusted effect gate. Accountability permits faulty controllers only for effects with an independent evidence path: for example, an effect recipient must demand a witnessed receipt, with the receipt/effect relationship itself enforced. Specify active auditing, evidence availability, which observer is honest, and what happens if evidence is withheld. Keep “suspected”, “proved deviation”, and “allocated contractual loss” distinct. Make variant 8 contain both an externally evidenced omission and an omission shared only with colluding recipients; require the latter to be explicitly outside coverage or prevented by the effect gate.

**Rewrite fidelity:** This partly comes from following Fable R1's recommendation to replace intact logs with one honest commitment-holder too literally. That recommendation was insufficient; the rewrite should not inherit it unexamined.

## 2. HIGH — Unbounded delivery and finite maximum propagation latency are incompatible

**Text attacked:** “Network delivery is not bounded”; step 5, “A cap change is one operation including its propagation ... with a stated maximum latency”; R7, “a departure from the envelope is detected and attributed within a stated time”; “controllers that have not received the current version refuse”.

**Verified wrong:** Under the stated delivery model, a remote update or piece of evidence cannot be guaranteed to arrive within any finite bound. Synchronized clocks do not imply timely communication. The asynchronous distinction between an unresponsive faulty node and a delayed correct one is explicitly addressed by [PeerReview, SOSP paper, §4.8](https://iris.pdos.csail.mit.edu/irisbib/papers/peerreview%3Asosp07/paper.pdf). The paper offers suspicion rather than infallible timed exposure in that case.

**My judgement / counterexample:** For any proposed maximum L, delay the update to one affected controller for longer than L. A pre-existing lease can expire and make it stop authorizing. That establishes a bound on stale authorization, not on propagation or installation of the new policy. An acknowledged epoch can postpone activation, but its acknowledgement phase has no bounded duration. Measuring latency only after the last acknowledgement hides the delay that the plan says the cap-change operation includes.

For R7, this is conditional on the observation boundary: a local monitor can detect a locally observable price excess promptly. It cannot guarantee prompt remote attribution of an unobserved vendor action when all reporting messages can be delayed. Missing a delivery deadline can be classified locally as a contractual breach; it does not prove whether the vendor or communication failed.

**Exact change:** Separate four quantities: request-to-activation time, last permissible old authorization, update installation time, and service recovery time. Under unbounded delivery, permit unbounded installation/recovery and prove a finite stale-authorization bound from leases and clock skew. Give finite completion/performance bounds only under a separate, explicit bounded-delay workload assumption. For R7 name the observation point and bound local detection separately from evidence delivery/adjudication. Define useful-behaviour deadlines under that same performance envelope. Keep indefinite refusal visible as unavailability, rather than treating it as successful bounded propagation.

This is HIGH because the current scale gate necessarily rejects the asynchronous design if “maximum latency” has its ordinary worst-case meaning. It is repairable without abandoning safe rule change or the wider programme.

## 3. MEDIUM — “Weakest record” is not a sound general assurance-composition operation

**Text attacked:** “Each [property] has ... an assurance record ... A composite reports the weakest record among the parts a property depends on.”

**Verified wrong, as a general implication:** Selecting one constituent record need not produce a conservative record for a composite. The records contain different assumptions, observation scopes, detection delays and intervention powers; these do not define a total order.

**My judgement / counterexample:** A leaf guarantees detection within one second of an effect. A parent guarantees its verdict within one second after receiving the leaf's report. Even with immediate delivery, the end-to-end bound can be two seconds. Neither original one-second record is an adequate composite record. If one part assumes an honest receipt service and another assumes an honest adjudicator, the composite needs both assumptions; retaining either record loses one. Alternative redundant evidence paths can conversely make the composite stronger than its weakest component.

**Evidence:** Established contract theory treats assumptions, guarantees and composition explicitly; it is not a minimum-of-labels operation. [De Alfaro and Henzinger, Interface Automata](https://luca.dealfaro.com/papers/01/FSE01.pdf) distinguishes environment assumptions and component guarantees and uses compatibility and alternating refinement. The arithmetic counterexample above is mine, not a theorem attributed to that paper.

**Exact change:** Replace the sentence with: “For each property a composition rule derives a new assurance record, discharging interface assumptions, retaining unresolved assumptions, and composing observation, latency and intervention guarantees; absent such a rule the composite is unestablished.” A weakest-label display can remain a UI summary after this derivation. Make this derivation part of the three-unit paper experiment, including serial and alternative evidence paths.

## 4. MEDIUM — R3 understates its baseline and proposes a counterexample that needs a different premise

**Text attacked:** R3's baseline “two-level compliance checking”; its candidate “Two mappings that each preserve compliance and whose composite does not, because an abstraction merges events the root distinguishes”.

**Verified wrong:** King et al. are not restricted to two levels. Definition 3 gives a directed acyclic graph of institutions, and the case study has three governance levels. Definitions 11–12 also address composite and synchronized event traces. See [King et al., §§3–4 and conclusion](https://orca.cardiff.ac.uk/id/eprint/130977/1/AS4805164377456641491575454833_content_1.pdf). Their result does not thereby establish R3's particular behavioural composition theorem, but “more than two levels” cannot be its novelty.

**My judgement / mathematical check:** Let L, M and H be compliant trace sets. If f(L) is a subset of M and g(M) is a subset of H, then (g composed with f)(L) is a subset of H. Event merging alone cannot refute that implication. A real failure requires weaker premises: pairwise checks performed in inconsistent contexts, different interpretations of the middle vocabulary, non-total mappings, or design compliance that does not imply behavioural preservation. The plan leaves open exactly these semantic choices, so this is a defective proposed test, not evidence that the research target is impossible.

The LGI comparison is also narrower than the cited baseline: LGI's conformance hierarchy allows superior laws to specify subordinate freedom, not merely impose increasingly restrictive prohibitions. See [Minsky, §4.1](https://arxiv.org/html/1403.4321).

**Exact change:** Describe King as multi-level institutional compliance with constitutive abstraction and LGI as enforced conformance under superior-law constraints. Reproduce King's three-level case. Write the precise pairwise predicate before attempting the counterexample. State the extra result sought as a behavioural enforcement/composition theorem, and test an explicit mismatch between pairwise institutional compliance and actual leaf behaviour. Do not count transitivity of sound trace mappings as new research.

## 5. MEDIUM — The central stop rule still has an avoidable false-positive and false-negative interpretation

**Text attacked:** “a chain verdict can be computed from per-unit verdicts without reading inner logs”; “If R2 has no true, non-definitional statement on a three-unit chain ... stop”; “If every statement that survives follows from one module plus controller correctness ... stop”; R5's candidate “most benchmark invariants come back unknown”.

**My judgement:** The R2 condition is discriminating only after fixing what a per-unit verdict may contain. If it contains the complete relevant causal history, the chain can reconstruct the logs under a different name. If it is just a set of locally guilty parties, it can discard the information needed for joint causation. This is precisely the planned research problem, but the interface must be fixed before the paper-stage stop decision, rather than adjusted until one statement survives.

At the other extreme, integration may succeed by translating the combined problem into an existing contract theory. That can be a useful compositional foundation even if the last proof invokes one existing module. A module-count novelty rule could discard that success. Counting imported definitions has a similar dependence on how files and interfaces are packaged.

**Evidence:** [Hvitved, Klaedtke and Zălinescu](https://www.research-collection.ethz.ch/bitstreams/180b3e56-44c5-42ac-a789-b8621d12c19a/download) already give compositional blame for contract conjunction/disjunction; the primary repository's indexed abstract was available, but full-text retrieval returned 403. [Künnemann et al., §II](https://arxiv.org/pdf/1805.10891) makes causal accountability depend on the chosen counterfactual relation. Neither source establishes that unit-local verdicts suffice for this plan's delegation composition.

R5 is considerably improved, but “most” is not a pass threshold and “unknown” makes the classifier formally satisfiable by a constant function. The cited [Whittaker–Hellerstein paper](https://mwhittaker.github.io/publications/iconfluence_vldbj2020.pdf) already offers interactive checks, sufficient conditions and segmented invariant confluence. It is more than an undecidability citation and should be the operational baseline. R1 likewise needs a fixed fragment before success is judged; “constitution” is a policy feature, not an undecidability proof.

**Exact change:** Freeze the R2 summary schema, observation projection and counterfactual relation before the proof attempt. Require a pair of histories that the summaries distinguish for the intended causal reason, and test whether same-summary histories can demand different global verdicts. Record summary size as part of scale. Replace the module-count stop rule with failure to satisfy the frozen cross-unit specification or failure to improve on a stated baseline; report reuse and novelty separately. For R5 predeclare benchmark coverage and time limits, with timeout yielding unknown, and compare against Lucy/segmented confluence plus escrow. For R1 freeze constitution/amplification syntax and permitted policy transitions first.

## 6. MEDIUM — The core's side conditions and bake-off obligations need an operational checkpoint

**Text attacked:** “Substitutivity therefore carries two explicit hypotheses”; “Each candidate encodes benchmark variants 1 to 3, proves closure and substitutivity, and carries the paper proof of R2 on a chain”.

**Verified correction:** There are not merely two automatically transferable conditions to attach to an otherwise available theorem. Attie–Lynch's creation theorem has six numbered premises, including start-state and internal-action restrictions, ordinary and terminating trace inclusion, initial configuration correspondence, and creation correspondence. See [Theorem 35](https://arxiv.org/pdf/1604.06030). For time progress, the relevant timed theory distinguishes receptive/progressive automata from the weaker feasibility condition; see [Kaynar et al., chapters 6–7](https://groups.csail.mit.edu/tds/papers/Kirli/mainfinal.pdf). These are obligations to instantiate, not a proof for the combined model.

**My judgement:** The plan correctly treats the combined theorem as open and supplies a core-switch criterion. I do not rate that HIGH. The practical gap is that variants 1–3 need not exercise timed dissolution, while dissolution is precisely where terminating traces and internal destruction matter. Also, creation by a hidden constitutional transition is not automatically covered by the creation theorem. This can be handled by observable creation or a stronger theorem, but that choice must be tested.

The bake-off can be fair as a bounded human research comparison; it is not a decidable search for a smallest calculus. Requiring finished closure/substitutivity proofs for every candidate without a budget risks never selecting a core. Interface automata also use assumptions and an alternating refinement, so the same word “substitutivity” does not ensure identical guarantees across entries.

**Exact change:** Replace “two explicit hypotheses” with two groups of proof obligations and cite the actual theorems. Before full bake-off, test a small creation–deadline–dissolution trace, including inherited outstanding commitments and hiding. Freeze common observable requirements and permitted environment assumptions across candidates. Time-box each encoding/proof attempt, allow “not established within budget”, and shortlist before full proofs. Count domain definitions, trusted assumptions, reused library machinery and proof effort separately; do not claim global minimality.

**Could not verify:** I did not find an existing theorem giving exactly the plan's timed-plus-dynamic substitutivity result. This search does not establish its absence. The plan's cautious “knows of no published” wording is appropriate.

## 7. MEDIUM — Tree containment does not make rule authority single-parent

**Text attacked:** “Four relations are kept separate ... rule authority”; “An amendment is valid when it passes the unit's own charter procedure and conforms to the constraints of the unit that contains it; with role instances in a tree, that unit is unique.”

**My judgement:** Giving each role instance one containing unit is a coherent repair for duplicate component/state ownership. It does not imply that each unit has one normative authority. Even a proper unit-containment tree does not imply that its rule-authority relation is that tree. The amendment check silently substitutes containment for the distinct authority relation introduced in step 0.

A subsidiary may be contained by its parent group while also constrained by an external regulator's rule. Its own charter and parent may permit an amendment which violates the regulator's rule. Either the regulator's constraint has been compiled into the parent law, with a preservation/update obligation, or validity must check another authority edge. Variant 10's overlapping norms makes this distinction relevant inside the benchmark, not merely to excluded real-world legitimacy.

**Evidence:** [King et al., Definition 3](https://orca.cardiff.ac.uk/id/eprint/130977/1/AS4805164377456641491575454833_content_1.pdf) models governance links separately as a DAG. That is a concrete baseline for multiple normative superiors, not proof that this candidate must adopt a DAG.

**Exact change:** State whether rule authority is restricted to containment. If so, specify how every non-containment constraint in variants 2 and 10 is represented and maintained. Otherwise an amendment must satisfy all applicable authority edges, with a declared conflict result. Add an amendment that passes the container's check but conflicts with an external authority to variant 10.

The version-indexed self-amendment proposal itself is coherent: checking version n+1 under version n avoids circular validation. It establishes a history of authorized change, not preservation of every old invariant. State which fixed anchor constraints persist and which policies may change; serialize competing amendments or specify a fork-resolution protocol. These are ordinary proof obligations, not a Gödel objection or a reason to forbid self-amendment.

## Remaining target and experiment audit

These are checks and scope assessments, not additional severity findings.

- **R1:** The RT baseline really studies possible future policy consequences under restricted changes, with decidability and complexity results; [Li, Winsborough and Mitchell, abstract and §1](https://www.cs.purdue.edu/homes/ninghui/papers/analysis_oakland03.pdf). The plan no longer confuses this with evaluating a current request. The proposed HRU counterexample is not established until an encoding of the relevant command language is supplied; unlimited creation of an authority by itself can make reachability trivially true rather than undecidable. Keep it as a reduction to attempt, not an established result about “constitution”.
- **R2:** The all-compliant vendor failure is now a legitimate test of separation between culpability and allocated loss, not a counterexample to part (a), whose premise excludes it. Joint causation remains a real candidate obstruction to local-summary composition. No source checked proves the proposed delegation theorem already.
- **R3:** Its defect is in the candidate test and novelty comparison, as finding 4 explains; the actual behavioural theorem remains worth attempting.
- **R4:** Safe refusal via leases or acknowledged epochs is credible. The late-update trace attacks a naive activation protocol, not a correctly specified lease protocol. Also define the cap carefully: already spent 80 cannot satisfy a newly imposed cumulative cap of 50 by refusing future purchases. Activation must reject an infeasible migration, preserve grandfathered spending explicitly, or record that the new invariant does not hold. Recording a violation does not itself establish that invariant.
- **R5:** The restricted-language/unknown design is sound in principle. Necessity of coordination remains relative to operations, reachable states, merge and availability assumptions; [Bailis et al., theorem 1 and §5.3](https://arxiv.org/html/1402.2237). Escrow changes the operation/state representation and can permit local spending of preallocated rights; [Balegas et al.](https://arxiv.org/abs/1503.09052). This is compatible with a sharded logical controller. It does not prove that arbitrary cap decreases, rights reclamation or Byzantine shards work without coordination.
- **R6:** Distinct role instances, unique resource ownership and explicit funding claims can make the diamond coherent. Two funders do not require two copies of the asset. The actual test must retain both funding claims and shared identity in the flattened reading; merely deleting one incidence to make the arithmetic work would not pass. No intrinsic impossibility found.
- **R7:** Violation detected after an invariant fails is not a counterexample to the stated conditional guarantee while the envelope holds. If the envelope was exceeded, its premise no longer holds. Test both preservation within the envelope and timely evidence outside it; do not advertise this as robustness beyond the envelope. Detection is subject to finding 2.

Step 0's inter-reviewer agreement threshold can genuinely fail, but measures interpretability rather than technical adequacy; agreement that almost every system fails is possible. The benchmark is substantially stronger than version 1, and the early paper stage is the right order. Its held-out case must be selected before it is used to shape a candidate; it cannot be one of the explicitly required paper/bake-off examples and also remain held out. None of this warrants another HIGH finding.

## Prior art and decision sequence that would change execution

The actionable additions are deeper use of existing sources, not a claim that an undiscovered complete foundation settles the question: PeerReview's complete protocol and observability contract; King's actual multi-level case; Whittaker–Hellerstein's Lucy and segmented confluence as a classifier baseline; and the full DIOA creation theorem. Hvitved's conjunction/disjunction result must be reproduced before claiming novelty for delegation blame. Its full text was not accessible in this check, so I do not claim to have verified its detailed side conditions. The ordinary I/O, timed I/O, interface-automaton and dynamic-creation results should be reused with their distinct refinement/compatibility assumptions, not merged by naming them together.

Recommended decision sequence:

1. Fix coverage and evidence for faulty controllers, the temporal contracts, and the R2 summary schema. Run the indistinguishable-execution and delayed-network counterexamples immediately.
2. Attempt the three-unit R2 proof and the corrected R3 test; reproduce the relevant existing baselines. Treat failure as failure of these fixed contracts, not a general impossibility theorem.
3. Check the overlap/amendment case and timed creation–dissolution example. Produce the assurance-record derivation and the early sharding/load argument, counting witness/audit traffic as well as authorization traffic.
4. Proceed to a bounded bake-off only if a useful cross-unit contract survives. Mechanise the chosen candidate after that decision. Use workload-conditioned performance gates and separate safety from availability.

## Verdict

The narrowed claim is stated honestly and still worth testing. A compositional account of authority, resource use, obligations and evidenced loss across creation and nesting would be useful even though it is not a foundation for governing human behaviour in general. The revised scale/load criteria, role-instance separation, explicit non-goals and early negative-result gate are substantial improvements; they should not be undone.

Proceed with the corrected paper stage, not yet with the expensive bake-off or mechanisation. The two urgent repairs are the evidence contract for faulty controllers and the incompatible latency assumptions. The other findings make the experiment more discriminating without pretending that a research plan must already contain its proofs. I found no justification for a CRITICAL rating and no evidence that all seven targets are already solved or impossible.

CRITICAL: 0  HIGH: 2  MEDIUM: 5  LOW: 0

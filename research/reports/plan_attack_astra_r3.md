# Adversarial review of the integrated foundation plan, version 3

Reviewed 17 November 2026. I formed my initial view from the current plan before consulting the earlier reviews, then checked the proposed repairs against primary sources. No subagents were used. This review distinguishes **verified wrong** (an invalid stated implication or a source-checkable correction), **my judgement** (my counterexample or experimental assessment), and **could not verify** (a limit of this review).

**Assessment: sound enough to start step 0.** I found no CRITICAL or HIGH defect under the requested severity rules. The following seven MEDIUM defects should be resolved in the definition freeze and paper checks. Some sentences are false read literally, but none requires abandoning the proposed research targets: the plan already allows conservative failures, restricted fragments and explicit negative results. I do not promote missing step-0 definitions into architectural impossibility findings.

## 1. MEDIUM — The assurance-record recipe still permits unsound composition

**Text attacked:** “Interface assumptions that a neighbour's guarantee discharges are removed”; “Observables are intersected”; “Detection times add along a dependency path”; “The mechanism is the least capable present”; “The derivation is conservative”.

**Verified wrong:** Unrestricted mutual discharge of conditional guarantees is not sound. Abadi and Lamport explicitly exhibit the circular inference and require additional premises to justify composition: [Conjoining Specifications, §4.3 and §5](https://lamport.azurewebsites.net/pubs/abadi-conjoining.pdf). Calling a guarantee a neighbour's guarantee does not establish those premises. The acyclic rule-authority relation does not make communication or contract dependencies acyclic.

**My judgement / counterexamples:** A promises to issue receipt a by time 1 if B issues b by time 1; B makes the symmetric promise. Both remain silent. Each conditional contract is satisfied. Removing both assumptions by matching the neighbours' conclusions would manufacture an unconditional delivery guarantee. Timed deadlines do not remove this circularity.

There is a second, independent problem with the numerical fields. A detects an event within one second and B processes A's report within one second of receiving it. Their cross-unit link has unbounded delay. The end-to-end bound is not two seconds. Channels must be explicit dependencies. Intersecting their observable sets can also erase the very evidence on which the retained detection bound rests. The record should then become unestablished, rather than carry a finite number alongside no usable evidence. Finally, “envelope assumed” and “logged and adjudicated” are not generally ordered by strength: one is a conditional guarantee and the other an evidence/intervention mechanism. The intervention field has no derivation rule at all.

**Exact change:** Treat these operations as candidate rules, not an already conservative algebra. Require a sound assume-guarantee inference rule for assumption discharge; include channel and processing contracts in each dependency path; index evidence and intervention by observer and property; return unestablished if the derived observation projection cannot support the conclusion. Keep the mechanism label as a display, not the proof order. Add the circular deadline example and an unbounded reporting link to the already scheduled serial/alternative-path checks.

**Rewrite fidelity:** Fable r2 M2 supplied much of this field arithmetic. Copying it did not establish soundness; Astra r2's more cautious requirement for a property-specific derivation remains necessary.

## 2. MEDIUM — Conflict resolution must be distinguished from conformance to all authorities

**Text attacked:** An amendment “conforms to every declared rule authority”; “Where two authorities conflict, a declared priority order decides”; R5 preserves “conformance to every declared authority”.

**My judgement / counterexample:** Suppose procurement law forbids disclosure of a record and group policy requires that same disclosure by tomorrow. Giving group policy priority does not make disclosure conform to procurement law. Giving procurement law priority does not satisfy the group's obligation. Recording the conflict or referring it to an adjudicator produces a procedural outcome, not a compliant behaviour. A total result type can always return pending or rejected; that is different from a total conflict-solving procedure producing valid amendments.

This need not invalidate R3 or R5. Their premises can exclude inconsistent obligations, or higher rules can explicitly authorize exceptions. It does mean that the current priority sentence cannot silently be the justification for preservation. In particular, a unit must not evade an entrenched external constraint by amending its local priority order or deleting the corresponding authority edge.

**Evidence:** [Minsky, §4.1](https://arxiv.org/pdf/1403.4321) describes subordinate freedom as freedom granted by the superior law itself. It does not identify arbitrary override with conformance. [King et al., Definition 3 and compliance semantics](https://orca.cardiff.ac.uk/id/eprint/130977/1/AS4805164377456641491575454833_content_1.pdf) supplies the DAG baseline, not a theorem that prioritization satisfies conflicting obligations. The contradiction above is my propositional counterexample.

**Exact change:** Freeze separate notions of raw authority constraints, authorized exceptions, and effective obligations. Preserve raw conformance by rejecting incompatible amendments unless the displaced authority itself permits the exception; alternatively state explicitly that the guarantee concerns resolved obligations. Define the conflict outcome type, with pending/rejected distinct from valid. Entrench authorization for changes to authority edges, priorities and adjudicator powers. Add an obligation/prohibition conflict to variant 10, since two conflicting permissions or prohibitions may be satisfied merely by refusing action.

**Rewrite fidelity:** The DAG repair is faithful and useful. Fable r2 H3's suggested “priority order” needs this qualification; it is not automatically consistent with its simultaneous “every authority” requirement.

## 3. MEDIUM — The receipt repair needs evidence-transfer and timing obligations, not just countersignatures

**Text attacked:** “a receipt countersigned by a witness”; “at least one honest witness”; the witness “holds signed commitments” and periodically replays; coverage requires an honest party “able to contact a witness”.

**My judgement:** These are compatible ingredients, but do not yet specify the evidence path. If there are one honest witness H and one colluding witness C, the receipt rule permits C to countersign. An honest recipient seeing that receipt does not by itself ensure H obtains it. “Able to contact” must become a protocol duty to submit evidence to an eligible honest audit path. Conversely, requiring every receipt to be signed by every witness lets one faulty witness block all useful behaviour. Neither choice follows from the current wording.

An honest witness holding only a hash cannot replay unavailable log contents. Also, bounded delivery plus a periodic audit does not bound replay completion: a growing backlog can take arbitrarily long to process. Suspecting a withholding controller is useful, but does not automatically reconstruct the causal verdict demanded by R2. This is not a renewed demand to cover colluder-only effects; that exclusion is now explicit and correct.

**Evidence / verified scope:** [PeerReview, §§4.3–4.8, §6.1 and Appendix A](https://haeberlen.cis.upenn.edu/papers/peerreview-tr.pdf) provides commitment, audit, consistency and challenge protocols, and distinguishes exposure from suspicion. Its theorem can identify a faulty accomplice rather than the original faulty node. Deterministic replay uses recorded inputs and relevant nondeterminism; it does not prove the truth of external inputs.

**Exact change:** Freeze the receipt acceptance predicate, evidence submission duty, witness selection, log availability/challenge protocol, and the processing bound or explicit absence of one. Add a colluding countersigner plus honest auditor case to variant 8. State precisely which verdict kinds satisfy each R2 clause; a suspicion must not silently count as proved causal attribution. Include witness backlog and evidence transfer in the existing load calculation.

Deterministic controllers do **not** conflict with learned agents or humans: the latter are expressly external inputs. Record their requests, event ordering and clock observations sufficiently for replay, without pretending to replay their reasoning. This is a specification completion, not a reason to discard the design.

## 4. MEDIUM — Scale still uses the containment tree after authority moved to a DAG

**Text attacked:** “Per-decision cost. Proportional to depth”; stale authorization ends according to leases “down the depth of the tree”; “Message cost is proportional to the affected subtree”.

**Verified wrong as a general bound / my counterexample:** Containment depth does not bound authority dependencies. Take N units all at containment depth one and connect their rule-authority edges into a chain. Alternatively give one leaf k unrelated authorities. Both satisfy the declared acyclic relation. A ministry update can affect enterprises outside its containment subtree. A dense affected authority DAG has quadratically many edges in its number of vertices. Whether an implementation traverses those edges, deduplicates delivery or maintains compiled policy summaries is an algorithmic question, not a consequence of tree depth.

**Evidence:** The graph allowed by [King et al., Definition 3](https://orca.cardiff.ac.uk/id/eprint/130977/1/AS4805164377456641491575454833_content_1.pdf) has an acyclicity restriction, not a constant-degree or containment-depth bound. [F1's schema-change protocol](https://research.google.com/pubs/archive/41376.pdf) is a lease precedent, not a proof about recursive authority propagation.

**Exact change:** Parameterize bounds by containment depth, authority-path length, applicable-authority count, policy size, and affected dependency vertices/edges. Alternatively freeze explicit bounds on those quantities for the claimed workload. Replace “affected subtree” with the affected authority/resource dependency closure and state the dissemination algorithm. Specify that any sublease supporting an authority dependency cannot be renewed indefinitely from expired ancestor authority. Add a long authority chain and a wide authority join with shallow containment to the paper scale experiment.

This is MEDIUM because adding parameters and benchmark cases repairs it within the existing scale methodology; no new scalability theorem is being demanded.

## 5. MEDIUM — Expiration of rights does not undo completed spending

**Text attacked:** “A cap lowered below the sum of rights already allocated holds only after those rights expire, and that interval is recorded.”

**My judgement / arithmetic counterexample:** Allocate 100, spend 80, then lower the cumulative cap to 50. After every remaining spending right expires, cumulative spending is still 80. No finite expiry interval establishes spending ≤ 50. Even when the new cap exceeds past spending, expiry only prevents use of old rights; renewed rights must be issued against the remaining capacity under the new cap.

**Evidence:** [Balegas et al., bounded-counter construction](https://arxiv.org/abs/1503.09052) separates numerical state changes from distributed rights to perform changes. The paper does not prove that expiry reverses prior consumption. The numerical contradiction is mine. [F1, §4.5](https://research.google.com/pubs/archive/41376.pdf) likewise uses fencing to terminate operations under expired schemas, not to reverse completed effects.

**Exact change:** Say: “After old rights expire, no further spending under those rights is authorized. The new cumulative cap is established only if completed spending and surviving commitments fit under it, and new rights respect the residual capacity. Otherwise apply R4's rejection, explicit grandfathering, or recorded failure.” Distinguish renewable rate limits from cumulative spending caps. Run this exact numeric case on the first day.

R4 already recognizes the infeasible-cap case, so this is an internal inconsistency in the new scale sentence, not a failure of R4's research programme.

## 6. MEDIUM — Dropping a hypothesis does not distinguish research from definitions

**Text attacked:** “The criterion is that a non-definitional statement is refuted by at least one benchmark variant when one named hypothesis is dropped.”

**Verified wrong / logical counterexample:** Let “valid amendment” be defined to mean “an amendment preserving I”. The proposition “every valid amendment preserves I” is definitional. Drop validity and variant 11 can remove logging, so the proposition passes the proposed criterion anyway. An essential-hypothesis test establishes neither novelty nor distance from the definition of validity.

**My judgement:** This matters especially for R5 and R4. R5 can be a useful implementation preservation theorem, but version indexing plus defining validity to include preservation cannot establish novelty. For R4, a frozen rule allowing every infeasible change simply to be rejected or labelled unestablished can make the stated refuter unreachable. The rule must identify some useful changes that it actually accepts and preserves commitments through. R6 is a potentially useful cross-unit integration demonstration, but joining complete authenticated receipts by identity is not by itself evidence of a new foundational result.

**Evidence:** [Minsky's existing conformance construction](https://arxiv.org/pdf/1403.4321) already enforces restrictions by construction. [Li, Mitchell and Winsborough](https://www.cs.purdue.edu/homes/ninghui/papers/analysis_oakland03.pdf) distinguishes reachability questions from checking compliance at a state. These are baselines to compare actual added content against; the logical counterexample above is independent of them.

**Exact change:** Call the hypothesis-dropping test a necessary stress test, not the criterion for non-definitional research. Have reviewers separately identify the implementation-to-specification obligation and the incremental result over a cited baseline. Freeze at least one nontrivial accepted amendment class for R4 and a useful accepted self-amendment for R5. Treat direct preservation-by-validity and receipt joins as reuse unless an additional theorem is identified. Retain the existing R1–R3 baseline stop rule.

**Rewrite fidelity:** The flawed criterion was recommended in Fable r2 M1. The editor adopted it faithfully; that does not make it discriminating.

## 7. MEDIUM — The lexical bake-off needs a normalized hypothesis measure and an inconclusive outcome

**Text attacked:** “How many hypotheses does it need? How large is it on paper?”; “not established within the budget” is allowed; step 3 mechanises “the chosen core and the runner-up” for the size comparison.

**My judgement:** The comparison is finite and time-boxed, so it need not solve undecidable theorem discovery. That earlier objection is repaired. But raw hypothesis counts remain representation-dependent: A ∧ B ∧ C can be written as one hypothesis or three. Equal environment assumptions do not resolve differences in theorem compatibility conditions or encoded semantic obligations. Nor does the decision rule say what to do when no candidate establishes R2, or when only one is eligible and there is no meaningful runner-up. A budget timeout must not become evidence that its candidate is inferior or impossible.

**Evidence:** [Interface Automata](https://luca.dealfaro.com/papers/01/FSE01.pdf) compares assumptions and guarantees through refinement; [Attie–Lynch, Theorem 35](https://arxiv.org/html/1604.06030v1) enumerates substantive conditions for creation substitutivity. Counting numbered premises across those presentations is not a semantic comparison. The gaming example is my observation.

**Exact change:** Freeze a common inventory of semantic obligations and external trust assumptions, with conjunction packaging normalized. Compare assumption strength where possible and report incomparable conditions rather than pretending the count resolves them. Add explicit outcomes for no eligible core, one eligible core, and ties; mechanise a runner-up only if there is one supported well enough for the comparison. Fix equal or justified budgets before encoding. This preserves the proposed three-candidate experiment.

## Checks that did not justify additional findings

- **R1:** The revised baseline is substantially accurate. [Li et al.](https://www.cs.purdue.edu/homes/ninghui/papers/analysis_oakland03.pdf) supplies restricted-policy safety/containment analysis; [Sasturkar et al.](https://www3.cs.stonybrook.edu/~stoller/arbac-analysis-TCS.html) supplies ARBAC reachability analysis. Neither establishes the unspecified consumable-resource fragment. Decidability is neither disproved nor established before that fragment is frozen. The trivial-amendment and undecidability tests are real tests.
- **R2:** Same-summary/different-required-verdict histories are a genuine refuter. The summary schema must actually be frozen; permitting hidden causal histories inside summaries would defeat the experiment. I did not prove R2 impossible for every permitted freeze. [Künnemann et al.](https://arxiv.org/abs/1805.10891) supports the causal-accountability baseline. [Caltais et al.](https://arxiv.org/abs/1608.07880) explicitly restricts its compositionality results to non-communicating systems, so it does not already prove this communicating delegation-chain target.
- **R3:** King's DAG and three-level example are verified in the full manuscript. Context-inconsistent pairwise checking is a meaningful proposed failure, unlike the old attempt to refute ordinary function composition. Novelty still needs the scheduled full comparison.
- **R4–R6:** No universal impossibility found. Entrenchment needs a fixed anchor, which the plan openly grants. R6 also needs receipt availability and an observation point; these belong in the freeze, not an allegation that all shared-identity constraints are impossible.
- **Shards:** Disjoint atomic shards with static assignment and explicit coordinated actions are coherent. Global invariants require allocated rights or coordination as stated. A skewed workload can make a shard fail the existing scale gate; that is a real negative outcome, not an inconsistency. Shared policy dissemination and common witness load must be included in the already promised inventory.
- **Reuse list:** [Whittaker–Hellerstein](https://mwhittaker.github.io/publications/iconfluence_vldbj2020.pdf), [Hamsaz](https://mohsenlesani.github.io/companion/popl19/POPL19.pdf), and bounded counters are real, relevant precedents. F1 is a valid lease precedent. Attie–Lynch's creation theorem really has six numbered premises. The combined timed/dynamic theorem is correctly treated as an obligation, not as published reuse. “Controller correctness” and “conservation with funding claims” are application lemmas to prove, not identified published theorem statements; attach a source/theorem/instantiation ledger during step 0 rather than treating the heading as evidence they are already available.

## Could not verify; potentially useful prior art

I could not verify an existing theorem giving exactly the proposed combined timed/dynamic substitutivity result. A relevant lead absent from the plan is [Dynamic networks of heterogeneous timed machines](https://www.cambridge.org/core/journals/mathematical-structures-in-computer-science/article/abs/dynamic-networks-of-heterogeneous-timed-machines/B9BAFFFD52920A34CBE359A88734BB2F): its publisher abstract describes discrete timed I/O machines and composition/refinement across clock granularities. The attempted PDF fetch redirected to the abstract. It should be screened in the already planned mechanisation/core survey, but I cannot claim it solves dynamic creation or dissolution under this plan's semantics.

The timed-I/O monograph fetch timed out; the HAL causality page and the DBLP contract-blame entry were inaccessible. I therefore do not certify their exact theorem statements. The Interface Automata PDF fetched with damaged text extraction; I did not audit its proofs. These limits are not findings of falsity. Abadi–Lamport is the most directly useful additional required reading: it is already in the background survey but missing from this plan's required source list, and it changes the assurance derivation in finding 1.

The unqualified sentence about two-party fair exchange needs the intended strong-fairness model and assumptions attached during the source ledger exercise. I did not verify the original impossibility proof here. An adjudicating unit by itself is not a proof of exchange fairness, but the plan only promises dispute referral, so I do not treat it as claiming one.

## Verdict and order of work

The narrowed claim is honest and worth testing. It would establish a compositional controller-and-evidence calculus, not a foundation for autonomous human institutions or learned cognition. The explicit limitations do not defeat that purpose. The benchmark now exercises formation, dissolution, concurrent change, several authorities and evidence failures, rather than merely giving institutional names to a static tree.

Proceed to step 0, with these seven repairs treated as freeze requirements. Keep the two immediate checks and paper scale argument before mechanisation; add the assurance circularity and authority-conflict checks alongside them. The R2 same-summary stop rule can genuinely fire, and its failure should remain explicitly candidate-specific. Do not spend the mechanisation budget until the revised assurance rules, authority semantics and scale parameters survive the paper review. There is no evidence here warranting a CRITICAL or HIGH rating merely because the remaining targets are open.

CRITICAL: 0  HIGH: 0  MEDIUM: 7  LOW: 0

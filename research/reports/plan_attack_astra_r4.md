# Adversarial review of the integrated foundation plan, version 4

Reviewed 17 November 2026. I assessed the current plan before consulting earlier reviews, then checked the latest repairs against their recommendations and fetched primary sources. No subagents were used. Only this review file was written.

**Sound enough to start step 0, but not to freeze its current wording unchanged.** I found six MEDIUM defects and no demonstrated CRITICAL or HIGH defect under the requested scale. I do not treat an unspecified protocol or a deliberately open target as an impossibility result. The counterexamples below require attention during the already scheduled definition freeze.

“Verified wrong” identifies an invalid implication or a correction checked against a source. “My judgement” identifies my counterexamples, interpretation and recommendations. “Could not verify” identifies limits of this review.

## 1. MEDIUM — Pairwise priority resolution does not establish joint consistency

**Text attacked:** R3 promises that “the effective norm set is consistent whenever the declared priorities form a strict partial order that decides every conflicting pair”; effective norms override a lower-priority norm “only where it conflicts”. Variant 10 supplies a priority cycle.

**Verified wrong under the ordinary pairwise reading; counterexample is mine.** Consider three unconditional norms concerning one deadline: perform A, perform B, and do not perform both A and B. Every two norms are satisfiable together; the three are not. Give their authorities a strict total order. The priority premise holds, but a resolver removing only pairwise-conflicting norms removes nothing. Separately feasible resource obligations that jointly exceed a cap give another example. Identity vocabulary mappings suffice, so a condition addressing only vocabulary interpretation does not remove this problem.

A cycle tests rejection of invalid priority declarations. It cannot refute this consistency implication because it violates the strict-partial-order premise. The new conclusion therefore needs another refuting case.

**Evidence:** [Liao, Oren, van der Torre and Villata, *Prioritized Norms in Formal Argumentation*, Definitions 3 and 5](https://arxiv.org/pdf/1709.08034) supplies explicit selection procedures, including consistency checks against accumulated consequences and context, and consistency results for its particular formalism. Priority ordering is not a substitute for those semantics. The three-norm example is my logical calculation, not a claim that their restricted literal language admits exactly that syntax.

**Exact change:** Define consistency over jointly realizable traces and specify the resolver in step 0. Either prove the frozen fragment has a pairwise-conflict property, or resolve inconsistent sets with a specified priority-sensitive selection procedure. Specify the outcome when several selections remain possible. Add the A/B/not-both example and resource overcommitment with acyclic, fully specified priorities. Keep the cycle as input validation. Put inherited resolver-consistency results in the reuse ledger; R3's prospective novelty remains context-dependent multi-authority composition.

**Severity and fidelity:** This sentence follows Fable r3 H2's suggested replacement, which was not a theorem. MEDIUM is appropriate because policy grammar and compliance semantics are explicitly frozen in step 0; a restriction or precise resolution rule can repair it without abandoning R3.

## 2. MEDIUM — An honest signer does not certify a unique or complete history

**Text attacked:** “A receipt is accepted only with f+1 witness signatures, so at least one signer is honest”; “Per-unit summaries carry f+1 signatures”; summaries state “whether [a guarantee] was broken and when”.

**Verified:** The honest-signer arithmetic is correct. Two accepted quorums need not intersect in an honest witness. For f=1, take honest H1 and H2 and faulty F. Both {H1,F} and {H2,F} meet the threshold. A faulty controller can show different individually replay-valid prefixes to H1 and H2, with F signing both. Neither honest witness needs to sign twice. A signature collection alone therefore establishes neither one history nor completeness of a whole-unit summary.

**My judgement:** “No breach in this authenticated prefix” can be accurate while “this unit did not breach during this interval” is false because a second fork contains a conflicting spend. Forwarding can eventually reveal the fork; it does not provide instantaneous finality, particularly across unbounded-delay links. The verdict might be provisional, revised later, or delayed until a completion condition. The plan needs to choose. A correct signing protocol could address this; it is not yet specified.

**Evidence:** [Syta et al., CoSi, §III-B and §IV footnote 1](https://arxiv.org/pdf/1503.08768) expressly distinguishes witnessed statements from prevention of divergent histories; its stated quorum-based anti-equivocation condition uses 2f+1 signatures from 3f+1 witnesses. [PeerReview, §§4.6–4.9](https://haeberlen.cis.upenn.edu/papers/peerreview-tr.pdf) adds consistency checking, challenges and evidence transfer. I am not claiming that 3f+1 is universally necessary under this plan's synchrony assumptions, or that accountability requires consensus.

**Exact change:** Freeze what honest witnesses check before signing receipts and summaries; bind signatures to controller identity, witness epoch, policy version and identified log prefix. Distinguish prefix-local evidence from complete interval claims. Add a split-view case with H1/F and H2/F signing different prefixes, followed by delayed evidence exchange. Specify R2's eventuality and revision semantics and charge the exchange to scale. If final non-equivocating certification is intended, justify a protocol supplying it instead. Do not change thresholds reflexively.

**Severity and fidelity:** The threshold faithfully implements Fable r3 H1(a) and fixes the single lying witness problem. The remaining signing/consistency semantics belong in the scheduled protocol freeze, hence MEDIUM. This is not a proof that every permitted R2 freeze fails.

## 3. MEDIUM — Refusal during adjudication can violate an obligation, not merely reduce availability

**Text attacked:** “Where a conflict has no declared priority, the unit refuses the affected actions until a named adjudicator decides, and the refusals count as unavailability.” Obligations require a receipt before a deadline; adjudicator links have no delivery bound.

**My judgement / counterexample:** Procurement law requires a report by time 10; group policy forbids it. No priority exists and adjudication arrives at time 20. Refusal avoids one violation but misses the other obligation's deadline. Calling this unavailability does not suspend the obligation or make the effective norm set consistent. This affects R2's deviation/loss distinction and R5's conformance invariant. The openly stated narrowing to resolved norms is sound; this is the unresolved branch.

**Evidence:** [David et al., *Timed I/O Automata*, §2's implementation conditions and subsequent consistency treatment](https://homes.cs.aau.dk/~adavid/publications/41-hscc10.pdf) makes timing/progress requirements substantive. It does not supply a legal suspension rule. The counterexample follows directly from this plan's discharge definition.

**Exact change:** Give pending conflicts an explicit normative status during step 0. If a superior procedural norm suspends the deadline, record the suspension and its authority. Otherwise record the missed obligation and its R2 classification and loss bearer, not only unavailability. Add the late-adjudication example to variant 10 and the assurance exercise. Restrict the consistency claim to valid resolution states, or define a pending state's effective obligations.

**Severity:** A missing branch in semantics already scheduled for step 0, not an objection to refusal or its admitted availability cost.

## 4. MEDIUM — R1 incorrectly treats finite-state analysis as automatically trivial

**Text attacked:** R1's refuter: “The analysis is trivial (bounded quantities and a fixed set of principals, or everything reachable after an amendment)”.

**Verified wrong:** Bounded quantities and fixed principals do not imply computationally trivial analysis. Even if all other state is finite, symbolic reachability can be difficult as the policy input grows. R1's own ARBAC baseline proves PSPACE-complete reachability. Finite-state decidability by enumeration is elementary; an efficient symbolic algorithm, tight bound or useful restriction need not be. Those two bounds alone also do not make arbitrary clocks, logs or amendment-generated syntax finite.

**Evidence:** [Sasturkar et al., *Policy Analysis for Administrative Role Based Access Control*, abstract and reachability analysis](https://www3.cs.stonybrook.edu/~stoller/papers/arbac-analysis-TCS.pdf) establishes PSPACE-completeness and studies restrictions changing complexity.

**Exact change:** Replace the parenthesis with: “Merely observing that a fully finite-state encoding is decidable does not count as an advance; a bounded encoding must meet the frozen complexity target in the succinct input size and improve on the stated baseline.” Freeze representation sizes, including binary/unary quantities and permitted evolution of rule syntax. Keep the separate failure where amendment makes every authority reachable. Do not require unbounded counters merely to escape the label “trivial”.

**Fidelity:** This inherits Fable r3 M2's inference from finite-state decidability to lack of useful analysis. Fixing the complexity class was good; automatically rejecting bounded fragments overcorrects it. This is a MEDIUM correction to the success criterion's wording, not a demand to redesign the analysis.

## 5. MEDIUM — The assurance-record citation puts the safety restriction on the wrong object

**Text attacked:** “An interface assumption is removed only through a sound assume-guarantee rule, which exists for safety guarantees of the form ‘held until time t’ and not for circular promises of eventual action”.

**Verified correction:** Abadi and Lamport's rule is not characterized simply by guarantees being safety properties. Their §4.3.1 permits fairness in component specifications while effectively requiring discharged environment assumptions to be safety properties. Breaking the circular inference also requires the extra-step/closure premise. Merely labeling a bounded-deadline promise as safety does not justify mutual discharge: two parties can each condition sending by the same deadline on the other's sending by that deadline, and both remain silent.

**Evidence:** [Abadi and Lamport, *Conjoining Specifications*, §4.3.1, Theorem 1, and §§5.1–5.2, Theorem 3](https://lamport.azurewebsites.net/pubs/abadi-conjoining.pdf). The discussion explicitly allows fairness and identifies the restriction on environment assumptions.

**Exact change:** Say: “Assumptions are discharged only by an instantiated sound assume-guarantee rule, including its safety-closure, temporal and hiding premises. The rule and its admissible assumptions and guarantees are recorded in the reuse ledger. Matching circular deadline promises alone does not discharge either assumption.” Retain the circular-promise and grouping-independence tests. A deadline does not automatically supply the extra-step condition.

**Severity and fidelity:** Requiring a *sound* rule is already correct, so this is not an architectural defect. The explanatory clause follows Fable r3 M5 too literally and needs a source-faithful correction.

## 6. MEDIUM — R5's test does not test completeness, and its failure branch still invokes R5

**Text attacked:** The check “is decidable, and it is complete against amendments that change the meaning of terms the invariants use”; refutation is “an accepted amendment after which an entrenched invariant no longer holds in substance; or a check that rejects the useful self-amendment frozen in step 0”. If R5 fails, self-reflection is withdrawn, but step 3 still requires amendments to “pass[] the entrenchment check of R5”.

**My judgement / logical distinction:** Acceptance of an unsafe amendment tests soundness. Rejection of one designated safe amendment tests that usefulness requirement. Neither tests semantic completeness over the grammar. A checker accepting only the designated amendment can pass both tests while rejecting other preserving amendments. Conversely, incompleteness need not destroy sound preservation: a conservative checker might still support useful self-amendment. Unsoundness is materially different and affects rule change and R4, not only the self-reflection label.

**Evidence:** This is my logical counterexample, not an undecidability assertion. [Sasturkar et al.](https://www3.cs.stonybrook.edu/~stoller/papers/arbac-analysis-TCS.pdf) is a concrete baseline for separately defining decision problems and their analysis in a restricted policy language; it does not prove R5. The checker semantics must come from this plan's freeze.

**Exact change:** Freeze a semantic preservation predicate and separately state termination, soundness and completeness, with completeness meaning acceptance of every preserving amendment in the declared class. Rejection of any such amendment is a completeness refuter. If “complete against” only meant sound under semantic redefinition, use that wording. Define the fallback: incompleteness may retain a sound conservative checker with narrower claims; unsoundness requires disabling the affected amendment class or repairing its checker before continuing. Make step 3's validity rule match that branch.

**Severity and fidelity:** Statement and branch clarifications within the existing frozen-grammar experiment. Fable r3 M4 supplied the ambiguous completeness language. No universal impossibility for a useful restricted grammar was established.

## Additional checks, prior art and review limits

**Witness/gateway repair:** Outside appointment, forwarding, full log availability, replay-load accounting and the vendor gateway are coherent improvements. They do not certify vendor internals, which are explicitly excluded. Specify eventual delivery separately from bounded delivery when freezing the network: “no delivery bound” does not itself settle whether permanent loss is admitted.

**R2:** Same-summary/different-verdict histories and summaries growing as large as inner logs are genuine refuters. A boundary trace prefix can defeat compression, but the plan makes that failure explicit. [Caltais et al.](https://arxiv.org/abs/1608.07880) really restrict their composition result to non-communicating systems. [Künnemann et al.](https://arxiv.org/abs/1805.10891) supplies a causal-accountability baseline. **Could not verify:** I did not establish a published theorem already proving this communicating-chain target or prove that no permitted summary works. Do not enlarge the frozen summary schema after a counterexample without reporting the original target's failure.

**R4 and R6:** R4 now has a useful accepted class and no automatic “record failure” success outcome; its refuter is meaningful. R6 identifies an observer and makes split purchases and vocabulary matching substantive. Inadequate receipts can defeat semantic purchase matching; that is the stated test, not an unacknowledged impossibility. Naming the ministry does not create a delivery path: instantiate the receipt-transfer duties in step 0.

**Missed prior art worth adding before new core work:** [David, Larsen, Legay and Poulsen, *Statistical Model Checking of Dynamic Networks of Stochastic Hybrid Automata* (2013), §2](https://homes.cs.aau.dk/~kgl/SSFT2015/DYN.pdf) provides timed I/O transition-system semantics with dynamic creation and termination and an UPPAAL extension. It draws on the specification-theory line selected for the second candidate. Add this concrete construction precedent to the source reading and bake-off inventory. **Could not verify:** I did not find the needed creation/hiding substitutivity theorem there. The paper does not therefore invalidate the qualified absence claim or eliminate the proposed new metatheory. Liao et al. above similarly changes R3's reuse ledger, not the whole proposal.

**Order and stop rules — my judgement:** Source reading, immediate cases, frozen statements and the paper-first gate are sensible. Run the new norm and split-view examples before general proofs. Failure of the default core's time/creation obligations should trigger the stated alternative-core evaluation, not become a claim about impossibility of the overall goal. Requiring an R1 or R3 advance in addition to R2 is a legitimate ambitious criterion. But missing it does not establish “repackaging” if R2 itself is novel: report the failed criterion and preserve any independently novel result. That reporting correction does not warrant a separate architectural finding.

**Assurance and scale:** Candidate record rules, boundary observables, delivery terms and grouping independence are faithful repairs. The authority-DAG parameters and dependent-lease rule address the earlier tree error. Instantiate policy-size dependence in the per-decision bound; authority count alone does not bound policy evaluation. This is already a scheduled proof/load obligation. No deployment at 10^10 has been demonstrated, and the plan says so.

## Verdict

**Sound enough to start step 0.** The latest edits introduce errors in the explanations of consistency, finite-state analysis and assume-guarantee reasoning, plus incomplete protocol and amendment-check branches. They are repairable within the scheduled freeze and immediate tests; none demonstrates that the narrowed programme cannot succeed.

The narrowing remains honest and worth testing: mediated interactions, resolved norms and anchored governance, rather than legitimacy, unmediated human behaviour or empirical control of ten billion independent actors. The strongest prospective contribution remains compact compositional accountability coupled to useful governance semantics. A positive result must demonstrate that contribution rather than count signatures, definitions or reproduced consistency lemmas as new foundations.

CRITICAL: 0  HIGH: 0  MEDIUM: 6  LOW: 0

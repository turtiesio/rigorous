# Adversarial review of the integrated foundation plan, version 6

Reviewed 17 November 2026. I read the current plan and its v5 diff before consulting the earlier reviews, then checked the round-5 recommendations and primary sources. I worked alone and wrote only this file. “Verified wrong” identifies a false implication or a source mismatch; “my judgement” identifies my deductions and severity assessment; “could not verify” identifies limits of the literature check.

**The plan is sound enough to start step 0, but its new pool shortcut is not a sound gate for choosing the core.** I found one HIGH defect and four MEDIUM defects. None establishes that the narrowed foundation is impossible. The strongest defect comes from combining two individually sensible round-5 recommendations without checking their interaction.

## 1. HIGH — The pool shortcut assumes away the eventuality that R2 now explicitly requires

**Text attacked:** “Every property the plan claims is decided on finite prefixes of a run”; if the pool theorems hold for every size with size-independent hypotheses, “the plan takes that as its core”. Step 2 says that if this argument succeeds, “the default is not needed”. R2 requires that every covered violation caused by deviation “eventually yields” a verdict and includes a convergence refuter.

**Verified wrong:** Eventual detection under unbounded delivery and eventual convergence are not safety properties established by checking all finite prefixes. A finite prefix with no verdict can still extend to a run with a verdict. Even a prefix ending in a verdict does not certify that its value never changes again. “No general liveness” does not remove these particular liveness obligations from R2.

**My counterexample to the proposed inference:** Consider a family of systems in which a covered event occurs at time zero and a verdict is produced at time N for pool size N. Every finite member satisfies eventual reporting, under identical environmental hypotheses. The limit execution can postpone the verdict forever: every finite prefix without a verdict is compatible with a sufficiently large member. Equivalently, a procedure may wait for creation to cease; every finite non-reusable pool eventually exhausts, whereas an unbounded creation execution need never become quiescent. The finite-prefix observation does not exclude this behaviour. This is a counterexample to the inference, not a claim that the planned controller must implement this procedure.

The error matters even if all finite-pool composition theorems are correct. Those theorems do not themselves supply an infinite-population semantics or a fairness-preserving correspondence with unbounded creation. Size-independent *hypotheses* are insufficient to supply that correspondence.

**Evidence:** [Alpern and Schneider, *Defining Liveness*](https://ecommons.cornell.edu/entities/publication/2ed32f4f-cc5c-413b-ba16-5498641f1939) distinguishes safety from eventuality. [PeerReview, §3.5 and Appendix A](https://haeberlen.cis.upenn.edu/papers/peerreview-tr.pdf) separately requires eventual completeness and accuracy; its evidence dissemination does not reduce completeness to finite-prefix safety. [Attie and Lynch](https://arxiv.org/abs/1604.06030) supplies conditional creation substitutivity, not a theorem permitting this pool-to-unbounded inference.

**Exact change:** Replace the shortcut with two proof obligations: (i) a coherent encoding of finite creation histories into sufficiently large pools, preserving interfaces, hiding and the required safety properties; (ii) a separate progress argument for each fixed covered event under ongoing creation, preserving delivery, witness processing and any required fairness. Require both before discarding the dynamic candidate. Alternatively restrict the positive result explicitly to finite populations and leave unbounded creation untested. Add a paper case with continuing creation while one earlier event awaits its verdict. A finite timeout is not a refutation of unbounded eventuality; the refuter must be an admissible infinite execution or a proof of non-convergence.

**Severity and fidelity:** HIGH because the core-selection gate and its proof obligations must change, not merely a step-0 parameter. Fable r5 M6 proposed the shortcut under a safety-only reading. V6 adopted that reading while simultaneously adopting eventual R2. Neither suggestion by itself establishes this combined shortcut. This is not an objection to researching a pool encoding.

## 2. MEDIUM — Lease expiry does not tell the ledger how much old authority was already spent

**Text attacked:** R4 keeps “one ledger of completed spending, outstanding commitments and every right that can still be exercised under any version”; “the grandfathered set is closed only when the last old lease has expired, when unspent old rights return to the ledger”.

**My judgement / counterexample:** The new ledger correctly reserves live old rights, but expiration and settlement remain different events. Suppose an old holder has 100 rights, spends them just before expiry, and its spending report is still in transit when the lease expires. The owner cannot distinguish this history from one in which the holder spent nothing. If it removes the old reservation because the rights are no longer exercisable and treats the unreported amount as returned, it can issue another 100 while the first 100 has already been spent. This can occur with correct controllers. A bound on the holder's own witness links does not establish that the resource owner already has the final spending report.

This is not a counterexample if “unspent” means *certified unspent after reconciliation*. But that reading requires keeping expired, unsettled reservations, a category omitted from the three-part formula. A single logical ledger is not automatically synchronously informed about actions taken under distributed rights. Likewise, a commitment backed by reserved rights must move value between categories rather than be charged twice.

**Evidence:** [Balegas et al., §III, especially the invariant argument](https://arxiv.org/pdf/1503.09052) maintains conservative local rights through explicit consumption and transfer records. It does not infer a remote replica's remaining balance from elapsed time. The timing counterexample above is mine.

**Exact change:** Add “expired but unsettled allocations” to the conserved reservations. Expiry fences future exercise; only an authenticated final balance, with all pre-expiry spending and commitments reconciled, releases the unused reservation. Otherwise retain the entire unresolved allocation conservatively. Define disjoint ledger categories so reserved commitments are not double counted. Change “closed ... when the last old lease has expired” to “closed after expiry and settlement of every old allocation”; settlement time can be unbounded outside the envelope. Add spend-immediately-before-expiry/report-after-expiry to variant 9.

**Severity and fidelity:** MEDIUM: this is a revocation/accounting detail already assigned to step 0 and the conservation lemma. R4 has a genuine failure test. The new expiration sentence closely follows Fable r5 M1; Astra r5 item 3 more cautiously required expiry/reclamation to be established. Keeping the old rights in the ledger fixes the overlap case but does not by itself fix their retirement.

## 3. MEDIUM — A total order does not automatically import the cited selection theorem into the plan's richer conflict semantics

**Text attacked:** “because the priority order on norms is total it leaves no conflict undecided”; R3 says “consistency and uniqueness of the selection under a total order are inherited from the chosen published procedure”. The plan defines conflict by joint satisfiability from a state, including triples of obligations.

**Verified source mismatch:** Liao et al. define a finite literal-based normative system, with norms as literal pairs and consistency by complementary literals. Their procedures are not interchangeable: Reduction can have multiple extensions even with priorities, while the stated uniqueness results concern particular constructions. This is not a ready-made theorem for timed, state-dependent feasibility of arbitrary sets of obligations.

**Evidence:** [Liao et al., Definition 1, Example 4, and Lemmas 1–2](https://arxiv.org/pdf/1709.08034). The paper itself supplies the restriction and the multiple-extension example.

**My judgement:** A finite greedy scan in total priority order using an exact joint-feasibility oracle could give a deterministic consistent selection, including triple conflicts. Thus this is not an impossibility and not another HIGH consistency finding. But the oracle, translation into the cited formalism and feasibility of the empty/base constraints matter. Sorting does not supply them. The current same-authority issue-time rule also needs a serial issue index or tie-break if timestamps can coincide.

**Exact change:** In step 0 select the actual construction and prove its applicability to the frozen policy grammar. Either provide a semantics-preserving encoding into the cited theorem's domain, or enter only the applicable portion as reuse and prove the finite greedy-selection lemma against the frozen joint-feasibility predicate. Do not select Reduction while promising unconditional uniqueness. Include the three-obligation example and same-time issue tie in the checks.

**Severity and fidelity:** MEDIUM because procedure, grammar and detection bound are explicitly scheduled for freezing. V6 faithfully removes pending choices and puts uniqueness in the reuse ledger, but it has not earned the claimed theorem instantiation. This requires a local applicability check, not reinstating conflict adjudication.

## 4. MEDIUM — Origin deduplication fixes priority inflation, but does not resolve contradictory translations

**Text attacked:** “Each norm carries its origin and is counted once, at its origin's priority, however many authority paths it arrives by.” Variant 10 now includes “a ministry norm that reaches the subsidiary directly and through the group with different translations”.

**My judgement / counterexample:** Let the same ministry norm translate to “report purchase x” on the direct path and “do not report purchase x” through the group. Both instances have the same origin and priority. Deduplicating their identifiers does not say which formula survives. Keeping whichever arrives first makes the effective norms depend on message order; keeping both produces a conflict between copies of one norm whose declared origin priority cannot distinguish them. Counting once therefore cannot substitute for a condition on vocabulary mappings.

This does not prove R3 false: the local predicate is not yet frozen, and its refuter now explicitly includes this case. It does mean the new effective-norm definition is incomplete on an input that the benchmark expressly requires.

**Evidence:** The distinction is a direct counterexample to the plan's definition. The [cited Liao formalism, Definition 1](https://arxiv.org/pdf/1709.08034) takes each norm's content as given; its priority machinery does not choose between path-dependent meanings of one norm. No theorem in that source repairs the missing interpretation rule.

**Exact change:** Freeze one explicit rule: path translations must agree semantically and inconsistent declarations are rejected; or an authorised canonical translation/path determines meaning at the target. If multiple refinements are intended, define their combination and prove its relation to the source norm. Retain provenance for checking that condition, even though the norm is counted once for priority. Give variant 10 a required outcome and check independence from arrival order.

**Severity and fidelity:** MEDIUM: the mapping class and local compliance predicate already belong to step 0. Fable r5 M5 offered commuting translations *or* origin deduplication as alternatives. They solve different problems; copying the second alternative did not solve the first.

## 5. MEDIUM — The new availability parameters still do not determine an expectation

**Text attacked:** Expected availability is a function of the ancestor graph, lease period/margin, and “a per-link probability of meeting the envelope bound”.

**Verified insufficiency; my calculation:** Two renewal links can each meet their bound with probability 1/2, on exactly the same ancestor graph. If successes are independent, both succeed with probability 1/4. If perfectly aligned, both succeed with probability 1/2. If mutually exclusive, both succeed with probability zero. The graph and marginals are identical. Graph overlap is not a joint probability law. Temporal dependence also matters when several renewal attempts fit inside one lease.

**Evidence:** [Orloff and Bloom, *Joint Distributions, Independence*, §§3–4](https://ocw.mit.edu/courses/18-05-introduction-to-probability-and-statistics-spring-2022/mit18_05_s22_class07-prep-a.pdf) gives the distinction between joint and marginal distributions. The three constructions above are elementary counterexamples of my own.

**Exact change:** Freeze a joint delay/renewal model, including dependence across links and time, or explicitly assume the needed independence. Alternatively report empirical availability for each fixed workload and graph, or distribution-free bounds rather than a uniquely determined expectation. State whether “inside the envelope” means conditioning on all required links meeting their bounds—in which case their success probabilities should not be applied again—or sampling the workload that sometimes meets it.

**Severity and fidelity:** MEDIUM, a workload-parameter correction. The zero asynchronous lower bound and ancestor graph are sound repairs. Fable r5 M8's per-link probability suggestion was insufficient; Astra r5 item 5 requested a joint renewal model. Excluding correlated *faults* does not by itself specify dependence of otherwise compliant network delays.

## Remaining targets, prior art and round-5 fixes

**R1 — my judgement:** The frozen grammar, input representation, restriction rule and separate complexity targets make the experiment meaningful. I found no basis to declare the proposed combination known impossible or already proved. **Could not verify:** no novelty theorem for the whole combination was established in this review; the reuse ledger must still do that work.

**R2 — my judgement:** Eventuality repairs the challenge-period refuter, and retaining evidence until finality is coherent, although storage can grow outside the envelope. The never-falsely-name requirement correctly still applies to provisional proved-deviation verdicts. Such verdicts must contain sufficient positive evidence; missing evidence may justify suspicion but not blame. [PeerReview](https://haeberlen.cis.upenn.edu/papers/peerreview-tr.pdf) distinguishes those outcomes, and [Künnemann et al.](https://arxiv.org/abs/1805.10891) makes causal deviation substantive. Neither source proves the proposed communicating-chain summary theorem. The same-summary/different-verdict refuter remains real. [Caltais et al.](https://arxiv.org/abs/1608.07880) explicitly restrict their composition result to non-communicating systems, as the plan now correctly reports.

The f+1 receipt threshold guarantees at least one honest signer, not honest intersection of two accepted certificates. The plan acknowledges that distinction. Witness receipts as an alternative discharge route can be made coherent in step 0 by specifying the signed performance, obligation/version identity and the threshold-time rule; I found no inherent impossibility in that repair. Conditional finality also needs processing bounds and an explicit evidence scope, already appropriate subjects for the scheduled envelope freeze. A protocol cannot simply infer that every remote envelope condition held from silence; the theorem can state that condition externally.

**R3 and R5 — my judgement:** The bounded selection description gives R3 a substantive compression/locality question, rather than unrestricted global checking disguised as a local check. It must admit the compliant/conflict-resolved benchmark cases; accepting no edge is not useful success. The amended R5 fallback and separation of its conformance dependency are coherent. Entrenching the obligation to run the check is an improvement; the frozen semantic-preservation test must cover changes to the check's meaning as well as its invocation. That is already within R5's explicit scope, so its difficulty is not another finding.

**Other removals — my judgement:** Fixed ranks make acyclicity locally checkable if ranks are immutable and authenticated. The rank domain must support the permitted creation histories; choosing it belongs to step 0. Anchor-issued priority changes are governed version changes and must use the same lease discipline. Exempting allocated-rights invariants from selection removes the earlier cap/override contradiction. Removing consent-founded authorities and norm-conflict adjudication eliminates the earlier cases rather than solving their general forms, and the limitations section says so honestly. Reclassifying R6 as an application lemma is appropriate; giving the audit body's reporting duty a state authority repairs its missing basis. The revised vendor-dispute case remains meaningful without pretending to test witness independence.

**Prior-art limits:** I searched for timed dynamic I/O composition and rechecked the dynamic I/O source; I did not verify a published theorem establishing the exact combined creation/time/hiding result. Absence from this search is not evidence of novelty. The most consequential missing applicability check is now the safety/liveness distinction in finding 1, rather than another governance framework. The stated source-reading stage is still necessary before a reuse ledger can be trusted.

## Verdict and order of work

**Sound enough to start step 0.** Do not accept the pool shortcut as written. First split the claimed properties into safety and progress obligations, and change the core-selection gate accordingly. Then run the delayed-settlement and disagreeing-translation cases while freezing their semantics, establish which norm-selection result actually applies, and freeze the joint availability model. Preserve the paper-first stop rule before mechanisation.

The narrowing remains honest and worth testing. It studies anchored, mediated governance with external witness and identity assumptions; it does not claim a self-governing foundation for all humans or institutions. R2's compositional evidence claim and R1/R3's analysis claims can still fail in meaningful ways. No demonstrated CRITICAL defect or reason to abandon this candidate emerged, but a successful finite-pool proof alone must not be reported as supporting the plan's new eventual-accountability claim under unbounded creation.

CRITICAL: 0  HIGH: 1  MEDIUM: 4  LOW: 0

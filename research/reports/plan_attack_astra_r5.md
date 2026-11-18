# Adversarial review of the integrated foundation plan, version 5

Reviewed 17 November 2026. I assessed the current plan and its difference from version 4 before consulting the earlier reviews. I then checked the round-4 recommendations against the edits and searched and fetched primary sources. No subagents were used. Only this review file was written.

**Sound enough to start step 0.** I found five MEDIUM defects, no demonstrated CRITICAL or HIGH defect. The most concerning new interaction is between eventual evidence delivery, a finite challenge period, and R2's new refuter. This requires correction before freezing R2, but the plan already permits revised verdicts and explicitly schedules the protocol semantics for step 0; it does not establish that the accountability programme is impossible.

Labels distinguish **verified wrong** implications, **my judgement** about the plan and counterexamples, and matters I **could not verify**. The counterexamples below are mine, not claims that the cited papers analyse this exact architecture.

## 1. MEDIUM — A challenge period cannot turn eventual evidence exchange into finality

**Text attacked:** “A chain verdict is therefore provisional until a stated challenge period has passed, and a verdict that is later contradicted is revised on the record.” R2's new refuter is “one history with two valid summaries that still give different verdicts after the challenge period”. Inter-unit messages have “no delivery bound, but every message is eventually delivered”.

**Verified wrong implication; my counterexample:** For any fixed challenge duration T, take the split-view case the plan explicitly permits. With f=1, the faulty controller and faulty witness F produce one accepted prefix with honest H1 and another with honest H2. Two external observers obtain the respective summaries. Delay the forwarded evidence that would reconcile their views until T+1. This meets eventual delivery and does not violate the bounded controller-to-own-witness links. The controller need not relay evidence between honest witnesses. The observers can therefore still reach different conclusions at T. Later revision, which the plan explicitly permits, does not undo failure of its literal new test.

There is a second ambiguity: two summaries of *different* prefixes can legitimately yield different verdicts without any fork. A prefix preceding a breach and one containing it need not have the same verdict. A refuter must compare the same adjudicated event and evidence scope, not arbitrary prefix verdicts.

**Evidence:** [PeerReview, §§3.5 and 4.9](https://haeberlen.cis.upenn.edu/papers/peerreview-tr.pdf) distinguishes eventual evidence dissemination from accuracy of exposure; it allows suspicion to be withdrawn. [CoSi](https://arxiv.org/abs/1503.08768) establishes exposure of signed statements to witnesses, not that a timer reconciles all views. Neither supplies the missing bounded-delivery premise.

**Exact change:** Replace the fixed-time refuter with: “For a fixed covered event and specified evidence scope, verdicts fail to converge after all required evidence is delivered and processed, or a proved-deviation verdict ever names a compliant party.” State that the challenge period is an operational settlement/reopening rule, not a completeness or finality guarantee. If a bounded finality result is wanted, state and test the additional dissemination and processing bounds separately. Freeze retention so that evidence needed for an outstanding challenge is not discarded merely because T elapsed; otherwise restrict the theorem's coverage explicitly. Add delivery at T+1 to variant 8.

**Severity and fidelity:** This follows Fable r4 M3's suggested provisional-verdict wording, but its new R2 refuter reinstates the fixed-time property that provisional verdicts avoid. This is MEDIUM because eventuality, challenge and retention semantics are already assigned to step 0, and the repair preserves the existing eventual-accountability design.

## 2. MEDIUM — Pending conflict is not sufficient evidence that nobody deviated

**Text attacked:** “A deadline caught in a pending conflict is either suspended by a recorded procedural norm of a superior authority or, if missed, classified as allocated loss with a named bearer.” Elsewhere, allocated loss is defined as applying “where nobody deviated”, and R2(a) requires a proved-deviation verdict for every covered violation caused by a deviation.

**My judgement / counterexample:** An authority's controller issues a receipted amendment that its charter forbids. That amendment creates a conflict with an obligation due at time 10. The constrained unit follows the pending-conflict procedure, and adjudication arrives at time 20. The leaf's refusal can be compliant while the authority's unlawful amendment caused the missed obligation. The deadline paragraph unconditionally assigns the loss-only classification, whereas R2 requires causal deviation to be considered. Merely recording which authority caused the conflict does not repair the verdict type.

An implementation might reject the invalid amendment before it becomes a pending conflict. That is a valid step-0 solution, but it must specify validation under the accountability threat model, which permits faulty controllers; the present unconditional sentence does not say this. Nor should a later change to the effective norm set erase an already established breach under the norms applicable when the action occurred.

**Evidence:** [Künnemann, Esiyok and Backes](https://arxiv.org/abs/1805.10891) define accountability using deviations that cause a security violation. This supports the distinction between causal blame and mere adverse outcome; the inconsistency here is between the plan's own definitions, not a claim that the paper proves this benchmark impossible.

**Exact change:** Say: “A missed deadline during a pending conflict is recorded and a loss bearer is assigned. It is classified as no-deviation allocated loss only when the frozen causal analysis finds no causal deviation; otherwise the proved-deviation or suspicion procedure also applies. Historical compliance is evaluated under the applicable norm/version history.” Freeze whether invalid amendments are excluded before conflict resolution. Add the lawful-conflict and unlawful-conflict versions of the same late-adjudication case.

**Severity and fidelity:** The automatic loss wording follows Fable r4 M10. Astra r4 instead requested the missed obligation's R2 classification. The difference matters, but is a verdict-semantics correction within step 0, not a reason to abandon conflict handling.

## 3. MEDIUM — The residual-cap formula needs to reserve still-live rights, not only commitments

**Text attacked:** “new rights are issued against the new cap minus completed and grandfathered spending, floored at zero”; step 5 concludes “total spending never exceeds the larger of the two caps”. Old rights stop authorising only when their leases expire.

**My judgement / counterexample to the formula read literally:** Old cap 100; no completed spending or commitments; an old shard holds 100 unspent rights with a live lease. Raise the cap to 120. The stated residual is 120 because completed and grandfathered spending are both zero. If the authority allocates those 120 to a new shard before the old rights expire, the two shards can spend 220. This exceeds both caps, even though each shard respects its local allocation. The old shard is explicitly allowed to hear the amendment late.

The same issue exists within one version if “issued against” does not debit a unique residual-rights account. Single resource ownership and the scheduled conservation lemma provide a natural repair; I am not claiming that every faithful implementation must double-issue. The formula alone is insufficient to justify the new conclusion.

**Evidence:** [Balegas et al., bounded-counter construction](https://arxiv.org/pdf/1503.09052) tracks rights available at each replica and transfers them explicitly. Rights already distributed remain part of the conservation accounting before they become completed operations. Its escrow construction is already in the plan's reuse list.

**Exact change:** Define a conserved residual ledger containing completed expenditure, outstanding commitments, and all still-exercisable unspent rights across versions, without double counting a commitment backed by reserved rights. Additional allocation must debit that ledger. At a version transition, either reserve old rights until their expiry/reclamation is established or delay new allocation until then. State the cap guarantee at every intermediate state, not only after expiry. Add the 100-to-120 example with a late old shard to variant 9.

**Severity and fidelity:** Fable r4 M6's residual formula was copied correctly, but omitted the outstanding-rights term needed during the overlap already admitted by the plan. This is MEDIUM because revocation semantics, rights conservation and lease installation are already scheduled; it does not require changing the architecture or R4's intended bound.

## 4. MEDIUM — R3 still lists the old counterexample as a refuter after changing the claim

**Text attacked:** R3's refuter includes “three norms that are pairwise satisfiable and jointly not, under acyclic and fully declared priorities”. Its new target instead promises conflict detection within a bound and a resolved set that “is unique, or its alternatives are bounded in number and each is recorded”.

**Verified wrong as a sufficient refutation:** A correct minimal-conflict detector should find exactly that triple. A correct prioritised selection can then omit a norm and produce a consistent effective set. The existence of the triple now exercises the resolver; it does not refute the new claims. This is an important regression in the test column even though the old false consistency assertion has been removed.

**My judgement:** “Bounded in number” also needs an explicit non-vacuous bound. For n active norms a selection among subsets has at most 2^n alternatives. Merely proving a finite bound for a finite norm set adds nothing. Step 0 freezes a conflict-detection bound, but does not explicitly freeze the acceptable resolution-count or enumeration-size bound.

**Evidence:** [Hansen's primary publication](https://drops.dagstuhl.de/entities/document/10.4230/DagSemProc.07122.25) treats maximal-subset selection as the response to conflicting imperatives. [Liao et al.](https://arxiv.org/pdf/1709.08034) give distinct prioritised constructions and their formal representations. These support selection semantics, not the claim that an inconsistent input itself refutes the resolver. The 2^n observation is elementary counting.

**Exact change:** Retain the triple as an input case. Replace the refuter by “the detector misses or misclassifies a minimal conflict, exceeds the frozen complexity bound, or the selector returns an inadmissible result, violates promised uniqueness, exceeds the frozen alternative-count/output-size bound, or omits a required alternative.” Add the latter bounds to step 0. Count consistency inherited from a published construction as reuse, as the plan already requires.

**Severity and fidelity:** The substantive round-4 repair is sound. The test column was not updated to match it. This is a MEDIUM test/parameter correction, not evidence against the remaining context-dependent conformance target.

## 5. MEDIUM — Availability is not determined by longest path and lease period

**Text attacked:** “A unit can authorise only while every authority on every path to a root has renewed its lease in time. Expected availability is stated as a function of the longest authority path and the lease period”.

**Verified insufficiency; my calculation:** Compare a unit with one direct authority and a unit with m independent direct authorities. Both have the same longest path and lease period. If each required lease is current with probability p at a sampled decision time, the lease condition has probability p for the first unit and p^m for the second, assuming independence. Shared ancestors and correlated renewal failures require a joint model instead. Counting pending-conflict time does not account for this difference.

Moreover, eventual delivery is not a probability distribution. It permits every renewal to arrive too late and supplies no positive availability floor by itself. A workload-conditioned expected-availability experiment is legitimate, but its stochastic assumptions must be supplied rather than inferred from eventual delivery.

**Evidence:** [F1's schema-change paper](https://research.google/pubs/online-asynchronous-schema-change-in-f1/) is the plan's lease precedent; it is not a theorem that longest authority path determines this new DAG's availability. The p^m comparison is my elementary calculation from the plan's all-authorities requirement, not a result attributed to F1.

**Exact change:** Parameterise availability by the affected authority/lease dependency graph and the renewal-delay/failure model, including shared dependencies, renewal margins and the sampling/workload definition. Alternatively report empirical availability for each frozen graph/workload without claiming a two-parameter bound. State explicitly that the asynchronous worst-case lower bound is zero. Keep the positive floor conditional on the frozen performance model and include the wide-join case in its measurement.

**Severity and fidelity:** Fable r4 M12 requested a path-length availability calculation. The new metric is useful, but that recommendation omitted width and probability assumptions. Adding them is a MEDIUM parameter correction; availability need not be abandoned.

## Other targets, prior art, and limits of verification

**R1:** The finite-state wording is now materially better. [Sasturkar et al.](https://www3.cs.stonybrook.edu/~stoller/arbac-analysis-TCS.html) explicitly establish PSPACE-complete ARBAC reachability; boundedness is not synonymous with trivial analysis. Freezing the two decision questions and their succinct input sizes is the right next step. I did not verify a theorem already proving the proposed combination of amendments, quantities and revocation.

**R2:** The same-summary/different-causal-verdict test remains substantive. Prefix binding, explicit fork coverage, atomic witnesses and the per-unit containment obligation are improvements. Per-unit containment must be proved using independently attributable evidence; a neighbour's signature must not be inferred from a failed witness set's allegation. This is already a named refuter, so I do not count its difficulty as another finding. Submission through the obligee's witnesses is a useful remedy, provided the freeze specifies acknowledgement and suspicion timing; receipt time at a witness is not automatically the obligation's discharge time at the controller. The plan openly limits coverage to receipts, so an unreceipted omission is not by itself a counterexample to R2(a).

**R4–R6:** Their remaining uncertainty is appropriate research uncertainty. R5 now separates termination, soundness and completeness for a frozen class; R1 and R3 do not prove R5, but the plan does not say they do. R6 now has an observer, reporting duty, vocabulary mapping and aggregation rule. Whether its summaries preserve enough information can genuinely fail. The fallbacks and narrowed claims make those failures reportable rather than automatic project successes.

**Core and missed prior art:** I searched for timed dynamic I/O composition and re-fetched [Attie and Lynch](https://arxiv.org/abs/1604.06030) and [David et al. 2013](https://homes.cs.aau.dk/~kgl/SSFT2015/DYN.pdf). The former explicitly makes creation monotonicity conditional; the latter supplies timed dynamic creation and termination. **Could not verify:** I found no primary theorem establishing the precise combined trace-inclusion/hiding result sought here, and no newly missed result that invalidates a step. This search is not proof of novelty. The plan's prior-art screening before new metatheory is necessary and sufficient as a research-plan commitment.

The dormant-pool decision must distinguish a fixed finite benchmark bound from an unbounded family of creation traces. Calling a pool “creation” cannot make it express arbitrarily many simultaneously live units. Conversely, a general transition-system encoding may represent dynamic populations in its state without native component creation; the question then is whether it preserves the required modular proof obligations. Record that distinction in the already scheduled equal-requirements comparison. A one-eligible-candidate outcome is now disclosed and is not itself a defect.

**Round-4 fixes accepted:** Authority ancestry, the independent regulator, state/ministry and subsidiary/team edges, consent-based formation, coordinated graph invariants, pending-deadline handling as a required case, external witness appointment, the assurance-rule safety premise, and the explicit failure consequences all improve the plan. The new independent-authority benchmark makes multi-authority resolution a real exercise. I found no basis to reopen the old pairwise-consistency theorem criticism as if the theorem were still present.

## Verdict and order of work

**Sound enough to start step 0.** Begin with the delayed-fork case, lawful versus unlawful pending conflicts, and overlapping old/new spending rights; freeze those semantics before attempting general proofs. Then freeze R3's actual output bounds and the stochastic model for availability. The existing paper-first gate, reuse ledger, external review and stop rules are sensible. No demonstrated defect requires abandoning the candidate, changing the research objective, or beginning mechanisation earlier.

The narrowing remains honest and worth testing. It studies anchored, mediated governance and compositional evidence, not legitimacy, autonomous human control or a deployed population of ten billion actors. Witnesses and anchors are explicit exceptions to the uniform governance interface; a positive result must preserve that qualification. The strongest potential contribution remains a useful composition law for accountable governed interactions, and the repaired failure tests can still reject it.

CRITICAL: 0  HIGH: 0  MEDIUM: 5  LOW: 0

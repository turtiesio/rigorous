# Adversarial review of "Plan for an integrated foundation", version 2 (fable, round 2)

Target: `/root/mic/reports/Integrated foundation plan.md` (second version, rewritten in one pass after round 1).

Method. I formed a view of the current plan first, then read the two round-1 reviews to check the fixes and avoid repeating settled points. Sources fetched in this round are given by URL. Each finding is labelled **verified** (checked against a fetched source), **could not verify**, or **judgement**.

Overall view. The rewrite applied the round-1 recommendations faithfully in form: the targets now have counterexamples, the paper stage is first, the narrowing is stated openly. The new problems are of three kinds. (1) The threat model the editor wrote contradicts the accountability and deadline targets. (2) Several restated targets are now falsifiable in wording but known or definitional in content, so the round-1 defect has moved, not gone. (3) The tree of role instances, adopted to fix mediation, has been stretched to cover rule authority, which the plan's own benchmark contradicts. No finding is CRITICAL: each has a concrete repair, and the plan's stated purpose (find out whether this candidate composes) survives.

---

## HIGH

### H1. The threat model makes the accountability and deadline targets unsatisfiable as stated

**Text attacked.** "Network delivery is not bounded; clocks are synchronised to within a stated skew." "Obligations are deadlines in a timed model, where missing a deadline is a safety violation." R2: "(a) every violation caused by a covered deviation yields a verdict naming the deviating parties; (b) no party that complied is named". R7: "a departure from the envelope is detected and attributed within a stated time". "A controller may deviate, provided at least one honest witness per unit holds signed commitments to that controller's log". "Any non-controller component may deviate arbitrarily and any set of them may collude."

**Why it is wrong.** Three separate problems, all from the one paragraph that was added without review.

(i) *Omissions have no proof when delivery is unbounded (verified).* The plan's only kind of obligation is a deadline, so the typical violation is an omission: something was not done, or not received, in time. Haeberlen and Kuznetsov show that in an asynchronous system "there is no self-contained proof of an omission fault; when a node is suspected of having omitted a message m, the suspicion can always turn out to be groundless when m eventually arrives", and that it is only "if bounds on communication and processing times are known" that "it is possible to establish in finite time that an omission fault has occurred" (https://haeberlen.cis.upenn.edu/papers/detection-opodis2009.pdf, proof sketch of the omission lower bound and the concluding section). Synchronised clocks do not help: the adjudicator cannot tell a late sender from a slow network. So for deadline violations R2(a) and R2(b) cannot both hold. The plan's escape is R2(c), "external failure", but then any party that misses a deadline can be classified as a network victim, and accountability for obligations collapses into the allocation rule. R7's "detected and attributed within a stated time" is false outright under unbounded delivery whenever the evidence travels by message.

(ii) *Holding commitments is not auditing (verified, in part from secondary descriptions).* PeerReview's guarantee is that faults "whose effects are observed by a correct node" are "eventually" detected, and it requires that a correct node's actions are deterministic and that each node is periodically checked by a correct witness that replays the log against a reference implementation (https://www.mpi-sws.org/tr/2007-003.pdf; summary at https://researchportal.ip-paris.fr/en/publications/peerreview-practical-accountability-for-distributed-systems/). A witness that only "holds signed commitments" gets tamper-evidence of what the controller chose to log. It detects neither a wrong derivation (nobody replays) nor an omitted entry (variant 8) unless an honest counterparty holds a signed receipt for that entry and brings it forward. The plan names no auditor, no receipt protocol and no reference implementation to replay against.

(iii) *Collusion scope (judgement).* With "any set" of non-controller components colluding and the controller deviating, a unit can be wholly faulty apart from a passive witness. The controller "owns the effect of every protected action" and keeps the balances, so it can emit an effect to a colluding requester and log nothing. No honest party observes it. R2(a) says "every violation caused by a covered deviation yields a verdict"; that is false for this case, and "covered" is not defined so as to exclude it.

(iv) *Useful behaviour (judgement).* "A compliant purchase completes before its deadline" cannot be a required outcome when delivery is unbounded and controllers "refuse when unsure". A correct controller may refuse everything for the whole run. So the rule "a controller that rejects everything fails" cannot be evaluated under the stated model.

**Change recommended.** Replace the network clause with a partial-synchrony assumption stated per link class: a bound on delivery between a controller and its witness and adjudicator, and no bound for vendor and cross-unit links. Then:
- Define discharge of an obligation as an event at the *obligee's* controller (a receipt logged before the deadline), so that a missed deadline is locally observable; state openly that lateness caused by the unbounded link is strict liability on the obligor under R2(c), not attributed deviation.
- Restrict R2(a) to "violations whose effects are observed by at least one honest party that can reach the witness", as PeerReview does, and say so in the target.
- Replace "holds signed commitments" by a named audit duty: counterparties receive signed receipts, the witness replays the controller (which must then be deterministic) at a stated period, and the detection time in R7 and in the assurance record is that period plus the delivery bound.
- State required outcomes as conditional on the delivery bound holding, and count refusals when it does not.

### H2. R1 as worded is already answered in the cited paper, and is trivial once rules can be amended

**Text attacked.** "R1 | Authority safety analysis: for the policy fragment with attenuating delegation, constitution and declared amplification, 'can principal p ever obtain right r' is decidable, with a complexity bound | ... | Decidability for the RT trust-management family under restricted policy change (Li, Mitchell & Winsborough)".

**Why it is weak.**

(i) *Verified.* The question "can p ever obtain r" is the *simple safety* query of the cited paper. For the language with linked roles and intersection it is already polynomial: "Simple safety, simple availability, and bounded safety queries can all be answered in time polynomial in the size of P". Only containment is hard (coNP-complete with intersection, in coNEXP with linking and intersection). https://www.cs.purdue.edu/homes/ninghui/papers/analysis_oakland03.pdf (section 1 and Theorems 4.3 to 4.5; text extracted from the PDF). Delegation is RT's simple and linked inclusion; "declared amplification" (officer approval combined with the agent's budget right) is RT's intersection. So unless "constitution" adds something RT lacks, R1 restates a 2003 result and cannot "go beyond" it. The plan does not say what constitution adds formally. If it is "a role defined by a vote of a named member role", it is a threshold structure of the RT family.

(ii) *Verified.* The cited decidability depends on a *restriction rule*: a declared set of growth-restricted and shrink-restricted roles, standing for the principals trusted not to change policy ("the restriction rule R represents the intuitive notion that certain principals are trusted", same source, section 4.6 area). R1 names no such rule. The plan also lets every unit amend its own rules, including the amendment rule (step 3). Under those two facts together, the honest answer to "can p ever obtain r" is "yes, after an amendment" for every p and r below the anchor, and the decision procedure is trivial. The plan's candidate counterexample (unrestricted constitution reduces to the access-matrix problem) is real, but it is not the nearest failure; triviality is.

(iii) *Judgement, with a verified pointer.* The interesting rights in the benchmark are not role memberships. They are quantities (a budget right that is spent, a cap, a threshold over a history of purchases, variant 7) and they are revoked (variant 4). Reachability for administrative policies with revocation and negative preconditions is already much harder than RT simple safety: user-role reachability for ARBAC is PSPACE-complete (Sasturkar, Yang, Stoller and Ramakrishnan, https://www3.cs.stonybrook.edu/~stoller/arbac-analysis-TCS.html). This prior art is missing and is closer to the plan's fragment than RT is.

**Change recommended.** Restate R1 as: "for the fragment F (grammar given) and a restriction rule naming which units' rules and amendment procedures are fixed, simple safety and containment are decidable with bound B, where F includes at least one feature outside RT[linking, intersection]: consumable quantities, revocation with negative preconditions, or creation of units". Name the feature. Add the amendment action itself to the transition relation of the analysis, so that "p can obtain r only by an amendment passing procedure P" is a possible answer. Add ARBAC analysis to the prior-work column. If no feature outside RT is intended, move R1 to "lemmas to reproduce".

### H3. "The containing unit is unique" contradicts the plan's own separation of rule authority from containment, and its own benchmark

**Text attacked.** "Four relations are kept separate: membership ..., component containment ..., resource ownership ..., and rule authority (which unit's rules constrain which)." Then, in step 3: "An amendment is valid when it passes the unit's own charter procedure and conforms to the constraints of the unit that contains it; with role instances in a tree, that unit is unique." Benchmark: "a subsidiary (organisation), its parent group (enterprise), a ministry (government), and the state bound by a supranational spending rule (nation)"; variant 10: "Two overlapping units impose conflicting norms on one role-holder."

**Why it is wrong (judgement; it is an internal contradiction and needs no outside source).** Round 1 recommended a tree of role instances so that mediation is well defined. The editor then reused the tree to make rule change well defined. That step does not follow, and the plan's own material shows it:

- Step 0 says rule authority is a separate relation from containment. Step 3 defines amendment validity by containment alone. One of the two must go.
- In the benchmark the parent group is not a component of the ministry, and the state is not a component of the supranational body. The ministry constrains the enterprise by regulation; the parent constrains the subsidiary by ownership; the audit body constrains the officer. The subsidiary is under at least two rule authorities (group policy and procurement law) that are not on one containment path. To get a ten-level chain the plan has silently made the enterprise a sub-unit of the ministry, which is true only of a state-owned enterprise. A subsidiary's amendment that satisfies its container and violates procurement law is "valid" under step 3.
- Variant 10 cannot be expressed. A role-holder is a role instance, and a role instance is in exactly one unit, so no two units overlap on it. The conflict in variant 10 is either on the identity, about which the plan proves nothing ("not mediated"), or it is between two rule authorities over one unit, which step 3 says cannot happen. Round 1 raised exactly this ("two parents may give conflicting change rules", fable r1 M1(a)); the rewrite asserted the problem away.
- The same affects R3. Conformance is stated down "adjacent levels", a chain. With several rule authorities over one unit, the unit must conform to each, and the authorities may conflict. R3 has no case for that.

**Change recommended.** Keep the tree for role instances and mediation only. Make rule authority an explicit directed acyclic relation, declared per unit, with possibly several authorities. Amendment validity then reads: passes the charter procedure and conforms to *every* declared authority. Add a stated conflict rule (priority order, or "conflict is recorded and referred to a named adjudicator") and a target or lemma that the conflict rule is total. Re-map the benchmark so that the ministry is a rule authority over the enterprise, not its container, or say that the enterprise is state-owned. Restate variant 10 in terms the model can express, and add the two-authority case to R3.

### H4. Four of the seven restated targets are known or definitional as stated, and the stop rule does not catch that

**Text attacked.** R4, R5, R6, R7 in the table, and "If every statement that survives follows from one module plus controller correctness, report that the integration adds nothing and stop."

**Why it is weak.** Round 1's central finding was that the targets could not fail. The rewrite gave each a counterexample, but for these four the counterexample is either excluded by the hypothesis or certain to occur, and the positive statement is in the literature or follows from the definitions.

- **R4 (judgement, with verified precedent).** "Every change either preserves each commitment ... or produces a recorded violation with a named bearer" is an exhaustive disjunction that the logging controller makes true by construction. The remaining clause, that the new cap holds after the effective time under "acknowledged epochs or expiring leases, refusing when unsure", is the standard lease argument. The candidate counterexample (a controller that has not heard) is excluded by the protocol in the hypothesis. The same design, with in-flight operations and at most two versions live, is published and deployed: schema leases in F1 ensure servers use at most two schema versions at once, and a change waits one lease period (Rae et al., https://research.google/pubs/online-asynchronous-schema-change-in-f1/). "Versioned policy; timed common knowledge" in the prior-work column carries no reference and misses this.
- **R5 (verified).** A sound classifier into "no coordination / local after pre-allocation / needs coordination, with a witness / unknown" exists in pieces. Invariant-confluence checking with an interactive procedure and the undecidability caveat is Whittaker and Hellerstein ("determining the invariant confluence of an object is undecidable in general", https://mwhittaker.github.io/publications/iconfluence_vldbj2020.pdf). Pre-allocation of rights for numeric invariants is the bounded counter (https://arxiv.org/abs/1503.09052). Static analysis that finds conflicting and dependent method pairs for an object with an invariant, and synthesises the coordination, is Hamsaz (Houshmand and Lesani, POPL 2019, https://mohsenlesani.github.io/companion/popl19/POPL19.pdf). The procurement invariants (caps, thresholds, uniqueness of an approval) are the textbook cases of these tools. The stated failure condition, "most benchmark invariants come back unknown", is therefore very unlikely to occur, and no unknown-rate ceiling is fixed. R5 is engineering reuse, not a research target.
- **R6 (judgement).** "Each resource is counted once" follows from the step-0 definition "which unit owns each resource, exactly one". "The nested and flattened readings agree" is associativity of composition, which every candidate core has. And variant 2's "one resource is funded by two units" must be encoded as two transfers into a singly-owned resource, because the definitions forbid joint ownership. So the diamond is defined away before the target is attempted. The identity half of the diamond is "not mediated" and so not covered by any statement.
- **R7 (judgement, with verified pointer).** "If the vendor stays within the envelope the invariants hold" is the definition of an assume-guarantee contract. The second half is run-time monitoring. The "candidate" counterexample, "a departure that is only detectable after the invariant is already lost", is not a candidate: for any pay-then-deliver or deliver-then-pay step it is certain, because fair exchange between two parties is impossible without a trusted third party (Pagnia and Gärtner; discussed and qualified in https://eprint.iacr.org/2023/585.pdf). The plan's benchmark has such a third party (the adjudicating unit of variant 6), but R7 does not use it.

The stop rule tests whether a statement "follows from one module plus controller correctness". It does not test whether the statement is already published, so all four pass it.

**Change recommended.**
- Move R5 and R7 to "lemmas to reproduce", citing the tools above; keep as a reported measure the share of benchmark invariants in each R5 class, with a ceiling on "unknown" fixed in step 0.
- Replace R4 by the part that is not a lease argument: *which* commitments made under version n must be honoured under version n+1, decided by a stated rule, and a proof that the rule and the cap invariant are jointly satisfiable when rights have been pre-allocated (see M9). The candidate counterexample is then a commitment made validly under the old cap whose honouring breaks the new cap.
- Replace R6 by a target that the definitions do not already imply: either allow joint ownership and prove single counting, or state a property of the unmediated identity (for example a declared envelope on what the officer may carry between the team role and the audit role, with detection), which is the actual content of the diamond.
- Add to the stop rule: "or follows from a cited published result by instantiation".
- After these changes the genuinely open targets are R2's composition claim, R3 and the restated R1. Say so; three open targets is an honest and sufficient paper stage.

---

## MEDIUM

### M1. The step-1 stop rule quantifies over statements that are not fixed in advance

**Text attacked.** "If R2 has no true, non-definitional statement on a three-unit chain, publish that as a negative result for this candidate and stop."

**Why it is weak (judgement).** The rule fires only if *no* acceptable statement exists. The authors choose the statement after seeing which proofs go through, so they can weaken R2 until something true remains. "Non-definitional" has no judge and no criterion. Also, step 1 "needs only the definitions and the benchmark", but R2's composition claim needs a composition operator, a log model and a verdict function, and the core is chosen only in step 2.

**Change recommended.** Fix the formal statements of R2(a), (b), (c) and the composition claim, the causal definition of deviation, and the verdict type (sets of minimally sufficient sets of parties, as in Künnemann et al.) in step 0, in the default core, before any proof attempt. The stop rule then reads: "if the fixed statement is false on the three-unit chain and the weakest repair needs inner logs, stop". Name the external reviewers as the judges of "definitional", with the criterion "the statement is refuted by at least one benchmark variant when one named hypothesis is dropped".

### M2. The composite assurance rule ("weakest record") is wrong

**Text attacked.** "A composite reports the weakest record among the parts a property depends on."

**Why it is wrong (judgement).** A record has six fields (mechanisms, assumptions, observability, detection time, intervention, unprotected remainder). They are not totally ordered, so "weakest" is undefined when one part has a longer detection time and another a larger assumption set. Where an order exists, the minimum is still the wrong combinator: assumptions of the parts conjoin (the composite assumes all of them, which is weaker than any one part's record); detection times along a chain add; unprotected remainders take the union. Reporting the weakest part overstates the composite. Every "controlled" claim the plan outputs depends on this rule.

**Change recommended.** Define the record as a product of ordered fields with a stated combinator per field: union of assumptions and of unprotected remainders, intersection of observables, sum of detection times along a dependency path and maximum across independent paths, and the least capable mechanism in the order block > certificate > envelope > log-and-adjudicate. State as a lemma that the composite record is no stronger than any part's.

### M3. Atomic controllers and sharded controllers contradict each other as defined

**Text attacked.** "only the controller emits the effect. A controller is either atomic or itself a unit with an atomic controller, so the regress ends. At the upper levels a controller is a logical service that may be sharded".

**Why it is weak (judgement).** A sharded controller is a composite, hence a unit, hence has an atomic controller that owns every protected effect of the shard set. Either that atomic controller sees every decision, and is the bottleneck sharding was to remove, or the shards emit effects without it, and "only the controller emits the effect" is false. The plan defers the question to step 5, but this is a definition problem, not a measurement problem. A cross-shard effect (budget in one shard, approval in another) is exactly R5's "needs an owner or committee" class.

**Change recommended.** Define a sharded controller as a set of atomic controllers, each owning a disjoint set of resources and rules, with every protected action assigned to exactly one shard by a static function, and cross-shard invariants handled only by pre-allocated rights. State that an action needing two shards is "coordinated" and counts toward the ceiling in step 5. Drop "itself a unit with an atomic controller" or restrict it to the rule-change path.

### M4. Self-applicable amendment has no target and no benchmark variant, and "well-founded" is vacuous

**Text attacked.** "A charter may contain one amendment rule that applies to itself, with each amendment checked by the rules in force at the previous version, which keeps the versions well-founded." And: "'Self-reflecting' is delivered as rule change checked by the rules in force".

**Why it is weak (judgement).** A sequence of versions indexed by the natural numbers is well-founded whatever the rules say. The statement constrains nothing. The substantive question, known since Suber's work on self-amendment and the game Nomic, is which properties survive a self-applied amendment; an amendment rule can validly replace itself by "anything goes", limited only by the container's constraints. No target among R1 to R7 concerns self-amendment (R4 is a cap change by a superior), and no variant amends an amendment rule. So the one property of the goal that the plan says it delivers in a narrow form has no test at all. This is below HIGH only because the narrowing is stated openly.

**Change recommended.** Add a variant: the subsidiary amends its own amendment rule, validly, while a commitment is outstanding; then attempts a second amendment valid under the new rule and invalid under the old. Add a target: a named set of entrenched invariants (single ownership, log continuity, conformance to declared rule authorities) is preserved by every valid amendment sequence, with the candidate counterexample an amendment that removes the logging duty. Replace "keeps the versions well-founded" by that statement.

### M5. The bake-off has no decision rule, uses a measure that is not available yet, and is scored on the default's home ground

**Text attacked.** "Each candidate encodes benchmark variants 1 to 3, proves closure and substitutivity, and carries the paper proof of R2 on a chain. They are compared on the 'smaller' measure and on which hypotheses each needs." And step 0: "Smaller. Lines of definitions ... measured in one proof assistant".

**Why it is weak (judgement).** (i) "Smaller" is measured in one proof assistant, but mechanisation is step 3 and the proof assistant is an output of step 2, so the measure cannot be applied in step 2. (ii) No rule says which candidate wins when one is smaller and another needs fewer hypotheses. (iii) Closure and trace substitutivity are the native theorems of I/O automata. Bigraphs and law-governed interaction have no refinement preorder of that kind, so they fail by construction; that tests fit to the criteria, not fitness for the goal. (iv) Variants 1 to 3 include creation but no time-dependent behaviour, so the timed half of the default's "two explicit hypotheses" is never exercised in the comparison. (v) Six candidates, each with an advocate and a paper proof of R2, is a large cost placed before the first mechanised line.

**Change recommended.** Cut to three candidates: the default, one interface or contract theory, and the governance baseline. State criteria that are neutral: can it express variants 2, 3 and 9; does it support the fixed R2 statement; count of hypotheses. Fix a lexical decision rule (expressiveness, then hypotheses, then size on paper in a common notation). Add variant 9 so that time is exercised. Measure "smaller" in the proof assistant only in step 3 and only for the winner and the runner-up.

### M6. The separability test is failed in advance by the targets as stated, or is undefined

**Text attacked.** "It fails if the proof of any structure's target imports definitions from more than one other structure."

**Why it is weak (judgement).** No mapping from R1 to R7 onto the five structures is given. On any natural mapping, R2 (accountability under delegation) uses authority, commitments and the log; R4 uses rule change, commitments, time and a resource cap. Both import from two or more other structures before any proof is written. So the hypothesis is already false, or the phrase "a structure's target" refers to something not in the plan.

**Change recommended.** Give each structure its own local lemma (those in "lemmas to reproduce") and apply the import test to those. Declare R1 to R7 as integration targets, and for them report the dependency graph without a pass or fail.

### M7. R3 is likely to collapse to transitivity of simulation, or to be false, with nothing between

**Text attacked.** "pairwise design compliance plus leaf enforcement implies end-to-end compliance, under conditions on the mappings".

**Why it is weak (judgement).** If a "counts as" mapping is a function on events and design compliance is trace inclusion through that function, the result is composition of homomorphisms and is immediate. If mappings are context-dependent relations, as in the institutional literature, the candidate counterexample applies and the result is false. The plan gives no intermediate class of mappings that would make the result both true and non-trivial. The transitivity of "counts as" is itself a debated matter in that literature, which the plan does not cite. Astra r1 item 7 raised the collapse; the rewrite changed the vocabulary and kept the shape.

**Could not verify.** The plan calls King et al. "two-level compliance checking". The paper's title and abstract say "multi-level" and speak of "increasingly vague and abstract" higher levels (https://researchportal.bath.ac.uk/en/publications/automated-multi-level-governance-compliance-checking/). I could not obtain the full text to check whether more than two levels are composed. If they are, R3's novelty is narrower still. Fetch the paper before the paper stage.

**Change recommended.** Fix the class of mappings in step 0 (for example: context-dependent, with the context read from the lower level's state), state the condition conjectured to be sufficient, and commit to both the counterexample without the condition and the proof with it. Add the several-authorities case from H3.

### M8. Prior art on compositional blame in component systems is missing and bears directly on R2

**Text attacked.** R2's prior-work column, and the composition claim "a chain verdict can be computed from per-unit verdicts without reading inner logs".

**Why it matters (verified existence; contents from abstracts only).** Gössler and Le Métayer give a trace-based framework for blaming components against per-component specifications from one violating trace, with necessary and sufficient causality defined by counterfactuals over component traces: "A General Trace-Based Framework of Logical Causality" (FACS 2013, https://inria.hal.science/hal-00873665) and "A general framework for blaming in component-based systems" (Science of Computer Programming 2015). Caltais, Leue and Mousavi study when causality does and does not decompose over composition in labelled transition systems (https://arxiv.org/abs/1608.07880). This is the setting of R2 (components with contracts, a system-level violation, blame from traces), and the second paper is about the composition claim itself. The plan cites protocol accountability and contract blame and misses the component-causality line.

**Change recommended.** Add both to R2's prior-work column and read them before attempting R2. State R2's composition claim as a property of *interface-level* verdicts ("unit U violated its guarantee at its boundary at time t, its assumptions having held until t"), since that is the form in which decomposition has a chance, and take the joint-causation counterexample from that literature.

### M9. The cap-change bound ignores pre-allocated rights, and lease cost is not in the load bound

**Text attacked.** "A cap change is one operation including its propagation. Acceptable cost is proportional to the affected subtree, with a stated maximum latency". R5: "local after pre-allocation of rights".

**Why it is weak (judgement).** R5 makes spending local by handing rights down in advance. Lowering a cap then requires recalling rights already handed down, or waiting for them to expire. Recall needs a reply from every holder, which unbounded delivery does not give. Expiry makes the latency the sum of the lease periods down the depth of the tree, and makes renewal a standing load on every upper controller proportional to its subtree divided by the lease period, whether or not any rule changes. Neither cost appears in the three bounds. For a national cap the affected subtree is the whole population, so "proportional to the affected subtree" permits a cost of N per change with no further test.

**Change recommended.** Add lease renewal to the load bound. State the cap-change latency as a function of depth and lease period. Add to step 1's paper argument the case "cap lowered below the sum of rights already pre-allocated" and say what happens (the old cap holds until expiry, or a recorded violation).

### M10. The step-0 exit test can pass when the definitions fail

**Text attacked.** "The step passes if they agree on at least four judgements in five".

**Why it is weak (judgement).** The test measures agreement, not discrimination. Two reviewers who agree that every system satisfies every definition, or that none does, pass it. Raw agreement of 80 percent is also easy when one answer dominates.

**Change recommended.** Use chance-corrected agreement (Cohen's kappa at a stated level). Add a discrimination condition: for each definition at least two of the ten systems are judged to satisfy it and at least two not to. Add a condition that a known-ungoverned system (for example an open message bus) is rejected.

---

## LOW

### L1. The time hypothesis is probably attached to the wrong theorem

"Substitutivity therefore carries two explicit hypotheses". **Could not verify from a fetched primary text; from memory.** In the timed I/O automata monograph (Kaynar, Lynch, Segala and Vaandrager, https://dspace.mit.edu/handle/1721.1/30403), substitutivity for timed trace inclusion holds for compatible automata without a non-blocking hypothesis. The receptiveness and progressive-strategy conditions are needed so that non-blocking of time is preserved by composition, and so that an implementation cannot satisfy a deadline specification vacuously by stopping time. The plan should attach the time condition to non-vacuity of the deadline targets, not to substitutivity. The plan marks this citation as unfetched; fetch it. I found no published combination of timed and dynamic I/O automata, which agrees with the plan's statement.

### L2. "Depth logarithmic in N" and "fan-out is not assumed bounded" do not fit together

With ten named levels and unbounded fan-out, depth is a constant and the per-decision bound is trivially met; the load bound carries all the content. State depth as fixed by the benchmark, or bound fan-out.

### L3. Benchmark runs with language-model agents have no repetition count

"Run all ten variants with language-model agents ... and check the required outcomes." Outcomes with such agents vary between runs. State the number of runs per variant and the pass rate required.

### L4. One held-out variant is named in advance

"One variant is held out" from a list of ten that the authors wrote and can read. It is held out from the bake-off, not from the designers. Have the external reviewers write the held-out variant after the core is chosen.

---

## Fixes from round 1 checked and found faithful

- Role instances in a tree for mediation (fable r1 H1): applied as recommended. The over-extension to rule authority is H3 above.
- Timed plus dynamic core with a kill criterion (fable r1 H3): applied. Attie and Lynch's abstract does say creation is monotonic "subject to some technical conditions" (verified, https://arxiv.org/abs/1604.06030).
- Whittaker and Hellerstein on undecidability (verified from the PDF); bounded counters at arXiv 1503.09052 (verified); Li, Mitchell and Winsborough's link and subject (verified).
- Non-goals for confidentiality, liveness, incentives, Sybil resistance; the section on what a positive result would establish; paper stage first; three scale bounds; executable conformance "tested, not proved": all applied, and stated honestly.

## Gap to the goal

The narrowing is stated honestly, and I do not hold it against the plan. My judgement is that the narrowed claim is still worth testing, on one condition. After H4, the content not already in the literature is R2's composition claim, R3 across vocabularies with several authorities, a properly restated R1, and the entrenchment target proposed in M4. If the plan keeps R4 to R7 as they stand, a "positive result" would mostly re-derive known results inside one notation, and the fourth failure mode the plan names ("an accounting system for trust") would be the outcome by construction. The plan has not defined the problem away, but it is closer to doing so than its wording suggests.

## Verdict

No CRITICAL defect. The plan is a credible route once four things change: the network and witness assumptions are made strong enough for the accountability and deadline targets to be satisfiable (H1); R1 gets a restriction rule and a feature outside RT (H2); rule authority is separated from the containment tree, as step 0 already says it is (H3); and the known or definitional targets are demoted so that the stop rule bites on the three or four that are open (H4, M1). All four changes belong in step 0 and step 1, before any mechanisation, and none needs new research to make. The order of work is otherwise sound. The one ordering change I recommend is to fix the formal statement of R2 in the default core in step 0, since the paper stage cannot be judged without it.

CRITICAL: 0  HIGH: 4  MEDIUM: 10  LOW: 4

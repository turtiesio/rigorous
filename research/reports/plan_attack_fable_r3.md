# Adversarial review of "Plan for an integrated foundation", version 3 (fable, round 3)

**Method.** I read version 3 and formed a view before reading the earlier reviews. I then read both round-2 reviews to check which of my points were new and whether the fixes were faithful. Sources were fetched where stated; "from memory" marks what I did not fetch. Each finding is labelled *verified wrong*, *could not verify* or *judgement*.

**Overall.** No CRITICAL defect. The narrowed claim is honest and worth testing. Two HIGH findings sit in text the editor wrote in this pass, and both come from round-2 reviewer suggestions (one of them mine) copied in without checking how they interact with the rest. Both must be fixed inside step 0, before the statements are frozen. The remaining findings are MEDIUM and are step-0 repairs.

---

## HIGH

### H1. "At least one honest witness" cannot support countersigned receipts, the suspicion verdict, or R2's summaries

**Text attacked.** "An effect is usable by any other party only with a receipt countersigned by a witness of the unit. Each unit has at least one honest witness with an active duty". R2: "the verdict for a chain can be computed from per-unit summaries ... without reading inner logs". Stop rule: "If the frozen statement of R2 is false on a three-unit chain and the weakest repair needs inner logs ... stop."

**Why it is wrong.** The plan takes PeerReview's assumption (one correct witness, identity unknown) and attaches three things that PeerReview does not build on it.

1. *Which witness countersigns (judgement).* If any one witness may countersign, a deviating controller uses a colluding witness, and the countersignature adds nothing. If all must countersign, one faulty witness blocks every effect of the unit, and the unit fails "useful behaviour" inside the envelope. A threshold that guarantees both an honest signer and progress needs f+1 signatures from 2f+1 witnesses, which is a stronger assumption than the one stated.
2. *PeerReview's detection does not come from countersigning (verified).* It comes from receivers forwarding authenticators to the whole witness set, and from witnesses forwarding them to other witness sets: "If a node i receives authenticators from another node j, it must eventually forward these authenticators to the witness set w(j)" (section 4.6). The plan gives the recipient no such duty and says only "able to contact a witness". Without it, the honest witness may never see the receipt that contradicts the log in variant 8.
3. *A summary is a claim by a witness, and only one witness is honest (judgement, verified basis).* PeerReview evidence is transferable because any correct node re-checks it against the log segment and authenticators (sections 4.7 and 4.9). A summary "U broke its guarantee at t" from an unknown-honesty witness is not self-certifying. The chain adjudicator will receive conflicting summaries from the honest and the faulty witnesses of the same unit. It can resolve them only by replaying inner logs, which the composition claim forbids. So R2(b) fails on any chain with one lying witness, the "weakest repair needs inner logs", and stop rule 1 fires. The project would end on an artefact of the witness assumption, not on a fact about compositional blame.
4. *Suspicion (judgement).* A faulty witness can declare a compliant controller unresponsive. The plan does not say whether a suspicion verdict "names" a party for R2(b), or what follows from one.
5. *Appointment (verified basis).* PeerReview assigns witnesses by a system-wide function w (consistent hashing in its deployments); a node does not choose its own. The plan does not say who appoints a unit's witnesses. If the unit's charter does, a deviating unit appoints colluders and R5's "witness duty" survives while its content is gone.

**Evidence.** PeerReview technical report, the numbered assumptions in section 4.3 (determinism, eventual delivery, a witness function w with at least one correct node in each node's set together with the node), sections 4.6 to 4.9, and section 5.3 on configuring witnesses: https://haeberlen.cis.upenn.edu/papers/peerreview-tr.pdf. Threshold witness cosigning, where the verifier sets how many cosignatures it requires: Syta et al., https://arxiv.org/abs/1503.08768. Receipts plus replay giving individual blame, with governance transactions in the same ledger: IA-CCF, https://www.usenix.org/conference/nsdi22/presentation/shamis.

**Fidelity.** This is my own round-2 recommendation (H1, "counterparties receive signed receipts, the witness replays") merged with the other reviewer's "witnessed receipt", without the quorum and forwarding rules either needs.

**Change recommended.** Choose one and freeze it in step 0.
- (a) Witness sets of 2f+1 with at most f faulty. Receipts and summaries carry f+1 signatures. Suspicion needs f+1 witnesses.
- (b) Keep one honest witness. Drop countersigning. Give recipients the PeerReview forwarding duty. Make every summary carry checkable evidence, and restate R2's composition claim as "from summaries plus evidence that is verified per unit, without the chain adjudicator joining inner logs across units".

In either case, witnesses are appointed by a rule authority or the anchor, never by the unit's own charter. Add to the first-day check of variant 8 a colluding witness and a lying witness. Add IA-CCF and CoSi to R2's prior-work column.

### H2. The conflict rule contradicts R3's conclusion, R5's entrenched invariant and the definition of a valid amendment

**Text attacked.** Step 3: "An amendment is valid when it ... conforms to every declared rule authority over the unit. Where two authorities conflict, a declared priority order decides; where none is declared, the conflict is recorded and referred to a named adjudicator." R3: "the leaf's behaviour complies with every authority above it". R5 entrenches "conformance to every declared authority". Variant 10: "Group policy and procurement law conflict for the subsidiary".

**Why it is wrong (judgement; internal contradiction).** If two authorities really conflict, no rule set conforms to both. Then:
- by the validity definition, no amendment is valid, and the current rules do not conform either;
- if priority decides, the unit violates the lower-priority authority, so R3's conclusion is false and R5's invariant is broken by the rule meant to protect it;
- variant 10 is the case built to exercise this, so R3 and R5 fail on the benchmark by construction, or the conflict rule is never used.

Three further gaps:
- "Declared" by whom? With several roots in the authority relation (the supranational body, the audit body), each authority can declare itself prior. That is the choice-of-law problem, and it needs a common superior or the anchor.
- The plan does not say what the unit may do while a referral is pending.
- "A lemma shows this rule always yields an outcome" is true by case split, since referral is an outcome. It is definitional under the plan's own stop-rule test.

**Evidence.** King et al. define compliance of a lower institution with a higher one and report non-compliance; the abstract gives no priority resolution among higher institutions (https://researchportal.bath.ac.uk/en/publications/automated-multi-level-governance-compliance-checking/; *could not verify* the full text, the Cardiff copy returned 403). Priority among conflicting policies is standard practice: XACML combining algorithms (https://docs.oasis-open.org/xacml/3.0/xacml-3.0-core-spec-os-en.html, from memory). The survey itself cites prioritised defaults for norms (survey, the paragraph on normative logic).

**Fidelity.** Round 2 (fable H3, astra 7) asked for "conforms to every authority" plus a conflict rule. The editor added both and did not reconcile them with R3 and R5.

**Change recommended.**
- Define the *effective norm set* of a unit as its authorities' norms resolved by a priority relation fixed by the anchor or by a common superior authority.
- Restate amendment validity, R3's conclusion and R5's invariant as conformance to the effective norm set.
- State the cost openly: a lower-priority authority's norm can be lawfully overridden, and the override is recorded.
- For an undeclared conflict, freeze the interim rule: the unit refuses the affected actions, and this counts as unavailability.
- Replace the totality lemma by a target that can fail: the effective norm set is consistent whenever the priority relation is a strict partial order that is total on each conflicting pair. The refuting test is a priority cycle across three authorities.

---

## MEDIUM

### M1. An unmodified vendor cannot check receipts, so the benchmark's main external effect is outside coverage

**Text attacked.** "An effect is usable by any other party only with a receipt"; step 4, "an unmodified vendor service"; "Links between a controller, its witnesses and its adjudicator deliver within a stated bound. Vendor links and links between units have no delivery bound."

**Why (judgement).** An unmodified vendor accepts an order with no receipt and cannot contact a witness. A deviating controller's purchase is then an effect that reaches an honest party who is not "able to contact a witness", so it is uncovered. Separately, the adjudicator of variant 6 is another unit, so its link falls in both the bounded class and the unbounded class.

**Change.** Add a vendor gateway that is a unit with a controller and witnesses, and say that the vendor's own behaviour behind it is environment. State which link class wins for an adjudicator outside the unit.

### M2. R1's fragment sits on a known trichotomy, and "a stated bound" does not constrain the result

**Text attacked.** "For a fragment that includes consumable quantities and revocation ... 'can p ever obtain r' and 'does role A always contain role B' are decidable with a stated bound."

**Why (theorems verified in part, mapping is judgement).**
- Bounded quantities and a fixed set of principals give a finite-state system. Decidability is then immediate and adds nothing.
- Unbounded quantities with delegation behave like Petri-net counters. Claw-back of "whatever remains" (variant 4) is a reset or transfer arc. For reset nets coverability is decidable and boundedness is undecidable (Dufourd, Finkel and Schnoebelen, https://researchr.org/publication/DufourdFS98). Reachability is undecidable and coverability is Ackermann-hard (from memory: Araki and Kasami 1976; Schnoebelen 2010). "Can p obtain at least k" is coverability. "A always contains B" is an inclusion question, and inclusion of reachability sets is undecidable for plain nets (Hack 1976, from memory).
- So "decidable with a stated bound" is satisfied by a non-primitive-recursive bound on the first question, and the second question is likely undecidable once quantities are unbounded.
- The cited baseline already runs from polynomial to coNEXP for containment (verified: https://www.cs.purdue.edu/homes/ninghui/papers/analysis_oakland03.pdf).

**Missed prior art.** Consumable authority in a linear access-control logic: Bowers et al., https://www.mpi-sws.org/~dg/papers/NDSS07Final.pdf.

**Change.** In step 0, fix whether quantities are bounded, whether principals can be created during the analysis, and the semantics of revocation. State a target complexity class (for example PSPACE for safety) that counts as success, and count a non-elementary bound as failure.

### M3. R4 cannot fail, R6 is nearly definitional, and the "definitional" criterion does not catch either

**Text attacked.** R4: "rejected, explicitly grandfathered, or recorded as an invariant that does not hold". R6: "detected by joining receipts ... keyed by identity". Stop rule: "a non-definitional statement is refuted by at least one benchmark variant when one named hypothesis is dropped."

**Why (judgement).**
- R4's third outcome is always available, so no trace has "no outcome the rule permits". This is round 1's defect again.
- R6 holds whenever both receipts carry the identity and a common purchase key, and fails otherwise. Its real content is whether two units with different vocabularies agree on "the same purchase", which is R3's problem; split purchases (variant 7) are the hard case.
- No declared authority owns the separation-of-duty norm or may demand the audit body's receipts. The ministry has no edge to the audit body.
- The criterion tests only that a hypothesis is used. "If the controller blocks x then x does not occur" passes it.

**Change.**
- R4: delete the third outcome. Require that every change is either rejected or installed with an explicit, bounded grandfather set, and that the new invariant holds for everything outside that set after the lease bound.
- R6: require detection when the two units name the purchase differently and the purchase is split. Name the authority that owns the norm and performs the join.
- Criterion: add "and the statement is not an instance of a theorem in the reuse list or of a step-0 definition", judged by the external reviewers with the instance exhibited.

### M4. R5 is true by definition or false, and its real holes are not in the benchmark

**Text attacked.** "Every valid sequence of amendments ... preserves a named set of entrenched invariants".

**Why (judgement).** If "valid" includes a check against the entrenched set, R5 is a one-line induction. If it does not, variant 11 succeeds. The content is whether the check is decidable for the frozen policy grammar, and whether it survives indirect attack: an amendment that keeps the logging duty and narrows the definition of "protected action", or one that keeps the witness duty and replaces the witnesses (see H1). After H2, "conformance to every declared authority" is also not a state invariant.

**Change.** Restate R5 as: the entrenchment check is decidable for the frozen grammar, and it is complete against amendments that change the meaning of terms used in the invariants. Add both indirect amendments to variant 11.

### M5. The assurance-record derivation has two wrong combinators and two missing side conditions

**Text attacked.** "Observables are intersected. Detection times add along a dependency path and take the maximum across independent paths. The mechanism is the least capable present, in the order: blocked at the controller, certificate checked, envelope assumed, logged and adjudicated." "Interface assumptions that a neighbour's guarantee discharges are removed".

**Why.**
- *Observables (judgement).* Units log their own actions. Intersecting the observables of two units with disjoint action sets gives the empty set for every composite. The rule is conservative and says nothing.
- *Mechanism order (judgement).* "Envelope assumed" is an assumption with no mechanism. It is not more capable than "logged and adjudicated", which detects. Assumed versus enforced is a separate axis from preventive versus detective, and the plan already has an assumptions field for it.
- *Discharge (verified in the survey's own source).* Mutual discharge of assumptions is sound only for safety properties, by induction over time. The plan does not state this. Abadi and Lamport: https://lamport.azurewebsites.net/pubs/abadi-conjoining.pdf.
- *Detection time (judgement).* Any dependency path that crosses a link "between units" has no delivery bound, so the summed detection time is unbounded for every multi-unit composite.
- *Fractal (judgement).* Nothing requires the derived record to be the same for (A with B) with C as for A with (B with C). Without that, records do not nest.

**Fidelity.** The combinators are my round-2 M2 suggestion copied verbatim. They were a sketch and are wrong in the two places above.

**Change.**
- Observables: the union of what is exported at the composite's boundary after hiding.
- Split the mechanism into two fields: prevented, certified or detected; and enforced or assumed.
- Allow discharge only for safety guarantees of the "held until t" form.
- Report detection time as a local bound plus a named delivery term that may be unbounded.
- Add associativity of the derivation as a lemma to the three-unit exercise.

### M6. The bake-off is decided before it starts, and step 1 is done in a core whose basic theorems do not exist

**Text attacked.** "Three concrete encodings: the default; one interface or assume-guarantee contract theory (interface automata); and a governance baseline, law-governed interaction". "Can the candidate express variants 2, 3, 5 and 9 ...? Does it support the frozen statement of R2?" Step 1: "done by hand in the default core"; reuse list: "Closure and substitutivity for the chosen core".

**Why.**
- Interface automata as published are untimed and have no creation (the cited paper; from memory, consistent with the separate later "Timed Interfaces" work). They fail the first criterion on variants 5 and 9.
- Law-governed interaction relies on "policy-independent trusted controllers" (verified from Minsky's abstracts, https://people.cs.rutgers.edu/~minsky/papers/coordination-and-control.pdf). R2 allows controllers to deviate, so it fails the second criterion.
- The lexical rule therefore returns the default without anyone looking. If the candidates are extended to compete, the comparison is of the authors' extensions.
- The frozen R2 summary is written in assume-guarantee vocabulary and proved in the default core first, so the other two are judged on a translated statement inside a time box.
- The plan says it "knows of no published theorem" for timed plus dynamic automata. Then "closure and substitutivity for the chosen core" is new metatheory, not reuse, and "everything else is reuse of published results" is false for it. I also found no such theorem.
- Verified: Attie and Lynch's creation theorems 34 and 35 have six numbered premises, as the plan says, and destruction is only self-destruction by reaching an empty signature (https://arxiv.org/pdf/1604.06030, section 5 and theorems 34 and 35). Variant 5's dissolution by an outside decision must be encoded as a request, and premise 1 forbids self-destruction by an internal action.

**Missed prior art.** A timed specification theory with refinement, conjunction, composition and quotient, and a tool: David et al., https://homes.cs.aau.dk/~adavid/publications/41-hscc10.pdf; extended in https://arxiv.org/abs/2302.04529. It is the fair second candidate.

**Change.**
- Replace interface automata by the timed specification theory.
- Keep law-governed interaction as a baseline for expressiveness only, and say so.
- Move the small creation, deadline and dissolution check of variant 5 in the default core ahead of the paper stage, with a time box.
- Reclassify the combined closure and substitutivity result as new work.
- State the frozen targets at trace level so they do not presuppose the default.

### M7. The step-0 exit test probably cannot pass, by the survey's own finding

**Text attacked.** "each definition is judged satisfied by at least two systems and unsatisfied by at least two".

**Why (judgement).** The survey's headline is that nobody has composed these structures. "Fractal" requires every unit to expose authority, resources, commitments and norms, a log and a rule-change procedure. "Controlled" requires six-field assurance records. It is unlikely that two of the ten existing systems satisfy either. The authors would then weaken the definitions until two systems pass. Also, ten items and two raters give a chance-corrected agreement figure with a very wide interval.

**Change.** Apply the test clause by clause (each of the five interface elements, each record field). For the composite definitions, add two small constructed positive examples to the ten. Raise the number of judged items by counting each clause and system pair.

### M8. Acyclic rule authority excludes mutual oversight, and units as members are undefined

**Text attacked.** "Rule authority is a declared directed acyclic relation"; "A principal is an identity plus a set of role instances."

**Why (judgement).** An audit institution audits a ministry whose legislature sets the auditor's mandate. Courts and legislatures constrain each other. These are cycles among units that do not contain one another, and they are how the top of a real hierarchy is governed. Acyclicity came from King et al.'s setting through round 2 (my H3). It is a narrowing that the section on what a positive result would not establish does not list. Separately, the benchmark has organisations acting as parties (the ministry funds the group), and the plan does not say whether a unit can be an identity holding a role.

**Change.** Either label authority edges with a subject-matter scope and require acyclicity per scope, with a variant in which the audit body audits the ministry that sets its mandate; or add "no mutual authority between units" to the list of what is not established. Say whether a unit can be a principal.

### M9. Synchronised clocks are an unlisted trust anchor, and the lease and cost bounds follow the wrong relation

**Text attacked.** "Clocks are synchronised to within a stated skew"; "proved from the lease period and clock skew down the depth of the tree, with no assumption on delivery"; "Per-decision cost. Proportional to depth."

**Why.**
- *Verified.* Achievable synchronisation is bounded below by the uncertainty in message delay: Lundelius and Lynch, u(1 - 1/n) (https://groups.csail.mit.edu/tds/papers/Lynch/podc84-DLS.pdf and citing sources). Across links with no delivery bound, no protocol gives a stated skew. The skew must come from an external time source, which is a trust anchor that the plan's list omits. "No assumption on delivery" is true only with that anchor.
- *Verified.* PeerReview replays timeouts from logged times ("we record the time of each event in the log", section 6.1). A deviating controller's logged clock readings can be checked by the witness only to within the bounded link delay plus skew.
- *Judgement.* Rules flow along rule authority, which is a graph with several parents, not "the tree". Per-decision cost and lease traffic grow with the number of authority ancestors, which depth does not bound.
- The lease precedent is correctly cited (verified: F1 uses schema leases so that servers are at most one version behind, https://research.google/pubs/online-asynchronous-schema-change-in-f1/).

**Change.** Add the time source to the trust anchors. State the lease bound and the per-decision cost over the authority graph: longest authority path, and number of authority ancestors.

### M10. Shards contradict "serialised by the controller" and "rules are controller state"

**Text attacked.** "Each shard owns a disjoint set of resources and rules, and a static function assigns every protected action to exactly one shard." R5: "Competing amendments are serialised by the controller."

**Why (judgement).** In a sharded unit there is no single controller to serialise amendments. An amendment to the amendment rule, or to the assignment function itself, touches every shard. A function that is static cannot follow creation of resources or growth in load unless it is a function of names, and then re-sharding is a rule change. The witness of a sharded unit must replay every shard, or each shard needs its own witness set.

**Change.** Name one shard as owner of the charter and of the assignment function. Count every amendment as a coordinated action installed by lease. State that the assignment is a function of resource name. Give witness sets per shard.

### M11. The stop rules let the project continue on one surviving target, and say nothing for R4 to R6 or variant 7

**Text attacked.** "If, for all of R1 to R3, the statement either fails or does not improve on its stated baseline ... stop."

**Why (judgement).** R1 alone surviving, which is a policy-analysis result with no composition in it, keeps the project alive as an "integrated foundation". "Improves on its baseline" has no judge. Failure of R5, which carries the plan's whole account of "self-reflecting", has no consequence. Variant 7 (split purchases) maps to no target and no reuse item.

**Change.** Continue only if R2 survives and at least one of R1 and R3 improves on its baseline, as judged by the external reviewers. If R5 fails, withdraw the "self-reflecting" claim from the list of what a positive result establishes. Attach variant 7 to R6 as in M3.

### M12. The summary form written into R2 is refuted by a breach that is early and irrelevant

**Text attacked.** "'unit U broke its guarantee at its boundary at time t, its assumptions having held until t'".

**Why (judgement, verified basis).** U1 breaks a guarantee at t1 on an output that the chain property does not depend on. U2 then causes the violation at t2 > t1. U2's assumptions about U1 were already false at t1, so U2 has no summary of this form and U1 is blamed for a violation it did not cause. Counterfactual relevance needs the boundary trace, not only a breach time. The cited compositional result for causality holds only for "non-communicating systems" (verified from the abstract, https://arxiv.org/abs/1608.07880), so it does not cover a delegation chain.

**Change.** Make a summary carry one entry per guarantee, with the assumptions that guarantee depends on and the boundary trace prefix needed for the counterfactual. Put this trace among the first-day checks.

---

## LOW

- **L1.** "Each role instance belongs to exactly one unit, so component containment is a tree" does not follow. It needs "each unit has at most one parent".
- **L2.** "Controller correctness for correct controllers" is a tautology as worded. Presumably complete mediation is meant.
- **L3.** The pass rate with language-model agents measures the agents' competence at the interface as well as the foundation. Report controller-side outcomes (wrongly blocked, wrongly allowed) separately from task completion.
- **L4.** *Could not verify.* The link given for "conformance hierarchies in law-governed interaction" (https://arxiv.org/abs/1403.4321) is Minsky's "Dependable Management of Untrusted Distributed Systems". Its abstract does not mention conformance hierarchies. Check the citation in the source reading.
- **L5.** Log and commitment retention periods are not stated. PeerReview truncates logs, and a suspected node may become trusted again after truncation (section 5.2).

---

## Fixes from round 2 checked

Faithful: link classes; obligations discharged at the obligee; three verdict kinds; the four rule-change quantities; the R1 restatement; King et al. described as multi-level; R3's exclusion of plain transitivity; variant 11; the six premises of the creation theorem (verified against the paper); the outside-coverage case in variant 8.

Faithful to the suggestion but wrong in the result: the receipt and witness scheme (H1), the conflict rule (H2), the record combinators (M5), the three-candidate bake-off (M6), and the acyclic authority relation (M8).

## Gap to the goal

The narrowing is stated honestly. Three limits are not yet in the list of what a positive result would not show: mutual authority between units (M8), an external time source as an anchor (M9), and, after H2, the fact that a lower-priority authority can be lawfully overridden. With those added, the claim tested is still worth testing: composition of recorded, mediated interaction with blame that composes at interfaces. The failure mode "an accounting system for trust" is the likely outcome if H1 is fixed by adding more trusted witnesses, and the plan gives no criterion for telling which outcome it got. A criterion would be: list, for each surviving theorem, the anchors it depends on, and report any theorem that depends on all of them.

## Verdict

The plan is not ready to freeze statements, and it is ready to start step 0, provided step 0 begins with H1 and H2. H1 matters most: as written, stop rule 1 would end the project on an artefact of the one-honest-witness assumption. H2 makes R3 and R5 false on variant 10 by construction. Everything else is a step-0 repair. No target is known impossible. R2's composition claim and R3 remain real open questions with real refuting tests once the summary schema (M12) and the effective norm set (H2) are fixed.

CRITICAL: 0  HIGH: 2  MEDIUM: 12  LOW: 5

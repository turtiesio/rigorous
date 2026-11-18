# Adversarial review of "Plan for an integrated foundation", version 4 (fable, round 4)

**Method.** I read version 4 and formed a view before reading the earlier reviews. I then read both round-3 reviews and the word-level difference between versions 3 and 4, to drop points already fixed and to check that the round-3 fixes were applied faithfully. Each finding is labelled *verified* (a source was fetched or the plan's own text shows it), *could not verify*, or *judgement*. "From memory" marks a source I did not fetch.

**Overall.** No CRITICAL defect. One HIGH defect: the consistency half of the restated R3 is false for the norms the benchmark itself contains, and definitional for the norms where it is true. It sits in text written in this pass. Thirteen MEDIUM defects, most of them interactions between round-3 suggestions that were copied in (three of them my own). The plan is sound enough to start step 0, provided H1 is settled before the effective norm set is frozen.

---

## HIGH

### H1. R3's consistency clause is false for conflicts among three or more norms, and true only by definition for pairs

**Text attacked.** R3: "the effective norm set is consistent whenever the declared priorities form a strict partial order that decides every conflicting pair". Effective norms: "A lower-priority norm is overridden only where it conflicts". Four relations: "the priority among authorities". Variant 10: "once as two prohibitions and once as an obligation against a prohibition".

**Why it is wrong.**

1. *Pairwise consistency does not give joint consistency (elementary logic; judgement on the mapping to the plan).* The set {obliged: a or b; forbidden: a; forbidden: b} has no conflicting pair and cannot be jointly followed. A priority that "decides every conflicting pair" has nothing to decide here, nothing is overridden, and the effective norm set is inconsistent. The clause is therefore false for any norm language with disjunction, conditions or quantities.
2. *The benchmark already contains such conflicts (verified from the plan's text).* A spending cap from the ministry, plus two commitments from different obligees that each fit under the cap and together exceed it, is a three-way conflict with no conflicting pair. Variants 2, 4 and 9 all have this shape. Variant 10's "two prohibitions" is the same: two prohibitions never conflict with each other, since refraining satisfies both. They conflict only together with a third element, an obligation or a required useful outcome.
3. *The literature defines conflict over sets, not pairs (verified at abstract level).* Hansen treats a conflict as a set of triggered imperatives that cannot all be satisfied, and resolves it by maximal subsets under the priority order: https://link.springer.com/doi/10.1007/s10458-007-9016-7 (full text https://core.ac.uk/download/62912271.pdf; I did not read section 4 in full).
4. *Where conflicts really are pairwise, the clause is definitional (judgement).* If norms are single-action permissions, prohibitions and obligations, every conflict is a pair. Then "override the lower member of every conflicting pair" yields a consistent set by construction. The plan's own stop rule ("not an instance of a step-0 definition") would strike it out.
5. *The priority is over authorities, so it cannot resolve everything it is asked to (judgement).* Two conflicting norms from one authority have no priority. In a three-way conflict a strict partial order need not have a unique lowest member. Whether an overridden norm still overrides a norm below it (reinstatement) is not stated, and the answer changes the effective set.
6. *Consistency is not static (judgement).* A norm set that is consistent at design time becomes unsatisfiable after spending. The clause has no state parameter.

**Fidelity.** Round 3 (fable H2, option a; astra 2) asked for an effective norm set and a consistency statement. The editor added both. Neither reviewer checked the pairwise formulation against quantities, and I did not either.

**Change recommended.**
- Define conflict in step 0 as a minimal set of norms that cannot be jointly satisfied from a given state, with pairs as the special case.
- Define the effective norm set by a published prioritised construction (for example maximal consistent subsets taken in priority order), and state the reinstatement rule.
- Restate the second half of R3 as two claims that can fail: (i) for the frozen grammar, detecting a minimal conflict set is decidable within a frozen bound; (ii) the resolved set is unique, or the number of resolutions is bounded and each is recorded.
- Add to variant 10 a cap with two commitments that jointly exceed it, and two conflicting norms from one authority.
- Add to the prior-work column: Hansen as above; Li et al., conflict detection between interacting legal systems, from the same group as King et al.: https://researchportal.bath.ac.uk/en/publications/legal-conflict-detection-in-interacting-legal-systems/ (abstract only verified); Li's thesis on conflict detection and resolution in cooperating institutions: https://researchportal.bath.ac.uk/en/studentTheses/normative-conflict-detection-and-resolution-in-cooperating-instit/ (title only verified). The second may already contain part of what R3 now claims, so it belongs in the step-0 reading list.

---

## MEDIUM

### M1. The benchmark has no unit with two independent authorities, and its authority relation leaves the leaves and the top unconstrained

**Text attacked.** "Rule authority: a supranational body over the state, the ministry over the group, the subsidiary and the audit of ministry-funded purchases through procurement law, the group over the subsidiary through group policy, and an external audit body over the officer's audit role." "Rule authority is a declared directed acyclic relation that is independent of containment."

**Why (verified from the plan's text).**
- The only unit with two authorities is the subsidiary, and its authorities are the ministry and the group, where the ministry is also over the group. R3's hypothesis is pairwise design compliance on every edge. If group policy complies with procurement law, the two cannot conflict for the subsidiary except through the mappings. So in variant 10 either R3's hypothesis is false and the variant does not test R3, or the conflict is purely a mapping effect. No variant has two authorities with no edge between them, which is the case the conflict rule was written for.
- No edge is declared from the state to the ministry, or from the subsidiary to the team, the officer's team role, the agent or the tool. Since authority is independent of containment, the supranational body's rules reach nothing below the state, and the purchasing agent has an empty effective norm set. The longest authority path in the benchmark is two edges. "Nation" and "government" are names on a dangling edge.
- R3's test speaks of "the root's norm", and its conclusion speaks of the leaf's "effective norm set". The plan does not say whether that set holds the norms of direct authorities only or of all ancestors. If direct only, a leaf violating the root's norm does not refute the conclusion.

**Change.** Add edges state to ministry and subsidiary to team, agent and tool, or say that containment implies a default authority edge. Add one authority over the subsidiary with no edge to or from the ministry (for example a regulator in the group's home jurisdiction). Say that the effective norm set ranges over all authority ancestors.

### M2. Who may set authority edges and priorities contradicts variant 10 and variant 3, makes the anchor a universal sovereign, and needs global coordination

**Text attacked.** "Authority edges, the priority among authorities and the powers of adjudicators can be changed only by the anchor or by an authority above all the units concerned, never by the constrained unit." Variant 10: "three authorities declare priorities that form a cycle". Variant 3: "a vote constitutes an authority no delegator held".

**Why (judgement).**
- If one party (the anchor or a common superior) declares priorities, a cycle is rejected at declaration and variant 10's cycle case is a one-line check. If three authorities each declare, the rule above forbids it. The variant and the rule cannot both stand.
- In the benchmark no authority is above both the ministry and the audit body, or above both the ministry and the group other than the ministry itself. "Above all the units concerned" is undefined for the declaring authority's own priority. In practice every priority is set by the anchor. In a world of overlapping jurisdictions with no common superior, the anchor decides every conflict of laws. This narrowing is not in the list.
- Bottom-up constitution, where founders vote an authority into being over themselves, is a change of authority edges by the constrained units. It is how associations, federations and decentralised organisations form. The rule forbids it, and variant 3 either means this and contradicts the rule, or means a power and should say so.
- Keeping a graph acyclic under concurrent edge insertion is a global invariant: two authorities that each add one edge can close a cycle that neither sees. It is not preserved by merging independent updates, so by the plan's own classification tools (https://mwhittaker.github.io/publications/iconfluence_vldbj2020.pdf) it needs coordination, across units whose links have no delivery bound. The same holds for keeping priorities a strict partial order.

**Change.** State who declares each priority in the benchmark. Replace the cycle case by one that can arise under the rule (two superiors with overlapping scope adding edges concurrently). Put acyclicity of authority and of priority in the coordination classification of step 1 and in the coordinated share of step 5. Add "all rule authority descends from the anchor; consensual formation of an authority over its founders is not modelled" to the narrowings, or model it.

### M3. Signatures from f+1 of 2f+1 witnesses do not make a summary unique or final

**Text attacked.** "A receipt is accepted only with f+1 witness signatures, so at least one signer is honest". "Per-unit summaries carry f+1 signatures." "Links between a controller and its own witnesses deliver within a stated bound." R2: "the verdict for a chain can be computed from signed per-unit summaries".

**Why.**
- *Verified (standard result).* Two quorums of f+1 out of 2f+1 may share only a faulty member. Uniqueness needs either quorums of 2f+1 out of 3f+1, or synchrony plus a protocol that uses it, or trusted hardware: https://arxiv.org/pdf/2102.01970 ("with only 2f+1 replicas there may be only a common Byzantine replica between two quorums of f+1"); https://arxiv.org/pdf/1704.02397 (f below n/2 is reachable under synchrony with signatures, through an explicit protocol).
- *Judgement.* A deviating controller with f colluding witnesses shows honest witness h1 a log without entry e and honest witness h2 a log with it. The receipt for e is signed by h2 and the colluders. The summary "guarantee not broken" is signed by h1 and the colluders. No honest witness lied. Both artefacts are valid under the plan's rules. The plan gives no witness-to-witness protocol and no bound on witness-to-witness links, and the recipient's forwarding duty runs over a link with no delivery bound. So a chain verdict computed from a valid summary can be wrong for an unbounded time, and two valid, different summaries for one unit can exist. R2's refuting test covers "two histories with the same summaries"; it does not cover one history with two summaries.
- *From memory, not verified from the page fetched.* The cited precedent for receipts with individual blame signs receipts with a quorum of N minus f from N = 3f+1 replicas (https://www.usenix.org/conference/nsdi22/presentation/shamis; the abstract confirms only "universally-verifiable receipts" and blame for any number of misbehaving replicas). PeerReview uses no threshold; its evidence is the node's own signed log heads, and detection is eventual.

**Fidelity.** This is option (a) of my round-3 H1, copied in. I did not state the uniqueness condition it needs.

**Change.** Freeze one of: (i) 3f+1 witnesses with 2f+1 signatures; (ii) 2f+1 witnesses, a stated delivery bound between witnesses, and an echo-and-wait rule before signing; (iii) keep f+1, require every summary to carry the controller-signed log head it summarises, declare chain verdicts provisional until a stated challenge period has passed, and say what happens to a verdict that is later contradicted. Add the forked-log case to variant 8 and to R2's refuting test.

### M4. The second bake-off candidate has no creation, so the bake-off has one possible outcome

**Text attacked.** "Two concrete encodings compete: the default, and a timed specification theory with refinement, conjunction, composition and quotient". "Can the candidate express variants 2, 3, 5 and 9". "if one is, it is chosen without a runner-up". Step 0, Smaller: "for the chosen core and the runner-up".

**Why (verified).** The cited theory works over a fixed set of deterministic, input-enabled timed specifications. The text of the paper contains no dynamic creation or destruction of components: https://homes.cs.aau.dk/~adavid/publications/41-hscc10.pdf (searched in full for creation and dynamic structure; the operators are refinement, consistency, conjunction, structural composition and quotient). Variants 3 and 5 create and dissolve a unit. Two readings follow, and each defeats something:
- If "express" requires native creation, the second candidate fails the first question before any work, and the default wins unexamined. That is the defect round 3 found in the three-candidate version.
- If a dormant, pre-declared subsidiary counts as expressing variant 5, the same encoding works in plain timed input/output automata, whose composition theorems are published. Then the dynamic half of the default, and the "new metatheory for the core", are not needed by any benchmark variant, since the benchmark creates one unit.

Also, the size comparison in step 3 needs a runner-up, and the decision rule allows there to be none.

**Fidelity.** The candidate is my round-3 suggestion (M6). I called it "the fair second candidate" without checking it for creation.

**Change.** Rule in step 0 whether a bounded pool of dormant units counts as creation. Add plain timed input/output automata with a bounded pool as a third, cheaper candidate. Add a variant with creation that cannot be bounded in advance (units created by created units), or state that the benchmark does not exercise dynamic creation. Say what the size comparison is when there is no runner-up.

### M5. "Not a published result" is true only for automata; timed calculi with creation and congruence theorems exist

**Text attacked.** "Closure and substitutivity for a model with both time and creation is not a published result as far as the plan knows."

**Why (verified at abstract level).** Berger gives an asynchronous pi-calculus with timers, which has dynamic creation of processes and names, and characterises its reduction congruence by a labelled bisimilarity: https://link.springer.com/doi/10.1007/978-3-540-28644-8_8 and https://martinfriedrichbergernet.netlify.app/concur04. Berger and Yoshida prove a congruence for a timed, distributed, probabilistic calculus and apply it to two-phase commit: https://martinfriedrichbergernet.netlify.app/aplas07. A congruence is a substitutivity result. *Could not verify* that either covers trace inclusion with hiding in the form the plan needs. I found no timed version of dynamic input/output automata, so the narrow claim stands.

**Change.** Reword to "for input/output automata with trace inclusion". Screen the timed calculi with the dynamic networks of timed machines. If one fits, it is a candidate with existing metatheory, and the new-work item shrinks.

### M6. R4 still cannot fail, and "grandfathered" has two opposite meanings

**Text attacked.** R4: "every change is either rejected or installed with an explicit, bounded set of grandfathered commitments, and that after the lease bound the new invariant holds for everything outside that set". Step 5: "The new cap holds only if completed spending and grandfathered commitments fit under it ... otherwise R4 applies."

**Why (judgement; arithmetic is mine).**
- If the frozen rule grandfathers every commitment validly made before the lease bound, the refuting test ("a commitment validly made under the old cap breaks the new cap outside the grandfathered set") can never fire. R4 is then the lease lemma from the reuse list plus a definition. "Bounded" has no stated bound.
- In R4 a grandfathered commitment is exempt from the new invariant. In step 5 it counts against the new cap. Take old cap 100, spent 80, new cap 50. Under R4's reading, past spending is exempt and spending outside the set may reach 50, for a total of 130, above the old cap. Under step 5's reading, 80 does not fit under 50, "R4 applies", and R4 sends the case back to grandfathering. A cap is an aggregate; it does not "hold for" individual commitments.

**Fidelity.** The R4 wording is my round-3 M3 suggestion. The step-5 wording is the other reviewer's finding 5. They were combined unreconciled.

**Change.** Freeze the bound on the grandfathered set as a function of the lease period and the rights outstanding at the change, fixed before the change is known. For aggregate invariants define the residual: new rights are issued against the new cap minus completed and grandfathered spending, floored at zero. Make the refuting test "the grandfathered set exceeds the frozen bound, or total spending exceeds the larger of the old cap and the new cap".

### M7. R6's outcome is fixed by what a receipt contains, and the ministry has no declared route to the receipts

**Text attacked.** "The ministry, as the authority that owns it, detects a breach by joining receipts from the team and the audit body, keyed by identity, when the two units name the purchase differently and the purchase is split". Test: "A violation that the receipts cannot reveal without inner logs or without a shared name for the purchase".

**Why (judgement).** A join on identity shows that one person approved something and audited something. Whether they are the same purchase needs a common referent. If receipts carry one (the gateway's receipt identifier), R6 is true by the receipt format, and that identifier is a shared name. If they do not, R6 is false at once. Splitting makes "the same purchase" a judgement about aggregation, which no join decides. Separately, receipts go to recipients and witnesses. The ministry is neither for the team, which sits three containment levels inside the group, nor for the audit body, over which it has no edge ("the audit of ministry-funded purchases" is not a unit). Reading the team's receipts is reading inner logs relative to the group's summary.

**Change.** Freeze what a receipt carries. Restate R6 as: given a declared, context-dependent mapping between the two purchase vocabularies (R3's class) and a stated aggregation rule for splits, the breach is detectable from summaries exported to the owning authority. Declare the edge or the reporting duty that gives the ministry those summaries.

### M8. A deviating obligee can make a compliant obligor bear the loss, and recorded clock readings are self-reported

**Text attacked.** "An obligation is discharged by a receipt logged at the obligee's controller before the deadline". "Lateness caused by an unbounded link is allocated loss borne by the obligor." "event order and clock readings are recorded as inputs". Accountability "cover[s] only effects that reach at least one honest party".

**Why (judgement).** The obligee's controller issues the discharging receipt. If it deviates by dropping the obligor's performance, or by recording a clock reading after the deadline, the obligor holds no receipt. The case is indistinguishable from a slow link, so the declared rule puts the loss on the compliant obligor. No one is wrongly named, so R2(b) holds, but R2(a) fails if "covered" means "seen by an honest party", because the obligor saw its own send. Replay cannot catch a false clock reading, because the reading is one of the replayed inputs. PeerReview handles the first half by challenges sent through the witnesses, ending in suspicion: https://haeberlen.cis.upenn.edu/papers/peerreview-tr.pdf (section 4.8, as cited in round 3; not re-fetched).

**Change.** Define coverage as "an honest party holds a receipt". Let an obligor submit performance through the obligee's witnesses, and let a suspicion verdict against the obligee shift the allocated loss. Require witnesses to reject an entry whose recorded time differs from their own receipt time by more than the link bound plus skew. Add this case to variant 8.

### M9. R5's decidability contains R1 and R3, and the stop rules treat the three as independent

**Text attacked.** R5: "the check that an amendment preserves the entrenched invariants (... and conformance to the effective norm set) is decidable". R1: "Amendment is an action in the analysis". Stop rules for R1, R3 and R5.

**Why (judgement).** Conformance to the effective norm set goes through mappings that "depend on context read from the lower level's state". A check made when the amendment is passed must hold for every state reachable afterwards, which is a reachability question of R1's kind over R3's mappings. So R5 cannot be decided if either R1's fragment or R3's mapping class is not. In the other direction, R1 quantifies over every amendment that R5's check accepts. The plan can continue with R3 failed and R5 "passed", which is not a coherent state. Also, a check that accepts the one frozen useful amendment and rejects all others passes R5's test. And "complete against" is used for what is normally called soundness (no bad amendment accepted).

**Change.** State the dependency: R5 is attempted for the fragment where R1 and R3 survive, and falls with them. Freeze a class of useful self-amendments, not one. Say "sound" where no bad amendment may be accepted.

### M10. Refusal while a conflict is pending does not resolve an obligation, and gives every authority a way to halt a unit

**Text attacked.** "Where a conflict has no declared priority, the unit refuses the affected actions until a named adjudicator decides, and the refusals count as unavailability."

**Why (judgement).** For an obligation against a prohibition (variant 10), refusing the action is compliance with the prohibition. The obligation's deadline passes, so "pending" is in effect a decision for the prohibition, and the plan does not say which verdict the missed deadline receives. Links to an adjudicator in another unit have no delivery bound, so the refusal can last without limit. Any one of a unit's authorities can therefore stop the unit by issuing a norm that conflicts with another authority's.

**Change.** State that a deadline is suspended while its conflict is pending, or that the miss is allocated loss with a named bearer. Add a default that applies after a stated time. Record who caused each pending conflict, and count pending time per authority in step 5.

### M11. The assurance-record checks, R4, R6 and the exit test have no consequence attached; the exit test's positive half is now vacuous

**Text attacked.** "checks that the derivation gives the same record however the units are grouped". Stop rules (R2, R1, R3 and R5 only). Exit test: "each clause is judged satisfied by at least two systems", with "Two small constructed examples that are meant to satisfy the definitions".

**Why (judgement).**
- If the record derivation is not independent of grouping, "fractal" fails for the definition of "controlled". Nothing says what follows. Different groupings hide different links, so the check needs a rule for which hide sets are compared.
- Failure of R4 undermines the rule-change bounds of step 5. Failure of R6 removes the only target about one identity across units. Neither has a stated consequence.
- The two constructed examples are written by the authors to satisfy every clause. They supply the "at least two" for every clause, so that half of the test cannot fail. The examples were needed only for the composite definitions.
- "If the weakest repair needs inner logs ... stop" lets a repaired R2 replace the frozen one. The repaired statement is then written after seeing the counterexample.
- R2's test "summaries that must grow to the size of the inner logs" has no threshold. A summary carries a boundary trace prefix. For a deterministic controller the state needed for a counterfactual is a function of the whole input history, so the prefix may be the whole boundary trace.

**Change.** Require each clause to be satisfied by at least one of the ten existing systems. Attach a consequence to each of: grouping dependence (withdraw "fractal" for assurance records), R4 failure (withdraw the cap bound in step 5), R6 failure (add to the narrowings). Freeze a growth bound for summaries in step 0, for example independent of the number of internal actions. Say that a repaired R2 is reported as a new conjecture, not as the surviving target.

### M12. The scale section still mixes relations, and lease dependence makes availability fall with authority path length

**Text attacked.** "Per-decision cost. Proportional to containment depth plus the number of applicable authorities." "a lease that depends on an authority cannot be renewed once that authority's own lease has expired."

**Why (judgement).** The paragraph above says rules flow along the authority relation, not containment, so containment depth has no stated role in the cost. The cost omits policy size, the number of authority ancestors if the effective norm set is transitive (M1), conflict detection (H1), and the round trip for f+1 witness signatures on every receipt. The lease rule means a unit can authorise only while every authority on every path to a root has renewed in time. Those links have no delivery bound. The plan counts the result as unavailability, but states no availability measure in terms of path length, and the scale claim's failure conditions do not include it.

**Change.** State per-decision cost in policy size, authority ancestors and witness quorum size. Add expected availability as a function of the longest authority path and the lease period to the bounds, with a ceiling whose breach fails the scale claim.

### M13. Witnesses are not placed in the model, and the fault bound is assumed for every witness set at once

**Text attacked.** "Each unit, and each shard, has a witness set of 2f+1 members of which at most f are faulty. Witnesses are appointed by a rule authority or the anchor". "Fractal ... every unit exposes ...". "Accountability results allow a controller and up to f of its witnesses to deviate".

**Why (judgement).** The plan does not say whether a witness is a unit. If it is, it has a controller that needs witnesses, and the regress needs a stated end. If it is not, the foundation has a primitive outside the uniform interface. An appointing authority is often a counterparty: the ministry appoints the witnesses of the group it funds and has a dispute with in variants 4 and 6. A deviating ministry controller, which the threat model allows, can appoint f+1 colluders, and summaries carry witness signatures only. A theorem that assumes the bound for every witness set is close to vacuous at 10^10 units with independent faults. The useful statement is local: what a unit's neighbours can still rely on when that unit's witness bound fails.

**Change.** Say what a witness is and where the regress stops. Require appointments to be receipted actions, and say which authority appoints when there are several. State each accountability result per unit, with a containment lemma: a unit whose witness bound fails cannot cause a compliant neighbour to be named. Add a variant in which the appointing authority is the counterparty.

---

## LOW

- **L1.** *Verified from the text.* The status paragraph says "Two rounds of independent adversarial review shaped it" and then describes three.
- **L2.** *From memory.* "a sound assume-guarantee rule, which exists for safety guarantees". In Abadi and Lamport's composition theorem the safety condition is on the assumptions; guarantees may contain liveness (https://lamport.azurewebsites.net/pubs/abadi-conjoining.pdf, not re-fetched). Since each guarantee is a neighbour's assumption the practical effect is the same, but the ledger should state the theorem as published.
- **L3.** *Verified at abstract level.* R1's test says counter systems with reset can be "undecidable or non-elementary". More exactly: reachability is undecidable for reset nets, and coverability is decidable and Ackermann-hard (Dufourd, Finkel and Schnoebelen, https://unpaywall.org/10.1007%2FBFB0055044; later statements in https://arxiv.org/pdf/1311.4915v1). "Can p ever obtain r" is a coverability question. "Does A always contain B" needs a negative condition and is a reachability question, so it sits on the undecidable side once revocation is a reset. The step-0 freeze should state the two questions' classes separately.
- **L4.** "Five small cases" lists five groups containing nine cases. None of them exercises a pending obligation conflict (M10), a forked log (M3) or R6. Add the first two.
- **L5.** Variant 11's "replaces the witnesses" is now outside the unit's power by the appointment rule, so the case is decided by a definition. Replace it by an amendment that changes the witnesses' replay period or the reference implementation they replay against.
- **L6.** The threshold cosigning source in R2's prior-work column lets the verifier choose its threshold; it is a precedent for cosigning, not for f+1 of 2f+1 (https://arxiv.org/abs/1503.08768, as described in round 3; not re-fetched).

---

## Round-3 fixes checked

Faithful and sound: the two mechanism axes; discharge only through a sound rule; observables at the boundary after hiding; the delivery term in detection time; suspicion never counting as blame; the time source as an anchor; the vendor gateway; links to an outside adjudicator placed in the unbounded class; the frozen items for R1; the summary with one entry per guarantee and variant 12; the charter shard; R4 without its third outcome; the scale parameters over the authority relation; the four rule-change quantities and the cap arithmetic in step 5; the reuse ledger; the common inventory and explicit outcomes of the bake-off; the three narrowings; the listing of anchors per theorem. Verified in this round: the cited timed specification theory has the operators the plan names; dynamic input/output automata have monotonicity of creation "subject to some technical conditions" and no time (https://arxiv.org/abs/1604.06030).

Faithful to the suggestion and wrong in the result: the consistency clause of R3 (H1); f+1 signatures on summaries (M3); the second bake-off candidate (M4); R4's grandfathering against step 5's (M6); the two constructed examples in the exit test (M11); the ministry as owner of R6's join (M7).

## Gap to the goal

The narrowing is stated honestly and the narrowed claim is still worth testing. Two further limits belong in the list. First, all rule authority descends from the anchor, and the anchor, or a common superior that usually does not exist, decides every conflict between authorities (M2). For a goal that includes nations, this is the largest remaining distance: the plan models a world with a sovereign. Second, the benchmark does not exercise dynamic creation beyond one unit or two independent authorities (M1, M4), so a positive result would say less about "fractal" and "several rule authorities" than the first paragraph of the section claims.

## Verdict

No target is known impossible, and no step is unbuildable. R1 and R2 have real refuting tests. R3's first half is a real open question once M1 is fixed; its second half is false as written and must be restated (H1). R4 and R6 are decided by step-0 definitions as they stand (M6, M7). The witness scheme is satisfiable and not vacuous, but it gives eventual detection, not unique summaries, and the plan should say so (M3).

The plan is sound enough to start step 0. Step 0 should begin with H1, M1 and M2 together, since they all concern the authority relation and the effective norm set, and then M3. The order of work is otherwise right: the immediate checks, the paper stage and the stop rules come before any mechanisation, and the R2 stop rule can fire.

CRITICAL: 0  HIGH: 1  MEDIUM: 13  LOW: 6

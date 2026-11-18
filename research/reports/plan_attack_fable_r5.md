# Adversarial review of "Plan for an integrated foundation", version 5 (fable, round 5)

**Method.** I read the current plan first and formed findings, then diffed it against version 4, then read the round-4 reviews to remove repeats and to check that the round-4 fixes were applied faithfully. Sources fetched in this round are marked "verified". Arguments I made myself are marked "judgement". Citations I give from memory are marked †.

**Overall.** No CRITICAL defect. Two HIGH defects, both in text written in this pass and both interactions between a round-4 suggestion and an assumption elsewhere in the plan:

- The challenge period cannot be a finite stated value under the plan's own network model, so R2's new refuting test succeeds by construction.
- Founding an authority by consent lets a unit choose who appoints its witnesses and turn its own breach into a blameless "allocated loss".

Twelve MEDIUM defects and four LOW follow. The plan is sound enough to start step 0, provided the two HIGH items are settled as design decisions before the freeze, because the freeze as scheduled cannot settle them.

---

## HIGH

### H1. No finite challenge period is sound when links between units have no delivery bound, so R2's fork refuter always succeeds

**Text attacked.**
- Receipts and witnesses: "With f+1 signatures out of 2f+1, two valid summaries of different prefixes of a forked log can exist; forwarded receipts reveal the fork eventually. A chain verdict is therefore provisional until a stated challenge period has passed, and a verdict that is later contradicted is revised on the record."
- Assumptions: "all links between units ... have no delivery bound, but every message is eventually delivered."
- R2 refuter: "one history with two valid summaries that still give different verdicts after the challenge period".
- Step 0 freeze: "the challenge period (R2)". Also "Log retention periods are frozen with the other parameters."

**Why it is wrong (judgement, from the plan's own assumptions).** The only mechanism that reveals a fork is a recipient in another unit forwarding its receipt to the forking unit's witnesses. That message travels on a link between units, which has no delivery bound. For any challenge period T frozen in step 0, take f = 1, honest witnesses H1 and H2, faulty witness F and a faulty controller. The controller shows prefix A to H1 and prefix B to H2, and F signs both. Both summaries are valid. The forwarded receipt that exposes the fork is delivered at T + 1. The plan's network model allows this history, so:

- The refuter "two valid summaries that still give different verdicts after the challenge period" is satisfied for every T. R2 as frozen is false by a three-line argument, and the stop rule "the work continues only if R2 survives" then fires on a technicality.
- Alternatively, if "revised on the record" is taken to mean no verdict is ever final, then the challenge period does nothing, and R2(b) "no compliant party is named" holds only in the limit. A compliant neighbour can be named in a verdict computed from the fork that hides the true breach, and stay named for an unbounded time.
- A finite log retention period has the same defect. Evidence that arrives after the retention period cannot be checked, so "revised on the record" can become impossible.

**Evidence.**
- PeerReview states its guarantees only as eventual, for exactly this reason: "Eventually, every detectably ignorant node is suspected forever by every correct node", under the assumption that a message between correct nodes "is eventually received, if retransmitted sufficiently often" (verified in the technical report, sections 3 and 4): https://haeberlen.cis.upenn.edu/papers/peerreview-tr.pdf
- Challenge-period designs in optimistic rollups are secure only if an honest challenge can be delivered within the period. The literature treats delaying the challenger past the period as the attack that breaks them (verified at abstract level): https://arxiv.org/abs/2502.20334 and https://ethereum.org/developers/docs/scaling/optimistic-rollups

**Fidelity.** Round 4 (fable M3) offered three options. The editor took option (iii), the challenge period, and kept unbounded links between units. Neither the reviewer nor the editor checked the option against the network assumption. Options (i) and (ii) do not have this defect.

**Change recommended.** Choose one before the freeze. This is a design decision, not a parameter.
1. Keep 2f+1 witnesses and add a stated delivery bound between the witnesses of one unit, with a rule that witnesses exchange signed log heads before signing a summary. With f+1 honest witnesses, two honest signers of different forks then detect the fork within a local bound, and no challenge period is needed. This fits the existing assumption that controller-to-witness links are bounded.
2. Use 3f+1 witnesses and 2f+1 signatures, so two valid summaries of different forks cannot both exist.
3. Keep the challenge period, and state finality only inside the performance envelope: "a verdict is final if every link on the forwarding paths met its envelope bound during the challenge period; otherwise it is provisional without limit". Restate the R2 refuter to match, and state R2(b) for final verdicts only. Make the retention period at least as long as the longest provisional period, or record that evidence arriving later is lost.

Whichever is chosen, charge its cost to step 5.

### H2. An authority founded by consent lets a unit pick its own witnesses and convert its own breach into allocated loss

**Text attacked.**
- Four relations: "Units can also found an authority over themselves by a recorded vote of the founders, which adds an edge by consent."
- Same paragraph: "Authority edges, the priority among authorities and the powers of adjudicators are declared by the anchor or by an authority above all the units concerned, and a constrained unit cannot alter them alone."
- Witnesses: "Witnesses are appointed by a rule authority or the anchor, never by the unit's own charter."
- Effective norms: "Where a conflict has no declared priority, the unit refuses the affected actions until a named adjudicator decides ... A deadline caught in a pending conflict is either suspended ... or, if missed, classified as allocated loss with a named bearer ... The authority that caused each pending conflict is recorded."
- R5 invariant: "the witness duty with witnesses appointed from outside".

**Why the decisions contradict each other (judgement).**
1. *Witness capture.* A consent-founded authority is "a rule authority" over its founders. Nothing stops it appointing their witnesses. The founders control it by vote, so the founders appoint their own witnesses at one remove. The rule "never by the unit's own charter" and R5's entrenched invariant are both satisfied in form and defeated in substance. Every accountability result rests on the witness bound.
2. *Manufactured conflicts.* A unit with an outstanding commitment founds an authority over itself. That authority issues a norm forbidding performance. The new authority has no priority against procurement law, because priorities are declared only by the anchor or a common superior. The conflict is therefore pending, the unit must refuse, and the missed deadline is either suspended or "allocated loss". No party deviated under the plan's definitions, so R2(a) has nothing to name. Recording "the authority that caused" the conflict records a body the obligor controls.
3. *"Cannot alter them alone."* A single unit founding an authority over itself by a vote of one alters its own authority set alone. The text does not require more than one founder.
4. *Load on the anchor.* Every consent-founded authority is a root outside the anchor's descent. Any conflict it has with an inherited norm has "no common superior", so by the first section it goes to the anchor. At the target scale the anchor adjudicates every such conflict.
5. *Unplaced facts.* Who appoints the new authority's own witnesses is not stated. Variant 3 exercises founding only on a compliant path.

**Evidence.** The argument uses only the plan's text. The legal analogue is that private agreement cannot displace mandatory law, which the plan's rule lacks.

**Fidelity.** Round 4 (fable M2) asked the editor to "add ... consensual formation of an authority over its founders is not modelled ... or model it". The editor modelled it in one sentence and did not check it against the witness rule or the pending-conflict rule, both also edited in this pass.

**Change recommended.** Either move consent-founded authorities back to the list of things not modelled, or add these rules to the definition:
- A consent-founded authority has, by default, lower priority than every authority that descends from the anchor. A norm of it that conflicts with an inherited norm is overridden at once, with no pending state.
- It cannot appoint or remove the witnesses of its founders or of any unit its founders control. Its own witnesses are appointed by the anchor or by an authority over the founders.
- If a pending conflict is caused by an authority that the obligor founded or controls, the miss is a proved deviation by the obligor.
- At least two founders, or state that one suffices and why.
- Add to variant 3 or variant 10: a unit founds an authority that forbids an outstanding commitment, and a founded authority tries to appoint a founder's witnesses. Both must be rejected.

---

## MEDIUM

### M1. R4's residual rule breaks the cap unless "grandfathered spending" means every outstanding old right, and the grandfathered set cannot be explicit at installation

**Text attacked.** R4: "installed with an explicit set of grandfathered commitments no larger than the frozen bound. For an aggregate invariant such as a cap, new rights are issued against the new cap minus completed and grandfathered spending, floored at zero." Step 5: "total spending never exceeds the larger of the two caps".

**Why (judgement, by calculation).** A controller that has not heard of the change keeps spending old rights until its lease expires. Those purchases do not exist when the change is installed, so the installer cannot list them. If "grandfathered spending" is read as spending known at installation, the claim fails:

| Quantity | Value |
|---|---|
| Old cap | 100 |
| Completed spending at the change | 10 |
| Old rights still held by a shard that hears late | 90 |
| New cap | 95 |
| New rights issued (95 − 10 − 0 known grandfathered) | 85 |
| Late shard spends its old rights within the lease | 90 |
| Total spending | 185 |

185 exceeds both caps. The claim holds only if the subtraction uses all old rights that have not expired or been returned, whether or not they have been spent. Then new rights are 95 − 10 − 90, floored at zero, and the total is at most 100. The frozen bound is stated in "rights outstanding at the change", which also omits commitments already made and not yet completed.

**Change.** Write: "new rights are issued against the new cap minus completed spending, commitments outstanding, and all old rights not yet expired or returned". Say that the grandfathered set is closed only when the last old lease expires, and that unspent old rights return to the pool then. Add the table above as an immediate check in step 1.

### M2. A cap is a norm, so prioritised selection can lawfully override it, which contradicts R4 and the step-5 cap claim

**Text attacked.** Effective norms: "a cap with two commitments that together exceed it is a conflict of three". Variant 10: "a cap and two commitments that each fit and together exceed it, under fully declared priorities". R4 refuter: "total spending exceeds the larger of the old cap and the new cap". Step 5: "total spending never exceeds the larger of the two caps".

**Why (judgement).** If the declared priorities rank either commitment's source above the cap's source (the regulator above the ministry, say), the selection leaves the cap out and spending exceeds it lawfully. The R4 refuter then fires on a case the plan itself calls valid. The plan treats the cap both as an invariant kept by rights allocated in advance and as a norm subject to override, and does not say which wins.

**Change.** State R4 and the step-5 claim "in states where the cap is in the effective norm set". Or declare that aggregate invariants enforced by allocated rights are never overridden, so a commitment that would breach one is refused when it is made and never reaches selection. Say in variant 10 which outcome is required.

### M3. R3's premise cannot hold in exactly the cases that are new, and with the alternative reading its conclusion is definitional

**Text attacked.** R3: "pairwise design compliance plus enforcement at the leaf implies that the leaf's behaviour complies with its effective norm set in every state where that set is resolved". Effective norms: "a prioritised selection over the norms of all its authority ancestors".

**Why (judgement).** In variant 10 the regulator obliges what procurement law forbids. No set of rules for the subsidiary is design-compliant with both authorities, so "pairwise design compliance" is false and the implication says nothing. Where no norms conflict, the claim is the published multi-level result over the plan's stated baseline. If instead the leaf's controller enforces the effective norm set directly, compliance follows from complete mediation and the pairwise premise is unused, which the plan's own stop rule would strike out as definitional. The new content is therefore in neither reading.

**Evidence.** King et al. define compliance of a lower institution with a higher one and do not treat conflict between two governors in the abstract (verified at abstract level only): https://researchportal.bath.ac.uk/en/publications/automated-multi-level-governance-compliance-checking/

**Change.** In the step-0 freeze of "the pairwise compliance predicate", define compliance of a unit's rules with one authority *relative to the norms of that authority that survive selection*, and say who computes the surviving set and from what information. State the theorem as: local checks of that predicate at each edge, each using only a bounded description of the selection, imply compliance with the effective set. The research question is then whether the selection can be summarised per edge. Add a refuter: two edges that each pass the local check while the leaf breaks a surviving norm.

### M4. The uniqueness half of R3 is known for total orders and known to fail otherwise, and the plan's priorities are not a total order over norms

**Text attacked.** R3: "the resolved set is unique, or its alternatives are bounded in number and each is recorded". Effective norms: "a published construction chosen in step 0 (for example maximal consistent subsets taken in priority order)". Four relations: priorities form "a strict partial order" over authorities.

**Why.**
- *Verified.* Liao et al. prove a unique extension only "for a totally ordered hierarchical abstract normative system" (Lemmas 1 and 2). Their Reduction procedure "may have a unique extension, multiple extensions, or an empty extension" even under a total order. The three procedures give different answers on the same input, and the paper leaves preorders to future work: https://arxiv.org/pdf/1709.08034
- *Judgement.* A priority over authorities lifts to at most a preorder over norms, since two norms of one authority tie. That is the case the cited paper does not cover. With k independent conflicts between incomparable norms the alternatives number 2^k, so "bounded in number" fails unless every such conflict is sent to adjudication, and then uniqueness holds by definition of "resolved".
- *Judgement.* "Where a conflict has no declared priority" is undefined for a conflict of three or more norms under a partial order. A set {a, b, c} with a above c and b incomparable to both has a declared priority for one pair only.

**Change.** Freeze in step 0: the lifting from authority priority to norm priority, including a tie-break that makes it total or a statement that ties go to adjudication; the definition of "decided" for a conflict set (for example, the set has a unique lowest member); and which of the published procedures is used, tested on the order puzzle from the cited paper. Restate the claim as "the selection is unique exactly when the lifted order is total on every conflict set", record it as reuse, and keep only the complexity of detection as the open part.

### M5. The benchmark's authority graph has two paths from the ministry to the subsidiary, and nothing requires the two translations of one norm to agree

**Text attacked.** Benchmark: "the ministry over the group and the subsidiary through procurement law; the group over the subsidiary through group policy". R3: "mappings between levels depend on context read from the lower level's state". Effective norms: "the norms of all its authority ancestors".

**Why (judgement).** A ministry norm reaches the subsidiary directly and through the group. Each path has its own vocabulary mapping. If the composed mapping through the group differs from the direct one, the same norm appears twice in the subsidiary's vocabulary with different content, and possibly with different priority (the ministry's and the group's). The selection can then keep one copy and drop the other. Round 1 (astra) asked for a commuting diagram for one chain. The directed acyclic authority relation added later needs the same condition across paths, and no version has stated it.

**Change.** Add to R3's "stated condition on the mappings": for every pair of authority paths between the same two units, the composed mappings agree on the norms of the upper unit, or the norm carries its origin and is counted once at its origin's priority. Add a case to variant 10 in which the two paths disagree.

### M6. The bake-off is decided by a ruling with no criterion, variant 3 does not require unbounded creation, and the pool candidates may make the "new metatheory" unnecessary

**Text attacked.** Step 0: "Whether a bounded pool of dormant units counts as creation." Variant 3: "The subsidiary then creates a project unit, which creates a further unit, so that creation is not bounded in advance." Step 2: "If step 0 rules that a bounded pool does not count as creation, only the default can express the unbounded creation in variant 3, and the bake-off then reduces to whether the default's new metatheory can be established."

**Why (judgement).**
1. Variant 3 creates three units. A pool of three dormant units expresses it exactly. "Not bounded in advance" does not follow from a fixed finite scenario.
2. If the ruling allows pools, the two pool candidates have published composition theorems and the default does not, so the default loses on the third decision question. If the ruling forbids pools, there is one candidate. The outcome of the bake-off is fixed by the ruling, and the plan gives no test for the ruling.
3. Every property the plan claims is a deadline or prevention property (non-goal: "No general liveness"). Such properties are decided on finite prefixes, and a finite prefix of a non-Zeno run contains finitely many creations. A theorem proved for a pool of size N, for every N, with dormant members inert until activated and a unit's rules held as controller state (which step 3 already says), therefore appears to give the same guarantees as unbounded creation. If that holds, the new metatheory item is avoidable. I did not prove this; the conditions on time-blocking must hold uniformly in N.

**Evidence.** David et al. 2013, cited as a creation precedent, gives a semantics and a statistical model-checking engine and proves no refinement or substitutivity theorem for creation (verified in the full text; its only theorem is about monitoring formulas): https://homes.cs.aau.dk/~kgl/SSFT2015/DYN.pdf. The definition of safety by finite prefixes is Alpern and Schneider†: https://www.cs.cornell.edu/fbs/publications/DefLiveness.pdf

**Change.** Replace the ruling by a criterion: "a pool counts if the theorems are proved for every pool size with hypotheses that do not depend on the size". Make variant 3 create a number of units that depends on an environment input. Add to step 1, before the new metatheory: check whether the for-every-N pool argument gives closure and substitutivity for the plan's property class. If it does, make plain timed input/output automata with a pool the default and move the combined model to the list of things not tested.

### M7. R5 falls whenever the plan continues on one of R1 and R3, and step 3 then has no amendment rule

**Text attacked.** Stop rules: "The work continues only if R2 survives and at least one of R1 and R3 improves on its stated baseline"; "R5 falls with R1 or R3". R5: "For the frozen grammar and the fragment in which R1 and R3 survive". Step 3: "An amendment is valid when it ... passes the entrenchment check of R5."

**Why (judgement).** The plan continues with exactly one of R1 and R3. In that case R5 falls by the stated rule, the self-reflection claim is withdrawn, and step 3's validity rule refers to a check that no longer exists. The dependency on R1 is also unexplained: none of the four entrenched invariants is an authority-reachability question. Only "conformance to the effective norm set" depends on R3. Two further gaps: the entrenchment check and the charter's obligation to run it are not themselves in the entrenched list, and variant 11 lets a unit change "the reference implementation the witnesses replay against" although witnesses are appointed from outside.

**Fidelity.** Round 4 (fable M9) said to "state the dependency: ... falls with them". The editor copied it without reconciling it with the continuation rule.

**Change.** Split R5 by invariant. Single ownership, log continuity and the witness duty depend on neither R1 nor R3. Conformance depends on R3 only. State what step 3 does if R5 has fallen: amendments are limited to a syntactic class that cannot touch the entrenched terms, and the claim is reported as "no self-amendment of entrenched terms". Add the entrenchment check itself to the entrenched set. Say that the reference implementation changes only with the consent of the authority that appoints the witnesses.

### M8. Availability is parameterised by the longest authority path, but the stated rule makes it depend on every ancestor, and the model has no delay distribution

**Text attacked.** Step 5: "A unit can authorise only while every authority on every path to a root has renewed its lease in time. Expected availability is stated as a function of the longest authority path and the lease period, with a floor fixed in advance".

**Why (judgement).** "Every authority on every path" is the whole ancestor set. A unit with k independent authorities at depth one has path length one and availability that falls with k. The step-5 preamble lists "the number of authorities over a unit" as a parameter; the availability bullet omits it. Second, "expected" needs a probability that a renewal arrives in time. The network model gives only eventual delivery, under which the worst-case availability is zero and no expectation is defined.

**Fidelity.** The wording is round 4's (fable M12), copied as written.

**Change.** State availability as a function of the number of ancestor authorities and edges in the ancestor closure, the lease period, and a per-link probability of meeting the envelope bound, which is a new stated workload assumption. Report the worst case separately as zero outside the envelope.

### M9. Acyclicity and the priority order have no owner that can enforce them, and the concurrent case is not in the benchmark

**Text attacked.** Four relations: "Keeping the authority relation acyclic and the priorities a strict partial order under concurrent declarations is a global invariant; it is classified with the other invariants in step 1 and its cost is counted in step 5." Controllers: "An invariant that spans shards is kept only through rights allocated in advance." Variant 10: "a priority declaration that would close a cycle, which must be rejected as invalid input."

**Why (judgement).** Coordinated actions are defined between shards of one unit. Acyclicity spans units. Two authorities that are both above units U and V can concurrently declare U over V and V over U; each declaration is valid alone. No controller named in the plan sees both, and the invariant is not a quantity that allocated rights can protect. Variant 10 tests a single declaration, which is the easy case; round 4 (fable M2) asked for the concurrent one.

**Change.** Either name the owner (the anchor, or the least authority above both declarers, with the cost counted), or make the invariant locally checkable: give every unit a rank at creation and allow authority edges only from higher to lower rank, and derive priority from a total order on authority identifiers unless declared otherwise. A fresh consent-founded authority can never close a cycle; say so. Add the concurrent pair of declarations to variant 10.

### M10. R6 is settled by the content of a summary, and the audit body's reporting duty has no authority behind it

**Text attacked.** R6: "a breach is detectable from the summaries that the team and the audit body report to the ministry". Benchmark: "The team and the audit body have a declared duty to report summaries of ministry-funded purchases to the ministry"; "an external audit body over the officer's audit role".

**Why (judgement).** If the frozen summary carries, for each purchase, the identity that approved it and the identity that audited it, detection is a join on the declared mapping and R6 is true by construction. If it does not, R6 is false. Either way it is a step-0 choice and would be struck out by the plan's own rule on definitional statements. The approval and the audit are internal actions of their units, while the summary growth bound "does not depend on the number of internal actions"; the two need to be reconciled. Separately, the ministry has no authority edge to the audit body, so nothing in the model imposes the reporting duty on it.

**Change.** Reclassify R6 as an application lemma, or give it content: detection when one of the two reporting units' controllers deviates within the R2 fault bound, at summary size linear in boundary purchases. Name the authority that imposes the duty on the audit body.

### M11. The default that applies after a stated time races with a late adjudication, and the plan does not say which stands

**Text attacked.** Effective norms: "step 0 fixes which, and fixes a default that applies after a stated time." Assumptions: links "to an adjudicator in another unit, have no delivery bound".

**Why (judgement).** The unit applies the default on its local clock while the adjudicator's contrary decision is in transit. Actions taken under the default then contradict the decision. The plan does not say whether the decision applies only from its arrival, or whether acts under the default are grandfathered under R4 or reversed.

**Change.** Freeze: a decision that arrives after the default applies from arrival only; acts under the default are treated as commitments made under the previous rule version and fall under R4. Add the case to variant 10.

### M12. Variant 6's conflicted appointment has no required outcome, so it cannot fail

**Text attacked.** Variant 6: "where the ministry is a party to the dispute and appointed the group's witnesses". Witnesses: "its honesty up to the bound is an assumption that is not itself witnessed".

**Why (judgement).** Inside the model, at most f witnesses are faulty whoever appointed them, so the appointment changes nothing. If more than f are faulty the guarantees are void by statement. The variant has no outcome that distinguishes a good design from a bad one.

**Change.** Give it one: for example, a rule that an appointing authority that is party to a dispute may appoint at most f of the 2f+1 witnesses of its counterparty, with the variant failing if the adjudicator's verdict can be determined by witnesses appointed by one party alone. Otherwise delete the clause and list appointer independence among the anchors.

---

## LOW

- **L1.** Variant 8 says "in four cases" and lists six.
- **L2.** The first section says "Three narrowings are specific to this design" and gives four (acyclic authority, override, a root of authority, dependence on named anchors).
- **L3.** "An obligor may submit its performance through the obligee's witnesses": the discharge rule still requires a receipt "logged at the obligee's controller before the deadline". Say whether the witness's receipt time or the controller's log time decides timeliness, and whether f+1 witness receipts discharge the obligation when the controller drops the entry.
- **L4.** "When a unit's witness bound fails ... no compliant neighbour is named". Being named and bearing loss differ. A compliant obligor whose obligee has a failed witness bound is not named, and still bears the allocated loss for "lateness". State the loss rule for that case next to the naming rule.

---

## Round-4 fixes checked

| Round-4 item | Applied | Faithful and sound |
|---|---|---|
| Conflicts as minimal sets; prioritised selection (fable H1, astra 1) | Yes | Faithful. Leaves M3, M4, M5 above |
| New authority edges, independent regulator (fable M1) | Yes | Faithful. The ministry now reaches the subsidiary by two paths (M5) |
| Who declares priorities; consent founding; acyclicity (fable M2) | Yes | Not sound: H2, M9 |
| Fork handling by challenge period (fable M3, astra 2) | Yes | Not sound under the network model: H1 |
| Third candidate and pool question (fable M4) | Yes | Faithful; exposes that the bake-off is decided by the ruling (M6) |
| "New metatheory" rewording (fable M5) | Yes | Faithful. Berger's paper is a congruence theory for timed asynchronous pi-calculi (verified by title and abstract): https://link.springer.com/doi/10.1007/978-3-540-28644-8_8 |
| R4 frozen bound and residual rights (fable M6) | Yes | Ambiguous in a way that breaks the claim: M1, M2 |
| R6 from reported summaries (fable M7) | Yes | Faithful; now definitional (M10) |
| Obligor through obligee's witnesses (fable M8) | Yes | Faithful; details in L3 |
| R5 termination, soundness, completeness; dependency (fable M9, astra 6) | Yes | Statement is faithful; dependency rule conflicts with the stop rule (M7) |
| Pending deadlines (fable M10, astra 3) | Yes | Faithful; M11, and exploited in H2 |
| Consequences for failed targets (fable M11) | Yes | Faithful |
| Availability bound (fable M12) | Yes | Wrong parameter: M8 |
| Witness placement, per-unit statement (fable M13) | Yes | Faithful; M12, L4 |
| R1 refuter (astra 4), assume-guarantee wording (astra 5) | Yes | Faithful. I found no error in either |

## Targets against known results

- **R1.** Not known false or true as stated, because the fragment is frozen in step 0. The refuter is real. I did not re-verify the reset-counter remark this round.
- **R2.** False as worded because of H1. With H1 fixed, the remaining refuters (same summaries with different verdicts, joint causation, growth bound) are real and can go either way.
- **R3.** The main implication needs M3. The uniqueness claim is known in both directions (M4). The detection claim is open only once the grammar is frozen.
- **R4.** Can now fail, and does on the reading in M1.
- **R5.** Can fail on both soundness and completeness. Its dependency rule needs M7.
- **R6.** Definitional (M10).

## Prior art that would change the plan

- **Fork consistency.** Storage systems that cannot prevent a faulty server from showing clients different histories guarantee only that the fork is detected when clients compare notes. This is the precise name for what the plan's summaries provide and belongs in R2's baseline (Mazières and Shasha; Li et al., SUNDR†): https://www.usenix.org/legacy/event/osdi04/tech/full_papers/li_j/li_j.pdf
- **Accountable agreement and forensics.** Results on which quorum sizes allow guilty parties to be identified after a fork bear on the choice in H1 (Civit et al., Polygraph†: https://eprint.iacr.org/2019/587; Sheng et al., forensics†: https://arxiv.org/abs/2010.06785).
- **Policy combination.** Prioritised selection among several authorities' permissions and prohibitions is the subject of policy-composition algebras and of combining rules in deployed policy languages. These give a decidable baseline for the non-deontic part of R3 (Bonatti et al.†: https://dl.acm.org/doi/10.1145/504909.504910).
- **Prioritised norms.** Verified this round: uniqueness holds only for total orders, and the choice among Greedy, Reduction and Optimization changes the result: https://arxiv.org/pdf/1709.08034
- I searched for a published composition theorem for input/output automata with both time and creation and found none. The plan's statement stands (search result page for the dynamic model: https://groups.csail.mit.edu/tds/papers/Attie/1604.06030.pdf).

## Gap to the goal

The narrowing is stated honestly and is still worth testing. Two points the first section should add. Witnesses, the anchor, the time source and the identity issuer are not units, so the recursion stops at components that the foundation does not govern; at the target scale the witness population is at least as large as the governed population. And if M6's argument holds, "creation and dissolution of units" is delivered by a pool for every size, which is a weaker reading of "dynamic" than the goal suggests and should be said.

## Order of work

The order is right. Three additions:
1. Settle H1 and H2 as design decisions before the step-0 freeze begins; the freeze can only pick parameter values.
2. Put the pool-for-every-N check (M6) before any work on the combined model's metatheory, since it may remove that work.
3. Add the M1 table and the M2 override case to the immediate checks, since both can be run on paper in an hour and both currently fail.

## Verdict

Sound enough to start step 0, on condition that H1 and H2 are decided first. Neither defeats the plan's purpose, and each has a known repair, but each needs a choice between designs and neither is something the scheduled freeze of parameters can settle. As in earlier rounds, the worst errors are in the newest text and come from reviewer suggestions adopted without checking them against the network assumptions and the witness rule. The MEDIUM items are corrections to statements that step 0 already schedules.

CRITICAL: 0  HIGH: 2  MEDIUM: 12  LOW: 4

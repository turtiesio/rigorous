# Adversarial review of "Plan for an integrated foundation", version 7 (fable, round 7)

**Method.** I read the current plan and wrote down my findings first. I then diffed it against version 6 and read both round-6 reviews in full, and the headings and selected findings of rounds 1 to 5, to check the "known problems" column and the claim that the architecture is stable. I worked alone and wrote only this file. Each finding is labelled:

- **verified**: checked against a source fetched in this round.
- **plan-internal**: follows from the plan's own text or from the earlier versions and reviews in /root/mic/reports.
- **judgement**: my own assessment.
- **could not verify**: from memory, or the source could not be fetched.

**Overall.** No CRITICAL and no HIGH defect. The plan is sound enough to start step 0, because step 0 is now where the design is done. Twelve MEDIUM and six LOW defects follow. The restructuring removed the previous pattern (a one-line mechanism written into the plan between rounds), but three things went wrong in the rewrite:

1. The step-0 gate that replaces those mechanisms is weaker than it looks: the cases have no expected outcomes, each case tests one row, and every earlier serious error was an interaction between rows.
2. Several contested mechanisms were dropped from the text without becoming a decision row, and two repairs from earlier rounds were dropped with them.
3. Two rows (D4 with D7, and D9) decide whether R3 and R2 have any content, and the table does not say so.

---

## MEDIUM

### M1. The step-0 gate can pass vacuously: cases have no expected outcome, no case spans two decisions, and step 0 has no budget

**Text attacked.** "Each row is an open choice... none of the options is asserted to be sound." Exit test: "every design decision has passed its cases and the reviewers' check." Failure mode: "The design decisions have no sound combination. Step 0 then ends with the decisions that could not be reconciled."

**Why (plan-internal and judgement).**
- *No expected outcomes.* A case is a scenario. "Passing" needs a required result. Only one case in the table states one ("which must come out as outside coverage", and that is in variant 7, not the table). Examples with no stated result: "The regulator obliges what procurement law forbids" (D3); "A higher-ranked authority obliges a purchase that exceeds the ministry's cap" (D6); "A colluding recipient that presents its receipt after the verdict" (D8); "An approval the officer does not give before the deadline" (D7). Version 6 had required outcomes for several of these ("must be refused when made", "must be rejected"); the rewrite dropped them. Any option that produces some defined result passes.
- *One row per case.* The earlier serious errors were all interactions: the cap exemption against priorities (r6 fable M1), the eventual verdict against the pool argument (r6 astra 1), the local check against the origin rule (r6 fable M4), the consent-founded authority against witness appointment (r5 fable H2). The cases are regression tests for the single-row symptoms those reviews reported. Nothing tests a combination of choices. Pairs that interact and have no joint case: D1 with D2 and D7; D3 with D4 and D6; D5 with D11 and R4; D8 with D9 and D10; D9 with D12; D4 and D7 with R3 (see M2).
- *No budget.* Step 0 now contains the whole unsolved design. The plan has had six rounds of exactly this work and each produced new defects. There is no limit on iterations and no rule for when "could not be reconciled" is declared, so the kill criterion cannot fire.

**Change.**
1. Give every case a required outcome, written before the option is chosen.
2. Add a joint case list: after the thirteen choices are made, run every case against the whole chosen set, plus at least one case for each interacting pair above.
3. Encode the chosen set and the cases in a small executable model (a bounded model checker is enough at this size) instead of prose. Prose checking is what failed six times.
4. Fix a number of decision rounds in advance. If the reviewers still find an inconsistency after that many, the failure mode in the last section fires.

### M2. D4 and D7 between them decide whether R3 has content, and in three of the four combinations it is definitional, detached from its premise, or already refuted

**Text attacked.** R3: "A local check at each authority edge... implies the conclusion D7 fixes about the unit's effective norm set." D4 options: "Only the norms of its direct authorities, with an ancestor's norm binding through each intermediate authority's implementation; or the norms of all ancestors, each keeping its origin." D7 options: "Every miss of an obligation in the effective set is detected and classified; or compliance under a named assumption". Stop rule: "at least one of R1 and R3 improves on its stated baseline".

**Why (plan-internal).**
- *D4 first option.* The effective set is the norms of direct authorities. Each edge check covers exactly one direct authority's surviving norms. The conclusion follows from the definition plus the selection. The round-6 review that proposed this option described it as "as in the multi-level baseline" (plan_attack_fable_r6.md, M4, option i); the table drops that remark. With this option R3's first claim is the King et al. setting plus a greedy selection, which the plan already says "are not counted as results".
- *D4 second option.* The table's own known problem ("a check on each edge can pass while the unit breaks an ancestor's norm that no edge covered") is R3's own refuter ("Two edges that each pass the local check while the unit breaks a norm in its effective set"). So with this option R3 as worded is already refuted, unless the edge check is widened to every ancestor norm routed through the edge. Round 6 stated that widening as part of the option; the table omits it.
- *D7 first option.* The conclusion becomes "every miss is detected and classified". Detection of a miss is done by the unit's witnesses and the deadline, whatever the edge checks say. The premise of R3 is then idle for obligations.

Only the combination "all ancestors, widened edge check, compliance under a named environment assumption" leaves a claim that can fail for a reason other than the definition. King et al. check institution designs against each other, not behaviour (verified at abstract level: https://researchportal.bath.ac.uk/en/publications/automated-multi-level-governance-compliance-checking/), so the step from edge checks to behaviour is where any new content would be.

**Change.** In D4, add to the first option "R3's conformance claim then reduces to the baseline and R3 rests on its conflict-detection half", and to the second "the edge check must cover every ancestor norm routed through that edge; cost counted in step 4". In D7, add "under the first option R3's conclusion for obligations does not depend on the edge checks". In the stop rule, say which combination R3 is judged under.

### M3. D12 chooses the core in step 0, step 2 chooses it again after step 1, and the new metatheory is now listed as reuse with no stop rule

**Text attacked.** Step 1: "Done by hand in whichever core step 0's decision D12 points to." Step 2: "choose the core... Each encoding gets the same time budget, fixed in advance". Reuse list: "Closure and substitutivity for the chosen core, which is published for the pool model and new work for the dynamic one." First section: "Three targets are open research (R1 to R3) and two are narrower (R4 and R5). Everything else is reuse of published results or application lemmas".

**Why (plan-internal).**
- D12's two options are two of step 2's candidates. D12 is settled in step 0 on two cases. Step 2's decision rule (expressiveness, support for R2, common inventory, equal budgets, advocates) then runs after the paper stage has been done in the core D12 picked. Either step 2 is a formality or the paper stage may be redone. The third step-2 candidate (the timed specification theory) is not a D12 option at all.
- If D12 picks the dynamic model, "closure and substitutivity" is new metatheory. It sits in a list headed "Reused results and application lemmas", and the first section says everything outside R1 to R5 is reuse. Version 6 said "One piece of metatheory for the core is new work" and gave a drop rule ("failing to discharge them for variant 5... is a reason to drop this core"). Both sentences are gone. Step 1 is called "cheap", which it is not with that proof in it.
- Attie and Lynch state monotonicity of creation with respect to trace inclusion only "subject to some technical conditions" (verified, abstract: https://arxiv.org/abs/1604.06030), so the dynamic option carries real proof work.

**Change.** Make D12 provisional ("the core used for the paper stage") and keep step 2 as the confirming comparison, or move step 2's rule into D12 and delete step 2. Restore "one piece of metatheory is new work if the dynamic model is chosen" in the first section, move it out of the reuse list, and restore a time-boxed drop rule for it.

### M4. Contested mechanisms that are in neither the fixed architecture nor the table

**Text attacked.** "It fixes the architecture that has survived review, and it turns each contested mechanism into a design decision for step 0".

**Why (plan-internal).** The following were attacked in earlier rounds, were in version 6, and are now absent or appear only as a clause inside another row.

| Mechanism | Earlier finding | Where it is now |
|---|---|---|
| Who may declare an authority edge, and how acyclicity is kept under concurrent declarations by two authorities | r4 fable M2; r5 fable M9 | The fixed text says only "a declared acyclic relation". The rank rule that made the check local is now one option of D3. If D3's partial-order option is chosen nothing enforces acyclicity, yet D3's case "an authority edge from a lower rank to a higher one" assumes ranks under both options |
| The rule that names the bearer of an allocated loss, and who may declare a suspicion | r4 fable M8; r5 astra 2 | R2 says "allocated to a bearer by a declared rule". No row, and not in the freeze list for R2. Version 6's rules (lateness borne by the obligor; a suspicion against the obligee moves the loss) were deleted |
| Which unit computes a chain verdict, and what happens when that unit deviates or refuses to log evidence | new with D9's second option | D9 refers to "the unit that computes the verdict". The architecture names an adjudicating unit only for the disputed delivery in variant 5 |
| Finality: no finality with unbounded retention, or an explicit time bar | r6 fable M3 | Stated in D8's known-problems column as "the choice is between..." but D8's options are all about quorum size. Step 4 refers to "whatever retention D8 requires" |
| Who appoints each witness set | r5 fable H2; r6 fable M10 | "must be named" in D8's known problems, and "its witness appointer from a creation rule" inside D3's total-order option. No options |
| Dissolution by an outside decision, and what happens to a dissolved unit's resources, commitments, log and witnesses | r3 fable M6; version 6 immediate checks | D12 covers creation only. Step 2 notes that the published theorem "allows a unit to end only by its own action" and stops there. With no finality (D8), a dissolved unit's evidence must be kept and its witnesses stay on duty without limit, and a pool member cannot be reused |

**Change.** Add rows: D14 authority-edge declaration and acyclicity (owner or rank, with the concurrent pair of declarations as a case); D15 loss allocation and suspicion (cases: a deviating obligee and a compliant obligor; a slow link); D16 verdict computation (who computes, who checks, a deviating computer); D17 dissolution (variant 4 as a case, with the required fate of the outstanding commitment). Split D8 into quorum, finality and appointment, each with options.

### M5. D8: the second option needs an assumption the fixed architecture does not grant, two published options are missing, and a published impossibility bears on the finality choice

**Text attacked.** D8 options: "2f+1 witnesses with f+1 signatures and verdicts that may be revised; or a delivery bound between a unit's witnesses, who exchange signed log heads before signing; or 3f+1 witnesses with 2f+1 signatures". Fixed assumptions: "Links between a controller and its own witnesses have a delivery bound; links between units and to vendors do not."

**Why.**
- *Plan-internal.* The second option is new in this version and was proposed by no review. It needs a delivery bound on links between witnesses of one unit. The fixed assumptions give a bound only between the controller and each witness. So the option changes the "fixed" architecture, and the table does not say so.
- *Plan-internal.* The first option's "verdicts that may be revised" sits beside R2's "no proved-deviation verdict ever names a compliant party". These are compatible only if revision can add or remove a suspicion or an allocated loss and never withdraw a proved deviation. That is not stated.
- *Verified.* With quorums of 2f+1 out of 3f+1, two conflicting certificates prove that at least f+1 signers equivocated, so a fork yields proof against named witnesses even when the fault bound has failed. Polygraph defines this (the verifier's output contains at least t0+1 processes, all Byzantine) and also shows that "stronger forms of the problem including a fixed time detection are impossible to solve": https://eprint.iacr.org/2019/587 and https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.DISC.2020.45. This is the published form of the round-5 finding against a challenge period, and it is the mechanism that would support R2's clause "including when a neighbour's witness bound has failed". IA-CCF, which the plan cites, also assigns blame "regardless of the number of misbehaving replicas" by auditing signed contradictory statements: https://www.usenix.org/conference/nsdi22/presentation/shamis. The table lists the third option with no advantage or cost.
- *Verified at the level of secondary sources.* A trusted counter or attested log at each witness prevents equivocation and keeps f of 2f+1: TrInc, https://www.usenix.org/legacy/event/nsdi09/tech/full_papers/levin/levin_html/index.html, which describes A2M this way. It adds a hardware trust anchor. It is a real fourth option and is absent.

**Change.** Add to the second option "requires a delivery bound between a unit's witnesses, added to the fixed assumptions". Add to the first "revision never withdraws a proved deviation". Add to the third "a fork proves f+1 named equivocators; costs a third more witnesses per unit, counted in step 4". Add the trusted-counter option with its anchor. Cite Polygraph for the impossibility of fixed-time detection in the finality row.

### M6. D2 omits the reading the rest of the plan needs, and its only case cannot tell the two listed readings apart

**Text attacked.** D2 options: "Some behaviour of the environment allows it; or the unit can ensure it whatever the environment does". Case: "Three obligations of which any two can be met and all three cannot."

**Why (plan-internal and judgement).**
- Under the second reading, with the fixed assumption that links between units have no delivery bound and with discharge at the obligee's witnesses (D10), no unit can ensure any obligation owed to another unit, because the environment can delay delivery past any deadline. Every such obligation is unsatisfiable alone. The same holds for the officer's approval and the vendor's delivery.
- Under the first reading, two norms conflict only if no environment at all lets both be met. A unit can then be held to a set of norms that it can meet only if the vendor and the links cooperate, and the conflict is found only when they fail.
- The reading in between is "the unit can ensure it against every environment that satisfies a declared assumption". The plan already has the pieces: the performance envelope, the vendor's assume-guarantee contract, and D7's "named assumption about the environment for each obligation". Round 6 (fable M6) named the two extremes; the table copied them and nobody added the middle.
- The three-obligation case contains no environment, so both readings give the same answer on it. D2's known-problems cell is about the cited selection procedure and says nothing about the two options.
- *Could not verify in this round:* the cost of the second and middle readings is realizability, doubly exponential for linear temporal logic (Pnueli and Rosner, https://doi.org/10.1145/75277.75293, from memory, also unverified in round 6). Realizability under environment assumptions has a cubic-time fragment (Bloem et al., https://doi.org/10.1016/j.jcss.2011.08.007, from memory). The frozen grammar should be chosen with that in view.

**Change.** Add the third option. Add two cases: an obligation whose discharge crosses a link with no delivery bound; and two norms that are jointly satisfiable only if the vendor delivers inside its contract. Put the cost of each reading in the known-problems cell.

### M7. D1 reports the round-6 finding for one option only, and has no case for a conflict caused by the environment

**Text attacked.** D1 options: "Evaluate when the later norm takes effect; or define compliance as staying among states from which the selected norms remain jointly satisfiable...; or evaluate at the current state and attribute causes". Known problems: the escape by waiting, and "The second option... costs more".

**Why (plan-internal).** Round 6 (plan_attack_fable_r6.md, H1) gave a problem for each of the three designs. The table keeps two.
- First option, omitted: "Conflicts that arise later are then not decided by the selection". Concretely: no conflict when the norms take effect; vendor V1 then fails through no act of the unit; only the forbidden V2 can deliver. Both norms stay in the effective set. A controller can refuse and cannot compel, so the prohibition wins whatever the priorities say. A higher-priority obligation then loses to a lower-priority prohibition, which is the inversion D6 is trying to rule out for caps.
- Third option, omitted: attributing causes "imports R2's causal definition into the definition of the effective norm set, which the R3 local check would then depend on". That makes R3 depend on R2, and the stop rule treats them as independent.
- Both listed cases are conflicts the unit causes or a conflict of one. No case has a conflict caused by the environment after the norms took effect.

**Change.** Add the two omitted problems. Add the case "the compliant vendor fails and only a forbidden vendor remains, with the obligation outranking the prohibition", with a required outcome and a named bearer.

### M8. D6's first option keeps the defect it is listed as fixing, and the option that removes it is missing

**Text attacked.** D6 option: "A tier above all norms, declared only by an authority over the resource's owner, with a conflicting higher-priority obligation recorded as overridden". Known problems: "Exempt with no further rule: a lower authority's cap defeats a higher authority's obligation".

**Why (plan-internal).** Under the first option the group, which ranks below the ministry, can set a cap of zero on the subsidiary's budget. Every payment obligation from the ministry, the regulator or the state is then "recorded as overridden". Recording the override does not change who won. Any authority over the owner holds a veto over every higher authority's spending obligations, so the "resolved set of norms" in the first section is not determined by priority. The known-problems cell describes a variant ("with no further rule") that is neither listed option, so the first option appears to have no known problem. The second option's problem ("contradicts R4") is real.

The missing option: a cap is absolute against commitments and against norms of equal or lower priority, and can be displaced only by an explicit cap change issued by an authority that outranks its declarer, processed as a rule change through D11's ledger. That keeps R4 (the cap never fails silently) and keeps priority (the higher authority can prevail, by changing the cap and not by issuing an obligation).

**Change.** Add that option. In the known problems, say of the first option "any authority over the owner can veto higher obligations by a cap; priority inversion is recorded, not prevented". Give the first case a required outcome.

### M9. D9's second option empties R2's completeness clause unless more is said, and "eventual delivery" is used by the plan but no longer assumed

**Text attacked.** D9: "within a stated bound after the evidence is logged at the unit that computes the verdict". R2: "every covered violation caused by a deviation yields a proved-deviation verdict naming the deviators, in the form D9 fixes". Assumptions: "links between units and to vendors do not" have a delivery bound. Step 4: "the worst case under eventual delivery reported as zero". D13: "Under eventual delivery alone".

**Why (plan-internal).**
- With the second option the clock starts when the computing unit logs the evidence. Evidence that never arrives, or that a deviating computing unit never logs, starts no clock. The clause then says: the verdict function, given the evidence, returns the deviators within a compute bound. That is a statement about a function and a processing time. It makes no promise that a covered violation is ever judged. The refuter "a verdict later than the bound" can fire only on slow computation.
- Version 6 assumed "every message is eventually delivered". Version 7 deleted the sentence, following round 6, but step 4 and D13 still reason "under eventual delivery", and D9's first option is false without it. The assumption is neither stated nor withdrawn.
- The first section lists what a positive result would not establish. "A covered violation need never receive a verdict" is not there.

**Change.** State under D9's second option what remains of completeness, and add to the first section: "no bound, and under the bounded form no guarantee, on when or whether a covered violation is judged outside the performance envelope". Either restore eventual delivery as a named liveness assumption used only by named claims, or remove the two remaining uses. Add the case "the computing unit withholds logging" (see M4, verdict computation).

### M10. The freeze has no floor, so R2, R5 and R1 can each be frozen into a statement that cannot fail

**Text attacked.** Freeze list: "a bound on summary growth in terms of activity at the unit's boundary"; "a class of useful self-amendments that must be accepted"; "the complexity class that counts as success". R2: summaries carry "the boundary trace prefix the counterfactual needs". R5 fallback: "amendments are limited to a syntactic class that cannot touch the entrenched terms and the claim about self-reflection is withdrawn."

**Why (judgement).**
- *R2.* A bound that is linear in boundary activity lets the summary be the whole boundary trace. If deviation is defined against interface guarantees, as in the cited component-causality work, the chain verdict is then a function of the boundary traces by definition, and "without reading inner logs" is true at once. The refuter "summaries that exceed the frozen growth bound" cannot fire. The claim has content only if the summary is smaller than the boundary trace by a stated factor, or of size fixed per guarantee.
- *R5.* If the frozen "class of useful self-amendments" is itself a syntactic class that does not touch entrenched terms, completeness is immediate and success is the same outcome as the stated failure fallback. Nothing requires the class to contain an amendment that touches an entrenched term.
- *R1.* The success class is chosen after the fragment. Nothing requires it to be below the trivial bound for the finite-state encoding, although the refuter says that encoding "does not count".
- The stop rule that guards against definitional results ("refuted by some benchmark variant when one named hypothesis is dropped", "not an instance of a step-0 definition") runs after the proofs. It should run at the freeze.

**Change.** Add to the freeze: the summary bound must be sublinear in the boundary trace or constant per guarantee; the R5 class must contain the valid amendment of the amendment rule in variant 10 and at least one amendment that changes what the witnesses replay with the appointer's consent; the R1 class must be strictly below the bound of the finite-state encoding. Apply the "drop one hypothesis" and "exhibit the instance" tests to each frozen statement as part of the step-0 exit test.

### M11. The scale failure conditions are relative to bounds the authors declare, and the anchor is exempt because it is not a unit

**Text attacked.** "The scale claim fails if any unit cannot be sharded at the target workload, any bound exceeds its declared growth". "The anchor's load, including witness appointment and any priority table, is counted." First section: "Witnesses, the anchor, the time source and the identity issuer are components and not units". D3: "the anchor's priority table is a dependency of every unit and must be versioned and leased".

**Why (plan-internal).** "Declared growth" has no ceiling. A bound declared linear in the population and met is a pass. The sharding condition applies to units; the anchor, the identity issuer and the time source are not units. Under D3's total order every unit leases the anchor's table, and under the creation rule the anchor may appoint witnesses for every new unit. At 10^10 units that is load proportional to population on a component the failure condition does not cover. The same applies to witness sets: 2f+1 or 3f+1 outside parties per unit and per shard, each holding the full log and replaying the controller.

**Change.** State the ceiling: "the scale claim fails if the load on any single controller, shard, witness or anchor component grows faster than polylogarithmically in the population at fixed per-unit activity, or if any anchor component cannot be partitioned". Count witnesses per unit in the population.

### M12. Two repairs from earlier rounds were dropped while the mechanisms they repaired stay in the fixed architecture

**Text attacked.** Evidence: "Controllers are deterministic given their recorded inputs; requests from humans and learned agents, event order and clock readings are inputs". Useful behaviour: "A controller that rejects everything inside the envelope fails."

**Why (plan-internal).**
- *Clock readings.* Round 4 (fable M8) showed that replay cannot catch a false clock reading, because the reading is a replayed input. Version 6 answered: "Witnesses reject a log entry whose recorded time differs from their own receipt time by more than the link bound plus the clock skew." Version 7 keeps clock readings as inputs and deletes the check. A deviating controller can then record an early reading and authorise after its lease has expired, or record a late reading for an obligor's performance, and replay passes. The lease argument and D10 both rest on recorded times.
- *Time blocking.* Version 6 noted that "the conditions that stop a component blocking time matter for whether a deadline can be met vacuously". That sentence is gone. Under trace inclusion a unit that stops time meets every deadline. The useful-behaviour rule catches a controller that rejects everything and does not catch one that blocks time. The timed input/output automata framework handles this with a receptiveness condition, under which an automaton does not block the passage of time (verified at the level of the monograph's summary: https://dspace.mit.edu/handle/1721.1/30403; I could not fetch the theorem numbers).

**Change.** Restore the witness time check in the Evidence paragraph. Add "every unit is receptive (does not block time), as a hypothesis of every deadline claim" to the fixed architecture, and name the condition in the reuse ledger.

---

## LOW

- **L1. "Stable across the last four rounds" is not accurate for the Evidence paragraph (plan-internal).** Witnesses that hold the log, their outside appointment, the bound on the controller-to-witness link and the gateway unit first appear in version 4. They have been reviewed three times (rounds 4 to 6) and were attacked each time: r4 fable M3 and M13, r5 fable H2, r6 fable M10. Say "three rounds" and "attacked in each; open points are in D8".
- **L2. D9's known problem misplaces the assume-guarantee point (plan-internal; source not re-fetched in full).** The cell says the eventual form "is not covered by an assume-guarantee rule that discharges only safety assumptions". Round 6 said something different: the *assumption* "every message is eventually delivered" is a liveness assumption and cannot be discharged by that rule. The published rule restricts assumptions, and the plan's own Controlled paragraph in version 6 put the restriction there ("the discharged assumptions are safety properties", https://lamport.azurewebsites.net/pubs/abadi-conjoining.pdf). Reword to "it needs a liveness assumption about delivery, which the rule cannot discharge".
- **L3. Citations.** "fork consistency and fail-aware storage (Li et al.)" links only the fork-consistency paper; fail-aware storage is a different paper by different authors (https://people.csail.mit.edu/idish/ftp/faust-sicomp.pdf, as given in round 6, not re-fetched). Two different "Li et al." appear in one sentence. The link given for conformance hierarchies is "Dependable Management of Untrusted Distributed Systems"; its abstract mentions neither conformance hierarchies nor law-governed interaction (verified: https://arxiv.org/abs/1403.4321). The plan admits this; until the paper is read, R3 should not list it as a baseline to go beyond.
- **L4. Missing from the freeze list.** The performance envelope is used by the useful-behaviour rule, the availability floor and D9, and version 6 froze it; version 7's list omits it. R4's suggested content ("a limit on how long new spending is blocked after a cap is raised") cannot be proved under the fixed assumptions, because D11 keeps an expired allocation reserved "until an authenticated final balance arrives" and no delivery bound is assumed between shards of one unit. State the limit as holding inside the envelope, or add a bound between a unit's shards.
- **L5. D10 has questions where the other rows have options, and it depends on D8's quorum.** "Latest presented time" is stated for f+1 receipts of 2f+1. Under D8's third option the rule becomes the (f+1)-th smallest of the presented times. Say so. Separately, the benchmark's regulator has "no edge to or from the ministry" and no stated authority above it, yet "all rule authority descends from one anchor"; state the regulator's path to the anchor, since D3's first case depends on it.
- **L6. Verdict computation has a known complexity floor (verified at the level of a search summary).** R2's verdicts are "sets of minimally sufficient sets of parties". Deciding actual causation under the modified Halpern-Pearl definition is D^P-complete (Halpern, https://arxiv.org/abs/1505.00162, Theorem 5.1 as reported). D9's "stated bound" and step 4's per-decision cost need a frozen bound on verdict computation in the size of the summaries; the R2 freeze list has none.

---

## The "known problems" column against the earlier reviews

| Row | Faithful to the reviews | Defect |
|---|---|---|
| D1 | Partly | Problems of the first and third options omitted (M7) |
| D2 | Source statement correct (Liao et al., checked in round 6) | Says nothing about the two options; middle option missing (M6) |
| D3 | Yes | Rank assumed in a case that must also run under the partial order (M4) |
| D4 | Mostly | Omits that option one is the baseline and that option two needs a wider edge check (M2) |
| D5 | Yes | No finding |
| D6 | Describes a variant that is not a listed option | First option keeps the inversion (M8) |
| D7 | Yes | Effect on R3 not stated (M2) |
| D8 | Yes for forks and finality | Three decisions in one row; second option unreviewed (M4, M5) |
| D9 | Mostly | L2; consequence of the bounded form not stated (M9) |
| D10 | Yes | L5 |
| D11 | Yes; all four cases check arithmetically | L4 |
| D12 | Yes | Collides with step 2 (M3) |
| D13 | Yes | No finding |

## Targets against known results

- **R1.** Not known true or false until the fragment is frozen. Consumable quantities with revocation can encode counters with resets, for which reachability is undecidable (verified at abstract level: Dufourd, Finkel and Schnoebelen, https://researchr.org/publication/DufourdFS98); the refuter already allows for this. Needs M10's floor.
- **R2.** Soundness and the composition claim can fail. Completeness depends on M9. The composition claim needs M10's floor. Nothing cited proves it: the compositional causality result covers only non-communicating systems, as the plan says (verified: https://arxiv.org/abs/1608.07880).
- **R3.** Content depends on D4 and D7 (M2). The conflict-detection half can fail.
- **R4.** Honest as now worded: it is called a lemma unless the frozen statement has content beyond the escrow method.
- **R5.** Soundness can fail. Completeness can be made trivial by the choice of class (M10).

## Gap to the goal

The narrowing is stated openly and is still worth testing. Deferral has not defined the problem away, provided M1, M2 and M10 are fixed; without them the plan can reach the end of step 0 with every case "passed", R3 reduced to its baseline, R4 a lemma, and R2 and R5 frozen in forms that cannot fail, and still report a consistent set of decisions. Two lines should be added to the first section: that a covered violation may never be judged outside the envelope (M9), and that a cap, under D6's first option, lets any authority over a resource's owner veto higher authorities (M8).

## Order of work

1. Add the missing rows (M4) and options (M5, M6, M8) before step 0 starts; they are additions to the table, not design work.
2. In step 0: write required outcomes, then choose, then run the joint cases in an executable model, within a fixed number of rounds (M1).
3. Resolve D12 against step 2 (M3) before the paper stage, since the paper stage is done in that core.
4. Apply the non-triviality floor at the freeze (M10), not after the proofs.
5. The stop rules of step 1 are otherwise sound and in the right place.

## Verdict

Sound enough to start step 0. No target is known false, impossible or already proved, and no claim about prior art is wrong in a way that invalidates a step. The fixed architecture does not keep a mechanism that an earlier round broke, with two exceptions that are repairs lost in the rewrite (M12), and its claim of stability overstates the record for the witness scheme (L1). The table is mostly faithful to the reviews; its omissions are in D1, D4 and D6, and six contested mechanisms have no row. The main risk has moved from the plan's text to the step-0 gate, which as written would not have caught any of the interaction errors found in rounds 3 to 6.

CRITICAL: 0  HIGH: 0  MEDIUM: 12  LOW: 6

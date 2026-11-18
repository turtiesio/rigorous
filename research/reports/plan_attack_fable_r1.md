# Adversarial review of "Plan for an integrated foundation" (fable, round 1)

Target: `/root/mic/reports/Integrated foundation plan.md`. Reviewed against the survey it follows from and against primary sources fetched during this review.

**How to read the evidence labels.** "Verified" means I fetched the source in this review and the text supports the statement. "From memory" means I am confident of the reference but did not fetch it now. "Could not verify" means the fetch failed. "Judgement" means the argument is mine and rests on the plan's own text.

**Summary.** The plan is honest about its limits and its non-goals are well chosen. Its main defect is that the theorem list cannot produce a negative answer. Under the plan's own design (every crossing action mediated, everything recorded, logs assumed intact), T2, T3, T5 and T6 hold by construction, T1 is textbook, T4 is either true by definition or false on the plan's own worked example, and T7 as worded is undecidable. The decision "does any delegation or norm composition theorem hold?" therefore cannot come out "no", and the plan cannot find out what it sets out to find out. The remaining serious problems are that the boundary-controller design conflicts with graph-shaped containment, the scale test is either vacuous or failed by the plan's own example, and the default core has neither time nor dynamic creation although three modules need them.

---

## CRITICAL

### C1. The theorem targets cannot fail, so the plan cannot answer its own question

**Text attacked.** Step 4 table and: "**Decision:** does any delegation or norm composition theorem hold?"; "If T1 to T3 and T7 are the only theorems that go through, the result is a tidy repackaging of known work. The plan counts that as a negative result"; "T4 ... This is the composition law the survey found missing".

**Why it is wrong.** The plan's purpose is to find out whether integrating authority, obligation and liability yields theorems that the separate substrates do not. For that, the targets must be statements that could turn out false. As written they are not. Taking each in turn:

- **T1** is Lynch and Tuttle's composition result, as the plan says. Verified: the cited paper states compatibility (disjoint outputs, private internal actions, each action in finitely many components) and the compositional results for `solves` and `implements` (Propositions 8 to 11). https://groups.csail.mit.edu/tds/papers/Lynch/CWI89.pdf
- **T2** says no principal "acts with authority that is not derivable from recorded delegations, constitutions and declared amplifications". The design makes every crossing action pass through a controller that evaluates exactly that derivation. So T2 is the statement that the monitor implements its own policy. It is a monitor-correctness lemma. The question that could fail is the safety-analysis question: can principal p ever come to hold right r under the delegations, constitutions and amplifications that the rules allow in future? The plan names that question only as a risk. It is the one HRU proved undecidable in general, and Li, Mitchell and Winsborough showed it decidable (mostly polynomial) for the RT trust-management family under restriction rules on policy change (verified from the abstract and secondary summaries; theorems not read): https://www.cs.purdue.edu/homes/ninghui/papers/analysis_oakland03.pdf
- **T3** ("balances ... consolidate across nested units by addition") is the statement that a sum over a partition is a sum. It is true for trees by arithmetic. It is false as stated under the plan's own containment graph: a principal who belongs to two units has its balance counted in both, so parent totals double count (judgement; real consolidation accounting handles this with elimination entries and proportional interests, which the plan does not model).
- **T5** with rule compliance defined as inclusion of permitted behaviour is transitivity of set inclusion. With "Prevent" as the enforcement class it holds because each controller blocks non-conforming actions. See H4 for the prior art that already does this.
- **T6** says a change "accepted by the stratified checker preserves T2 and T3". If T2 and T3 are invariants of the fixed mediation kernel, every rule change preserves them. If rules can alter them, the checker is defined as accepting only preserving changes. Either way the theorem is definitional. The hard content, in-flight commitments made under the old rules, appears only in the risk column and not in the statement.
- **T4** is the only target the plan presents as new. It says that if a commitment is violated and logs are intact, "the calculus identifies at least one responsible principal on the delegation chain and never blames a principal who complied". "Responsible" and "complied" are not defined, and the two readings give opposite results.
  - Reading A (strict liability): the responsible party is whoever currently holds the commitment as debtor. Then T4 is true by definition of "violated" for chains, trees and graphs alike, and proves nothing about composition.
  - Reading B (fault or causation): a principal is responsible only if it deviated from a norm that applied to it. Then T4 is false on the plan's own worked example. The government lawfully lowers the cap mid-flight. The agent's purchase is then blocked by a controller acting correctly, and the commitment to the external service is violated. Every principal complied with every rule in force when it acted. "At least one responsible" and "never blames a principal who complied" cannot both hold. This is the problem of many hands (Thompson 1980, from memory: https://doi.org/10.2307/1954312).
  - The protocol-accountability literature the plan cites gets non-trivial theorems only because it fixes a causal definition and a notion of deviation. Verified from the abstract: Künnemann, Esiyok and Backes require a protocol to "identify all misbehaving parties", where misbehaviour is a deviation that causes a violation, which is stronger than the plan's "at least one". https://arxiv.org/abs/1805.10891 (the plan's CISPA link returned HTTP 403). The fairness-and-completeness formulation that T4 paraphrases is due to Küsters, Truderung and Vogt, not cited by the plan: https://eprint.iacr.org/2010/236 (existence and abstract verified; definitions not read).
  - Blame assignment that composes over contract conjunction and disjunction already exists for trace-based multiparty contracts. Verified from the abstract: Hvitved, Klaedtke and Zălinescu, "A trace-based model for multiparty contracts", where "every breach is attributed to a set of parties" and blame assignment is shown compositional. https://dblp.org/rec/journals/jlp/HvitvedKZ12 . T4 on chains under Reading A is a special case of that kind of result.
- **T7** is not provable as worded (see H5).

So the outcome is fixed in advance: T1, T2, T3, T5, T6 and T4 under Reading A all "go through", and the plan reports that the integrated foundation exists. The stated failure condition "the integration adds nothing" can never fire, because it is triggered only by theorems failing.

**Severity reasoning.** The plan says "Each is a target, not a result", and step 0 promises formal statements. That does not rescue it. Nothing in step 0 or step 4 requires a target to be non-trivial, and the single decision point on which the answer to the survey's open question depends has only one possible outcome. This meets "the plan cannot achieve its stated purpose as written".

**Change recommended.**
1. For each target, write down before any proof work (a) the formal statement, (b) a candidate counterexample system in which it would fail, and (c) what the theorem says beyond the correctness of the monitor. Drop or demote to "lemma" any target for which (b) is empty.
2. Replace T2 by the safety-analysis theorem: for the chosen policy fragment with constitution and amplification, "can p ever obtain r" is decidable, with a complexity bound. State which of constitution and amplification breaks the Li et al. decidability proof, if either does.
3. Replace T4 by two theorems with a fixed causal definition of responsibility: a completeness theorem and a no-false-blame theorem, each over a stated class of deviations, and a third statement classifying the violations where nobody deviated (rule change, external failure, joint causation) and saying who bears the loss by a declared allocation rule. The interesting composition question is then whether per-unit verdicts compose into the chain verdict without re-reading inner logs.
4. Put in-flight commitments into the statement of T6: a rule change either preserves every commitment created under the old rules or produces a recorded, attributed violation with a named bearer.
5. Add a real failure condition: the plan reports "integration adds nothing" if every proved theorem is a consequence of one module alone plus monitor correctness.

---

## HIGH

### H1. "Every crossing action passes through the boundary controller" conflicts with graph-shaped containment and with the chosen core

**Text attacked.** "Every composite has a boundary controller: a distinguished component through which every action crossing the boundary passes. The controller is an ordinary unit at the next level. Containment is a graph and not a tree, because one principal can belong to several units."

**Why it is weak.** Three separate problems (all judgement, resting on verified properties of the core).

1. *A shared member is an unmediated channel.* If principal p belongs to U1 and U2, p's local state is inside both boundaries. Whatever p learns or holds in U1 is available to p's actions in U2 without any action crossing either boundary. So "every action crossing the boundary passes through the controller" is satisfied while authority-relevant information and commitments move between units freely. Confinement (T2) and any observation declaration (module 5) then hold only for explicit tokens. The survey itself lists capability confinement as composing only when "authority follows references".
2. *The core cannot express it.* Verified: I/O automata composition requires disjoint output sets, "at most one system component controls the performance of any given action", and composition is a product that is hidden level by level, which is a tree. https://groups.csail.mit.edu/tds/papers/Lynch/CWI89.pdf . A component cannot be a member of two composites that are hidden separately. The plan lists this under "H1 fails if", but then every theorem with "shared membership" in its risk column (T1, T4, T5) depends on machinery that the default core is already known not to have.
3. *Regress and undefined "next level".* The controller "is an ordinary unit at the next level". If it is a composite (a board, a ministry) it needs its own boundary controller. If containment is a graph, "the next level" is not unique.

**Change recommended.** Decide now, before the bake-off: model a principal as an identity plus a set of role instances, each role instance in exactly one unit, so that containment of role instances is a tree. State explicitly that flows between role instances of the same identity are unmediated and belong to the Bound or Account class. Prove the Prevent-class theorems for the tree only. Say that the controller of an atomic-controller unit is atomic, and give the termination argument for composite controllers. Prior art to consult for shared components with membranes: the Fractal component model (sharing and membrane controllers verified from secondary sources; https://arxiv.org/pdf/1105.6010v2.pdf) and its Alloy and Coq formalisations.

### H2. The scale test is either vacuous or failed by the plan's own worked example, and it ignores load on upper controllers

**Text attacked.** "The test is locality: the cost of one governance decision, and the number of parties that must coordinate for it, do not grow with the total population"; "Show by argument over the calculus that neither depends on total population, given bounded delegation depth and bounded unit size"; "**Scale claim fails if** any decision type has cost that grows with population, or if the coordinated class dominates the workload."

**Why it is weak (judgement, with verified support).**

1. *Vacuous under its own assumptions.* With unit size at most b and depth at most d the population is at most b^d. A claim that cost "does not depend on total population" under hypotheses that bound the population says nothing about growth. To let the population grow, either depth grows as log N, in which case a purchase mediated by every enclosing controller costs O(log N) and the test as worded fails, or unit size grows, which the argument excludes. Real units at the upper levels are not bounded: a nation has of the order of 10^8 to 10^9 direct members.
2. *Failed by the worked example.* "Midway, the government lowers the spending cap, and the change has to propagate." That decision must reach every descendant controller before its pre-announced effective time. Its cost is proportional to the subtree, which is the population. Step 6 tests "each decision type in the worked example", so the scale claim fails on the plan's own criterion. The alternative is that controllers pull the current rule version at decision time, which puts every ancestor on the read path of every decision. This is the freshness problem Zanzibar solves with globally ordered timestamps and "at-least-as-fresh" tokens (verified from secondary descriptions of the USENIX ATC 2019 paper: https://authzed.com/learn/google-zanzibar ; slides https://www.usenix.net/sites/default/files/conference/protected-files/atc19_slides_pang.pdf). The plan's "pre-announced time" plus synchronised clocks is a version of this, but it needs a stated notification bound and a fail-closed rule for controllers that have not heard.
3. *Per-decision cost is the wrong metric.* A controller at level k is on the path of every crossing action of everything beneath it. Per-decision cost can be constant while the load on the nation-level controller is proportional to N. Replicating that controller turns its state (rules, balances, logs) into replicated state, and the coordination question returns inside the controller.
4. *No thresholds.* "Dominates", "stays flat" and "increasing sizes" have no numbers, and the workload is not defined. The simulation cannot fail.

**Change recommended.** Replace the locality test by three stated bounds as functions of N, depth d and fan-out b: per-decision cost, load per controller per unit time, and rule-change propagation cost and latency. Declare in advance acceptable growth (for example O(d) per decision, O(subtree) messages for a rule change with a stated maximum latency). Fix the workload mix and the maximum share of coordinated decisions before measuring. Add the fail-closed behaviour of a controller that has not received the current rule version, and count the resulting unavailability.

### H3. The default core has no time and no dynamic creation, and three modules and four theorems need them

**Text attacked.** "The core is input/output automata ... composition of compatible automata, hiding, and refinement as trace inclusion"; module 4 "a deadline and a sanction"; module 5 "Clocks are assumed synchronised to within a stated skew"; module 6 "take effect at a pre-announced time"; module 2 "constitution, where a recognised procedure creates authority that no delegator held".

**Why it is wrong.**

- *Time.* Verified: the Lynch–Tuttle model in the cited paper has no notion of time; its only progress notion is fairness, which the plan excludes ("liveness excluded"). A deadline is not expressible in the untimed model. In a timed model a missed deadline is a safety violation, but composition then needs conditions that rule out components that block time (from memory: Kaynar, Lynch, Segala and Vaandrager, *The Theory of Timed I/O Automata*; Gawlick, Segala, Søgaard-Andersen and Lynch, "Liveness in timed and untimed systems", which the DIOA paper cites). Without time and without fairness, safety-only trace inclusion admits the unit that never emits an output as a refinement of any unit whose specification only forbids. T1's substitutivity would then let the subsidiary be replaced by a unit that does nothing, and no theorem about commitments survives the replacement (judgement).
- *Dynamic creation.* Verified: the cited paper says it uses infinite products "in order to model systems that can create processes dynamically", meaning all units exist from the start. Chartering an organisation, spawning an agent, opening an account and dissolving a unit are creation and destruction. The model that adds these is Dynamic I/O Automata (Attie and Lynch 2016). Verified from its text: composition, hiding and renaming are monotonic for trace inclusion, but creation is monotonic only "under certain technical conditions", because otherwise a system "can create the replacement A in more contexts than those in which Y creates B". The paper calls this "inherent in situations where the creation of new automata can depend upon global conditions". https://arxiv.org/pdf/1604.06030 . So T1 does not extend to a calculus with constitution without those conditions. DIOA has no time, so the plan needs a combination that, to my knowledge, has no published substitutivity theorem (could not verify the absence).
- Probability and mobility are excluded by the plan's "Not targeted" list and are not held against it here.

**Change recommended.** State H1 as "timed I/O automata with dynamic creation" and list the two side conditions (no time-blocking; creation correlated with the trace) as explicit hypotheses of T1. Add "T1 fails to extend to creation plus time" as a kill criterion for the default core. Add timed and dynamic I/O automata and interface automata (de Alfaro and Henzinger, from memory) as named bake-off entries, since they address exactly the two known costs the plan lists (input-enabledness) or omits (time, creation).

### H4. T5 in its "Prevent" form already exists, and the cited result is about designs, not behaviour

**Text attacked.** "T5 | Norm conformance down a hierarchy: if each unit's rules comply with its parent's, the leaf's behaviour complies with the root's ... | Prevent and account | Multi-level compliance checking (King et al.); law-governed conformance hierarchies".

**Why it is weak.**

- Verified: Law-Governed Interaction organises laws into conformance hierarchies in which "if N ≺ M then N conforms to M", subordination is transitive, and "the hierarchy is enforced by its very construction. That is, the very definition of a law N as subordinate to M, prevents N from violating the restriction imposed by M". Each agent has a private controller that mediates all its interactions under the law. https://arxiv.org/abs/1403.4321 (Minsky 2014, section on conformance hierarchies; the construction is in Ao and Minsky, ESORICS 2003, not read). This is T5 under Prevent together with the plan's boundary controller. The plan cites it as "prior work to reuse" while listing T5 as open research.
- Verified from the abstract only (full text returned 403): King et al. define "compliance between concrete lower level institutions and abstract higher level institutions", which is a relation between institution designs at different levels of abstraction. T5 concludes something about "the leaf's behaviour". Passing from design compliance at each pair of levels to behavioural compliance at the leaf needs the abstraction mappings between vocabularies at successive levels to compose. That is the non-trivial content, and the plan does not mention it. Could not verify whether King et al. treat more than two levels or transitivity.
- A monitor cannot prevent the violation of an obligation; it can only detect a missed deadline (from memory: Schneider, "Enforceable security policies", execution monitors enforce safety properties only; https://www.cs.cornell.edu/fbs/publications/EnforceableSecPol.pdf). So the "Prevent" half of T5 covers prohibitions only, which is the plan's own fallback. The fallback is therefore the LGI result.

**Change recommended.** State T5 as: for norms expressed in different vocabularies at each level, connected by declared counts-as mappings, pairwise design compliance implies end-to-end compliance, with the conditions on the mappings. Treat the prohibition-only, same-vocabulary case as known (LGI) and as a baseline to reproduce. Add overlapping parents as a separate target.

### H5. T7 as worded asks for a decision procedure that is known not to exist

**Text attacked.** "T7 | Coordination classification: a static check sorts each declared invariant into 'needs no coordination' or 'needs an owner or committee' | Prove | Invariant confluence (Bailis et al.)"; ranked fourth most likely to succeed and counted among "integrations of results the survey reports as established".

**Why it is wrong.** Verified: Whittaker and Hellerstein state that "designing such a general-purpose decision procedure is impossible because determining the invariant confluence of an object is undecidable in general", and they give an interactive procedure and sufficient conditions instead. https://mwhittaker.github.io/publications/iconfluence_vldbj2020.pdf . Bailis et al. classify invariants by hand analysis for a fixed catalogue; they do not give a total static check (the plan's link is correct: https://arxiv.org/abs/1402.2237). A two-way total classifier therefore cannot be proved correct for any invariant language expressive enough to state the plan's own invariants over delegation chains. Separately, the two classes are too coarse. The standard treatment of the plan's main example, a non-negative shared budget, is neither "no coordination" nor "owner or committee" per decision: rights to spend are split in advance so that decisions are local until a share runs out (from memory: O'Neil's escrow method, 1986; bounded counters, Balegas et al., https://arxiv.org/abs/1503.09052). Supporting the plan: asset transfer from a single-owner account needs no consensus, and an account shared by k owners needs k-consensus (verified: Guerraoui et al., https://arxiv.org/abs/1906.05574).

**Change recommended.** Reword T7 as a sound three-way check (provably coordination-free, provably not, unknown) over a named restricted invariant language, with the fraction of the worked example's invariants that land in "unknown" as a reported measure. Add a third operational class, "local after pre-allocation", and connect it to T3. Move T7 out of the "established" group.

### H6. There is no adversary model, and the controller, which is the party most in need of accountability, is assumed honest

**Text attacked.** "if a commitment is violated and the logs are intact"; "Run the worked example with real language-model agents, scripted humans and an unmodified external service"; the worked example as a whole.

**Why it is weak (judgement).** Every Prevent theorem trusts the boundary controller, and T4 assumes the logs, which the controllers keep, are intact. In the procurement setting the realistic failures are on the part of the controller and its colluders. None of the following is in the example or the theorem list:

- A controller that omits or reorders log entries, or approves outside the rules. Accountable systems handle this with witnesses that hold signed commitments to each node's log (from memory: Haeberlen, Kuznetsov and Druschel, PeerReview, SOSP 2007). The plan cites "tamper-evident logs†" but assigns nobody to audit them.
- Collusion between the agent and the external service, and between a shared member's roles (H1).
- Splitting purchases to stay below the human-approval threshold. Blocking this needs a history-dependent aggregate over the team's purchases, which is shared state and so falls into T7's coordinated class for the most common decision in the example.
- Disputes about facts outside the system. Whether the external service delivered the compute decides whether the commitment was discharged or violated, and no controller observes it. T4 needs an adjudicator and a rule for contested evidence. None is modelled.
- Revocation after spending (claw-back), and dissolution or insolvency of a unit holding undischarged commitments. The plan names revocation as the first place to look for H2 failure, but the worked example has no revocation of an already-used grant and no unit that ceases to exist.

"Not targeted: incentive compatibility" is a stated limitation and is not held against the plan. The absence of any Byzantine or colluding party in the benchmark is not stated, and it means the example cannot break the design.

**Change recommended.** Add a threat model to step 0: which units may deviate arbitrarily, which may collude, and what each theorem assumes about them. Add to the worked example: one dishonest controller, one threshold-splitting attempt, one disputed delivery with a named adjudicating unit, one claw-back, and the dissolution of the subsidiary with a commitment outstanding. Restate T4 with "logs intact" replaced by "at least one honest witness per unit holds log commitments", or state plainly that controller misbehaviour is outside every guarantee.

### H7. The bake-off cannot falsify H1, because the benchmark does not contain what would break it

**Text attacked.** "**H1 fails if** another candidate states and proves the same theorems with a smaller trusted base, or if nesting with shared membership ... cannot be expressed in the baseline without ad hoc machinery"; "Encode the worked example (step 3) at small size in four candidates"; "The example exercises every level ... delegation across four hops".

**Why it is weak (judgement).**

- The worked example is a single chain: government, enterprise, subsidiary, team, agent. No principal belongs to two units. It contains no constitution (no vote or charter), no amplification, no creation or dissolution of a unit, and no conflict between norms of overlapping units. So the bake-off never exercises the stated H1 failure condition, two of the three authority operations in T2, or the main risks of T4 and T5.
- "States and proves the same theorems" cannot be evaluated at step 2 of the order of work, since the theorems are proved at steps 3 and 4. What will be compared is one person's estimate of provability.
- "Smaller trusted base" is not comparable across a set-theoretic automaton model, a categorical framework, a graphical calculus and an executable logic without a common measure, and the plan defines none. "Ad hoc machinery" is undefined.
- The candidates are of different kinds, and the list omits the nearest competitors to the default: interface automata and interface theories (non-input-enabled, with refinement that is contravariant on inputs), assume-guarantee contract theories, and timed and dynamic I/O automata (H3).
- One author, who holds the default hypothesis, does all four encodings.

**Change recommended.** Extend the example before the bake-off with: a human who is a member of the team and of an external audit body; a charter step that creates the subsidiary and a vote that constitutes an authority; one declared amplification (human approval combined with the agent's budget right); and two overlapping units with conflicting norms. Define the comparison measure in advance (for example lines of definitions plus the count and statement of axioms and trust anchors, in one proof assistant). Compare on T1 and one pencil-and-paper proof of T4 on a chain in each candidate, and nothing else. Add interface automata and contract theories as candidates.

### H8. The observation module has no theorem, and its natural theorem does not compose under the chosen refinement

**Text attacked.** "Each structure is typed state plus invariants on the core, with its own theorem"; "5. **Observation and time.** Each interface declares what each party can observe."; T1 "for safety properties".

**Why it is wrong.** Of the six modules, identity and observation-and-time have no theorem among T1 to T7, so the sentence is false of the plan itself. For observation the omission matters. "What each party can observe" is an information-flow statement. Such statements are sets of sets of traces, and they are not in general preserved by trace-inclusion refinement or by feedback composition. The survey states this and cites McLean and Clarkson and Schneider for it (survey, "Syntactic composition does not make properties compose": "Possibilistic security properties are preserved by product and cascade and not generally by feedback or refinement"; https://ecommons.cornell.edu/bitstream/1813/11660/4/hyperproperties-tr.pdf). So after any substitution licensed by T1, the observation declarations of the composite may be false. Accountability (T4) depends on who could observe what, so it inherits the gap. Privacy and confidentiality appear nowhere in the plan, including the non-goals.

**Change recommended.** Either add "no confidentiality or privacy guarantee; observation declarations are upper bounds on what is logged, not on what is learned" to the non-goals, or add a theorem for module 5 with a refinement notion that preserves it and say how that notion relates to the one used in T1. Give identity a theorem or call it an assumption and not a module.

---

## MEDIUM

### M1. The stratified rule-change design is under-determined and excludes self-amendment

**Text attacked.** "Changing them is an action governed by rules held one level up, or by a distinct procedure in the unit's own charter. The stratification is finite and ends in an anchor procedure outside the system."

**Why it is weak (judgement).** (a) In a containment graph "one level up" is not unique; two parents may give conflicting change rules. (b) For constituted units the power to change the rules sits with the members, below the unit, and not with a container above it. The "or" clause permits this but gives no condition for choosing between the two routes, or for what happens when both exist. (c) A finite stratification that ends outside the system forbids a charter whose amendment procedure applies to itself, which is how most real constitutions work (from memory: Suber, *The Paradox of Self-Amendment*). The goal says "self-reflecting"; the plan delivers "rules are controller state changed by someone else". (d) A change effective at a pre-announced time is safe only if every affected controller has received it by then, which is the propagation cost in H2.

**Change recommended.** State which route takes precedence and prove the two cannot conflict. Either allow one self-applicable amendment rule with a well-foundedness condition on versions (each amendment is checked by the rules in force at the previous version) or list "no self-amendment" under non-goals and say that this narrows "self-reflecting".

### M2. The enforcement-class scheme is applied inconsistently

**Text attacked.** The "Enforcement class" column of step 4.

**Why it is weak (judgement).** "Prove" is defined as "the actor supplies a certificate that a small checker verifies". T1, T7 and T6 are labelled "Prove", but they are metatheorems proved once by the authors, and no actor supplies anything. "Bound" is used by no theorem, although the learned agent and the unspecified external service are the two cases it was defined for. T5 is "Prevent and account" without saying which norms fall in which class (H4). The scheme also needs a rule for what a composite's class is when its parts have different classes; presumably the weakest, but the plan does not say, and without that rule "controlled" is not compositional.

**Change recommended.** Separate "how the theorem is established" from "how the property is enforced at run time". Add one Bound-class target for the external service (an assumed rate and price envelope whose violation is detected and attributed). State the composition rule for classes.

### M3. Exit tests and failure conditions have no thresholds

**Text attacked.** "Two independent reviewers can each say, for any proposed system, whether it meets each definition"; "H2 fails if the modules interact so much that each theorem needs the whole system"; "the coordinated class dominates the workload".

**Why it is weak (judgement).** The step 0 test does not require the reviewers to agree, names no sample of systems, and can be passed by any definitions. H2's "so much" and step 6's "dominates" are not quantities. H2 has no stated consequence if it fails.

**Change recommended.** Step 0: two reviewers independently classify ten named systems (including LGI, Cedar, a UCAN deployment, a DAO, and a national procurement regime) against each definition, with a stated minimum agreement. H2: fails if the proof of any module's theorem imports definitions from more than one other module; on failure the plan drops the "six separable modules" claim and reports the dependency graph. Step 6: numeric thresholds (see H2).

### M4. What the plan defines away from the goal

**Text attacked.** "Learned and human actors are treated as unspecified environments behind a boundary controller in the first version"; "No claim about legitimacy"; "ends in an anchor procedure outside the system".

**Why it matters (judgement).** These are stated openly, so they are not rated HIGH. Their sum should be stated in one place. Of the ten levels, humans, agents and external services are environments about which nothing is proved. Teams up to nations are composites whose only modelled behaviour is their controller's. "Self-reflecting" becomes state update by an outside party (M1). "Fractal" stops at an external anchor at the top. "10^10" becomes a locality argument that is currently vacuous (H2). "Controlled" becomes a list with evidence levels. A successful run of the plan would therefore show that a mediated message-passing system with typed budgets, a policy evaluator and a log has the expected invariants. It would not show whether a foundation for governing humans, agents and institutions exists, and the plan should say that this is what "the core" means.

**Change recommended.** Add a section "What a positive result would and would not establish", with the sentence above. Add at least one target that is about an unspecified actor (the Bound-class target in M2).

### M5. Order of work

**Text attacked.** "Order of work and decision points", items 1 to 7.

**Why it is weak (judgement).** (a) The only targets the plan regards as new (T4, T5) come fourth, after the bake-off and the mechanisation of four theorems that are known or definitional. A pencil-and-paper attempt at T4 on a three-node chain, in the default core, costs days and decides whether anything after it is worth doing. (b) The locality argument (step 6, first bullet) is a paper argument that needs only the definitions and the example. It is placed sixth; the H2 objections above can be found at step 1. (c) External adversarial review is last, after everything is built. (d) There is no decision point after step 4 that stops the work: if T4 to T6 fail, steps 5 and 6 still run. (e) H2 has a decision but no branch.

**Change recommended.** New order: definitions, threat model and extended example; paper proofs or counterexamples for T4 and T5 on the smallest case, and the paper locality argument; external review of those; then the bake-off; then mechanisation; then the monitor and measurements. Add a stop rule after the paper stage: if T4 has no non-trivial true statement on a chain, publish the negative result and stop.

### M6. Prior art that changes specific steps

Each item is named where it applies above; collected here with what it changes.

- Li, Mitchell and Winsborough, security analysis in trust management: the real content of T2. https://www.cs.purdue.edu/homes/ninghui/papers/analysis_oakland03.pdf (abstract verified).
- Hvitved, Klaedtke and Zălinescu, compositional blame assignment for multiparty contracts: baseline for T4. https://dblp.org/rec/journals/jlp/HvitvedKZ12 (abstract verified).
- Küsters, Truderung and Vogt, accountability as fairness and completeness: the definitions T4 paraphrases. https://eprint.iacr.org/2010/236 (abstract verified).
- Minsky, LGI conformance hierarchies with per-agent controllers: T5 under Prevent, and the boundary-controller architecture. https://arxiv.org/abs/1403.4321 (verified).
- Attie and Lynch, Dynamic I/O Automata: creation and its substitutivity conditions. https://arxiv.org/pdf/1604.06030 (verified).
- Whittaker and Hellerstein: undecidability of invariant confluence. https://mwhittaker.github.io/publications/iconfluence_vldbj2020.pdf (verified).
- Fractal component model: hierarchical components with membranes and shared sub-components, with Alloy and Coq specifications; the closest existing design to "boundary controller plus graph containment". https://arxiv.org/pdf/1105.6010v2.pdf (secondary source verified; primary specification not fetched).
- Object-capability confinement: the plan cites "confinement proofs†" for T2. The mechanised results I know of (from memory: Maffeis, Mitchell and Taly 2010; Devriese, Birkedal and Piessens 2016, https://lirias.kuleuven.be/retrieve/368761) are about language-level reference reachability and effect parametricity, not about policy-level delegation with constitution and amplification. Amplification is the operation that pure reference-graph arguments do not cover. They are weaker support for T2 than the table suggests. Could not verify the details in this review.

---

## LOW

### L1. The example does not distinctly exercise all ten levels

"The example exercises every level". "Program" and "resource" have no distinct role beyond the agent and the budget, and "nation" and "government" are the same unit in the example. Either map each of the ten levels to a named unit in the example or say which are merged.

### L2. "Smallest established baseline" is asserted

"it is the smallest established baseline" has no measure behind it, and the plan's own step 0 says "smaller" is "measured, not asserted". Remove the claim or measure it in the bake-off.

### L3. Mechanisations of the baseline already exist and should drive the proof-assistant choice now

From memory, not fetched: I/O automata have long-standing Isabelle formalisations (Müller and Nipkow's HOLCF/IOA), and TLA has Isabelle and TLAPS support. The plan defers the choice of assistant to after the bake-off "on the availability of existing libraries"; that availability can be surveyed in a day and should be an input to the bake-off, not an output.

---

## Claims checked and found correct

- I/O automata compatibility conditions, input-enabledness ("the automaton is unable to block its input"), and compositional results for `implements`: verified in the cited paper. https://groups.csail.mit.edu/tds/papers/Lynch/CWI89.pdf
- The Cedar citation: verified. arXiv 2407.01688 is Disselkoen et al., "How We Built Cedar: A Verification-Guided Approach", and it describes differential random testing of production code against an executable model. https://arxiv.org/abs/2407.01688v1
- Bailis et al. arXiv 1402.2237 is the invariant-confluence paper: verified by search result.
- King et al. is "Automated multi-level governance compliance checking", JAAMAS 2017, with a "sound and complete computational framework": verified from repository abstracts (https://researchportal.bath.ac.uk/en/publications/automated-multi-level-governance-compliance-checking/); the plan's Cardiff link returned 403.
- Take-grant safety decidable in linear time and access-matrix safety undecidable in general: correct from memory (Jones, Lipton and Snyder 1976; Harrison, Ruzzo and Ullman 1976); not fetched.
- The non-goals are consistent with the limits the survey reports. I found no non-goal that contradicts a cited theorem.

## Verdict

The plan is a reasonable outline of a mechanised reference-monitor calculus, and its non-goals and its list of ways it could fail are better than most. As a route to "finding out whether an integrated foundation can be built" it does not work as written, for one reason: none of the seven targets is stated so that it could fail for an interesting reason, and the one target presented as new (T4) is either true by definition or contradicted by the plan's own example, depending on a definition the plan does not give. Fixing that is a matter of restating T2, T4, T5 and T6 with explicit counterexample candidates, which is a few weeks of pencil-and-paper work and should come first. The other changes needed before any mechanisation are: resolve the conflict between complete mediation and shared membership (H1), replace the locality test with stated bounds that the worked example's own rule change can meet (H2), move the default core to a timed model with dynamic creation and carry its side conditions (H3), add a threat model and put adversaries, disputes and dissolution into the benchmark (H6, H7), and correct T7 (H5). With those changes the plan would be a credible test of a narrower claim than the goal: that mediated, recorded interactions among specified controllers compose with stated guarantees. It would still say nothing about humans, learned agents or legitimacy, and it should say so in one place (M4).

CRITICAL: 1  HIGH: 8  MEDIUM: 6  LOW: 3

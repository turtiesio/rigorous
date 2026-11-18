# Adversarial review, round 2: "Small rigorous substrates exist; the integrated foundation is unbuilt"

Reviewed 17 November 2026: `/root/mic/reports/Compositional foundations for governed systems.md` (second version), against the seven note files, the first version, the two round-1 reviews, and primary sources fetched this session. I formed my view of the current report before reading the earlier reviews. Nothing in the report or the notes was modified.

Labels used for every finding:

- **VERIFIED WRONG / VERIFIED DISTORTION**: a source fetched this session contradicts or materially weakens the claim.
- **COULD NOT VERIFY**: the source was unreachable or silent.
- **JUDGEMENT**: my argument. Where a reference is cited from my memory and not fetched, I say so.

Summary: the revision fixed what round 1 broke, and the bibliography again held up (about 45 items checked this round, no fabricated source, no theorem stated backwards). The new damage is in what the revision added. The one positive recommendation, proof-carrying design, was imported from the reviewers' counterexample, was not in version 1 or in any researcher's findings, and is contradicted by the report's own at-scale evidence. Two other headline sentences (nothing formally founded runs at scale; two shapes at 10^9) contradict the report's own body, table and notes.

---

## 1. CRITICAL (borderline HIGH): "so the supportable design is proof-carrying" does not follow from the body, and the report's own at-scale evidence points elsewhere

**Claim attacked.** Lead: "guarantees over a universal language are paid for with proof effort, conservatism or runtime enforcement, so the supportable design is proof-carrying: unrestricted actors, a small trusted checker, and evidence demanded only for designated governance invariants." Conclusion: "That points toward a proof-carrying design ... evidence demanded only for a short list of governance invariants (conservation of budgets, bounded authority, attributable identity, append-only provenance, contract refinement)."

**Why it is unsupported (JUDGEMENT, with verified evidence).**

1. **Non sequitur.** The premise names three ways to pay (proof, conservatism, runtime enforcement). The "so" selects one without argument. The notes list four co-equal alternatives and do not rank them: `self_reference_and_limits.md` line 154 gives "(a) correctness by construction in a restricted language, (b) proof-carrying/attested behaviour ..., (c) runtime enforcement of safety properties (monitors, capabilities), (d) sound over-approximation with false alarms."
2. **Provenance.** Version 1 contains the string "proof-carrying" zero times (checked by grep). The idea entered through round 1: Astra offered it as the "strongest counterargument" to a false dichotomy and added "This does not prove that such a system will achieve social legitimacy, full alignment or planetary deployment"; Fable r1 said the reworded trade-off "implies the foundation must be a proof-carrying one". A reviewer's existence counterexample was promoted to the report's headline recommendation with one citation ([Necula 1997](https://homes.cs.washington.edu/~mernst/teaching/6.893/readings/necula-popl97.pdf)) and no research pass on proof-carrying systems.
3. **The report's own scaled evidence is not proof-carrying.** Every governed system the report cites at 10^8 or above enforces by a runtime reference monitor over a restricted policy language: Zanzibar, Cedar/IAM, UPI's central switch. So does its agent evidence: the Cournot result uses an "Oracle/Controller runtime" that enforces a public governance graph ([arXiv 2601.11369](https://arxiv.org/abs/2601.11369), fetched), and CaMeL is a capability-enforcing interpreter (from memory of [arXiv 2503.18813](https://arxiv.org/abs/2503.18813)). The report's own sentence says authority "reached that scale by being decidable, monotone or stratified, default-deny and locally evaluable", which is alternative (a) plus (c). The notes say the same and draw the opposite moral: "Generality and scale have so far traded off against each other" (`logic_formal_methods.md` line 108). In the report's one verified-policy case, proofs found 4 bugs and testing found 21 ([Cedar](https://arxiv.org/abs/2407.01688v1), fetched).
4. **Several of the five listed invariants are not certificate-checkable.** "Attributable identity" is a trust-anchor property; the report itself cites Douceur for why no local check establishes it. "Append-only provenance" needs non-equivocation, which is an agreement property. "Conservation of budgets" includes non-negative balance, which the report's seventh element classifies as non-monotone and assigns to "a single owning boundary ... or a small consensus group". Only "bounded authority" and "contract refinement" have the shape of a locally checked proof obligation, and refinement checking is undecidable for unrestricted contract languages.
5. **Humans cannot carry proofs.** For human and organisational actors, conformance is established after the fact by audit and adjudication. The report's own body says "pure liveness is never enforceable by prevention and needs deadlines or sanctions" and that the specification "for organisations ... is contested social intent, [and] is the dominant problem". A proof-carrying design moves all the weight onto the specification the report says is the unsolved part.

**Recommended rewording.** Lead: "...paid for with proof effort, conservatism or runtime enforcement. The evidence at scale is of the last two: restricted, decidable policy languages evaluated by runtime reference monitors. Proof-carrying admission (unrestricted actors, a small trusted checker) is a research-stage candidate for the subset of invariants that are locally checkable; it is this report's and its reviewers' inference, not a finding of the survey." Conclusion: replace "That points toward a proof-carrying design" with "That leaves a mixed design: capability confinement and runtime monitors for safety invariants, restricted languages where decisions must be made at population scale, certificates where a producer can supply them, and ex-post accountability for everything else", and split the five invariants into locally checkable ones and those that need an identity authority or agreement.

Severity note: the fix is a rewording, but this is the report's only positive design conclusion and it is stated as a consequence ("so"). I rate it CRITICAL on the "headline conclusion unsupported" criterion and flag that a stricter reader could call it HIGH.

---

## 2. HIGH: "nothing formally founded has been exercised near 10^10" is propped up by two false supporting sentences, and contradicts the report's body and notes

**Claims attacked.** Table: "Control at 10^10 actors | Not demonstrated ... | Largest formally founded artefacts are single kernels". Open problem ten: "no formally founded kernel operates anywhere near 10^10 actors, while the systems that do operate at that scale are not formally founded."

**VERIFIED WRONG (the two supporting sentences).**

- AWS replaced its IAM authorization engine with one written and proved in Dafny against a formal specification; the engine is "invoked 1 billion times per second" ([Chakarov et al., ICSE 2025](https://conf.researchr.org/details/icse-2025/icse-2025-research-track/119/Formally-Verified-Cloud-Scale-Authorization)); AWS states every API call passes through it, "over 1.2 billion requests per second" ([AWS Security Blog](https://aws.amazon.com/blogs/security/an-unexpected-discovery-automated-reasoning-often-makes-systems-more-efficient-and-easier-to-maintain)). This is a formally verified governance component (authority decisions) at roughly 10^14 decisions per day. It is not a "single kernel" and it is formally founded.
- The notes already had the neighbouring facts and the report dropped them: `logic_formal_methods.md` line 100 (Zelkova, "A Billion SMT Queries a Day", [CAV 2022](https://doi.org/10.1007/978-3-031-13185-1_1)) and line 149 ("Authorization is the one normative area with proven planetary scale (Zanzibar, IAM/Zelkova, Cedar)").
- Internal contradiction: the body says "Authority is the one normative area with proven planetary scale", and the table credits object capabilities and authorization logics with "planetary-scale authorization". Zanzibar's abstract reports "trillions of access control lists and millions of authorization requests per second" for "billions of people" ([Pang et al.](https://www.usenix.org/conference/atc19/presentation/pang)).

**What survives.** No formally founded system spans 10^10 *heterogeneous actors across levels*, and the scaled examples are single-operator, narrow-vocabulary and safety-only. That is the claim the report should make.

**Recommended rewording.** Table basis: "Formally verified components run at 10^9 decisions per second inside single-operator, narrow-vocabulary authorization systems (AWS IAM, Cedar); no formally founded system governs 10^10 heterogeneous actors across organisational levels." Open problem ten: delete "the systems that do operate at that scale are not formally founded" and replace with "the formally verified systems at that scale cover one property (authorization decisions) under one operator." Note that this evidence is the best available support for a small-checker architecture, and that it is evidence for restricted languages, not for unrestricted actors (finding 1).

---

## 3. HIGH: "the systems that do run at 10^9 take two shapes" contradicts the report's own comparison table and body

**Claims attacked.** Lead: "the systems that do run at 10^9 take two shapes: thin-interface federations, and centrally rooted identity systems with a narrow vocabulary." Conclusion: "Both shapes seen at 10^9 ... are live options, and the choice between them is a design and political decision". Body: "The only governed actor systems at 10^9 scale are digital public infrastructure with a unique centrally issued identity".

**VERIFIED from the report's own text (internal contradiction).**

- The comparison table lists "Price mechanism and general equilibrium ... Largest evidence of use: The world economy" and "Law, institutional economics, enterprise ontology ... 8.3 billion people, roughly 200 states". Neither is a thin-protocol federation of the Internet kind, and neither is a centrally rooted identity system with a narrow vocabulary. Law has the widest vocabulary of anything in the report.
- The body says "Law is the one field that has run a recursive, self-modifying governance system on billions of people" and "The empirical shape of planetary governance is a polycentric overlay of shallow hierarchies" (China, 1.4 billion, four administrative tiers). Twenty lines later it says "The only governed actor systems at 10^9 scale are digital public infrastructure". Both cannot stand. The "only" sentence survived from version 1 unchanged.
- Hybrids are also missing: multi-rooted federated identity (the Web PKI's plural roots, mobile roaming among national SIM issuers) is neither shape. (JUDGEMENT, from memory; not fetched.)

The two-shape taxonomy came from Fable r1 section 7, which said the report's evidence showed "two shapes", as a correction to a one-shape claim. The editor turned a lower bound into an exhaustive taxonomy and then into a binary design choice.

**Recommended rewording.** "At 10^9 the evidence shows at least four coordination shapes: thin-protocol federation (Internet routing and naming), centrally rooted identity with a narrow vocabulary (Aadhaar, UPI, single-operator platforms), price-mediated exchange, and polycentric legal hierarchy. Only the first two are engineered digital systems; only the last has a wide vocabulary, and it is the least formal." Delete "The only governed actor systems at 10^9 scale are ..." or restrict it to "engineered digital systems". In the conclusion, replace "the choice between them" with "the mix among them".

---

## 4. HIGH: missed prior art that bears directly on the new headline: proof-carrying data and recursive proof composition

**Claim attacked.** The proof-carrying recommendation cites only Necula 1997; the notes contain zero hits for "Necula", "SNARK", "proof-carrying data" or "zero-knowledge" (grep). The report says of its packagings "none has been built for this purpose".

**Why it matters (existence VERIFIED; relevance is JUDGEMENT).** Proof-carrying data is the published formal framework for exactly the design the report now recommends, and it is recursive in the sense the question asks about: every message in a distributed computation among mutually untrusting parties carries a short proof that it and its entire history satisfy a "compliance predicate", checked locally by each recipient, with proofs composed from prior proofs ([Chiesa & Tromer, ICS 2010](https://projects.csail.mit.edu/pcd); thesis at [MIT DSpace](https://dspace.mit.edu/handle/1721.1/61151)). Recursive composition was put on a standard footing by Bitansky, Canetti, Chiesa and Tromer (STOC 2013, [ePrint 2012/095](https://eprint.iacr.org/2012/095), from memory, not fetched). It supplies what the report's sketch lacks: a definition of the checker, a composition theorem for evidence along chains of unbounded depth, and constant-size evidence independent of history. It also exposes the limits the report should state: the guarantee covers only predicates on local transitions and history (safety-shaped), soundness is computational and rests on setup or knowledge assumptions (a named trust anchor), and prover cost is orders of magnitude above native execution.

Related omissions on the same axis (from memory, not fetched): foundational proof-carrying code (Appel, LICS 2001), and the deployed descendants of Necula's idea, which are type-checking verifiers over restricted bytecode (JVM, eBPF, WebAssembly validation), that is, "conservatism", not proofs over unrestricted programs.

**Recommended fix.** Add a row "Proof-carrying data / recursive proof composition" to the comparison table (primitives: compliance predicate, proof, recursive verifier; proves: history compliance composes along arbitrary message graphs; does not express: liveness, incentives, identity; evidence: deployed proof-carrying ledgers, not general governance), and cite it wherever the report says the proof-carrying packaging has no formal instance.

---

## 5. MEDIUM: ARIA is reported one-sidedly, and the omitted half cuts against the report's reading

**Claim attacked.** "ARIA's £59m Safeguarded AI programme aborted its machine-learning track in November 2025, with its director judging it 'more likely than not we'd select zero successful teams' ... the retreat is to domains where the world model is already formal, and 'auditable multi-agent systems' is named as a target with no public deliverable."

**VERIFIED DISTORTION.** The cited update ([ARIA, 28 Nov 2025](https://ariaresearch.substack.com/p/november-update-a-safeguarded-ai), fetched) gives the reason: the value of a dedicated ML R&D team "seemed much lower than when we started" because "we now expect many of these capabilities to come online soon by default." The track was stopped because frontier models were expected to do the proof-generation work anyway, not because the approach failed. The same update says the tooling track was broadened into a general toolkit for mathematical assurance, and that is where "auditable multi-agent systems" appears (so that phrase is verified, in the Substack post, not on the programme page the report links next to it). The quote, the £59m, the April 2026 director change and the cybersecurity focus all check out ([ARIA page](https://aria.org.uk/opportunity-spaces/trust-everything-everywhere/safeguarded-ai), fetched).

This matters because the stated reason is evidence *for* cheap machine-generated proofs, which is the main enabling assumption of the proof-carrying direction, while the report presents the event as evidence of retreat.

**Recommended rewording.** "...halted selection for its machine-learning track in November 2025; its director judged it 'more likely than not we'd select zero successful teams' because the capabilities the track would have built were now expected from frontier models by default. The application focus narrowed from cyber-physical control to cybersecurity, while the tooling track was broadened to include 'auditable multi-agent systems', with no public deliverable yet."

---

## 6. MEDIUM: over-correction: "these impossibilities are sharpest for small numbers" and "agreement impossibilities soften in large populations"

**Claims attacked.** Lead: "several incentive and agreement impossibilities soften in large populations". Body, after listing Arrow, Gibbard-Satterthwaite, Sen, List-Pettit and Myerson-Satterthwaite: "These impossibilities are sharpest for small numbers."

**VERIFIED / JUDGEMENT.** Round 1 showed that two results soften with scale (the double auction, [Rustichini, Satterthwaite & Williams](https://econpapers.repec.org/RePEc:ecm:emetrp:v:62:y:1994:i:5:p:1041-63), fetched, O(1/m²); strategy-proofness in the large). The revision generalised that to the whole list.

- Arrow, Sen and List-Pettit hold for every finite number of voters and do not weaken with n (JUDGEMENT; standard).
- Gibbard-Satterthwaite has a quantitative form that survives scale: for rules far from non-manipulable, a random profile is manipulable with probability at least inverse-polynomial in n ([Mossel & Rácz](https://arxiv.org/abs/1110.5888), search-level check).
- No agreement impossibility softens with population. FLP and the n > 3f bound are independent of n. King and Saia escape the quadratic *message* bound by randomisation, which the body states correctly ("a bound that randomised protocols escape"); the lead misattributes this to population size.
- The softening results require exchangeable, small-relative-to-market actors. The report's own seventh open problem says the target population is "heterogeneous, hierarchical, self-modifying ... with identities".

**Recommended rewording.** Lead: "some incentive impossibilities (bilateral-trade inefficiency, strategy-proofness) soften in large populations of interchangeable actors; the aggregation and agreement impossibilities do not, though randomisation lowers agreement's message cost." Body: "The bilateral-trade and strategy-proofness results are sharpest for small numbers".

---

## 7. MEDIUM: closure and congruence are conflated, and the fix applied the side conditions to the wrong one

**Claims attacked.** Lead: "Closure under composition ... is available by construction in operad algebras, in DEVS coupling and, under side conditions, in process calculi." First element: closure is "stated as an operad-algebra law, a closure-under-coupling proof or a congruence result." Body: "a behavioural equivalence that is a congruence ... That holds for CCS."

**VERIFIED / JUDGEMENT.**

- Closure (the parallel composition of two processes is a process) is unconditional in every process calculus; it is part of the grammar. What needs side conditions is *congruence* (substitutivity of equivalent parts). Round 1 attacked "bisimilarity is a congruence"; the editor attached "under side conditions" to closure instead. A congruence result is also not a closure statement, so the first element lists two different properties as one.
- "That holds for CCS" is true of strong bisimilarity only. Weak bisimilarity, the equivalence that abstracts from internal steps and is the one used for substitution in practice, is not preserved by sum in CCS; Milner's observation congruence exists for that reason. Round 1 said this for the pi-calculus ("weak bisimilarity is additionally not preserved by sum", [MPRI notes](https://www.lix.polytechnique.fr/~catuscia/teaching/MPRI/05Fall/mpri-concurrency-2005/www/c8/cours8.pdf); [Bengtson & Parrow](https://arxiv.org/pdf/0809.3960)); the fix did not carry it to CCS.
- Conversely, the body says DEVS closure "fail[s] for some natural subclasses", so DEVS is the leg that needs the hedge.

**Recommended rewording.** Lead: "Closure under composition is available by construction in operad algebras, in process calculi and, for the general class, in DEVS coupling. Substitutivity (replacing a part by an equivalent part) holds under side conditions: for strong bisimilarity in CCS, for bigraphs where relative pushouts exist, and in the pi-calculus only after closing under substitution." First element: separate "closure" from "congruence" as two requirements.

---

## 8. MEDIUM: the flagship evidence for the "dependent type theory" candidate is not dependent type theory

**Claim attacked.** Table row "Dependent type theory with a resource layer (Lean, Rocq, Iris) ... seL4, CompCert, Cedar's verified model". Body: "The most general kernel is dependent type theory used as a meta-language with a small proof checker. It has carried the largest verified artefacts, and the cost is documented: about 200,000 lines of Isabelle proof ... in seL4".

**VERIFIED WRONG (classification).** seL4's proof is in Isabelle/HOL (the cited [Trustworthy Systems page](https://trustworthy.systems/projects/OLD/l4.verified/numbers) says "200,000 lines of Isabelle proof script"). Isabelle/HOL is classical higher-order logic over simple types with an LCF-style kernel; it has no dependent types (from memory; see the [Isabelle overview](https://isabelle.in.tum.de/overview.html), not fetched). The seL4 numbers themselves check out (7,500 verified lines of C out of 8,700; about 25 person-years overall).

This matters for the candidate comparison the question asked for: the largest verified artefact the report cites was carried by a *simpler* logic than the candidate it is filed under, which is evidence that dependent types are not needed for the "small trusted checker" role.

**Recommended rewording.** Rename the row "Proof assistants with a small trusted kernel (Isabelle/HOL, Rocq, Lean; Iris as resource layer)" and write "The most general kernel is a higher-order logic or dependent type theory used as a meta-language ... seL4 was verified in Isabelle/HOL, CompCert in Rocq".

---

## 9. MEDIUM: GLEIF figure misdescribed

**Claim attacked.** "the legal entity nests: more than 3.1 million entities held active identifiers recording parent relationships in mid-2026".

**VERIFIED DISTORTION.** The cited [GLEIF post](https://www.gleif.org/en/newsroom/blog/the-lei-in-numbers-q2-2026-signals-growing-regional-momentum-for-the-lei) (fetched) says the active LEI population is "over 3.1 million" and, separately, that registrants "reported direct and ultimate parent information". Reporting includes declaring an exception (no parent, opt-out, parent has no LEI). GLEIF's own files for 23 June 2026 hold about 654,000 relationship records against about 6.03 million reporting-exception records and 3.35 million LEI records ([GLEIF download page](https://www.gleif.org/en/lei-data/gleif-concatenated-file/lei-download), via search). Most LEIs record an exception, not a parent.

**Recommended rewording.** "more than 3.1 million entities held active identifiers in mid-2026; about 650,000 parent-relationship records exist, and most entities instead file a reporting exception."

---

## 10. MEDIUM: further missed foundations that affect the "minimality" and "accountability" rows

Existence verified where linked; relevance is JUDGEMENT. None is in the report; grep of the notes shows zero hits for the first and third.

1. **Sequential ASM thesis.** Gurevich derives from three postulates (sequential time, abstract state, bounded exploration) the theorem that every sequential algorithm, at any level of abstraction, is step-for-step a sequential abstract state machine ([Gurevich, ACM TOCL 2000](https://web.eecs.umich.edu/~gurevich/Opera/141.pdf)); Blass and Gurevich extend it to parallel and interactive algorithms. This is a proved completeness result for a small primitive set over "systems at any level of abstraction", with industrial use. The table row "Minimality or completeness of a primitive set | Proven only locally" should cite it, and it belongs among the candidates.
2. **TLA.** One logic in which a system is a formula, composition is conjunction and refinement is implication, with the composition theorem the report already cites from [Abadi & Lamport](https://lamport.azurewebsites.net/pubs/abadi-conjoining.pdf). The report uses that paper only for a side condition and never names its framework as a candidate, although the notes mention TLA in four files.
3. **Formal accountability and responsibility.** Open problem two says "no formal semantics covers how accountability and liability propagate along chains that cross human, organisational and AI principals". Hedged that way it survives, but formal definitions exist and are not mentioned: accountability as the ability to identify all misbehaving parties, with automated verification ([Künnemann, Esiyok & Backes, CSF 2019](https://publications.cispa.de/articles/conference_contribution/Automated_Verification_of_Accountability_in_Security_Protocols/24612942/1)); Küsters, Truderung and Vogt (CCS 2010); degrees of responsibility and blame in structural causal models (Chockler & Halpern 2004, from memory). Jones and Sergot's "counts-as" formalisation of institutional power is in the notes (`logic_formal_methods.md` line 125) and was dropped; it is the closest formal account of where authority *originates*, which the third element says "is left to the ninth element".

**Recommended fix.** Add 1 and 2 to the list of candidates to test; reword open problem two to "formal definitions of accountability exist for security protocols and causal models; none has been composed with delegation across human, organisational and AI principals."

---

## 11. MEDIUM: "None of the interoperability protocols specifies how authority, budget or liability attenuate" overstates against the notes

**Claim attacked.** "None of the interoperability protocols specifies how authority, budget or liability attenuate when an agent delegates onward."

**VERIFIED DISTORTION (against the notes; search-level against sources).** `ai_agent_governance_frontier.md` line 146 records AP2 "mandates (intent, cart, payment) as cryptographically signed verifiable credentials expressing user authorisation and limits", and line 105 records an IETF draft profiling OAuth token exchange for agent credential delegation. Secondary descriptions of AP2 confirm scoped, signed mandates with price limits and a verifiable chain from user to intent to cart to charge ([AP2 site](https://ap2-protocol.org/), via search; spec not read). So authority and budget attenuation is specified for the user-to-agent hop in at least one protocol. What is unspecified is agent-to-agent onward delegation and liability.

**Recommended rewording.** "AP2 mandates and OAuth token exchange specify scoped, budget-limited delegation from a user to an agent; no interoperability protocol specifies attenuation across further agent-to-agent hops, and none addresses liability."

---

## 12. MEDIUM: the answer is now safe to the point of saying little, and residue of the withdrawn thesis remains

**JUDGEMENT.**

- Of the eight headline conclusions, three are true by construction or near-tautological: closure is by construction (closure is how these formalisms are defined, as round 1 said); "guarantees are conditional and per-property" (Astra called this "nearly tautological unless the conditions and guarantees are specified"); and "neither constructed nor shown impossible". The report could commit to more without overreach: its own red-flag list identifies specific *combinations* that are impossible (unconditional self-certification in a sufficiently strong theory, complete automatic verification of arbitrary actors, permissionless plus Sybil-proof without authority or scarce resource), and its own evidence identifies combinations that are demonstrated at 10^9 (closure, capability-style authority, safety invariants, single operator). Stating both lists as the answer would answer "does there exist" better than "open".
- The level-by-level test (resource, program, ..., nation) is still absent. The report says so three times, so I do not rate this higher, but it is the comparison the question required.
- The acceptance-test paragraph still presents heuristics "as consequences of the cited theorems": "It keeps the waist content-agnostic" (the report now concedes the Hourglass Theorem supports a thin waist only relative to an unspecified application set), "audits by sampling" (the arithmetic section now says rare-event safety cannot be audited by sampling), and "anticipates that any control metric will be gamed" (no theorem cited). These are leftovers of the version-1 narrow-waist thesis the conclusion withdraws.
- Roughly a quarter of the report still expounds the nine elements after the section header and reviewers demote them to a checklist.

**Recommended fix.** Split the acceptance tests into "follows from a cited theorem" and "design heuristic"; replace the final "neither constructs it nor shows it impossible" with the two explicit lists above.

---

## 13. LOW findings

- **Wonham & Ramadge misattribution (VERIFIED at abstract level).** "hierarchical supervision needs hierarchical consistency and the observer property ([Wonham & Ramadge])". The linked 1988 paper is on modular synthesis; its abstract mentions neither term ([Princeton record](https://collaborate.princeton.edu/en/publications/modular-supervisory-control-of-discrete-event-systems/), fetched). Hierarchical consistency is Zhong & Wonham (IEEE TAC 1990) and the observer property is Wong & Wonham (1996) (from memory). The error was in the notes, was repeated by Fable r1, and was copied into the revision. Fix: cite Wonham & Ramadge for non-conflict only and add the two hierarchical references.
- **Kim et al. cites a superseded version (VERIFIED).** "Across 180 configurations" is v1/v2. The current v3 (April 2026) reports 260 configurations; 17.2x, 4.4x and the 39-70% range are unchanged in its body ([v3](https://arxiv.org/html/2512.08296v3), fetched). Round 1 flagged this; the revision did not update. Fix: "Across 260 configurations (v3)".
- **Jordan cited for a result he did not prove (search-level).** Minimality of the competitive message space is Hurwicz and Mount & Reiter (JET 1974); Jordan 1982 proves uniqueness ([PSU record](https://pure.psu.edu/en/publications/the-competitive-allocation-process-is-informationally-efficient-u/)). The body sentence is right; the table rows "minimal message space among efficient decentralised mechanisms" and "Price-mechanism message space (Jordan 1982)" should add Mount-Reiter. The body should also say the theorem concerns static realisation of allocations and gives neither incentive compatibility nor a convergent adjustment process (JUDGEMENT; the scanned PDF could not be read this session).
- **"rule changes propagate eventually and never simultaneously" (VERIFIED overreach).** Halpern and Moses themselves show that timestamped common knowledge is attainable and coincides with common knowledge when clocks are synchronised, and gives ε-common knowledge when clocks are within ε ([arXiv cs/0006009](https://arxiv.org/pdf/cs/0006009), Theorem 12, via search). Statutes with effective dates and clock-bounded databases use exactly this. Fix: "rule changes cannot become common knowledge on receipt; they can take effect at a pre-announced time, to within clock skew, and must be versioned."
- **Self-certification still stated without its hypothesis.** "for formal theories the derivation is provably unavailable" needs "consistent, recursively axiomatised, interpreting enough arithmetic". Willard's self-verifying theories are in the notes (`self_reference_and_limits.md` line 72) and were flagged in round 1; still omitted. MetaCoq is also overstated: the verified checker excludes the module system and template polymorphism and is proved against a specification (PCUIC), not Coq's implemented kernel ([POPL 2020 abstract](https://popl20.sigplan.org/details/POPL-2020-Research-Papers/36/Coq-Coq-Correct-Verification-of-Type-Checking-and-Erasure-for-Coq-in-Coq), via search). Fix: "a type checker for Coq's core calculus, without modules or template polymorphism, is verified in Coq assuming strong normalisation".
- **Lynch et al. scope (VERIFIED, minor).** What "was previously not known" is whether these systems theories "supported clock systems"; the non-deterministic case is a definition, with no classical notion to recover ("would just be a definitional equality"). The restriction of stochasticity to systems is confirmed (Example 2.4, "necessary for theorem 4.10") ([arXiv 2603.29573v2](https://arxiv.org/html/2603.29573v2), fetched). Fix: "Whether stochastic systems theories admit clock systems recovering classical behaviours was open until March 2026; the non-deterministic case is so far a definition."
- **"says" and "speaks for" attributed to Garg & Abadi.** The calculus is Abadi, Burrows, Lampson and Plotkin (TOPLAS 1993); Garg & Abadi 2008 is a modal reconstruction (from memory). Fix: cite both.
- **Ligatti et al. sits oddly beside the next clause (VERIFIED).** The paper's abstract says edit automata enforce "every safety property, some liveness properties, and some properties that are neither" ([PDF](https://www.cs.princeton.edu/~dpw/papers/nonsafety05.pdf), read locally). "pure liveness ... needs deadlines or sanctions" should read "liveness is not enforceable by halting alone; inserting actions enforces some of it, and the rest needs deadlines or sanctions."
- **"mean-field methods make large populations of interchangeable actors tractable"** was added in the revision with no citation. Add one (Lasry & Lions; Huang, Malhamé & Caines).
- **Links that failed this session (COULD NOT VERIFY).** McLean (`web.mst.edu/~ff/classes/6600/mclean-sp94.pdf`, 404), Bloem et al. (`BJKKRVW16.pdf`, 404 on fetch though indexed by search as the 2016 SIGACT News article), Zeigler (403), Saoud et al. on HAL (bot wall; the small-gain claim was confirmed from a search excerpt of the abstract), Bliudze & Sifakis on Infoscience (429; content confirmed via search), Hewitt & Baker on DSpace (429), Jordan PDF (scanned image, unreadable).

---

## Checks that survived this round

Fetched and confirmed as stated: arXiv 2410.08373 withdrawn with the pre-monoidal comment; Spivak 2602.17917 (single author, 20 Feb 2026); Libkind & Myers 2505.18329; Bracale Syrnikov et al. (3.1 to 1.8, prompt-only "no reliable improvement"); Kim et al. v2 figures; DAO Gini "all exceed 0.94" for voting power (read in the PDF; five protocols); draft-ietf-wimse-aims-01 (2 Nov 2026) and all three scope exclusions; ARIA quote, £59m, April 2026 director change; seL4 numbers; Cedar 4 and 21; Hourglass Theorem now stated in the right direction (post(S1) ⊆ post(S2), pre(S1) ⊇ pre(S2), read in the PDF); Chen et al. 1810.10636; Legros & Matthews; Rustichini et al. O(1/m²); Clavel & Meseguer; BGP 1,044,785 entries and 78,339 ASes (18 Apr 2026); CPC "nearly 101.29 million" and "over 5.43 million"; UPI 25.47 billion (article dated 2 Nov 2026, +23% year on year, consistent with October 2026); WhatsApp 465 million, 11,000 cores, >70M messages per second; Anwar et al. 14-35% (read in the PDF); 130,000-node GKE cluster "in experimental mode"; Zanzibar. The 10^10 arithmetic (34 bits, 5 × 10^19 pairs, tree depths, 0.9^9 = 0.39, the 1% attention ratio) recomputes correctly.

Fidelity of the round-1 fixes: the dichotomy, Hourglass, compositional-control, minimality, Holmström, world-model assumptions, Gao-Rexford limits, Finkbeiner-Schewe citation and first-author corrections were applied faithfully. The fixes that went wrong are the ones above: proof-carrying promoted from counterexample to recommendation (1), "two shapes" promoted from lower bound to taxonomy (3), large-population softening over-generalised (6), side conditions attached to closure instead of congruence (7), and the Wonham-Ramadge misattribution inherited from a reviewer (13).

---

## Verdict on the headline conclusions

| Headline | Verdict |
|---|---|
| Small rigorous substrates exist | Established. |
| Closure is by construction; cross-scale governance is unproven | True; the first half is definitional, and the wording confuses closure with congruence (finding 7). |
| Guarantees are conditional and per-property | True and close to tautological (finding 12). |
| A proof-carrying design is the supportable direction | Unsupported as stated; imported from reviewers, contradicted by the report's own at-scale evidence (finding 1), and missing its main formal prior art (finding 4). |
| The nine elements are a checklist, not a kernel | Established, and stated candidly. |
| Two shapes exist at 10^9 | Wrong as a taxonomy; contradicted by the report's own table and body (finding 3). |
| Nothing is demonstrated near 10^10 | Survives for heterogeneous actors across levels; its two supporting sentences are false (finding 2). |
| The full combination is neither constructed nor shown impossible | True, and less than the body supports (finding 12). |

**Does it answer the question?** Partly. It answers "is there one today" (no) and "is it ruled out" (not shown) honestly. It still does not perform the required comparison of the strongest candidates level by level, which it admits, and its single positive recommendation is the least-evidenced sentence in the document. The factual record is reliable; the synthesis needs one more pass limited to findings 1 to 4, which are all rewordings plus one added table row and do not require new research beyond proof-carrying data and the verified-authorization literature.

CRITICAL: 1  HIGH: 3  MEDIUM: 8  LOW: 9

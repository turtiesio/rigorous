# Adversarial review: Compositional foundations for governed systems

Reviewed 17 November 2026. Line references refer to the 171-line report as supplied. I read the full report and consulted the underlying notes, particularly their qualifications about control, resources, self-reference and composition. I did not modify either. Findings are ordered by damage to the argument, not by how conspicuous an error is.

**Classification:** “Verified wrong” means a source or explicit counterexample contradicts the stated claim. “My judgement” identifies an unsupported inference or inadequate argument, not a disproved theorem. “Could not verify” does not mean fabricated. Verification here includes reading relevant theorem statements and assumptions, but is not an independent proof audit or experimental replication.

## 1. The central expressiveness-versus-guarantees dichotomy has the wrong quantifiers

**Claim attacked:** L3: “universal in what it can express or strong in what it can guarantee”; no foundation can construct everything and also govern it “with guarantees.” L169 presents the resulting trade-off as established by the literature.

**Classification: Verified wrong as an unrestricted dichotomy; my judgement about its consequences for the user's question.**

Universal expression, automatic discovery of proofs, completeness of verification, and guaranteed behaviour of admitted components are different requirements. A universal language can host arbitrary computations while a sound, incomplete proof system certifies strong properties of particular computations. A resource monitor can constrain a universal computation without deciding what that computation will eventually compute. Neither arrangement decides an arbitrary nontrivial semantic property for every program.

The report itself describes this escape in its small-checker architecture, then forgets it in the headline. Necula's *Proof-Carrying Code* supplies a concrete precedent: a consumer checks a producer-supplied proof against a specified safety policy. The source establishes conditional safety of accepted code, not completeness of proof generation. [Primary paper](https://homes.cs.washington.edu/~mernst/teaching/6.893/readings/necula-popl97.pdf). Necula and Schneck subsequently allow untrusted high-level proof rules while proving that they preserve the trusted low-level invariant. [Paper and Coq formalization](https://people.eecs.berkeley.edu/~necula/ISSS02/).

**Strongest counterargument:** use a small universal semantic substrate, compositional interfaces and a proof checker; require evidence only for designated governance invariants. Represent humans as environment processes constrained at enforceable interfaces. Permit arbitrary internal strategies, including learning. This does not prove that such a system will achieve social legitimacy, full alignment or planetary deployment. It does refute the claimed logical incompatibility between expressive universality and strong guarantees. The actual impossibility is a sound, complete, effective verifier for arbitrary semantic properties over an unrestricted computational domain.

**Damage: Critical.** Withdraw the dichotomy as written. Its defensible replacement is much narrower and does not answer the user's existence question negatively.

## 2. The theorem used to justify the title is stated backwards

**Claim attacked:** L114: a weaker spanning-layer specification admits “more supports below and applications above.”

**Classification: Verified wrong.**

Beck's Hourglass Theorem, Theorem 1, says that if S1 is weaker than S2, then post(S1) is a subset of post(S2), while pre(S1) is a superset of pre(S2). Weakening expands possible supports but contracts, or leaves unchanged, the supported applications. The subsequent minimal-sufficiency condition is essential: first choose necessary applications, then find a sufficiently weak layer that still supports them. [Author's full paper, pp. 51–52](https://web.eecs.utk.edu/~mbeck/OnTheHourglassModel.pdf).

The self-reference note repeats the error, adding “subject to sufficiency,” which does not reverse the inclusion. This is not just a careless citation: the report has not specified its necessary application set, so it cannot use the theorem to show its proposed waist is sufficient or preferable.

**Damage: High.** The narrow-waist proposal can remain a design hypothesis; its supposed theorem-backed dominance cannot.

## 3. “No compositional viability theorem” overlooks directly relevant results

**Claim attacked:** L56: no theorem that viability kernels or supervisors of parts compose into a viable or controllable whole; L163 repeats “no compositional viability theorem.”

**Classification: Verified wrong as a blanket absence claim.**

Chen, Anderson, Kalsi, Low and Ames, *Compositional Set Invariance in Network Systems with Assume-Guarantee Contracts* (2018), explicitly develops an assume-guarantee method for computing robust invariant sets of network systems. [Primary source](https://arxiv.org/abs/1810.10636). Ghasemi, Sadraddini and Belta, *Compositional Synthesis of Decentralized Robust Set-Invariance Controllers for Large-scale Linear Systems* (2019), constructs decentralized controllers for coupled linear systems with bounded disturbances using subsystem invariant-set contracts. [Full paper](https://arxiv.org/pdf/1909.06425). Later work covers finite-time reach/avoid and infinite-time invariance through compositional convex contract synthesis. [Primary source](https://arxiv.org/abs/2002.06494).

These results have substantial restrictions. They do not give exact maximal viability kernels for arbitrary nonlinear societies, nor show that arbitrary locally viable parts remain viable under arbitrary coupling. But that is no reason to say the conditional composition theorem is missing: conditional composition is precisely the report's advertised standard.

The cybernetics note at line 200 explicitly says contract methods for invariance exist but were not sourced. The synthesis promotes that research omission into a major theoretical gap.

**Damage: High.** Reclassify this from missing theory to an existing restricted theory whose applicability and scaling need comparison.

## 4. “Fractality is fully met” substitutes closure for the requested cross-scale construction

**Claim attacked:** L3, L119 and L147: the same-structure requirement is a proven theorem, hence fully met.

**Classification: My judgement—unsupported identification of requirements.**

Vagner, Spivak and Lerman construct algebras of open dynamical systems over wiring diagrams. Their result supplies coherent composition within a defined mathematical class. [Primary paper](https://arxiv.org/abs/1408.1598). It supplies neither a representation theorem for nations, persons and resources nor proof that authority, membership, liability and control survive the chosen abstractions between these levels.

Closure alone is extremely cheap: sets of labelled transition systems can be closed under a chosen composition. The difficult requirement is a faithful common interface, compatible notions of entity identity and admissible intervention, and preservation of the properties that make something governed. No translation from the user's heterogeneous entities into the proposed kernel is given, let alone a theorem about it.

The report knows this: L64 says organisational composition has no corresponding theorem; L163 says the integrated composition semantics is missing. These admissions are incompatible with announcing the user's fractal requirement as fully satisfied.

**Damage: High.** Retain “formal closure is available in several substrates.” Withdraw “the fractal requirement is fully met.”

## 5. The nine primitives do not converge to one semantics; the resource claim is demonstrably false

**Claim attacked:** L117–139, especially L127: Iris resource algebras, linear logic and double-entry accounting agree that budgets, tokens, permissions and shares obey a conservation law.

**Classification: Verified wrong for the claimed common conservation law; my judgement for minimality and compatibility.**

Iris resource algebras deliberately include duplicable cores and persistent knowledge, as well as exclusive ownership. They are not generally numerical conserved quantities. The original paper defines these structures in §2.2 and discusses their distinctions from earlier separation algebras in §9.3. [Full paper](https://cs.au.dk/~birke/papers/iris-journal.pdf). A nonzero duplicable resource r with r·r=r cannot also obey an additive strictly positive quantity law: q(r)=2q(r) forces q(r)=0. This is a counterexample to the report's identification, not a criticism of Iris.

Balanced accounting entries likewise do not conserve productive capacity, asset value, information or aggregate authority. Debiting a newly acquired asset and crediting a liability keeps books balanced while changing the economic resources and obligations being represented. Bookkeeping identity and physical conservation must be distinguished.

The proposed primitives also mix semantic objects, representation choices and architectural policies. A controller is a unit; a boundary can be an interface policy; a commitment can be a protocol state machine; a ledger can be state plus an invariant; meta-rules can be rules about rule-changing transitions. The list has no independence argument.

Conversely, identity is assumed later but not specified; deadlines require time; epistemic uncertainty requires observations or beliefs; strategic behaviour requires goals/preferences; stochastic guarantees require probability; learning requires rules governing policy/model updates. These can be libraries, but exactly the same answer permits commitments, ledgers and controllers to be libraries.

A smaller candidate signature is an open transition system (states, actions, observations, transition relation), a composition operator and a logic of admissibility/refinement. The listed objects can be encoded in it. That is a candidate baseline for comparison, not a new claimed solution. “Smallest” has no meaning until the report chooses a metric that charges for encodings, proof rules and trusted assumptions.

**Damage: High.** The nine items are a useful architectural checklist at most, not an evidenced fundamental kernel. Duplicable authority and conserved tokens require distinct resource types and explicit bridges.

## 6. The safety/liveness divide is overstated, including against the cited composition paper

**Claim attacked:** L32: mutual promises compose as prohibitions but not obligations to eventually act; L3: only safety-type invariants are controllable at scale, whereas liveness needs statistics and re-verification.

**Classification: Verified wrong for the categorical inference; my judgement for the planetary extrapolation.**

Abadi and Lamport's *Conjoining Specifications* includes liveness in its specification framework. Its general composition theorem, Theorem 3, separates premises concerning safety closures C(M) from premises about full guarantees M. It does not establish that liveness guarantees cannot compose. [Full paper, §5.3, printed p. 529](https://lamport.azurewebsites.net/pubs/abadi-conjoining.pdf).

Circular “I eventually act if you eventually act” promises are insufficient. That does not rule out compositional progress proved using fairness, scheduling, well-founded ranks or stronger interface assumptions. Likewise, safety of isolated components is not automatically preserved by arbitrary interaction. The report gives safety a charitable “under stated conditions” heading while treating liveness's need for conditions as disqualification.

FLP survives: deterministic consensus cannot guarantee termination in every admissible execution of its asynchronous one-crash model. That is the actual scope of the source, not a theorem forbidding all large-scale liveness guarantees. [Original paper, Theorem 1](https://www.cs.cornell.edu/courses/cs614/2004sp/papers/FLP85.pdf).

**Damage: High.** Replace the safety-only thesis with a per-property and per-environment analysis. Statistical evidence is not the only alternative to safety proofs.

## 7. The survey omits or demotes some of the closest candidate foundations

**Claim attacked:** L9's comprehensive comparison, and L163's claim that the glue and delegation calculus simply do not exist.

**Classification: My judgement about survey adequacy; verified existence of the omitted precedents.**

The following deserve explicit comparison before inventing the kernel:

- **Law-Governed Interaction (LGI), Minsky and Ungureanu (2000).** This is an implemented mechanism for explicit, decentralized enforcement over heterogeneous, independently designed agents, with a generality comparison against centralized coordination. It directly targets the problem the report proposes to solve. [Institutional primary publication record](https://www.researchwithrutgers.org/en/publications/law-governed-interaction-a-coordination-and-control-mechanism-for/), [authors' paper](https://www.cs.rutgers.edu/~minsky/papers/coordination-and-control.pdf). This is not evidence of ten-billion-actor validation, but omitting it makes the novelty assessment unreliable.
- **Universal composability, Canetti (FOCS 2001).** Defines ideal functionalities, adversarial environments and security-preserving composition. It is a serious candidate for the semantics of interfaces carrying guarantees, not merely an example of a quantitative workaround. [Primary publication](https://research.ibm.com/publications/universally-composable-security-a-new-paradigm-for-cryptographic-protocols), [full revised treatment](https://eprint.iacr.org/2000/067.pdf). The self-reference note includes it; the report drops it from the comparison. Setup and realizability limitations must be compared, rather than treating all non-trace properties as outside the compositional core.
- **Proof-carrying authentication/authorization.** Appel–Felten and subsequent Bauer work give a logical and implemented account of authorization with delegation, including obtaining and checking proofs across principals. [Appel–Felten publication](https://collaborate.princeton.edu/en/publications/proof-carrying-authentication/), [Bauer et al. full paper](https://users.ece.cmu.edu/~lbauer/papers/pcaprototr.pdf). This does not solve real-world legal liability, but formal delegation semantics is not absent.
- **Probabilistic assume-guarantee verification, Kwiatkowska, Norman, Parker and Qu (TACAS 2010).** Gives automated asymmetric and circular reasoning rules for probabilistic/nondeterministic systems using regular safety assumptions. [Full paper](https://eprints.gla.ac.uk/39607/1/tacas10.pdf). L163's comparative maturity claim might be defensible, but requires this baseline and an actual account of what remains missing for stochastic agents.

**Damage: High.** “No single demonstrated end-to-end solution was found” remains possible. “The essential integration and delegation semantics do not exist” needs narrower definitions and a substantially stronger search.

## 8. Strict semantics does not imply tractable verification or arbitrary-scale deployment

**Claim attacked:** L26: strict semantics means “verifiable locally, scales to any size”; lax semantics needs global analysis or a bound on the gap.

**Classification: Verified wrong as a mathematical implication.**

Strict preservation of composition is an equality, not a complexity bound, an algorithm or a decision procedure. The identity functor is strict on any suitable semantic category; choosing it cannot make a hard semantic property easy. Even if component behaviour composes exactly, computing that behaviour or testing the desired property may be intractable or undecidable. Conversely, a lax abstraction may give a sound local sufficient condition for a safety property without a numerical “gap.”

The recent systems paper itself separates system behaviour and behaviour composition, and does not promise an arbitrary-scale local verifier. [Libkind–Myers](https://arxiv.org/abs/2505.18329). In the 2026 clock-systems paper, §6.1 explains representable behaviour and its compositionality; it does not derive computational scalability. [Full source](https://arxiv.org/html/2603.29573v2).

**Damage: High.** This removes a proposed acceptance test that confuses algebraic compositionality with verification cost.

## 9. Authority, obligation and delegation are conflated

**Claim attacked:** L123: pi-calculus restriction, actor locality, object capabilities, Hohfeldian power, speaks-for and delegable commitment “describe the same thing”; attenuation makes chains verifiable offline.

**Classification: My judgement, supported by incompatible semantics.**

Name scope restricts access to names; a commitment assigns responsibility for a condition; speaks-for licenses inference from one principal's statements to another's. None is automatically an attenuating capability. Delegating a debtor's commitment changes who owes performance; it need not grant access to a resource, and delegating a capability need not create any obligation to use it. The report supplies no interpretation preserving these distinctions.

The proof-carrying authorization source explicitly separates logical authorization policies, proofs and delegations. [Bauer et al.](https://users.ece.cmu.edu/~lbauer/papers/pcaprototr.pdf). Offline verification additionally requires trusted initial credentials, an understood policy language, and an acceptable treatment of expiry and revocation. Attenuation alone supplies none of these.

At scale, Zanzibar provides externally consistent authorization amid changing ACLs, using coordinated storage and causal ordering. It is not evidence that authorization succeeds solely by local monotonicity. [Primary system paper](https://www.usenix.org/conference/atc19/presentation/pang).

**Damage: Medium–high.** The purported cross-field convergence erases exactly the semantics the proposed foundation must integrate.

## 10. Holmström does not prove recursive ownership or a residual controller at every level

**Claim attacked:** L62 calls the team theorem a formal argument for nesting; L127 says every level needs a named residual claimant and residual controller.

**Classification: My judgement—unsupported extrapolation from a real theorem.**

*Moral Hazard in Teams* analyzes incentive compatibility under a specified production, observability and sharing model. Budget breaking permits incentive schemes excluded by exact team budget balance. The paper discusses a principal administering credible incentives, and its example can leave no residual on the equilibrium path. It does not prove that every organisational level must contain an owner/controller or that recursion is necessary. [Original paper, especially pp. 326–328](https://people.duke.edu/~qc2/BA532/1982%20Rand%20Holmstrom%20team.pdf).

The unobservability, objective, payoff and sharing assumptions have to be established separately at each proposed level. Mathematical processes, public institutions and autonomous humans do not automatically instantiate them. A budget breaker is also not synonymous with the residual control rights of incomplete-contract theory.

**Damage: Medium–high.** Remove the theorem-grade endorsement of nesting and residual control; retain a conditional incentive-design consideration.

## 11. The 2026 categorical results are real, but their scope is enlarged and then denied

**Claim attacked:** L24's stochastic breakthrough and L163's “no theorem says what self-rewiring organisations do.”

**Classification: Verified scope mismatch; my judgement about the sweeping negative.**

The March 2026 clock paper exists. Theorem 4.10 identifies behaviours with maps out of a clock for its specified stochastic systems. Crucially, §3.2 restricts stochasticity to systems rather than lenses or charts, explicitly to make that theorem work. Section 5 defines nondeterministic behaviour via a clock; it does not recover a previously established classical definition by an analogous theorem. [Full paper](https://arxiv.org/html/2603.29573v2). “The stochastic case since 2026” is too broad without this scope.

Spivak's February 2026 paper also exists. It constructs polynomial trees, a monoidal closed category and an OrgTr bicategory for interactions that change interfaces. [Primary source](https://arxiv.org/abs/2602.17917). That is already a formal account of certain changing-interface systems. It proves neither safety nor accountability of social organisations, but the report must distinguish those missing guarantees from the absence of *any* theorem about dynamic structure.

**Damage: Medium.** These are not fabricated IDs. Correct the scope, and replace the blanket negative with the specific missing governance properties.

## 12. The empirical agent evidence does not establish a general hierarchy advantage

**Claim attacked:** L169: measured agent teams track organisational theory—hierarchies contain error and flat swarms amplify it.

**Classification: My judgement—selective extrapolation; underlying quoted figures verified.**

Kim et al. does report 180 configurations and 17.2× versus 4.4× error amplification. But it stresses task contingency, and its Table 5 reports aggregate success of 0.477 for decentralized systems versus 0.463 for centralized ones. The “independent” architecture is not the same as a communicating decentralized organisation. Its limitations exclude important long-horizon and real-world feedback settings. [Full paper, Table 5 and §5](https://arxiv.org/html/2512.08296v2).

The Cournot paper reports the quoted reduction from 3.1 to 1.8, across six configurations and 90 runs per condition. It tests a specific enforcement regime, not a general theory of institutional governance. Its first listed author is M. Bracale Syrnikov; “Pierucci et al.” is not the usual lead-author citation for this paper. [Full paper](https://arxiv.org/html/2601.11369).

Jones, Dragan and Steinhardt's 43% versus under 3% result also checks out. It concerns adversarially arranged model combinations on a particular vulnerable-code task; it is not the failure of a composition theorem for formally safe components. [Full paper, §4.1](https://arxiv.org/html/2406.14595v2).

**Damage: Medium.** Retain these as bounded experimental observations; remove the general architectural inference from the conclusion.

## 13. The 10^10 argument is a scenario, not a demonstrated necessity result

**Claim attacked:** L112's quadratic agreement cost, scale-independent audit sampling, and L3's assertion that control is achievable only inside the recommended structure.

**Classification: My judgement; arithmetic qualifications independently derived.**

The 34-bit name count and roughly 5×10^19 unordered pairs are correct. A quadratic lower bound in the number of tolerated Byzantine faults does not become 10^20 messages merely because there are 10^10 actors: one must specify that they are all consensus participants, how faults scale, the protocol model, and whether the argument concerns worst-case or ordinary operation. The report itself proposes committees, so actor count and agreement membership must not be silently equated. The cited source is specifically a bound on Byzantine agreement, not on all governance interactions. [Dolev–Reischuk](https://doi.org/10.1145/2455.214112).

For independent uniform audit samples, missing a violation of fixed prevalence p after m samples has probability (1-p)^m. Hence m ≥ log(delta)/log(1-p) is independent of population **only while p stays fixed**. Finding one malicious actor among N has p=1/N and needs order N samples for fixed detection confidence. Strategic concealment and inaccessible events further invalidate the simple sampling model. This elementary calculation qualifies, rather than refutes, the report's explicitly fixed-frequency statement; what fails is its use to justify “sampled audit works” as a governance answer.

Neither argument proves feasibility of the recommended architecture. A resource budget, communication model, adversary model, workloads and operational meaning of control remain unspecified.

**Damage: Medium–high.** The deployment claim must remain open. The report supplies no constructive scaling demonstration and no exhaustive impossibility result for alternatives.

## 14. “General agents contain world models” loses decisive assumptions in the synthesis

**Claim attacked:** L52 presents extraction of world models from general multistep agents as a strengthened good-regulator result supporting the kernel's model requirement.

**Classification: Verified missing assumptions; not a fabricated result.**

Richens, Abel, Bellot and Everitt assume a communicating, stationary, finite controlled Markov process with at least two actions. Their extraction result uses a specifically defined bounded goal-conditioned agent and access to its policy. The paper explicitly limits its results to simple environments and discusses extensions as future work. [Full paper, Assumption 1, Definition 5, discussion](https://arxiv.org/html/2506.01622v5).

An arbitrary institution, partially observed human team or changing agent population is not automatically such an agent/environment pair. The theorem is useful evidence for a model requirement in its domain, not a proved universal requirement for governance.

**Damage: Medium.** Add the assumptions wherever the result is used to turn an architectural preference into a necessary condition.

## Checks that survived, and limits of this audit

The following specific claims survived the attempted checks; this does not endorse their broader extrapolations:

- **Withdrawn delegation paper:** arXiv 2410.08373 is withdrawn, and its withdrawal notice explicitly identifies the pre-monoidal versus monoidal error. [Record](https://arxiv.org/abs/2410.08373). That says nothing about the nonexistence of every other delegation calculus.
- **Recent categorical references:** 2505.18329, 2602.17917 and 2603.29573 exist with the relevant authors and topics. Their existence survives; the scope issues are documented above.
- **CaMeL:** the source reports 77% AgentDojo task completion versus 84% undefended, with guarantees under its stated trusted-query, control-flow and policy assumptions. [Primary source](https://arxiv.org/abs/2503.18813). This is not a universal information-flow guarantee for arbitrary LLM use.
- **FLP:** the source supports the appropriately scoped deterministic/asynchronous/one-crash impossibility. It does not establish a general impossibility of governing large organisations.
- **The agent-study numbers** checked above occur in the sources. I did not replicate the experiments or audit the datasets.

**Could not verify either way in this audit:** the exhaustive empirical ceiling (“none” above hundreds of formal components or dozens of cooperating agents); the absence of any end-to-end calculus satisfying the complete nine-item package; the strongest 2026 programme-change claims concerning ARIA; the October 2026 UPI figure; the 2026 routing, identity and party-membership counts; and the complete bibliographic accuracy of all dagger-marked references. These should remain unverified, not be reported as errors or silently treated as confirmed. The report's own URLs identify the claims to check: [ARIA](https://aria.org.uk/opportunity-spaces/trust-everything-everywhere/safeguarded-ai), [UPI report](https://www.freepressjournal.in/business/upi-transactions-hit-2547-billion-in-october-value-soars-to-3153-lakh-crore), [routing report](https://seclists.org/nanog/2026/Apr/44), [UIDAI](https://uidai.gov.in/aadhaar-dashboard/enrollment-dashboard), [party membership report](https://news.cgtn.com/news/2026-06-30/CPC-membership-reaches-101-286-mln-1OoiX3xQAog/p.html).

I also do not treat the broad “no complete solution” claim as refuted by any single omitted framework. The counterexamples above defeat particular absence and impossibility claims, not establish a complete planetary-governance foundation.

## Verdict

**Stand:** Mathematical closure and conditional composition are available in several established formalisms. Verification and consensus have real limits under their stated models. No ten-billion-actor integrated governance system is demonstrated by this report. The report's admission that its nine-item synthesis lacks minimality, completeness and implementation proofs is correct.

**Must be weakened:** “Fractality is proven” must become “closure is proved within particular formal classes; cross-scale adequacy is unproved.” “Control is conditional” stands as a methodological statement, but is nearly tautological unless the conditions and guarantees are specified. The narrow waist and nine primitives are design proposals requiring comparison and construction, not consequences of convergent theorem-grade evidence. The empirical case for hierarchy is task-specific.

**Withdraw:** the unrestricted expressiveness-versus-strong-guarantees dichotomy; the reversed Hourglass Theorem; the blanket absence of compositional viability results; the safety-only reading of compositional guarantees; and the shared conservation-law interpretation of Iris, accounting and authority. Narrow the claimed absence of delegation and dynamic-structure semantics.

**Does it answer the question?** Only partially. It compares many relevant ingredients, then replaces the requested constructive existence/comparison problem with an impossibility claim about much stronger requirements—complete automatic verification, unconstrained aggregation and unrestricted self-certification. It neither constructs the requested foundation nor rules it out. The defensible answer is: several smaller rigorous substrates already exist; none is shown here to deliver the entire requested combination, and this report has not established that the combination is impossible.

# Adversarial review, round 2

Reviewed 17 November 2026. I formed an initial view from the current report before reading the previous attacks and comparing the versions. I consulted the researcher notes, then independently searched and fetched primary sources, including several sources introduced by the revision. Neither the report nor the notes was modified. This is a targeted adversarial audit, not a verification of every citation or proof.

**Classification:** “Verified wrong” means a source, mathematical counterexample, or explicit internal contradiction defeats the wording. “Could not verify” denotes insufficient access or evidence, not falsity. “My judgement” marks an assessment of significance, coverage, or inference. Severity applies to the current version, including its explicit qualifications, not to errors already removed from v1.

## 1. HIGH — The nearest formal work on governance across abstraction levels is still missing

**Claim attacked:** “What is not proven anywhere is that governance properties (authority, membership, liability, control) recur faithfully from resource to nation”; “norms exist in three untranslated forms … with no refinement or soundness relation among them”; the comparison gives normative multi-agent systems only “Decentralised enforcement of a shared law” and “Composition metatheory is thin.”

**Classification: My judgement about the significant omission; verified existence of substantially stronger partial results.** I am not claiming that the full resource-to-nation requirement has been solved.

King, De Vos, Dignum, Jonker, Li, Padget and van Riemsdijk, *Automated Multi-level Governance Compliance Checking* (2017), directly treats governance of lower-level institutions by higher-level institutions at different abstraction levels. Its formal semantics relates concrete and abstract norms; its translation into AnsProlog has soundness and completeness theorems for a given event trace. See §5.5, Theorems 1–2, with proofs in Appendix C. This is precisely the sort of partial bridge the report says is absent. It does **not** prove faithful translation of arbitrary natural-language law or supply a planetary implementation. [Primary manuscript](https://orca.cardiff.ac.uk/id/eprint/130977/1/AS4805164377456641491575454833_content_1.pdf), [publication record](https://researchportal.bath.ac.uk/en/publications/automated-multi-level-governance-compliance-checking/).

The new LGI row is also an inadequate repair. Minsky's 2014 *Dependable Management of Untrusted Distributed Systems*, §4.1, describes hierarchies of different laws with conformance enforced by construction, including each law's control over permissible subordinate changes, and enforced obligations. The work explicitly includes management of the management process. These are much closer to recursive governance than merely enforcing one shared law. Its limitations include trusted middleware and unresolved application to legacy systems. [Primary paper, especially pp. 8–11](https://arxiv.org/pdf/1403.4321). The earlier interaction-control treatment also has budget, capability and delegation examples. [Primary paper, §§2.4, 3.3.1, 3.4.1](https://www.cs.rutgers.edu/~minsky/papers/IC-model.pdf).

**Why it matters:** The report's remaining integration agenda is too nearly a list of things to invent. These works change what the starting point should be. A survey that promotes a new checklist should first compare existing institutional semantics and recursive enforcement mechanisms, not just list their field.

**Exact fix:** “Formal cross-level normative compliance and recursively constrained policy hierarchies already exist, notably King et al. (2017) and LGI conformance hierarchies. We found no demonstrated integration of these mechanisms with all the requested physical, human, organisational and AI semantics at planetary scale. The missing bridges include validated natural-language interpretation, legal liability and implementation-to-world correspondence.” Add both as substantive comparison entries. Restrict the opening negative claim to the **complete combination**, not cross-level governance in general.

## 2. HIGH — The acceptance tests still turn general undecidability into prohibitions on particular successes

**Claim attacked:** “Claims that a system proves its own consistency, safety or alignment … that an optimal decentralised policy or global plan is computed … each contradict a specific result.” Also: “Its guarantees hold for all sizes by a parametric argument … because checked instances do not generalise.”

**Classification: Verified wrong as universal claims; my judgement that this materially distorts the design requirements.**

There is no theorem prohibiting computation of an optimal policy for a particular finite problem. Hardness and undecidability statements quantify over problem classes and algorithms; they do not prohibit successful instances. The corrected Finkbeiner–Schewe citation does concern an architecture-level decision problem for synchronous finite-state distributed synthesis with temporal specifications. It is not a prohibition on solving every instance on an architecture with an information fork. [Primary paper](https://finkbeiner.groups.cispa.de/_assets/paper.BUE6QkoK.pdf).

Similarly, consistency of a sufficiently strong effective theory, unrestricted reflection about that theory's proofs, and a particular safety invariant of a transition system are not interchangeable. A system can have an inductively proved safety invariant without thereby proving the consistency of its reasoning theory. The report's own proof-carrying proposal depends on this distinction. Mechanised self-formalisation already provides qualified soundness results with explicit universe assumptions, rather than an indiscriminate prohibition. [Kumar et al., *Self-Formalisation of Higher-Order Logic*](https://research.chalmers.se/publication/234751/file/234751_Fulltext.pdf).

Finally, the user asks for **about 10^10 actors**, not all finite populations. A verified finite upper bound with explicit workload and failure assumptions could satisfy that request. Parametric verification is valuable but is not logically required. An isolated instance does not establish an unbounded claim; it can establish the bounded claim actually tested.

**Exact fix:** “Red flags are claims of a sound, complete automatic solution for unrestricted verification or synthesis problems, or unconditional self-consistency proofs by theories satisfying the incompleteness hypotheses. Particular safety proofs and optimal solutions to restricted or individual instances are not excluded. State whether scalability is guaranteed up to a specified bound, proved parametrically, or only measured; match that evidence to the claimed population and workload.”

This is HIGH, not CRITICAL: the introduction and conclusion now make the right conditional distinction. The acceptance tests fail to maintain it.

## 3. HIGH — A mature, small compositional baseline is omitted from the candidate comparison

**Claim attacked:** The concluding shortlist is “polynomial functors, double-categorical systems theory, bigraphs, a dependent type theory with a resource layer, reflective rewriting logic, Mesarović's coordination theory”; the nine-element discussion says all nine “can be encoded in an open transition system with a composition operator and a refinement logic.”

**Classification: My judgement — significant missing candidate, not a claim that it solves the whole problem.**

The report proposes an open-transition-system baseline without evaluating one of its most established realizations: **Lynch–Tuttle input/output automata**, including composition, hiding, fair executions, implementation relations and local proof techniques. These supply much of the small semantic core against which the more elaborate candidates should be measured. The model distinguishes environment-controlled inputs from locally controlled outputs and internal actions, an especially relevant distinction for governing autonomous participants. [Authors' introductory paper](https://groups.csail.mit.edu/tds/papers/Lynch/CWI89.pdf).

TLA is cited through Abadi–Lamport but likewise not compared as a foundational candidate. *Conjoining Specifications* supplies both decomposition and assumption/guarantee composition results within a common temporal semantics. [Primary paper, §§3–5](https://lamport.azurewebsites.net/pubs/abadi-conjoining.pdf).

This omission matters because the survey is choosing mathematical packaging, not merely collecting applications. Nothing shown establishes that Poly is smaller, more general for the desired properties, or easier to verify than these baselines once all encodings and assumptions are charged. The report admits that its comparison is incomplete; that admission avoids a CRITICAL rating but does not make a significant missing candidate immaterial.

**Exact fix:** “Include I/O automata and TLA-style transition-system specifications as baseline candidates. Compare syntax size, composition and refinement results, fairness, environmental assumptions, and the cost of encoding authority, norms and resources. The current evidence does not rank Poly or dependent type theory as the most fundamental foundation.”

## 4. MEDIUM — The report both supplies and denies a composable incentive surrogate

**Claim attacked:** “Eighth, composable surrogates for alignment and incentive compatibility do not exist.” Earlier: “the composable surrogate, smoothness, preserves only approximate efficiency.”

**Classification: Verified internal inconsistency; verified source mismatch for the blanket absence claim.**

Syrgkanis–Tardos explicitly construct smooth and weakly smooth mechanisms whose efficiency guarantees survive specified simultaneous or sequential compositions, under valuation and bidding assumptions. [Primary paper](https://arxiv.org/abs/1211.1325). This does not make dominant-strategy incentive compatibility compose universally, nor does smoothness itself imply truthfulness. But the report cannot call it a surrogate in its evidence section and declare all such surrogates nonexistent in its research agenda.

A further quantifier issue: even exact incentive compatibility is preserved in elementary separable cases. If the agent's utility is the sum of independent utilities and each component makes truthful reporting dominant, summing those component inequalities proves truthful reporting dominant in the product. Complementarities, coupling constraints and changing information can break that argument; arbitrary composition is the problem.

**Exact fix:** “Composable approximate-efficiency guarantees exist for restricted mechanism classes, notably smooth mechanisms. We found no general compositional guarantee of truthfulness or alignment for heterogeneous, learning, self-modifying organisations; specify which objective and composition operator the proposed surrogate is intended to preserve.”

The broad headline that guarantees are per-property survives.

## 5. MEDIUM — Initial algebra versus final coalgebra is a false mandatory choice, and coalgebras can terminate

**Claim attacked:** “a final coalgebra for open-ended behaviour (bisimulation and safety, no termination)”; “It declares whether its systems are an initial algebra or a final coalgebra.”

**Classification: Verified wrong.**

Ordinary systems can be coalgebras **mapping into** a final coalgebra; each individual system need not be the final object. Nor does finality imply nontermination. For the functor F(X) = 1 + A × X on sets, the final coalgebra consists of finite **and** infinite A-sequences. A finite sequence terminates at the 1 summand. One can prove that a particular state's behaviour lies in the finite part while retaining a semantic domain that also supports infinite behaviour. Rutten's account expressly includes deterministic systems with termination. [Primary paper, examples and final-system treatment](https://fldit-www.cs.tu-dortmund.de/~peter/Rutten/UniversalCoalgebra.pdf).

Finite inductive syntax and coalgebraic operational behaviour can coexist. Requiring an exclusive choice wrongly excludes normal combinations of finite descriptions, terminating transactions and continuously running services.

**Exact fix:** “Specify the inductive structures, operational coalgebras and behavioural equivalences used. Where initial or final objects exist, state their role. Prove termination, productivity or liveness for the relevant components and operations separately; an ongoing service may support terminating requests.”

## 6. MEDIUM — seL4 is misclassified as evidence for dependent type theory

**Claim attacked:** “The most general kernel is dependent type theory … It has carried the largest verified artefacts”; the associated comparison row lists “Lean, Rocq, Iris” and gives “seL4, CompCert, Cedar's verified model” as evidence.

**Classification: Verified wrong for the classification; could not verify the comparative superlatives.**

The seL4 evidence is an Isabelle proof, as the report's own source says. Isabelle/HOL uses higher-order logic; it is not the Lean/Rocq dependent-type-theory foundation. The project's primary overview also describes the LCF approach to preserving soundness. [Isabelle overview](https://isabelle.in.tum.de/overview.html), [seL4 proof statistics](https://trustworthy.systems/projects/OLD/l4.verified/numbers). Iris is a program logic/framework rather than a rival foundational proof kernel of the same category as Lean or Rocq.

The corrected seL4 quantities are supported, but they do not select dependent type theory as the privileged substrate. “Most general,” “smallest algebra,” and “only small trusted checker” need specified metrics or comparison classes; the report supplies none.

**Exact fix:** “Small trusted proof kernels support general-purpose formal verification in several foundations, including dependent type theories and higher-order logic. seL4 uses Isabelle/HOL; CompCert and Iris provide evidence for the Rocq ecosystem, and Cedar for Lean. These examples establish feasibility and cost, not a unique or most general foundation.”

## 7. MEDIUM — The scale evidence changes its unit and contradicts its own exclusivity claim

**Claim attacked:** “The only governed actor systems at 10^9 scale are digital public infrastructure with a unique centrally issued identity”; “nothing formally founded has been exercised near 10^10 actors”; “Largest formally founded artefacts are single kernels.”

**Classification: Verified wrong for the first sentence; my judgement / could not verify for the broader scale comparison.**

The report itself discusses law, commercial services and Zanzibar at billions-of-user scale. Zanzibar is an authorization system for hundreds of Google services; its primary paper reports billions of users, trillions of ACLs and millions of authorization requests per second. It is not digital public infrastructure in the Aadhaar/UPI sense. [Primary publication](https://www.usenix.org/conference/atc19/presentation/pang).

More fundamentally, user counts, issued identifiers, transactions per month, cluster nodes, ACLs, autonomous systems and independently governed active principals are different denominators. Twenty-five billion transactions do not demonstrate twenty-five billion actors. Billions of users do not imply billions of simultaneous policy decisions or control loops. A kernel's program size is not a population count. These examples are valuable, but they do not establish an empirical capacity boundary at a common actor scale.

I found no counterexample demonstrating the **full requested combination** near 10^10 actors. I also cannot verify the broad absence claim as written: “formally founded” and “near” lack operational definitions, and the user explicitly includes resources and programs among entities.

**Exact fix:** “Observed billion-user deployments include federated networks, public identity systems and centrally operated commercial services. These demonstrate different restricted functions and count different units. No deployment identified in this survey demonstrates the full requested governance semantics for approximately 10^10 independently acting principals. This is a search result, not an established upper capacity bound.” Replace “only” and define actor count, concurrency, workload, fault model and property before comparisons.

The two architectural shapes remain useful examples, not an exhaustive or disjoint classification: a service can have central identity issuance and federated execution simultaneously.

## 8. MEDIUM — The 2026 clock result answers a narrower open question than the report states

**Claim attacked:** “Whether stochastic and non-deterministic systems theories admit well-behaved behaviour functors was open until March 2026.”

**Classification: Verified scope overstatement.**

The authors say the open question was whether these systems theories supported **clock systems**, which represent particular behaviour functors and simplify their coherence proofs. Their §1.2 specialises to monadic Moore machines, and the paper constructs representations of classical stochastic behaviours. That is not the same as saying well-behaved stochastic or nondeterministic behavioural semantics were unavailable before 2026. [Primary text, §§1.1–1.2 and 6](https://arxiv.org/html/2603.29573v2).

The revision correctly restricts stochastic interfaces later in the sentence; it did not correct the quantifier at the beginning. The narrower theorem is still a substantive result.

**Exact fix:** “In March 2026, Lynch, Myers, Rischel and Staton established clock-system representations for the stochastic and nondeterministic categorical systems theories studied in their paper, resolving a representability question in that framework. This is not the origin of stochastic compositional semantics generally.”

## 9. LOW — The new Jordan citation is real, but its theorem has more hypotheses than convexity

**Claim attacked:** “The competitive price mechanism is the unique informationally decentralised mechanism that achieves efficient, individually rational allocations with a message space of minimal dimension … a theorem about classical convex economies.”

**Classification: Verified omission of hypotheses, not a false minimality result.**

Jordan's abstract explicitly includes regularity conditions alongside efficiency, individual rationality and dimensional minimality. The technical uniqueness result involves an appropriate equivalence of message descriptions, not literal uniqueness of syntax or algorithm. See the paper's definitions and uniqueness results. [Primary paper](https://gwern.net/doc/economics/mechanism-design/1982-jordan.pdf).

Thus the correction from v1 is substantially sound. It should not be read as a theorem over every imaginable decentralized encoding, nor as a theorem of minimal computation, communication bits, incentives or convergence time.

**Exact fix:** “Within Jordan's exchange-economy domain and regularity conditions, the competitive allocation process is uniquely informationally efficient in the paper's specified sense of message-space dimension and equivalence. This is a local minimality theorem, not a minimal foundation for governance.”

## Audit of the eight headline conclusions

These assessments are not additional counted findings.

| Headline | Assessment and strongest counterargument |
|---|---|
| Small rigorous substrates exist | **Established**, if “small” means a compact formal signature. **Unestablished** as a comparative minimum: encoding governance and proving the encodings correct may dominate the substrate. I/O automata is a serious omitted baseline. |
| Closure is by construction; cross-scale governance is unproven | **Established for specified constructions**, and close to definitional once the semantic class is chosen. The full mixed-domain claim remains unproved in the reviewed evidence. The strongest counterargument is the existing formal work on cross-level institutional compliance and conformance hierarchies: partial governance recursion is already more than syntax. |
| Guarantees are conditional and per-property | **Established**, though nearly a methodological truism without actual hypotheses. Per-property analysis does not forbid one theorem handling a class of properties. The acceptance tests must retain the distinctions the headline now makes. |
| Proof-carrying design is the supportable direction | **Supportable engineering judgement, not an established winner or necessary consequence.** Proof checking is one route; construction, confinement, monitoring and incentive design are alternatives or complements. A certificate connects formal descriptions, not automatically people and physical outcomes. The report should specify the trusted enforcement path before treating checked evidence as governance. [Necula's foundational case is code against a consumer's safety policy](https://homes.cs.washington.edu/~mernst/teaching/6.893/readings/necula-popl97.pdf), not proof that an arbitrary human will comply. The report admits this integration is open, so this is not a CRITICAL finding. |
| Nine elements are a checklist, not a kernel | **Correct self-description.** Recurrence does not show necessity, independence, consistency or sufficiency. The strongest counterargument is that many items can be derived encodings, while identity, observations, time and goals carry much of the actual difficulty. The revision now acknowledges this. |
| Two shapes exist at 10^9 | **Supported as examples**, not as an exhaustive theorem or dichotomy. Central roots and federation are compatible dimensions; “only digital public infrastructure” is wrong. |
| Nothing is demonstrated near 10^10 | **Could not verify as an unrestricted world claim.** Defensible as “this survey identified no demonstration of the full combination.” Counts and governance properties need normalization before this becomes an empirical finding. |
| Full combination neither constructed nor shown impossible | **The defensible answer on the evidence presented.** Not a proof that nobody has constructed any admissible interpretation of the target. A permissive definition allows trivial encodings; an excessively strong definition builds impossibility into the requirement. An operational target is needed to make the existence question decidable even in principle as a research claim. |

## Revision fidelity and verification limits

The large corrections are mostly faithful. The full-combination caveat, distinction between closure and governance, per-property guarantees, conditional liveness, resource-algebra caveat and task-dependent interpretation of agent-team experiments should remain. I am not reissuing v1's CRITICAL dichotomy finding. The current Hourglass direction is mathematically the appropriate correction, although the author's PDF fetch timed out during this audit, so I do not count it as a newly completed source verification.

Independent spot-checks confirmed the withdrawn status and pre-monoidal explanation for [arXiv:2410.08373](https://arxiv.org/abs/2410.08373), the existence and interface-changing construction of [Spivak 2026](https://arxiv.org/abs/2602.17917), the clock paper and its March/April 2026 versions, and the corrected Jordan citation. The corrected [Finkbeiner–Schewe paper](https://finkbeiner.groups.cispa.de/_assets/paper.BUE6QkoK.pdf) was retrievable through the authors' current publication page even though the old PDF location returned 404. The seL4 source supports the revised proof/code counts. These checks do not independently verify the proofs.

For recent empirical material, [Kim et al. v2](https://arxiv.org/abs/2512.08296v2) is a real, version-specific source; the report now includes the sequential-task caveat. The publisher record explicitly identifies a later April 2026 version. Citing v2 is not itself false, but a late-2026 synthesis should explain the version choice or reconcile changed results. [The Cournot-governance paper](https://arxiv.org/html/2601.11369) is real; its result remains experiment-specific, as the revised conclusion says. Neither check supports a general theorem that hierarchy works.

**Could not verify:** I did not establish that every 2025–2026 item is correct, that all dagger-marked classical references have correct theorem hypotheses, or that any literature-wide negative is exhaustive. GLEIF's Q2 2026 page was intermittently retrievable and subsequent fetches timed out; its search record distinguishes issuance from renewal. I therefore do not certify the stronger wording “active identifiers recording parent relationships” merely from a headline total. [Source to inspect](https://www.gleif.org/en/newsroom/blog/the-lei-in-numbers-q2-2026-signals-growing-regional-momentum-for-the-lei). I did not independently revalidate the ARIA chronology or the UPI, CPC and routing totals in this pass. Earlier reviewers' statements that these were checked are provenance, not independent verification by this reviewer.

The revision's newly named sources should be tagged as reviewed at the level actually checked: full theorem and assumptions, full-paper reading, abstract, search excerpt, or inherited review assertion. “Unmarked citations were checked this session” is too coarse to communicate these differences reliably.

## Does it answer the question? Verdict

It now gives a defensible provisional answer: compact compositional mathematics exists, and no reviewed result establishes or rules out the full requested system. It does not yet give the requested comparative evaluation of the strongest complete candidates. The leading candidates are not put through the same resource–program–service–human–team–organisation–government construction, and neither “controlled” nor “smaller” has an agreed test. The report openly admits those limitations; I do not inflate them into CRITICAL findings.

The strongest constructive challenge is to choose an existing transition-system baseline, an institutional compliance model and a recursive enforcement scheme, then exhibit one common example with explicit authority, resource, norm, observation and rule-change semantics. Compare that against Poly, bigraphs, type theory and reflective rewriting on encoding size, trusted assumptions and preservation proofs. That would test whether the proposed integration is new and whether its bottleneck is mathematics, enforcement, legitimacy or workload.

**Verdict:** The revised headline survives, but several supporting claims and the prescribed acceptance tests require correction. The most damaging remaining weaknesses are missing formal institutional prior art, missing conventional compositional baselines, and repeated slippage from impossibility of unrestricted decision procedures to impossibility of particular successful systems. No source checked here justifies reversing the headline into either a positive construction claim or an impossibility claim.

CRITICAL: 0  HIGH: 3  MEDIUM: 5  LOW: 1

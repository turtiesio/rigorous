# Prior art and foundation selection

**Provenance of this survey.** It is written from the author's knowledge of the literature. No source was
re-fetched or re-read for this document, so treat the citations as pointers to check, not as verified
quotations. Nothing below is a new result; the point is to decide what to reuse.

## 1. Comparison

Columns: what the formalism *is* mathematically; its composition law; what is decidable; what must be
trusted; what we take from it; and where it stops being enough on its own.

| Family | Semantics | Composition law | Decidability | Trusted assumptions | Taken into this system | Limit as a sole foundation |
|---|---|---|---|---|---|---|
| **I/O automata** (Lynch–Tuttle) | Labelled transition systems with input/output/internal actions; inputs always enabled | Parallel composition synchronising on shared actions; requires disjoint outputs; trace inclusion is a precongruence | Finite-state: reachability decidable; general: not | Fairness for liveness | **The unit model**: input-enabled units, one owner per output kind, composition by shared event kinds | Says nothing about norms, evidence or authority |
| **TLA+** (Lamport) | Temporal logic of actions over one global state | Conjunction of specs; refinement = implication | Model checking of finite instances (TLC); proofs in TLAPS | The spec matches the system | Refinement as implication between behaviours; invariants as inductive predicates | One global state vector; composition is not first-class |
| **Process calculi** (CCS, CSP, π) | Structural operational semantics; bisimulation / failures | Algebraic laws (∥ associative, commutative) | Bisimilarity decidable for finite-state; π-calculus undecidable | — | Associativity/commutativity as *required laws* of composition; hiding as scoping | Mobility (π) costs decidability; no data-level norms |
| **Rewriting logic** (Maude) | Terms modulo equations, concurrent rewrites | Module composition | Reachability semi-decidable | — | Idea that concurrent, independent rewrites commute | Very general: gives no narrow waist by itself |
| **Mazurkiewicz traces / event structures** | Words modulo an independence relation; partial orders of events | Trace concatenation | Many trace problems decidable for finite alphabets | Independence relation is correct | **Partial-order view of the log**: independent events commute (T5) | Needs an independence relation supplied from elsewhere |
| **Type theory / proof assistants** (Lean, Coq) | Dependent types; proofs are programs | Function composition; modules | Type checking decidable; proof search not | The kernel; the statement actually says what you meant | **Mechanised proofs** of the composition lemmas (`proofs/Rigor.lean`) | A proof about a model is not a proof about the running system |
| **Refinement calculi / simulations** | Forward/backward simulation relations | Simulations compose horizontally and vertically | Finite-state: decidable | — | Refinement = forward simulation with equal outputs (T2, T3) | Finding the relation is manual |
| **Separation logic, resource algebras** (Reynolds, O'Hearn; Iris) | Partial commutative monoids; frame rule | Separating conjunction; local reasoning | Fragments decidable | Footprints are honest | **Resources as a commutative monoid; local invariants compose over disjoint state** (T1, T7) | Higher-order ghost state is far more than needed here |
| **Category theory of open systems** (operads of wiring diagrams, lenses, coalgebras; Spivak, Jacobs, Myers) | Systems as coalgebras / lenses; wiring as operad algebra | Functorial: composing then interpreting = interpreting then composing | Not the right question | — | The *shape* of the claim "a system of systems is a system", and that scoping must be functorial (tested) | Gives structure, not content: no norms, no time, no evidence |
| **Assume–guarantee contracts, interface automata** (Benveniste et al.; de Alfaro–Henzinger) | Pairs (A, G) of behaviours; compatibility games | Contract composition and refinement | Finite-state: decidable | Assumptions hold of the environment | Vocabulary of compatibility; knowledge that circular A/G is unsound for liveness (counterexample kept) | Contracts are about behaviour, not about who answers when they fail |
| **Standard deontic logic** | Modal logic KD: O, P, F over worlds | Boolean | Decidable | — | Negative lesson only | Contrary-to-duty paradoxes (Chisholm): cannot say "violated and a secondary duty holds" |
| **Social commitments** (Singh; Chopra; Telang) | Objects C(debtor, creditor, antecedent, consequent) with a lifecycle | Commitment protocols compose by events | Lifecycle is finite-state | Events are observed faithfully | **The single normative primitive** | No account of evidence, authority, or revision |
| **Institutional action / count-as** (Searle; Jones–Sergot; InstAL; event calculus) | Brute events generate institutional events under constitutive rules | Institutions as rule sets | ASP: decidable, expensive | The rule set is the institution | **Authority as count-as**: an attempt counts as an act iff empowered | Usually a closed world with one institution |
| **Hohfeld** | Eight relations in two squares: claim/duty/privilege/no-claim; power/liability/immunity/disability | — | — | — | The analysis that permissions and powers are *different kinds* (first-order vs second-order) | A taxonomy, not a calculus |
| **Enterprise ontology** (DEMO, REA) | Transaction patterns: request–promise–execute–state–accept | — | — | — | Request → commit → perform → claim → accept as the scenario's skeleton | Fixed ontology; we avoid committing to one |
| **Object capabilities, SPKI/SDSI, macaroons, UCAN** | Unforgeable references; delegation chains with caveats | Attenuation is monotone | Chain validation linear in depth | Unforgeability; a root of trust | **Delegation as attenuating chains** (T6); revocation by chain | Revocation under partition is impossible in general (CAP) |
| **Authorization logics** (ABLP "says", Binder, SecPAL) | Modal/Datalog "A says φ" | Delegation as implication | Datalog fragments PTIME | Principals' keys | Rules as stratified Datalog | Richer than needed; "speaks for" amplifies easily |
| **Abstract argumentation** (Dung; ASPIC+; preference-based) | Directed attack graphs; extensions | Decomposes over connected components (grounded) | Grounded: PTIME, unique; preferred: NP-hard | Which arguments exist | **Grounded semantics for evidence assessment**; rebut/undercut; subargument attack | Abstract: we must define what an argument is |
| **Truth maintenance, belief revision** (Doyle JTMS; AGM) | Justification graphs; revision postulates | — | PTIME for JTMS labelling | Justifications are complete | Dependency-directed re-evaluation | AGM revises *beliefs*; we need revisable *institutional decisions* |
| **Epistemic logic** | Kripke models of knowledge | Product updates (DEL) | Model checking PSPACE-ish | Agents are logically omniscient | Negative lesson: we record what was *notified*, never what was *known* | Omniscience; no evidence objects |
| **Provenance (PROV), assurance cases (GSN)** | Derivation graphs; claim–argument–evidence trees | Modular cases | — | The argument is sound | `derived_from`; claims bound to evidence bound to artifacts | Mostly documentary: not executable, not revisable automatically |
| **Event sourcing, CRDTs, consistency models** | State = fold of an event log; join-semilattices | Merge is associative/commutative/idempotent | — | Log integrity | **Hash-chained log; state as a fold; content-derived ids** so independent creations commute | CRDTs cannot express non-monotone facts (revocation, spending limits) without coordination |
| **Stratified Datalog, incremental computation** (Adapton, Salsa, build systems à la carte) | Least models; dependency graphs with early cutoff | Rule union (if still stratified) | PTIME data complexity | — | **Rule language**; change propagation along dependencies | Negation needs stratification; no arithmetic |
| **Cybernetics, supervisory control** (Ashby; Beer's VSM; Ramadge–Wonham) | Feedback loops; supervisors restricting plant behaviour | Hierarchical/recursive (VSM) | Supervisor synthesis decidable for finite automata | Observability, controllability | Recursion of the same structure at each level; controllable (prevent) vs merely observable (detect) events | Synthesis blows up; VSM is informal |

## 2. What was selected, and why it is enough

The narrow waist is three things, all established:

1. **An append-only, hash-chained event log** (event sourcing). Everything else is a fold over it.
2. **Units** — deterministic, input-enabled open transition systems — closed under **parallel composition** and
   **boundary** (I/O automata + scoping). A composite is a unit; that is the whole recursion principle.
3. **Two off-the-shelf reasoners with unique, polynomial answers**: grounded argumentation and stratified Datalog.

On top, *as ordinary units and not as kernel features*:

| Concern | Formalism reused | Unit |
|---|---|---|
| Authority, power, delegation, revocation | count-as + attenuating capability chains | `inst` |
| Resources, ownership, conservation | ledger over a commutative monoid | `ledger` |
| Claims, evidence, provenance, revision | preference-based argumentation, grounded | `epi` |
| Duties, obligations, lifecycles, remediation | social commitments | `norms` |
| Responsibility, accountability, liability | stratified Datalog with proofs | `adj` |
| Rules, amendment, entrenchment | rules-as-data, validated before enactment | `gov` |

## 3. What had to be added, and the counterexample that forced it

Each addition is small and each is pinned by an executable test.

| Addition | Counterexample that forced it | Test |
|---|---|---|
| Commitment status is a **function of current facts**, not a stored state machine | With stored states, `satisfied` is terminal; a claim refuted after acceptance leaves the duty wrongly discharged | `main_scenario`, `model_check_single_commitment` |
| Derived duties carry a **basis** and are **voided** when the basis falls | Monitor retracts its probe: the remediation duty would otherwise survive the violation it remedied | `retracting_the_refutation_reinstates_everything_except_what_already_happened` |
| Composition delivers **one event at a time**; a single scheduler owns feedback | Run-to-completion composites are not associative | `run_to_completion_composition_is_not_associative` |
| A kernel **`settle`** marker at quiescence | Without it the adjudicator judged a half-propagated state and created, then voided, a liability within one cascade | found while testing; `main_scenario` asserts one verdict per cascade |
| **Content-derived identifiers** | Counter-based ids made independent stimuli non-commuting | `independent_stimuli_commute` |
| Evidence is **bound to artifact digests** and goes stale | A test that passed on yesterday's code "supports" today's | `evidence_is_bound_to_the_artifact_it_was_about` |
| Support **rooted in the claimant** is inadmissible, including via inference | An agent certifies its own claim by deriving it from its own statement | `no_self_certification` |

| **Sorts** on the predicates of the rule language | A rule with `Debtor`'s arguments swapped was accepted and silently derived nothing | `rules_without_a_unique_meaning_are_rejected` |
| Events carry **no actor field**; the kernel assigns it | In an untyped prototype a unit could put any name in its payload and had to be ignored correctly everywhere | `a_unit_cannot_impersonate` |

No new logic, calculus or ontology was introduced. The typing adds no semantics; it removes ways of
writing the same semantics wrongly.

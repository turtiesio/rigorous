# Adversarial review of Step 0 worked on paper

Reviewed the worked paper and the integrated plan. No other file was changed. The findings below distinguish **shown wrong** (a calculation or explicit execution), **could not verify** (an unproved implication), and **my judgement** (scope or novelty). Numbered findings alone enter the final count. An admitted uncertainty is not counted merely for remaining unresolved.

The central distinction is important: D8's single-slot safety argument works; its advertised consequences are much stronger. Result 1 has a sound elementary core. Result 2 has a repairable numerical example but does not establish the advertised conclusion for an unspecified causal definition. The decision set is repairable, but the paper has not demonstrated that its present choices fit together.

## 1. HIGH — Single-slot uniqueness does not establish prefix consistency, complete evidence, or final verdicts

**Text attacked:** “One certified log per unit. Verdicts no longer depend on evidence in transit from other units”. Also: “every honest witness holds that proof within 2Δ.”

**Status: shown wrong for the unconditional evidence bound; could not verify the stronger log and finality implications.**

First, the timing proof the author actually gives is sound. Let the earliest honest receipt of any controller-signed head for slot k occur at t. Every honest witness receives that head by t+Δ. Every honest witness considering a different head first received at s≥t waits strictly longer than Δ, and therefore sees the first head before signing. Faulty witnesses cannot supply f+1 signatures alone. Simultaneous first receipts cause both heads to be rejected, which is safe. A controller sending to just one honest witness still triggers dissemination. A head disclosed only to faulty witnesses cannot be certified without eventually crossing to an honest witness. This requires authenticated forwarding, retention of conflicts, and timers whose **real elapsed** wait exceeds the delivery bound; a clock-offset bound alone is not a clock-rate bound.

The proof-distribution claim nevertheless fails. With Δ=1, give A to honest H1 at t=0 and keep signed B exclusively with a faulty witness until t=100. No honest witness can possess the two-head proof at t=2. Once B reaches any honest witness at 100, all honest witnesses can have the proof by 101. The relevant origin is disclosure of the second head, not creation or disclosure of the first.

More substantially, uniqueness at each sequence number is not consistency across sequence numbers. As the signing rule is written, certify head A at slot 10 at t=0; at t=10 propose head B at slot 11 whose predecessor is an incompatible, uncertified slot-10 head. No different slot-11 head exists, so the stated echo predicate permits signing B. An honest **prefix-validation** rule would reject this, but the echo predicate is not that rule. Replaying whichever log is presented is insufficient unless replay also checks extension of the previously accepted prefix. This is a missing protocol invariant, not a counterexample to the single-slot lemma.

Even granting prefix validation, a certified prefix does not prove that it closes all evidence relevant to a verdict. At day 9, adjudicator J has a sender's summary through day 8 and no closed receiver summary. A receipt recorded at the receiver on day 8.9 can be delayed to J until day 20. Within each unit all witnesses are timely and honest. A single certified log has not bounded that inter-unit delay. D9 can avoid error by waiting for all designated, deadline-covering summaries; it then gives no bound on when its trigger occurs. That is a legitimate repair, but not the stated elimination of evidence-in-transit concerns.

**Evidence:** The executions above are direct deductions from D8/D9. [Dolev–Strong](https://doi.org/10.1137/0212045) solves authenticated Byzantine agreement with agreement/termination requirements beyond this certificate-safety lemma; its round bound is not a refutation of D8. [RFC 9162, §§2.1.3–2.1.4](https://www.rfc-editor.org/rfc/rfc9162.html) treats inclusion and consistency as separate proof obligations. [PeerReview](https://haeberlen.cis.upenn.edu/papers/peerreview-tr.pdf) provides eventual detection using secure histories and auditing, not the paper's unconditional finality conclusion.

**Exact repair:** State only the single-slot lemma here. Add mandatory certified-prefix extension, durable witness state, entry availability, and a rule binding receipts to that prefix. Define a closed summary by commitment, deadline, rule version, and relevant log frontier. Trigger D9 only after every required closed summary is present. State explicitly that obtaining those summaries can take unbounded time. Measure equivocation-proof dissemination from first honest disclosure of both conflicting heads.

## 2. HIGH — The satisfiability predicate does not match the individual contract semantics

**Text attacked:** “one way of acting that keeps every guarantee whenever all the assumptions hold.”

**Status: shown wrong under the literal predicate.**

At t=0 select these three norms, with an environment bit b revealed at t=1:

- A: if b=0, spend 2 by t=2.
- B: if b=1, send a free report by t=2.
- C: unconditionally, total spending is at most 1.

The conjunction of all assumptions is false in every history. Thus any strategy passes the quoted test. At t=1 set b=0. A and C are both owed under the paper's individual-assumption definition, and no strategy can satisfy them. The test has accepted an inconsistent effective set. This does not depend on computation being expensive, the uncertainty already acknowledged by the author.

**Evidence:** Direct propositional counterexample. The relevant distinction is between `(∧ A_i) ⇒ (∧ G_i)` and `∧(A_i ⇒ G_i)`. [Abadi–Lamport, §5](https://lamport.azurewebsites.net/pubs/abadi-conjoining.pdf) supplies assume-guarantee composition with explicit premises, not permission to substitute one formula for the other.

**Exact repair:** Use `exists one causal strategy σ, for every admitted environment history e, for every i: A_i(σ,e) implies G_i(σ,e)`. State which observations σ can use and when. Require assumptions to refer to specified environment events; do not let the unit make its own excuse true. Run selection against that predicate. If the author intended this predicate, replace the sentence rather than retaining the weaker wording.

## 3. HIGH — Raising a cap does not reserve the override's resources

**Text attacked:** “It is then carried out as a recorded raising of the cap through the ledger”; “A second commitment that would exceed the cap is refused”.

**Status: shown wrong as a sufficient implementation of the override.**

At t=0 the cap is 100, 100 is already spent, and a higher authority issues an unconditional obligation to make purchase P costing 20 by t=3. There is no vendor failure. At t=1 the controller records cap 120 as D6 prescribes. At t=1.1 it accepts an unrelated commitment Q for 20, reserving the newly available rights. Q does not exceed the new cap, so D6's stated refusal rule does not reject it. At t=2 P cannot reserve rights. At t=3 either P is missed or the controller must raise the cap again or frustrate Q. Merely implementing an override as a cap raise does not implement the winning obligation.

There is a related semantic fork. Raising a cap to 120 is a general permission for total expenditure up to 120. Overriding a lower cap specifically to perform P need not be that general permission. The paper must choose, rather than silently equate them. If the raised cap is freely usable, selection and commitment admission must still preserve the outstanding higher obligation.

**Evidence:** Direct ledger execution. [Balegas et al.](https://arxiv.org/abs/1503.09052) supports the reuse of escrow rights for numerical invariants; numerical conservation alone does not establish fulfilment of the purpose for which rights were issued.

**Exact repair:** Atomically install the selected override and reserve its required rights for P, with an explicit rule for releasing them. Define whether the cap increase is general or obligation-specific, who authorizes it, and which version later reductions supersede. Check new commitments against both reservations and the remaining winning strategy, not just the numerical cap.

## 4. HIGH — Commitments outside selection still need admission against existing norms

**Text attacked:** “An obligation from a unit's own commitment does not enter the selection. A later norm that conflicts with an outstanding commitment is a rule change.”

**Status: could not verify the claimed consistency; an explicit uncovered admission case exists.**

At t=0, an unconditional norm requires action A during [1,2]. The unit has one machine slot during that interval. At t=0.5 it accepts a commitment to perform incompatible action B in the same slot. Both actions cost zero, so D6's reservation test passes. No later norm is issued, so D5's grandfather-or-frustrate procedure never runs. The norm set itself remains satisfiable; the obligations actually owed do not. The same issue occurs when two zero-cost commitments contend for that slot.

This is not an argument for putting commitments into priority selection. It is an argument that excluding them does not remove their feasibility constraints. If admission already checks these constraints, that premise is essential to the claim that all decisions fit.

**Evidence:** Direct scheduling counterexample to the sufficiency of the listed choices; no literature premise is needed.

**Exact repair:** At commitment admission, check the proposed commitment jointly with all outstanding commitments and selected conditional guarantees, using one causal strategy. Reserve scarce non-money resources where needed. Specify refusal or an explicit, already-authorized frustration procedure. Later norm changes must use the same combined obligation state.

## 5. HIGH — The “weaker reading” does not collapse minimal sufficient causes to local breaches

**Text attacked:** “under a weaker reading ... the verdict depends only on who broke a guarantee, and it composes.”

**Status: shown wrong as a general implication, even though the paper flags this area as uncertain.**

Consider a protected output bit Y due at t=2; failure means Y=1. At t=1 two parties A and B each send forbidden bit 1; their local guarantees require bit 0. In system AND, Y=A∧B; in system OR, Y=A∨B. Both parties break precisely the same guarantees at precisely the same times, and the actual output is identical. If a party that is not in the candidate deviating set follows its bit-0 protocol, the minimal sufficient sets are `{A,B}` for AND and `{A}`, `{B}` for OR. “Could cause the failure by some deviation” still requires the system's composition semantics. A set of local breach labels alone does not determine it.

This example does not purport to refute composition when the whole public circuit is supplied to the adjudicator. It refutes the claimed **reason** for composition. The public protocol, admissible deviations, environment restrictions, and counterfactual relation remain necessary inputs.

**Evidence:** [Künnemann–Esiyok–Backes, Definition 1](https://arxiv.org/pdf/1805.10891) defines an a posteriori verdict relative to a protocol, property, trace and counterfactual relation, using minimal sets permitting violating counterfactual traces. It is not simply a list of local contract breaches. [Halpern's modified definition](https://www.ijcai.org/Proceedings/15/Papers/427.pdf) likewise makes causal claims relative to a specified model and intervention semantics.

**Exact repair:** Delete the claimed automatic collapse. Fix either KEB's relation or a particular structural model/intervention language; then prove that the proposed summary is sufficient relative to those fixed public inputs. Label enumeration of broken conditional guarantees as a different verdict, not as an established implementation of minimal sufficient causal sets.

## 6. HIGH — R3's conjunction observation does not discharge the original R3 target

**Text attacked:** “R3 ... reduces to a conjunction except for one question about exporting context between levels”.

**Status: shown incomplete as a reduction of the plan's stated target; my judgement is that the conclusion is overstated.**

For a fixed concrete trace and correctly translated effective predicates, satisfying every predicate is indeed satisfying their conjunction. I found no error in that tautology. But the plan's R3 also asks for sound **local edge checks using a bounded description of selection**, and a bounded decision procedure for minimal conflict sets. These are not removed by choosing a canonical path or caching a global effective set.

For example, at t=0 authority A imposes a job requiring the only machine slot [1,2], while authority B imposes a different job using that slot. Each edge's independently chosen implementation can pass its own realizability check; there is no common implementation. A global selection can repair this, but then an edge needs the correct selection result and its version, and one must establish the stipulated information bound. This is a different premise from assuming the checks already test one fixed trace against exactly the surviving predicates. Finding every minimal infeasible subset also remains a separate computational problem even if checking one candidate set is decidable.

This finding does **not** count the paper's admitted uncertainty about joint satisfiability a second time. It concerns two explicit deliverables of R3 omitted from the claimed reduction.

**Evidence:** [Integrated foundation plan, R3](/root/mic/reports/Integrated%20foundation%20plan.md:137) states both requirements. [David et al.](https://homes.cs.aau.dk/~adavid/publications/41-hscc10.pdf) distinguishes consistency, conjunction, composition and refinement; it does not identify all these tasks with tracewise conjunction.

**Exact repair:** Retain the tautological observation as a lemma. List bounded selection export, sound translated edge checking, and minimal-conflict detection with a specified grammar and complexity bound as unfinished R3 obligations. Only then assess what remains beyond the baseline.

## 7. MEDIUM — Result 2's numbers do not establish the claimed changed verdict

**Text attacked:** “U2 hands over on day 8.5, in time”; “Both: either one complying would have saved the delivery”.

**Status: shown wrong for a permitted execution; the information-loss idea is repairable.**

Use the allowed one-day U2→U1 delivery time. In History 1, making U3 timely yields U2 handover 8.5 and U1 receipt 9.5: still late. Making U2 satisfy its three-day guarantee after receipt 5.8 permits handover 8.8 and U1 receipt 9.8: also still late. “Complying” does not specify an earlier handover, and the day-8 guarantee's upstream assumption is false in this intervention. The author conflates handover with receipt and one chosen compliant policy with all compliant policies.

The boundary summaries can indeed be identical as written. What fails is the calculation of the verdict, not observational identity.

**Exact repair:** For a concrete model, fix U2→U1 delay at 0.1 day and define the compliant policy to take exactly 3 days. Actual receipt is 9.4. Timely U3 in History 1 gives receipt 8.6, and compliant U2 gives receipt 8.9. In History 2, changing only U3 leaves receipt 9.4. Define variables for upstream lateness and downstream compliance, plus a hidden controller-policy parameter. Under the elementary but-for tests these give the desired difference; in a simple acyclic structural model they can support an HP example. State the chosen HP variant and admissible variable ranges before claiming its complete verdict.

Make the hidden policy a varying private parameter in **one specified model**, or explicitly say the example compares different models. If controller code is known public input to the verdict, different code is distinguishable without inner logs, so the example is not an impossibility proof against that setting. A valid narrow conclusion is that boundary timestamps alone do not identify counterfactual response functions across unspecified controllers.

**Evidence:** Arithmetic above. [Halpern–Pearl](https://arxiv.org/abs/1301.2275) defines actual causation through structural equations and interventions; [Halpern on appropriate causal models](https://www.cs.cornell.edu/info/people/halpern/papers/causalmodeling.pdf) explains why causal conclusions depend on modelling choices. Neither licenses the unspecified phrase “the correct verdict”.

## 8. MEDIUM — Result 1 finds a witness explanation, not an exclusive or complete blame partition

**Text attacked:** “exactly one of these is found”; “Each step moves to an earlier deadline, so the search ends.”

**Status: shown wrong as an exclusive classification; the finite-chain search itself is sound.**

Let all orders be timely. U3 hands over at day 4.5, violating its day-4 guarantee; goods arrive at U2 at 5.5. U2 hands over at 8, meeting both its relative three-day guarantee and the numerical day-8 deadline. The next link delivers at 9.5. The chain property fails, and the algorithm stops at allocated link loss. U3 nevertheless deviated. Thus two of the purported categories occur, and the algorithm names nobody despite a deviation. This does not refute the narrower statement that the algorithm returns **one** valid explanation, or that all actual guarantee breaches can be enumerated separately from complete summaries.

On a fixed three-unit chain, recursion ends because the dependency graph is finite. Strictly decreasing real deadlines alone are not a general well-foundedness argument: deadlines 1, 1/2, 1/4, ... form an infinite descending dependency chain. For finite commitment graphs this is harmless; for an unbounded extension it requires a finite-ancestry or rank assumption.

“No compliant unit is named” is correct for the **defined local breach predicate**, if its own witnesses faithfully certify complete, deadline-covering facts and authentic input receipts. It does not follow from ownership of the log alone: a log can contain false assertions supplied by other parties. Conversely, a faulty neighbour's denial of delivery cannot prove that a sender did not send. The paper is right to distinguish that loss allocation from proof of sender deviation.

**Evidence:** Direct timed execution and elementary order-theoretic example. The reuse comparison is appropriately to [Abadi–Lamport](https://lamport.azurewebsites.net/pubs/abadi-conjoining.pdf), with its hypotheses, not to a general causal-accountability theorem.

**Exact repair:** Say “returns one explanation by this specified priority of tests; explanations need not be exclusive.” Separately state whether the verdict must enumerate all broken guarantees. Make the chain property conditional on the timely initial orders, close the summaries as in finding 1, and require a finite dependency DAG or an explicit well-founded rank. Scope innocence to the named unit's witness bound and authenticated facts.

## 9. MEDIUM — D10's timestamp tolerance is not exact deadline evidence

**Text attacked:** “That certificate discharges the obligation”; “A controller that then declares a breach has deviated, provably.”

**Status: shown wrong if discharge means actual receipt before a sharp deadline; otherwise the rule changes the meaning of timeliness.**

Take Δ=1, zero clock skew, f=1 and a deadline of day 9. All honest receiver witnesses first see the goods at 9.4. A proposed timestamp τ=8.5 is within Δ of their first receipt, so an honest witness following the stated acceptance predicate can sign it; one faulty signature completes the certificate. It says timely although no receiver witness received the goods by 9. The opposite shift can make an early receipt look late. This is a bounded error, exactly as D10 acknowledges, but the later unconditional discharge and blame statements fail to account for the uncertainty at the deadline.

**Evidence:** Direct use of D10's stated signing condition. No external clock theorem is needed.

**Exact repair:** Choose explicitly between contractual time defined by the certificate and real-time performance. For the latter, certify a conservative interval and use three outcomes: certainly timely, certainly late, and boundary uncertainty allocated by a declared rule. Do not call a controller's accurate statement about physical lateness a proved deviation merely because a tolerated timestamp lies before the deadline. Also separate “sent by” obligations from “received by” obligations; a receipt certificate does not discharge every possible guarantee.

## 10. MEDIUM — D11's basic conservation works, but a cap raise need not free its increment

**Text attacked:** “A raised cap frees the difference at once.”

**Status: shown wrong without a precondition; the four-category arithmetic and example bounds are sound under their stated disjointness.**

At t=0 cap=100 and outstanding rights=100. At t=1 lower the cap to 50. At t=2 raise it to 60 before settlement. The four-category total remains 100; available issuance remains zero. No part of the increment 10 is freed. This is even the paper's third table row. Available issuance is `max(0, cap − total)` and the gain is the difference between those positive parts, not the change in cap.

There is no counterexample to the main conservation argument if each resource unit occupies exactly one category, conversions live→committed→spent preserve totals, and final settlement releases only demonstrably unused amounts. With spent=10 and late rights=90, total=100; counting later spending against that 90 requires replacing that portion, not adding it again. Expiry is not release. These are the correct escrow instincts.

The “largest cap” formulation needs provenance through conversion: if a right issued under cap 100 becomes spent, its issuance version must remain represented by “still-counted”. If that phrase means only currently exercisable rights, spend 100, expire all leases, then lower the cap to 50: the claimed maximum is empty despite spent=100. This is a clarification, not a separate defect. The claimed bound follows with provenance: if all rights from the previous maximal-cap version disappear, remaining lower-version mass was either already bounded when that version was issued, or had to fit under a later lower-cap issue check.

**Evidence:** Arithmetic and invariant reasoning above. [Balegas et al.](https://arxiv.org/abs/1503.09052) is a relevant escrow-derived numeric-invariant baseline, but does not by itself prove this paper's changing-cap and commitment semantics.

**Exact repair:** Replace the cap-raise sentence with the positive-part formula. Give the conversion/settlement transitions and retain issuance provenance for spent and committed mass. Say “authenticated and verified final balance”: authentication identifies a reporter and does not alone make a balance true. State whether this ledger lemma assumes correct controllers, as the plan's prevention results do. R4's numeric part is plausibly an application lemma; explicit grandfathered commitments and their size bound still need their own argument.

## 11. MEDIUM — Canonical-path changes are effective norm changes even without a new issuance

**Text attacked:** “Conflicts are found and resolved once, when a norm takes effect”; D4's “direct edge ... if there is one”.

**Status: shown gap in the event boundary; not a refutation of freezing obligations against ordinary delay.**

At t=0 a norm reaches leaf L only through intermediate M and its translation requires action a by t=5. Another surviving norm prohibits action b. At t=1 a direct origin→L edge is declared; its translation of the same origin norm requires b. No new origin norm has been issued. D4 now makes the direct path canonical. If the old cached selection survives, L has incompatible obligations; if the new canonical norm is silently substituted, the prior obligation may disappear without D5's loss handling. Priority-table changes and authority revocation raise the same version-boundary issue.

**Evidence:** Direct execution using the two-path case expressly allowed in D4. This does not depend on a literature claim.

**Exact repair:** Treat any change to path, translation, authority membership, priority version, or relevant declared context as an effective norm-version event. Re-run selection and commitment migration at that event. Ordinary passage of time and a unit's self-induced loss of options must not erase obligations. With that distinction, “evaluate only when norms take effect” is a coherent policy.

## Checks that did not produce an additional defect

### D3 and the static part of D4

D3's level rule prevents authority cycles, assuming authenticated, immutable levels. A finite set of conflicting norms has a total priority once the versioned table and identifier comparison are fixed. Equal-level authorities cannot govern one another, and an old unit cannot newly acquire an authority at an equal or greater level; these are explicit expressive restrictions, not counterexamples to the hierarchy the author chose. I found no honest concrete failure of the static rule. A declarer must verify endpoint level certificates rather than trust self-reported levels.

D4's canonical path resolves the stated static diamond ambiguity. It does not prove translation soundness or bounded context export, but the author admits that. The new issue I count is reconfiguration (finding 11), not the admitted translation problem.

### D5 and invalid-authority losses

Prospective removal of an invalid norm and attribution to its issuing authority is a coherent policy choice. To call it proved deviation under Result 1's own-log rule requires the issuer's signed norm, applicable charter version, and evidence of breach; a downstream adverse finding alone is insufficient. I did not count this as another finding because the fixed plan already supplies replay and charter ownership. D5 also cannot promise that an allocated loss will actually be paid; the paper does not clearly make that stronger promise.

### D12: finite-pool safety transfer is conditionally correct

The precise prefix argument is valid: if every finite prefix of every admitted limit execution embeds in some sufficiently large pool, and all those pool prefixes satisfy a prefix-closed property, the limit execution satisfies that safety property. No new dynamic-creation theorem is needed **for that statement**. D9's bounded response can be safety: a prefix extending beyond the deadline without a verdict witnesses violation.

The provisos matter. At t=0 trigger a verdict due at t=2. Create units at 1−2^−k for k=1,2,... . An admitted execution with infinitely many creations before time 1 followed by passage beyond time 2 would have a bounded-time prefix not represented in any finite pool. Ordinary locally finite, time-divergent execution semantics exclude this; a completed supertask semantics would not. Also, a pool of size n cannot accept creation request n+1. The limit proof must select a pool large enough for the whole prefix, not pretend that every finite pool meets an unrestricted creation contract.

These are conditions to state, not an established counterexample to the proposed limit. The author explicitly identifies embedding agreement and theorem compatibility as checks still required, so I do not count their absence as a finding. [David et al., Definition 4](https://homes.cs.aau.dk/~adavid/publications/41-hscc10.pdf) supplies a timed specification framework with explicit implementation progress conditions; I could not verify from this sketch an instantiation for its dormant units. Claiming that this conditional safety observation also supplies all closure/substitutivity results would be premature. The plan's final decision rule also still says to report pool results as bounded-population results; revise that rule explicitly if adopting the stronger limit interpretation.

### Result 3: conditional oracle construction, not yet an identified new theorem

The caveat about human and learned-agent counterfactual inputs is correctly acknowledged and is not counted as a finding. I cannot certify novelty, and I do not see an unconditional impossibility under a sufficiently restricted reading.

For a finite acyclic chain of deterministic, terminating local transducers, with certified code/state, fixed exogenous inputs, and a specified intervention policy, an adjudicator can evaluate a counterfactual by querying transducers in causal order and propagating the resulting **changed** messages. It can then enumerate intervention sets and minimize the successful sets. Witness signatures attest those local computations under the fault bound. This is a straightforward distributed evaluation construction; its limitations include unbounded answers or computation, and it says nothing about privacy merely because no file called an inner log is transferred.

The interface must distinguish an intervention from an endogenous consequence. If U3's earlier shipment causes U2 to send an earlier acknowledgement which changes U3's next shipment, keeping the actual acknowledgement as a “recorded input unchanged” is a different intervention from replaying the connected system. A chain of orders and deliveries is not necessarily acyclic as an event dependency graph. Certification also must bind the controller policy and pre-intervention state; a reference implementation alone need not determine the behavior of the actual deviating controller. These are specifications for the conjecture, not discoveries contradicting its admitted replay uncertainty.

[Halpern–Pearl](https://arxiv.org/abs/1301.2275) already supplies intervention-based structural counterfactual evaluation. [Caltais–Leue–Mousavi](https://arxiv.org/abs/1608.07880) proves compositional causal results for non-communicating transition systems; it is relevant prior art, not a theorem covering this communicating witness protocol. [PeerReview](https://haeberlen.cis.upenn.edu/papers/peerreview-tr.pdf) supplies the deterministic auditing baseline, not the claimed counterfactual semantics. I found no verified source proving this exact local-query conjecture. My judgement: its restricted version is likely routine; a result with communicating controllers, specified causal semantics, malicious participants, and bounded query disclosure might be substantive. Do not announce either novelty or impossibility before fixing those parameters.

## Conclusions for the plan and verdicts

**Result 1:** Correct as a finite-chain, one-explanation search over authentic closed summaries, with explicitly conditional initial orders and a declared link-loss rule. The literal exclusive classification is false, and it is not a computation of all causal culprits. Its no-false-local-blame property follows under the blamed unit's own evidence assumptions. This is appropriately presented as reuse, once D8's missing evidence premises are added.

**Result 2:** The published numerical counterexample does not establish its claimed verdicts. A repaired, explicitly modelled example supports the narrower information-theoretic point that boundary observations alone do not determine an unknown response function. The categorical statement about the plan's minimally sufficient verdict is not established without a fixed causal relation and public-model inputs. The weaker-reading caveat is positively wrong as a general reduction to local breaches.

**Result 3:** Ill-posed at present rather than shown false. A finite deterministic acyclic version has an elementary oracle-evaluation construction. The exact conjecture's novelty and its applicability to communicating, deviating controllers could not be verified. The author's acknowledged uncertainty is warranted.

**Decision set:** No global impossibility of a consistent repaired design was found. The claimed demonstration that the current thirteen choices fit together is premature: the conditional realizability predicate, commitment admission, override reservations, log closure, and effective-version events need repair. These are substantive changes, but they do not force abandonment of the three-unit architecture or the restricted Result 1.

**R3/R4 and novelty:** R3's tracewise conjunction observation is correct but omits explicit deliverables of the plan. D11's numeric conservation is a plausible escrow application; calling all of R4 discharged ignores its grandfathering and size requirements. “Mostly an assembly of known results” is a reasonable judgement about ingredients, not a verified theorem about their compatibility or a literature-established novelty verdict. The most potentially valuable contribution is precisely the composition contract between those ingredients; the current paper has not yet proved it. Nothing reviewed establishes that the candidate contains a new foundational result, and nothing establishes that its remaining composition questions are worthless.

CRITICAL: 0  HIGH: 6  MEDIUM: 5  LOW: 0

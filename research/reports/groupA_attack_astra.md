# Adversarial review: Step 0 group A norms

I read the group A sketch first and formed the counterexamples below before checking the corrected first pass, both earlier attacks, the integrated plan and group B. Only this review file was written. Numbered findings alone enter the severity count. “Shown wrong” means an explicit execution or calculation; “could not verify” means a necessary implication is unspecified; “my judgement” identifies an assessment rather than a theorem. An acknowledged cost or open question is not itself counted as a defect.

Notation: P(X) permits effects in X; F(X) prohibits them; O(X,d) requires one such effect by deadline d; C(r,k) caps resource r at k. Priorities decrease from left to right. Where an example concerns something other than default denial, its permissions are explicitly processed before its obligations. All numerical needs in the examples are honest.

## 1. CRITICAL — The feasibility predicate does not establish temporal realizability

**Text attacked:** “It is sound: a set that passes can always be honoured, by emitting each obligation's allowed effect from its own reservation.”

**Status: shown wrong for timed resources; could not verify the intended trigger multiplicity.**

**Small case.** One controller has a serial output device. Emitting A or B occupies it for one time unit; these are local actions, with no cooperation assumption. At time 0 the norms are P({A,B}), O({A},1), O({B},1), C(device-time,2). Each obligation declares maximum device-time 1. Both patterns are permitted; neither is prohibited; 1+1≤2. The test passes. Only one action can finish by time 1. Reservations for aggregate device-time do not reserve a feasible schedule.

Representing time slots as resources can fix this instance, but that is an additional mandatory encoding condition. Case 11 says a slot *can* be declared; nothing requires all action durations, release times, deadlines and compatibility constraints to be represented by independently reservable slots. The proof silently assumes precisely that condition.

**Repeated-trigger case.** Permit reply(i). A standing rule says “after a certified request i, reply(i) within one day,” maximum one token per reply, cap one token. It passes at installation. Two distinct requests arrive before either is answered. They require different effects, so one reply cannot discharge both. The installation reservation covers one instance, not two. If “maximum need” instead means lifetime need over every trigger, an unbounded standing rule needs an infinite declaration and cannot be admitted under any finite cap. If named events are single-use identifiers, say so: that would exclude this counterexample but also excludes ordinary standing response duties until a separate admission mechanism is supplied.

**Other interference.** Aggregate vectors suffice for independent consumables, including several resources, only if each obligation has an executable witness within its vector and executing it cannot destroy another witness. Two operations changing the same one-shot document from unsigned to incompatible final states need an exclusive document-state resource or a transition model. Merely giving each a budget of zero does not make them jointly executable. This is not an argument that independent vector reservations are intrinsically unsound; it identifies the missing semantic restriction.

**Evidence:** direct scheduling calculation and two-event history. The grammar mentions deadlines and certified events; the test consults neither a schedule nor event multiplicity.

**Exact repair:** restrict the theorem to finite, single-instance obligations with positive feasible deadlines, atomic effects, validated cost bounds, complete independent resource accounting and no hidden state interference. For timed actions, require a certified reservation schedule satisfying release times and deadlines; for recurrent obligations, specify a bounded trigger envelope and reserve against it, or explicitly refuse triggers before a duty is created. State which environment assumptions this guarantee is conditional on. Prove that each execution preserves the reserved witnesses of the remaining instances. The unrestricted soundness sentence must go.

## 2. CRITICAL — Adding a cap can invalidate the completed selection; R4 is false

**Text attacked:** “a permission or cap is added”; “If it fails only because of caps whose origins it outranks, those caps are raised”; “At every change of version the feasibility test holds for the new effective set together with everything reserved.”

**Status: shown wrong.**

**Small case.** Use priorities P({buy}), C(money,100), O({buy},1) needing 20, C(money,10). The two cap issuers are authorities over the owner. Spending is initially zero. The obligation passes against 100 and reserves 20. The last cap is added without a test. The final lowest cap is 10; the outstanding obligation needs 20. Thus the final effective set fails the document's own feasibility predicate, even if existing reservations are counted only once. Emission must either break the effective cap or miss the obligation. The obligation outranks the offending cap, so the stated priority policy also demands the opposite outcome.

The cap-raising branch is unreachable for lower-priority caps in a fresh priority scan: every cap already encountered has at least as high an origin priority as the obligation, in this example with direct routes and no ties. Case 9 cannot execute as described. Preloading all caps would make that branch reachable, but that is a different algorithm, and it needs to distinguish pending caps from accepted ones.

An independent R4 contradiction needs no obligation: 80 is already spent under cap 100; a new cap 50 is added. The old issuance invariant can remain true, but “the feasibility test holds” cannot, because its left side includes 80. Keeping old spending grandfathered is sensible and is already contemplated in the first pass; it does not make 80≤50 true.

**Evidence:** literal simulation of the three selection clauses and arithmetic. This is a definite failure beyond the author's admitted uncertainty about later obligations interacting with a cap raise.

**Exact repair:** enforce an invariant after every restrictive insertion, including caps. Either process a complete versioned cap environment before obligations, with explicit override authority, or adjust/except each newly encountered cap against obligations already selected. Distinguish non-overridable physical capacity from an administratively raisable spending ceiling. Separate R4's new-reservation test from grandfathered old exposure, and specify an atomic version transition preserving the ledger invariant. R4 cannot claim the present feasibility predicate for arbitrary reductions below spent amounts.

## 3. CRITICAL — R3 mistakes authorized effects for fulfillment of obligations

**Text attacked:** “If a unit's controller emits only effects that have a derivation from its effective set, the unit's behaviour conforms to every norm in that set”.

**Status: shown wrong.**

**Small history.** P({report}) precedes O({report},day 8); no resource is needed. The obligation is selected. The controller emits nothing through day 9; its assumption remains intact. Every effect it emitted has a derivation, vacuously. It does not conform to the obligation. Case 1 and case 13 explicitly require this failure to be classified as a finding.

**Evidence:** the empty output trace satisfies R3's premise and violates its conclusion. The subsequent sentence “An obligation is not enforced; a miss is recorded and classified” concedes the counterexample rather than repairing the statement. The plan's D7 already warns about exactly this distinction.

**Exact repair:** split R3 into safety of emitted effects and detection/classification of outstanding missed duties under the group B evidence assumptions. If actual fulfillment is wanted, add a live controller/scheduler hypothesis that guarantees it. Do not call a logged violation conformance. Retain assumptions about complete evidence, certified time and witness faults in the detection claim.

## 4. HIGH — A total order on origins is not yet a determined selection

**Text attacked:** “takes the lowest priority among its origin and the intermediate authorities”; “Because the order is total and each step is determined, the result is unique.”

**Status: could not verify uniqueness; explicit missing choices.**

**Route tie.** Let ranks be S>T>M>U. S and T each route an obligation through M to U. Both obtain M's priority. Their sequence numbers are local to different origins. Give U a high permission for A and B and a high cap of one token; each routed obligation needs one token. Processing S's norm first selects A; processing T's first selects B. The origin ordering and the “later first” rule do not specify how two distinct origins demoted to the same effective priority are ordered. Keeping original-origin priority as a secondary key is a reasonable repair, but is not stated.

**Choice of exception.** P({a,b}), O({a,b},1), F({a,b}). Neither a nor b individually is an effect the obligation “requires and has no other way to perform”: each has an alternative before the prohibition, and both are blocked afterwards. Does the procedure except a, b, both, or neither? The first two are different minimal repairs, the third is a broader repair, and the fourth makes the set infeasible. This is a concrete witness to the acknowledged uncertainty, not a complaint merely that uncertainty exists.

**Evidence:** two legal extensions of the missing tie rule give different selected obligations; two equally small repairs of the same prohibition give different effective patterns. I do not claim that a fully specified greedy algorithm must be nondeterministic.

**Exact repair:** define a complete norm key, e.g. effective route priority followed by original-origin key and unique sequence number; specify authenticated non-reuse of sequence numbers and a finite input snapshot. Define exceptions using a deterministic executable witness selection, or an exact set-valued rule with a proof of feasibility. State the ordering of effect alternatives if one witness is chosen. Then prove uniqueness by induction. The current proof assumes its crucial premise.

## 5. HIGH — Permissions arriving after an obligation are never reconsidered

**Text attacked:** “some permission allows”; “an obligation is added if the test still passes”; “Otherwise the obligation is left out.”

**Status: shown wrong as a priority-respecting admission procedure.**

**Small case.** One authority issues O({report},8) at sequence 2 and P({report}) at sequence 1. Later-first order processes the obligation first. No permission is yet in the set, so the obligation is discarded. Permission is added next. The final set has no reporting duty, although the two norms together are trivially satisfiable. Reversing their sequence numbers admits the duty. Permissions are support for obligations, not just conflicting candidates; a one-pass prefix test loses that support.

For P_H({a}) followed by F_L({a}), no obligation exists to earn an exception, so the lower prohibition wins. This is coherent if permission means merely a necessary authorization condition and never a protected entitlement. It is not a general “higher norm wins” semantics. Conversely F_H({a}) followed by P_L({a}) still denies a, as expected under deny-overrides.

**Evidence:** direct execution. Cases 1, 4, 5, 9 and 13 silently need suitably early permissions; no universal ordering guarantee supplies them. Anchor duties in R5 are especially vulnerable if their permissions are lower or later in the scan.

**Exact repair:** define whether obligations themselves confer narrowly scoped authorization or evaluate their permission support against the complete permission environment. State explicitly whether permissions are weak authorizations or strong exceptions to prohibitions. If the former, qualify all priority claims accordingly; if the latter, process priority conflicts between permissions and prohibitions. Add same-origin issuance-order tests.

## 6. HIGH — Exceptions need obligation-instance scope, not just an effect pattern

**Text attacked:** “a prohibition is added, with an exception for any effect that an obligation already added requires”; case 4: “the prohibition stands for everything else”.

**Status: shown wrong under the stated pattern-level exception; could not verify a narrower derivation semantics.**

**Small history.** P({send-secret-to-X}) precedes a high obligation to send one such report by day 1, followed by a lower prohibition on sending that secret to X. The obligation has exactly one way to act, so the prohibition is excepted. The unit sends the report at t=0.5, discharging the obligation. There is no norm change and selection does not run. At t=0.6 it sends the secret to X again. The same permission and excepted prohibition still admit the effect. It is no longer needed by the obligation. A still-lower obligation to send another copy can likewise exploit a general pattern exception.

There is a separate order effect that is not itself a defect: P({a,b}), O({a,b},1), F(a), F(b) yields b under a natural sequential repair; swapping the prohibitions yields a. Once a complete order and exception semantics exist, different norm orders can legitimately express different policy. The defect here is uncontrolled reuse after the overriding reason has disappeared.

**Evidence:** the exception has no instance token, discharge condition or consumption rule in the four-form language. “What stays owed” ends a duty on discharge but does not revoke its pattern exception.

**Exact repair:** make every override a non-transferable authorization tied to a live obligation instance, its remaining quantity, purpose, beneficiary and deadline. Consume it atomically on discharge. Lower norms must not borrow it as general permission. Have the interpreter validate this link on each effect without re-running selection. State whether multiple obligations can share one performance and prevent accidental duplicate discharge.

## 7. HIGH — A harmless version change can erase a duty, reset it, or count its reservation twice

**Text attacked:** “An obligation in the effective set is owed until it is discharged or the set changes”; selection runs at “a norm issued or withdrawn”.

**Status: shown wrong under the literal reservation formula; could not verify preservation under a charitable deduplicated reading.**

**Small history.** High P({pay}), C(money,10), then O({pay},day 8) needing 6. It is selected and reserves 6. At day 7 the unit adds an unrelated own permission P({wave}), an accepted amendment. Re-running selection tests the old obligation's declared 6 “plus what is already ... reserved,” namely 6. Total 12 exceeds the high cap 10, which the obligation cannot raise. It is left out. The old effective set changed, so the old duty ceases to be owed. The unit manufactured an unrelated rule change to shed its debt-like duty.

If reservations are implicitly deduplicated, this arithmetic attack disappears, but the transition is still undefined: are the old instance, trigger, original deadline, partial performance and breach record retained? Recreating a “within eight days from when the norm takes effect” instance on each new version permits indefinite postponement. Keeping original issuance time prevents that attack and must be stated.

**Higher norm later.** An obligation is partly performed; a later, higher prohibition removes its last permitted completion. Dropping the old obligation may be an intended override. The present rule gives no migration record or bearer for its sunk cost and beneficiary loss, although the corrected first pass explicitly described this as a rule change with issuer loss. D5 supplies that rule only for commitments.

**Evidence:** the test's addition formula and the literal stopping condition for duties. The timing exploit identified in the plan has moved from passage of time to discretionary norm-version events.

**Exact repair:** maintain persistent obligation instances outside the replaceable selection snapshot. Recompute prospective applicability without resetting existing triggers, deadlines or findings. Charge each reservation exactly once by identity. Withdraw or frustrate an existing instance only through an explicit transition naming the defeating norm and loss bearer; irrelevant amendments must leave it unchanged. Norm invalidation needs a separately specified historical treatment.

## 8. HIGH — Commitment admission tests resource needs, not the promised performance

**Text attacked:** “A commitment is admitted only if the feasibility test still passes with its declared needs added.”

**Status: shown wrong.**

**Small case.** Current norms are P({a}), F({b}); no obligations or resource use. The unit promises to emit b tomorrow, with zero resource need. Adding zero needs leaves both clauses of the feasibility test true: there is no obligation whose pattern must be checked. The commitment is admitted. It cannot be honored. Because the prohibition predates the commitment, the “later norm” grandfather/frustrate rule never applies.

Two commitments for incompatible final states of one document produce the same problem unless admission includes the action model and scarce state as well as resource totals. Case 11 addresses a declared machine slot, but does not establish general admission soundness.

**Evidence:** commitments are explicitly “not a norm”; the rule adds their needs, not their required effects, deadlines or assumptions. The corrected first pass required checking commitments against selected norms and outstanding commitments together. This sketch narrows that repair and loses it.

**Exact repair:** give commitments a performance predicate and persistent instance identity. Test the proposed performance with the effective prohibitions, permissions, outstanding obligation instances, existing commitments and complete reservations. Keep commitments outside priority competition, but inside feasibility. Record refusal before a promise becomes binding or define who bears an unauthorized promise.

## 9. HIGH — Automatic frustration lacks conflict, liability and invalidation semantics

**Text attacked:** “If it does, each affected commitment is frustrated, and the loss falls on the authority that issued the norm”; “void from its issue for the purpose of verdicts and from the finding for effects”.

**Status: shown wrong if “affected” includes compatible regulation; could not verify the loss/invalidation transition.**

**Small cases.**

1. U owes delivery of x. Authority A issues a permission for x marked as applying to existing commitments. A permission can affect the commitment without making its performance impossible. Automatic frustration would cancel a performable promise. “Affected” must mean a specified conflict, not mere scope overlap.
2. A controls U and issues F(x), applying to existing commitments. U's obligation to deliver becomes a loss against A. If A has no reserved assets, a real performance obligation has become an uncollectible accounting entry. This demonstrates the author's stated abuse concern rather than merely repeating it. It does not prove a loophole if “loss allocation” is deliberately allowed to be unsecured; that weaker promise must be explicit.
3. The same F(x) is later found charter-invalid after the original delivery deadline. It was effective when U refused to deliver. Retroactively remove it for verdicts: is U now blamed for not delivering under a prohibition it then had to obey? Does A's frustration liability disappear, leaving the counterparty unpaid? Does the impossible past delivery duty revive? None is specified.

**Evidence:** the binary flag provides no transition among performance, frustration, compensation and retroactive legality. Final logs in group B can remain unchanged while verdicts are revised, so log finality alone is not a contradiction or a cure. Group B's time-based loss convention supplies no general valuation for a canceled non-time performance.

**Exact repair:** frustration requires a certified incompatibility finding, identifying the defeated performance. Define the compensation obligation, beneficiary, amount/rule, due date and security or explicitly unsecured status. Preserve accrued compensation and reliance protection when a norm is invalidated; alternatively specify a separate restitution transition with its bearer. Judge obedience to then-effective rules separately from issuer charter legality. Do not revive a missed past performance by retrospective erasure.

## 10. HIGH — R5's trusted interpreter argument does not cover its amendment and delegation semantics

**Text attacked:** “Every amendment a unit makes to its own rules is accepted”; “an amendment cannot express a breach of an entrenched rule”; “take it outside the norms of its authorities”.

**Status: could not verify the strong claim; shown wrong if acceptance means effective authorization of every amendment.**

**Amendment authorization.** Anchor permission allows requests to amend; an authority forbids amendment of field f; U requests exactly that amendment. With no higher obligation requiring it, selection provides no exception to the prohibition. If amendment is an effect, it cannot be accepted and performed. If “accepted” means stored as an inert proposal while prohibited changes are ignored, that is coherent, but “every amendment” is not completeness for useful self-amendment. The plan also requires a charter's amendment procedure, checked under the previous version. A charter requiring two signatures and a request with one signature expose the same conflict unless “amendment” already means an authorized request.

**No-action charter.** U prohibits every effect in its own rules. Exceptions preserve higher obligations only if those obligations were admitted with permissions and have a well-defined exception. Higher permissions alone lose to the new prohibition (finding 5). Thus the unit can block all optional useful action, including amendment requests if they are ordinary effects, without violating a safety policy. This is not itself a breach of R5's narrow safety promise; it defeats an inference from that promise to the plan's useful-amendment and useful-behavior requirements.

**Data, not vocabulary.** The anchor owns the term vendorClass and the rule “never pay a sanctioned vendor.” U cannot rename vendorClass, but if the interpreter reads a class membership table or effect attribute that U may amend, U changes X's membership and pays X. Fixed names and fixed evaluator code do not ensure trustworthy denotations. The proof needs authenticated, protected data provenance and a constraint relating emitted effects to real counterparties. This is a conditional attack: it fails if those inputs already are anchor-controlled, which the text does not say.

**Creation and delegation.** A forbids U from emitting pay(X), but permits create and send-message. U amends its workflow to create V and asks V to pay X. If creation does not automatically install U→V authority and inherit A's norms, V can pay. If it does, an arbitrary edge mapping can classify V's payment as a different effect. If authenticated inherited mappings preserve pay(X), the attack fails. Even then a prohibition on U's *own emitted effects* does not logically prohibit causing another unit to pay: that requires an explicit delegation-sensitive policy. All effects may be logged in this history; logging alone is no protection against it.

**What is right:** a unit cannot change an immutable anchor interpreter or rename an immutable anchor-owned symbol merely by submitting data in its own namespace. Replaying that fixed input interpreter is well defined. If effect mediation and witness operation are trusted mechanisms, local amendments cannot switch those mechanisms off. Ordinary high-priority obligations *to* log do not by themselves make logging inevitable: the idle/deviating controller counterexample of R3 still applies. Group B's autonomous witness slots help, under its explicit witness assumptions.

**Evidence:** the described amendment requests and delegation history; the proof omits trusted data, creation-edge installation, attribution and authorization premises. I have not proved that every implementation satisfying stronger mediation assumptions admits a logging bypass.

**Exact repair:** distinguish proposal acceptance, authorization under the previous charter, installation and effective normative force. Put protected data, effect attribution, atomic logging/dispatch, witness appointment and creation inheritance into the trusted transition semantics. Define which norms concern direct actions and which propagate to delegated actions. State R5 conditionally over those transitions, and separately prove preservation of the plan's resource ownership and log-continuity invariants. Do not label arbitrary inert submissions a complete class of useful amendments.

## 11. HIGH — Certified leased values are not a definition of valid context over time

**Text attacked:** “The mapping is declared ... by the authority at the upper end”; “exports what is needed as a certified value under a lease”; selection does not run “because the environment has changed”.

**Status: could not verify semantic conformance; explicit freshness fork.**

**Small history.** S→M→U translates “pay only approved suppliers” using M's approved set. At t=0 M certifies {X}, lease until t=10. At t=1 M removes X from the current approved set. At t=2 U pays X using the old certified value. If “approved” means M's current state, U violates the norm. If it means the leased snapshot, U conforms, but the policy grants a deliberate delayed revocation right. Either can be coherent. Certification proves who certified the old value, not which semantics applies after t=1.

If a renewed lease changes the mapping's extension from {X} to {Y} without changing mapping syntax, the sketch does not call that a selection event. An outstanding obligation to pay X may retain a now-blocked reserved witness; a prohibited/required classification can change while its recorded selection remains the same. Conversely expiration may block its only allowed action before its deadline without a named assumption or bearer for the intermediate's failure to renew.

The admitted cost “stale context refuses” covers expiration refusal, not the semantic status of changed but unexpired data or changed denotations on renewal.

**Route choice:** S rank 10, I rank 8, R rank 6, J rank 4, U rank 0; S has no direct edge to U and can route via I or J. An S obligation conflicting with R's prohibition wins via I and loses via J. This is an intended consequence of named-route priority, not a new defect by itself. A low intermediate can attenuate a superior norm; the paper openly accepts that cost. Capping priority prevents that intermediate's tightening from masquerading as S's *priority*, but does not certify that the translation preserves S's *meaning*.

**Evidence:** the two possible readings give opposite legality for the same payment. No mapping language, semantic preservation condition or data-update transition resolves them.

**Exact repair:** define norms over versioned snapshots or require revocation/update propagation with an explicit effective time. Make changes to relevant context a version event for applicability, while preserving already incurred duties as in finding 7. Bind leases to issuer, mapping version, input version and validity interval; classify nonrenewal and its loss bearer. Require a decidable restricted mapping language and either origin-approved translations or an explicit statement that translated meanings are independently authored lower-priority rules.

## 12. MEDIUM — The targets were narrowed without accounting for the plan's remaining deliverables

**Text attacked:** “R3, R4 and R5 are design rules with short arguments, not research results”; “all of the record for that unit ... is linear in the number of norms”.

**Status: shown omission; my judgement on significance.**

**Gap and evidence:** the [integrated plan](/root/mic/reports/Integrated foundation plan.md) requires R3 local translated checks and bounded minimal-conflict detection, R4 a frozen bound on grandfathered commitments plus useful change, and R5 single resource ownership, log continuity, outside witness appointment and a useful self-amendment class. Group A does not discharge these by supplying one global selected list. The earlier attacks explicitly identified the R3 omission; it remains.

A record cannot be bounded in bytes by the number of norms alone: one norm can enumerate N counterparties, and a route or exception certificate also has nonconstant size. “One record entry per norm” is a defensible count; “linear-size record” needs a syntax-size measure and a representation of exceptions. The paper also allows mappings without a computable grammar, so its blanket decidability result cannot cover every routing/translation question as stated.

**Exact repair:** rename the narrowed statements R3′–R5′ or explicitly amend the plan, list the remaining obligations, and give bounds in total encoded input and output size, including mappings, routes, leases and exception witnesses. Establish decidability only for a finite effectively represented language. Do not treat the plan's stop rules as satisfied by changed targets.

## 13. MEDIUM — The unary-pattern linear-time claim needs a representation qualification

**Text attacked:** “With prohibitions that each constrain one attribute it is linear.”

**Status: could not verify as stated; a tightening suffices.**

**Small case.** One attribute ranges over all strings. A prohibition's “set of allowed values” is presented as an arbitrary program deciding membership, or even a nonterminating program. The rule language does not specify a finite representation of those sets. Checking whether any permitted value remains need not be linear or even decidable in this representation. With finite explicit sets and suitable indexed operations, the intended union/subtraction check is straightforward; with symbolic predicates its cost belongs to the predicate language.

**Evidence:** unrestricted set descriptions and mappings are not an algorithmic input grammar. This is not a claim that the intended finite implementation has exponential unary checking cost.

**Exact repair:** specify finite domains or a decidable set algebra, account for its representation and operation costs, and state linearity only under the corresponding assumptions. Separate pattern nonemptiness from temporal feasibility, which has the additional issues in finding 1.

## Audit of all fourteen cases

This table records outcomes; it adds no findings to the count.

| Case | Does the stated procedure deliver it? | Qualification or objection to the required outcome |
|---|---|---|
| 1 | Yes, **if** the duty was admitted, its instance/deadline survives, and evidence is available. | It refutes R3 as stated. A missed duty is a finding, not conformance. |
| 2 | Yes under the named assumption and a certified refusal. | Not rerunning solely for environmental failure is an openly chosen cost. “Nobody has a finding” must mean nobody for this unit's excused guarantee; a vendor that broke its own delivery/acceptance contract can still have a finding. An irreversible or nonrenewable refusal assumption must be explicit; one transient refusal need not excuse all remaining opportunities. |
| 3 | Yes if a binding cap and permissions have been processed first, no cap override is allowed, and needs are not double-counted. | Without the priority of the cap this is underdetermined: obligations that outrank it are supposed to raise it. “Lowest-priority one left out” is not unconditional. |
| 4 | Yes for a singleton report pattern with prior permission and a live, scoped exception. | It does not generalize to alternatives or repeated use (findings 4 and 6). |
| 5 | Yes if the procurement prohibition blocks **all** reporting alternatives. | Blocking one format is not enough to remove a duty whose other formats remain available. |
| 6 | Only for the intended obligation-versus-prohibition example and correct mappings. | No forms are specified for N′ and R. A higher permission does not defeat a lower prohibition; two conflicting permissions need not be conflicting norms at all. The priority slogan is too broad. |
| 7 | Yes as priority attenuation, subject to the missing demoted-origin tie key. | Direct binding is an acknowledged design cost; this deliberately sacrifices indirect superior priority. |
| 8 | Yes for the bound on priority. | It establishes no fidelity of the stricter reading to the original meaning. An issuer can also change a direct edge's translation; priority capping only addresses intermediate translations. |
| 9 | No in the stated fresh scan. | A lower ministry cap arrives after the obligation; it is not raised by the obligation branch. Atomic reservation would solve the old stealing-resources attack only after the ordering bug is fixed. Raising financial authority does not create physical goods or machine slots. |
| 10 | Yes with the higher cap processed first and no other missing prerequisites. | An equal issuer is not outranked merely because its obligation has a later sequence number; clarify that intentional issuer-level choice. |
| 11 | Yes if slot capacity is represented, enforced and available to admission at that point. | This is a good instance of complete resource modeling. It does not show all deadlines and state conflicts are modeled. |
| 12 | Yes as an allocation entry if there is actual conflict. | Enforceable compensation, invalid norms and who pays for lost performance remain unspecified (finding 9). |
| 13 | Yes if the team's own obligation remains owed and officer approval is its internal responsibility. | If approval is expressly a named external assumption with a certified refusal, the no-finding rule instead applies. Fix the responsibility boundary; do not choose it after the miss. |
| 14 | Yes. | Strict decrease rules out all finite directed cycles, not merely opposite pairs, assuming immutable/version-consistent certified ranks. This part is sound. |

## What survives, what costs more, and what earlier work fixed

**Shown sound under stated finite/static premises:** strict ranks imply acyclicity: a cycle would require r₀>r₁>…>r₀. Different top-level units at equal rank cannot form authority edges even though their norms can be ordered by the anchor table; that is a restriction, not a contradiction. A created unit can govern its lower-ranked descendants and unrelated lower-ranked targets. It cannot govern its creator or a higher-ranked sibling. Ordered domains with enough lower elements (for example rationals) support arbitrarily long finite creation chains; an integer domain with a bottom needs a depth limit. A creation rule still must install useful initial edges; rank eligibility alone does not do so.

**The independent-resource core is right:** for finite one-shot atomic effects, exhaustive independent resource vectors, an actually executable allowed witness for each obligation, and no interfering state or timing constraints, disjoint reservations are a sufficient construction. Componentwise maxima across several resources are safe in this restricted class. They can be wasteful: if one obligation can use either (1,0) or (0,1), reserving (1,1) for it is safe but unnecessarily large.

**Benchmark cost, not an additional defect:** take n mutually exclusive procurement branches, each costing the full budget B. A realizable conditional strategy spends B in any run; the static sum demands nB and can admit only one branch. The over-reservation factor is n and has no constant bound as the number of branches grows. Recurring reporting obligations require either a bounded horizon/arrival rate or an unbounded lifetime reservation. Identical obligations dischargeable by one report are also overcounted if each gets a distinct reservation. These are concrete manifestations of the stated conservatism, not new severity items. They make the benchmark admission rate an essential measurement; I cannot determine it from this paper.

**Prior findings fixed or moved:** ranks are now independent of creation depth, fixing the particular benchmark-edge contradiction. Origin-named routes and lower translated priority resolve the earlier priority-laundering choice, with its stated loss of indirect state priority. Certified refusal and obligations to act address the earlier unobservable-delivery assumption. Triggering selection on route, mapping and authority changes fixes several omitted version events, but not context-value changes or persistence of existing obligations. Reserving an override atomically addresses the old race in intent, but the scan cannot perform the claimed raise. Commitment admission now mentions resources and slots, but drops the full performance test previously required. The old R3 omission remains, and its new statement introduces the direct omission counterexample. Group B strengthens evidence and provides a different, restricted R2′; it neither schedules group A's duties nor supplies its version/compensation semantics.

**No extra charge for the declared no-relief policy:** if a vendor refuses and the alternative is forbidden, retaining the prohibition while allocating a loss is the author's explicit choice. I do not count that as a bug. A refusal arranged by U or a colluding vendor still needs assumption provenance and a separate responsibility policy; automatic blanket innocence is not established by a signed refusal alone.

## Closest prior art and what, if anything, differs

These are sourced comparisons, not a comprehensive novelty search or a proof of equivalence. Each source supports the literature description adjacent to it; the assessment of this sketch is my judgement.

- **Prioritized normative reasoning is the closest selection baseline.** Liao, Oren, van der Torre and Villata define Greedy, Reduction and Optimization approaches for hierarchical abstract normative systems, relating them to argumentation and weakest/last-link principles. This is much closer to priority selection than ordinary access control. Their stated setting does not itself validate timed resource reservations or this exception procedure. The route-minimum rule resembles a weakest-link preference, but translating authority paths is not automatically the same object as an argument chain. [Prioritized Norms in Formal Argumentation](https://arxiv.org/abs/1709.08034).
- **Defeasible deontic logic already distinguishes kinds of permission and priority-based exceptions.** Governatori et al. explicitly represent permissive norms that override opposite obligations and strong permissions derogating from prohibitions. The sketch must choose its permission semantics instead of treating that choice as settled by an origin ranking. [Computing Strong and Weak Permissions in Defeasible Logic](https://arxiv.org/abs/1212.0079).
- **Constrained input/output logic is a close abstract consistency baseline.** Makinson and van der Torre's construction restricts generators to maximal subsets whose outputs satisfy constraints. Selecting an admissible normative subset is therefore established territory; deadline realizability and escrow state are extra obligations for this sketch, not consequences of that construction. I verified this description through the abstract and the later authors' discussion; the original PDF fetch was blocked, so I did not verify every original theorem. [Constraints for Input/Output Logics](https://citeseerx.ist.psu.edu/document?doi=5be4af7cf9399840688e5cda5067a7fd7440e3cd&repid=rep1&type=pdf), [later primary research discussion](https://orbilu.uni.lu/bitstream/10993/33477/1/ComplexityOfIOLogic.pdf).
- **XACML supplies explicit policy-combining semantics.** XACML 3.0 defines deny-overrides and first-applicable algorithms, as well as obligations attached to authorization decisions. Those obligations are not, just by that name, a scheduler for independently triggered future duties. The useful comparison is its precisely specified combining behavior; this sketch additionally attempts to select duties, reserve resources and transform lower caps. [OASIS XACML 3.0 specification](https://docs.oasis-open.org/xacml/3.0/xacml-3.0-core-spec-os-en.pdf).
- **Cedar is close to the permission/prohibition sublanguage.** Its documented semantics are default-deny and forbid-overrides-permit. That is coherent without a universal priority override. This sketch's duty-dependent exceptions, deadlines, cap mutation and persistent commitments go beyond that request-level decision rule. [Cedar security semantics](https://docs.cedarpolicy.com/other/security.html), [Cedar request/policy concepts](https://docs.cedarpolicy.com/overview/terminology.html).
- **Institutional action languages are close to the event-and-norm layer.** InstAL provides event-based institutions, permissions, obligations and institutional power, with a formal model and translation to answer-set programs. Comparing this paper only to static access-control languages would omit that closer institutional baseline. No equivalence or superiority for the proposed leases, ranks or reservations has been demonstrated here. [Padget et al., InstAL: An Institutional Action Language](https://purehost.bath.ac.uk/ws/files/148119668/instal.pdf).
- **The numerical reservation core is escrow reuse.** O'Neil's method supports concurrent/long-lived updates through constrained aggregate quantities; later bounded-counter work applies escrow ideas to numeric invariants in eventually consistent stores. Those results do not prove that this particular norm-version migration preserves duties or allocates cancellation losses correctly. [O'Neil, The Escrow Transactional Method](https://www.cs.umb.edu/~poneil/EscrowTM.pdf), [Balegas et al., Extending Eventually Consistent Cloud Databases for Enforcing Numeric Invariants](https://arxiv.org/abs/1503.09052).

**My judgement:** this is an attempted integration of established priority reasoning, authorization, institutional duties and escrow. Its potentially distinctive content is the exact composition contract: authority-path attenuation, deterministic scoped overrides, persistent obligation instances and financed rule changes under leases. That contract is presently incomplete and inconsistent. I found no basis to call the four-form language or greedy selection individually novel, and no basis to claim the complete intended integration already exists unchanged in any one cited system.

## Final verdicts

**Feasibility test:** false as an unrestricted soundness claim. A restricted independent, atomic, single-instance reservation theorem is plausible and elementary. Deadlines, repeated triggers and state interference need explicit restrictions or a richer test; the conservatism can grow without a constant bound.

**Selection:** it can return an infeasible effective set, and does not yet define a unique result on every input. A fixed total order would settle iteration order only after demotion ties are specified; it does not settle ambiguous exceptions. Permission support, cap processing and persistent instance migration need repair.

**R3:** false as written, even with a perfect selection and perfect logs. Authorized emissions do not imply fulfillment. A safety-plus-detection replacement is defensible under explicit evidence assumptions; the plan's translated local-check and conflict-bound target remains unproved.

**R4:** false as written. Unconditionally inserted caps and reductions below past spending contradict its first conjunct. The last-issuance ledger bound can survive independently, with disjoint categories and grandfathered exposure; it does not prove the full statement or compensation semantics.

**R5:** not established in the advertised generality. The immutable-interpreter/namespace safety core is sound under trusted mediation, but does not supply trusted data, delegation inheritance, amendment authorization, useful changes or all of the plan's entrenched invariants. Universal effective amendment acceptance is incompatible with prohibitions and charter authorization; universal proposal acceptance is a much weaker claim. I do not claim an unconditional logging bypass against a fully specified immutable trusted boundary.

CRITICAL: 3  HIGH: 8  MEDIUM: 2  LOW: 0

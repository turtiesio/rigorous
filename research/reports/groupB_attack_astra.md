# Adversarial review: group B accountability

I read the group B sketch first, then the corrected first pass, both earlier attacks, and the governing plan. Only this review file was written. Numbered findings alone enter the severity count. “Shown wrong” means an explicit counterexample; “could not verify” identifies missing specifications; “my judgement” is an assessment, not a theorem. Admitted open questions are not counted merely for being open.

The proposal improves substantially on the first pass: authenticated synchronous broadcast can solve the honest-witness disagreement problem, a declared time bar acknowledges unbounded external delivery, and the worked 0.8/0.5 example is arithmetically correct. But the proposed model does not reproduce even every contract-shaped delivery chain, and closed summaries do not supply the numerical data its verdict requires.

For the arithmetic examples below, times are exact, margins are zero, and links have zero duration unless specified. Equivalently, use bounds already adjusted for the declared margin and scale the examples far above timestamp uncertainty. No example depends on rounding at a deadline.

## 1. CRITICAL — Clipping early responses to zero falsifies B3, even on a chain

**Text attacked:** “its response time to an input is the response bound plus an amount that does not depend on when the input arrived”; “For a tree of contract-shaped controllers … the same answer”; “the time the last unit receives is the start time plus the bounds, the shortfalls and the link delays.”

**Status: shown wrong.** Consider this chain, starting at time 0, with final deadline 7:

| Unit | Bound | Fixed actual processing duration | Input | Output | Shortfall |
|---|---:|---:|---:|---:|---:|
| A | 2 | 4 | 0 | 4 | 2 |
| B | 4 | 1 | 4 | 5 | 0 |
| C | 1 | 3 | 5 | 8 | 2 |

All three are contract-shaped under the stated definition: their signed deviations from their bounds are constant, respectively +2, −3, +2. The deadline follows from the bounds, whose sum is 7. In an actual-controller rerun, repairing A alone produces final delivery at 6; repairing C alone also produces delivery at 6. The minimal repair sets are {A} and {C}.

The declared contract arithmetic replaces B's duration 1 by bound 4 plus shortfall 0. It predicts actual delivery at 11, not 8. Repairing only A or only C then gives 9, still late; repairing both gives 7. Its unique minimal repair set is {A,C}. This is inside the admitted delivery-chain class, not the acknowledged open problem of non-time guarantees. B3 and R2(d) are false as stated.

**Evidence:** Direct substitution above. The problem is information discarded by `max(0, duration − bound)`, although the timestamps themselves contain that information. The same issue occurs for a link faster than its allowance if only its positive excess is retained.

**Exact repair:** Retain signed residuals for response durations and actual link delays. For unrepaired units preserve the signed residual; for repaired units replace only positive residuals by zero, explicitly preserving already compliant early responses. Alternatively restrict B3 to controllers with nonnegative constant residuals and links that take allowance plus excess exactly. State this restriction in R2. Define the counterfactual's response equations, not merely upper bounds. Also weaken the introductory “exactly for”: non-contract-shaped controllers can happen to have identical repair verdicts, so contract shape is a sufficient condition, not a necessary one.

## 2. CRITICAL — B4's two general arithmetic tests are false

**Text attacked:** “If the link excesses alone amount to E or more, no set of units repairs it”; “shortfalls sum to at least E less the slack the contracts left.”

**Status: shown wrong.** Let A receive at 0, have response bound 9, and hand over at 12: shortfall 3. The outgoing link allowance is 1, actual delay 5, excess 4. Delivery is at 17 against deadline 15. The baseline is 10, slack S=5, observed miss E=2. Link excess 4 exceeds E, but repairing A gives delivery at 14, which succeeds. The second test gives threshold E−S=−3, declaring even the empty set a repair although actual delivery at 17 fails.

**Evidence:** In the additive, nonnegative-residual model let baseline B include response bounds and allowances, X be total link excess, Q total shortfall, and D=B+S. Actual delivery is B+X+Q, so E=X+Q−S. Repair R succeeds exactly when `sum(shortfalls in R) >= E`. A repair exists exactly when `X <= S`. Slack has already been subtracted in E.

**Exact repair:** Replace both tests by these formulas. For signed residuals use the reconstructed actual delivery and its reduction under each intervention. Do not subtract slack a second time.

**What is right:** In B4's worked example the implied U3 input is day 1 and response bound 3: hand-over 4.8, receipt 5.8, U2 hand-over 9.3, final receipt 10.3. The two one-day links leave baseline 9, no slack, E=1.3, and Q=0.8+0.5. Either repair alone fails; both give day 9. The 8:5 allocation follows the chosen convention. These are model counterfactuals; rerunning the first pass's input-dependent controller instead can make U3 alone sufficient.

## 3. CRITICAL — Closed-at-deadline evidence cannot determine actual shortfalls

**Text attacked:** “the certified time … or the certified fact that there was none by the coverage time”; “Every variable of the model appears in some closed summary”; “a shortfall is computed from the owner's certified input and hand-over times.”

**Status: shown wrong.** One unit receives at 0 and must hand over within 1 day. Its coverage time is 1. In history H it hands over at 2; in history H′ it hands over at 100. The closed summary is identical: input 0, no hand-over by 1. Its actual shortfall is 1 in H and 99 in H′. If later hand-overs are admitted into a subsequently signed summary, the claimed unique closed summary changes instead (finding 9).

This is already a problem in B4's own example: U3's nominal coverage is day 4 and U2's is day 8, yet the example uses hand-overs at 4.8 and 9.3. Small margins do not bridge those gaps. Moreover, if a late input arrives at 4.8 and the response bound is 3, a unit closing at nominal day 7 has not yet violated its response guarantee at 7; its actual due time is 7.8. Treating “no output by nominal coverage” as a positive shortfall manufactures a finding against a compliant unit.

**Evidence:** Two identical allowed summaries with different required numerical variables disprove B1's justification and the unqualified functional claim R2(c). A permanent omission has no finite shortfall at all. This is distinct from a summary being absent: the unit has supplied exactly the closed summary requested.

**Exact repair:** Choose explicitly between (i) eventual completed-event summaries, sacrificing a fixed finite closing time; (ii) censored summaries with lower bounds/intervals and an “undetermined” causal verdict; or (iii) a finite-horizon legal model with a separately specified terminal-omission penalty. Define triggering inputs, deadlines after late inputs, and missing inputs in that model. Do not divide monetary loss in proportion to undefined or infinite shortfalls. Claim exact repair sets only for fully observed finite responses, or prove a different theorem for the chosen censored semantics.

## 4. CRITICAL — Unqualified voiding on equivocation is incompatible with final receipts

**Text attacked:** “the one batch the controller proposed, or ‘void’ if the controller proposed two”; “two proposals for one position are proof that the controller deviated, and the position is void.”

**Status: shown wrong under the stated unqualified rule.** With f=1, honest H1,H2 and faulty F, all decide A at slot 7 by time 3, and H1,H2 certify it. An honest outside party retains that receipt. At time 100 the controller signs a conflicting B for slot 7, or F discloses a B it held privately throughout. If the slot is now void, an irrevocable receipt has been erased and successors referencing A have lost their basis. If it remains A, the quoted unconditional voiding rule is false. No bounded-time protocol can detect a proposal kept exclusively by Byzantine parties or one not yet created.

There is a separate pre-decision attack: the controller gives two admissible proposals during the slot. A default/void decision can be correct, but erasing the batch must not erase authenticated observations of already performed effects or of inputs delivered to honest witnesses. Otherwise a receiver can void the batch containing its input, then avoid a finding whose predicate requires that input to be recorded. Broadcast agreement alone supplies no completeness condition for those observations.

**Evidence:** The hidden-message indistinguishability argument is elementary. Published broadcast protocols promise agreement, termination, and validity for an honest sender, not detection of every secret or future equivocation; see the protocol audit below. This is a concrete contradiction despite the author's admitted uncertainty about the protocol.

**Exact repair:** Finalize the slot once. Default only on conflicts admissible within the defined instance; append later equivocation evidence without revoking prior receipts. Preserve independently authenticated observations and conflict proofs, and require witnessed inputs to enter the log despite controller omission or equivocation. A void is a certified tombstone with a successor hash, not destruction of history.

## 5. HIGH — The time bar lacks an agreed, accountable intake boundary

**Text attacked:** “on the closed summaries then held”; “the collector … is trusted only to collect”; “the verdict is a function anyone can recompute.”

**Status: shown under-specified by an explicit boundary history; could not verify a unique verdict from the stated intake rules.** Let the collector have H1,H2,F, internal delay bound 1, and bar 10. A valid summary reaches H1 at 9.9 and H2 via honest forwarding at 10.9. Both are honest. At the bar one holds it and one does not. Does the unit count as present? No acceptance rule says. If only F receives at 9.9 and withholds it until 11, presence cannot depend on F's private knowledge. If a collector controller can choose the submitted list, it can omit an honestly received exculpatory summary and shift loss to that compliant unit. Merely recomputing arithmetic on that chosen list cannot expose the omission.

A faulty unit may deliberately withhold its own summary; suspicion and loss on itself are the stated consequence, so this alone is not an attack refuting the scheme. Nor can f faulty witnesses forge an invalid summary if honest signers validate its full contents. But submitting only to faulty collector witnesses, or just before the bar, leaves precisely the undefined boundary above. A collector whose entire witness bound fails cannot be defended by this scheme; that limitation must remain explicit.

**Evidence:** Direct timed delivery history, all internal honest links within their bound. The admitted unfairness of genuinely late delivery does not settle whether an on-time intake counts or how anyone verifies the collector's omission claim.

**Exact repair:** Define acceptance by a collector-certified intake entry, with a deterministic cutoff in the collector's agreed log and a bounded drain/decision period after it. Have witnesses directly ingest and forward submissions, publish a certified manifest including missing-unit markers, and bind the verdict to that manifest and contract version. Give senders intake receipts. Distinguish timely delivery to one endpoint from accepted submission. R2(c)'s inputs must include the manifest/bar metadata, not only free-standing summaries. Suspicion remains unavoidable for submissions not accepted by the cutoff.

## 6. HIGH — Multiple guarantees and tree joins need equations that bounds do not supply

**Text attacked:** “each guarantee names the inputs it responds to”; “its equations are the arithmetic of the response bounds”; “every shortfall of those units set to zero.”

**Status: shown inconsistent under the equation reading, and otherwise could not verify a uniquely specified model.** One unit receives X at 0 and has two guarantees for the same Y: within 2 days and within 3 days. It emits Y at 4, with shortfalls 2 and 1. Turning both shortfalls to zero as response equalities requires Y=2 and Y=3 simultaneously. Reading guarantees merely as inequalities permits Y≤2 but does not select a response time. Both contracts are jointly satisfiable; the counterfactual construction, not the contracts, created the inconsistency.

A genuine join similarly needs a trigger rule. If inputs arrive at 2 and 5 and the promise is “within 1 day of receiving X and Z”, all-input readiness normally yields 6; adding path durations is not that operation. The public model must say whether readiness is max, first arrival, or separate conditional guarantees. One scalar “total needed” does not describe all branch constraints.

**Evidence:** Direct equations above. This is not the admitted uncertainty about quality/spending guarantees: these are ordinary temporal response contracts. It also reopens the earlier review's multi-guarantee ambiguity in a new form.

**Exact repair:** Use one variable per physical event, a declared readiness function for each output, and one deterministic response equation per event. Treat additional bounds as predicates on that equation, with a canonical simultaneous-compliance intervention. Alternatively restrict the theorem to one guarantee per output and max-readiness joins with independent constant processing times. Require fixed link delays, no shared resources or cross-output scheduling effects, and closure of this class under the contemplated interventions. Under those conditions an induction over the tree proves the repaired B3; “tree” by itself does not.

## 7. HIGH — The loss rules prescribe incompatible payers

**Text attacked:** “Where a repair set exists the loss falls on the units with findings”; “lateness on a link is borne by the sender”; “Absent summaries are treated as above.”

**Status: shown wrong as a single allocation rule.** Grant complete event evidence here to isolate allocation from finding 3. A receives at 0, bound 1, hand-over 1. Its link allowance is 1 but receipt by B occurs at 3, excess 1. B's response bound is 1; it hands over at 6, shortfall 2. Final deadline is 4 (baseline 3, slack 1). Repairing B gives final delivery at 4, so {B} repairs. The first loss rule puts the entire loss on B, the only unit with a finding. The sender rule puts link lateness on compliant A. These are compatible only if “loss” is divided into separate priced components or one rule has priority; neither is specified.

For absence, take two parallel branches into a final all-input join. A is due at 1 and delivers at 2; B is due at 1 and delivers at 1. A's closed evidence is available but compliant B's summary misses the bar. The findings rule charges A; the missing-summary rule charges B. It is unspecified whether B pays everything, shares with A, or pays a separate amount. With two missing summaries, “the loss is allocated to it” is likewise not a normalized allocation.

**Evidence:** Direct application of the three clauses to the same histories. This is not an objection that proportional sharing is morally unfair; it is failure to define one payable amount and payer set.

**Exact repair:** Declare an ordered total allocation function, including multiple missing units and multiple link/environment bearers, whose weights sum to one. If link loss is separately compensable, define its amount and exclude double recovery. If the findings rule overrides sender liability, explicitly qualify the sender convention.

**Further judgement, not a separate defect:** Counting shortfalls across guarantees is sensitive to representation. In a join of branches A and B with respective response bounds 2 and 3, both finish at 4 and final deadline is 3. Repairing A alone fails because B still finishes at 4; repairing B alone also fails. Both repair. Weights are 2:1. Duplicating A's identical guarantee changes the weights to 4:1 without changing behaviour or feasible outcomes. Canonicalize obligations or expressly accept this dependence; it is especially undesirable in a foundation concerned with invariance under regrouping.

## 8. HIGH — Minimal repairs are not the plan's minimally sufficient deviator sets

**Text attacked:** “The causal definition is a choice … one choice makes R2 true”; “the causal part of the verdict is the collection of minimal repair sets.”

**Status: shown different from the governing target; my judgement is that this is a legitimate replacement target only if labelled as such.** A receives at 0, bound 1, emits at 2. B receives at 2, bound 1, emits at 4. Final deadline is 3, so slack is 1. Repairing either unit gives delivery at 3: minimal repairs are {A} and {B}. Keeping only either unit's deviation also gives 3, not failure. Both deviations together are needed for failure: the minimal sufficient deviator set under this fixed-deviation model is {A,B}.

**Evidence:** [Künnemann–Esiyok–Backes, §II, Definition 1](https://arxiv.org/pdf/1805.10891) asks for minimal sets of deviating parties sufficient for a violating related trace. It does not ask for minimal sets restored to compliance. With unrestricted deviations the relation changes the answer again; the relation must be explicit. The plan names minimally sufficient sets and says a counterexample-driven replacement is a new conjecture, not the surviving target. This is the earlier reviews' warning about causal definitions, now resolved by choosing a different object rather than implementing the original one.

**Exact repair:** Call the output “minimal sufficient repairs under a declared intervention model”. Amend the target and stop-rule assessment explicitly. If the intended target remains KEB accountability, specify the protocol and counterfactual relation, compute sufficient deviator sets, and prove the summary abstraction preserves that relation.

**Relation to other standard notions:** In a flat acyclic model with one binary deviation variable per unit (actual deviation or zero), fixed context, and a single deterministic failure output, these minimal repairs can instantiate conjunctive but-for causes and the modified HP definition. This requires the restricted variable domains and intervention language; it is not general HP actual causation with arbitrary contingencies. See [Halpern 2015, AC1–AC3 and AC2(am)](https://arxiv.org/pdf/1505.00162). Adding intermediate timing variables and allowing them to be held fixed can alter the causal model. The sketch must state its instance.

Model-based diagnosis is another useful analogy, not an identification: a Reiter diagnosis minimally relaxes normal-component assumptions to make a system description consistent with observations, whereas this rule changes components to achieve a desired outcome. The diagnosis/conflict minimal-hitting-set relation is stated in [Greiner, Smith and Wilkerson's correction to Reiter](https://webdocs.cs.ualberta.ca/~rgreiner/PAPERS/correct.pdf). That result does not by itself establish the sketch's repair semantics. This source was available as search-indexed text; its direct PDF fetch failed.

## 9. MEDIUM — Common logs do not alone define exactly one closed summary

**Text attacked:** “Since all honest witnesses hold the same log, there is one closed summary per unit and property.”

**Status: could not verify without a canonical extraction rule.** A shared log can contain multiple performances or re-deliveries for one guarantee, different property/contract versions, and events after coverage. At input 0, deadline/coverage 1, deliveries at 2 and 3, a signer after time 3 needs a rule selecting “none by 1”, the first delivery, or a designated discharge event. Agreement on the source log does not choose the function extracting the summary. A silent controller may also never propose the “batch covering” the time, leaving zero summaries, not exactly one.

**Evidence:** This is a specification gap, not proof that honest witnesses can sign contradictory values under a deterministic canonical summarizer. That stronger premise would close it.

**Exact repair:** Bind a summary to property instance, contract version, fixed coverage frontier, canonical event-selection rules, and a deterministic encoding. Sign the summary once that frontier is finalized. Provide scheduled closure/tombstone slots independent of controller activity. State “at most one valid summary value; eventually one under the stated witness-service liveness conditions”. Multiple signature subsets need not produce identical certificate bytes.

## Protocol audit: what is available, and what it actually buys

[Dolev–Strong, Authenticated Algorithms for Byzantine Agreement (1983)](https://www2.imm.dtu.dk/courses/02220/2015/L12/DolevStrong83.pdf) supplies synchronous authenticated agreement with honest-sender validity and bounded termination, with t+1 rounds for t Byzantine participants. Its optimized algorithm advertises O(nt) messages; this is not a bound on payload bytes or signature words. The familiar capped forwarding construction sends O(n²) messages carrying up to O(t) signatures each, hence O(n²t) signature words. Sender membership, round numbering, authentication and value-size limits must be fixed before instantiation. These guarantees allow n=2f+1, t=f; signatures remove the unauthenticated three-to-one requirement.

[Lamport–Shostak–Pease, signed-message algorithm SM(m), §4](https://lamport.azurewebsites.net/pubs/byz.pdf) is another suitable agreement primitive with up to m faulty participants. Its signed-path forwarding through depth m takes m+1 communication stages with synchronized timeouts. The naive algorithm can have O(n^(m+1)) transmissions (a loose bound on distinct paths), so it is not the efficient implementation to choose for large m. Its deterministic choice from received orders can use a default for empty or conflicting sets. Neither algorithm promises that a malicious sender supplies useful content or that all private equivocations become known.

The following are my application deductions, not additional theorems attributed to those papers:

- If the controller is one of the n parties, count its faults in t. If it is an additional potentially faulty sender beside 2f+1 witnesses, the direct instantiation has n=2f+2 and t=f+1, giving f+2 rounds. One can instead design a witness-only protocol around externally signed controller proposals, but must specify it rather than silently reuse f+1.
- A fixed schedule and default for no admissible proposal let a silent/slow controller cause a default, not a stall. There is no bounded useful-batch liveness against a permanently silent controller. The plan already bounds controller-to-witness delivery; scheduling and controller computation still need rules. A per-batch clock started only by controller activity does not advance on silence.
- Domain-separate signatures by unit, epoch, slot, predecessor and decision type. Once all honest witnesses decide one value and obtain its data, require all to broadcast their decision signatures. An extra bounded dissemination round with O(n²) transmissions lets all reconstruct f+1 signatures. Faulty witnesses cannot forge an alternate decision or prevent honest reconstruction. They can assemble or privately hold their own copy of the valid certificate; “cannot complete or withhold” must mean cannot control its availability to honest witnesses. Delivery to an external collector is still unbounded.
- Serial instances with a certified predecessor, common slot schedule, durable storage, available batch data and a canonical void successor support an induction establishing a common sequence at slot completion. Merely hashing a predecessor does not provide any of these liveness conditions. Broadcasting a digest without distributing the payload is insufficient.
- A continuously correct witness cannot miss a completed batch in the specified reliable synchronous model. A crashed or recovering witness that did miss it is outside that premise. The literal “only if it joined the decision on its predecessor” excludes it forever. Either exclude recovery, or permit validated state transfer of certified prefixes and payloads followed by joining a future scheduled instance, with an explicit fault/recovery model. State transfer is not retrospective participation.
- Agreement is not accurate timestamping, validity of performance, or complete inclusion. Honest signatures must validate authenticated event identities, local observations, admissible timestamps and canonical extraction. Inherited replay checks from the plan help, but controller proposals still need an omission-resistant path into witnessed history.

The old split-signature attack is therefore genuinely repairable with published machinery. The blanket claim that no protocol can work with 2f+1 signed synchronous witnesses would be wrong. Finding 4 concerns the stronger, impossible unconditional voiding semantics, not the solvability of authenticated broadcast.

## B2, burden of production, and what is genuinely protected

Under the plan's terminology only a proved finding “names” a unit. Read that way, B2 is sound **conditionally**: an honest witness must validate its unit's actual input observation and output event, not merely a neighbour's claimed timestamp. A late input by itself is not a response shortfall: input 5, bound 3, output 8 gives zero, however late the chain is. This correctly avoids blaming a compliant downstream unit for inherited lateness.

A faulty neighbour cannot create an own-log finding merely by denying receipt. Link excess, however, uses two units' evidence, and suspicion uses the collector's intake record. Thus B2 supplies no protection against loss imposed using someone else's evidence. The heading is defensible only with the plan's narrow meaning of “named”; it must not be advertised as protection against all adverse verdict components. No additional defect is counted for that already-established distinction. If the local controller is not delivered an input when its witnesses are, define the unit's response trigger as witness receipt or explicitly account for that internal delay; changing the event whose time is used changes the obligation.

## B5: the hardness argument is right; the output contract needs limits

Here is a complete reduction, rather than a complaint that the author left one open. For positive integer subset-sum weights a_i and target T, make ordinary shortfalls 2a_i, a distinguished unit's shortfall 1, and repair threshold 2T+1. A minimal repairing set containing the distinguished unit must have other weights summing exactly 2T: removing that unit must cross below the odd threshold. Conversely any subset summing to 2T, together with the distinguished unit, is minimal because deleting any positive weight crosses below the threshold. This is a polynomial reduction; membership is in NP by checking the sum and each deletion. The integer equality problem is Karp's KNAPSACK formulation in [Reducibility Among Combinatorial Problems](https://www2.seas.gwu.edu/~simhaweb/champalg/tsp/papers/KarpOriginal.pdf). The reduction is my derivation.

Enumeration also has an unconditional output obstacle: n unit shortfalls of 1 and threshold k yield all binomial(n,k) k-subsets as minimal repairs. One minimal repair is easy in this monotone setting: start with all and greedily remove dispensable units. Exponential output and the membership decision problem should not be conflated with difficulty of finding any repair. The subset-sum reduction establishes weak NP-hardness; bounded integer weights permit pseudo-polynomial methods.

Findings plus a compact timing model are a reasonable bounded report, but not a complete list of causal sets. On a tree retain the public topology, join functions and per-event data, rather than a single chain threshold. Say explicitly “enumeration omitted above K units”, and make K a fixed declared constant if claiming linear output size. With fixed K, the extra enumeration contributes a constant O(K·2^K); if K is input-dependent, R2(c) cannot promise linear explicit output. Count timestamp bit lengths, guarantee-input incidences, and certificate sizes too: “linear” can mean linear records with fixed-size numeric fields and fixed witness parameters, not unconditionally linear bits in the number of units. These are qualifications, not a counted refutation of B5.

## Earlier findings, novelty, and final verdicts

The earlier prefix and split-signature criticisms are addressed in direction, but need the concrete protocol wrapper above. Late hidden equivocation remains unfixed. Coverage times repair stale-prefix selection only after defining canonical extraction; they introduce the censored-shortfall problem. The time bar makes the old unbounded waiting problem an explicit policy cost, but the collector's intake semantics and allocation precedence remain unsettled. The earlier causal ambiguity has been replaced by a clear intention to compute repairs, yet the response equations and relationship to the plan's sufficient-deviator target remain unresolved.

**My judgement on novelty:** “An application of known pieces” is a defensible description of a repaired narrow model. Broadcast is published; the constant-delay induction is elementary; restricted but-for repair semantics can be stated as a standard structural-model instance; and a time bar is a policy choice. “A consistent assembly” is false for the current text. “Nothing new” is not a verified exhaustive novelty finding, and missing integration arguments cannot be dismissed by naming their ingredients. No inner logs are needed to repair the arithmetic or event-level equations; the plan's specific stop rule about a weakest repair requiring inner logs has not been established here. That does not mean the current R2 has survived unchanged.

| Claim | Verdict |
|---|---|
| B1 | False as a complete claim over the admitted histories: closed summaries lack eventual shortfalls, and the model is not total for omissions. True by construction only after fixing the model, observation horizon and intake manifest. |
| B2 | Sound for the narrowly defined own-summary finding with authentic locally validated inputs and the local witness bound. Does not protect against external-evidence loss or suspicion. |
| B3 | False as written, already on the constant-duration chain in finding 1. A useful sufficient-condition theorem survives with signed residuals, explicit joins, fixed environment/link behaviour and intervention closure. The class is nonempty and useful for isolated fixed-duration tasks; queues, calendars, shared resources and input-dependent execution generally escape it. |
| B4 | General formulas false. The particular 0.8/0.5 example and its 8:5 convention are correct under complete event evidence. |
| B5 | Correct for finite encoded additive shortfalls; reduction supplied. Bounded enumeration is an honest output compromise, not complete causal accountability at arbitrary size. |
| R2(a) | Not established over the stated domain. With complete finite observations, closed dependencies, sound joins and consistently margin-adjusted deadlines, backward reasoning gives the intended disjunction. Censored/missing-input cases currently lack the needed semantics. Failure must use the same adjusted bounds as findings; raw deadline failure need not entail any beyond-margin local finding. |
| R2(b) | True by the finding definition, hence largely tautological. It is weaker than proving that real compliant units never acquire false shortfalls. That needs timestamp, trigger and completeness soundness. |
| R2(c) | Not true as an unqualified total-verdict claim: findings 3, 5 and 6 identify missing numerical or public inputs/semantics. Linear records are plausible for a repaired finite-horizon report with fixed enumeration cap and declared encodings. |
| R2(d) | False by finding 1, independently of broadcast faults. True for a tightened explicitly defined constant-residual intervention model by induction over its acyclic event equations; it does not thereby become the plan's minimally sufficient deviator verdict. |

CRITICAL: 4  HIGH: 4  MEDIUM: 1  LOW: 0

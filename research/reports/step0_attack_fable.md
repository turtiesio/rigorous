# Adversarial review of "Step 0 worked on paper"

Reviewer: Fable. Target: `/root/mic/reports/Step 0 worked on paper.md`, read against `/root/mic/reports/Integrated foundation plan.md`.

Each finding is tagged **shown wrong** (an explicit history or a quoted source contradicts the text), **could not verify**, or **judgement**. Sources marked ‡ are cited from my memory and were not opened in this review; all others were opened or returned by a search during it.

**What is right.** The core uniqueness claim of D8 is true as argued. The D11 case arithmetic is correct. The D12 limit argument is sound for properties with a bound that does not depend on pool size. Result 2's underlying point survives with a repaired example. The assessment that R4 is known work is correct. No finding is CRITICAL.

---

## HIGH

### H1. D8: uniqueness holds, but the three consequences drawn from it do not follow

**Text attacked.** "One certified log per unit. Verdicts no longer depend on evidence in transit from other units, so the choice between 'never final' and 'a time bar' that the reviews forced does not arise." Also the case rows "Cannot happen: a receipt needs an honest signature, which is bound to a position in the one certified log" and "A valid receipt is already in the certified log."

**What is true.** "At most one head per sequence number can be certified" is correct. The pairwise form is cleaner than the document's: if honest w1 signs A (received at a) and honest w2 signs B (received at b), then w2 not having A at b+Δ+ε forces a > b+ε, and w1 not having B at a+Δ+ε forces b > a+ε, a contradiction. So no two honest witnesses sign different heads, and with at most f faulty signers only one head reaches f+1. This is the published "forward, wait Δ, vote, certificate of f+1" rule of 1Δ-SMR for f < n/2 ([Abraham, Nayak, Ren, Xiang, post](https://decentralizedthoughts.github.io/2021-03-09-good-case-latency-of-byzantine-broadcast-the-synchronous-case); [paper](https://arxiv.org/abs/2102.07240)‡). It counts as reuse.

**Counterexample to the consequences (shown wrong).** Take f = 1, witnesses w1 and w2 honest, w3 faulty, Δ = 1, ε = 0.1, sequence number 7.

| Time | Event |
|---|---|
| 0.00 | Controller sends head A to w1 only. w1 forwards it |
| 0.95 | Controller sends head B to w2 only. w2 forwards it |
| 1.00 | A reaches w2 (echo, at the bound) |
| 1.10 | w1 has waited Δ+ε, has not seen B, and signs A |
| 1.95 | B reaches w1 (echo, at the bound) |
| after | w2 holds A and B and signs neither. w3 signs A and keeps the signature |

A now has exactly one honest signature. Whether A is "certified" is decided by w3 alone, at a time of its choosing, including after a verdict. A receipt on an entry of A signed by w1 and w3 is "valid with f+1 witness signatures" under the document's rule, yet its standing is evidence withheld by a colluder. That is the situation the document says cannot arise.

**Why this is not a boundary-case slip.** The echo gives non-equivocation among honest signers. It does not give agreement on whether a head was certified: honest witnesses can end in different states (w1 signed, w2 refused). Agreement with a faulty sender costs f+1 rounds in the worst case even with signatures and synchrony ([Dolev–Strong, exposition](https://decentralizedthoughts.github.io/2019-12-22-dolev-strong); lower bound restated in [Lewis-Pye & Roughgarden](https://arxiv.org/pdf/2101.07095)). 1Δ-SMR closes the gap with locking and leader change, which a unit with one fixed controller does not have. Voiding sequence number 7 on proof of a fork does not work either, because a controller could then erase a fully certified entry by signing a conflicting head later.

**Two further gaps, same heading.**
- *Stale summaries.* The echo prevents two logs; it does not prevent two valid summaries of different prefixes of one log. A summary of U2's log at sequence 5 ("goods from U3 not received") and one at sequence 6 ("received day 4.5") are both validly signed. Presenting the older one moves the finding from U2 to U3. The plan's known problem for D8 named exactly this for forked logs.
- *The trigger in D9.* The verdict bound starts when "the summaries for a chain are logged at the unit that computes the verdict". Links from a unit's witnesses to the adjudicator have no bound, so a missing summary cannot be told from a withheld one. "Never final or a time bar" has been moved into this precondition, not removed.

**Repair.** (a) Either run f+1 forwarding rounds on the certificate so that all honest witnesses agree by (f+2)Δ whether a head is committed, or state a time bar and say so. (b) Give every summary a certified coverage time, and have honest witnesses sign a summary for deadline d only after d + Δ + skew. (c) Withdraw the sentence that the finality choice does not arise.

### H2. Result 2: the counterexample is arithmetically wrong in the stated setting

**Text attacked.** "History 1 … U2 hands over on day 8.5, in time … Both: either one complying would have saved the delivery."

**Shown wrong.** The setting says each hand-over allows one day for the link, U2 guarantees hand-over by day 8, and the chain property is receipt by day 9.

- History 1, U3 on time (goods on day 5): U2 hands over on 8.5. That breaks U2's day-8 guarantee, whose assumptions now hold, and with a one-day link U1 receives on 9.5. Not in time.
- History 1, U2 complying (goods on 5.8): the day-8 guarantee is not owed, so compliance means hand-over by 8.8, and U1 receives by 9.8. Not saved.
- History 2 gives the same two outcomes (9.3 + 1, and 8.8 + 1).

Under the stated link allowance both histories have the same counterfactual structure: the failure is avoided only if both comply. The claimed difference in verdicts appears only if the link is assumed to take at most 0.2 days in the counterfactual, which the document does not say.

**Second gap.** "If this unit had complied" is not one behaviour. With goods on 5.8, any hand-over up to 8.8 complies. Whether U2 is a cause depends on which compliant behaviour is substituted, and the document does not choose.

**Repair (the result survives).** Fix the link at exactly one day in every counterfactual. Keep the contracts. Actual run: goods reach U2 on 5.8, U2 hands over on 9.3.

| | U2's rule | If U3 had been on time (goods on day 5) | U3's lateness |
|---|---|---|---|
| History 1′ | 3 days if the goods arrive by day 5, otherwise 3.5 days | Hand-over day 8, receipt day 9, property holds | A but-for cause |
| History 2′ | Hand over on 9.3 whenever the goods arrive | Hand-over 9.3, property fails | Not a cause, with or without holding other variables at their actual values |

The boundary events are identical and U3's causal status differs under the modified Halpern–Pearl definition ([Halpern 2015](https://arxiv.org/pdf/1505.00162)). State the substitution rule for "complied" explicitly: latest compliant time, or all compliant behaviours.

### H3. Result 1: "exactly one" is false, and the search is not a function when a unit has two guarantees

**Text attacked.** "exactly one of these is found: a unit that deviated; a hand-over made in time and received late …; or a broken assumption about the environment", and "If U2's assumptions were met, U2 deviated, and the search stops."

**Counterexample A (shown wrong).** All orders on time. U3 hands over on day 4.0. The link delivers to U2 on day 5.5. U2 hands over on day 9.0. U1 has nothing by day 9.
- Read with U2's day-8 guarantee: its assumption (goods by day 5) failed, so move to U3; U3 was on time; the finding is a link loss borne by U3. U2 is not named.
- Read with U2's 3-day guarantee: its only assumption (order by day 0) held and 3.5 > 3, so U2 deviated and the search stops. The link loss is not found.

Two of the three kinds are present, and the text does not say which guarantee "its guarantee" is. Note also that U2's breach changed nothing here: a compliant U2 hands over by 8.5 and U1 still receives after day 9. This is the plan's variant 11 (an irrelevant breach).

**Counterexample B (the document's own).** In the Result 2 history U3 is late and U2 breaks its 3-day guarantee. Two units deviated. The search stops at U2 and never reports U3.

**Consequence the document misses (judgement).** The chain property is implied by the day-8 guarantee and does not need the 3-day guarantee. A search that follows only the guarantees the property depends on names U3 alone in the Result 2 history, while the actual-difference verdict for History 2 is U2 alone. The two notions can name disjoint parties, so calling one "the weakest repair" of the other is not accurate.

**Repair.** Define the verdict as the set of all findings on the dependency tree of the failed property, where each guarantee lists the guarantees and link bounds it relies on. Replace "exactly one" by "at least one". Report breaches outside the tree separately as breaches that are not findings for this property. With that change the composition claim holds, and it remains reuse: deterministic compositional blame for contracts is published ([Hvitved, Klaedtke & Zălinescu](https://dblp.org/rec/journals/jlp/HvitvedKZ12)).

### H4. D1/D7 against Result 1: the benchmark's assumptions cannot appear in a summary

**Text attacked.** D1: "on the assumption that some permitted vendor can deliver if ordered by day 8 … The assumption held (a permitted vendor could have delivered)". Result 1: a unit deviates when "its own certified receipts show every assumption of that guarantee had been met".

**Counterexample (shown wrong, as an inconsistency between two parts).** History X: the unit never orders, and a permitted vendor would have delivered. History Y: the permitted vendor failed on day 2 and the unit never orders. D1 classifies X as a deviation and Y as nobody's deviation. The unit's certified log is the same in both: no order, no receipt. "Could deliver if ordered" is a disposition of the environment, and no receipt records it. Under Result 1's definition as written the unit is not a deviator in X either, because no certified receipt shows the assumption met. The escape by waiting that D1 was chosen to close is open again.

**Repair.** Restate obligations to achieve as obligations to act, with assumptions over certified inputs only: "place an order with a permitted vendor by day 8", plus a vendor contract "deliver within two days of a certified order". Add a stated default with an evidence rule: an assumption counts as met unless a certified event shows it broken, such as the gateway's certified refusal or failure notice. The plan already lists a vendor contract with a named observation point as reuse.

### H5. D6 contradicts D1: overriding a cap is evaluated at the current state

**Text attacked.** D1: "Conflicts are found and resolved once, when a norm takes effect, and not again as the state changes." D6: "An obligation to spend beyond a cap wins only if its origin outranks every authority whose cap it would exceed. It is then carried out as a recorded raising of the cap".

**Counterexample.** State cap 200, ministry cap 100. A regulator ranked between them issues on day 1 an obligation to buy for 150 by day 10.
- If the test is made on day 1 with nothing spent, the purchase exceeds only the ministry's cap and the obligation wins. The unit then lawfully spends 60 on other things. On day 8 the purchase would bring the total to 210 and exceed the state's cap, which the regulator does not outrank. D1 says the conflict is not looked at again; D6's test now gives the opposite answer.
- If the test is made at spending time, that is evaluation at the current state, which D1 rules out.

Which caps an obligation "would exceed" depends on the ledger total, so the test is state-dependent by nature.

**Two further defects.** (a) The rule is not "selection like any norm". A greedy selection drops the ministry's cap as a whole; D6 weakens it by an amount it does not specify. (b) The outcome depends on order. Discretionary 60 first and the obligation second gives 210 if the raise is "by what is needed"; the obligation first gives 150 and no discretionary spending. A unit gains by spending to the cap before a mandated purchase arrives.

**Repair.** An obligation to spend must state a maximum amount. When it takes effect the ledger reserves that amount as a commitment, and any cap raise is fixed then, as the minimum that admits the reservation. If it cannot be reserved under every cap its origin does not outrank, the obligation is left out at that moment. This keeps D1 and adds obligation reservations to D11's "committed" category.

### H6. D3: the level rule rejects the benchmark's own authority edges and contradicts D4's second case

**Text attacked.** "the anchor's top-level units are at level 0, and a created unit is one level below its creator. An authority edge may run only from a smaller level number to a larger one."

**Shown wrong against the plan's benchmark.** The plan lists "a supranational body over the state" and "the ministry over the group". The group is an enterprise that no benchmark unit creates, so it is top-level, at level 0. The ministry is inside the state, at level 1 or more. The edge ministry → group runs from a larger level number to a smaller one and is rejected. If the state and the supranational body are both top-level, that edge is rejected too. The regulator "in the group's home jurisdiction" is also top-level, so it outranks the ministry and ties with the state, which contradicts "the regulator and the ministry are at the same level" and D4's ordering "state, regulator, ministry". In D4's second case the group's version "loses" to the ministry's norm; with the group at level 0 it wins.

**The anchor's table cannot fix it,** because it orders only units at the same level. Levels fixed at creation also reject a deep public body gaining authority over a shallow private one, for example an inspectorate at level 4 over a group at level 3, and reject an acquisition by a deeper group.

**Repair.** Separate rank from creation depth. The anchor assigns each top-level unit a rank from a stated ordered domain with gaps. A created unit takes a rank strictly below its creator's. An edge is legal when rank decreases. Changing the rank of a subtree is an anchor rule change, versioned and leased. The claim "each declarer can check alone" then needs the target's certified rank.

### H7. D4: an intermediate authority can give its own tightening the origin's priority

**Text attacked.** "Each norm reaches a unit by exactly one path … An intermediate authority's own tightening of a norm is a separate norm with that authority's priority."

**Counterexample.** Priority is state, regulator, ministry. State norm N: purchases above 1000 need approval. The subsidiary's vocabulary has no amount field visible to the officer role, so the ministry translates N on the named path as T(N): every purchase needs approval. T(N) is a sound translation, since whoever complies with T(N) complies with N. Regulator norm R: emergency purchases under 500 are made without waiting for approval. R conflicts with T(N) and not with N. T(N) carries the state's priority, so R is left out, although the regulator outranks the ministry and the state never required this. This is the same case as D4's first row, with the tightening placed in the translation.

**Gap.** The rule assumes a translation can be split into "N itself" and "the tightening". A soundness check accepts any stronger translation. When the target vocabulary is coarser, no exact translation exists and every sound one is a tightening. Deciding the split needs equivalence of norms across vocabularies, which the document does not have.

**Repair options.** (a) A translation carries the lower of the origin's and the translator's priority, which reopens D4's first row. (b) Require a certified exact translation and refuse edges where none exists. (c) Let the origin sign the translation. Each has a cost that should be stated. This is also why the R3 conclusion is too quick (M11).

---

## MEDIUM

### M1. Result 1: an assumption that no one guarantees

**Text.** "it is either U1's order … or U3's goods. In that case look at U3's summary and repeat."

**Counterexample.** U1's order arrives on day 0. U2 sends its order to U3 on day 3. U3 hands over on day 6, U2 on day 9.5. The contracts as listed give U2 no guarantee to order. U3's assumption (order by day 1) failed, so U3 did not deviate. U2's day-8 assumption failed and its 3-day guarantee was kept. Nobody is named, although U2 alone caused the failure.

**Repair.** Add a closure condition: every assumption is either a named guarantee of another unit, with an earlier deadline, or a declared environment assumption with a bearer. Add "U2 sends its order by day 0, assuming U1's order by day 0". This is the standard side condition for circular assume-guarantee reasoning ([Abadi & Lamport](https://lamport.azurewebsites.net/pubs/abadi-conjoining.pdf)‡).

### M2. Result 1: a compliant unit can be named within the fault bound

**Text.** "No compliant unit is named. A unit is named only on its own certified log."

**Counterexample.** D10 lets faulty witnesses shift a certified time by Δ plus the skew. Witnesses are appointed by a registry, not by the unit, so they may act against it. Goods reach U2's honest witness at 5.0 + Δ. One honest and f faulty witnesses certify τ = 5.0, which is within tolerance. U2's controller works exactly three days from when it holds the goods and hands over at 8.0 + Δ. U2's own certified log shows the day-8 guarantee broken with its assumption met. Separately, no bound is stated on the link from witnesses to their own controller, and D10's certificate binds "whatever the obligee's controller logs".

**Repair.** Measure every deadline against certified times with a margin of 2(Δ + skew). State a delivery bound from witnesses to their controller, or define the unit as controller plus witnesses and say that the named party is that whole.

### M3. D10: two valid receipt certificates can disagree about a deadline

**Text.** "an honest witness signs only if τ is within Δ plus the clock skew of its own first receipt … Faulty witnesses can shift the recorded time by at most Δ plus the skew in either direction".

**Counterexample (f = 1).** The performance crosses unit links, which have no bound. Honest w1 first receives it on day 4.9 and honest w2 on day 5.6. D10 states no echo among the obligee's witnesses. Faulty w3 can co-sign τ = 4.9 with w1 or τ = 5.6 with w2. Both certificates are valid. Against a day-5 deadline one discharges the obligation and the other shows a breach, and "a controller that then declares a breach has deviated, provably" is false for a controller holding the second. With an echo the honest receipts lie within Δ of each other, and two valid certificates can still differ by up to 3Δ + 2·skew. The same holds for the "did not receive" entry in a summary: w2 can honestly sign "nothing by day 5" while w1 holds a receipt at 4.95.

**Repair.** Put the receipt through the sequence-numbered echo of D8 so that one τ per performance is certifiable. Sign absence only after the deadline plus Δ plus skew. State the residual uncertainty as a margin in every contract.

### M4. Result 2 is not a refutation "as the plan words it"

**Text.** "the target as the plan words it is false".

**Evidence.** The plan's wording, sets of minimally sufficient sets of parties, is the verdict form of Künnemann, Esiyok and Backes. Their a posteriori verdict is a function of the trace: apv(t) is the set of minimal S for which some related trace t′ has corrupted(t′) = S and violates the property. They say why a deviator's dispositions are left out: "we can only observe deviating parties, but not know which program they run" ([paper](https://arxiv.org/pdf/1805.10891), Definition 1 and the discussion of the control-flow relation). Histories 1 and 2 are one trace, so they get one verdict. Under their weakest relation it is {{U2},{U3}} in both.

**Could not verify.** Whether Gössler and Le Métayer's counterfactual construction extends other faulty components by their specification or by their actual behaviour; the paper was not retrievable ([record](https://dx.doi.org/10.1016/j.scico.2015.06.010)).

**Judgement.** The counterexample is valid only for definitions that use a structural model of the deviating controller. That is a stronger reading than the cited source, and the document's caveat describes the alternative too narrowly as "could have caused the failure by any deviation". The lesson is also understated: under the mechanism reading no record of the actual run supports the verdict, inner logs included, because the fact needed is the controller's program. The plan's axis "needs inner logs or not" is the wrong one. Since the plan leaves the causal definition to step 0, the accurate statement is "one candidate definition is rejected", not "the frozen statement is false".

**Repair.** Restate Result 2 as: for any definition under which the verdict depends on a deviator's response to inputs it did not receive, the verdict is not a function of any log of the run. Cite the Künnemann definition as one under which the refuter does not fire.

### M5. Result 3 is ill-posed for a controller that deviated

**Text.** "U2's witnesses already hold U2's log and can replay its controller."

**Gap.** Replay in accountable systems checks recorded behaviour against a reference program, and a faulty node is exactly one whose replay diverges ([Accountable Virtual Machines](https://www.usenix.org/conference/osdi10/accountable-virtual-machines); [PeerReview](https://haeberlen.cis.upenn.edu/papers/peerreview-tr.pdf)). If U2's controller left its certified program, the witnesses have nothing to replay: the reference gives what a compliant U2 would have done, which is not the question. If U2's controller followed its certified rules and those rules breach the contract, replay is defined. The conjecture covers only the second kind and does not say so.

**Further points.**
- The third open bullet (false answers in the unit's favour) is closed by the summary rule whenever replay is deterministic: f+1 signatures include one honest replay.
- A counterfactual output of U2 changes what other units send back, so queries are adaptive rounds across units, not single local answers.
- A counterfactual query oracle reveals U2's decision rule on inputs of the asker's choice. That discloses more than the inner log the plan set out not to read.
- Not new in its parts: program-level counterfactuals for accountability ([Datta et al.](https://arxiv.org/abs/1505.01131)); replay against a reference event for diagnosis ([Chen et al.](https://web.eecs.umich.edu/~chenang/papers/sigcomm-2016.pdf)). I found no source for the combined statement with a witness quorum (**could not verify** novelty).

**Repair.** State the class (rule-following controllers, deterministic given recorded inputs), the counterfactual (one input changed, all other recorded inputs fixed, link delays fixed), and the query model (adaptive, with a bound on rounds). The first conjecture to test is whether the number of queries is polynomial in the number of broken guarantees on a chain.

### M6. D1/D2: satisfiability is defined under all assumptions, blame under each

**Text.** "one way of acting that keeps every guarantee whenever all the assumptions hold", against "the unit broke a guarantee whose assumption held".

**Counterexample.** N1: buy compute costing 150 by day 10, assuming a permitted vendor can deliver. N2: report revenue by day 5, assuming revenue of 50 arrives by day 4. The budget is 100. With both assumptions the set is satisfiable. The revenue does not arrive. N1's own assumption still holds, N1 is impossible, and the unit is recorded as a deviator.

**Repair.** Require a way of acting that keeps each guarantee whenever its own assumption holds, which is the conjunction of contracts in the contract literature ([Benveniste et al.](https://nowpublishers.com/article/DownloadSummary/EDA-053)‡ for the formula), or require each assumption to name every resource its guarantee needs.

### M7. D1: "once, when a norm takes effect" leaves three events out

- *Repeal or expiry.* With three obligations of which two can be met, the lowest is left out. If the highest is repealed a day later, nothing says the third returns. The selection has to be recomputed at every change of the norm set.
- *Commitments.* D5 keeps commitments out of the selection, and D6 refuses a commitment for which no rights can be reserved. A commitment can also take the unit out of the states from which its norms stay satisfiable. So a check runs at every commitment, and "paid when a norm is issued, not on every decision" understates the cost.
- *The state used at issue.* On day 9 an authority forbids vendor V. The unit had lawfully planned to order from V on day 9. Evaluated from the current state, the older obligation conflicts and, if lower, is dropped, which is the escape by waiting moved to issue time. Evaluated from the obligation's original state, there is no conflict and the unit is blamed for a miss the authority caused. The document does not say which.

**Repair.** Evaluate at every change of the norm set, from the current state. Treat a later prohibition that removes the last compliant plan as a rule change under D5, with the loss on the issuing authority.

### M8. D5: a norm void "from the time of the finding" and verdicts given before it

**Text.** "removed from the time of the finding, and the losses in between fall on that authority as a deviation."

**Counterexample.** Day 1: the ministry issues N, which its charter forbids. Day 3: unit U declines to perform N. Day 4: verdict, U deviated. Day 10: N is found to breach the charter. Either the day-4 verdict stands and names a unit for breaking a void norm, or it is revised and verdicts are not final, against D9. The authority's "deviation" also has no guarantee entry in any summary, so Result 1's definition does not produce it.

**Repair.** Make charter conformance a guarantee of every authority, logged at issue. State that a finding voids the norm from its issue for the purpose of verdicts and from the finding for effects, and that this is the one declared case of verdict revision.

### M9. D8: one head per sequence number is not yet one log

**Text.** "signs only if it has seen no different head from the controller with the same sequence number."

**Gap.** Nothing requires head n+1 to extend the head the witness signed at n. A controller sends A₇ to all, obtains signatures, then sends C₈ whose predecessor is an unsent B₇. C₈ is the only head at 8 and is certified. The certified chain omits A₇'s entries without any equivocation at one sequence number.

**Repair.** Heads carry the hash of their predecessor. A witness signs head n+1 only if it signed that predecessor and holds the entries between. Add ε ≥ ρΔ for clock drift ρ.

### M10. D6 and D11 state different invariants

**Text.** D6: "the cap is never exceeded within a version." D11, first case: cap 95 in force, spending up to 100.

**Shown wrong** by the document's own table. D11 has the true statement. Replace D6's sentence with it.

### M11. The R3 conclusion drops half the target and assumes the hard part

**Text.** "'complies with every surviving norm on every edge' is the same as 'complies with the effective set'".

**Assessment.** The conjunction step is correct and trivial. Three things are left out.
- R3's second half, decidability of minimal conflict sets under D2's reading, is not mentioned. Under D1/D2 it is the existence of a winning strategy in a timed game, which is EXPTIME-complete for timed automata without quantities ([Benattar et al.](https://arxiv.org/pdf/1207.4984), restating the classical result). "A scheduling question" understates it. I **could not verify** the status with consumable quantities.
- The edge check is local only because it is handed "one certified effective set". Who computes and certifies that set, at what size, is the content of R3's "description of the selection within the frozen size".
- H7 shows that translation and priority interact. This is a second open question beside context export.

**Repair.** Report R3 as: the conjunction step is trivial; three items are open (selection certificate size, translation priority, context export); the decidability half is untouched and made harder by D1/D2.

### M12. D12: the limit argument needs bounds that do not depend on pool size

**Text.** "a property that holds on every finite prefix in every sufficiently large pool holds in the limit."

**Assessment.** Correct for a fixed property. Two hypotheses are missing. (a) The bound in D9 and the loads in D3 and D8 must not grow with the number of live units; D3 says the anchor's table "is read by every selection", which suggests they do. A bound B(n) per pool gives no bound in the limit. (b) "A run with n creations" must read "at most n creation requests". A pool of size n receiving request n+1 is a legitimate run in which the required response fails, which is the plan's own known problem for D12. Also exclude unboundedly many creations in bounded time.

---

## LOW

- **L1. D8.** "every honest witness holds that proof within 2Δ" holds only if both heads reach an honest witness; a head held only by faulty witnesses proves nothing until released. Safety against forks rests entirely on the bound Δ, which no party can observe. That is the objection the plan made to "final when every link met its bound", and it should be stated as applying here.
- **L2. D11.** No arithmetic error found. The invariant as worded is weak: spent amounts stay counted for ever, so "the largest cap under which any still-counted right was issued" is the largest cap ever in force at an issue. The sharper true statement is that the total is at most the cap in force at the most recent issue. Period caps and late reports across a period boundary are not addressed.
- **L3. Well-foundedness.** "Each step moves to an earlier deadline" ends the search because contracts are finite and acyclic, not because deadlines decrease. "Earlier" has to mean earlier by at least the link allowance plus Δ plus skew. The 3-day guarantee has no fixed deadline, so the condition as stated does not cover it.
- **L4. Weaker-reading caveat.** Under the Künnemann definition the verdict is not "who broke a guarantee" but the minimal sets able to produce the violation alone. A unit that broke an irrelevant guarantee is excluded.
- **L5. D3/D4.** Which of two norms of one authority prevails is not stated, only that they are ordered. Who names the path of a norm with no direct edge is not stated, nor what happens when an edge on that path is revoked. The origin should name it, and revocation should force a new path before it takes effect.
- **L6. R4.** The conclusion is right and could be more exact. Changing a bound safely at once in one direction and only after settlement in the other is the demarcation protocol's rule for moving limits ([Barbará & Garcia-Molina](https://www.vldb.org/journal/VLDBJ3/P325.pdf), "if a node wishes to perform an unsafe operation, it requests that the other node perform a corresponding safe operation and waits"), alongside escrow ([O'Neil](https://dl.acm.org/doi/10.1145/7239.7265)‡).

---

## Verdicts

**Result 1 (blame by broken guarantee composes from summaries).** True in substance and reuse, as the author says. False as stated: "exactly one" fails on the document's own Result 2 history (H3); nobody is named when an assumption has no guarantor (M1); a compliant unit can be named within the fault bound unless margins are added (M2); and the definition cannot be applied to the benchmark's vendor assumption (H4). Every repair is a tightening. None needs inner logs.

**Result 2 (blame by actual difference does not compose).** The table is wrong under the stated one-day link (H2). A repaired pair of histories exists, so the conditional claim stands for definitions that model the deviator's mechanism. It does not stand for the definition the plan cites, under which the verdict is a function of the trace (M4). The caveat in "Where I am least sure" is right in direction and too narrow.

**Result 3 (conjecture).** Neither known nor obviously false. Ill-posed as written: undefined for a controller that left its program, silent on what is held fixed across units, and blind to the disclosure it creates (M5). Its parts are published.

**The decision set.** A consistent set is not shown. As written, D1 and D6 contradict each other (H5), D3 rejects benchmark edges and contradicts D4 (H6), D4 permits priority laundering (H7), D1/D7 do not fit Result 1's definition (H4), and D8's consequences are overstated (H1). I found no evidence that a consistent set cannot exist, and a repair is given for each. D9, D11, D12 and D13 stand with minor tightening.

**Conclusions for the plan.** "Mostly an assembly of known results" is fair; D8 in particular is a published voting rule. "R4 is the escrow method" is right. "R3 reduces to a conjunction" is overstated (M11, H7). The claim that the R2 stop rule "does not fire by its letter" depends on calling a change of definition a repair (H3, M4). The undersold item is the observation behind Result 2, that mechanism-level blame needs the controller's program and not any log.

CRITICAL: 0  HIGH: 7  MEDIUM: 12  LOW: 6

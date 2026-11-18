# Adversarial review of "Step 0, group A: norms, priority, commitments and rule change"

Reviewer: fable. Target: [Step 0 group A norms.md](/root/mic/reports/Step 0 group A norms.md). Context read: [Step 0 worked on paper.md](/root/mic/reports/Step 0 worked on paper.md), its two reviews, the decisions table, benchmark and targets of [Integrated foundation plan.md](/root/mic/reports/Integrated foundation plan.md), and the sibling group B document for cross-references.

**How I applied the scale.** Several stated results are false as literally written (soundness of the test, uniqueness, R3, the first clause of R4). Where a restated result survives after a one-line tightening I rate MEDIUM; where it survives only with a new rule or assumption I rate HIGH; CRITICAL is kept for a result that is false and whose central argument does not survive. Each finding is marked **shown wrong** (explicit case under the document's own rules), **could not verify**, or **judgement**. Nothing in the document's "What this costs" list is counted as a defect. Items from "Where I am least sure" are counted only where I show an actual failure.

**What is right.** The rank rule does prevent cycles while ranks are fixed. Lex posterior within one origin behaves correctly for a prohibition against an obligation in both orders. The author's first doubt (several ways to act, different prohibitions blocking different ways) is well defined: the lowest-priority prohibition is the one that yields (see H2 for the case that is not). The complexity remark on patterns is correct: with one-attribute prohibitions the surviving set is a product of per-attribute differences, and the general problem is the complement of covering a product by boxes. Cases 5, 10, 12 and 14 follow from the rules as stated (14 for fixed ranks only). The cost list is honest.

---

## CRITICAL

### C1. R5, third clause, is false: a unit can leave its authorities' norms without touching an anchor term, and "entrenchment needs no check at all" does not hold

**Text attacked.** "No amendment a unit makes to its own rules can ... take it outside the norms of its authorities." "They are phrased only in terms that belong to the anchor. A unit's rules cannot define or redefine those terms." "So there is no check to be sound or complete: an amendment cannot express a breach." Result 3: "Entrenchment needs no check at all."

**Gap in the argument.** The name-space argument covers only the three anchor duties (log, replay, run the selection). The clause about authorities' norms rests on one line: own rules are lowest priority, so a *conflicting norm* loses. That handles an amendment that is a norm conflicting with an authority's norm. It does not handle an amendment that changes what an authority's norm ranges over, and the plan's refuter for R5 names exactly this ("sound, including against amendments that change the meaning of a term").

**Counterexample 1: the mapping reads unit state (shown wrong).** D4: "An authority edge carries a mapping from the lower unit's effects *and state* to the upper unit's vocabulary." Patterns are over "counterparty class, resource, purpose".
- Ministry norm P: prohibition, kind = order, counterparty class in {unapproved}.
- The class of vendor X is a field of the unit's vendor register (unit state); the purpose of a payment is a label the unit's rules attach.
- Amendment: the unit's rule "vendor X is filed under class approved", or "purchases of kind K carry purpose = research".
- The amendment is not a prohibition, permission, obligation or cap that conflicts with P, so the selection excepts nothing and leaves nothing out. It writes no anchor term. After it, the order to X has a derivation, P is "conformed to", and the unit is doing what P was issued to stop.

Purpose cannot be certified by anyone outside the unit, so this is not removed by saying "attributes come from a certified registry". Benchmark variant 6 (purchases split below the approval threshold, named differently by team and audit body) is the same attack on an aggregate.

**Counterexample 2: a created unit (the author's fourth doubt; shown wrong, not merely open).** Under the document's rules a norm reaches a unit only along an edge from its origin or a route its origin names. A new unit V created by U has no edges except any U declares.
1. U amends its own rules: permission "create a unit", permission "transfer budget to a unit I created". Own permissions are norms ("origin: an authority, or the unit itself") and "a permission ... is added" unconditionally.
2. U creates V and transfers 50 to V. The cap on U's resource is respected (50 is spent, within cap). P does not match: the counterparty is V.
3. No authority of U has an edge to V or has named a route to it; none knows V exists. V's effective set is empty of P. V orders from X.

Every effect of U and V is logged and replayable, and the outcome is exactly what P forbids. The same works with an existing colluding counterparty paid under a commitment.

**Counterexample 3: the unit boundary (judgement plus one case).** "Every effect is protected, so there is no narrower class to redefine." "Effect" is "what a unit does to the outside", so the class depends on the unit's boundary and declared interface. If U's rules give a role to a principal who also holds a role in vendor X, value and information move through that identity; the plan states this channel is not mediated ("What an identity carries between its own role instances in different units is not mediated"). Admitting members is an amendment. So a narrower class does exist: inside versus outside.

**Repair.** R5 needs at least three checks, so Result 3 is withdrawn:
(a) every attribute a pattern or mapping reads is either computed by the anchor-supplied interpreter from the effect itself or certified by a party outside the unit; attributes that cannot be (purpose) may be used only in norms whose breach is detected after the fact, and are excluded from the "prevented" claim;
(b) a creation rule: a created unit inherits every authority edge of its creator with the composed mapping, and caps and aggregates on the creator count the creator's whole creation subtree;
(c) the unit's interface and membership boundary are anchor terms, or changes to them are effects.
With these the statement becomes "no amendment changes the value of a certified attribute or removes an inherited edge", which is a check, has to be shown sound, and blocks some useful amendments.

---

## HIGH

### H1. The selection, as written, tests obligations before the permissions and caps they depend on, and never re-tests; cases 1, 4 and 9 do not come out, and R4's first clause is false

**Text attacked.** "Take the norms in priority order. For each: a permission or cap is added; ... an obligation is added if the test still passes. If it fails only because of caps whose origins it outranks, those caps are raised." R4: "At every change of version the feasibility test holds for the new effective set together with everything reserved ... The first is how the selection is defined."

**Case (i): permission below the obligation (shown wrong).** Regulator (high) obliges a report. The permission to emit reports comes from the group or the unit (lower). In priority order the obligation is tested first; "some permission allows" fails because no permission is in the set yet; the obligation is left out for good. Case 4's required outcome ("the report is made") fails. Same origin, two orders: regulator issues "permit report" (seq 1) then "oblige report" (seq 2); later is first, so the obligation is tested before the permission and is left out. Issued in the other order it is admitted. The outcome depends on the order of two norms that do not conflict.

**Case (ii): the cap-raise rule can never fire (shown wrong).** An obligation that outranks a cap's origin is processed before that cap. At that moment the cap is not in the set, so the obligation cannot "fail because of" it. Then "a ... cap is added" with no test. Case 9: obligation 20 from the higher authority passes against no cap; the ministry's cap 100 is added; the set now holds 100 used + 20 reserved against cap 100. The loop ends with a set that fails the test, and no raise is recorded.

**Case (iii): a lowered cap (shown wrong).** Cap 100, commitments admitted and reserved 90, no obligations. The ministry replaces the cap by 50. Re-run: the cap is added. The new effective set with everything reserved fails part two of the test (90 > 50). R4's first clause is false with no obligation in play, and it contradicts R4's own "Standing" note, which admits use above a lowered cap.

**Fork if "permissions first" is chosen instead.** Then a lower permission supports a higher obligation, and withdrawing it removes the obligation: the group, or the unit itself by amending its own rules, withdraws "permit report"; the selection re-runs (a norm was withdrawn); the regulator's obligation "has no allowed effect and is left out"; no finding. That is a second counterexample to R5's third clause and to priority.

**Repair.** (1) An obligation carries the permission for its own pattern at its own priority (the D axiom; standard in the deontic literature). (2) Symmetric cap rule: "a cap is added, raised by the least amount that admits the reservations of obligations already in the set whose route priority exceeds the cap's". (3) Restate R4's first clause as: the test holds for the new set with grandfathered amounts carried as an explicit, non-increasing term (which is what D11 already says). (4) Run the full test once more on the final set as a check.

### H2. The exception rule is not defined when one prohibition blocks every way to act, and its scope is not stated; each completion breaks a stated result

**Text attacked.** "a prohibition is added, with an exception for any effect that an obligation already added requires and has no other way to perform". Case 4: "the prohibition stands for everything else". "the result is unique."

**Case.** O (regulator, high): disclose the purchase summary to a counterparty in class {r1, r2}. P (ministry, lower): never disclose purchase summaries. P blocks both ways.
- *Reading A, the whole pattern is excepted, standing.* The unit may disclose to r1 and r2, any number of times, on anyone's request, since a derivation exists (permitted, not prohibited). Case 4's "stands for everything else" is false. A lower obligation O′ (the unit's own, lowest priority) with the same pattern is then admitted against P, because "no prohibition in the set forbids" it: a lowest-priority norm rides a high-priority exception through a mid-priority prohibition.
- *Reading B, one effect is excepted.* Which one is not determined. Uniqueness fails.
- *Reading C, nothing is excepted,* since each effect has "another way" (the other one). Then O is in the set with no allowed effect: the loop passed and the result is unsatisfiable.

**Evidence.** Shown wrong under each reading. The author's own doubt covers different prohibitions blocking different ways; that case is fine. This one is not.

**Repair.** An exception is attached to the obligation instance, not the pattern: it authorises one discharging emission, chosen by a fixed order on attribute values (or left to the unit, with the choice logged before emission), and lapses on discharge. A lower obligation cannot use it.

### H3. The feasibility test is sound only under five assumptions the document does not state; one contradicts D4

**Text attacked.** "It is sound: a set that passes can always be honoured, by emitting each obligation's allowed effect from its own reservation."

I could not break the test inside the model "the only state is a set of capped resource totals, each obligation fires once, time does not matter". That model is not the document's. **Shown wrong** on each of:

1. **No cap, or capacity that is not a cap.** Part two compares with "the lowest cap in the set". A cap is a norm set by "an authority over the resource's owner". Two obligations each needing the one machine slot, no cap norm on the slot: the test passes and one cannot be honoured. Case 11 says "with the slot declared as a resource of capacity one", but capacity is not one of the four forms. Likewise cap 100 with 50 actually in the account: two obligations of 40 pass. The cap raise in case 9 raises a limit; it does not create funds.
2. **Repeated triggers.** "After a named certified event ... It declares the most of each resource it can need." The benchmark's duties recur ("report summaries of ministry-funded purchases"; pay each invoice). If the declared need is per firing, cap 100, O1 "after each certified invoice, pay up to 40", O2 reserves 60: passes; the second invoice cannot be paid. If the need is over all firings, the obligation must bound the number of firings or declare an unbounded need and be refused. Nothing says when a reservation is released if the trigger never occurs.
3. **Lease lapse.** D4: "When the lease has run out, the lower unit refuses the effects that depend on it." Deciding that no routed prohibition forbids an effect needs the intermediate's context, so every effect depends on it. Regulator (direct, high) obliges a report by day 8; the ministry (low intermediate on a route for the state's norm) lets its lease lapse on day 6. The unit is required by D4 to refuse and required by O to emit. The set passed the test and cannot be honoured. See H6 for who is blamed.
4. **Cost above the declared maximum.** The issuer declares the need. Authority declares 0 for an obligation whose only allowed effect costs 30 with the cap full. The test passes; the effect is refused by the ledger; the miss is "with its assumption intact"; finding against the unit. Under-declaration moves blame from issuer to unit.
5. **Time.** An obligation "by day 8" issued on day 9, or with a window shorter than the witness round trip, has an allowed pattern and passes.

**Repair.** State soundness as: every obligation fires at most a declared number of times; every interference between effects is through a resource with a certified capacity, and the test uses the lesser of cap and capacity; a cost above the declared maximum and a lapsed lease are certified assumption-breaking events with the loss on the declarer and the intermediate respectively; deadlines are at least a stated margin after effect. Item 2 needs a new rule (per-period replenishment or a firing bound), which is why this is HIGH.

**On cost (already listed by the author, not counted; this only sizes it).** The test is the trivial sufficient condition "sum of maximum claims fits". That is strictly more conservative than the banker's algorithm, which admits over-subscribed claims whenever a safe completion order exists, and which is itself known to be unusable where claims are unknown or inflated ([lecture notes on the algorithm](https://cs.nyu.edu/~gottlieb/courses/2000s/2008-09-spring/os202/lectures/lecture-14.html)). With no time index on a resource, a machine with one slot a day is either 365 resources or admits one obligation a year. With budget 100 and conditional obligations of 40 each, the unit can hold two, whatever their triggers. The plan's "a controller that rejects everything inside the envelope fails" is at real risk; the author's fifth doubt is justified.

### H4. Commitments have no priority and therefore the highest: first-come reservations defeat later norms, and the "issuer pays" rule turns that into a payment to the unit

**Text attacked.** "A commitment ... is not a norm and has no priority." "Its needs are then reserved." "If it does [apply], each affected commitment is frustrated, and the loss falls on the authority that issued the norm."

**History (shown wrong).** Cap 100 set by the ministry. The unit creates V (or uses a friendly counterparty) and commits 100 to it. Admission passes: no obligation yet. The ministry now issues an obligation needing 20.
- Flag off: the test counts "what is already ... committed and reserved" first; 120 > 100; the cap's origin is the ministry itself, which the obligation does not outrank; the obligation is left out, on the record, with no finding. The unit has made itself immune to every later obligation from any authority not above the cap-setter.
- Flag on: commitments are frustrated at the ministry's cost. The loss is owed to V, which the unit controls. The ministry pays the unit for obeying.

Either way the unit wins. The author's doubt was abuse by the authority; the sharper abuse is by the unit.

**Three further gaps.**
- *Which commitments are "affected"* by a resource norm? An obligation needing 20 against ten commitments of 10 each can be admitted by frustrating any two. No rule picks them: non-unique.
- *The issuer's liability is unreserved.* The ministry frustrates commitments worth 1000 while its own cap is 100. Either issuing a norm must be refused on facts about lower units' ledgers, or the ministry's total use exceeds its cap, against R4's third clause applied to the ministry.
- *Self-issue.* "Each norm states whether it applies to commitments already admitted" includes the unit's own norms. The unit issues its own prohibition on paying class C, flag on. Its commitment to a C counterparty is "frustrated" and recorded as an allocated loss. In the plan's verdict types an allocated loss is "where nobody deviated", so the unit has turned a breach of its own promise into a no-fault entry.
- *The author's doubt as stated:* a parent authority M cancels its unit U's debt to W by a flagged prohibition. If the loss is paid to W in full it is harmless. The document does not say to whom, how much, or from which reserved resource; with the loss only "recorded" it is a debt cancellation.

**Repair.** A commitment needs an admission rule that looks at more than the test: commitments to units in the committer's creation subtree or under common control do not reserve against authority norms; the flag is not available to a unit's own norms (a self-frustrated commitment is a finding); frustration picks commitments latest-admitted-first; the loss is a commitment of the issuer, admitted against the issuer's own test before the norm takes effect, and payable to the counterparty. That last rule makes norm issue depend on lower units' ledgers, which is a real cost and should be listed.

### H5. The plan's escape by waiting survives as escape by certified refusal, and silence blames a compliant unit

**Text attacked.** "with its assumption shown broken by a certified event, nobody has a finding and the loss is allocated." Case 2. "The alternative ... would let a unit that can arrange a refusal obtain an exception."

**History A (shown wrong).** Obligation: order from a permitted vendor by day 8. The only permitted vendor publishes that it takes orders until day 5. The unit does nothing until day 6, sends the order, and receives a certified refusal. Rule: missed, assumption shown broken, no finding. This is D1's required case ("a unit that waits until only a forbidden vendor can deliver"), which the earlier document said must be a deviation ("a unit that lets its options run out has deviated"). Case 1 tests only the unit that never orders. Variants: send a malformed order; ask a price the vendor must refuse; collude with the gateway. The document refuses the unit an *exception* for an arranged refusal and grants it an *acquittal* for the same act.

**History B (shown wrong).** The unit orders on day 1. The gateway never answers; links to vendors have no delivery bound. No certified refusal exists, so the assumption counts as intact, and "the unit has a finding". The plan requires that no proved deviation names a compliant party.

**Also unclear.** If the obliged effect is the emission of an order, it is always in the unit's power and the gateway's refusal is irrelevant to discharge (case 2 then needs no assumption). If it is the accepted order, History B applies. The document uses both.

**Repair.** An obligation states an act window that ends before the deadline by a margin; the finding is "did not emit a well-formed order within the act window". A refusal acquits only if it answers an order emitted inside that window. Emission without an answer by the deadline is neither a finding nor acquittal but the "unestablished/suspicion" outcome that group B must carry. Whether a refusal was arranged cannot be decided from the log; say so.

### H6. A change in an intermediate's exported context, or a lapsed lease, changes what binds the unit without a selection, a flag or a bearer; R4's second clause fails, and a low intermediate can block a high obligation

**Text attacked.** "It does not run ... because the environment has changed." "When the lease has run out, the lower unit refuses the effects that depend on it." R4: "Every commitment admitted earlier is honoured as an exception or recorded as frustrated with a bearer."

**History (shown wrong).** State norm N, routed through the ministry: "order only from vendors on the ministry's approved list". The list is ministry state, exported under a lease. The unit has an admitted commitment to pay vendor X, on the list. The ministry removes X from the list. No norm was issued or withdrawn and no route or mapping changed, so the selection does not run and no norm carries an "applies to existing commitments" flag. The payment to X now matches the prohibition and is refused. The commitment is neither honoured as an exception nor recorded as frustrated with a bearer. The ministry has issued a prohibition at the state's wording, with no priority contest, no flag and no cost. If the unit also has an obligation whose only allowed vendor was X, the effective set is now unsatisfiable without the loop having run.

The same with no change of value: the ministry simply stops renewing the lease. Every effect of the unit depends on it (H3 item 3), so every obligation from every authority, including those ranked above the ministry with direct edges, cannot be discharged. Rank says the ministry yields to the regulator; availability says the ministry can stop the regulator's obligations at will. The miss is classified "with its assumption intact" unless lapse is declared an assumption-breaking event, which the document does not do.

**Repair.** A change to an exported value that changes which effects a routed norm matches is a norm change by the exporter: it triggers the selection, carries the flag, and is ranked at the exporter's priority. On lapse, only norms routed through the lapsed intermediate are suspended, and they are suspended as not binding (fail open) for effects required by obligations of higher priority than the intermediate, with the lapse certified and the loss on the intermediate. Fail-closed for everything is the availability inversion.

### H7. The rule language cannot state an amendment procedure or a charter, so R5's "every amendment is accepted" is either false or empty, and benchmark variants 9 and 10 cannot be written

**Text attacked.** "Every amendment a unit makes to its own rules is accepted." "This includes amendments to the amendment procedure." D5: "A norm found to breach its origin's charter".

**Gap.** The four forms regulate effects on the outside. None says who may change a rule, by what vote or approval, or which norms an authority may issue. The plan requires "An amendment is valid when it passes the unit's own charter procedure" and "version n+1 is checked under version n"; variant 10 has the subsidiary "validly amend its own amendment rule"; variant 9 has "an authority that issues a norm its own charter forbids". There is no amendment rule to amend and no charter to breach in this language. Institutional action languages have a separate construct for exactly this (institutional power, events that count as others; see prior art).

**Fork (shown wrong on each branch).**
- *Amendment is an effect* (it must be logged to be replayed). Then a prohibition can match kind = amend. An authority issues "never amend rule class K"; the unit's amendment has no derivation and is refused: "every amendment is accepted" is false. The unit issues its own "never amend": the next amendment is refused by the rule in force, so the unit has locked itself for good, although lex posterior says the later rule would win if it could be issued.
- *Amendment is not an effect.* Then it is outside "every effect is protected", and nothing in the document logs it or says who may make it. Any component that can reach the controller amends.

**The completeness half is empty (judgement).** The plan's refuter is "a preserving amendment in the frozen class that is rejected". The document relabels rejection as "accepted and left out". An own obligation that is refused by the conservative test (a cost the author admits) is a preserving amendment with no effect. No class of useful amendments is named.

**Repair.** Add a fifth form, a power: "an amendment/norm of class K by origin X takes effect only after certified event E". Charters are sets of powers. State R5's second sentence as: every amendment that passes the powers in force is installed, and the selection then decides what of it is effective. Name the frozen class of amendments that must come out effective.

---

## MEDIUM

### M1. The order is not total on routed norms, so uniqueness is false as stated

**Text.** "A norm arriving by a route of more than one edge takes the lowest priority among its origin and the intermediate authorities." "Because the order is total ... the result is unique."
**Case (shown wrong).** State obligation N (need 40) and supranational obligation S (need 40), both routed through the ministry to the group, plus the ministry's own obligation M (need 40); cap 100. All three have the ministry's priority. Sequence numbers are per origin and incomparable. Which is left out is undefined. Worse, if the tie goes by sequence or identifier, the ministry's own norm can beat the state's norm it carries.
**Also.** Since edges run from higher to lower rank, "lowest on the route" is always the last intermediate; the rule can be stated that way.
**Repair.** Order by (route priority, origin priority, sequence number), lexicographic.

### M2. R3 is false for obligations as worded, is not the plan's R3, and the confirmation claim is undersized

**Text.** "If a unit's controller emits only effects that have a derivation ... the unit's behaviour conforms to every norm in that set." "linear in the number of norms reaching it."
**Shown wrong.** A unit that emits nothing satisfies the premise and misses every obligation. The argument itself says obligations are "not enforced". Restate: conforms to every prohibition and cap as excepted; every obligation miss is recorded and classified.
**Moved, not fixed.** The plan's R3 is a *local check at each edge using a bounded description of the selection*, plus detection of minimal conflict sets. The document's answer is "all of the record" and no conflict detection. Both earlier reviews said this (astra 6, fable M11); the plan itself excludes "uniqueness of a greedy selection under a total order" as a result. R3 here is a definition.
**Undersized (judgement).** To confirm, an authority needs every other authority's norms with proof of authenticity and validity, each route, each lease value at each time, the anchor's table at each version, and the ledger state at each re-run; and the selection re-runs at every change. That is norms times versions, plus ledger history, across vocabularies the checking authority does not own.

### M3. The routing rule closes tightening and leaves loosening open

**Text.** Result 2: "This closes the hole where an intermediate authority passes off its own tightening as its superior's rule." D4: the mapping is declared "by the authority at the upper end".
**Case (shown wrong as a claim of closure).** State norm N routed through the ministry. The edge ministry-to-group has a mapping declared by the ministry. The ministry declares a mapping under which no group effect falls in N's forbidden class. N binds nothing. No priority rule is engaged. The "state must bind directly" cost covers priority, not nullification.
**Judgement.** With direct binding as the only safe course, and rank the only condition on an edge (M4), every ancestor binds every descendant directly and writes its own mapping per class. The multi-level problem the plan set for R3 is not solved; it is avoided, and the intermediate route has no remaining use except for origins that do not care.
**Earlier finding fable H7** (tightening by translation) is fixed by taking its repair (a), at the cost it predicted. Its coarse-vocabulary point remains: on a direct edge the state's own mapping must tighten, now at the state's priority, which is acceptable.
**Repair.** A routed norm's mapping composition is signed by the origin, or the origin audits it against a finding for the intermediate.

### M4. Rank is the only condition on an authority edge; ranks can be minted; rank change breaks case 14

**Text.** "An authority edge may run only from a higher rank to a lower one. A declarer checks this against the target's certified rank." "A created unit gets a rank strictly below its creator's."
**Shown wrong / gap.**
- Nothing says who is entitled to declare. A gateway unit, a foreign group or the regulator may declare itself an authority over any unit ranked below it and issue obligations to buy. The benchmark states the regulator has no edge to the ministry; the rules permit one if the ranks allow.
- Who picks the created unit's rank is not stated. If the creator does, a high-ranked unit mints shells just below itself, each outranking every unrelated unit below the creator.
- Case 14 holds "however the declarations interleave" only for fixed ranks. A declares A-to-B on a certificate rank(B) = 3 < 5; the anchor concurrently re-ranks B to 7; B declares B-to-A on a fresh certificate. Both pass. Rank change is also missing from the list in "When it runs".
- A created unit can never be an authority over its creator or anything ranked above it: no supervisory board created by charter. The plan states this limit, so it is not counted.
**Repair.** An edge needs a grant (from the anchor, or from an authority already over the target), and rank is a necessary check only. A created unit's rank is fixed by the creation rule. Rank certificates are leased; an edge is valid only while both leases are.

### M5. Permission against prohibition ignores priority; default-deny is vacuous or undefined

**Text.** "Without a permission an effect has no derivation." Case 6: "N and R hold; N′ gives way."
**Shown wrong for one instance of case 6.** Let R be the regulator's permission (emergency purchases under 500 may be made) and N′ the ministry's prohibition. Prohibitions get exceptions only for obligations. N′ stands and R is dead, though the regulator outranks the ministry. The required outcome fails when the higher norm is a permission. The priority of a permission never matters anywhere in the selection (except through H1).
**Vacuity.** The unit's own rules may be permissions. "Permit everything" as an own rule makes default-deny constrain nothing. If own permissions are disallowed, the document does not say who permits ordinary business.
**Repair.** Either say plainly that permissions are weak (forbid always wins, as in Cedar and XACML deny-overrides) and that case 6 is required only for obligations, or add a strong permission that excepts lower prohibitions.

### M6. "Owed until ... the set changes", and what a re-run does to an obligation in progress

**Text.** "An obligation in the effective set is owed until it is discharged or the set changes." "from the moment the norm takes effect".
**Shown wrong on the literal text.** The unit issues a trivial own permission every four days. Each is "a norm issued", the set changes, and an obligation "within five days from the moment the norm takes effect" either stops being owed or restarts its clock. A unit sheds a relative deadline by amending.
**Gap.** A higher obligation issued later displaces a lower one already half spent; the document allocates loss for frustrated commitments only, not for a displaced obligation's beneficiary.
**Gap (fable M7, third bullet, moved).** The test is over patterns, not the current state. An authority forbids vendor V on day 7, when every other permitted vendor's window has closed. The obligation still has an allowed pattern, stays in the set, and the unit is found against unless it collects a certified refusal from each remaining vendor by day 8.
**Repair.** An obligation keeps its identity and clock across versions while its norm stays in the set. A displaced obligation is recorded with the displacing norm's issuer as bearer.

### M7. A norm void "from its issue for the purpose of verdicts" convicts a unit that followed the recorded selection

**Text.** "A norm found to breach its origin's charter is void from its issue for the purpose of verdicts and from the finding for effects."
**Case (shown wrong against R3).** Invalid high obligation H displaced the ministry's obligation O (resource conflict). The unit performed H and not O, exactly as the certified selection said. H is later void from issue for verdicts, so O was owed throughout and was missed with its assumption intact: a finding against a unit whose every act had a derivation. The reverse also holds: a unit that broke H is acquitted, so a unit is rewarded for gambling on invalidity. (Earlier finding fable M8 is restated here, not answered.)
**Repair.** Reliance on the certified selection is a defence; the void norm's issuer bears the losses of what it displaced. This retrospective treatment matches annulment as against abrogation in the legal-change literature (prior art).

### M8. A cap is raised by an authority that has no authority over the resource's owner, and one resource appears in several units' selections

**Text.** D6: "Only an authority over the resource's owner can set one." Selection: "those caps are raised."
**Case (shown wrong as a contradiction).** The budget is owned by the group; the ministry caps it. The regulator has an edge to the subsidiary only. It obliges the subsidiary to spend 20. The cap is raised in the subsidiary's selection by an authority with no edge to the owner. Raising is setting.
**Gap.** The selection is per unit; the ledger is the owner's. Two units drawing on one budget each run a selection; whose obligation gets the last 40 depends on which norm change happened first, not on priority.
**Repair.** Resource norms are selected once, at the owner, over the obligations of every unit that draws on the resource. An obligation raises a cap only if its origin is an authority over the owner.

### M9. The cap raise is a ratchet (the author's second doubt, answered)

**Case (shown wrong against case 9's "cannot be taken by anything else").** Cap 100 fully used; higher obligation with maximum 20; cap raised to 120 "as a change of version". The purchase costs 5 and is discharged; 15 is released. If the raise stands, the unit has 15 of discretionary room above the ministry's cap. If the raise is recomputed at the next re-run, it depends on what has settled by then, so the cap in force varies with the timing of unrelated norm changes. A later lower obligation is correctly left out in both readings; that part of the doubt is unfounded.
**Repair.** The raise is a term tied to the reservation: it equals the amount reserved or spent under that obligation and nothing else may draw on it.

### M10. Under-specified loss allocation makes cases 2 and 12 unverifiable

"The loss is allocated" (case 2) names no bearer, amount or rule in this document, and group B's text does not supply one for a refused order. Case 12's outcome is checkable only as "a record exists". **Could not verify** that these two outcomes are produced by the rules, as opposed to asserted. Repair: one allocation rule per assumption, stated in the obligation.

---

## LOW

- **L1. Greedy anomalies (judgement).** Needs 60, 50, 40 in priority order against 100: the second is left out and the third admitted. The second issuer keeps its norm by splitting it into 40 and 10. The outcome depends on how an authority cuts its norms.
- **L2. Case 3 depends on the cap's rank.** If the three obligations outrank the cap's origin, the (repaired) rule raises the cap to 120 and none is left out. The case should say the cap is at or above the obligations.
- **L3. Rank domain.** "An ordered set with gaps" must be dense, or a creation chain deeper than the gap (project units creating units, variant 2) cannot be ranked.
- **L4. Result 2's wording.** "Takes the lowest priority among its origin and the intermediate authorities" always equals the last intermediate's; the origin is never the lowest.
- **L5. "Every effect is protected: it needs a recorded derivation ... and it is logged"** presupposes a correct controller; the plan's accountability results allow a deviating controller. R5's logging clause should say which assumption it is under.

---

## The fourteen cases

| | Do the rules give the stated outcome? | Is the required outcome sound? |
|---|---|---|
| 1 | Yes, if the permission is in the set when the obligation is tested (H1) | Yes |
| 2 | Yes for "no finding"; bearer of the loss unstated (M10) | Questionable: the same rule acquits a unit that waited for or arranged the refusal (H5); a silent gateway gives a finding against a compliant unit |
| 3 | Yes, only if the cap ranks at or above the obligations (L2) | Yes; greedy anomaly with unequal needs (L1) |
| 4 | No as written when the permission is lower or was issued earlier by the same origin (H1); undefined when the report has several recipients (H2) | Yes |
| 5 | Yes | Yes |
| 6 | Yes when R is an obligation and N′ a prohibition or obligation. No when R is a permission (M5). Inconsistent set when N′ is a cap (H1 ii) | Yes |
| 7 | Yes; tie with the ministry's own norms undefined (M1) | Questionable: the ministry can also nullify N by its mapping (M3) |
| 8 | Yes | Yes for tightening; loosening not covered |
| 9 | No as written: the cap is not in the set when the obligation is tested, and is then added untested (H1 ii). With the repair, yes, but the surplus leaks (M9) and the raiser may lack authority over the owner (M8) | Questionable: raising a limit does not provide funds (H3 item 1) |
| 10 | Yes | Yes |
| 11 | Only if capacity is a cap in the set; capacity is not one of the four forms (H3 item 1) | Yes |
| 12 | Yes as a record | Questionable: payable to a counterparty the unit controls (H4); issuer's liability unreserved |
| 13 | Yes | Questionable: a non-approval is silence, never a certified event, so the team is always blamed and the officer never; the plan's D7 says a controller cannot make a human act |
| 14 | Yes for fixed ranks | Yes; fails under concurrent rank change (M4) |

---

## Relation to prior art

Marked **verified** where a search confirmed the source at abstract level, **from memory** otherwise. I did not read full texts in this pass.

- **Permission and prohibition.** Default deny with forbid overriding permit, no priority among them, is Cedar's decision rule ([Cedar authorization](https://docs.cedarpolicy.com/auth/authorization.html), verified) and XACML's deny-overrides; processing in a fixed order is first-applicable / ordered-deny-overrides ([XACML 3.0 core](https://docs.oasis-open.org/xacml/3.0/xacml-3.0-core-spec-en.html), verified that these algorithms are defined there). Nothing here differs.
- **The selection.** Taking items in a total priority order and keeping each if consistency survives is Brewka's preferred subtheory for a linearly ordered base ([Brewka 1989](https://ijcai.org/proceedings/1989/167), verified) and the Greedy approach for prioritised norms ([Liao et al.](https://arxiv.org/abs/1709.08034), verified; [Hansen](https://link.springer.com/doi/10.1007/s10458-007-9016-7), the plan's citation). Maximal consistent sets of norms under a consistency constraint are constrained input/output logic ([Makinson and van der Torre 2001](https://dc2.philarchive.org/rec/MAKCFI-2), verified). The plan already rules out "uniqueness of a greedy selection under a total order" as a result.
- **The exception for an obliged effect.** Resolving an obligation/prohibition conflict by narrowing the prohibition's scope is "curtailment" ([Kollingbaum, Vasconcelos, García-Camino and Norman 2007](https://www.iiia.csic.es/media/filer_public/a4/3a/a43abd6c-4fe2-4316-8595-5607ac7626ce/iiia-2007-1635.pdf), verified that norms are curtailed by adding constraints; [Vasconcelos, Kollingbaum and Norman 2009](https://doi.org/10.1007/s10458-008-9070-9), verified that it formalises lex superior and lex posterior and conflicts from delegation; that the 2009 paper also uses curtailment is from memory). This is the closest single source and it treats the multi-value case H2 raises. Precedence relations between conflicting obligation and authorisation policies are older still ([Lupu and Sloman 1999](https://doi.org/10.1109/32.824414), verified).
- **Rank, then later-first.** Lex superior and lex posterior; defeasible deontic logic with a superiority relation gives the same with strong and weak permission distinguished, which M5 shows this language lacks (from memory: Governatori, Olivieri, Rotolo and Scannapieco, [arXiv 1212.0079](https://arxiv.org/abs/1212.0079); not verified in this pass).
- **Void from issue versus from the finding.** Annulment (ex tunc) versus abrogation (ex nunc), formalised in temporal defeasible logic ([Governatori and Rotolo 2010](https://dblp.org/rec/journals/igpl/GovernatoriR10), verified).
- **Reservation of a declared maximum.** Maximum claims are the banker's algorithm's input ([notes](https://cs.nyu.edu/~gottlieb/courses/2000s/2008-09-spring/os202/lectures/lecture-14.html), verified); reserving against an aggregate bound is the escrow method ([O'Neil 1986](https://doi.org/10.1145/7239.7265), verified). The test is weaker than the banker's safety check.
- **Powers, charters, amendment.** Institutional action languages separate institutional power and event generation from permission and obligation ([InstAL site](https://instsuite.github.io), [King et al.](https://arxiv.org/pdf/1704.06654), verified that InstAL has generation rules and models interacting institutions; the detail on power is from memory). Entrenchment, self-entrenchment and amendment of the amendment clause are treated by [Suber](https://legacy.earlham.edu/~peters/writing/psa/index.htm) (verified). The document's language has no counterpart (H7).

**What is different.** Three things, none of them a result yet: (1) reservation of an obligation's declared need at the moment the norm takes effect, joined to the selection, so that a deontic conflict over a consumable is settled once and stays settled; (2) the "lowest priority on the route" rule for translated norms; (3) loss allocation to the issuer of a retroactive norm. I found no source that combines greedy norm selection with an escrow ledger, but I did not search for one specifically (**could not verify** novelty). Each of the three is the subject of a HIGH finding above.

---

## Earlier findings: fixed or moved

| Earlier finding | Status here |
|---|---|
| astra 2, fable M6: satisfiability under each assumption | Replaced by a syntactic test; the question moves to the test's unstated assumptions (H3) |
| astra 3, fable H5: cap raise does not reserve | Fixed in intent; the written loop does not perform it (H1 ii); surplus leaks (M9) |
| astra 4: commitments need admission | Fixed; admission now over-protects commitments (H4) |
| fable H6: rank against creation depth | Fixed; entitlement and rank choice now open (M4) |
| fable H7: tightening by translation | Fixed by lowest-on-route; loosening open (M3) |
| astra 11, fable M7: when the selection runs | Mostly fixed; context value changes and rank changes missing (H6, M4); third bullet moved (M6) |
| fable M8: void norms and earlier verdicts | Restated, not answered (M7) |
| astra 6, fable M11: R3 drops the local check and conflict detection | Not fixed; stated as "standing: a design rule" (M2) |
| fable L5: who names the route, what revocation does | Route naming fixed; revocation of an edge on a route still unstated |

---

## Verdict

- **The feasibility test.** Not sound as claimed. Sound in a model with single-firing obligations, certified capacities for every interfering resource, costs within declarations and no lease dependence; the document's own D4 breaks the last. Its conservatism is worse than the banker's algorithm and is the likeliest reason the benchmark's useful-behaviour clause fails. HIGH.
- **The selection.** Deterministic once ties (M1) and the exception's scope (H2) are fixed, but as written it gives the wrong outcome in cases 4 and 9, can end with a set that fails its own test (H1), and lets first-come commitments outrank every later norm (H4). Uniqueness is false as stated and recoverable.
- **R3.** True only for prohibitions and caps, and then by definition. It is not the plan's R3: no local edge check, no conflict detection. Not a result.
- **R4.** First clause false (H1 iii). Second clause false for context changes and lease lapse (H6). Third clause could not be checked beyond the earlier reviews; it does not cover the issuer's frustration liability (H4). What survives is the escrow method with a grandfathered term, as the document says, and by the plan's own criterion that is not a result.
- **R5.** The logging and replay clauses hold under the stated dependence plus a fixed unit boundary. The clause about authorities' norms is false (C1), with a second route through permission withdrawal if H1 is repaired the obvious way. "Every amendment is accepted" is false if amendments are effects and empty if they are not (H7). The headline that entrenchment needs no check does not stand.

CRITICAL: 1  HIGH: 7  MEDIUM: 10  LOW: 5

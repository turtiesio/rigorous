# Adversarial review of the executable norm layer

Reviewed the unmodified `model/norms.lp` against **Step 0: an executable norm layer, first increment**, especially its “rules, in words.” The original test script passes (it actually prints 18 successes, counting its two issue-order variants). The companion attack script ran 36 scenarios; every one has exactly one answer set. Four independent implementation defect groups are CRITICAL because they change allowed effects or blame within the stated scope. Four additional MEDIUM findings are explicitly design objections, not claimed implementation violations. No HIGH or LOW findings are counted.

Reproduce from any directory:

```sh
PYTHONDONTWRITEBYTECODE=1 PYTHONPATH=/root/mic/vendor python3 /root/mic/model/attack_norms_astra.py
```

`--report` regenerates this report from the real solver results. The script asserts the decisive observed atoms, not the desired repaired behavior. “DEFECT” means an implementation contradiction; “as specified” includes questionable specified behavior and underspecified edge cases, explained individually. No source program or existing tests were changed. All displayed output below is the complete sorted **shown** atom set returned by clingo, not invented expected output; hidden internal predicates are not included.

## Confirmed defects, most damaging first

### F1 — CRITICAL: one obligation buys arbitrarily many simultaneous exceptions

Evidence: `simultaneous_exception` versus `sequential_exception_control` below. Two distinct reports at step 1 are both allowed and both excepted by one obligation, with only one discharge atom. Moving the second attempt to step 2 refuses it. The description says the obligation lasts “until one allowed effect meets it” and the exception ends on discharge; its examples promise one emission. Coarsening the timestamps defeats that guarantee, with arbitrarily many distinct matching effects.

Correct outcome: at most one report may use this single obligation's exception. The other must be refused absent another independent entitlement. The model need not pick which one without an externally supplied order, but it must not permit both.

Minimal repair: give attempts an explicit order within each step and evaluate `live`, `done_before`, `excepted`, and `discharged` at those ordered event positions. Preserve the deadline's outer time and process discharge before the next attempt. Do **not** replace `<` with `<=` in `done_before`: that makes the current discharge negate its own prerequisite. For a smaller restricted-interface patch, require one attempt per unit per step, reject batches with an integrity constraint, and require callers to serialize attempts; that rejects this valid concurrent input rather than fully modeling it. Do not use effect identifiers as the ordering policy. This is separate from the known gap for repeated obligation triggers: there is just one issue and one obligation here.

### F2 — CRITICAL: detectable-only norms enforce and suppress blame

Evidence: `detect_only_blocks` and `detect_only_overrides`. The first has both `detect_only(p)` and `refused_by(p,u,a,1)`; in the second a duty is overridden and no missed finding appears. This contradicts “The model does not treat it as preventing anything.” The original test used a nonmatching self-declared value, so never exercised enforcement of a matching detectable-only norm. Mixed certified and self-declared constraints reproduce it.

Correct outcome: p is still detectable-only but cannot refuse the permitted payment, block a way of fulfilling the duty, or manufacture the absence of a duty. In the second scenario the attempted payment should be allowed and discharge o. If no attempt is made, the duty should remain owed and yield a finding at its deadline.

Minimal repair: add `not detect_only(N)` to the prohibition premises in `forbids`, `blocked_way`, `touched`, and the amendment-prohibition decision (or centralize those premises in an `enforceable_prohibition` predicate). Otherwise fixing only `forbids` leaves overridden duties and exception eligibility wrong. Keep the annotation for detection. The description does not specify what positive authorization from self-declared data should mean, so this finding does not claim a separate permission defect.

### F3 — CRITICAL: amendment uses a hard-coded partial pattern test

Evidence: `amend_wildcard_bypass`, `amend_ignores_other_attributes`, and `child_new_authority_edge`. `amend_forbidden` requires the literal `pattern(N,kind,amend)` and tests nothing else. Thus an unconstrained prohibition that matches everything does not stop amendment. Conversely, a prohibition constrained to amendments to vote also refuses an amendment to board. The paired ordinary `attempt` facts expose opposite outcomes for the same intended action. New direct authorities over children have the same wildcard bypass.

Correct outcome: the wildcard prohibition must block amendment. A prohibition with an unsatisfied recipient constraint must not block the board amendment. This follows from “Changing the charter's procedure is itself an effect” and the ordinary pattern semantics. The existing `amend/4` interface lacks a general attribute payload; this missing bridge is precisely part of the defect, not evidence that arbitrary amendment patterns work.

Minimal repair: materialize a canonical effect `amend_effect(U,Used,New,T)` for every amendment, with certified `kind=amend`, `to=New` (or an explicitly documented new-procedure attribute), and other supported attributes. Replace the hard-coded pattern premise with `matches(E,N)` for that effect. Empty patterns then work and other constraints are checked. Keep procedure validity as an additional condition. This matching repair alone does not fix F4.

### F4 — CRITICAL: accepted amendments bypass the effect/obligation lifecycle

Evidence: `amend_ignores_obligation_exception`, `amend_does_not_discharge`, and `amend_own_prohibition_bypass`. An amendment effect is allowed and discharges a higher duty while the amendment event is refused; an actually accepted amendment without a duplicate manual attempt leaves a missed finding; and a matching own prohibition refuses the ordinary attempt but the procedure changes anyway. These are consequences of a separate amendment path which does not use `allowed` and does not generate discharge. They are one defect group, not three counts.

Correct outcome: the accepted amendment is the actual effect. It must discharge a matching live duty, respect applicable own prohibitions, and use the same higher-obligation exception as any other effect. A refused amendment must never discharge its duty merely because a disconnected `attempt` was allowed. The amendment paragraph mentions authority prohibitions but does not exempt amendments from the general rule that an applicable prohibition refuses an effect.

Minimal repair: use F3's canonical effect to generate an attempt and drive both amendment acceptance and discharge through one decision. Supply the procedure-valid event with the intended amendment authorization (or explicitly require a permit); require both a valid procedure and `allowed(U,E,T)` for `amended`. Ensure invalid-procedure amendment attempts cannot be allowed/discharge duties; they need an explicit refusal condition. Remove the authority-only `O != U` filter and separate unconditional amendment veto. The existing next-step procedure update can remain. This is a local integration change, not a change in norm priority or the duty deadline.

## Design objections (MEDIUM), not implementation contradictions

### D1 — two amendments fork the charter procedure

`two_amendments` accepts both replacements and subsequently accepts own norms under either procedure. This is one answer set containing both procedures, **not** multiple answer sets. The text speaks of “the procedure” but gives no concurrency arbitration. Specify serial order or reject simultaneous conflicting amendments; a uniqueness constraint on procedures is a useful validation check but alone makes the input unsatisfiable. No claim of a fully specified correct winner is made.

### D2 — a finite declared effect universe decides blame

`empty_effect_universe` has no prohibition and nevertheless overrides the obligation. `unattempted_effect_added` adds a single matching, unattempted effect and obtains `finding(u,missed,o)`; an unrelated effect in `irrelevant_effect_added` changes nothing. With a complete enumeration of possible effects, changing that universe changes feasibility and the program implements its quantifier literally. With mere observed/test effects, this lets omission erase duties. This is not scored as irrelevant-fact nondeterminism: a matching possible effect is relevant under the current representation. State and validate completeness, or reason over satisfiable patterns independently of observed effects. Treat an obligation with no possible witness as impossible/invalid explicitly rather than calling it overridden by nonexistent prohibitions. This finite-domain contract is not stated in the description's Known gaps.

### D3 — inheritance duplicates personal obligations

`inherited_obligation` discharges the creator's duty but finds against the child. This is exactly what “every norm” entails, and so is not a defect against the stated rule. Decide whether duties are intentionally copied or shared; distinguish inheritable policy constraints from assigned personal performances. If personal, restrict automatic inheritance by kind and require explicit delegation for obligations.

### D4 — issue and withdrawal at the same time

`same_step_issue_withdraw` allows the permission forever within the horizon, because `ended` requires the issue strictly before withdrawal. No tie ordering is stated. Document issue-wins, or use `T0 <= T1` for withdrawal-wins when norm identifiers name single instances. Reissuing a withdrawn identifier would need instance identities; repeated triggers are already a known gap. This finding counts only the ambiguous simultaneous single issue/withdrawal, not repeated triggers.

## Negative results and scope boundaries

All 36 scenarios returned exactly one stable model. I did not find zero or multiple answer sets for ordinary finite fact inputs. The apparent negation path through `live`, discharge, and `done_before` moves strictly backward in time; same-time discharge does not feed same-time liveness. Procedure changes similarly feed only the next step. These observations explain the result, but are not a proof for arbitrary malformed inputs or extensions. Duplicate simultaneous amendments cause a nonfunctional procedure relation, not solver nondeterminism.

Controls cover mixed prohibition priorities and multiple ways; two different-priority duties discharged by one emission; ties; late prohibition issue; override or withdrawal at the exception step; withdrawal before deadline; inclusive deadline discharge; issue after deadline; charter-void duties; own low-priority obligations and permissions; missing attributes and empty patterns; direct authority edges over children; creator targeting without power; and nonconflicting issue order. None produced an additional implementation contradiction. The identifiers are opaque symbols and no order on them is used by the program; no identifier-dependence defect was established.

Grandchildren and two creators work as transitive set inheritance in the tested case but exceed the description's explicit one-creator/child scope. Copying duties is D3. When a higher prohibition disappears exactly at deadline, the finding is consistent with the stated deadline snapshot rule; fairness for earlier refused attempts or exhausted opportunities belongs to Known gap 5 (act windows and counterparty refusals). Creation is atemporal here (Known gap 6); resources, commitments, routed authority, leases, repeated triggers, and common control were not treated as new defects. A missing constrained attribute fails matching; the description offers no input-completeness guarantee to falsify on that basis.

## Reproducible evidence: facts and complete shown atoms

Each scenario below is concatenated with the unmodified rules and these common facts:

```prolog
time(0..6).
prio(supra,0). prio(state,1). prio(regulator,2). prio(ministry,3).
prio(group,4). prio(u,9). prio(v,9). prio(w,9).
source(kind,certified). source(to,certified). source(vendor,certified).
```

### simultaneous_exception — DEFECT / F1

Both distinct emissions consume the same single obligation exception at the same step. At most one may use it.

```prolog
over(regulator,u). over(ministry,u).
effect(a). eattr(a,kind,report). eattr(a,to,x).
effect(b). eattr(b,kind,report). eattr(b,to,y).
norm(o,oblige,regulator,u). pattern(o,kind,report). deadline(o,4). issue(o,0).
norm(p,prohibit,ministry,u). pattern(p,kind,report). issue(p,0).
attempt(u,a,1). attempt(u,b,1).
```

Actual answer sets: 1.

```text
allowed(u,a,1)
allowed(u,b,1)
discharged(o,u,1)
excepted(p,u,a,1)
excepted(p,u,b,1)
```

### sequential_exception_control — as specified

Only the first emission is allowed; a later step closes the exception.

```prolog
over(regulator,u). over(ministry,u).
effect(a). eattr(a,kind,report). eattr(a,to,x).
effect(b). eattr(b,kind,report). eattr(b,to,y).
norm(o,oblige,regulator,u). pattern(o,kind,report). deadline(o,4). issue(o,0).
norm(p,prohibit,ministry,u). pattern(p,kind,report). issue(p,0).
attempt(u,a,1). attempt(u,b,2).
```

Actual answer sets: 1.

```text
allowed(u,a,1)
discharged(o,u,1)
excepted(p,u,a,1)
refused(u,b,2)
refused_by(p,u,b,2)
```

### detect_only_blocks — DEFECT / F2

The mixed certified/self prohibition is marked detect_only yet prevents the payment, contrary to the Attributes rule.

```prolog
source(purpose,self).
over(ministry,u). effect(a). eattr(a,kind,pay). eattr(a,purpose,hospitality).
norm(p,prohibit,ministry,u). pattern(p,kind,pay). pattern(p,purpose,hospitality). issue(p,0).
norm(q,permit,ministry,u). pattern(q,kind,pay). issue(q,0).
attempt(u,a,1).
```

Actual answer sets: 1.

```text
detect_only(p)
refused(u,a,1)
refused_by(p,u,a,1)
```

### detect_only_overrides — DEFECT / F2

A detectable-only prohibition also overrides a duty and suppresses its missed finding.

```prolog
source(purpose,self).
over(ministry,u). effect(a). eattr(a,kind,pay). eattr(a,purpose,hospitality).
norm(p,prohibit,ministry,u). pattern(p,kind,pay). pattern(p,purpose,hospitality). issue(p,0).
norm(q,permit,ministry,u). pattern(q,kind,pay). issue(q,0).
over(group,u). norm(o,oblige,group,u).
pattern(o,kind,pay). deadline(o,3). issue(o,0). attempt(u,a,1).
```

Actual answer sets: 1.

```text
detect_only(p)
overridden(o,u,0)
overridden(o,u,1)
overridden(o,u,2)
overridden(o,u,3)
overridden(o,u,4)
overridden(o,u,5)
overridden(o,u,6)
refused(u,a,1)
refused_by(p,u,a,1)
```

### amend_wildcard_bypass — DEFECT / F3

An unconstrained prohibition matches every effect, including amendment. The normal attempt is refused but the procedure changes.

```prolog
over(ministry,u). proc(u,officer,0).
effect(a). eattr(a,kind,amend). eattr(a,to,board).
norm(q,permit,ministry,u). pattern(q,kind,amend). issue(q,0).
norm(p,prohibit,ministry,u). issue(p,0).
attempt(u,a,2). amend(u,officer,board,2).
```

Actual answer sets: 1.

```text
amended(u,board,2)
refused(u,a,2)
refused_by(p,u,a,2)
```

### amend_ignores_other_attributes — DEFECT / F3

The prohibition only matches amendments to vote, not board; the ordinary effect is allowed but the amendment is refused.

```prolog
over(ministry,u). proc(u,officer,0).
effect(a). eattr(a,kind,amend). eattr(a,to,board).
norm(q,permit,ministry,u). pattern(q,kind,amend). issue(q,0).
norm(p,prohibit,ministry,u).
pattern(p,kind,amend). pattern(p,to,vote). issue(p,0).
attempt(u,a,2). amend(u,officer,board,2).
```

Actual answer sets: 1.

```text
allowed(u,a,2)
amend_refused(u,2)
```

### amend_ignores_obligation_exception — DEFECT / F4

The higher duty supplies an exception: the model allows and discharges its amendment effect while refusing to change the procedure.

```prolog
over(ministry,u). proc(u,officer,0).
effect(a). eattr(a,kind,amend). eattr(a,to,board).
norm(q,permit,ministry,u). pattern(q,kind,amend). issue(q,0).
over(regulator,u).
norm(p,prohibit,ministry,u). pattern(p,kind,amend). issue(p,0).
norm(o,oblige,regulator,u). pattern(o,kind,amend). deadline(o,3). issue(o,0).
attempt(u,a,2). amend(u,officer,board,2).
```

Actual answer sets: 1.

```text
allowed(u,a,2)
amend_refused(u,2)
discharged(o,u,2)
excepted(p,u,a,2)
```

### amend_does_not_discharge — DEFECT / F4

An accepted actual amendment does not discharge the duty to amend; a missed finding follows.

```prolog
over(ministry,u). proc(u,officer,0).
effect(a). eattr(a,kind,amend). eattr(a,to,board).
norm(q,permit,ministry,u). pattern(q,kind,amend). issue(q,0).
over(regulator,u).
norm(o,oblige,regulator,u). pattern(o,kind,amend). deadline(o,3). issue(o,0).
amend(u,officer,board,2).
```

Actual answer sets: 1.

```text
amended(u,board,2)
finding(u,missed,o)
```

### amend_own_prohibition_bypass — DEFECT / F4

Own prohibitions apply under the general effect rule too; the amendment bypasses one.

```prolog
over(ministry,u). proc(u,officer,0).
effect(a). eattr(a,kind,amend). eattr(a,to,board).
norm(q,permit,ministry,u). pattern(q,kind,amend). issue(q,0).
norm(p,prohibit,u,u). pattern(p,kind,amend).
issue_by(p,officer). issue(p,0). attempt(u,a,2). amend(u,officer,board,2).
```

Actual answer sets: 1.

```text
amended(u,board,2)
refused(u,a,2)
refused_by(p,u,a,2)
```

### two_amendments — as specified / D1

Both procedures persist and both norms are valid. The text does not specify simultaneous amendment arbitration; this is a design ambiguity.

```prolog
proc(u,officer,0).
amend(u,officer,board,1). amend(u,officer,vote,1).
norm(a,permit,u,u). issue_by(a,board). issue(a,2).
norm(b,permit,u,u). issue_by(b,vote). issue(b,2).
effect(e). attempt(u,e,3).
```

Actual answer sets: 1.

```text
allowed(u,e,3)
amended(u,board,1)
amended(u,vote,1)
```

### empty_effect_universe — as specified / D2

With no declared witness, the obligation is vacuously overridden without any prohibition. This exposes the finite closed-universe design.

```prolog
over(regulator,u). norm(o,oblige,regulator,u).
pattern(o,kind,report). deadline(o,3). issue(o,0).
```

Actual answer sets: 1.

```text
overridden(o,u,0)
overridden(o,u,1)
overridden(o,u,2)
overridden(o,u,3)
overridden(o,u,4)
overridden(o,u,5)
overridden(o,u,6)
```

### unattempted_effect_added — as specified / D2

Adding just one unattempted possible effect changes the verdict to missed. Under a complete effect universe this is as specified, not an implementation violation.

```prolog
over(regulator,u). norm(o,oblige,regulator,u).
pattern(o,kind,report). deadline(o,3). issue(o,0).
effect(a). eattr(a,kind,report).
```

Actual answer sets: 1.

```text
finding(u,missed,o)
```

### irrelevant_effect_added — as specified

Adding an effect outside the obligation pattern does not alter the verdict.

```prolog
over(regulator,u). norm(o,oblige,regulator,u).
pattern(o,kind,report). deadline(o,3). issue(o,0).
effect(z). eattr(z,kind,pay).
```

Actual answer sets: 1.

```text
overridden(o,u,0)
overridden(o,u,1)
overridden(o,u,2)
overridden(o,u,3)
overridden(o,u,4)
overridden(o,u,5)
overridden(o,u,6)
```

### late_higher_prohibition — as specified

Every actual attempt is refused and the deadline duty is overridden; no finding, as specified.

```prolog
over(regulator,u). norm(o,oblige,regulator,u).
pattern(o,kind,report). deadline(o,3). issue(o,0).
effect(a). eattr(a,kind,report).
over(state,u). norm(p,prohibit,state,u). pattern(p,kind,report). issue(p,2).
attempt(u,a,2). attempt(u,a,3).
```

Actual answer sets: 1.

```text
overridden(o,u,2)
overridden(o,u,3)
overridden(o,u,4)
overridden(o,u,5)
overridden(o,u,6)
refused(u,a,2)
refused(u,a,3)
refused_by(p,u,a,2)
refused_by(p,u,a,3)
```

### override_ends_at_deadline — as specified

The deadline is not reset. A finding despite a refused earlier attempt follows the stated deadline rule; act-window fairness is a known gap.

```prolog
over(regulator,u). norm(o,oblige,regulator,u).
pattern(o,kind,report). deadline(o,3). issue(o,0).
effect(a). eattr(a,kind,report).
over(state,u). norm(p,prohibit,state,u). pattern(p,kind,report). issue(p,0). withdraw(p,3).
attempt(u,a,2).
```

Actual answer sets: 1.

```text
finding(u,missed,o)
overridden(o,u,0)
overridden(o,u,1)
overridden(o,u,2)
refused(u,a,2)
refused_by(p,u,a,2)
```

### withdraw_before_deadline — as specified

Withdrawing the duty before its deadline prevents a finding.

```prolog
over(regulator,u). norm(o,oblige,regulator,u).
pattern(o,kind,report). deadline(o,3). issue(o,0).
effect(a). eattr(a,kind,report). withdraw(o,2).
```

Actual answer sets: 1.

```text
(no shown atoms)
```

### deadline_discharge — as specified

The deadline is inclusive.

```prolog
over(regulator,u). norm(o,oblige,regulator,u).
pattern(o,kind,report). deadline(o,3). issue(o,0).
effect(a). eattr(a,kind,report). attempt(u,a,3).
```

Actual answer sets: 1.

```text
allowed(u,a,3)
discharged(o,u,3)
```

### issue_after_deadline — as specified

Never in force at its deadline and never live: no finding follows the literal rules, though rejecting this input would be clearer.

```prolog
over(regulator,u). norm(o,oblige,regulator,u).
pattern(o,kind,report). deadline(o,3). issue(o,4).
effect(a). eattr(a,kind,report). attempt(u,a,4).
```

Actual answer sets: 1.

```text
refused(u,a,4)
```

### void_obligation — as specified

The issuer gets the charter finding; the unit has no duty.

```prolog
over(regulator,u). norm(o,oblige,regulator,u).
pattern(o,kind,report). deadline(o,3). issue(o,0).
charter_forbids(regulator,o). effect(a). eattr(a,kind,report). attempt(u,a,1).
```

Actual answer sets: 1.

```text
finding(regulator,exceeded_charter,o)
refused(u,a,1)
void(o)
```

### withdraw_at_exception_step — as specified

Withdrawal is effective at that step; no exception remains.

```prolog
over(regulator,u). over(ministry,u).
effect(a). eattr(a,kind,report). eattr(a,to,x).
effect(b). eattr(b,kind,report). eattr(b,to,y).
norm(o,oblige,regulator,u). pattern(o,kind,report). deadline(o,4). issue(o,0).
norm(p,prohibit,ministry,u). pattern(p,kind,report). issue(p,0).
withdraw(o,1). attempt(u,a,1).
```

Actual answer sets: 1.

```text
refused(u,a,1)
refused_by(p,u,a,1)
```

### override_at_exception_step — as specified

An effective higher prohibition removes the live duty at the same step.

```prolog
over(regulator,u). over(ministry,u).
effect(a). eattr(a,kind,report). eattr(a,to,x).
effect(b). eattr(b,kind,report). eattr(b,to,y).
norm(o,oblige,regulator,u). pattern(o,kind,report). deadline(o,4). issue(o,0).
norm(p,prohibit,ministry,u). pattern(p,kind,report). issue(p,0).
over(state,u). norm(h,prohibit,state,u).
pattern(h,kind,report). issue(h,1). attempt(u,a,1).
```

Actual answer sets: 1.

```text
overridden(o,u,1)
overridden(o,u,2)
overridden(o,u,3)
overridden(o,u,4)
overridden(o,u,5)
overridden(o,u,6)
refused(u,a,1)
refused_by(h,u,a,1)
refused_by(p,u,a,1)
```

### prohibition_issued_later — as specified

A lower prohibition issued later still gives way.

```prolog
over(regulator,u). over(ministry,u).
effect(a). eattr(a,kind,report). eattr(a,to,x).
effect(b). eattr(b,kind,report). eattr(b,to,y).
norm(o,oblige,regulator,u). pattern(o,kind,report). deadline(o,4). issue(o,0).
norm(p,prohibit,ministry,u). pattern(p,kind,report). issue(p,1).
attempt(u,a,1).
```

Actual answer sets: 1.

```text
allowed(u,a,1)
discharged(o,u,1)
excepted(p,u,a,1)
```

### mixed_priority_ways — as specified

The higher prohibition blocks a, while b remains possible by the lower exception.

```prolog
over(regulator,u). over(ministry,u).
effect(a). eattr(a,kind,report). eattr(a,to,x).
effect(b). eattr(b,kind,report). eattr(b,to,y).
norm(o,oblige,regulator,u). pattern(o,kind,report). deadline(o,4). issue(o,0).
norm(p,prohibit,ministry,u). pattern(p,kind,report). issue(p,0).
over(state,u). norm(h,prohibit,state,u).
pattern(h,to,x). issue(h,0). attempt(u,a,1). attempt(u,b,1).
```

Actual answer sets: 1.

```text
allowed(u,b,1)
discharged(o,u,1)
excepted(p,u,a,1)
excepted(p,u,b,1)
refused(u,a,1)
refused_by(h,u,a,1)
```

### two_duties_one_emission — as specified

One allowed effect meets both live duties of different priority.

```prolog
over(regulator,u). over(ministry,u).
effect(a). eattr(a,kind,report). eattr(a,to,x).
effect(b). eattr(b,kind,report). eattr(b,to,y).
norm(o,oblige,regulator,u). pattern(o,kind,report). deadline(o,4). issue(o,0).
norm(p,prohibit,ministry,u). pattern(p,kind,report). issue(p,0).
over(state,u). norm(h,oblige,state,u).
pattern(h,kind,report). deadline(h,3). issue(h,0). attempt(u,a,1).
```

Actual answer sets: 1.

```text
allowed(u,a,1)
discharged(h,u,1)
discharged(o,u,1)
excepted(p,u,a,1)
```

### own_duty_cannot_escape — as specified

The low own duty stays overridden and cannot extend the authority exception.

```prolog
over(regulator,u). over(ministry,u).
effect(a). eattr(a,kind,report). eattr(a,to,x).
effect(b). eattr(b,kind,report). eattr(b,to,y).
norm(o,oblige,regulator,u). pattern(o,kind,report). deadline(o,4). issue(o,0).
norm(p,prohibit,ministry,u). pattern(p,kind,report). issue(p,0).
proc(u,officer,0).
norm(own,oblige,u,u). pattern(own,kind,report). deadline(own,3).
issue_by(own,officer). issue(own,0). attempt(u,a,1). attempt(u,b,2).
```

Actual answer sets: 1.

```text
allowed(u,a,1)
discharged(o,u,1)
excepted(p,u,a,1)
overridden(own,u,0)
overridden(own,u,1)
overridden(own,u,2)
overridden(own,u,3)
overridden(own,u,4)
overridden(own,u,5)
overridden(own,u,6)
refused(u,b,2)
refused_by(p,u,b,2)
```

### priority_tie — as specified

Ties favor the prohibition.

```prolog
over(regulator,u). over(ministry,u).
effect(a). eattr(a,kind,report). eattr(a,to,x).
effect(b). eattr(b,kind,report). eattr(b,to,y).
norm(o,oblige,ministry,u). pattern(o,kind,report). deadline(o,4). issue(o,0).
norm(p,prohibit,ministry,u). pattern(p,kind,report). issue(p,0).
attempt(u,a,1).
```

Actual answer sets: 1.

```text
overridden(o,u,0)
overridden(o,u,1)
overridden(o,u,2)
overridden(o,u,3)
overridden(o,u,4)
overridden(o,u,5)
overridden(o,u,6)
refused(u,a,1)
refused_by(p,u,a,1)
```

### own_permission_no_escape — as specified

Own permission cannot defeat the authority prohibition after the duty expires.

```prolog
over(regulator,u). over(ministry,u).
effect(a). eattr(a,kind,report). eattr(a,to,x).
effect(b). eattr(b,kind,report). eattr(b,to,y).
norm(o,oblige,regulator,u). pattern(o,kind,report). deadline(o,4). issue(o,0).
norm(p,prohibit,ministry,u). pattern(p,kind,report). issue(p,0).
proc(u,officer,0).
norm(q,permit,u,u). issue_by(q,officer). issue(q,0).
attempt(u,a,5).
```

Actual answer sets: 1.

```text
finding(u,missed,o)
refused(u,a,5)
refused_by(p,u,a,5)
```

### inherited_obligation — as specified / D3

The child receives its own copy of the duty and is blamed even though the creator met it. Every norm is inherited according to the explicit rule.

```prolog
over(regulator,u). norm(o,oblige,regulator,u).
pattern(o,kind,report). deadline(o,3). issue(o,0).
created(u,v). effect(a). eattr(a,kind,report). attempt(u,a,1).
```

Actual answer sets: 1.

```text
allowed(u,a,1)
discharged(o,u,1)
finding(v,missed,o)
```

### grandchild_two_creators — as specified

Transitive inheritance still blocks the grandchild despite another creator and own permission; beyond the documented one-generation scope.

```prolog
over(regulator,u). over(ministry,u).
effect(a). eattr(a,kind,report). eattr(a,to,x).
effect(b). eattr(b,kind,report). eattr(b,to,y).
norm(o,oblige,regulator,u). pattern(o,kind,report). deadline(o,4). issue(o,0).
norm(p,prohibit,ministry,u). pattern(p,kind,report). issue(p,0).
created(u,v). created(v,w). created(x,w).
norm(q,permit,w,w). proc(w,officer,0). issue_by(q,officer). issue(q,0).
attempt(w,a,5).
```

Actual answer sets: 1.

```text
finding(u,missed,o)
finding(v,missed,o)
finding(w,missed,o)
refused(w,a,5)
refused_by(p,w,a,5)
```

### creator_direct_target_without_edge — as specified

Creation does not itself grant issuing power; a direct norm without over is void.

```prolog
created(u,v). norm(p,prohibit,u,v). issue(p,0).
```

Actual answer sets: 1.

```text
void(p)
```

### child_new_authority_edge — DEFECT / F3

An additional direct authority reaches the child, but its wildcard prohibition is bypassed by child amendment too.

```prolog
created(u,v). over(group,v).
norm(p,prohibit,group,v). issue(p,0).
norm(q,permit,v,v). proc(v,officer,0). issue_by(q,officer). issue(q,0).
effect(a). attempt(v,a,1). amend(v,officer,board,2).
```

Actual answer sets: 1.

```text
amended(v,board,2)
refused(v,a,1)
refused_by(p,v,a,1)
```

### same_step_amend_issue — as specified

The old procedure governs issue at the amendment step; the new procedure begins next step.

```prolog
proc(u,officer,0). amend(u,officer,board,1).
norm(a,permit,u,u). issue_by(a,officer). issue(a,1).
norm(b,permit,u,u). issue_by(b,board). issue(b,1).
effect(e). attempt(u,e,1).
```

Actual answer sets: 1.

```text
allowed(u,e,1)
amended(u,board,1)
void(b)
```

### missing_attribute — as specified

A missing constrained attribute does not match; empty permit pattern matches everything. Input completeness is not enforced.

```prolog
over(ministry,u). effect(a). eattr(a,kind,pay).
norm(p,prohibit,ministry,u). pattern(p,to,x). issue(p,0).
norm(q,permit,ministry,u). issue(q,0). attempt(u,a,1).
```

Actual answer sets: 1.

```text
allowed(u,a,1)
```

### same_step_issue_withdraw — as specified / D4

Withdrawal at the first issue step is ignored. Event ordering is unspecified, so this is a temporal design ambiguity rather than a proven violation.

```prolog
over(ministry,u). norm(q,permit,ministry,u).
issue(q,1). withdraw(q,1). effect(a). attempt(u,a,2).
```

Actual answer sets: 1.

```text
allowed(u,a,2)
```

### nonconflicting_issue_order_0_1 — as specified

Nonconflicting issue order leaves obligation discharge unchanged.

```prolog
over(regulator,u). norm(o,oblige,regulator,u).
pattern(o,kind,report). deadline(o,3). issue(o,0).
over(group,u).
norm(q,permit,group,u). pattern(q,kind,pay). issue(q,1).
effect(a). eattr(a,kind,report). attempt(u,a,2).
```

Actual answer sets: 1.

```text
allowed(u,a,2)
discharged(o,u,2)
```

### nonconflicting_issue_order_1_0 — as specified

Nonconflicting issue order leaves obligation discharge unchanged.

```prolog
over(regulator,u). norm(o,oblige,regulator,u).
pattern(o,kind,report). deadline(o,3). issue(o,1).
over(group,u).
norm(q,permit,group,u). pattern(q,kind,pay). issue(q,0).
effect(a). eattr(a,kind,report). attempt(u,a,2).
```

Actual answer sets: 1.

```text
allowed(u,a,2)
discharged(o,u,2)
```

Counts are per root defect/design objection, not per reproducer.

CRITICAL: 4  HIGH: 0  MEDIUM: 4  LOW: 0

# Adversarial test of the executable norm layer (norms.lp)

**Target.** [norms.lp](/root/mic/model/norms.lp), described in [Step 0 norm layer model.md](/root/mic/reports/Step 0 norm layer model.md).
**Scenarios.** [attack_norms_fable.py](/root/mic/model/attack_norms_fable.py): 43 scenarios plus a 400-case random run. Every atom quoted below is copied from a run of that script.

    cd /root/mic/model && PYTHONPATH=/root/mic/vendor python3 attack_norms_fable.py

**Result.** Four reproducible wrong outcomes inside the stated scope that change what is allowed (CRITICAL), three HIGH, ten MEDIUM (three real defects, seven design objections), five LOW. All seventeen original scenarios still pass on a copy of norms.lp carrying the repairs below, and on that copy every CRITICAL and HIGH scenario except the effect-universe pair (H3b, H3c, which no rule change can fix) gives the correct outcome. The repaired copy was a scratch file; norms.lp was not changed.

**Labels.** Finding labels are the scenario names in the script. The severity is stated per finding; H4, H5 and H6 are rated MEDIUM despite the letter.

**How the three classes are separated.**
- *Real defect*: the outcome contradicts the description's own rules, or two of its rules together, or the stated purpose of a rule (priority, "one emission").
- *Design objection*: the program does what the description says, and the outcome is questionable. The script prints "as specified" plus "design objection" for these.
- *Known gap*: listed in the description. Not counted. Nothing below relies on resources, commitments, routes, triggers, act windows or timed creation.

---

## CRITICAL

### C1. The "one emission" exception admits any number of emissions at one step

Real defect. Rule attacked: "A prohibition of strictly lower priority gives way to **an** emission that meets a live obligation ... The exception belongs to that obligation instance, and ends when the obligation is discharged."

```
over(regulator,u). over(ministry,u).
norm(o,oblige,regulator,u). pattern(o,kind,disclose). pattern(o,to,r1). pattern(o,to,r2). deadline(o,4).
norm(p,prohibit,ministry,u). pattern(p,kind,disclose).
issue(o,0). issue(p,0).
attempt(u,rep_r1,1). attempt(u,rep_r2,1).
```
Derived: `allowed(u,rep_r1,1) allowed(u,rep_r2,1) discharged(o,u,1) excepted(p,u,rep_r1,1) excepted(p,u,rep_r2,1)`

With five effects of identical content attempted at step 1 (C1b): `allowed(u,c1,1) ... allowed(u,c5,1)`, five `excepted` atoms.

Correct outcome: one report passes the ministry's prohibition. The model's own test "one emission is excepted, then the exception lapses" refuses the second report when it comes one step later, so the outcome depends on whether two attempts share a step. Cause: `live(O,U,T)` is tested with `done_before`, which only sees discharges at earlier steps, so every attempt at the discharging step sees a live obligation.

Repair (verified): refuse the exception when more than one distinct emission claims it at the same step. This is independent of identifiers; the unit can retry with one emission at the next step.
```
wants(N,O,U,E,T) :- forbids(N,U,E,T), live(O,U,T), matches(E,O), nprio(O,PO), nprio(N,PN), PO < PN,
                    not cheaper_way(O,U,N,T).                         % see C3 for cheaper_way
contested(O,U,T) :- wants(_,O,U,E1,T), wants(_,O,U,E2,T), E1 != E2.
excepted(N,U,E,T) :- wants(N,O,U,E,T), not contested(O,U,T).
```
A better repair needs a total order on events within a step, which the fact format does not have.

### C2. Every created unit gets its own exception for the creator's duty

Real defect: two stated rules ("a created unit inherits every norm", "the exception belongs to that obligation instance") combine into the escape the inheritance rule was written to close.

```
over(regulator,u). over(ministry,u). created(u,v). created(u,w). created(w,w2).
norm(o,oblige,regulator,u). pattern(o,kind,disclose). pattern(o,to,r1). deadline(o,4).
norm(p,prohibit,ministry,u). pattern(p,kind,disclose).
issue(o,0). issue(p,0).
attempt(u,rep_r1,1). attempt(v,rep_r1,2). attempt(w,rep_r1,3). attempt(w2,rep_r1,4).
```
Derived: `allowed(u,rep_r1,1) allowed(v,rep_r1,2) allowed(w,rep_r1,3) allowed(w2,rep_r1,4) discharged(o,u,1) discharged(o,v,2) discharged(o,w,3) discharged(o,w2,4)` and four `excepted(p,...)` atoms.

Correct outcome: the regulator required one report from u. After u's report at step 1 the ministry's prohibition stands for u and for everything u created. The model lets a unit send n+1 forbidden disclosures by creating n units. The obligation instance is keyed `(O,U)`, and inheritance mints a new key per descendant. The same cause produces M1 (a finding against a child for a duty the creator met).

Repair (verified): duties are not inherited; prohibitions and permissions are.
```
applies(N,V) :- norm(N,K,O,U), K != oblige, descends(V,U), O != U.
```
If a shared duty is wanted instead, key `discharged`, `done_before` and the exception on the obligation alone, not on the pair.

### C3. A unit's own prohibition switches on the exception against an authority's prohibition; in general the wrong prohibition yields

Real defect in the stated rule, faithfully implemented. Rule attacked: the exception applies "only if the obligation has no way that no prohibition touches". "Touched" ignores priority, so a prohibition of any rank, including the unit's own at priority 9, removes the free way.

Control (C3): duty to disclose to r1 or r2, ministry forbids r1, r2 is free.
Derived: `refused(u,rep_r1,2) refused_by(p,u,rep_r1,2) finding(u,missed,o)`. Correct.

Add one own norm:
```
proc(u,officer,0).
norm(self,prohibit,u,u). pattern(self,to,r2). issue_by(self,officer). issue(self,1).
```
Derived: `allowed(u,rep_r1,2) discharged(o,u,2) excepted(p,u,rep_r1,2)`

Correct outcome: refused. The unit can meet the duty by breaking its own rule (priority 9); the ministry's prohibition (priority 3) must not yield. A unit has defeated an authority's prohibition by legislating over itself. The description claims own rules are lowest priority and cannot do this.

Same root with authorities only (C3b): supra obliges disclosure to r1 or r2, state (priority 1) forbids r1, group (priority 4) forbids r2, the unit attempts r1.
Derived: `allowed(u,rep_r1,1) discharged(o,u,1) excepted(pa,u,rep_r1,1)`
Correct outcome: refused; only the group's prohibition should yield. The earlier review of the prose version recorded "the lowest-priority prohibition is the one that yields" as the required behaviour for different prohibitions blocking different ways.

Repair (verified): a prohibition N yields only if no way of the obligation is touched solely by prohibitions ranked strictly below N.
```
touched_ge(E,U,N,T) :- effect(E), in_force(N,T), norm(N,prohibit,_,_), applies(N,U), nprio(N,PN),
                       in_force(M,T), norm(M,prohibit,_,_), applies(M,U), matches(E,M), nprio(M,PM), PM <= PN.
cheaper_way(O,U,N,T) :- live(O,U,T), matches(E,O), in_force(N,T), norm(N,prohibit,_,_), applies(N,U),
                        not touched_ge(E,U,N,T).
```
and replace `not free_way(O,U,T)` by `not cheaper_way(O,U,N,T)`. `free_way` and `touched` are then unused.

### C4. A norm that has been withdrawn can never be issued again, and nothing says so

Real defect.
```
over(ministry,u).
norm(p,prohibit,ministry,u). pattern(p,vendor,x).
norm(perm,permit,ministry,u). pattern(perm,kind,order).
issue(perm,0). issue(p,0). withdraw(p,2). issue(p,4).
attempt(u,ord_x,1). attempt(u,ord_x,3). attempt(u,ord_x,5).
```
Derived: `refused(u,ord_x,1) refused_by(p,u,ord_x,1) allowed(u,ord_x,3) allowed(u,ord_x,5)`

Correct outcome: refused at step 5. The second issue is valid (no `void(p)`), yet `ended(p,T)` holds for every T from 2 on because it pairs the withdrawal with the first issue. For an obligation (C4b: `issue(o,0). withdraw(o,1). issue(o,3). attempt(u,rep_r1,4).`) the derived set is just `refused(u,rep_r1,4)`: the required report is refused and no finding can ever arise.

If norm identifiers are meant to be single-use, the model must say so and reject the second issue; at present it accepts it silently.

Repair (verified; also fixes H4): end each period of validity separately.
```
ended(N,T0,T) :- valid(N,T0), withdraw(N,T1), time(T), T0 <= T1, T1 <= T.
in_force(N,T) :- valid(N,T0), time(T), T0 <= T, not ended(N,T0,T).
```
Residue: `done(O,U)` survives a re-issue, so a re-issued duty that was met the first time is not owed again. A re-issue should be a new instance.

---

## HIGH

### H1. Detect-only norms prevent, override and unlock, contrary to the description

Stated rule not implemented: "A norm that reads an attribute the unit declares for itself is marked as detectable only. The model does not treat it as preventing anything." In the program `detect_only/1` is a label that no other rule reads. The existing test passes only because its payment does not match the prohibition.

H1, the unit labels the payment honestly:
```
source(purpose,self).
effect(pay_h). eattr(pay_h,kind,pay). eattr(pay_h,purpose,hospitality).
over(ministry,u).
norm(p,prohibit,ministry,u). pattern(p,kind,pay). pattern(p,purpose,hospitality).
norm(perm,permit,ministry,u). pattern(perm,kind,pay).
issue(p,0). issue(perm,0). attempt(u,pay_h,1).
```
Derived: `detect_only(p) refused(u,pay_h,1) refused_by(p,u,pay_h,1)`. The norm prevented something. Under the stated rule the payment goes through and the breach is found afterwards.

H1b, a detect-only prohibition of higher rank on the unit's only declared payment: `overridden(o,u,0..6)`, no finding. A self-declared label has cancelled a regulator's duty.

H1c, a detect-only prohibition placed on the free way (`pay_b` labelled hospitality): `allowed(u,pay_a,1) excepted(p,u,pay_a,1)`. A self-declared label has unlocked the exception against the ministry's certified prohibition on `pay_a`. This is C3 reached by relabelling instead of legislating.

Repair (verified): add `not detect_only(N)` to `forbids`, to the prohibition in `blocked_way`, and to the prohibition in `touched` / `touched_ge`; record the breach after the fact:
```
finding(U,breached,N) :- allowed(U,E,T), in_force(N,T), norm(N,prohibit,_,_), applies(N,U),
                         matches(E,N), detect_only(N).
```
With this, H1 gives `allowed(u,pay_h,1) finding(u,breached,p)`.

Related, LOW (M8): an attribute with no `source/2` fact is treated as certified. A prohibition on `memo` with no source fact is not marked. Missing should mean uncertified.

### H2. Two amendments at one step leave two procedures in force

Real defect; the model has one answer set but it encodes both outcomes.
```
proc(u,officer,0).
amend(u,officer,vote,1). amend(u,officer,board,1).
norm(n1,permit,u,u). pattern(n1,vendor,x). issue_by(n1,vote).  issue(n1,3).
norm(n2,permit,u,u). pattern(n2,vendor,y). issue_by(n2,board). issue(n2,3).
attempt(u,ord_x,4). attempt(u,ord_y,4).
```
Derived (with `#show proc/3`): `amended(u,board,1) amended(u,vote,1) proc(u,vote,2..6) proc(u,board,2..6) allowed(u,ord_x,4) allowed(u,ord_y,4)`

Correct outcome: a charter has one procedure at a time. After step 1 the unit can issue norms by either of two procedures, so "by the procedure its charter has at that moment" has no single referent, and the rule "the new one then binds" is false for both.

Repair (verified): conflicting amendments at one step are all refused.
```
amend_clash(U,T) :- amend(U,_,N1,T), amend(U,_,N2,T), N1 != N2.
amend_ok(U,Used,New,T) :- amend(U,Used,New,T), proc(U,Used,T), not amend_forbidden(U,New,T), not amend_clash(U,T).
amended(U,New,T) :- amend_ok(U,_,New,T).
amend_refused(U,T) :- amend(U,Used,New,T), not amend_ok(U,Used,New,T).
```
The last line also fixes H2b (LOW): `amend(u,officer,vote,1). amend(u,ghost,board,1).` derives only `amended(u,vote,1)`; the amendment under a procedure that does not exist is neither accepted nor reported, because `amend_refused/2` is suppressed by any accepted amendment at the step. `amend_refused` should carry the amendment, not only the step.

### H3. The declared effect universe decides who is blamed

Dependence on something that should not matter, and one outcome that contradicts the stated rule.

H3, a duty that no declared effect can meet:
```
effect(rep_r1). eattr(rep_r1,kind,disclose). eattr(rep_r1,to,r1).
over(regulator,u).
norm(o,oblige,regulator,u). pattern(o,kind,disclose). pattern(o,to,r9). deadline(o,3). issue(o,0).
```
Derived: `overridden(o,u,0) ... overridden(o,u,6)`, no finding.
The rule says overridden "while every way of meeting it is blocked **by a prohibition**". No prohibition exists. A mistyped value in a pattern, or one forgotten `effect` fact, turns a duty into "not owed" and reports a cause that is not there.

H3b, same norms, same behaviour (the unit does nothing): regulator obliges any disclosure, state forbids disclosure to r1.
- Universe {rep_r1}: `overridden(o,u,0..6)`, no finding.
- Universe {rep_r1, rep_r2}, rep_r2 never attempted: `finding(u,missed,o)`.

H3c, same norms, same single attempt `attempt(u,rep_r1,1)`: regulator obliges any disclosure, ministry (lower) forbids r1.
- Universe {rep_r1}: `allowed(u,rep_r1,1) discharged(o,u,1) excepted(p,u,rep_r1,1)`.
- Universe plus unattempted rep_r2: `refused(u,rep_r1,1) refused_by(p,u,rep_r1,1) finding(u,missed,o)`.

H3b and H3c are what the rules mean if `effect/1` is the complete, certified set of things the unit could emit. The description does not say that, does not say who declares the set, and declares it globally, not per unit. If the unit or the scenario author supplies it, leaving out one effect buys an exception or an acquittal. The existing tests declare `rep_x` in scenarios where it is irrelevant and omit other recipients, so the set is already treated as a convenience.

Repair: (a) verified, for the vacuous case:
```
overridden(O,U,T) :- in_force(O,T), norm(O,oblige,_,_), applies(O,U), blocked_way(_,O,U,T), not has_way(O,U,T).
unmeetable(O) :- norm(O,oblige,_,_), not matches(_,O).      #show unmeetable/1.
```
and exclude `unmeetable` obligations from the missed finding (the fault is the issuer's). (b) Not a rule change: state that the effect set is a certified input fixed per unit before norms are issued, and add `can(Unit,Effect)`.

---

## MEDIUM: real defects

### H4. A withdrawal at the step of issue is ignored for ever
`issue(p,1). withdraw(p,1). attempt(u,ord_x,3).` Derived: `refused(u,ord_x,3) refused_by(p,u,ord_x,3)`. A withdrawal one step later ends the norm; at the same step the norm becomes permanent, because `ended` requires `T0 < T1`. The same happens for a withdrawal recorded before the issue. Either reading of a same-step pair is defensible; "in force for ever" is not. Fixed by the C4 repair (`T0 <= T1`: withdrawal at a step acts after issue at that step).

### H5. A prohibition whose origin has no priority fact cannot be overridden against or excepted, and the unit is blamed
```
over(regulator,u). over(council,u).            % no prio(council,_)
norm(o,oblige,regulator,u). pattern(o,kind,disclose). pattern(o,to,r1). deadline(o,3).
norm(p,prohibit,council,u). pattern(p,kind,disclose).
issue(o,0). issue(p,0). attempt(u,rep_r1,1). attempt(u,rep_r1,2). attempt(u,rep_r1,3).
```
Derived: `refused(u,rep_r1,1) refused(u,rep_r1,2) refused(u,rep_r1,3)` (each `refused_by(p,...)`) and `finding(u,missed,o)`.
`blocked_way` and `excepted` both need `nprio` on both norms, so the prohibition neither overrides the duty nor yields to it. The unit is refused every attempt and still gets the finding. This is the case the brief calls "finding raised although the unit was refused every attempt". The same holds for any created unit not listed in `prio` (the common facts list only u and v), though there the blame is on the unit's own rule. Repair: `:- norm(_,_,O,_), not prio(O,_).` (no answer set, which the harness reports), or a shown `input_error` atom. Not applied in the verified copy.

### H6. Amendment is not "an effect like any other"
- H6: a prohibition with no pattern forbids every effect but not amendment. `norm(p,prohibit,ministry,u). issue(p,0). amend(u,officer,vote,2). attempt(u,ord_y,2).` Derived: `amended(u,vote,2) refused(u,ord_y,2) refused_by(p,u,ord_y,2)`.
- H6b: a prohibition narrowed by another attribute forbids all amendment. `pattern(p,kind,amend). pattern(p,to,board). amend(u,officer,vote,2).` Derived: `amend_refused(u,2)`.

`amend_forbidden` tests the literal fact `pattern(N,kind,amend)` and never uses `matches`. Amendment also needs no permission and is unaffected by priority or by the exception rule, so three of the four parts of "deciding an effect" do not apply to it. Repair (verified) for the matching part:
```
amend_mismatch(N,New) :- amend(_,_,New,_), constrained(N,A), A != kind, A != to.
amend_mismatch(N,New) :- amend(_,_,New,_), constrained(N,kind), not pattern(N,kind,amend).
amend_mismatch(N,New) :- amend(_,_,New,_), constrained(N,to), not pattern(N,to,New).
amend_forbidden(U,New,T) :- amend(U,_,New,T), in_force(N,T), norm(N,prohibit,O,_), applies(N,U), O != U,
                            not amend_mismatch(N,New).
```

---

## MEDIUM: design objections (the model follows its description)

**M1. A creator's duty becomes each created unit's duty.** `created(u,v)`, u reports at step 1. Derived: `allowed(u,rep_r1,1) discharged(o,u,1) finding(v,missed,o)`. A unit the regulator never addressed has a finding for a duty already met. Creating a unit creates a finding. Removed by the C2 repair.

**M2. The finding rule looks only at the deadline step.**
- A higher prohibition issued at the deadline step, unit idle for four free steps: `overridden(o,u,4) overridden(o,u,5) overridden(o,u,6)`, no finding.
- M2b, a higher prohibition in force for steps 0 to 3 and withdrawn at the deadline step 4, unit attempted at 1, 2 and 3: three `refused_by(p,...)` and `finding(u,missed,o)`.

An idle unit is acquitted by a late prohibition and a diligent one is blamed when the block lifts on the last step. The deadline is not moved when an override ends. A fair rule needs the act window of known gap 5; until then, at least require that the duty was live at some step for a finding, and count steps live.

**M3. An authority cannot make an exception to its own prohibition.** Regulator forbids disclosure at step 0 and at step 2 obliges disclosure to r1. Derived: `overridden(o,u,2..6) refused(u,rep_r1,3) refused_by(p,u,rep_r1,3)`. Ties go to the prohibition regardless of order or specificity, so the later, narrower order is dead on arrival with no finding and no warning. The prose version had later-wins within one origin, and the earlier review recorded that as correct.

**M4. Priority is used only between a duty and a prohibition.** A supranational permission (priority 0) for vendor x against a group prohibition (priority 4) on orders: `refused(u,ord_x,1) refused_by(p,u,ord_x,1)`. Any prohibition from any authority, or from the unit itself, beats any permission. This may be intended; the description's phrase "priority among authorities" suggests more.

**M5. Default refusal is empty.** A unit with a procedure issues itself a permission with no pattern: `allowed(u,ord_x,1)` although the ministry permitted only vendor y. "An effect is refused unless something permits it" constrains only a unit with no procedure. The existing inheritance test relies on this (the created unit permits itself). Either an own permission should work only inside an authority's permission, or the default-refusal rule should be dropped as decoration.

**M6. No one can address a created unit.** `norm(n,prohibit,ministry,v)` with `over(ministry,u). created(u,v).` Derived: `void(n)`; the creator's norm over v is `void(m)` as well; v is allowed both orders. The ministry can bind v only by binding u identically. It cannot permit u and forbid v, and the creator has no power over what it created unless the scenario adds an edge. Suggested: `power(O,V) :- power(O,U), descends(V,U).`

**M7. A unit's own prohibition on amendment does not bind it.** `norm(e,prohibit,u,u). pattern(e,kind,amend).` then `amend(u,officer,vote,2)`. Derived: `amended(u,vote,2)`. The description says only "an authority may forbid it", so this is as specified, but a unit cannot entrench its own procedure, while its own prohibition does refuse its other effects.

---

## LOW

- **L1.** One norm issued twice, first void, then valid: `void(perm)` and `allowed(u,ord_y,4)` together. `void/1` should carry the step. Repair: `void(N,T) :- issue(N,T), not valid(N,T).`
- **L2.** An obligation with no `deadline` fact, or a deadline past the horizon, is inert: it carries no permission (`refused(u,rep_r1,1)`) and can never be missed (no `finding(u,missed,o2)` with `deadline(o2,9)`, horizon 6). No input check.
- **L3.** An obligation with no pattern permits every effect and is met by any of them: `allowed(u,ord_x,1) discharged(o,u,1)`. As specified.
- **L4 (M8 in the script).** An attribute without a `source` fact counts as certified.
- **L5 (H2b).** `amend_refused/2` hides a refused amendment behind an accepted one at the same step.

---

## Attacks that failed

The model gave the correct outcome on these (all in the script, marked "OK"):

- Mixed priorities where the higher prohibition covers one way and the lower covers all: the higher stands, the lower yields.
- Two duties of different priority and one emission: the higher is discharged, the lower is overridden and cannot ride.
- Override lifted before the deadline: live again; a finding if unused.
- Discharge at the deadline step; an attempt one step late is refused and the duty is a finding.
- A duty withdrawn before its deadline, issued after its deadline, or void: no finding and no permission.
- Grandchildren and a unit with two creators inherit prohibitions.
- An own norm issued at the step of the amendment is judged by the old procedure.
- A created unit inherits a ban on amendment.
- A unit's own obligation cannot open an exception against an authority.
- **Answer-set count.** 400 random scenarios (up to six norms of random kind, origin, target, pattern, issue and withdrawal step, attempts, amendments, a creation cycle `created(u,v). created(v,u).`): exactly one answer set every time. The negation cycle through `allowed`, `discharged`, `done_before` and `live` always passes through a strictly earlier step, and the program has no constraints, so I could not produce zero or several answer sets and do not think it is possible with integer steps.
- **Identifiers.** Renaming the norms in reverse order changed no outcome in the 400 cases.

## Not counted

- `withdraw/2` has no actor and no power check, while issuing has both. The fact format cannot express "who withdrew", so this could not be tested as a wrong outcome.
- Creation is not an event in time (known gap 6), so "norm issued before the unit existed" cases were not counted.

## Verification of the repairs

The repairs for C1, C2, C3, C4, H1, H2, H3 (vacuous case), H4, H6 and L1 were applied together to a scratch copy of norms.lp. On that copy all seventeen scenarios of test_norms.py pass, and attack_norms_fable.py reports four remaining defects: H3b, H3c (the effect universe, not fixable by a rule), H5 and L2 (input checks, not applied). The script accepts a rules file as its first argument for this purpose.

CRITICAL: 4  HIGH: 3  MEDIUM: 10  LOW: 5

# Formal model

Notation is ordinary set theory. Section numbers are cited from the code and the status ledger.
What is *proved*, *tested* or *assumed* about each statement is recorded in `03-status-ledger.md`, not here.

## 1. Kernel

**1.1 Events.** An event is `e = (seq, id, kind, actor, payload, cause, src)`. `payload` is a JSON value.
`id = H(prev_id, seq, kind, actor, payload, cause, src)` with `H` = SHA-256 truncated to 64 bits, so the log
is a hash chain. `cause` is the id of the event whose processing emitted `e`, or ⊥ for an external stimulus.
`actor` and `src` are assigned by the kernel from the position of the emitting unit; a unit cannot choose them.

**1.2 Units.** A unit is `U = (S, s₀, In, Out, δ)` with `In, Out` sets of kind patterns and
`δ : S × Event → S × Emit*` total and deterministic. Events whose kind is not in `In` leave the state unchanged
(input-enabledness). `U` may only emit kinds in `Out` (checked at run time).

**1.3 Parallel composition.** For units `U₁…Uₙ` with distinct names, `Par(U₁…Uₙ)` has state `S₁ × … × Sₙ`,
`In = ⋃ Inᵢ`, `Out = ⋃ Outᵢ`, and

    δ((s₁…sₙ), e) = ((s₁'…sₙ'), o₁ ++ … ++ oₙ)   where (sᵢ', oᵢ) = δᵢ(sᵢ, e) if e.kind ∈ Inᵢ, else (sᵢ, ε)

Well-formedness (checked when `Par` is built): for every kind not declared *open*, at most one part may emit
it; no part listens inside another entity's namespace. `Par` adds no identity: it is transparent.

**1.4 Boundary.** `Entity(n, U, imports, exports)` is a unit named `n`. Internal kinds `k` appear outside as
`n/k` with actor `n/a`; a kind in `exports` leaves with actor `n`; a kind in `imports` enters. The inside sees
its own names unprefixed. So an entity is *one actor* outside and *a system* inside.

**1.5 Runtime (closed system).** Given a root unit and a FIFO queue `Q`: an external stimulus is enqueued;
repeatedly dequeue `x`, append it to the log as event `e`, compute `δ(s, e)`, order the emitted events by `src`
(stable, so each leaf's own order is kept) and enqueue them with `cause = e.id`. When `Q` empties and something
other than `settle` was processed since the last `settle`, enqueue one `settle` event. Stop when `Q` is empty.
A fuel bound turns non-termination into an explicit `Divergence` failure.

**1.6 Independence.** The *footprint* of an event is the set of leaves that accept its kind and whose state it
changes. Two events with disjoint footprints commute (T5). The log's partial order is the transitive closure of
`cause` and of log order restricted to non-independent pairs.

**1.7 Refinement.** `A ⊑ B` iff there is a relation `R ⊆ S_A × S_B` with `R(a₀, b₀)` and, whenever `R(a, b)`,
`R(δ_A(a,e).state, δ_B(b,e).state)` and equal outputs. Then `A` and `B` have equal traces (T3) and refinement
is preserved by `Par` (T2).

## 2. Authority (unit `inst`)

**2.1** A capability is `c = (id, issuer, holder, actions, scope, limits, expires, delegable, parent, revoked)`.
Root capabilities (`parent = ⊥`) are created only by the genesis event: every member `m` receives
`base:m` (speech acts) and `own:m` (`transfer` with scope `from = m`, delegable). **Ownership is holding the
root capability over an account.**

**2.2** `c` *attenuates* `p` iff `actions(c) ⊆ actions(p)`, `scope(c) ⊇ scope(p)`, `limits(c) ≤ limits(p)`
pointwise on `p`'s resources, and `expires(c) ≤ expires(p)`. Delegation creates `c` with `parent = p` only if
`p` is delegable, held by the delegator, and `c` attenuates `p`.

**2.3** The chain of `c` is *valid at time t* iff no capability on it is revoked or expired.
`revoke(x)` is permitted to any issuer on `x`'s chain; it invalidates every chain through `x`.

**2.4 Count-as.** `do(a, action, args, c)` by actor `a` **counts as** `ok.action(args)` iff `a` is a member,
holds `c`, the chain of `c` is valid, `action ∈ actions(c)`, `args` agree with `scope(c)`, and (for transfers)
the amount fits the remaining budget of *every* capability on the chain. Otherwise the event `denied` is
recorded. `inst` is the only unit that may emit `ok.*`.

## 3. Resources (unit `ledger`)

Balances `bal : Holder → RType → ℕ`. `ok.transfer(f, t, r, n)` is enabled iff `bal(f)(r) ≥ n`; it yields
`bal(f)(r) − n`, `bal(t)(r) + n`. Invariant: `Σ_h bal(h)(r) = minted(r)`.

## 4. Claims and evidence (unit `epi`)

**4.1** A *proposition* is an opaque name. A *claim* records that an actor asserted it. An *evidence item* is
`(id, source, prop | target, polarity, method, derived_from, artifact digests, time)`: it supports (+) or
rebuts (−) a proposition, or undercuts another evidence item.

**4.2 Admissibility** under evidence rules `(ranks, trusted, self_support)`: not retracted; method ranked;
source trusted for the method; every bound artifact still has the recorded digest; every parent admissible;
and, for support of `P`, no *root source* (source of an underived ancestor) is a claimant of `P`.

**4.3 Attack relation** on admissible items, with `rank(e) = min` of its method's rank and its parents' ranks:
support `s` and rebuttal `r` of the same proposition attack each other, except that an attack on a strictly
higher-ranked item fails; an undercutter always attacks its target; any successful attack on `e` is also an
attack on every item derived from `e`.

**4.4 Assessment.** Label items by the grounded extension. `P` is **refuted** if some rebuttal is *in*;
else **accepted** (since the earliest *in* supporter's time) if some supporter is *in*; else **contested** if
some item is undecided; else **unsupported**. Assessment is a function of (evidence, claims, artifacts, rules):
any change to those re-evaluates the connected component it touches.

## 5. Commitments (unit `norms`)

**5.1** `C = (id, debtor, creditor, ant, cons, deadline, tags, meta, parent, basis)`. A *condition* is
`prop P` (holds at the time `P` became accepted, while it is accepted) or `event (kind, match)` (holds from
the first matching institutional event; irrevocable).

**5.2 Status** is the function (first matching line wins):

| Status | When |
|---|---|
| released / cancelled / void | the creditor released it / the debtor withdrew it while still conditional / its basis no longer stands |
| satisfied | `cons` held at some time ≤ deadline |
| violated | the debtor repudiated it after detachment; or `ant` held by the deadline and now > deadline |
| expired | `ant` did not hold by the deadline and now > deadline |
| detached | `ant` holds (an unconditional obligation is in force) |
| conditional | otherwise |

**5.3 Powers over a commitment.** Only an actor can commit *itself* (`debtor = actor`). Only the creditor can
release or assign. Rules can impose a commitment on any member (5.4): membership is what makes one
subject to that power.

**5.4 Secondary commitments.** Rule templates `on_violation(tag)` and `on_verdict(finding, tag)` instantiate a
new commitment whose `basis` names the violated commitment (and finding). If the basis ceases to hold, the
derived commitment becomes `void`; if it holds again, it is revived.

## 6. Responsibility (unit `adj`)

For each commitment family (the tree under `parent`), at each quiescent point, the adjudicator evaluates the
responsibility rules — a stratified Datalog program — over the facts `debtor, creditor, status, parent, cons,
tag, claimed, notified, prop, kind, principal, role`. A **verdict** for a violated commitment lists the derived
findings, a proof tree for each, the rule version, and every negated atom the proof used (its closed-world
assumptions).

## 7. Governance (unit `gov`)

The rule set `ρ = (evidence, norms, responsibility, governance)` is data with a version and digest. A patch is
enacted when approved by `quorum(ρ)` distinct holders of the `approve` power, provided the patched rule set
(i) leaves entrenched paths untouched, (ii) has safe, stratified responsibility rules, (iii) has an acyclic
template tag graph, and (iv) keeps `quorum ≥ 1`. Enactment emits `rules.changed`; `epi` and `adj` re-evaluate.

## 8. Typing discipline

The mathematical model above is realised with one rule: **no meaning is carried by the content of a string.**

- The kernel is generic in an event type `E` with a closed set of kinds `E::Kind` and a closed set of
  failures `E::Failure`. A unit's interface is a pair of sets of kinds.
- The institutional vocabulary is one closed sum type `Event`. What an actor can attempt is the closed sum
  `Act`; the count-as gate turns `Do { act }` into `Ok { by, via, act }` without ever inspecting text.
- Identifiers are opaque. Those that name things chosen by people are validated `Name`s (`[a-z0-9_-]+`);
  those that name things created by the system are 64-bit digests of the thing's canonical encoding, each in
  its own newtype. A capability id says where it came from: `Base`, `Own`, `Charter`, or `Delegated(digest)`.
- A proposition is structured: `Done(task)`, `Remediated(commitment)`, `Audited(commitment)`,
  `Conforms(requirement)`, `Fact(name)`. Templates build propositions and parties from small expression
  types, never by substitution into text.
- Responsibility rules are sorted Datalog: every predicate has a signature over the sorts *actor, commitment,
  proposition, status, tag, kind, role, boolean, assessment*, and a rule in which a variable is used at two
  different sorts is rejected.
- A refusal is an ordinary event carrying an enumerated reason. A *fault* (the machinery itself cannot
  proceed) is a separate, also enumerated, error type; library code has no other way to fail.
- Canonical form: everything persisted or used as an identity is hashed from its JSON encoding, in which
  maps are ordered and field order is fixed by the type.

`03-status-ledger.md` §1a lists what this makes inexpressible, and what it does not.

## 9. How the questions in the brief are answered

1. **Creation, transfer, discharge of normative positions.** Created by an empowered act (`commit`,
   `delegate`) or by a rule template; transferred by `assign` (claims) and `delegate` (powers); discharged by
   satisfaction or release; violated by deadline or repudiation; revoked along the chain; voided with their basis.
2. **How events create or modify obligations.** Only three ways: an institutional act, the status function
   re-evaluating (time, evidence), and templates firing on violation or verdict.
3. **Duty, obligation, commitment, responsibility, accountability, liability.**

   | Term | Definition here | Primitive or derived |
   |---|---|---|
   | Commitment | the object `C(debtor, creditor, ant, cons, deadline)` | **primitive** |
   | Obligation | a commitment whose status is `detached` | derived (status) |
   | Duty | an obligation seen from the debtor; its correlative *claim* is the same object seen from the creditor | derived (viewpoint) |
   | Permission | an action for which the actor holds a valid capability | derived (authority) |
   | Power | a capability whose action changes normative relations (`commit`, `delegate`, `release`, `approve`) | derived (authority) |
   | Liability (Hohfeld) | being subject to another's power, e.g. the auditor role to rule-imposed duties | derived |
   | Responsibility (prospective) | being debtor of a commitment | derived |
   | Responsibility (retrospective) | a finding (`culpable`, `contributed`, …) derived from the trace by rules | derived (rules) |
   | Accountability | the debtor of a violated commitment answers for it regardless of delegation (`acc`) | derived (rules) |
   | Liability (remedial) | a secondary commitment whose basis is a verdict | derived (template) |

   What cannot be derived away and must stay explicit: debtor vs performer vs causer; the direction of a
   commitment; antecedent vs consequent; whether a norm came from an act or a rule; assessed vs actual truth.
4. **Claims, evidence, refutation.** §4. Refutation is new evidence; assessment is recomputed; nothing is deleted.
5. **Authority and trust anchors.** §2 and §7. The only anchor is the genesis event; its digest is the constitution.
6. **Humans, AIs, services, organisations.** All are units with a name. Their differences are attributes read
   by rules (rule `liab1`: an AI's liability lifts to its principal), never kernel types.
7. **Composition without product enumeration.** `Par` keeps component states separate; invariants are checked
   per unit (T1); revision touches only the affected component and family (measured).
8. **What survives composition.** See the classification table in `03-status-ledger.md`.
9. **Closed mathematics, open world.** The system is closed over the log. The world enters only as external
   stimuli that are *reports* (a probe result, a clock tick, a file digest); every such source is a named
   assumption. Truth is never represented, only assessment.
10. **Bounded cost.** Measured in `04-measurements.md`; limits stated there and in the ledger.

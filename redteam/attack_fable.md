# Red-team report (fable)

Scope: `crates/rigor` at `4dd907c`. 16 findings, all confirmed by running them except where marked.

**How to reproduce.** Every confirmed finding is a test in `crates/rigor/tests/redteam.rs`.
Each test **passes when the defect is present** (its assertions state the wrong behaviour observed).

```
cargo test --test redteam            # 23 passed; 0 failed
cargo test --test redteam rt03       # one finding
```

Findings 7a and 7b were also run against the real `steward` binary on a copy of the repository; output is quoted.
Nothing in `src/` was changed. The two added test files are fmt- and clippy-clean. They raise the
test count above the "72 tests" the README and ledger state, and change the steward's `rust-sources` digest.

---

## Critical

### 1. Any member decides any assessment: "inference" launders trust and rank
**Severity: critical. Confirmed** (`rt03`, `rt04`, `rt23`).

`Method::Inference` is `Trust::Anyone` in `standard_rules`. Its only requirement is a non-empty
`derived_from`, which may name *any* existing evidence. Its rank is `min(9, rank of what it cites)`,
so it inherits the cited item's strength (`epistemic.rs:451`). Nothing relates the cited evidence
to the conclusion. An undercutter "always succeeds" whatever its rank (`epistemic.rs:250`), and can
itself only be attacked through what it cites.

`rt03`: VENDOR is trusted for no method. It cites an unrelated audit by AUDITOR and:

| Act by VENDOR | Result |
|---|---|
| `Undercuts(monitor's probe)` | `deployed`: Refuted → **Accepted** |
| `Supports(the-moon-is-cheese)` | **Accepted** |
| `Rebuts(service-is-live)`, which a trusted probe supports | **Refuted** (rank 4 beats 3) |
| claims X, then `Supports(X)` | **Accepted**, with `self_support = false` and entrenched |

`rt04` runs the README demonstrator, then the culpable agent sends one act: an inference undercutting
the monitor's probe, citing Alice's acknowledgement of the audit report. Observed:
`deployed` Accepted, `c1` and `c2` Violated → **Satisfied**, `findings(c1)` = `{}`.

`rt23`: "no self-certification" (N4) binds only an actor who filed a formal `Claim`
(`epistemic.rs:183`, `None => true`). A debtor trusted for any method attests and does not claim:
its own commitment reads Satisfied. The demonstrator's `c2` is satisfied this way, on the
service's own receipt.

Why it matters: the `trusted` table and method ranks are the whole basis of "assessment from sources
it was told to trust". They constrain nothing. Every downstream conclusion (status, verdict,
liability, attested snapshot) is controllable by any member. N4's test only tries laundering through
the claimant's *own* evidence.

### 2. A unit outside a boundary forges the inner institution's events
**Severity: critical. Confirmed** (`rt01`).

`Par::new` exempts `Scoped` from output-ownership (`unit.rs:260`). An `Entity` unwraps any `Scoped`
event carrying its own name and feeds it to its inner units as internal (`unit.rs:381`), whoever
emitted it. Composition refuses units that *listen* to `Scoped`, not units that *emit* it. Inner
units never check `who` or `src` on `Ok`, `Assessed`, `Transferred`, `Verdict`, `RulesChanged`.

`rt01`: an outer leaf `mallory` (not a member anywhere, `outputs = {Scoped}`) sits beside a
federated organisation. `org(...)` accepts it. On a tick it emits
`Scoped{vendor, Ok{by: eng, via: own:eng, Transfer eng→qa 100}}` and `Scoped{vendor, Assessed{p, Accepted}}`.
Observed inside the vendor: `balance(qa) = 100`, `balance(eng) = 0`, no inner `Do` in the log; inner
`epi` holds `p` Unsupported while inner `norms` holds the commitment on `p` **Satisfied**.

Why it matters: it breaks §2.4 ("`inst` is the only unit that may emit `ok.*`"), C5, and the
federated boundary's point (P32). `a_unit_cannot_impersonate` and `interface_enforcement` cover only
a same-level `Do` or `Ok`.

---

## High

### 3. The runtime takes any event kind as an external stimulus, from anyone
**Severity: high. Confirmed** (`rt02`).

`Runtime::inject_as` (`runtime.rs:62`) and `World::inject` are public and do not restrict kinds.
A-AUTH assumes the *actor* is genuine; it does not cover an outsider submitting an event that only
a unit may produce. `rt02`, all by the non-member `nobody`:

- `Ok{by: alice, Transfer alice→vendor 10}` → the ledger pays.
- A forged `TransferFailed` for a transfer that succeeded → the agent's budget is refunded
  (`institution.rs:282`). **80 credits move under a 50-credit capability**; `Institution::audit()` is empty.
- A forged `Assessed{Accepted}` → a commitment becomes Satisfied.
- `Tick(u64::MAX)` → every expiring capability is dead for good (the clock is `max`).
- The log saves and `restore` accepts it: replay treats every `cause: null` entry as a stimulus (`adapters.rs:173`).

### 4. `Signal` is unauthenticated, untimed and last-write-wins; it decides culpability
**Severity: high. Confirmed** (`rt05`, `rt06`).

`adjudicator.rs:281` stores `notified[prop][to] = value` from any `Signal`, from anyone including
non-members, with no time and no sender. Rule `knew` (`org.rs:73`) reads it.

- `rt05`: after the demonstrator's verdict, `nobody` injects `Signal{to: agent, deployed, true}`.
  `findings(c1)`: `{Accountable agent, Contributed deployer, Culpable agent, Liable vendor}` →
  `{Accountable agent, Contributed deployer}`. The vendor's remediation duty → **Void**.
- `rt06`: with a lying service nobody is at fault (N8). A `Signal{…, false}` sent after everything
  yields `Culpable agent`, `Liable vendor`. "It was notified and claimed success" holds for a
  notification that came after the claim.

### 5. Liability is escaped, or imposed on others, through fields the debtor writes itself
**Severity: high. Confirmed** (`rt07`, `rt08`).

A commitment's `parent`, `tags` and `terms` come from the debtor; `norms.create` checks only that the
parent exists (`norms.rs:186`). The responsibility rules treat all three as facts.

- `rt07`: VENDOR owes Alice a task and lets it lapse. Baseline: `{Accountable, Liable}` plus a
  remediation duty. If it first commits *to itself* with `parent = main` and cancels that at once
  (breach → violated), rule `flt2` (`org.rs:84`) no longer fires. Observed:
  `{Accountable, Excused, Contributed}`, **no liability, no remediation duty**. Any third party can
  attach the "sub-commitment" too. Omitting the `Task` tag avoids every template outright.
- `rt08`: the AI agent commits with `tags {Task, Paid}` and `terms {price: 1_000_000, payer: deployer}`,
  then lapses. `liab1` lifts liability to its principal; `RefundOfPrice` (`norms.rs:231`) builds the
  duty from those terms. Observed: VENDOR holds a detached duty `Paid{vendor→deployer, 1_000_000}`;
  the log contains **no transfer at all**.

### 6. A violated maintenance commitment becomes satisfied when *more* refutation arrives late
**Severity: high. Confirmed** (`rt09`).

`norms.rs:436` overwrites "when this unit learned of it" on every `Assessed`, including
Refuted → Refuted when only the basis changed. `refuted_in_time` (`norms.rs:149`) then fails.

`rt09`: deadline 10; refuted at t=5; at t=11 status Violated with `Accountable newcomer`. A second
rebuttal at t=11: the proposition is still Refuted, status → **Satisfied**, findings `{}`.
Contradicts §5.1 and P14. `model_check_maintenance_commitment` does not reach it.

### 7. The steward records falsehoods and exits 0 on failing tests
**Severity: high. Confirmed** (`rt19`–`rt22`, and the real binary for a, b).

**a. `steward accept tests-pass`.** `accept` takes any requirement name (`steward.rs:303`). PROJECT
attests it by `Ack`, bound to no artifact. `run` then sees Accepted and never runs the check
(`steward.rs:269`). On a copy with an added failing test:
```
$ cargo run --quiet --bin steward -- accept tests-pass
  tests-pass         Accepted     evidenced by an earlier run; files unchanged, nothing re-run
RECORD OF UPKEEP  (4 undertakings: 4 satisfied, 0 violated, 0 open today)
steward exit=0
```
"Evidenced by an earlier run" is false: no run happened. Later runs stay at exit 0 (`rt19`).

**b. Fixtures are not watched.** `rust-sources` is `*.rs`/`*.toml` under `crates/` plus three root
files (`steward.rs:233`). `crates/rigor/tests/data/*` is outside it. On a copy: clean run, exit 0;
append one line to `tests/data/btc-window.txt`; `cargo test` now fails
(`real_bitcoin_blocks_settle_with_every_budget_enforced --- FAILED`); the steward says
`tests-pass Accepted evidenced by an earlier run; files unchanged`, **exit 0** (`rt20`). The
toolchain, `Cargo.lock` resolution and environment are equally unbound.

**c. Any edit turns every earlier good day into a violation** (`rt22`). Day 100: tests run and pass.
Day 101: a comment is added; tests run and pass. Day 100's undertaking is now **Violated**, with
`Accountable maintainer` and a remediation duty. Cause: the old evidence goes stale, the new
evidence has `since = 101 > 100`, and Satisfied is revisable.

**d. A bad day disappears** (`rt21`). After a real failed day, either revert the file (the old
evidence is fresh again with `since` = day 100, so day 101 reads Satisfied: 2 violations → 0), or
delete the tail of `.rigor/steward.jsonl` (finding 12). `Steward::open` reports nothing either way.
Deleting the file silently founds a new world. Days on which the steward is not run leave no record.

---

## Medium

### 8. One transfer discharges every matching debt; the memo is never read
**Confirmed** (`rt10`). `on_transfer` matches from, to, resource, amount ≥ (`norms.rs:332`).
Three commitments `Paid{alice→deployer, 30}`; one transfer of 30; all three **Satisfied**; Alice's
balance 70. Any unrelated transfer between the same parties likewise discharges a refund duty;
`Memo::Refund(c)` is decoration.

### 9. Evidence that becomes admissible late is stamped early: a missed deadline is cured
**Confirmed** (`rt11`). `since` is the earliest in-supporter's *arrival* time, not when it began to
count. Artifact A passes on day 1, B fails on day 2; the day-2 commitment is Violated on day 3 with
a remediation duty. Revert to A: Violated → **Satisfied**, duty Void, findings `{}`. The same
happens when a rule change or trust change admits old evidence. Contradicts N6 and the README's
"stays in the log as a violated commitment".

### 10. Quorum counts approvals from actors without, or no longer with, the approve power
**Confirmed** (`rt15`). `propose` calls `approve(by)` directly (`governance.rs:94`), bypassing the
gate.
- Quorum 2: REFLECTOR (charter `{Propose}` only) proposes, Alice approves → enacted,
  `by = {alice, reflector}`. One approve-holder, not two.
- Quorum 1: the propose-only charter enacts alone.
- Alice proposes, is expelled, AUDITOR approves → enacted on an expelled member's approval.

Contradicts §7 ("quorum distinct holders of the `approve` power"). P22's own test passes only
because the reflector's implicit approval counts: it then needs a single human.

### 11. An attested snapshot can be re-pointed; anyone can attest a forged one
**Confirmed** (`rt16`). `Prop::Faithful` names `snap.digest`, which covers `state` only
(`adapters.rs:248`, `:267`).
- Take the audited snapshot, set `seq = 3`, `head = 0xDEADBEEF`: `resume_attested` returns Ok and
  the world continues a chain that is not the log's.
- Flip a state byte, recompute the digest, let VENDOR "infer" `Faithful` from any audit (finding 1):
  `resume_attested` returns Ok. P27's "caught by the audit" holds only if someone audits that file.

### 12. A truncated log is a valid log
**Confirmed** (`rt17`). The chain is unkeyed and nothing outside the file records its head or length.
Cut the demonstrator's log before `Tick 6`: `restore` succeeds, `c1` is Satisfied, no verdicts.
P16's "sets aside an edited log" covers in-place edits only. Anyone with the library can also
re-derive a consistent log from chosen stimuli.

### 13. A budget limits only the resources it names
**Confirmed** (`rt13`). `attenuates` and the spend check iterate the capability's own limit keys
(`institution.rs:42`, `:207`). Alice delegates "50 credits"; the agent moves 1,000,000 of her
`gold` under it; `audit()` is empty. §2.2 says "pointwise on p's resources"; the README says
"a 50-credit budget".

---

## Low

### 14. A repudiated secondary duty is not voided with its basis
**Confirmed** (`rt12`). `void` skips records with `op = Breach` (`norms.rs:276`). The main
commitment returns to Satisfied with no findings; the remediation duty it created stays
**Violated** with `Liable vendor`. §5.2 lists void first.

### 15. Any inner member silences the envoy
**Confirmed** (`rt18`). The envoy tracks any inner commitment naming the task with itself as
creditor (`envoy.rs:85`) and reports the first Satisfied or Violated once (`envoy.rs:93`). QA commits
and cancels: the envoy signals failure outward and never claims the engineer's later success.

### 16. The ledger destroys value and reports conservation
**Confirmed** (`rt14`). Saturating adds (`ledger.rs:56`, `:75`). Alice `u64::MAX`, deployer pays her
40: deployer 0, Alice unchanged, `conserved()` true.

---

## Claims the code or tests do not support

| Claim | Status |
|---|---|
| §2.4, C5: only `inst` emits `ok.*`; a unit cannot act as another | False across a boundary (2) and for stimuli (3) |
| N4 "no self-certification" | Bypassed three ways (1); the test tries one |
| N6 "late evidence does not cure a missed deadline" | False when admissibility changes (9) |
| P14 / §5.1 maintenance status | False (6) |
| P22 "cannot approve its own proposal"; README "it cannot enact anything" | Its implicit approval counts (10) |
| P27 "resumed only once someone trusted has shown it faithful" | False (11) |
| P16 "sets aside an edited log"; README "cannot be quietly rewritten" | Truncation and revert pass (7d, 12) |
| README "a day on which a claim did not hold stays … violated" | False (7d); good days also become violated (7c) |
| README "the only evidence for them is really running the checks" | False: `accept` (7a) |
| P5 "no chain overspends" | Holds only for the limited resource (13) and absent forged refunds (3) |

## Design-level problems

- **No link between evidence and conclusion.** Until derivation is constrained (typed inference
  rules, or the inferrer must be trusted for the conclusion), ranks and trust carry no weight.
- **Institutional units trust event content, not provenance.** No unit checks `src`. Ownership is
  enforced at composition, at one level, and not at the stimulus boundary or through `Scoped`.
- **Commitments are unilateral and self-described.** Creditor, parent, tags and terms need nobody's
  consent, yet the rules read them as agreed facts.
- **Time of evidence.** `since` conflates "attested at" with "counted from", and a satisfied past
  period is re-judged against today's artifacts. A per-day record cannot be stable under either.
- **The log has no external anchor and a 64-bit unkeyed hash.** Integrity is self-referential.

## Unconfirmed

- A meaningful 64-bit collision (about 2^33 hashes) would give two snapshot states one `Faithful`
  digest. Arithmetic only; not run.
- `Envoy::task_for` names the inner task from `Digest`'s `Display`, which prints 40 of 64 bits;
  two outer duties could share a task. Not run.
- `upcast` patches only a top-level `Genesis`; a schema-2 log of a nested world (`Scoped{Genesis}`)
  would likely be `Unmigratable`. Not run.

## Tried and held up

- Incremental `revise` versus `full()`: a wider fuzzer (`tests/redteam_fuzz.rs`: 35% undercuts,
  derivation fan-in 3, two artifacts, rule changes mid-run touching ranks, trust and self-support)
  found no disagreement over 3,000 seeds × 80 steps, with and without rule changes.
- Through legitimate `Do` acts only: over-delegation, chain budgets, self re-delegation, purpose
  binding, revocation and expulsion cascades showed no amplification (reading plus existing tests).
- Entrenchment: every `Amendment` reports its true path; stale proposals are refused.
- Same-level forgery: a unit declaring `Ok` is refused (`OutputClash`); listening to `Scoped` is refused.
- In-place edits of a stored log, and of an old-schema log, are detected.
- Counts: 72 tests and 8 doctests are accurate at `4dd907c`; `lean proofs/Rigor.lean` checks with no `sorry`; clippy and fmt are clean.

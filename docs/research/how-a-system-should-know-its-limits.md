# how-a-system-should-know-its-limits

**Question.** How should a system that governs work represent what it cannot do, what it would need in order to do it, and what it must never do, so that the representation stays true as the system changes?

*Drafted by a language model and accepted by a second, different model acting as reviewer. It is an answer the project has adopted as a working position, not established fact.*

## Separate the three kinds, because they behave differently

- **Capabilities** decay: code changes, so a capability claim is only as good as its last verification.
- **Lacks** are cheap to re-check: either the missing thing exists or it doesn't.
- **Prohibitions** are the opposite of capabilities: they persist until a named authority explicitly lifts them, and they must never silently lapse.

The representation should not be one list with flags. It should be three record types with different rules for how each stays true.

## Attach to every entry: evidence, condition, authority

- A capability entry names the check that demonstrates it (the test, lint, or proof) and the last state where that check passed. Unverified capability claims should be treated as absent.
- A lack entry names the missing thing and its *kind*: a decision not yet made (only the owner can supply it), an artifact not yet built (someone must build it), or a boundary by design (no condition would satisfy it — "establish-truth" belongs here, not among ordinary lacks, because nothing could fill it).
- A prohibition entry names the authority that imposed it and the exact act that would remove it. "Cannot change-itself-unsupervised" should record that only a recorded owner decision in favour of unsupervised changes lifts it — and that such a decision is itself a change to the system's governance, subject to the same approval rules.

## Make the record checked, not prose

- Capabilities: re-run the evidence on every change to the governed things; a failed check demotes the entry automatically.
- Lacks: check mechanically where possible (does the tool exist on this machine? is the credential configured?). A lack that becomes fillable should be detected, not discovered.
- Prohibitions: enforce them in the code path that would violate them, so the record and the enforcement cannot drift apart. A prohibition that exists only as a written claim is a wish, not a constraint.

## Put the record under its own governance

The record of what the system can and cannot do is itself a thing the system manages. Changes to it — adding a capability, filling a lack, lifting a prohibition — must go through the same propose-verify-approve path as any other change. Otherwise the system could quietly edit its own account of itself, which is exactly what the unsupervised-change prohibition exists to prevent.

## Stamp everything with state

Every entry carries the version (commit or equivalent) at which it was last verified or imposed. A claim without a stamp is stale by default.

## What I am unsure of

Whether capability evidence can be re-run cheaply enough at every change, or whether sampling is needed. And whether "lacks by design" should be structurally distinct from ordinary lacks or merely annotated — I have treated them as distinct, but a simpler single type with a "no satisfying condition" marker may be more robust.

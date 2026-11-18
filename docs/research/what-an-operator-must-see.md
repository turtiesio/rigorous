# what-an-operator-must-see

**Question.** A person is responsible for a system that largely runs itself and records its own obligations, evidence, abilities and limits. What must one screen show that person so they can tell in a minute whether anything needs them, and what must it never hide?

*Drafted by a language model and accepted by a second, different model acting as reviewer. It is an answer the project has adopted as a working position, not established fact.*

## The one-minute test

The screen answers one question: **does anything need me, yes or no?** It should be organised so that answer is visible before any detail:

- **Waiting on you.** Every item whose next step is a human decision — approvals, rejections, choices between options. If the system has proposed changes, they appear here with what was verified about them. If nothing is waiting, the screen says so plainly, with the date it last said so.
- **Broken or stale.** Anything the system claims it can do but whose latest run failed, or hasn't run within its expected interval. A check that silently stopped running is the classic failure mode of self-governing systems; the screen must show "last ran X, expected within Y", not just a green tick from the last successful run.
- **Blocked by its own limits.** Work the system could not do because of something it records it cannot do. This is where the system's CANNOT entries become operational: if a task stalled for lack of a capability or a missing owner decision, that shows here, with the specific lacking item named.
- **Recent self-activity.** A short log of what it did on its own authority (checks, proofs, drafts) versus what it did with your approval. The distinction must be visible at a glance, because that distinction is the whole governance model.

Everything else — histories, evidence archives, model consultations — belongs behind one click, not on the screen.

## What it must never hide

1. **Its own recorded limits.** The CANNOT entries are part of the system's account of itself, and the screen must surface the ones currently affecting work. It must never present a limit as satisfied, or omit a limit because it is inconvenient to the current task.
2. **Absence of evidence.** "No evidence recorded" must look different from "evidence recorded and passing". An empty result is a result.
3. **The evidence/truth boundary.** The system records that it assesses evidence, never establishes truth. The screen must not convert "checks passed" into "this is correct". Labels should say what was checked, by what means, when.
4. **Its own uncertainty.** Where a claim rests on a model consultation or a review rather than a check or proof, that provenance must show. Weaker evidence gets weaker labels.
5. **Failures and reversals.** Anything that failed, was rolled back, or contradicts an earlier record stays visible. No silent overwriting.
6. **The cost of running.** If the system spends a budget on your behalf (for example, consulting models), the screen shows what was spent. You cannot govern what you cannot see the cost of.

## What I am unsure of

The exact thresholds for "stale" and the ranking of items within "waiting on you" depend on how often the system runs and what its obligations actually are; those must come from the system's own records, not from this answer.

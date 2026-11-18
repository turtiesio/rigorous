# managing-work-beyond-software

**Question.** Which practices for managing software work (review, tests as evidence, staged rollout, incident review, change approval) carry over to work that is not software, such as procurement, contracts or operations, and which do not? Be specific about what replaces a test.

*Drafted by a language model and accepted by a second, different model acting as reviewer. It is an answer the project has adopted as a working position, not established fact.*

## What transfers well

**Review.** A second reader catching errors before commitment is universal: redline review of contracts, a second evaluator scoring procurement bids, peer review of an operations procedure. The software practice of "review the diff, not the whole thing" also transfers — review the changed clauses, the changed steps, not the entire document each time.

**Staged rollout.** Pilot before fleet-wide, trial order before framework agreement, one site or one region before all, parallel running (old and new process side by side) before cutover. This is arguably stronger outside software because reversibility is worse, so staging matters more.

**Incident review.** Blameless postmortems, timelines reconstructed from records, and corrective actions with owners transfer directly. The main weakness: outside software there is often no automatic log, so the timeline depends on recollection unless record-keeping is deliberately built in.

**Approval before apply.** Signature gates, purchase authorization, change advisory for operational procedures — these already exist in non-software work and map cleanly onto your apply-changes-with-approval rule.

## What transfers only in weakened form

**Tests as evidence.** A software test is cheap, fast, repeatable, and automated; you can run it after every change. Non-software artifacts are usually one-off, expensive, and not re-runnable, so the *evidence-recording* part transfers but the *continuous re-running* part does not. What replaces a test, specifically:

- *Procurement:* inspection of samples, a trial order, reference checks, benchmarking a bid against the written specification, acceptance criteria checked at delivery.
- *Contracts:* a clause-by-clause checklist against requirements; a dry run of the obligations (who does what in month one); and — the strongest analogue — writing acceptance tests *into* the contract itself, so delivery is checked against pre-agreed criteria rather than judgement after the fact.
- *Operations:* tabletop exercises, drills, shadow runs, and parallel running. A drill is a test of a procedure; it is slower and less repeatable than a unit test, but it is the same idea: execute the artifact against expected behaviour and record the result.

**Sandbox verification.** You cannot execute a contract or a procurement decision in isolation. The nearest equivalents are drafts, redlines, simulations, and pilots — all of which are costlier and less faithful than a sandbox.

## What does not transfer

- **Continuous integration.** A contract may govern ongoing obligations (some involve only discrete ones), but the real difference is that software checks are re-run on every change, while checks on a contract are tied to its obligations, milestones or events (at signature, at delivery, at renewal).
- **Machine proofs.** There is no meaningful formal-verification analogue for most contracts or procurement decisions; formal analysis of contract text exists but is immature. Treat review and checklists as the ceiling.
- **Small, frequent, reversible changes.** Software improves by many small reversible steps. Contracts and major purchases are large, infrequent, and hard to reverse, so the workflow must front-load verification rather than rely on rollback.

## Uncertainty

How well blameless incident review survives where accountability is legal rather than technical is unclear — blamelessness may conflict with contractual liability. And counterparties may simply refuse pilots or staged commitments, limiting rollout staging for contracts.

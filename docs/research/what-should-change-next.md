# what-should-change-next

**Question.** Given this system's own account of what it can and cannot do, and its open obligations, what are the three most valuable changes to make next? For each give a one-line reason and what evidence would show it is done.

*Drafted by a language model and accepted by a second, different model acting as reviewer. It is an answer the project has adopted as a working position, not established fact.*

## Three changes, in priority order

**1. Build the code-to-model proof chain (install RustVerifier, write the connecting proof)**

Reason: every other capability rests on the model being trustworthy, and right now the proved model and the running code are connected only by hope — this is the largest gap between what the system claims and what it can show.

Evidence done: a machine-checked artifact, produced by running RustVerifier on this machine and recording its output, that states and proves the property "code implements model" for at least one non-trivial component; the check-itself run log shows this proof passing alongside the existing model proofs. Partial credit if it covers one module; done means the main state machine or change-verification path is covered.

**2. Build controlled editing of existing files**

Reason: an append-only change mechanism means the system can propose and verify changes it can never actually apply to anything that already exists, which quietly makes propose-changes, verify-changes, and apply-changes decorative for all real work.

Evidence done: a change to an existing file goes through the full pipeline — proposed as a request, verified in sandbox with checks passing, applied with the owner's approval — and the version-control history afterwards shows the old content recoverable and the diff exactly matching what was proposed. The CANNOT line for edit-existing-files disappears from the system's own account.

**3. Get the owner's decision on the forge credential, then build the review/check-run crawler**

Reason: read-histories currently sees only what commit logs say, not the review discussion and check results that are the real record of how decisions were made, so the system's evidence base is systematically thinner than the available record.

Evidence done: first, a recorded owner decision either way on use-the-owners-forge-credential (a refusal is also progress — it closes the question). If granted: a crawl of one real project that returns review and check-run counts matching the forge's own API totals for the same query, recorded alongside the crawl. If refused: the CANNOT line is updated to reflect the decision rather than left ambiguous.

## What I would not do next, and why

- **Unsupervised changes**: the owner's standing refusal is doing real work; removing it adds risk before the proof chain (item 1) exists to constrain what gets applied.
- **Multi-machine consistency**: valuable once the system runs in more than one place, but it solves a problem the single-machine system does not yet have.

## What I am unsure of

- The ranking of 2 vs 3 depends on how much of the system's actual work is changing files versus reading histories; if it is mostly observational today, swap them.
- I cannot tell from the account given how large the code-to-model gap is — if the code is small, item 1 may be cheaper than assumed; if large, it may need to be staged, and "done" should then be defined per stage.

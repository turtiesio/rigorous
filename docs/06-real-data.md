# Stress against real data

The system was fed three real, public git histories in full, twice: once with a first mapping, and again
after fixing what the first run exposed. This document records what the data contained, what the system
concluded, what broke, what was changed, and what is still not handled.

## What was run

```sh
git clone --bare --filter=tree:0 https://github.com/git/git.git               # commits only
git clone --bare --filter=tree:0 https://github.com/nodejs/node.git
git clone --bare --filter=tree:0 https://github.com/kubernetes/kubernetes.git
cargo run --release --bin stress -- git.git node.git kubernetes.git
```

Histories were read on 2026-11-18 at their then-current heads, merge commits included; the counts will drift
as the projects grow. People are pseudonymised at the boundary: an actor id is a digest of the (mailmap-
resolved) e-mail address, and no name or address enters the log. Findings are reported only in aggregate.

A 4,000-commit slice of the Node.js history is checked into the repository in that reduced, pseudonymous form
(`crates/rigor/tests/data/node-window.txt`) so that real data is also part of `cargo test`.

## The mapping (`crates/rigor/src/gitlog.rs`)

| In the history | In the model | Exercises |
|---|---|---|
| A new author whose first change someone else committed | Admitted by that committer (their sponsor), under admission authority **delegated** by the registrar | admission, delegation chains |
| A new author who commits their own first change; a new committer | Admitted by the registrar | admission |
| Each commit | The author undertakes a **maintenance** commitment: the change stays unrefuted for 30 days from when the record arrives | maintenance norms, deadlines |
| Committer differs from author | Independent acknowledgement (supporting evidence) | evidence, self-support rule |
| `This reverts commit <hash>` | Rebutting evidence against that change, plus an undercut of every piece of evidence the reverted commit had itself produced | argumentation, reinstatement |
| The project's rule set | Amended once at the end, by its own procedure (propose + second approval) | governance, mass reassessment |
| Three projects | Run alone, and as three separately bounded organisations in one world, interleaved by time | nesting |

These are modelling choices, not facts read off the data. "A distinct committer sponsors a newcomer" and
"a revert is a refutation" are defensible readings of git practice; they are assumption A-ADAPTER.

## Results

| | git | Node.js | Kubernetes | Total |
|---|---|---|---|---|
| Commits ingested | 83,045 | 49,568 | 143,935 | 276,548 |
| Events recorded | 1,118,058 | 699,809 | 1,843,511 | 3,661,378 |
| Members admitted at run time | 2,517 | 4,522 | 5,827 | 12,866 |
| …by a sponsor, under delegated authority | 2,469 | 4,485 | 849 | 7,803 |
| Sponsors given delegated authority | 42 | 168 | 769 | 979 |
| Capabilities in existence at the end | 5,086 | 9,222 | 12,433 | |
| Commits with an independent acknowledgement | 49,034 (59%) | 27,534 (56%) | 49,962 (35%) | |
| Reverts declared | 174 | 466 | 545 | 1,185 |
| …that resolve to no single earlier commit | 3 | 30 | 14 | 47 (4%) |
| …that revert a revert | 0 | 12 | 32 | 44 |
| …by the original author | 72 | 141 | 163 | 376 |
| Commits whose clock is behind one already seen | 1 | 176 | 713 | 890 |
| Commits backdated by more than the 30 days | 0 | 10 | 0 | 10 |
| Acts the institution refused | 0 | 0 | 0 | 0 |
| **Changes the system ends up holding refuted** | 171 | 415 | 466 | 1,052 |
| Commitments violated (refuted within the 30 days) | 78 | 321 | 362 | 761 |
| Ingestion time (release build) | 8.4 s | 5.5 s | 14.5 s | |
| Throughput (events per second) | 133,000 | 127,000 | 127,000 | |

## Checks made on every run

| Check | git | Node.js | Kubernetes |
|---|---|---|---|
| The set of refuted changes equals an **independent oracle** (a different algorithm: plain recursion on "is this commit still in effect") | equal, 171 | equal, 415 | equal, 466 |
| Incrementally maintained assessments equal a from-scratch recomputation | 49,078 propositions | 27,749 | 50,381 |
| Every act was authorised: nothing denied, nothing rejected | holds | holds | holds |
| No capability amplifies its parent | holds | holds | holds |
| Log hash chain verifies | holds | holds | holds |
| Replaying only the external stimuli reproduces the log bit-for-bit | 8.4 s | 4.5 s | 14.5 s |
| **Snapshot** taken half-way, resumed from the file, second half ingested: state and chain head equal the uninterrupted run | equal | equal | equal |
| Snapshot size / time to resume | 34.5 MB / 0.96 s | 19.4 MB / 0.19 s | 49.9 MB / 0.42 s |
| **Governance**: the project amends its evidence rules (an acknowledgement now outranks a reversal); refuted changes | 171 → 44 | 415 → 201 | 466 → 400 |
| …violated commitments, and verdicts, revised with it | 78 → 27 | 321 → 162 | 362 → 308 |
| …and incremental assessment still equals from-scratch afterwards | holds | holds | holds |
| **Nested**: as one of three organisations in one world (3,661,378 events, one hash chain), its conclusions equal the oracle and equal its own run alone | equal | equal | equal |
| **Reflection**: share of acknowledgements later overturned, as recorded by the reflector | 127 of 49,034 (0.26%) | 217 of 27,537 (0.79%) | 71 of 49,967 (0.14%) |
| …rule changes the reflector proposed on that record (threshold 5%) | none | none | none |

The oracle is evidence, not proof: it was written by the same author from the same reading of what a revert
means. What it rules out is a bug in the argumentation, incremental-revision or event-propagation machinery
that the simple recursion does not share. The amendment is a constructed decision applied to the real record:
it shows that governance works at this scale, not that any project would want that rule.

## What broke, and what was changed

1. **Throughput collapsed as the world grew.** The first full run of the git history took 132 s, each
   successive 10,000 commits slower than the last (5 s, 10 s, 16 s, 23 s, 33 s, 37 s). Every settled cascade
   re-judged the touched family, building a fact for *every member of the institution*, although nothing was
   violated. Fixed (judge only families with a violation or a standing verdict; facts only about actors the
   family mentions; facts indexed by predicate; interfaces computed once): 6–8 s at a steady rate. Pinned by
   `judging_costs_nothing_until_something_is_violated_whatever_the_membership`.
2. **Members could not join.** Added `Act::Admit`, a power held by charter and delegable.
3. **There was no way to say "this must stay true".** Added the maintenance condition `Condition::Unrefuted`.
4. **Restart cost grew with history.** Added snapshots, resume and compaction: restoring a world of ~900,000
   events takes under a second from a 50 MB file instead of 15 s of replay, and a resumed world need not hold
   earlier entries.
5. **A stored log died with any change of format.** The steward's own first log could not be read after the
   format gained a header. Added chain verification from the stored bytes (independent of the event
   vocabulary) and migration; the steward migrated its real log and kept its history.
6. **Backdated records were refused.** Ten Node.js commits were dated more than 30 days behind the clock, so
   their commitments' deadlines were already past. Policy now: the period runs from arrival.
7. **11% of reverts could not be resolved; one person was several members.** Merge commits are now ingested
   and the repository's mailmap is applied: unresolved reverts fell to 4%, and distinct members from 13,341
   to 12,866.

## What the data showed that the system still does not handle

| Observation | Consequence | Status |
|---|---|---|
| Review and CI evidence lives in the forge (pull requests, check runs), not in git | "Independent acknowledgement" here is only "someone else committed it"; 35–59% of commits have even that | open: forge adapter (roadmap) |
| No real data with resource transfers or spending limits was available | Delegation and admission ran on real data; **transfers, budgets and conservation are still exercised only on constructed scenarios and randomised tests** | open (roadmap) |
| 47 reverts still name a hash that is ambiguous or not in the history | Those refutations are absent from the model; counted, not resolved | open |
| A snapshot is trusted | Its digest shows it is uncorrupted, not that it is the state its log produces; only a full replay shows that | open (roadmap) |
| Peak memory 4.7 GB for the 3.7-million-event federation run (~1.3 KB per event held) | Compaction exists and is tested, but the stress binary keeps the whole log in order to replay it | by choice in the stress run |
| Nested organisations ran side by side, never interacting | Nesting is shown to be behaviour-preserving at scale; one organisation acting as a member of another is still not built | open |
| A change reverted after its 30 days leaves its commitment satisfied | Stated policy; the 30 days are arbitrary | by design |

## What this stress test does not establish

A git history is one kind of real-world record. Nothing here says how the model fares on contracts, payments,
incidents or regulation. The mapping from commits to commitments is an interpretation. And the checks compare
the system with itself (replay, from-scratch recomputation, snapshot, solo versus nested) and with one oracle
written alongside it; none of them is an independent party's judgment of the same data.

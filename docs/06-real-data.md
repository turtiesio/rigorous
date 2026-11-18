# Stress against real data

The system was fed three real, public git histories in full, twice: once with a first mapping, and again
after fixing what the first run exposed. This document records what the data contained, what the system
concluded, what broke, what was changed, and what is still not handled.

## What was run

```sh
git clone --bare --filter=tree:0 https://github.com/git/git.git               # commits only
git clone --bare --filter=tree:0 https://github.com/nodejs/node.git
git clone --bare --filter=tree:0 https://github.com/kubernetes/kubernetes.git
cargo run --release --bin stress -- git.git node.git kubernetes.git           # each alone, then together
cargo run --release --bin stress -- --federate-only git.git node.git kubernetes.git
cargo run --release --bin stress -- --chain crates/rigor/tests/data/btc-window.txt
```

Histories were read on 2026-11-18 at their then-current heads, merge commits included; the counts will drift
as the projects grow. People are pseudonymised at the boundary: an actor id is a digest of the (mailmap-
resolved) e-mail address or of the Bitcoin output script, and no name, address or script enters the log.
Findings are reported only in aggregate.

Three slices of this real data are checked into the repository in reduced, pseudonymous form, so that real
data is part of `cargo test`: 4,000 Node.js commits rich in reverts, 1,500 Node.js commits carrying review
trailers, and 22 Bitcoin blocks (`crates/rigor/tests/data/`).

## Part 1: three git histories

### The mapping (`crates/rigor/src/gitlog.rs`)

| In the history | In the model | Exercises |
|---|---|---|
| A new author whose first change someone else committed | Admitted by that committer (their sponsor), under admission authority **delegated** by the registrar | admission, delegation chains |
| Each commit | The author undertakes a **maintenance** commitment: the change stays unrefuted for the period the project's rules currently prescribe, counted from when the record arrives | maintenance norms, deadlines, rules read at run time |
| Committer differs from author | Independent acknowledgement (supporting evidence) | evidence, self-support rule |
| `Reviewed-by:` / `Acked-by:` trailers | Each named reviewer is admitted and attests an acknowledgement | evidence from several sources |
| `This reverts commit <hash>` | Rebutting evidence against that change, plus an undercut of every piece of evidence the reverted commit had itself produced | argumentation, reinstatement |
| The record of how late refutations arrive | The reflector proposes a longer maintenance period; the project approves; later commits use it | **the rules revised from the record** |
| The project's rule set | Amended once more at the end (an acknowledgement outranks a reversal) | governance, mass reassessment |
| Three projects | Run alone, and as three separately bounded organisations in one world, interleaved by time | nesting |

These are modelling choices, not facts read off the data: assumption A-ADAPTER.

### Results

| | git | Node.js | Kubernetes | Total |
|---|---|---|---|---|
| Commits ingested | 83,045 | 49,568 | 143,935 | 276,548 |
| Events recorded | 1,142,319 | 1,317,292 | 1,843,585 | 4,303,196 |
| Members admitted at run time | 2,609 | 4,673 | 5,827 | 13,109 |
| …by a sponsor, under delegated authority | 2,460 | 4,473 | 849 | 7,782 |
| Capabilities in existence at the end | 5,270 | 9,524 | 12,433 | |
| Commits with a distinct committer | 49,034 | 27,534 | 49,962 | |
| Commits with reviewers named in trailers | 4,441 | 37,606 | 0 | |
| Reviews attested | 4,753 | 123,359 | 0 | 128,112 |
| Reverts declared | 174 | 466 | 545 | 1,185 |
| …that resolve to no single earlier commit | 3 | 30 | 14 | 47 (4%) |
| …that revert a revert | 0 | 12 | 32 | 44 |
| Commits whose clock is behind one already seen | 1 | 176 | 713 | 890 |
| Acts the institution refused | 0 | 0 | 0 | 0 |
| **Changes the system ends up holding refuted** | 171 | 415 | 466 | 1,052 |
| Ingestion (release build) | 11.2 s | 13.1 s | 18.8 s | |
| Throughput (events per second) | 102,000 | 101,000 | 98,000 | |

### The system revising its own rule from the record

Each project starts with a maintenance period of 30 days. Its reflector records how long after a change each
refutation arrived, and when nine in ten of those observed would not have fallen inside the current period
(by a margin of a quarter) it proposes the period that would. The project's approval stands in for the human
vote. Later commits are undertaken for the new period.

| | git | Node.js | Kubernetes |
|---|---|---|---|
| Period over the history, in days | 30 → 353 → 464 | 30 → 41 → 59 → 84 | 30 → 38 → 48 → 61 |
| Refutations observed; nine in ten arrived within | 171; 359 days | 432; 98 days | 490; 76 days |
| Refutations that fell inside the period in force (violated commitments), with a fixed 30 days | 78 | 321 | 362 |
| …with the period the system learned | 133 | 346 | 382 |
| Acknowledgements later overturned (the reflector's other record) | 127 of 49,269 (0.26%) | 321 of 41,371 (0.78%) | 71 of 49,967 (0.14%) |
| Rank changes it proposed on that record (threshold 5%) | none | none | none |

What this shows and does not: the loop from record to proposal to enacted rule to changed behaviour runs on
real data, and the three projects end with very different periods because their records differ. It does not
show that the learned period is *right*; "nine in ten" and "a quarter" are fixed parameters of the reflector.

### Checks made on every run

| Check | git | Node.js | Kubernetes |
|---|---|---|---|
| The set of refuted changes equals an **independent oracle** (plain recursion on "is this commit still in effect") | equal, 171 | equal, 415 | equal, 466 |
| Incrementally maintained assessments equal a from-scratch recomputation | 49,313 propositions | 41,474 | 50,381 |
| Every act was authorised; no capability amplifies its parent; the hash chain verifies | holds | holds | holds |
| Replaying only the external stimuli reproduces the log bit-for-bit | 9.3 s | 11.0 s | 14.3 s |
| **Snapshot** taken half-way, resumed from the file, second half ingested: equals the uninterrupted run | equal | equal | equal |
| Snapshot size / time to resume | 37.4 MB / 0.38 s | 31.4 MB / 0.30 s | 54.0 MB / 0.58 s |
| **Governance**: an acknowledgement now outranks a reversal; refuted changes | 171 → 44 | 415 → 102 | 466 → 400 |
| …violated commitments and verdicts revised with it | 133 → 38 | 346 → 91 | 382 → 325 |
| …and incremental assessment still equals from-scratch afterwards | holds | holds | holds |
| **Nested**: as one of three organisations in one world, conclusions equal the oracle (and, in the run that also ran them alone, equal the solo run) | equal | equal | equal |

The three-organisation run recorded 4,303,196 events under one hash chain, verified stretch by stretch and
compacted as it went: 32,287 entries held at the end, peak memory 1.28 GB (4.7 GB before compaction was used).

The oracle is evidence, not proof: it was written by the same author from the same reading of what a revert
means. The final amendment is a constructed decision applied to the real record.

## Part 2: real transfers of value

No git history contains resources. For transfers, spending limits and conservation the system was run on 22
consecutive real Bitcoin blocks (heights 150000–150021, April 2011), fetched from a public block explorer.
A larger window was attempted and abandoned when the explorer stopped serving one block's transactions.

**The mapping** (`crates/rigor/src/chain.rs`). A signed input is a *delegation*: the owner grants a settler
the power to transfer from the owner's account, limited to exactly the input's amount and bound to that one
transaction. The settler moves the value to the outputs under those capabilities, and the remainder (the fee)
to a treasury; new coin enters only as the treasury's payment of each block's reward. Since the real network
accepted every one of these transactions, the institution must refuse none — and must refuse a replay.

| | |
|---|---|
| Blocks / transactions | 22 / 938 |
| Accounts | 2,365 (2,364 admitted at run time) |
| Purpose-bound, limited delegations granted | 1,359 |
| Transfers made under them | 2,532 |
| Value moved | 53,886.8 BTC |
| Transfers the institution refused although the network had accepted them | **0** |
| Replays attempted after settlement (one more unit under a spent capability) / refused | 916 / **916** |
| Uses of a capability for another purpose attempted / refused | 916 / **916** |
| Every delegated capability spent to exactly its limit | holds |
| Final balance of every account equals plain bookkeeping (opening + received − spent) | holds |
| Conservation; no capability amplifies its parent; chain verifies; replay reproduces the log | holds |

The first comparison with the bookkeeping oracle failed on one account. The fault was in the oracle (it
processed a block's reward before the fees that fund it and clamped at zero), not in the system; it is
recorded here because a check that disagrees is only useful if one finds out which side is wrong.

This is 22 blocks, not a chain: it shows the ledger and authority units are right on real transfers with real
amounts, not that they scale, and "a signature is a bounded delegation" is again a reading, not a fact.

## What broke, and what was changed

1. **Throughput collapsed as the world grew** (132 s for the first git run, slowing with every 10,000
   commits): each settled cascade re-judged the touched family with a fact for every member. Fixed; pinned by
   `judging_costs_nothing_until_something_is_violated_whatever_the_membership`.
2. **Members could not join.** Added admission by a delegable charter; later, expulsion and role change.
3. **There was no way to say "this must stay true".** Added the maintenance condition.
4. **Restart cost and memory grew with history.** Added snapshots, resume and compaction; a snapshot can now
   be audited by replay and resumed only if that audit is accepted evidence.
5. **A stored log died with any change of format.** Added chain verification from stored bytes and migration.
6. **Backdated records were refused.** The period now runs from arrival.
7. **11% of reverts could not be resolved; one person was several members.** Merges ingested, mailmap applied.
8. **Review evidence was invisible.** `Reviewed-by` and `Acked-by` trailers are now read: 128,112 reviews.
9. **The 30-day period was arbitrary.** It is now a rule, revised from the record through governance.
10. **A capability could not be tied to one transaction.** Added purpose-bound scope; needed for real transfers.
11. **Those very changes orphaned the steward's own log.** New fields made its stored events unparseable;
    it set the log aside and started again. The schema was bumped, the upgrade step written, the log
    recovered by migration, and a test now fails if events change shape without a schema bump.

## What the data showed that the system still does not handle

| Observation | Consequence | Status |
|---|---|---|
| Kubernetes records review in pull requests, not trailers: 0 reviews seen | For such projects the only acknowledgement is a distinct committer | open: needs the forge's own data |
| Nested organisations ran side by side, never interacting | Nesting is shown to be behaviour-preserving at scale; one organisation acting as a member of another is not built | open |
| Only two kinds of real record so far (version-control histories, a ledger) | Nothing is known about contracts, incidents, regulation | open |
| The real ledger window is 22 blocks | Correctness on real transfers is shown; scale is not | open |
| 47 reverts still name a hash that is ambiguous or not in the history | Those refutations are absent; counted, not resolved | open |
| The reflector knows two kinds of improvement and has fixed thresholds | It is a mechanism, not a judge of what the right rule is | by design, for now |

## What this stress test does not establish

A git history is one kind of real-world record. Nothing here says how the model fares on contracts, payments,
incidents or regulation. The mapping from commits to commitments is an interpretation. And the checks compare
the system with itself (replay, from-scratch recomputation, snapshot, solo versus nested) and with one oracle
written alongside it; none of them is an independent party's judgment of the same data.

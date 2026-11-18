# How it runs itself

`cargo run --bin steward -- cycle` is one turn of the system looking after itself. Everything it does is an
act in its own log, under its own rules. This document says what a turn does, what the system knows about
itself, how it is changed, and what is the owner's alone.

## Who is in the room

| Member | What it is | What it may do |
|---|---|---|
| `owner` | The person the project answers to | Grant a budget, decide questions, approve or reject changes. Nothing else can |
| `maintainer` | Whoever does the work | Undertakes the daily upkeep and the roadmap items |
| `ci` | The steward's service account | Runs real checks and probes; pays for model calls out of the budget it was granted |
| `researcher` | A language model (GLM 5.3 Flash) | Drafts answers and revisions. What it says is a claim, never evidence |
| `reviewer` | A second, different model (GPT-6 Luna) | Reviews what the first drafted: acknowledges or objects |
| `gateway` | Where money for model calls goes | Receives payment |
| `project` | The project as a party | Is owed the undertakings |

The models are reached through the Vercel AI Gateway. The credential is looked for in `AI_GATEWAY_API_KEY`,
then in a key file (`RIGOR_KEY_FILE`, then `~/.pi/agent/auth.json`); it is read at the moment of use and is
never stored, logged or printed.

## One turn

1. **Check itself.** Tests, lints, format, proofs: really run, unless the files they are about are unchanged
   since evidence was last recorded. A day's lapse becomes a violated undertaking and a duty to put it right.
2. **Examine itself.** Every need of every ability is probed (is the program there, is there a credential,
   does the gateway answer, is there unspent budget, has the owner decided). What changed is recorded as
   evidence about "the system can do X".
3. **Research.** The next open question on its agenda is put to the researcher; the reviewer reads the
   answer; an objection goes back for up to two revisions. Each call is paid for through the ledger, under a
   capability the owner granted, so the limit is enforced by the same units that enforce any other budget.
4. **Propose.** Accepted answers become a change request. The request is verified in a sandbox copy of the
   repository with the project's own checks, then reviewed again as a change.
5. **Wait.** Nothing is applied. The dashboard says what is waiting for the owner.

## What it knows about itself

`cargo run --bin steward -- can`. Each ability is one of:

- **can**: every need is met, as of the last probe;
- **can, with the owner's approval each time**;
- **cannot**, with exactly what is lacking, and each lacking thing is of a stated kind: a program not
  installed, a credential, the gateway not answering, no budget, a decision the owner has not made in favour,
  another ability, something not built, or *by design never*.

It currently cannot, and says so: change its own code, proofs or rules (changes reach only research notes and
generated tests); apply anything without the owner; prove its code matches its proved model; crawl one
project's forge with a credential (the owner decided against using theirs); run across machines; establish
that anything is true.

## How it is changed

A change is a request before it is a fact (`crates/steward/src/change.rs`).

| Step | Who | What makes it count |
|---|---|---|
| Proposed | anyone, including a model | Identity = digest of the whole request. A stored request that no longer matches its identity is refused |
| Verified | `ci` | The project's checks, all of them, run in a sandbox copy with the change applied. A check not run establishes nothing |
| Reviewed | `reviewer` | Acknowledgement or objection. An empty or unclear reply is recorded as nothing, not as an objection. A re-review supersedes the earlier one |
| Approved | `owner` only | Outranks a reviewer's objection; does not outrank a failed verification |
| Applied | `ci` | Only if assessed sound, verified, approved, and the repository is still what it was verified against |

Never outweighed: a failed verification (not by a later pass, not by approval) and the owner's rejection (not
by a later approval).

A change may add a file or replace the content of an existing one, only under `docs/research/` or as a
generated test. An edit names the digest of the content it replaces and applies to exactly that content. An
edit that keeps fewer than half the original lines is refused outright.

**What real use showed** (2026-11-18, the first day):

- The reviewer rejected the first research answer's change request for over-claiming; the owner rejected it.
- A model asked to revise a document returned only the paragraph it had changed. The sandbox checks passed
  (they do not check documents) and the reviewing model accepted. The owner's inspection caught that the
  edit would have replaced the whole file with one line. The half-the-lines guard and the line counts shown
  to reviewer and owner exist because of this.
- The second attempt was a true one-line edit but over-corrected; the reviewer objected and the owner
  rejected it. The third was acknowledged, approved and applied.
- An empty model reply was at first recorded as an objection. It is now recorded as nothing.

## What is the owner's alone

```
steward grant-budget <millionths of a dollar>     set money aside for model calls
steward decide <requirement> yes|no               decide a question put to the owner
steward approve|reject <change id>                rule on a change
steward apply <change id>                         apply an approved change
steward accept <roadmap item>                     accept that an undertaking is done
```

In this session those commands were run by the assistant on the owner's standing instruction to act for
them. The log attributes them to `owner`; it does not distinguish the owner from someone acting for the owner.

## The dashboard

`.rigor/dashboard.html`, rewritten every run. Its order follows the system's own research on what an operator
must see (`docs/research/what-an-operator-must-see.md`): what needs the owner; a glance of counts and money
spent; each claim it makes about itself with the means and age of the evidence; what is stalled by one of its
own limits; what it can and cannot do; changes; research; undertakings; recent acts, marked as the owner's
decision or its own. No evidence is worded differently from evidence against, and a passed check is never
shown as "correct".

## What this does not do

It does not write or change code. It does not decide anything that is the owner's. Its research answers are
positions two models agreed on, not findings. Its checks do not cover the content of documents. And the
models are a trusted-for-nothing source: nothing they say is evidence until someone else, under the rules,
acknowledges it.

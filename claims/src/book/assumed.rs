//! Assumed: taken on trust from outside the system.

use crate::claim::{id, Claim, Support};

pub const CLAIMS: &[Claim] = &[
    Claim { id: id("a-genesis"), statement: "The genesis event (members, root capabilities, rules) is the intended constitution", support: Support::Assumed { breaks: "Everything; it is the only trust anchor" } },
    Claim { id: id("a-auth"), statement: "An external stimulus really comes from the actor it names", support: Support::Assumed { breaks: "All authority checks" } },
    Claim { id: id("a-order"), statement: "One total order of events exists (a single log)", support: Support::Assumed { breaks: "Revocation, budgets, deadlines: see N3" } },
    Claim { id: id("a-clock"), statement: "`tick` events are monotone and reflect real time", support: Support::Assumed { breaks: "Deadlines, expiry" } },
    Claim { id: id("a-hash"), statement: "SHA-256 is collision resistant; note ids are **truncated** to 64 bits", support: Support::Assumed { breaks: "Log integrity; identity of commitments/evidence at large scale" } },
    Claim { id: id("a-trust"), statement: "The `trusted` table names sources that report honestly for each method", support: Support::Assumed { breaks: "Every assessment" } },
    Claim { id: id("a-adapter"), statement: "Adapters report what they observed (file digests, exit codes, commit metadata), and the mapping they apply means what it claims (a revert is a refutation; a distinct committer is an acknowledgement)", support: Support::Assumed { breaks: "Conformance evidence; every conclusion drawn from real data" } },
    Claim { id: id("a-unit"), statement: "Each unit's transition function is deterministic and touches only its own state", support: Support::Assumed { breaks: "Replay, composition laws. State-locality is compiler-enforced (C10); purity is not: a unit could still read a clock or a file" } },
    Claim { id: id("a-rules"), statement: "The rule set expresses what the institution means", support: Support::Assumed { breaks: "Verdicts are sound for the rules, not for justice" } },
    Claim { id: id("a-obs"), statement: "\"Notified\" means a message was delivered, not that anyone understood it", support: Support::Assumed { breaks: "The `knew` finding" } },
    Claim { id: id("a-model"), statement: "Language models are trusted for nothing: what one says becomes a claim or a proposal by the member that asked, and counts only if another trusted party acknowledges it. The reviewing model IS such a party for research answers and change reviews, so two models agreeing can be wrong together", support: Support::Assumed { breaks: "Research answers; change reviews (not verification, not approval)" } },
    Claim { id: id("a-owner"), statement: "Acts recorded as the owner's were made by the owner or on their instruction. In the first session they were made by an assistant acting for the owner", support: Support::Assumed { breaks: "Budgets, decisions, approvals" } },
    Claim { id: id("a-snapshot"), statement: "For a plain resume: the snapshot file is the state its log produces. For an attested resume: the auditor who replayed is trustworthy", support: Support::Assumed { breaks: "Everything after a resume" } },
    Claim { id: id("a-tcb"), statement: "The waist (budgeted at 1,500 lines, P40), the reasoners, the vocabulary, the concepts and their wiring are correct", support: Support::Assumed { breaks: "Everything not independently proved" } },
    Claim { id: id("a-tools"), statement: "The Rust compiler, `serde`, `serde_json`, `sha2`, and the Lean kernel are correct", support: Support::Assumed { breaks: "Everything marked compiler-enforced or proved" } },
];

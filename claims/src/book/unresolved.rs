//! Unresolved: not established here.

use crate::claim::{id, Claim, Support};

pub const CLAIMS: &[Claim] = &[
    Claim { id: id("u1"), statement: "No proof connects the Lean model to the Rust code. A differential test against a transcription of the model does (P38), for `Par` and the scheduler only", support: Support::Unresolved },
    Claim { id: id("u2"), statement: "Termination of a cascade is not guaranteed for arbitrary units (only a fuel bound). Acyclic templates are *intended* to bound chains of secondary duties; that is not proved", support: Support::Unresolved },
    Claim { id: id("u3"), statement: "Soundness of the static impact cone is property-tested (P23), not proved", support: Support::Unresolved },
    Claim { id: id("u4"), statement: "Only the adjudicator waits for quiescence. Any other unit that reads several facts at once can still see a half-propagated state", support: Support::Unresolved },
    Claim { id: id("u5"), statement: "Commutativity of `Par` is tested, not proved. There is no static independence analysis and no partial-order reduction; independence is demonstrated per case", support: Support::Unresolved },
    Claim { id: id("u6"), statement: "No liveness property of any kind is established", support: Support::Unresolved },
    Claim { id: id("u7"), statement: "Refinement relations must be supplied by hand; there is no automatic refinement checker", support: Support::Unresolved },
    Claim { id: id("u8"), statement: "Distributed execution, log merging, and operation under partition are not addressed", support: Support::Unresolved },
    Claim { id: id("u9"), statement: "One organisation acting as a member of another is built and tested (P32) on a constructed case only; on real data organisations have only run side by side", support: Support::Unresolved },
    Claim { id: id("u10"), statement: "Uncertainty is ordinal (method ranks). No probabilities, no quantitative confidence", support: Support::Unresolved },
    Claim { id: id("u11"), statement: "Confidentiality: boundaries stop units *listening*, but the log is global and readable", support: Support::Unresolved },
    Claim { id: id("u12"), statement: "Members can be admitted, expelled and given roles at run time; units are still fixed at build time", support: Support::Unresolved },
    Claim { id: id("u13"), statement: "Goals and requirements exist only as propositions and commitment consequents; intentions are not modelled", support: Support::Unresolved },
    Claim { id: id("u14"), statement: "Propositions are opaque names: no entailment between them except explicit `derived_from`, and no time-indexed facts (redoing a task needs a new proposition)", support: Support::Unresolved },
    Claim { id: id("u15"), statement: "Rules of non-retroactivity are per family and coarse; evidence-rule changes are always retroactive", support: Support::Unresolved },
    Claim { id: id("u16"), statement: "The typed vocabulary is closed: a new kind of proposition, tag, method or predicate means editing an enum and recompiling. The kernel and reasoners are generic, so a second domain can bring its own event type, but two domains cannot yet be *combined* without a common enum. One concept (value) now states its own vocabulary and is wired in by translation (P43); the others still speak the shared one, and propositions are still one closed type", support: Support::Unresolved },
    Claim { id: id("u18"), statement: "A snapshot can be audited and resumed only on accepted evidence (P27), but the audit is a full replay by a trusted auditor: cost is not avoided, only moved, and plain `resume` still exists", support: Support::Unresolved },
    Claim { id: id("u19"), statement: "Migration replays old stimuli under today's units, so derived events may differ from the old log's; only the stimuli and the old chain head are carried over. Upgrade steps are written by hand for each change of shape; a guard test fails when the stored form of events changes without a schema bump, but nothing checks that a hand-written step preserves meaning", support: Support::Unresolved },
    Claim { id: id("u20"), statement: "Backdated records get their period from arrival: a policy, not a model of event time versus record time", support: Support::Unresolved },
    Claim { id: id("u21"), statement: "Identity merging is only what the project's own mailmap says", support: Support::Unresolved },
    Claim { id: id("u22"), statement: "Three kinds of real record have been tried (version-control histories, a ledger window of 86 blocks, a registry of 397 packages). Each adapter is an interpretation, and each oracle was written by the same author", support: Support::Unresolved },
    Claim { id: id("u24"), statement: "The system changes only research notes and generated tests. It cannot change its own code, proofs or rules under control, and there is deliberately no way to apply anything without the owner", support: Support::Unresolved },
    Claim { id: id("u25"), statement: "The dashboard is a static page rewritten each run; it could not be rendered for inspection in this environment (the preview tool failed), so its appearance is unverified", support: Support::Unresolved },
    Claim { id: id("u23"), statement: "The reflector's thresholds are fixed parameters and it knows two kinds of improvement (a method's rank; the maintenance period). It shows that rules can be revised from the record, not that the revision is right. In the stress runs the project's approval was automatic", support: Support::Unresolved },
    Claim { id: id("u17"), statement: "In-memory state digests (used only by the model checker) rely on `std::hash`, which is not stable across compiler versions; they are never persisted or compared across builds", support: Support::Unresolved },
];

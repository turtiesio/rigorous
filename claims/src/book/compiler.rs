//! Compiler-enforced: what cannot be written at all.

use crate::claim::{id, Claim, Support};

pub const CLAIMS: &[Claim] = &[
    Claim {
        id: id("c1"),
        statement: "An event of an unknown kind; a unit interface naming a kind that does not exist",
        support: Support::Compiler { how: "`Event` and `Kind` are closed enums; interfaces are `BTreeSet<Kind>`" },
    },
    Claim {
        id: id("c2"),
        statement: "Confusing a commitment id, evidence id, capability id, proposal id, actor, resource, amount, time or version with one another",
        support: Support::Compiler { how: "distinct newtypes; doctest `compile_fail`" },
    },
    Claim {
        id: id("c3"),
        statement: "An invalid name literal",
        support: Support::Compiler { how: "`Name::lit` is evaluated at compile time; doctest `compile_fail`" },
    },
    Claim {
        id: id("c4"),
        statement: "Committing another actor",
        support: Support::Compiler { how: "`Offer` has no debtor field; the debtor is the kernel-stamped actor; doctest `compile_fail`" },
    },
    Claim {
        id: id("c5"),
        statement: "A unit claiming to be someone else",
        support: Support::Compiler { how: "events carry no actor field; the kernel assigns it from the emitter's position" },
    },
    Claim {
        id: id("c6"),
        statement: "Evidence that bears on both a proposition and another piece of evidence, or on neither",
        support: Support::Compiler { how: "`About` is an enum with exactly three cases" },
    },
    Claim {
        id: id("c7"),
        statement: "A template that builds a proposition or a party by string substitution",
        support: Support::Compiler { how: "`PartyExpr`, `ConsExpr` are enums; propositions are structured (`Prop::Remediated(CommitmentId)`)" },
    },
    Claim {
        id: id("c8"),
        statement: "An amendment to a path that does not exist",
        support: Support::Compiler { how: "`Amendment` and `RulePath` are closed enums" },
    },
    Claim {
        id: id("c9"),
        statement: "A refusal without an enumerated reason",
        support: Support::Compiler { how: "`DenyReason`, `NormError`, `EvidenceError`, `GovError`, `RulesError`, `Failure`" },
    },
    Claim {
        id: id("c10"),
        statement: "A unit reading or writing another unit's state",
        support: Support::Compiler { how: "each unit owns its state; no shared references; `unsafe` is forbidden" },
    },
    Claim {
        id: id("c11"),
        statement: "Library code that can panic by `unwrap`, `expect`, `panic!`, indexing, or lose data by an `as` cast",
        support: Support::Compiler { how: "denied lints (`Cargo.toml`); `cargo clippy --all-targets` is clean" },
    },
    Claim {
        id: id("c12"),
        statement: "Non-deterministic iteration order inside a unit",
        support: Support::Compiler { how: "only ordered maps and sets are used (no `HashMap`); checked by inspection and by replay tests, not by a lint" },
    },
];

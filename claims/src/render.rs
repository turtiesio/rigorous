//! The status document, generated from the records. Nothing in it is written by hand except
//! the explanations between the tables, which are kept here beside the code that places them.

use crate::book;
use crate::claim::{Axiom, Claim, Id, Support};
use crate::property::Facet;
use std::fmt::Write as _;

fn shown(id: &Id) -> String {
    id.0.as_str().to_uppercase()
}

fn table(out: &mut String, title: &str, intro: &str, header: &[&str], claims: &[Claim], row: impl Fn(&Claim) -> Vec<String>) {
    let _ = writeln!(out, "## {title}\n");
    if !intro.is_empty() {
        let _ = writeln!(out, "{intro}\n");
    }
    let _ = writeln!(out, "| {} |", header.join(" | "));
    let _ = writeln!(out, "|{}", "---|".repeat(header.len()));
    for c in claims {
        let _ = writeln!(out, "| {} | {} |", shown(&c.id), row(c).join(" | "));
    }
    out.push('\n');
}

fn names(xs: &[&str]) -> String {
    xs.iter().map(|x| format!("`{x}`")).collect::<Vec<_>>().join(", ")
}

/// The whole status document.
#[must_use]
pub fn ledger() -> String {
    let mut out = String::from(INTRO);
    let s = |c: &Claim| c.statement.to_owned();
    table(&mut out, "1. Proved (about the model)", "", &["Id", "Statement", "Lean name", "Axioms used"], book::proved::CLAIMS, |c| match c.support {
        Support::Proved { theorems, axioms } => {
            let ax: Vec<&str> = axioms
                .iter()
                .map(|a| match a {
                    Axiom::Propext => "propext",
                    Axiom::QuotSound => "Quot.sound",
                })
                .collect();
            vec![s(c), names(theorems), if ax.is_empty() { "none".to_owned() } else { ax.join(", ") }]
        }
        _ => vec![s(c)],
    });
    out.push_str(GAP);
    table(&mut out, "1a. Compiler-enforced", COMPILER_INTRO, &["Id", "What cannot be expressed", "How"], book::compiler::CLAIMS, |c| match c.support {
        Support::Compiler { how } => vec![s(c), how.to_owned()],
        _ => vec![s(c)],
    });
    out.push_str(STRINGS);
    table(&mut out, "2. Model-checked (finite instances)", "", &["Id", "Statement", "Instance", "Test"], book::model_checked::CLAIMS, |c| match c.support {
        Support::ModelChecked { instance, tests } => vec![s(c), instance.to_owned(), names(tests)],
        _ => vec![s(c)],
    });
    table(&mut out, "3. Tested", "", &["Id", "Statement", "Extent", "Test"], book::tested::CLAIMS, |c| match c.support {
        Support::Tested { extent, shown_by, .. } => vec![s(c), extent.to_owned(), shown_by.to_owned()],
        _ => vec![s(c)],
    });
    table(
        &mut out,
        "3a. Measured on external data (reproducible with network access; not part of `cargo test`)",
        "",
        &["Id", "Statement", "Extent", "How"],
        book::measured::CLAIMS,
        |c| match c.support {
            Support::Measured { extent, how } => vec![s(c), extent.to_owned(), how.to_owned()],
            _ => vec![s(c)],
        },
    );
    out.push_str(PYTHON);
    table(
        &mut out,
        "4. Assumed (the trusted-assumption inventory)",
        "",
        &["Id", "Assumption", "What breaks if it is false"],
        book::assumed::CLAIMS,
        |c| match c.support {
            Support::Assumed { breaks } => vec![s(c), breaks.to_owned()],
            _ => vec![s(c)],
        },
    );
    table(&mut out, "5. Unresolved", "", &["Id", "Open item"], book::unresolved::CLAIMS, |c| vec![s(c)]);
    table(&mut out, "6. Negative results and counterexamples (executable)", "", &["Id", "Result", "Exact assumptions", "Test"], book::negative::CLAIMS, |c| {
        match c.support {
            Support::Counterexample { assuming, shown_by, .. } => vec![s(c), assuming.to_owned(), shown_by.to_owned()],
            _ => vec![s(c)],
        }
    });
    out.push_str(LIMITS);
    out.push_str("## 7. Classification of properties\n\n");
    out.push_str("| Property | Preserved by composition | Statically checked | Enforced at run time | Observable only | Rests on external trust | Not established |\n|---|---|---|---|---|---|---|\n");
    for p in book::properties::PROPERTIES {
        let cells: Vec<&str> = p.facets().iter().map(|f| f.map_or("", |Facet { note, .. }| note)).collect();
        let _ = writeln!(out, "| {} | {} |", p.name, cells.join(" | "));
    }
    out
}

const INTRO: &str = "# Status ledger

GENERATED from the records in `claims/src/book/` by `cargo run --bin claims -- --write`. Do not edit:
a test fails if this file differs from what the records say, or if a record cites a test or a theorem
that does not exist.

Every claim this repository makes, with how it is supported. The kinds of support are kept strictly apart:

- **Proved** — machine-checked in Lean 4 (`proofs/Rigor.lean`). These are theorems about the *mathematical
  model* in `02-semantics.md`. They say nothing directly about the Rust code.
- **Compiler-enforced** — the Rust type checker or a denied lint rejects the program. Holds for every
  execution, with no test needed.
- **Model-checked** — exhaustive exploration of a *finite instance* to a fixed point.
- **Tested** — seeded randomised or scenario tests. Evidence, not proof.
- **Measured** — run on external data; reproducible, but not part of the test suite.
- **Assumed** — taken on trust from outside the system.
- **Unresolved** — not established here.

Section 7 places each property in the six classes: preserved by composition, statically checked, enforced
at run time, observable only, resting on external trust, not established.

Run `cargo test`, `cargo clippy --all-targets` and `lean proofs/Rigor.lean` — or `cargo run --bin steward`,
which runs all of them and records the evidence.

";

const GAP: &str = "**Gap between model and code.** The Lean model has binary `par` over a single event type and a FIFO
scheduler with feedback (T8, T9). It still omits `settle`, the ordering of simultaneous outputs, `Entity`,
kinds, and all the institutional units. The Rust code
is connected to these theorems only by the tests below. That correspondence is **unresolved** (U1).

";

const COMPILER_INTRO: &str = "Each line is something that cannot be written, not something that is checked when it happens.";

const STRINGS: &str = "Not compiler-enforced, and checked instead when rules are proposed: a responsibility rule that is ill-sorted
(arguments of the wrong kind), unsafe, or unstratifiable — because rules are run-time data
(`rules_without_a_unique_meaning_are_rejected`).

Where strings remain, and why they are not \"stringly\": the free text of `Content::Note` and the program name
in `Content::Command` (documentation that nothing interprets); the diagnostic text of a JSON encoding or
parsing failure; the statements and notes of the claim records (prose for people; what a record cites is
checked to exist); and the JSON serialisation boundary itself, where an unknown variant or invalid name is
rejected on the way in.

";

const PYTHON: &str = "**No longer reproducible.** An earlier Python prototype of the same semantics was kept alongside the Rust
code until the two were shown to agree on five scenario variants (commitments, statuses, assessments,
verdicts, rule version, event count) and on every shared measurement count. It has since been removed, so
that agreement cannot be re-run from this repository; it is recorded here as history, not as a current claim.

";

const LIMITS: &str = "Known theorems that bound what is achievable, cited and **not** re-proved here: Rice's theorem (no non-trivial
behavioural property of arbitrary unit code is decidable — hence rules are restricted to stratified Datalog);
the CAP theorem (under partition one cannot have both availability and the single order A-ORDER requires);
and Gödel/Löb-style limits (the system offers no argument for the soundness of its own trusted base, A-TCB).

";

//! `cargo run --bin steward [-- --force] [--skip-proofs] [accept <requirement>]`
//!
//! The project checking, recording and holding itself to account. Exits non-zero when a
//! self-claim is not currently evidenced.

use rigor::kernel::Name;
use rigor::model::{Condition, Prop, Requirement, Status, Tag};
use rigor::steward::{today, Check, Outcome, Steward, StewardError};
use std::collections::BTreeSet;
use std::path::Path;
use std::process::ExitCode;

fn requirement_of(c: &rigor::model::Commitment) -> String {
    match &c.terms.cons {
        Condition::Holds(Prop::Conforms(r)) | Condition::Unrefuted(Prop::Conforms(r)) => r.to_string(),
        other => format!("{other:?}"),
    }
}

fn main() -> Result<ExitCode, StewardError> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let has = |flag: &str| args.iter().any(|a| a == flag);
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let log = root.join(".rigor/steward.jsonl");
    let day = today()?;
    let mut steward = Steward::open(&root, &log, day)?;
    if let Some(aside) = &steward.previous_log_rejected {
        println!("NOTE: the stored log could not be replayed by this version and was set aside as {}", aside.display());
    }
    if let Some(schema) = steward.migrated_from {
        println!("NOTE: the stored log was in schema {schema}; its chain was verified and its history migrated (old file kept)");
    }
    if let Some(i) = args.iter().position(|a| a == "accept") {
        let Some(name) = args.get(i + 1).and_then(|s| Name::new(s).ok()) else {
            println!("usage: steward accept <requirement>");
            return Ok(ExitCode::from(2));
        };
        steward.accept(Requirement(name), day)?;
    }
    let skip: BTreeSet<Check> = if has("--skip-proofs") { std::iter::once(Check::Proofs).collect() } else { BTreeSet::new() };
    let outcomes = steward.run(day, has("--force"), &skip)?;

    println!("SELF-CLAIMS (day {day})");
    for (check, outcome) in &outcomes {
        let what = match outcome {
            Outcome::StillEvidenced => "evidenced by an earlier run; files unchanged, nothing re-run",
            Outcome::RanAndPassed => "check run: passed",
            Outcome::RanAndFailed => "check run: FAILED",
            Outcome::CouldNotObserve => "could NOT be observed (tool missing?): nothing established",
            Outcome::Skipped => "skipped by request: nothing established",
        };
        let standing = steward.world.epi()?.assessment(&check.prop());
        println!("  {:<18} {:<12} {what}", check.requirement().to_string(), format!("{standing:?}"));
    }

    let upkeep = steward.commitments(Tag::Upkeep)?;
    let count = |s: Status| upkeep.iter().filter(|(_, st)| *st == s).count();
    println!(
        "\nRECORD OF UPKEEP  ({} undertakings: {} satisfied, {} violated, {} open today)",
        upkeep.len(),
        count(Status::Satisfied),
        count(Status::Violated),
        count(Status::Detached)
    );
    for (c, _) in upkeep.iter().filter(|(_, s)| *s == Status::Violated) {
        println!("  day {:<6} {:<18} did not hold; findings: {:?}", c.terms.deadline.0, requirement_of(c), steward.world.adj()?.findings(&c.id));
    }

    let remediation = steward.commitments(Tag::Remediation)?;
    if !remediation.is_empty() {
        println!("\nDUTIES TO PUT A LAPSE RIGHT");
        for (c, status) in &remediation {
            println!("  {}  due day {:<6} {status:?}", c.id, c.terms.deadline.0);
        }
    }
    println!("\nROADMAP OBLIGATIONS");
    let mut backlog = steward.commitments(Tag::Task)?;
    backlog.sort_by_key(|(c, _)| c.terms.deadline);
    for (c, status) in &backlog {
        let due = c.terms.deadline.0;
        let when = if due >= day { format!("review in {} days", due - day) } else { format!("OVERDUE by {} days", day - due) };
        println!("  {:<38} {:<10} {when}", requirement_of(c), format!("{status:?}"));
    }
    let log_len = steward.world.rt.log().len();
    println!("\nLOG  {log_len} events at {}, chain verified: {}", log.display(), steward.world.rt.log().verify().is_ok());
    Ok(if steward.conformant(&skip)? { ExitCode::SUCCESS } else { ExitCode::FAILURE })
}

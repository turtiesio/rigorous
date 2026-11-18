//! The project looking after itself.
//!
//! ```text
//! steward                     check itself, examine itself, write the dashboard
//! steward cycle [N]           the same, then research up to N open questions (default 1) and
//!                             turn accepted answers into a change request, verified and reviewed
//! steward can                 what it can do, what it cannot, and what is lacking
//! steward ui                  only rewrite the dashboard
//!
//! Decisions that are the owner's (run these only as, or for, the owner):
//! steward grant-budget <millionths of a dollar>
//! steward decide <requirement> yes|no
//! steward approve|reject <change id>
//! steward apply <change id>
//! steward accept <roadmap item>
//! ```
//! Exits non-zero when a claim the project makes about itself is not currently evidenced.

use rigor::kernel::Name;
use rigor::mind::Gateway;
use rigor::model::{ChangeId, Requirement, Status, Tag};
use rigor::steward::change::{ChangeRequest, NewFile};
use rigor::steward::research::{agenda, Outcome as Researched};
use rigor::steward::selfmodel::{describe_need, SelfReport, Standing};
use rigor::steward::{today, Check, Outcome, Steward, StewardError, RESEARCHER};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

fn gateway() -> Gateway {
    let mut files = Vec::new();
    if let Ok(explicit) = std::env::var("RIGOR_KEY_FILE") {
        files.push(PathBuf::from(explicit));
    }
    if let Ok(home) = std::env::var("HOME") {
        files.push(Path::new(&home).join(".pi/agent/auth.json"));
    }
    Gateway::vercel(files)
}

fn print_abilities(report: &SelfReport) {
    println!("\nWHAT IT CAN AND CANNOT DO");
    for (a, s) in &report.abilities {
        match s {
            Standing::Can => println!("  can      {}", a.name()),
            Standing::CanWithApproval => println!("  can*     {}   (* with the owner's approval each time)", a.name()),
            Standing::Cannot(lacking) => {
                println!("  CANNOT   {:<28} lacking: {}", a.name().to_string(), lacking.iter().map(describe_need).collect::<Vec<_>>().join("; "))
            }
        }
    }
}

fn checks(steward: &mut Steward, day: u64, force: bool, skip: &BTreeSet<Check>) -> Result<(), StewardError> {
    let outcomes = steward.run(day, force, skip)?;
    println!("SELF-CLAIMS (day {day})");
    for (check, outcome) in &outcomes {
        let what = match outcome {
            Outcome::StillEvidenced => "evidenced by an earlier run; files unchanged, nothing re-run",
            Outcome::RanAndPassed => "check run: passed",
            Outcome::RanAndFailed => "check run: FAILED",
            Outcome::CouldNotObserve => "could NOT be observed (tool missing?): nothing established",
            Outcome::Skipped => "skipped by request: nothing established",
        };
        println!("  {:<18} {:<12} {what}", check.requirement().to_string(), format!("{:?}", steward.world.epi()?.assessment(&check.prop())));
    }
    let upkeep = steward.commitments(Tag::Upkeep)?;
    let count = |s: Status| upkeep.iter().filter(|(_, st)| *st == s).count();
    println!("  upkeep on record: {} undertakings, {} held, {} lapsed", upkeep.len(), count(Status::Satisfied), count(Status::Violated));
    Ok(())
}

fn write_dashboard(steward: &Steward, day: u64, report: &SelfReport) -> Result<PathBuf, StewardError> {
    let path = steward.home().join("dashboard.html");
    std::fs::write(&path, steward.dashboard(day, report)?).map_err(|e| StewardError::Adapter(rigor::adapters::AdapterError::Io(e)))?;
    Ok(path)
}

fn change_id(text: Option<&String>) -> Option<ChangeId> {
    text.and_then(|s| Name::new(s).ok()).map(ChangeId)
}

fn main() -> Result<ExitCode, StewardError> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let has = |flag: &str| args.iter().any(|a| a == flag);
    let command = args.first().map_or("", String::as_str);
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let log = root.join(".rigor/steward.jsonl");
    let day = today()?;
    let gateway = gateway();
    let mut steward = Steward::open(&root, &log, day)?;
    if let Some(aside) = &steward.previous_log_rejected {
        println!("NOTE: the stored log could not be replayed by this version and was set aside as {}", aside.display());
    }
    if let Some(schema) = steward.migrated_from {
        println!("NOTE: the stored log was in schema {schema}; its chain was verified and its history migrated (old file kept)");
    }
    if steward.refounded {
        println!("NOTE: the project was re-founded under its current constitution; its history was carried over (old file kept)");
        steward.persist()?;
    }
    let skip: BTreeSet<Check> = if has("--skip-proofs") { std::iter::once(Check::Proofs).collect() } else { BTreeSet::new() };

    // The owner's decisions.
    match command {
        "grant-budget" => {
            let Some(amount) = args.get(1).and_then(|s| s.parse::<u64>().ok()) else {
                println!("usage: steward grant-budget <millionths of a dollar>");
                return Ok(ExitCode::from(2));
            };
            println!("budget granted: {}", steward.owner_grants_budget(amount)?);
        }
        "decide" => {
            let (Some(name), Some(answer)) = (args.get(1).and_then(|s| Name::new(s).ok()), args.get(2)) else {
                println!("usage: steward decide <requirement> yes|no");
                return Ok(ExitCode::from(2));
            };
            steward.owner_decides(Requirement(name), answer == "yes", day)?;
        }
        "approve" | "reject" => {
            let Some(id) = change_id(args.get(1)) else {
                println!("usage: steward approve|reject <change id>");
                return Ok(ExitCode::from(2));
            };
            steward.owner_rules_on(&id, command == "approve")?;
        }
        "apply" => {
            let Some(id) = change_id(args.get(1)) else {
                println!("usage: steward apply <change id>");
                return Ok(ExitCode::from(2));
            };
            match steward.apply(&id)? {
                Ok(()) => println!("applied {id}"),
                Err(missing) => println!("NOT applied {id}: still missing {missing:?}"),
            }
        }
        "accept" => {
            let Some(name) = args.get(1).and_then(|s| Name::new(s).ok()) else {
                println!("usage: steward accept <requirement>");
                return Ok(ExitCode::from(2));
            };
            steward.accept(Requirement(name), day)?;
        }
        _ => {}
    }

    if !matches!(command, "ui" | "can") {
        checks(&mut steward, day, has("--force"), &skip)?;
    }
    let mut report = steward.examine_itself(&gateway)?;
    if command == "can" {
        print_abilities(&report);
    }

    if command == "cycle" {
        let rounds = args.get(1).and_then(|s| s.parse::<usize>().ok()).unwrap_or(1);
        println!("\nRESEARCH (budget left: {} millionths of a dollar)", steward.budget_left()?);
        let mut fresh = Vec::new();
        let mut asked = 0;
        for q in agenda() {
            if asked >= rounds {
                break;
            }
            if !report.can(rigor::steward::selfmodel::Ability::Research) {
                println!("  cannot research now: see what is lacking below");
                break;
            }
            match steward.research(&gateway, &q, &report)? {
                Researched::AlreadyAnswered => {}
                Researched::Accepted { rounds, .. } => {
                    asked += 1;
                    println!("  {}: answered, reviewed and accepted (round {rounds})", q.id);
                    fresh.push(q.id.clone());
                }
                Researched::NotAccepted { rounds } => {
                    asked += 1;
                    println!("  {}: the reviewer still objects after {rounds} rounds; left open", q.id);
                }
                Researched::OutOfBudget => {
                    println!("  {}: out of budget; stopped", q.id);
                    break;
                }
            }
        }
        // Accepted answers enter the repository only as a change request, like any other change.
        if !fresh.is_empty() {
            let mut files = Vec::new();
            for q in &fresh {
                let answer = std::fs::read_to_string(steward.home().join("research").join(format!("{q}.md")))
                    .map_err(|e| StewardError::Adapter(rigor::adapters::AdapterError::Io(e)))?;
                let text = agenda().into_iter().find(|x| x.id == *q).map_or("", |x| x.text);
                files.push(NewFile { path: format!("docs/research/{q}.md"), content: format!("# {q}\n\n**Question.** {text}\n\n*Drafted by a language model and accepted by a second, different model acting as reviewer. It is an answer the project has adopted as a working position, not established fact.*\n\n{answer}\n") });
            }
            let request = ChangeRequest {
                title: format!("Record {} research answer(s)", fresh.len()),
                reason: "Accepted research answers are kept with the project so that later decisions can cite them.".to_owned(),
                proposed_by: RESEARCHER,
                files,
            };
            match steward.propose(&request)? {
                Ok(id) => {
                    println!("\nCHANGE {id} proposed: {}", request.title);
                    println!("  sandbox verification: {}", if steward.verify(&id)? { "passed" } else { "FAILED" });
                    match steward.review_change(&gateway, &id)? {
                        Some(true) => println!("  reviewer: acknowledged"),
                        Some(false) => println!("  reviewer: OBJECTS (see .rigor/changes/{id}.review.md)"),
                        None => println!("  reviewer: not consulted (out of budget)"),
                    }
                    println!("  still missing before it can be applied: {:?}", steward.progress(&id)?.missing);
                }
                Err(refusal) => println!("\nchange request refused: {refusal:?}"),
            }
        }
        report = steward.examine_itself(&gateway)?;
    }

    if matches!(command, "cycle" | "") {
        print_abilities(&report);
    }
    let pending: Vec<_> = steward.change_ids()?.iter().filter_map(|id| steward.progress(id).ok()).filter(|p| !p.applied).collect();
    if !pending.is_empty() {
        println!("\nCHANGES NOT YET APPLIED");
        for p in &pending {
            println!("  {}  {:<40} assessed {:?}; missing {:?}", p.id, p.title, p.assessment, p.missing);
        }
    }
    let page = write_dashboard(&steward, day, &report)?;
    steward.persist()?;
    println!("\nDASHBOARD  {}", page.display());
    println!("LOG        {} events, chain verified: {}", steward.world.rt.log().next_seq(), steward.world.rt.log().verify().is_ok());
    Ok(if steward.conformant(&skip)? { ExitCode::SUCCESS } else { ExitCode::FAILURE })
}

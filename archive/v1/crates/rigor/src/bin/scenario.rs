//! `cargo run --bin scenario`: the narrated end-to-end demonstrator.

use rigor::kernel::Entry;
use rigor::model::{Assumption, Condition, Const, Event, Finding, RFact, RProof, Source};
use rigor::scenario::{run, Cast, Outcome, AGENT};
use rigor::world::{Error, World};

fn show(f: &RFact) -> String {
    let args: Vec<String> = f
        .args
        .iter()
        .map(|c| match c {
            Const::Actor(a) => a.to_string(),
            Const::Commitment(c) => c.to_string(),
            Const::Prop(p) => p.to_string(),
            Const::Status(s) => format!("{s:?}"),
            Const::Tag(t) => format!("{t:?}"),
            Const::Kind(k) => format!("{k:?}"),
            Const::Role(r) => format!("{r:?}"),
            Const::Bool(b) => b.to_string(),
            Const::Assessment(a) => format!("{a:?}"),
        })
        .collect();
    format!("{:?}({})", f.pred, args.join(", "))
}

fn proof(p: &RProof, depth: usize) {
    let pad = "  ".repeat(depth + 2);
    match p {
        RProof::Given(f) => println!("{pad}{}", show(f)),
        RProof::Derived { fact, rule, from, absent } => {
            println!("{pad}{}   [rule {rule}]", show(fact));
            for sub in from {
                proof(sub, depth + 1);
            }
            for a in absent {
                println!("{pad}  not {}   [no record: closed-world assumption]", show(a));
            }
        }
    }
}

fn changes(entries: &[Entry<Event>]) {
    for e in entries {
        match &e.event {
            Event::Assessed { prop, was, assessment, .. } => println!("  claim       {prop}: {was:?} -> {assessment:?}"),
            Event::NormStatus { id, was: Some(was), status, debtor, .. } => println!("  commitment  {id} ({debtor}): {was:?} -> {status:?}"),
            Event::NormCreated(c) => println!("  new duty    {}: {} owes {} {:?}", c.id, c.terms.debtor, c.terms.creditor, c.terms.tags),
            Event::Transferred { transfer: t, .. } => println!("  transfer    {} -> {}: {} ({:?})", t.from, t.to, t.amount.0, t.memo),
            _ => {}
        }
    }
}

fn report(w: &World) -> Result<(), Error> {
    println!("\nCOMMITMENTS");
    let mut records: Vec<_> = w.norms()?.records().collect();
    records.sort_by_key(|r| (r.created_at, r.commitment.id));
    for r in records {
        let t = r.terms();
        let cond = match &t.cons {
            Condition::Holds(p) => p.to_string(),
            Condition::Unrefuted(p) => format!("unrefuted {p}"),
            Condition::Paid { at_least, .. } => format!("paid >= {}", at_least.0),
        };
        let src = match (&r.commitment.source, &t.basis) {
            (Source::Rule, Some(b)) => format!("rule v{} <- {}", r.commitment.rules.0, b.of),
            _ => "act".to_owned(),
        };
        let status = r.status.map_or_else(|| "-".to_owned(), |s| format!("{s:?}"));
        println!(
            "  {}  {:>8} -> {:<6} {:<28} due {:<3} {:<10} {:?} ({src})",
            r.commitment.id,
            t.debtor.to_string(),
            t.creditor.to_string(),
            cond,
            t.deadline.0,
            status,
            t.tags
        );
    }
    println!("\nPROPOSITIONS (assessment, not truth)");
    for (p, s) in w.epi()?.standings() {
        println!("  {:<28} {:<12} basis={:?}", p.to_string(), format!("{:?}", s.assessment), s.basis);
    }
    println!("\nVERDICTS");
    for (c, findings) in w.adj()?.verdicts() {
        let fs: Vec<String> = findings.iter().map(|(f, a)| format!("{f:?}({a})")).collect();
        println!("  {c}: {}", fs.join(", "));
    }
    let log = w.rt.log();
    println!("\nRULES     version {}   LOG {} events, chain verified: {}", w.gov()?.version().0, log.len(), log.verify().is_ok());
    Ok(())
}

fn culpability(o: &Outcome) {
    let verdict = o.refutation.iter().rev().find_map(|e| match &e.event {
        Event::Verdict(v) if v.commitment == o.c1 => Some(v),
        _ => None,
    });
    let Some(v) = verdict else { return };
    println!("\nWHY THE AGENT IS CULPABLE (rules v{})", v.rules.0);
    for p in &v.proofs {
        if let RProof::Derived { fact, .. } = p {
            if fact.pred == Finding::Culpable.pred() && fact.args.first() == Some(&Const::Actor(AGENT)) {
                proof(p, 0);
            }
        }
    }
    println!("   assumptions:");
    for a in &v.assumptions {
        match a {
            Assumption::Authentication => println!("     - actors are who the log says they are (authentication is external)"),
            Assumption::RecordsNotKnowledge => println!("     - 'notified' and 'claimed' are records of messages, not of what anyone knew"),
            Assumption::AssessmentIsRevisable => println!("     - 'violated' rests on the current assessment of evidence, which is revisable"),
            Assumption::NoRecordOf(f) => println!("     - no record that {}", show(f)),
        }
    }
}

fn main() -> Result<(), Error> {
    let (w, o) = run(Cast::default(), true, |line| println!("{line}"))?;
    println!("\nWHAT THE REFUTATION CHANGED (one evidence event at t=6)");
    changes(&o.refutation);
    culpability(&o);
    println!("\nAUTHORITY");
    println!("  over-delegation   denied: {:?}", o.over_delegation);
    println!("  overspend         denied: {:?}", o.overspend);
    println!("  revoked use       denied: {:?}", o.revoked_use);
    println!("  revoked sub-use   denied: {:?}", o.revoked_sub_use);
    println!("  entrenched rule   rejected: {:?}", o.entrenched);
    report(&w)
}

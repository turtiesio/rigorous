//! The claims checked against the repository: prints what is wrong, exits non-zero if anything is.
//! With `--write`, regenerates the status document from the records.

use rigor::claims::{render, shape, witness};
use std::path::PathBuf;
use std::process::ExitCode;

fn main() -> ExitCode {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().map(PathBuf::from).unwrap_or_default();
    let document = root.join(rigor::claims::LEDGER_DOCUMENT);
    if std::env::args().any(|a| a == "--write") {
        if let Err(e) = std::fs::write(&document, render::ledger()) {
            eprintln!("could not write {}: {e}", document.display());
            return ExitCode::FAILURE;
        }
        println!("wrote {}", document.display());
    }
    let rust = shape::Layer::ALL.iter().map(|l| root.join(l.dir())).collect();
    let missing = witness::missing(&witness::Sources { rust, proofs: root.join("proofs") });
    let shape = shape::problems(&root);
    let stale = std::fs::read_to_string(&document).map(|on_disk| on_disk != render::ledger());
    match (missing, shape, stale) {
        (Ok(missing), Ok(shape), Ok(stale)) => {
            for m in &missing {
                println!("cited but not there: {m:?}");
            }
            for s in &shape {
                println!("out of shape: {s:?}");
            }
            if stale {
                println!("the status document differs from the records: run with --write");
            }
            let claims = rigor::claims::book::all().count();
            println!(
                "{claims} claims, {} properties: {} missing witnesses, {} shape problems",
                rigor::claims::book::properties::PROPERTIES.len(),
                missing.len(),
                shape.len()
            );
            if missing.is_empty() && shape.is_empty() && !stale {
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            }
        }
        (m, s, d) => {
            eprintln!("could not read the repository: {:?} {:?} {:?}", m.err(), s.err(), d.err());
            ExitCode::FAILURE
        }
    }
}

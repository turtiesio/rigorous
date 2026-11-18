#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::indexing_slicing, missing_docs)]
//! The book checked against the repository it describes.

use rigor_claims::witness::{missing, Sources};
use std::path::PathBuf;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf()
}

#[test]
fn everything_a_claim_cites_exists() {
    let r = root();
    let rust = ["waist", "explore", "logic", "concepts", "assembly", "claims", "edges", "operate", "app"].iter().map(|d| r.join(d)).collect();
    let gone = missing(&Sources { rust, proofs: r.join("proofs") }).unwrap();
    assert!(gone.is_empty(), "cited but not there:\n{gone:#?}");
}

#[test]
fn the_code_has_the_shape_the_design_says() {
    let wrong = rigor_claims::shape::problems(&root()).unwrap();
    assert!(wrong.is_empty(), "out of shape:\n{wrong:#?}");
}

#[test]
fn a_concept_may_not_use_another_concept_and_the_waist_uses_nothing() {
    use rigor_claims::shape::Layer;
    assert!(Layer::Waist.may_use().is_empty());
    assert!(!Layer::Concept.may_use().contains(&Layer::Concept));
    assert!(!Layer::Concept.may_use().contains(&Layer::Assembly));
    assert!(!Layer::Assembly.may_use().contains(&Layer::Edge));
}

#[test]
fn the_status_document_says_exactly_what_the_records_say() {
    let on_disk = std::fs::read_to_string(root().join(rigor_claims::LEDGER_DOCUMENT)).unwrap();
    assert!(on_disk == rigor_claims::render::ledger(), "docs/03-status-ledger.md is out of date: cargo run --bin claims -- --write");
}

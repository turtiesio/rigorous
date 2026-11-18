//! Does everything a claim cites exist? A test by that name, a theorem by that name, a claim by
//! that identifier. A claim whose witness is gone is reported, not silently kept.

use crate::book;
use crate::claim::Id;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

/// Something a record cites that is not there.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Missing {
    Test {
        claim: Id,
        name: &'static str,
    },
    Theorem {
        claim: Id,
        name: &'static str,
    },
    /// A property rests on a claim that is not in the book.
    Claim {
        property: &'static str,
        id: Id,
    },
    /// Two claims share an identifier.
    DuplicateId(Id),
    /// A claim supported by tests or by proof cites none.
    NoWitness(Id),
    /// A property placed in none of the six classes.
    Unplaced(&'static str),
}

/// Where to look for what is cited.
#[derive(Clone, Debug)]
pub struct Sources {
    /// Rust source directories (tests and library code alike).
    pub rust: Vec<PathBuf>,
    /// Directory of Lean proofs.
    pub proofs: PathBuf,
}

fn files(dir: &Path, extension: &str, out: &mut Vec<PathBuf>) -> std::io::Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            if path.file_name().is_some_and(|n| n != "target") {
                files(&path, extension, out)?;
            }
        } else if path.extension().is_some_and(|e| e == extension) {
            out.push(path);
        }
    }
    Ok(())
}

/// The names declared after `keyword ` anywhere in the files with this extension.
fn declared(dirs: &[PathBuf], extension: &str, keywords: &[&str]) -> std::io::Result<BTreeSet<String>> {
    let mut paths = Vec::new();
    for d in dirs {
        files(d, extension, &mut paths)?;
    }
    let mut names = BTreeSet::new();
    for p in paths {
        let text = std::fs::read_to_string(&p)?;
        for line in text.lines() {
            let line = line.trim_start();
            for k in keywords {
                if let Some(rest) = line.strip_prefix(k) {
                    let name: String = rest.chars().take_while(|c| c.is_alphanumeric() || *c == '_' || *c == '.').collect();
                    if !name.is_empty() {
                        let _ = names.insert(name);
                    }
                }
            }
        }
    }
    Ok(names)
}

/// Everything the book cites that does not exist.
///
/// # Errors
/// If a source file cannot be read.
pub fn missing(sources: &Sources) -> std::io::Result<Vec<Missing>> {
    let tests = declared(&sources.rust, "rs", &["fn "])?;
    let theorems = declared(std::slice::from_ref(&sources.proofs), "lean", &["theorem ", "lemma "])?;
    let mut out = Vec::new();
    let mut ids: BTreeSet<&Id> = BTreeSet::new();
    for c in book::all() {
        if !ids.insert(&c.id) {
            out.push(Missing::DuplicateId(c.id.clone()));
        }
        out.extend(c.support.tests().iter().filter(|t| !tests.contains(**t)).map(|name| Missing::Test { claim: c.id.clone(), name }));
        out.extend(c.support.theorems().iter().filter(|t| !theorems.contains(**t)).map(|name| Missing::Theorem { claim: c.id.clone(), name }));
        let cites_nothing = match c.support {
            crate::claim::Support::Proved { theorems, .. } => theorems.is_empty(),
            crate::claim::Support::ModelChecked { tests, .. } | crate::claim::Support::Tested { tests, .. } => tests.is_empty(),
            _ => false,
        };
        if cites_nothing {
            out.push(Missing::NoWitness(c.id.clone()));
        }
    }
    for p in book::properties::PROPERTIES {
        if p.facets().iter().all(Option::is_none) {
            out.push(Missing::Unplaced(p.name));
        }
        for f in p.facets().into_iter().flatten() {
            out.extend(f.rests_on.iter().filter(|i| !ids.contains(i)).map(|i| Missing::Claim { property: p.name, id: i.clone() }));
        }
    }
    Ok(out)
}

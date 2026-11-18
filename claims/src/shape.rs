//! Is the code still shaped the way the design says?
//!
//! Three things are checked, none of which a compiler checks: which layer may depend on which;
//! how large the trusted waist is; and how long a file may grow. A file that must stay too
//! long can be listed with a ceiling it may not exceed; the list is empty.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// The layers, innermost first.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Layer {
    /// The trusted substrate.
    Waist,
    /// Reasoners with no notion of institutions.
    Logic,
    /// The explorer: a harness over the waist.
    Explore,
    /// One concept per crate, each in its own words.
    Concept,
    /// Where concepts meet.
    Assembly,
    /// The claims and their checks.
    Claims,
    /// Translations of the outside world.
    Edge,
    /// The system operating itself, and the demonstrations.
    Operate,
    /// Binaries and tests.
    App,
}

impl Layer {
    pub const ALL: [Self; 9] = [Self::Waist, Self::Logic, Self::Explore, Self::Concept, Self::Assembly, Self::Claims, Self::Edge, Self::Operate, Self::App];

    /// The top-level directory holding this layer's crates.
    #[must_use]
    pub const fn dir(self) -> &'static str {
        match self {
            Self::Waist => "waist",
            Self::Logic => "logic",
            Self::Explore => "explore",
            Self::Concept => "concepts",
            Self::Assembly => "assembly",
            Self::Claims => "claims",
            Self::Edge => "edges",
            Self::Operate => "operate",
            Self::App => "app",
        }
    }

    /// The layers a crate in this layer may depend on.
    #[must_use]
    pub const fn may_use(self) -> &'static [Self] {
        match self {
            // The waist and the reasoners stand alone. A concept knows them and NO other concept.
            Self::Waist | Self::Logic => &[],
            Self::Explore | Self::Claims => &[Self::Waist],
            Self::Concept => &[Self::Waist, Self::Logic],
            Self::Assembly => &[Self::Waist, Self::Logic, Self::Concept],
            Self::Edge => &[Self::Waist, Self::Logic, Self::Concept, Self::Assembly, Self::Edge],
            Self::Operate => &[Self::Waist, Self::Logic, Self::Concept, Self::Assembly, Self::Claims, Self::Edge],
            Self::App => &Self::ALL,
        }
    }
}

/// The most lines the trusted waist may hold in all, and in any one file.
pub const WAIST_LINES: usize = 1500;
pub const WAIST_FILE_LINES: usize = 160;
/// The most lines any other source file may hold.
pub const FILE_LINES: usize = 300;

/// Files allowed to be longer than [`FILE_LINES`], each with the ceiling it may not exceed.
/// There are none. An entry that is no longer needed is itself reported.
pub const OVERSIZE: [(&str, usize); 0] = [];

/// A way the code has left its intended shape.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Problem {
    /// A crate depends on a crate in a layer it may not use.
    Depends {
        from: PathBuf,
        layer: Layer,
        on: String,
        on_layer: Layer,
    },
    WaistTooLarge {
        lines: usize,
    },
    FileTooLong {
        file: PathBuf,
        lines: usize,
        limit: usize,
    },
    /// A file is listed as oversize but no longer needs to be: remove it from the list.
    StaleException {
        file: PathBuf,
    },
    /// A workspace dependency whose path is in no known layer.
    OutsideEveryLayer {
        krate: String,
    },
}

fn layer_of(path: &str) -> Option<Layer> {
    let first = path.split('/').next()?;
    Layer::ALL.into_iter().find(|l| l.dir() == first)
}

fn walk(dir: &Path, wanted: &dyn Fn(&Path) -> bool, out: &mut Vec<PathBuf>) -> std::io::Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            if path.file_name().is_some_and(|n| n != "target") {
                walk(&path, wanted, out)?;
            }
        } else if wanted(&path) {
            out.push(path);
        }
    }
    Ok(())
}

/// `name = { path = "…" }` lines of the root manifest: crate name to its layer.
fn workspace_crates(root: &Path, problems: &mut Vec<Problem>) -> std::io::Result<BTreeMap<String, Layer>> {
    let mut map = BTreeMap::new();
    for line in std::fs::read_to_string(root.join("Cargo.toml"))?.lines() {
        let Some((name, rest)) = line.split_once(" = { path = \"") else { continue };
        let Some((path, _)) = rest.split_once('"') else { continue };
        match layer_of(path) {
            Some(layer) => {
                let _ = map.insert(name.to_owned(), layer);
            }
            None => problems.push(Problem::OutsideEveryLayer { krate: name.to_owned() }),
        }
    }
    Ok(map)
}

/// Everything about the repository at `root` that has left the intended shape.
///
/// # Errors
/// If a file cannot be read.
pub fn problems(root: &Path) -> std::io::Result<Vec<Problem>> {
    let mut out = Vec::new();
    let crates = workspace_crates(root, &mut out)?;
    let relative = |p: &Path| p.strip_prefix(root).unwrap_or(p).to_path_buf();

    // Who depends on whom.
    for layer in Layer::ALL {
        let mut manifests = Vec::new();
        walk(&root.join(layer.dir()), &|p| p.file_name().is_some_and(|n| n == "Cargo.toml"), &mut manifests)?;
        for manifest in manifests {
            for line in std::fs::read_to_string(&manifest)?.lines() {
                let Some(name) = line.strip_suffix(".workspace = true") else { continue };
                let Some(on_layer) = crates.get(name).copied() else { continue };
                if !layer.may_use().contains(&on_layer) {
                    out.push(Problem::Depends { from: relative(&manifest), layer, on: name.to_owned(), on_layer });
                }
            }
        }
    }

    // How large things are. Tests and the records of the book are data, not machinery.
    let mut waist = 0usize;
    let mut seen_oversize = Vec::new();
    for layer in Layer::ALL {
        let mut sources = Vec::new();
        walk(&root.join(layer.dir()), &|p| p.extension().is_some_and(|e| e == "rs"), &mut sources)?;
        for file in sources {
            let rel = relative(&file);
            if rel.components().any(|c| c.as_os_str() == "tests" || c.as_os_str() == "book") {
                continue;
            }
            let lines = std::fs::read_to_string(&file)?.lines().count();
            let listed = OVERSIZE.iter().find(|(p, _)| Path::new(p) == rel);
            let limit = match (layer, listed) {
                (Layer::Waist, _) => WAIST_FILE_LINES,
                (_, Some((_, ceiling))) => *ceiling,
                (_, None) => FILE_LINES,
            };
            if layer == Layer::Waist {
                waist += lines;
            }
            if lines > limit {
                out.push(Problem::FileTooLong { file: rel.clone(), lines, limit });
            }
            if listed.is_some() {
                if lines <= FILE_LINES {
                    out.push(Problem::StaleException { file: rel.clone() });
                }
                seen_oversize.push(rel);
            }
        }
    }
    if waist > WAIST_LINES {
        out.push(Problem::WaistTooLarge { lines: waist });
    }
    out.extend(OVERSIZE.iter().map(|(p, _)| PathBuf::from(p)).filter(|p| !seen_oversize.contains(p)).map(|file| Problem::StaleException { file }));
    Ok(out)
}

//! Real files as artifacts: a digest change becomes an event.

use crate::error::AdapterError;
use rigor_institution::model::{ArtifactId, ContentDigest, Event};
use rigor_institution::world::World;
use rigor_waist::digest_bytes;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

pub use rigor_institution::model::FILESYSTEM;

/// Observes real files; reports digest changes as `ArtifactChanged` events.
/// An artifact is a set of files: its digest covers every file's path and content.
#[derive(Debug, Default)]
pub struct ArtifactWatcher {
    paths: BTreeMap<ArtifactId, Vec<PathBuf>>,
    seen: BTreeMap<ArtifactId, ContentDigest>,
}

/// Every file under `dir` (recursively) whose extension is one of `extensions` (every file, if
/// none are given), sorted.
///
/// # Errors
/// If a directory cannot be read.
pub fn files_under(dir: &Path, extensions: &[&str]) -> Result<Vec<PathBuf>, AdapterError> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        for entry in std::fs::read_dir(&d).map_err(AdapterError::Io)? {
            let path = entry.map_err(AdapterError::Io)?.path();
            if path.is_dir() {
                stack.push(path);
            } else if extensions.is_empty() || path.extension().and_then(|e| e.to_str()).is_some_and(|e| extensions.contains(&e)) {
                out.push(path);
            }
        }
    }
    out.sort();
    Ok(out)
}

impl ArtifactWatcher {
    /// One file per artifact.
    #[must_use]
    pub fn new(paths: impl IntoIterator<Item = (ArtifactId, PathBuf)>) -> Self {
        Self::of_sets(paths.into_iter().map(|(a, p)| (a, vec![p])))
    }

    /// A set of files per artifact.
    #[must_use]
    pub fn of_sets(sets: impl IntoIterator<Item = (ArtifactId, Vec<PathBuf>)>) -> Self {
        Self { paths: sets.into_iter().collect(), seen: BTreeMap::new() }
    }

    /// The digest last observed for an artifact.
    #[must_use]
    pub fn seen(&self, a: &ArtifactId) -> Option<ContentDigest> {
        self.seen.get(a).copied()
    }

    /// Digest every watched file; inject an event for each one that changed.
    ///
    /// # Errors
    /// If a file cannot be read, or on a kernel fault.
    pub fn scan(&mut self, w: &mut World) -> Result<Vec<ArtifactId>, AdapterError> {
        let mut changed = Vec::new();
        for (artifact, paths) in &self.paths {
            let mut bytes = Vec::new();
            for path in paths {
                bytes.extend(path.to_string_lossy().as_bytes());
                bytes.push(0);
                bytes.extend(std::fs::read(path).map_err(AdapterError::Io)?);
                bytes.push(0);
            }
            let digest = ContentDigest(digest_bytes(&bytes));
            if self.seen.get(artifact) != Some(&digest) {
                let _ = self.seen.insert(artifact.clone(), digest);
                let _ = w.inject(&FILESYSTEM, Event::ArtifactChanged { artifact: artifact.clone(), digest })?;
                changed.push(artifact.clone());
            }
        }
        Ok(changed)
    }
}

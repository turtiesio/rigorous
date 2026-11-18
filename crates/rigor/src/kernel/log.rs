//! The append-only, hash-chained log.

use super::base::{digest_of, Digest, EncodingError, Path};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::io::{BufRead, Write};

/// One thing that happened.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Entry<E> {
    pub seq: u64,
    pub id: Digest,
    /// Who performed it: stamped by the kernel from the emitter's position, never by the emitter.
    pub actor: Path,
    /// The entry whose processing emitted this one; `None` for an external stimulus.
    pub cause: Option<Digest>,
    /// The emitting leaf (provenance).
    pub src: Path,
    pub event: E,
}

#[derive(Serialize)]
struct Link<'a, E> {
    prev: Digest,
    seq: u64,
    actor: &'a Path,
    cause: Option<Digest>,
    src: &'a Path,
    event: &'a E,
}

/// Why a stored log could not be used.
#[derive(Debug)]
pub enum LogError {
    Io(std::io::Error),
    Parse { line: usize, message: String },
    BrokenChain { seq: u64 },
    Encoding(EncodingError),
}

impl From<EncodingError> for LogError {
    fn from(e: EncodingError) -> Self {
        Self::Encoding(e)
    }
}

#[derive(Clone, Debug)]
pub struct Log<E> {
    entries: Vec<Entry<E>>,
    head: Digest,
}

impl<E> Default for Log<E> {
    fn default() -> Self {
        Self { entries: Vec::new(), head: Digest::default() }
    }
}

impl<E: Serialize> Log<E> {
    fn link(prev: Digest, seq: u64, actor: &Path, cause: Option<Digest>, src: &Path, event: &E) -> Result<Digest, EncodingError> {
        digest_of(&Link { prev, seq, actor, cause, src, event })
    }

    /// # Errors
    /// If the event cannot be encoded.
    pub fn append(&mut self, actor: Path, cause: Option<Digest>, src: Path, event: E) -> Result<&Entry<E>, EncodingError> {
        let seq = u64::try_from(self.entries.len()).map_err(|e| EncodingError(e.to_string()))?;
        let id = Self::link(self.head, seq, &actor, cause, &src, &event)?;
        self.head = id;
        self.entries.push(Entry { seq, id, actor, cause, src, event });
        self.entries.last().ok_or_else(|| EncodingError("log empty after push".to_owned()))
    }

    /// Recompute the chain.
    ///
    /// # Errors
    /// `BrokenChain` at the first entry that was altered, reordered or removed.
    pub fn verify(&self) -> Result<(), LogError> {
        let mut prev = Digest::default();
        for (i, e) in self.entries.iter().enumerate() {
            let seq = u64::try_from(i).map_err(|_| LogError::BrokenChain { seq: e.seq })?;
            if e.seq != seq || e.id != Self::link(prev, seq, &e.actor, e.cause, &e.src, &e.event)? {
                return Err(LogError::BrokenChain { seq });
            }
            prev = e.id;
        }
        if prev == self.head {
            Ok(())
        } else {
            Err(LogError::BrokenChain { seq: u64::MAX })
        }
    }

    /// Write one JSON object per line.
    ///
    /// # Errors
    /// On I/O failure.
    pub fn save<W: Write>(&self, mut w: W) -> Result<(), LogError> {
        for e in &self.entries {
            serde_json::to_writer(&mut w, e).map_err(|x| LogError::Parse { line: 0, message: x.to_string() })?;
            w.write_all(b"\n").map_err(LogError::Io)?;
        }
        Ok(())
    }
}

impl<E: Serialize + DeserializeOwned> Log<E> {
    /// Read a log and verify its chain.
    ///
    /// # Errors
    /// On I/O failure, malformed lines, or a broken chain.
    pub fn load<R: BufRead>(r: R) -> Result<Self, LogError> {
        let mut entries = Vec::new();
        for (i, line) in r.lines().enumerate() {
            let line = line.map_err(LogError::Io)?;
            let e: Entry<E> = serde_json::from_str(&line).map_err(|x| LogError::Parse { line: i + 1, message: x.to_string() })?;
            entries.push(e);
        }
        let head = entries.last().map_or_else(Digest::default, |e| e.id);
        let log = Self { entries, head };
        log.verify()?;
        Ok(log)
    }
}

impl<E> Log<E> {
    #[must_use]
    pub fn entries(&self) -> &[Entry<E>] {
        &self.entries
    }

    #[must_use]
    pub const fn head(&self) -> Digest {
        self.head
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Test hook: overwrite an entry in place (to demonstrate that `verify` notices).
    pub fn tamper(&mut self, index: usize, f: impl FnOnce(&mut Entry<E>)) {
        if let Some(e) = self.entries.get_mut(index) {
            f(e);
        }
    }
}

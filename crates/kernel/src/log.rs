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
    Parse {
        line: usize,
        message: String,
    },
    BrokenChain {
        seq: u64,
    },
    /// The file was written in another version of the format: migrate it, do not parse it.
    Schema {
        found: u32,
    },
    Encoding(EncodingError),
}

impl From<EncodingError> for LogError {
    fn from(e: EncodingError) -> Self {
        Self::Encoding(e)
    }
}

#[derive(Clone, Debug)]
pub struct Log<E> {
    /// Sequence number and chain head at which the retained entries begin (non-zero after a
    /// snapshot: the earlier entries are summarised by the snapshot's state).
    base_seq: u64,
    base_head: Digest,
    entries: Vec<Entry<E>>,
    head: Digest,
}

impl<E> Default for Log<E> {
    fn default() -> Self {
        Self::starting_at(0, Digest::default())
    }
}

/// The version of the on-disk log format written by this build.
pub const SCHEMA: u32 = 3;

#[derive(Serialize, Deserialize)]
struct Header {
    rigor_log_schema: u32,
}

#[derive(Deserialize)]
struct RawLine<'a> {
    seq: u64,
    id: Digest,
    #[serde(borrow)]
    actor: &'a serde_json::value::RawValue,
    cause: Option<Digest>,
    #[serde(borrow)]
    src: &'a serde_json::value::RawValue,
    #[serde(borrow)]
    event: &'a serde_json::value::RawValue,
}

/// One entry of a stored log, checked against the chain but not yet interpreted.
#[derive(Clone, Debug)]
pub struct RawEntry {
    pub actor: Path,
    pub external: bool,
    /// The event exactly as stored, in whatever vocabulary wrote it.
    pub event: serde_json::Value,
}

/// A stored log whose hash chain has been verified WITHOUT interpreting its events.
#[derive(Clone, Debug)]
pub struct RawLog {
    /// 1 for files written before the format carried a header.
    pub schema: u32,
    pub head: Digest,
    pub entries: Vec<RawEntry>,
}

/// Verify a stored log's chain from the bytes on disk alone. This works for a log written by
/// any version of the event vocabulary, which is what makes migration checkable.
///
/// # Errors
/// On I/O failure, a malformed line, or a broken chain.
pub fn read_raw<R: BufRead>(r: R) -> Result<RawLog, LogError> {
    let bad = |line: usize, x: serde_json::Error| LogError::Parse { line, message: x.to_string() };
    let mut schema = 1;
    let mut prev = Digest::default();
    let mut entries = Vec::new();
    let mut expect = 0u64;
    for (i, line) in r.lines().enumerate() {
        let line = line.map_err(LogError::Io)?;
        if i == 0 {
            if let Ok(h) = serde_json::from_str::<Header>(&line) {
                schema = h.rigor_log_schema;
                continue;
            }
        }
        let raw: RawLine<'_> = serde_json::from_str(&line).map_err(|x| bad(i + 1, x))?;
        // The link is re-assembled from the stored bytes of each field, in the order it was hashed.
        let link = format!(
            "{{\"prev\":{},\"seq\":{},\"actor\":{},\"cause\":{},\"src\":{},\"event\":{}}}",
            prev.0,
            raw.seq,
            raw.actor.get(),
            raw.cause.map_or_else(|| "null".to_owned(), |c| c.0.to_string()),
            raw.src.get(),
            raw.event.get()
        );
        if raw.seq != expect || raw.id != super::base::digest_bytes(link.as_bytes()) {
            return Err(LogError::BrokenChain { seq: expect });
        }
        prev = raw.id;
        expect += 1;
        entries.push(RawEntry {
            actor: serde_json::from_str(raw.actor.get()).map_err(|x| bad(i + 1, x))?,
            external: raw.cause.is_none(),
            event: serde_json::from_str(raw.event.get()).map_err(|x| bad(i + 1, x))?,
        });
    }
    Ok(RawLog { schema, head: prev, entries })
}

impl<E: Serialize> Log<E> {
    fn link(prev: Digest, seq: u64, actor: &Path, cause: Option<Digest>, src: &Path, event: &E) -> Result<Digest, EncodingError> {
        digest_of(&Link { prev, seq, actor, cause, src, event })
    }

    /// # Errors
    /// If the event cannot be encoded.
    pub fn append(&mut self, actor: Path, cause: Option<Digest>, src: Path, event: E) -> Result<&Entry<E>, EncodingError> {
        let seq = self.next_seq();
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
        let mut prev = self.base_head;
        for (i, e) in self.entries.iter().enumerate() {
            let seq = self.base_seq.saturating_add(u64::try_from(i).map_err(|_| LogError::BrokenChain { seq: e.seq })?);
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
        serde_json::to_writer(&mut w, &Header { rigor_log_schema: SCHEMA }).map_err(|x| LogError::Parse { line: 0, message: x.to_string() })?;
        w.write_all(b"\n").map_err(LogError::Io)?;
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
            if i == 0 {
                match serde_json::from_str::<Header>(&line) {
                    Ok(h) if h.rigor_log_schema == SCHEMA => continue,
                    Ok(h) => return Err(LogError::Schema { found: h.rigor_log_schema }),
                    Err(_) => return Err(LogError::Schema { found: 1 }),
                }
            }
            let e: Entry<E> = serde_json::from_str(&line).map_err(|x| LogError::Parse { line: i + 1, message: x.to_string() })?;
            entries.push(e);
        }
        let head = entries.last().map_or_else(Digest::default, |e| e.id);
        let log = Self { base_seq: 0, base_head: Digest::default(), entries, head };
        log.verify()?;
        Ok(log)
    }
}

impl<E> Log<E> {
    /// An empty log that continues a chain from a known point (after a snapshot).
    #[must_use]
    pub const fn starting_at(seq: u64, head: Digest) -> Self {
        Self { base_seq: seq, base_head: head, entries: Vec::new(), head }
    }

    /// The sequence number the next entry will get: the number of events ever recorded.
    #[must_use]
    pub fn next_seq(&self) -> u64 {
        self.base_seq.saturating_add(u64::try_from(self.entries.len()).unwrap_or(u64::MAX))
    }

    /// The retained entries (all of them, unless the log continues from a snapshot).
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

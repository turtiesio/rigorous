//! The append-only, hash-chained log: what makes history tamper-evident.

use crate::digest::{digest_of, Digest, EncodingError};
use crate::entry::Entry;
use crate::path::Path;
use serde::Serialize;

#[derive(Serialize)]
struct Link<'a, E> {
    prev: Digest,
    seq: u64,
    actor: &'a Path,
    cause: Option<Digest>,
    src: &'a Path,
    event: &'a E,
}

/// The chain does not hold at entry `seq` (`u64::MAX`: the head does not match the last entry).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BrokenChain {
    pub seq: u64,
}

/// Why a chain could not be verified.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ChainError {
    Broken(BrokenChain),
    Encoding(EncodingError),
}

impl From<EncodingError> for ChainError {
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
    pub fn verify(&self) -> Result<(), ChainError> {
        let mut prev = self.base_head;
        for (i, e) in self.entries.iter().enumerate() {
            let seq = self.base_seq.saturating_add(u64::try_from(i).map_err(|_| ChainError::Broken(BrokenChain { seq: e.seq }))?);
            if e.seq != seq || e.id != Self::link(prev, seq, &e.actor, e.cause, &e.src, &e.event)? {
                return Err(ChainError::Broken(BrokenChain { seq }));
            }
            prev = e.id;
        }
        if prev == self.head {
            Ok(())
        } else {
            Err(ChainError::Broken(BrokenChain { seq: u64::MAX }))
        }
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

    /// A log holding exactly these entries, starting from nothing. NOT verified: call
    /// [`Log::verify`].
    #[must_use]
    pub fn from_entries(entries: Vec<Entry<E>>) -> Self {
        let head = entries.last().map_or_else(Digest::default, |e| e.id);
        Self { base_seq: 0, base_head: Digest::default(), entries, head }
    }

    /// Test hook: overwrite an entry in place (to demonstrate that `verify` notices).
    pub fn tamper(&mut self, index: usize, f: impl FnOnce(&mut Entry<E>)) {
        if let Some(e) = self.entries.get_mut(index) {
            f(e);
        }
    }
}

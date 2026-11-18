//! How a log is written to and read from storage, and which format version this build writes.

use crate::chain::{BrokenChain, ChainError, Log};
use crate::digest::{Digest, EncodingError};
use crate::entry::Entry;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::io::{BufRead, Write};

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
    /// The file does not end with the mark that closes a complete log, or the mark does not
    /// match what precedes it: the history was cut short, or more was written after its end.
    Unterminated,
}

impl From<EncodingError> for LogError {
    fn from(e: EncodingError) -> Self {
        Self::Encoding(e)
    }
}

impl From<ChainError> for LogError {
    fn from(e: ChainError) -> Self {
        match e {
            ChainError::Broken(BrokenChain { seq }) => Self::BrokenChain { seq },
            ChainError::Encoding(e) => Self::Encoding(e),
        }
    }
}

/// The version of the on-disk log format written by this build.
pub const SCHEMA: u32 = 4;

#[derive(Serialize, Deserialize)]
pub(crate) struct Header {
    pub(crate) rigor_log_schema: u32,
}

/// The first schema whose files close with an end mark.
pub(crate) const FIRST_CLOSED_SCHEMA: u32 = 4;

/// The last line of a complete log: how many events were ever recorded, and the chain head.
/// It does not make a log tamper-proof (whoever can cut a file can write a mark); it makes a
/// log that lost its tail distinguishable from a whole one. Against deliberate rollback, keep
/// the head somewhere the log's writer cannot reach.
#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct End {
    pub(crate) rigor_log_end: (u64, Digest),
}

impl<E: Serialize> Log<E> {
    /// Write one JSON object per line.
    ///
    /// # Errors
    /// On I/O failure.
    pub fn save<W: Write>(&self, mut w: W) -> Result<(), LogError> {
        serde_json::to_writer(&mut w, &Header { rigor_log_schema: SCHEMA }).map_err(|x| LogError::Parse { line: 0, message: x.to_string() })?;
        w.write_all(b"\n").map_err(LogError::Io)?;
        for e in self.entries() {
            serde_json::to_writer(&mut w, e).map_err(|x| LogError::Parse { line: 0, message: x.to_string() })?;
            w.write_all(b"\n").map_err(LogError::Io)?;
        }
        let end = End { rigor_log_end: (self.next_seq(), self.head()) };
        serde_json::to_writer(&mut w, &end).map_err(|x| LogError::Parse { line: 0, message: x.to_string() })?;
        w.write_all(b"\n").map_err(LogError::Io)?;
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
        let mut end = None;
        for (i, line) in r.lines().enumerate() {
            let line = line.map_err(LogError::Io)?;
            if i == 0 {
                match serde_json::from_str::<Header>(&line) {
                    Ok(h) if h.rigor_log_schema == SCHEMA => continue,
                    Ok(h) => return Err(LogError::Schema { found: h.rigor_log_schema }),
                    Err(_) => return Err(LogError::Schema { found: 1 }),
                }
            }
            if end.is_some() {
                return Err(LogError::Unterminated);
            }
            if let Ok(mark) = serde_json::from_str::<End>(&line) {
                end = Some(mark);
                continue;
            }
            let e: Entry<E> = serde_json::from_str(&line).map_err(|x| LogError::Parse { line: i + 1, message: x.to_string() })?;
            entries.push(e);
        }
        let log = Self::from_entries(entries);
        log.verify()?;
        if end != Some(End { rigor_log_end: (log.next_seq(), log.head()) }) {
            return Err(LogError::Unterminated);
        }
        Ok(log)
    }
}

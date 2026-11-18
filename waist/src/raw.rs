//! How a stored log is checked without understanding its events: what makes migration checkable.

use crate::digest::{digest_bytes, Digest};
use crate::path::Path;
use crate::stored::{End, Header, LogError, FIRST_CLOSED_SCHEMA};
use serde::Deserialize;
use std::io::BufRead;

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
    let mut end = None;
    for (i, line) in r.lines().enumerate() {
        let line = line.map_err(LogError::Io)?;
        if i == 0 {
            if let Ok(h) = serde_json::from_str::<Header>(&line) {
                schema = h.rigor_log_schema;
                continue;
            }
        }
        if end.is_some() {
            return Err(LogError::Unterminated);
        }
        if let Ok(mark) = serde_json::from_str::<End>(&line) {
            end = Some(mark);
            continue;
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
        if raw.seq != expect || raw.id != digest_bytes(link.as_bytes()) {
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
    // Files written before logs were closed carry no mark; later ones must carry the right one.
    let closed = end == Some(End { rigor_log_end: (expect, prev) });
    if (schema >= FIRST_CLOSED_SCHEMA && !closed) || (schema < FIRST_CLOSED_SCHEMA && end.is_some()) {
        return Err(LogError::Unterminated);
    }
    Ok(RawLog { schema, head: prev, entries })
}

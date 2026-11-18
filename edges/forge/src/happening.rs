//! A public event of a forge: read from the archive, exported to and imported from a fixture.

use crate::error::*;
use flate2::read::GzDecoder;
use rigor_institution::model::{ActorId, ChangeId, Time};
use rigor_waist::{digest_bytes, Name, NameError};
use serde::Deserialize;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// What a review said.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    Approved,
    ChangesRequested,
    /// The reviewer's earlier review was dismissed.
    Dismissed,
}

/// One thing that happened to a pull request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum What {
    Opened,
    Reviewed(Verdict),
    Merged,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Happening {
    pub change: ChangeId,
    /// The author, the reviewer whose review this is, or whoever merged.
    pub who: ActorId,
    pub what: What,
    pub at: Time,
}

#[derive(Deserialize)]
pub(crate) struct Login {
    login: String,
}

#[derive(Deserialize)]
pub(crate) struct Repo {
    name: String,
}

#[derive(Deserialize)]
pub(crate) struct Numbered {
    number: u64,
}

#[derive(Deserialize)]
pub(crate) struct Review {
    state: String,
    user: Option<Login>,
}

#[derive(Deserialize)]
pub(crate) struct Payload {
    action: Option<String>,
    number: Option<u64>,
    review: Option<Review>,
    pull_request: Option<Numbered>,
}

#[derive(Deserialize)]
pub(crate) struct Raw {
    #[serde(rename = "type")]
    kind: String,
    actor: Login,
    repo: Repo,
    created_at: String,
    payload: Payload,
}

pub(crate) fn person(login: &str) -> Result<ActorId, NameError> {
    Ok(ActorId(Name::new(&format!("u-{}", digest_bytes(login.to_lowercase().as_bytes())))?))
}

pub(crate) fn change(repo: &str, number: u64) -> Result<ChangeId, NameError> {
    Ok(ChangeId(Name::new(&format!("pr-{:016x}", digest_bytes(format!("{repo}#{number}").as_bytes()).0))?))
}

/// Seconds since the epoch for a UTC timestamp `YYYY-MM-DDTHH:MM:SSZ`.
pub(crate) fn seconds(stamp: &str) -> Option<u64> {
    let n = |range: std::ops::Range<usize>| stamp.get(range).and_then(|s| s.parse::<u64>().ok());
    let (y, m, d, h, mi, s) = (n(0..4)?, n(5..7)?, n(8..10)?, n(11..13)?, n(14..16)?, n(17..19)?);
    // Days from civil date (proleptic Gregorian), for years after 1970.
    let (y2, m2) = if m <= 2 { (y - 1, m + 9) } else { (y, m - 3) };
    let days = 365 * y2 + y2 / 4 - y2 / 100 + y2 / 400 + (153 * m2 + 2) / 5 + d - 1;
    let epoch = 365 * 1969 + 1969 / 4 - 1969 / 100 + 1969 / 400 + (153 * 10 + 2) / 5; // 1970-01-01 in the same count
    Some((days.checked_sub(epoch)?) * 86_400 + h * 3600 + mi * 60 + s)
}

/// Read one hourly archive file (gzip, one JSON event per line), keeping only what the model uses.
///
/// # Errors
/// If the file cannot be read. Lines that are not recognised events are skipped.
pub fn read_archive(path: &Path, into: &mut Vec<Happening>) -> Result<(), ForgeError> {
    let file = std::fs::File::open(path).map_err(ForgeError::Io)?;
    for line in BufReader::new(GzDecoder::new(file)).lines() {
        let Ok(line) = line else { break };
        let Ok(raw) = serde_json::from_str::<Raw>(&line) else { continue };
        let Some(at) = seconds(&raw.created_at).map(Time) else { continue };
        let found = match raw.kind.as_str() {
            "PullRequestEvent" => raw.payload.number.and_then(|n| match raw.payload.action.as_deref() {
                Some("opened") => Some((n, raw.actor.login.clone(), What::Opened)),
                Some("merged") => Some((n, raw.actor.login.clone(), What::Merged)),
                _ => None,
            }),
            "PullRequestReviewEvent" => raw.payload.pull_request.as_ref().zip(raw.payload.review.as_ref()).and_then(|(pr, review)| {
                let reviewer = review.user.as_ref().map_or_else(|| raw.actor.login.clone(), |u| u.login.clone());
                let verdict = match review.state.as_str() {
                    "approved" => Some(Verdict::Approved),
                    "changes_requested" => Some(Verdict::ChangesRequested),
                    "dismissed" => Some(Verdict::Dismissed),
                    _ => None,
                };
                verdict.map(|v| (pr.number, reviewer, What::Reviewed(v)))
            }),
            _ => None,
        };
        if let Some((number, login, what)) = found {
            if let (Ok(change), Ok(who)) = (change(&raw.repo.name, number), person(&login)) {
                into.push(Happening { change, who, what, at });
            }
        }
    }
    Ok(())
}

/// The reduced, pseudonymous form: one happening per line, `<O|A|C|D|M> <change> <who> <seconds>`.
#[must_use]
pub fn export(all: &[Happening]) -> String {
    let mut out = String::new();
    for h in all {
        let letter = match h.what {
            What::Opened => 'O',
            What::Reviewed(Verdict::Approved) => 'A',
            What::Reviewed(Verdict::ChangesRequested) => 'C',
            What::Reviewed(Verdict::Dismissed) => 'D',
            What::Merged => 'M',
        };
        out.push_str(&format!("{letter} {} {} {}\n", h.change, h.who, h.at.0));
    }
    out
}

/// Read back what [`export`] wrote.
///
/// # Errors
/// If a line is malformed.
pub fn import(text: &str) -> Result<Vec<Happening>, ForgeError> {
    let mut out = Vec::new();
    for (i, line) in text.lines().enumerate() {
        let bad = || ForgeError::Malformed { line: i + 1 };
        let fields: Vec<&str> = line.split(' ').collect();
        let [letter, change, who, at] = fields.as_slice() else { return Err(bad()) };
        let what = match *letter {
            "O" => What::Opened,
            "A" => What::Reviewed(Verdict::Approved),
            "C" => What::Reviewed(Verdict::ChangesRequested),
            "D" => What::Reviewed(Verdict::Dismissed),
            "M" => What::Merged,
            _ => return Err(bad()),
        };
        out.push(Happening { change: ChangeId(Name::new(change)?), who: ActorId(Name::new(who)?), what, at: Time(at.parse().map_err(|_| bad())?) });
    }
    Ok(out)
}

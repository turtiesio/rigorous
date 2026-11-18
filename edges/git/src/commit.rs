//! A commit as the adapter sees it: parsed from git, exported to and imported from a fixture.

use crate::error::*;
use rigor_institution::model::{ActorId, ChangeId, Time};
use rigor_waist::{digest_bytes, Name, NameError};
use std::path::Path;
use std::process::Command;

/// Someone who appears in a history.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Identity {
    pub id: ActorId,
    /// Declared to be an automated account by its own name or address.
    pub automated: bool,
}

/// One commit, reduced to what the model uses.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Commit {
    pub id: ChangeId,
    pub author: Identity,
    pub committer: Identity,
    pub committed: Time,
    /// Hash prefixes this commit says it reverts (as written; possibly abbreviated or unknown).
    pub reverts: Vec<String>,
    /// People the commit message records as having reviewed or acknowledged the change
    /// (`Reviewed-by:` / `Acked-by:` trailers), other than its author.
    pub reviewers: Vec<ActorId>,
    /// People a forge records as having requested changes and never approved (see [`apply_reviews`]).
    pub objectors: Vec<ActorId>,
}

pub(crate) fn identity(email: &str, name: &str) -> Result<Identity, NameError> {
    let key = email.trim().to_lowercase();
    let id = ActorId(Name::new(&format!("u-{}", digest_bytes(key.as_bytes())))?);
    let automated = [email, name].iter().any(|s| s.contains("[bot]") || s.to_lowercase().contains("noreply@github"));
    Ok(Identity { id, automated })
}

/// The hash prefixes following each "This reverts commit " in a message.
pub(crate) fn reverts_in(message: &str) -> Vec<String> {
    const MARK: &str = "This reverts commit ";
    message
        .match_indices(MARK)
        .filter_map(|(i, _)| message.get(i + MARK.len()..))
        .map(|rest| rest.chars().take_while(char::is_ascii_hexdigit).collect::<String>().to_lowercase())
        .filter(|h| h.len() >= 7)
        .collect()
}

/// The people named in `Reviewed-by:` and `Acked-by:` trailers, by address.
pub(crate) fn reviewers_in(message: &str) -> Vec<ActorId> {
    let mut out = Vec::new();
    for line in message.lines() {
        let lower = line.trim_start().to_lowercase();
        if !(lower.starts_with("reviewed-by:") || lower.starts_with("acked-by:")) {
            continue;
        }
        let address = line.split_once('<').and_then(|(_, rest)| rest.split_once('>')).map(|(a, _)| a);
        if let Some(id) = address.and_then(|a| identity(a, "").ok()) {
            if !out.contains(&id.id) {
                out.push(id.id);
            }
        }
    }
    out
}

/// Parse the output of [`LOG_FORMAT`].
///
/// # Errors
/// If a record is malformed.
pub fn parse(output: &str) -> Result<Vec<Commit>, GitError> {
    let mut commits = Vec::new();
    for (i, record) in output.split('\u{1e}').map(str::trim_start).filter(|r| !r.is_empty()).enumerate() {
        let fields: Vec<&str> = record.splitn(7, '\u{1f}').collect();
        let [hash, author_email, author_name, committer_email, committer_name, committed, message] = fields.as_slice() else {
            return Err(GitError::Malformed { record: i });
        };
        let author = identity(author_email, author_name)?;
        let reviewers = reviewers_in(message).into_iter().filter(|r| *r != author.id).collect();
        commits.push(Commit {
            id: ChangeId(Name::new(hash)?),
            author,
            committer: identity(committer_email, committer_name)?,
            committed: Time(committed.trim().parse::<u64>().map_err(|_| GitError::Malformed { record: i })?),
            reverts: reverts_in(message),
            reviewers,
            objectors: Vec::new(),
        });
    }
    Ok(commits)
}

/// Fields separated by U+001F, records by U+001E. `%aE`/`%cE` apply the repository's mailmap,
/// so one person with several addresses is one identity wherever the project has said so.
pub const LOG_FORMAT: &str = "--format=%H%x1f%aE%x1f%aN%x1f%cE%x1f%cN%x1f%ct%x1f%B%x1e";

/// A history in reduced, pseudonymous form: one line per commit, fields separated by spaces:
/// hash, author id, author automated (0/1), committer id, committer automated, time, reverts
/// (comma-separated, or `-`), reviewers (comma-separated ids, or `-`). This is what the test fixture holds: real structure, no personal data.
#[must_use]
pub fn export(commits: &[Commit]) -> String {
    let mut out = String::new();
    for c in commits {
        let reverts = if c.reverts.is_empty() { "-".to_owned() } else { c.reverts.join(",") };
        let reviewers = if c.reviewers.is_empty() { "-".to_owned() } else { c.reviewers.iter().map(ToString::to_string).collect::<Vec<_>>().join(",") };
        out.push_str(&format!(
            "{} {} {} {} {} {} {} {}\n",
            c.id,
            c.author.id,
            u8::from(c.author.automated),
            c.committer.id,
            u8::from(c.committer.automated),
            c.committed.0,
            reverts,
            reviewers
        ));
    }
    out
}

/// Read back what [`export`] wrote.
///
/// # Errors
/// If a line is malformed.
pub fn import(text: &str) -> Result<Vec<Commit>, GitError> {
    let mut commits = Vec::new();
    for (record, line) in text.lines().enumerate() {
        let fields: Vec<&str> = line.split(' ').collect();
        let [hash, author, a_auto, committer, c_auto, time, reverts, reviewers] = fields.as_slice() else {
            return Err(GitError::Malformed { record });
        };
        let who = |id: &str, auto: &str| -> Result<Identity, GitError> { Ok(Identity { id: ActorId(Name::new(id)?), automated: auto == "1" }) };
        commits.push(Commit {
            id: ChangeId(Name::new(hash)?),
            author: who(author, a_auto)?,
            committer: who(committer, c_auto)?,
            committed: Time(time.parse::<u64>().map_err(|_| GitError::Malformed { record })?),
            reverts: if *reverts == "-" { Vec::new() } else { reverts.split(',').map(str::to_owned).collect() },
            reviewers: if *reviewers == "-" { Vec::new() } else { reviewers.split(',').map(|r| Name::new(r).map(ActorId)).collect::<Result<_, _>>()? },
            objectors: Vec::new(),
        });
    }
    Ok(commits)
}

/// Read a repository's history, oldest first.
///
/// # Errors
/// If git cannot be run or its output cannot be parsed.
pub fn read(repo: &Path, limit: Option<usize>) -> Result<Vec<Commit>, GitError> {
    let mut cmd = Command::new("git");
    let _ = cmd.arg("--git-dir").arg(repo).arg("log").arg("--reverse").arg(LOG_FORMAT);
    let out = cmd.output().map_err(GitError::Io)?;
    if !out.status.success() {
        return Err(GitError::Git(out.status.code().unwrap_or(-1)));
    }
    let mut commits = parse(&String::from_utf8_lossy(&out.stdout))?;
    if let Some(n) = limit {
        commits.truncate(n);
    }
    Ok(commits)
}

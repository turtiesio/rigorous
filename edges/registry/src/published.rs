//! Index records: releases, their requirements, yanks; parsed from the index.

use crate::error::*;
use rigor_institution::model::Release;
use rigor_waist::Name;
use serde::Deserialize;

/// How a requirement constrains versions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Op {
    /// Compatible updates (the default): same leading non-zero component, not older.
    Caret,
    /// Same major and minor, not older.
    Tilde,
    Exact,
    AtLeast,
    Any,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Requirement {
    pub package: Name,
    pub op: Op,
    pub major: u64,
    pub minor: Option<u64>,
    pub patch: Option<u64>,
}

impl Requirement {
    /// Whether a version satisfies it.
    #[must_use]
    pub fn admits(&self, v: (u64, u64, u64)) -> bool {
        let floor = (self.major, self.minor.unwrap_or(0), self.patch.unwrap_or(0));
        match self.op {
            Op::Any => true,
            Op::AtLeast => v >= floor,
            Op::Exact => v.0 == self.major && self.minor.is_none_or(|m| m == v.1) && self.patch.is_none_or(|p| p == v.2),
            Op::Tilde => v >= floor && v.0 == self.major && self.minor.is_none_or(|m| m == v.1),
            Op::Caret => {
                v >= floor
                    && if self.major > 0 {
                        v.0 == self.major
                    } else {
                        match self.minor {
                            None => v.0 == 0,
                            Some(0) if self.patch.is_some() => v.0 == 0 && v.1 == 0 && Some(v.2) == self.patch,
                            Some(m) => v.0 == 0 && v.1 == m,
                        }
                    }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Published {
    pub release: Release,
    pub yanked: bool,
    pub requires: Vec<Requirement>,
}

pub(crate) fn package(name: &str) -> Option<Name> {
    Name::new(&name.to_lowercase()).ok()
}

pub(crate) fn version(text: &str) -> Option<(u64, u64, u64)> {
    let mut parts = text.split('.').map(str::parse::<u64>);
    match (parts.next(), parts.next(), parts.next(), parts.next()) {
        (Some(Ok(a)), Some(Ok(b)), Some(Ok(c)), None) => Some((a, b, c)),
        _ => None,
    }
}

/// Read one requirement as written in the index (only its first comparator is used).
pub(crate) fn requirement(dep: &str, written: &str) -> Option<Requirement> {
    let first = written.split(',').next().unwrap_or("").trim();
    let (op, rest) = [("^", Op::Caret), ("~", Op::Tilde), (">=", Op::AtLeast), (">", Op::AtLeast), ("=", Op::Exact), ("<=", Op::Any), ("<", Op::Any)]
        .into_iter()
        .find_map(|(sign, op)| first.strip_prefix(sign).map(|rest| (op, rest.trim())))
        .unwrap_or((Op::Caret, first));
    let mut numbers = rest.split('.').map(|p| p.parse::<u64>().ok());
    let (major, minor, patch) = (numbers.next().flatten(), numbers.next().flatten(), numbers.next().flatten());
    let package = package(dep)?;
    Some(match major {
        Some(major) => Requirement { package, op, major, minor, patch },
        None => Requirement { package, op: Op::Any, major: 0, minor: None, patch: None },
    })
}

#[derive(Deserialize)]
pub(crate) struct RawLine {
    n: String,
    v: String,
    y: bool,
    d: Vec<(String, String)>,
}

/// Parse index records as fetched (one JSON object per line: name, version, yanked, normal
/// non-optional dependencies). Pre-releases and unparseable names are skipped and counted.
///
/// # Errors
/// If a line is not such a record.
pub fn parse(text: &str) -> Result<(Vec<Published>, usize), RegistryError> {
    let (mut out, mut skipped) = (Vec::new(), 0);
    for (i, line) in text.lines().enumerate() {
        let raw: RawLine = serde_json::from_str(line).map_err(|_| RegistryError::Malformed { line: i + 1 })?;
        match (package(&raw.n), version(&raw.v)) {
            (Some(package), Some(version)) => {
                let requires = raw.d.iter().filter_map(|(dep, req)| requirement(dep, req)).collect();
                out.push(Published { release: Release { package, version }, yanked: raw.y, requires });
            }
            _ => skipped += 1,
        }
    }
    Ok((out, skipped))
}

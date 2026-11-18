//! Adapter: a real package registry (the crates.io index).
//!
//! A third kind of real record, with two things the others lack: evidence that is DERIVED
//! from other evidence (a release rests on the releases it depends on), and RETRACTION (a
//! publisher yanks a release).
//!
//! The mapping:
//!
//! * each release is attested fit by its publisher, derived from the attestations of the
//!   releases its dependencies resolve to;
//! * the publisher undertakes to the registry that the release is fit;
//! * a yank is the publisher retracting its own attestation; everything derived from it loses
//!   its support with it.
//!
//! Dependency resolution here is deliberately simple and is an INTERPRETATION: each
//! requirement resolves to the highest known release it admits, yanked or not, so "fit" means
//! "not yanked, and not resting on a yanked release by that resolution". It is not what a
//! package manager would install today.

use rigor_institution::model::{
    About, Act, ActionKind, ActorId, Assessment, Attestation, CapId, Charter, CharterName, Condition, Constitution, Content, Event, EvidenceId, Member,
    MemberKind, Method, Offer, Prop, Rank, Release, Status, Tag, Time, Trust,
};
use rigor_institution::org::{org, standard_rules, Trusted};
use rigor_institution::world::{Error, World, CONSTITUTION};
use rigor_kernel::{Name, NameError};
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};

pub const REGISTRY: ActorId = ActorId::lit("the-registry");
pub const REGISTRAR: ActorId = ActorId::lit("the-registrar");
pub const ADMISSIONS: CharterName = CharterName::lit("admissions");

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

/// Why registry data could not be read.
#[derive(Debug)]
pub enum RegistryError {
    Malformed { line: usize },
    Name(NameError),
    World(Error),
}
impl From<Error> for RegistryError {
    fn from(e: Error) -> Self {
        Self::World(e)
    }
}

fn package(name: &str) -> Option<Name> {
    Name::new(&name.to_lowercase()).ok()
}

fn version(text: &str) -> Option<(u64, u64, u64)> {
    let mut parts = text.split('.').map(str::parse::<u64>);
    match (parts.next(), parts.next(), parts.next(), parts.next()) {
        (Some(Ok(a)), Some(Ok(b)), Some(Ok(c)), None) => Some((a, b, c)),
        _ => None,
    }
}

/// Read one requirement as written in the index (only its first comparator is used).
fn requirement(dep: &str, written: &str) -> Option<Requirement> {
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
struct RawLine {
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

/// The release each requirement of each release resolves to: the highest known one it admits.
#[must_use]
pub fn resolve(all: &[Published]) -> BTreeMap<Release, Vec<Release>> {
    let mut by_package: BTreeMap<&Name, Vec<(u64, u64, u64)>> = BTreeMap::new();
    for p in all {
        by_package.entry(&p.release.package).or_default().push(p.release.version);
    }
    all.iter()
        .map(|p| {
            let deps = p
                .requires
                .iter()
                .filter_map(|r| {
                    let best = by_package.get(&r.package)?.iter().copied().filter(|v| r.admits(*v)).max()?;
                    Some(Release { package: r.package.clone(), version: best })
                })
                .filter(|d| *d != p.release)
                .collect();
            (p.release.clone(), deps)
        })
        .collect()
}

/// Independent computation of which releases are fit: not yanked, and resting on no release
/// that is not fit. Plain memoised recursion; a release on a dependency cycle is not fit.
#[must_use]
pub fn oracle(all: &[Published]) -> BTreeSet<Release> {
    let deps = resolve(all);
    let yanked: BTreeSet<&Release> = all.iter().filter(|p| p.yanked).map(|p| &p.release).collect();
    let mut known: BTreeMap<Release, bool> = BTreeMap::new();
    fn fit(
        r: &Release,
        deps: &BTreeMap<Release, Vec<Release>>,
        yanked: &BTreeSet<&Release>,
        known: &mut BTreeMap<Release, bool>,
        path: &mut Vec<Release>,
    ) -> bool {
        if let Some(k) = known.get(r) {
            return *k;
        }
        if path.contains(r) {
            return false;
        }
        path.push(r.clone());
        let ok = !yanked.contains(r) && deps.get(r).is_some_and(|ds| ds.iter().all(|d| fit(d, deps, yanked, known, path)));
        let _ = path.pop();
        let _ = known.insert(r.clone(), ok);
        ok
    }
    for p in all {
        let _ = fit(&p.release, &deps, &yanked, &mut known, &mut Vec::new());
    }
    known.into_iter().filter(|(_, ok)| *ok).map(|(r, _)| r).collect()
}

/// What ingestion met.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Stats {
    pub packages: usize,
    pub releases: usize,
    pub yanked: usize,
    pub dependencies_resolved: usize,
    pub dependencies_unresolved: usize,
    /// Releases left out because their dependencies form a cycle under this resolution.
    pub on_a_cycle: usize,
    pub refused: usize,
}

/// A founded registry world.
///
/// # Errors
/// On a kernel fault.
pub fn found() -> Result<World, Error> {
    let m = |id: ActorId, kind| Member { id, kind, principal: None, role: None };
    let mut rules = standard_rules(&Trusted { receipts: REGISTRY, probes: REGISTRY, tests: REGISTRY, audits: REGISTRY, acks: REGISTRY });
    rules.evidence.ranks = std::iter::once((Method::Receipt, Rank(1))).collect();
    rules.evidence.trusted = std::iter::once((Method::Receipt, Trust::Anyone)).collect();
    let constitution = Constitution {
        members: vec![m(REGISTRY, MemberKind::Org), m(REGISTRAR, MemberKind::Service)],
        charters: vec![Charter { name: ADMISSIONS, holder: REGISTRAR, actions: std::iter::once(ActionKind::Admit).collect(), delegable: false }],
        balances: Vec::new(),
        rules,
    };
    let mut w = World::new(Box::new(org(Vec::new())?));
    let _ = w.inject(&CONSTITUTION, Event::Genesis(Box::new(constitution)))?;
    Ok(w)
}

/// Publish every release (dependencies first), then apply every yank.
///
/// # Errors
/// On a kernel fault.
pub fn ingest(w: &mut World, all: &[Published]) -> Result<Stats, Error> {
    let deps = resolve(all);
    let mut stats = Stats { releases: all.len(), yanked: all.iter().filter(|p| p.yanked).count(), ..Stats::default() };
    for p in all {
        let resolved = deps.get(&p.release).map_or(0, Vec::len);
        stats.dependencies_resolved += resolved;
        stats.dependencies_unresolved += p.requires.len().saturating_sub(resolved);
    }
    // Dependencies first: an iterative depth-first order; whatever sits on a cycle is left out.
    let mut order: Vec<&Release> = Vec::new();
    let mut state: BTreeMap<&Release, bool> = BTreeMap::new(); // false = in progress, true = done
    for p in all {
        let mut stack: Vec<(&Release, usize)> = vec![(&p.release, 0)];
        while let Some((r, i)) = stack.pop() {
            if i == 0 {
                if state.contains_key(r) {
                    continue;
                }
                let _ = state.insert(r, false);
            }
            match deps.get(r).and_then(|ds| ds.get(i)) {
                Some(d) => {
                    stack.push((r, i + 1));
                    if !state.contains_key(d) {
                        stack.push((d, 0));
                    }
                }
                None => {
                    let _ = state.insert(r, true);
                    order.push(r);
                }
            }
        }
    }
    let mut evidence: BTreeMap<&Release, EvidenceId> = BTreeMap::new();
    let mut known: BTreeSet<ActorId> = BTreeSet::new();
    for r in order {
        let publisher = ActorId(r.package.clone());
        if known.insert(publisher.clone()) {
            stats.packages += 1;
            let member = Member { id: publisher.clone(), kind: MemberKind::Org, principal: None, role: None };
            let _ = w.act_with(&REGISTRAR, &CapId::Charter(ADMISSIONS), Act::Admit(member))?;
        }
        let parents: Option<BTreeSet<EvidenceId>> = deps.get(r).map(|ds| ds.iter().map(|d| evidence.get(d).copied()).collect()).unwrap_or_default();
        let Some(derived_from) = parents else {
            stats.on_a_cycle += 1;
            continue;
        };
        let fit = Prop::Fit(r.clone());
        let offer = Offer {
            creditor: REGISTRY,
            ant: None,
            cons: Condition::Holds(fit.clone()),
            deadline: Time(1),
            tags: std::iter::once(Tag::Upkeep).collect(),
            parent: None,
            terms: None,
        };
        let _ = w.act(&publisher, Act::Commit(offer))?;
        let a = Attestation {
            about: About::Supports(fit),
            method: Method::Receipt,
            derived_from,
            artifacts: BTreeMap::new(),
            content: Content::Note("published".to_owned()),
        };
        let entries = w.act(&publisher, Act::Attest(a))?;
        match entries.iter().find_map(|e| match &e.event {
            Event::EvidenceRecorded(ev) => Some(ev.id),
            _ => None,
        }) {
            Some(id) => {
                let _ = evidence.insert(r, id);
            }
            None => stats.refused += 1,
        }
    }
    for p in all.iter().filter(|p| p.yanked) {
        if let Some(id) = evidence.get(&p.release) {
            let entries = w.act(&ActorId(p.release.package.clone()), Act::Retract { evidence: *id })?;
            stats.refused += usize::from(!entries.iter().any(|e| matches!(e.event, Event::EvidenceRetracted { .. })));
        }
    }
    Ok(stats)
}

/// The releases the system currently assesses as fit, and commitment statuses.
///
/// # Errors
/// If the world lacks an institutional unit.
pub fn conclusions(w: &World) -> Result<(BTreeSet<Release>, BTreeMap<Status, usize>), Error> {
    let fit = w
        .epi()?
        .standings()
        .filter_map(|(p, s)| match (p, s.assessment) {
            (Prop::Fit(r), Assessment::Accepted) => Some(r.clone()),
            _ => None,
        })
        .collect();
    let mut by_status = BTreeMap::new();
    for r in w.norms()?.records() {
        if let Some(s) = r.status {
            *by_status.entry(s).or_default() += 1;
        }
    }
    Ok((fit, by_status))
}

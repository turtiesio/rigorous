//! Independent resolution of the same index, and what the world concluded.

use crate::published::*;
use rigor_institution::model::{Assessment, Prop, Release, Status};
use rigor_institution::world::{Error, World};
use rigor_waist::Name;
use std::collections::{BTreeMap, BTreeSet};

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

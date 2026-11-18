//! Which evidence counts at all: who stands behind it, how strong its method is, whether its source is disinterested.

use super::*;

impl Epistemic {
    /// The sources of the underived ancestors of an item. Memoised: a derivation graph shares
    /// ancestors, and walking every path separately is exponential in its depth.
    pub(super) fn roots(&self, id: EvidenceId, memo: &mut BTreeMap<EvidenceId, BTreeSet<ActorId>>) -> BTreeSet<ActorId> {
        if let Some(known) = memo.get(&id) {
            return known.clone();
        }
        let out = match self.ev.get(&id) {
            None => BTreeSet::new(),
            Some(item) if item.ev.derived_from.is_empty() => std::iter::once(item.ev.source.clone()).collect(),
            Some(item) => item.ev.derived_from.iter().flat_map(|p| self.roots(*p, memo)).collect(),
        };
        let _ = memo.insert(id, out.clone());
        out
    }

    /// Effective strength: the weakest link among the item and what it is derived from.
    pub(super) fn rank(&self, id: EvidenceId, memo: &mut BTreeMap<EvidenceId, Option<Rank>>) -> Option<Rank> {
        if let Some(known) = memo.get(&id) {
            return *known;
        }
        let out = self.ev.get(&id).and_then(|item| {
            let mut r = *self.rules.ranks.get(&item.ev.method)?;
            for p in &item.ev.derived_from {
                r = r.min(self.rank(*p, memo)?);
            }
            Some(r)
        });
        let _ = memo.insert(id, out);
        out
    }

    pub(super) fn admissible(&self, id: EvidenceId, memo: &mut BTreeMap<EvidenceId, bool>, roots: &mut BTreeMap<EvidenceId, BTreeSet<ActorId>>) -> bool {
        if let Some(known) = memo.get(&id) {
            return *known;
        }
        let ok = self.ev.get(&id).is_some_and(|item| {
            let e = &item.ev;
            let trusted = match self.rules.trusted.get(&e.method) {
                Some(Trust::Anyone) => true,
                Some(Trust::Only(set)) => set.contains(&e.source),
                None => false,
            };
            let fresh = e.artifacts.iter().all(|(a, d)| self.art.get(a) == Some(d));
            let independent = match &e.about {
                About::Supports(p) if !self.rules.self_support => !self.interested(p, self.roots(id, roots).into_iter()),
                About::Supports(_) | About::Rebuts(_) | About::Undercuts(_) => true,
            };
            !item.retracted
                && self.rules.ranks.contains_key(&e.method)
                && trusted
                && fresh
                && independent
                && e.derived_from.iter().all(|p| self.admissible(*p, memo, roots))
        });
        let _ = memo.insert(id, ok);
        ok
    }

    /// Whether any of these sources has a stake in the proposition being accepted.
    pub(super) fn interested(&self, p: &Prop, mut sources: impl Iterator<Item = ActorId>) -> bool {
        sources.any(|s| self.claims.get(p).is_some_and(|c| c.contains_key(&s)) || self.owing.get(p).is_some_and(|o| o.contains_key(&s)))
    }

    /// Whether an item passes every test that does not depend on other items' labels,
    /// given that what it is derived from is admissible.
    pub(super) fn passes(&self, item: &Item) -> bool {
        let e = &item.ev;
        let trusted = match self.rules.trusted.get(&e.method) {
            Some(Trust::Anyone) => true,
            Some(Trust::Only(set)) => set.contains(&e.source),
            None => false,
        };
        let independent = match &e.about {
            About::Supports(p) if !self.rules.self_support => !self.interested(p, item.roots.iter().cloned()),
            About::Supports(_) | About::Rebuts(_) | About::Undercuts(_) => true,
        };
        !item.retracted
            && self.rules.ranks.contains_key(&e.method)
            && trusted
            && e.artifacts.iter().all(|(a, d)| self.art.get(a) == Some(d))
            && independent
            && e.derived_from.iter().all(|p| self.ev.get(p).is_some_and(|x| x.admissible))
    }

    /// Effective rank from the recorded ranks of what an item is derived from.
    pub(super) fn rank_of(&self, method: Method, derived_from: &BTreeSet<EvidenceId>) -> Option<Rank> {
        let mut r = *self.rules.ranks.get(&method)?;
        for p in derived_from {
            r = r.min(self.ev.get(p)?.rank?);
        }
        Some(r)
    }
}

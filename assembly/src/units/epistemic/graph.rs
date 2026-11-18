//! How evidence is connected: what rests on what, and what attacks what.

use super::*;

impl Epistemic {
    pub(super) fn descendants(&self, id: EvidenceId) -> BTreeSet<EvidenceId> {
        let mut out = BTreeSet::new();
        let mut stack = vec![id];
        while let Some(x) = stack.pop() {
            for k in self.kids.get(&x).into_iter().flatten() {
                if out.insert(*k) {
                    stack.push(*k);
                }
            }
        }
        out
    }

    /// Everything connected to the seeds by shared propositions, undercuts or derivation.
    #[must_use]
    pub fn component(&self, seeds: impl IntoIterator<Item = EvidenceId>) -> BTreeSet<EvidenceId> {
        let mut out = BTreeSet::new();
        let mut stack: Vec<EvidenceId> = seeds.into_iter().filter(|s| self.ev.contains_key(s)).collect();
        while let Some(x) = stack.pop() {
            if !out.insert(x) {
                continue;
            }
            let Some(item) = self.ev.get(&x) else { continue };
            stack.extend(item.ev.derived_from.iter().copied());
            stack.extend(self.kids.get(&x).into_iter().flatten().copied());
            stack.extend(self.cutters.get(&x).into_iter().flatten().copied());
            match &item.ev.about {
                About::Undercuts(t) => stack.push(*t),
                About::Supports(p) | About::Rebuts(p) => stack.extend(self.by_prop.get(p).into_iter().flatten().copied()),
            }
        }
        out
    }

    /// Everything a change to the seeds can reach: evidence on the same proposition, the
    /// target of an undercut, and whatever is derived from any of those.
    pub(super) fn downstream(&self, seeds: impl IntoIterator<Item = EvidenceId>) -> BTreeSet<EvidenceId> {
        let mut out = BTreeSet::new();
        let mut stack: Vec<EvidenceId> = seeds.into_iter().filter(|s| self.ev.contains_key(s)).collect();
        while let Some(x) = stack.pop() {
            if !out.insert(x) {
                continue;
            }
            let Some(item) = self.ev.get(&x) else { continue };
            stack.extend(self.kids.get(&x).into_iter().flatten().copied());
            match &item.ev.about {
                About::Undercuts(t) => stack.push(*t),
                About::Supports(p) | About::Rebuts(p) => stack.extend(self.by_prop.get(p).into_iter().flatten().copied()),
            }
        }
        out
    }

    /// The admissible items that attack `y` itself (not through what it is derived from).
    pub(super) fn direct_attackers(&self, y: EvidenceId, into: &mut BTreeSet<EvidenceId>) {
        let Some(item) = self.ev.get(&y) else { return };
        // An undercut fails against strictly stronger evidence, like any other attack.
        into.extend(self.cutters.get(&y).into_iter().flatten().copied().filter(|c| self.ev.get(c).is_some_and(|x| x.admissible && item.rank <= x.rank)));
        let supports = match &item.ev.about {
            About::Supports(_) => true,
            About::Rebuts(_) => false,
            About::Undercuts(_) => return,
        };
        let Some(p) = item.prop() else { return };
        for other in self.by_prop.get(p).into_iter().flatten() {
            let Some(o) = self.ev.get(other).filter(|o| o.admissible) else { continue };
            let opposed = matches!(o.ev.about, About::Rebuts(_)) == supports;
            // An attack on strictly stronger evidence fails.
            if opposed && item.rank <= o.rank {
                let _ = into.insert(*other);
            }
        }
    }

    /// Everything that attacks `x`: its own attackers and those of everything it is derived from.
    pub(super) fn attackers(&self, x: EvidenceId) -> BTreeSet<EvidenceId> {
        let mut out = BTreeSet::new();
        let mut seen = BTreeSet::new();
        let mut stack = vec![x];
        while let Some(y) = stack.pop() {
            if seen.insert(y) {
                self.direct_attackers(y, &mut out);
                stack.extend(self.ev.get(&y).into_iter().flat_map(|i| i.ev.derived_from.iter().copied()));
            }
        }
        out
    }
}

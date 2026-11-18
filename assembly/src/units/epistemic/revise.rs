//! Revision: relabel only what lies downstream of a change, and announce what moved.

use super::*;

impl Epistemic {
    /// Revise what lies downstream of the seeds, and announce every assessment that changed.
    pub(super) fn revise(&mut self, seeds: impl IntoIterator<Item = EvidenceId>, extra: impl IntoIterator<Item = Prop>, out: &mut Vec<Event>) {
        let area = self.downstream(seeds);
        self.work.add(area.len());
        // 1. Admissibility, shallowest first so that what an item rests on is settled first.
        let mut ordered: Vec<(u64, EvidenceId)> = area.iter().filter_map(|e| self.ev.get(e).map(|i| (i.order, *e))).collect();
        ordered.sort_unstable();
        for (_, e) in &ordered {
            let ok = self.ev.get(e).is_some_and(|i| self.passes(i));
            let now = self.now;
            if let Some(i) = self.ev.get_mut(e) {
                if ok && !i.admissible {
                    i.counts_from = now;
                }
                i.admissible = ok;
            }
        }
        // 2. The attack graph over the area; attackers outside it enter with their recorded
        //    labels as fixed boundary conditions (`None` is a node that is always in).
        let mut nodes: BTreeSet<Option<EvidenceId>> = BTreeSet::new();
        let mut attacks: BTreeSet<(Option<EvidenceId>, Option<EvidenceId>)> = BTreeSet::new();
        for (_, x) in &ordered {
            if !self.ev.get(x).is_some_and(|i| i.admissible) {
                continue;
            }
            let _ = nodes.insert(Some(*x));
            for a in self.attackers(*x) {
                let _ = attacks.insert((Some(a), Some(*x)));
                if !area.contains(&a) && nodes.insert(Some(a)) {
                    match self.ev.get(&a).map(|i| i.label) {
                        Some(Label::In) | None => {}
                        Some(Label::Out) => {
                            let _ = nodes.insert(None);
                            let _ = attacks.insert((None, Some(a)));
                        }
                        Some(Label::Undecided) => {
                            let _ = attacks.insert((Some(a), Some(a)));
                        }
                    }
                }
            }
        }
        let labels = grounded(&nodes, &attacks);
        for (_, x) in &ordered {
            let label = labels.get(&Some(*x)).copied().unwrap_or(Label::Out);
            if let Some(i) = self.ev.get_mut(x) {
                i.label = label;
            }
        }
        // 3. The propositions touched.
        let mut props = self.props_of(&area);
        props.extend(extra);
        for p in props {
            let new = self.assess_recorded(&p);
            let old = self.standing(&p);
            if new != old {
                out.push(Event::Assessed {
                    prop: p.clone(),
                    assessment: new.assessment,
                    since: new.since,
                    basis: new.basis.clone(),
                    was: old.assessment,
                    rules: self.version,
                });
                let _ = self.status.insert(p, new);
            }
        }
    }
}

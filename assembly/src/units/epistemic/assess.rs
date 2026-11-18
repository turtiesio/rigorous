//! From labelled evidence to the standing of a proposition; and the from-scratch reference.

use super::*;

impl Epistemic {
    pub(super) fn label(&self, ids: &BTreeSet<EvidenceId>) -> BTreeMap<EvidenceId, Label> {
        let (mut memo, mut roots, mut ranks) = (BTreeMap::new(), BTreeMap::new(), BTreeMap::new());
        let adm: BTreeSet<EvidenceId> = ids.iter().copied().filter(|e| self.admissible(*e, &mut memo, &mut roots)).collect();
        let rank: BTreeMap<EvidenceId, Rank> = adm.iter().filter_map(|e| self.rank(*e, &mut ranks).map(|r| (*e, r))).collect();
        let mut direct: Vec<(EvidenceId, EvidenceId)> = Vec::new();
        let mut sides: BTreeMap<&Prop, (Vec<EvidenceId>, Vec<EvidenceId>)> = BTreeMap::new();
        for e in &adm {
            let Some(item) = self.ev.get(e) else { continue };
            match &item.ev.about {
                About::Supports(p) => sides.entry(p).or_default().0.push(*e),
                About::Rebuts(p) => sides.entry(p).or_default().1.push(*e),
                About::Undercuts(t) if adm.contains(t) && rank.get(t) <= rank.get(e) => direct.push((*e, *t)),
                About::Undercuts(_) => {}
            }
        }
        for (sup, reb) in sides.values() {
            for a in sup {
                for b in reb {
                    if let (Some(ra), Some(rb)) = (rank.get(a), rank.get(b)) {
                        if rb <= ra {
                            direct.push((*a, *b));
                        }
                        if ra <= rb {
                            direct.push((*b, *a));
                        }
                    }
                }
            }
        }
        let mut attacks: BTreeSet<(EvidenceId, EvidenceId)> = direct.iter().copied().collect();
        for (a, b) in &direct {
            attacks.extend(self.descendants(*b).into_iter().filter(|d| adm.contains(d)).map(|d| (*a, d)));
        }
        grounded(&adm, &attacks)
    }

    pub(super) fn assess(&self, p: &Prop, labels: &BTreeMap<EvidenceId, Label>) -> Standing {
        let (mut sup, mut reb, mut undecided) = (Vec::new(), Vec::new(), false);
        for e in self.by_prop.get(p).into_iter().flatten() {
            let Some(item) = self.ev.get(e) else { continue };
            match (labels.get(e), &item.ev.about) {
                (Some(Label::In), About::Supports(_)) => sup.push(item),
                (Some(Label::In), About::Rebuts(_)) => reb.push(item),
                (Some(Label::Undecided), _) => undecided = true,
                (Some(Label::In | Label::Out) | None, _) => {}
            }
        }
        if !reb.is_empty() {
            Standing { assessment: Assessment::Refuted, since: None, basis: reb.iter().map(|i| i.ev.id).collect() }
        } else if !sup.is_empty() {
            Standing { assessment: Assessment::Accepted, since: sup.iter().map(|i| i.counts_from).min(), basis: sup.iter().map(|i| i.ev.id).collect() }
        } else if undecided {
            Standing { assessment: Assessment::Contested, since: None, basis: Vec::new() }
        } else {
            Standing::default()
        }
    }

    /// From-scratch assessment of every proposition (the reference for the incremental path).
    #[must_use]
    pub fn full(&self) -> BTreeMap<Prop, Standing> {
        let all: BTreeSet<EvidenceId> = self.ev.keys().copied().collect();
        let labels = self.label(&all);
        self.by_prop.keys().chain(self.status.keys()).map(|p| (p.clone(), self.assess(p, &labels))).collect()
    }

    pub(super) fn assess_recorded(&self, p: &Prop) -> Standing {
        let labels: BTreeMap<EvidenceId, Label> =
            self.by_prop.get(p).into_iter().flatten().filter_map(|e| self.ev.get(e).filter(|i| i.admissible).map(|i| (*e, i.label))).collect();
        self.assess(p, &labels)
    }
}

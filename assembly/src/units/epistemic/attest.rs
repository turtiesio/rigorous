//! Recording an attestation, or refusing it with a reason.

use super::*;

impl Epistemic {
    pub(super) fn attest(&mut self, by: &ActorId, a: &crate::model::Attestation, out: &mut Vec<Event>) -> Result<(), LeafError> {
        let reject = |reason| Event::EvidenceRejected { by: by.clone(), reason };
        if let About::Undercuts(t) = &a.about {
            if !self.ev.contains_key(t) {
                out.push(reject(EvidenceError::UnknownTarget));
                return Ok(());
            }
        }
        if a.derived_from.iter().any(|p| !self.ev.contains_key(p)) {
            out.push(reject(EvidenceError::UnknownProvenance));
            return Ok(());
        }
        if a.method == Method::Inference {
            if a.derived_from.is_empty() {
                out.push(reject(EvidenceError::UngroundedInference));
                return Ok(());
            }
            let About::Supports(conclusion) = &a.about else {
                out.push(reject(EvidenceError::InferenceMustSupport));
                return Ok(());
            };
            let follows = a
                .derived_from
                .iter()
                .all(|p| matches!(self.ev.get(p).map(|i| &i.ev.about), Some(About::Supports(premise)) if conclusion.follows_from(premise)));
            if !follows {
                out.push(reject(EvidenceError::DoesNotFollow));
                return Ok(());
            }
        }
        let id = EvidenceId(digest_of(&(by, a, self.now))?);
        if self.ev.contains_key(&id) {
            out.push(reject(EvidenceError::Duplicate));
            return Ok(());
        }
        let ev = Evidence {
            id,
            source: by.clone(),
            about: a.about.clone(),
            method: a.method,
            derived_from: a.derived_from.clone(),
            artifacts: a.artifacts.clone(),
            content: a.content.clone(),
            at: self.now,
        };
        match &ev.about {
            About::Supports(p) | About::Rebuts(p) => {
                let _ = self.by_prop.entry(p.clone()).or_default().insert(id);
            }
            About::Undercuts(t) => {
                let _ = self.cutters.entry(*t).or_default().insert(id);
            }
        }
        for p in &ev.derived_from {
            let _ = self.kids.entry(*p).or_default().insert(id);
        }
        for art in ev.artifacts.keys() {
            let _ = self.by_art.entry(art.clone()).or_default().insert(id);
        }
        out.push(Event::EvidenceRecorded(ev.clone()));
        let roots: BTreeSet<ActorId> = if ev.derived_from.is_empty() {
            std::iter::once(ev.source.clone()).collect()
        } else {
            ev.derived_from.iter().filter_map(|p| self.ev.get(p)).flat_map(|i| i.roots.iter().cloned()).collect()
        };
        let rank = self.rank_of(ev.method, &ev.derived_from);
        let depth = ev.derived_from.iter().filter_map(|p| self.ev.get(p)).map(|i| i.order).max().map_or(0, |d| d + 1);
        let _ = self.ev.insert(id, Item { ev, retracted: false, order: depth, roots, rank, counts_from: self.now, admissible: false, label: Label::Out });
        self.revise([id], [], out);
        Ok(())
    }
}

//! What happens when the evidence unit hears something.

use super::*;

impl Leaf<Event> for Epistemic {
    fn name(&self) -> Name {
        EPI
    }
    fn inputs(&self) -> BTreeSet<Kind> {
        [Kind::Tick, Kind::RulesChanged, Kind::Ok, Kind::ArtifactChanged, Kind::NormCreated, Kind::NormStatus].into_iter().collect()
    }
    fn outputs(&self) -> BTreeSet<Kind> {
        [Kind::Assessed, Kind::EvidenceRecorded, Kind::EvidenceRejected, Kind::EvidenceRetracted].into_iter().collect()
    }
    fn on(&mut self, env: &Envelope<'_, Event>) -> Result<Vec<Event>, LeafError> {
        let mut out = Vec::new();
        match env.event {
            Event::Tick(t) => self.now = self.now.max(*t),
            Event::RulesChanged(rc) => {
                self.rules = rc.rules.evidence.clone();
                self.version = rc.version;
                // Ranks are rule-dependent: recompute them shallowest first, then revise everything.
                let mut ordered: Vec<(u64, EvidenceId)> = self.ev.iter().map(|(e, i)| (i.order, *e)).collect();
                ordered.sort_unstable();
                for (_, e) in &ordered {
                    let rank = self.ev.get(e).and_then(|i| self.rank_of(i.ev.method, &i.ev.derived_from));
                    if let Some(i) = self.ev.get_mut(e) {
                        i.rank = rank;
                    }
                }
                let known: Vec<Prop> = self.status.keys().cloned().collect();
                self.revise(ordered.into_iter().map(|(_, e)| e), known, &mut out);
            }
            Event::ArtifactChanged { artifact, digest } => {
                if self.art.get(artifact) != Some(digest) {
                    let _ = self.art.insert(artifact.clone(), *digest);
                    self.revise(self.evidence_about_artifact(artifact), [], &mut out);
                }
            }
            Event::NormCreated(c) => {
                // Whoever owes a commitment has a stake in the proposition that would discharge it.
                if let crate::model::Condition::Holds(p) | crate::model::Condition::Unrefuted(p) = &c.terms.cons {
                    let _ = self.owed.insert(c.id, (p.clone(), c.terms.debtor.clone()));
                    let theirs = self.owing.entry(p.clone()).or_default().entry(c.terms.debtor.clone()).or_default();
                    let newly = theirs.is_empty();
                    let _ = theirs.insert(c.id);
                    if newly {
                        self.revise(self.evidence_on(p), [p.clone()], &mut out);
                    }
                }
            }
            Event::NormStatus { id, closed: true, .. } => {
                // A commitment that nothing can change any more gives its debtor no stake.
                if let Some((p, debtor)) = self.owed.remove(id) {
                    let stakes = self.owing.entry(p.clone()).or_default();
                    let none_left = stakes.get_mut(&debtor).is_some_and(|theirs| theirs.remove(id) && theirs.is_empty());
                    if none_left {
                        let _ = stakes.remove(&debtor);
                        self.revise(self.evidence_on(&p), [p], &mut out);
                    }
                }
            }
            Event::Ok { by, act: Act::Claim { prop }, .. } => {
                let claimants = self.claims.entry(prop.clone()).or_default();
                if !claimants.contains_key(by) {
                    let _ = claimants.insert(by.clone(), self.now);
                    self.revise(self.evidence_on(prop), [prop.clone()], &mut out);
                }
            }
            Event::Ok { by, act: Act::Retract { evidence }, .. } => match self.ev.get_mut(evidence) {
                Some(item) if item.ev.source == *by => {
                    if !item.retracted {
                        item.retracted = true;
                        out.push(Event::EvidenceRetracted { id: *evidence, by: by.clone() });
                        self.revise([*evidence], [], &mut out);
                    }
                }
                Some(_) | None => out.push(Event::EvidenceRejected { by: by.clone(), reason: EvidenceError::NotTheSource }),
            },
            Event::Ok { by, act: Act::Attest(a), .. } => self.attest(by, a, &mut out)?,
            _ => {}
        }
        Ok(out)
    }
}

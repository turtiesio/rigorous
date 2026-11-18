//! Judging one family: run the rules over its facts and announce what changed.

use super::*;

impl Adjudicator {
    pub(super) fn adjudicate(&mut self, root: CommitmentId, out: &mut Vec<Event>) -> Result<(), LeafError> {
        let fam = self.family(root);
        // Nothing violated and nothing previously found: there is nothing to judge or withdraw.
        let live =
            fam.iter().any(|id| self.c.get(id).is_some_and(|k| k.status == Some(Status::Violated)) || self.verdicts.get(id).is_some_and(|f| !f.is_empty()));
        if !live {
            return Ok(());
        }
        self.work.add(1);
        let version = if self.rules.retroactive { self.version } else { self.arose.get(&root).copied().unwrap_or(self.version) };
        let rules = self.versions.get(&version).unwrap_or(&self.rules).clone();
        let model = datalog::solve(&rules.rules, self.facts(&fam)).map_err(|e| LeafError::Broken(Failure::EnactedRulesDoNotCheck(e)))?;
        for id in fam {
            let violated = self.c.get(&id).is_some_and(|k| k.status == Some(Status::Violated));
            let mut findings = Findings::new();
            let mut hits: Vec<RFact> = Vec::new();
            if violated {
                for finding in &rules.findings {
                    for f in model.of(finding.pred()) {
                        if let [Const::Actor(who), Const::Commitment(c)] = f.args.as_slice() {
                            if *c == id {
                                let _ = findings.insert((*finding, who.clone()));
                                hits.push(f.clone());
                            }
                        }
                    }
                }
            }
            if findings == self.findings(&id) {
                continue;
            }
            let _ = self.verdicts.insert(id, findings.clone());
            let mut assumptions = vec![Assumption::Authentication, Assumption::RecordsNotKnowledge, Assumption::AssessmentIsRevisable];
            let absent: BTreeSet<RFact> = hits.iter().flat_map(|f| model.assumptions(f)).collect();
            assumptions.extend(absent.into_iter().map(Assumption::NoRecordOf));
            out.push(Event::Verdict(Verdict { commitment: id, findings, rules: version, proofs: hits.iter().map(|f| model.proof(f)).collect(), assumptions }));
        }
        Ok(())
    }

    pub(super) fn touch_prop(&mut self, p: &Prop) {
        let roots: Vec<CommitmentId> = self.by_prop.get(p).into_iter().flatten().map(|c| self.root_of(*c)).collect();
        self.dirty.extend(roots);
    }
}

//! What happens when the obligation unit hears something.

use super::*;

impl Leaf<Event> for Norms {
    fn name(&self) -> Name {
        NORMS
    }
    fn inputs(&self) -> BTreeSet<Kind> {
        [Kind::Member, Kind::Expelled, Kind::Tick, Kind::RulesChanged, Kind::Ok, Kind::Assessed, Kind::Verdict, Kind::Transferred].into_iter().collect()
    }
    fn outputs(&self) -> BTreeSet<Kind> {
        [Kind::NormCreated, Kind::NormStatus, Kind::NormRejected, Kind::NormAssigned, Kind::NormSkipped].into_iter().collect()
    }
    fn on(&mut self, env: &Envelope<'_, Event>) -> Result<Vec<Event>, LeafError> {
        let mut out = Vec::new();
        match env.event {
            Event::Member(m) => {
                // A member holds at most the role last given to them.
                self.roles.retain(|_, who| *who != m.id);
                if let Some(role) = m.role {
                    let _ = self.roles.insert(role, m.id.clone());
                }
            }
            Event::Expelled { member } => self.roles.retain(|_, who| who != member),
            Event::RulesChanged(rc) => {
                self.rules = rc.rules.norms.clone();
                self.version = rc.version;
            }
            Event::Tick(t) => {
                self.now = self.now.max(*t);
                let later = self.due.split_off(&self.now);
                let past = std::mem::replace(&mut self.due, later);
                self.refresh(&mut out, past.into_values().flatten())?;
            }
            Event::Ok { by, act: Act::Commit(o), .. } => {
                let terms = CommitmentTerms {
                    debtor: by.clone(),
                    creditor: o.creditor.clone(),
                    ant: o.ant.clone(),
                    cons: o.cons.clone(),
                    deadline: o.deadline,
                    tags: o.tags.clone(),
                    parent: o.parent,
                    terms: o.terms.clone(),
                    basis: None,
                };
                let _ = self.create(&mut out, terms, Source::Act, Some(by))?;
            }
            Event::Ok { by, act: Act::Request(r), .. } => {
                // Asking someone to take on part of one's own commitment is what lets them do so.
                if let Some(parent) = r.parent.filter(|p| self.c.get(p).is_some_and(|c| c.terms().debtor == *by)) {
                    let _ = self.asked.insert((parent, r.to.clone()));
                }
            }
            Event::Ok { by, act, .. } => self.operate(&mut out, by, act)?,
            Event::Assessed { prop, assessment, since, .. } => {
                // When this unit learned of the CURRENT assessment: unchanged if only its basis changed.
                let learned = match self.prop.get(prop) {
                    Some((was, _, at)) if was == assessment => *at,
                    Some(_) | None => self.now,
                };
                let _ = self.prop.insert(prop.clone(), (*assessment, *since, learned));
                self.refresh(&mut out, self.depending_on(prop))?;
            }
            Event::Verdict(v) => self.on_verdict(&mut out, v.commitment, &v.findings)?,
            Event::Transferred { transfer, .. } => self.on_transfer(&mut out, transfer)?,
            _ => {}
        }
        Ok(out)
    }
}

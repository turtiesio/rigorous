//! What happens when the judgment unit hears something.

use super::*;

impl Leaf<Event> for Adjudicator {
    fn name(&self) -> Name {
        ADJ
    }
    fn inputs(&self) -> BTreeSet<Kind> {
        [Kind::Member, Kind::RulesChanged, Kind::NormCreated, Kind::NormStatus, Kind::NormAssigned, Kind::Ok, Kind::Assessed, Kind::Signal, Kind::Settle]
            .into_iter()
            .collect()
    }
    fn outputs(&self) -> BTreeSet<Kind> {
        std::iter::once(Kind::Verdict).collect()
    }
    fn on(&mut self, env: &Envelope<'_, Event>) -> Result<Vec<Event>, LeafError> {
        let mut out = Vec::new();
        match env.event {
            Event::Member(m) => {
                let _ = self.members.insert(m.id.clone(), m.clone());
            }
            Event::RulesChanged(rc) => {
                self.rules = rc.rules.responsibility.clone();
                self.version = rc.version;
                let _ = self.versions.insert(rc.version, self.rules.clone());
                let roots: Vec<CommitmentId> = self.c.keys().map(|c| self.root_of(*c)).collect();
                self.dirty.extend(roots);
            }
            Event::NormCreated(c) => {
                let t = &c.terms;
                let prop = match &t.cons {
                    Condition::Holds(p) | Condition::Unrefuted(p) => Some(p.clone()),
                    Condition::Paid { .. } => None,
                };
                if let Some(p) = &prop {
                    let _ = self.by_prop.entry(p.clone()).or_default().insert(c.id);
                }
                if let Some(parent) = t.parent {
                    let _ = self.kids.entry(parent).or_default().insert(c.id);
                }
                let known = Known { debtor: t.debtor.clone(), creditor: t.creditor.clone(), parent: t.parent, prop, tags: t.tags.clone(), status: None };
                let _ = self.c.insert(c.id, known);
            }
            Event::NormStatus { id, status, .. } => {
                if let Some(k) = self.c.get_mut(id) {
                    k.status = Some(*status);
                }
                let root = self.root_of(*id);
                if *status == Status::Violated {
                    let _ = self.arose.entry(root).or_insert(self.version);
                }
                let _ = self.dirty.insert(root);
            }
            Event::NormAssigned { id, creditor, .. } => {
                if let Some(k) = self.c.get_mut(id) {
                    k.creditor = creditor.clone();
                }
                let root = self.root_of(*id);
                let _ = self.dirty.insert(root);
            }
            Event::Ok { by, act: Act::Claim { prop }, .. } => {
                if !self.claims.get(prop).is_some_and(|m| m.contains_key(by)) {
                    let n = self.claimed_so_far.entry(by.clone()).or_default();
                    *n += 1;
                    let ordinal = *n;
                    let _ = self.claims.entry(prop.clone()).or_default().insert(by.clone(), ordinal);
                }
                self.touch_prop(prop);
            }
            Event::Assessed { prop, assessment, .. } => {
                let _ = self.prop.insert(prop.clone(), *assessment);
                self.touch_prop(prop);
            }
            Event::Signal { to, prop, value } => {
                // The kernel says who sent it. Each sender's first telling is kept.
                if let Some(from) = env.who.local().map(|n| ActorId(n.clone())) {
                    let told = self.claimed_so_far.get(to).copied().unwrap_or(0);
                    let heard = self.notified.entry(prop.clone()).or_default().entry(to.clone()).or_default();
                    if heard.iter().all(|n| n.from != from) {
                        heard.push(Notice { from, value: *value, told });
                        self.touch_prop(prop);
                    }
                }
            }
            Event::Settle => {
                // Judge only quiescent states: never a half-propagated one.
                for root in std::mem::take(&mut self.dirty) {
                    self.adjudicate(root, &mut out)?;
                }
            }
            _ => {}
        }
        Ok(out)
    }
}

//! What happens when the authority unit hears something.

use super::*;

impl Leaf<Event> for Institution {
    fn name(&self) -> Name {
        INST
    }
    fn inputs(&self) -> BTreeSet<Kind> {
        [Kind::Genesis, Kind::Tick, Kind::Do, Kind::TransferFailed].into_iter().collect()
    }
    fn outputs(&self) -> BTreeSet<Kind> {
        [Kind::Ok, Kind::Delegated, Kind::Denied, Kind::Member, Kind::Expelled].into_iter().collect()
    }
    fn on(&mut self, env: &Envelope<'_, Event>) -> Result<Vec<Event>, LeafError> {
        let mut out = Vec::new();
        match env.event {
            Event::Genesis(c) if !self.founded => {
                self.founded = true;
                for m in &c.members {
                    self.enrol(m);
                    out.push(Event::Member(m.clone()));
                }
                for ch in &c.charters {
                    self.root(CapId::Charter(ch.name.clone()), ch.holder.clone(), ch.actions.clone(), Scope::default(), ch.delegable);
                }
            }
            Event::Tick(t) => self.now = self.now.max(*t),
            Event::TransferFailed { transfer, via, .. } => {
                let ids: Vec<CapId> = self.lineage(via).iter().map(|c| c.id.clone()).collect();
                for id in ids {
                    if let Some(s) = self.spent.get_mut(&(id, transfer.resource.clone())) {
                        s.0 = s.0.saturating_sub(transfer.amount.0);
                    }
                }
            }
            Event::Do { act, cap } => {
                let path: Path = match &env.who {
                    Who::Here(p) | Who::Outer(p) => p.clone(),
                };
                let verdict = match env.who.local() {
                    Some(n) => self.attempt(&ActorId(n.clone()), act, cap.as_ref())?,
                    None => Err(DenyReason::NotAMember),
                };
                match verdict {
                    Ok(ev) => {
                        out.push(ev);
                        // Admission is announced so that every unit learns of the new member.
                        match act {
                            Act::Admit(m) => out.push(Event::Member(m.clone())),
                            Act::Expel { member } => out.push(Event::Expelled { member: member.clone() }),
                            Act::SetRole { member, .. } => out.extend(self.members.get(member).cloned().map(Event::Member)),
                            _ => {}
                        }
                    }
                    Err(reason) => out.push(Event::Denied { by: path, act: act.clone(), via: cap.clone(), reason }),
                }
            }
            _ => {}
        }
        Ok(out)
    }
}

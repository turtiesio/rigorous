//! Turning happenings into acts: reviews as evidence for and against, withdrawn when superseded.

use crate::cast::*;
use crate::happening::*;
use rigor_institution::model::{
    About, Act, ActorId, Attestation, CapId, ChangeId, Condition, Content, Event, EvidenceId, Member, MemberKind, Method, Offer, Prop, Tag, Time,
};
use rigor_institution::world::{Error, World};
use std::collections::{BTreeMap, BTreeSet};

/// What ingestion met.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Stats {
    pub happenings: usize,
    pub changes_opened: usize,
    pub approvals: usize,
    pub change_requests: usize,
    pub dismissals: usize,
    /// Earlier reviews withdrawn because the same reviewer reviewed again, or was dismissed.
    pub reviews_withdrawn: usize,
    pub merges: usize,
    pub members_admitted: usize,
    /// Attempts the institution refused (there should be none).
    pub refused: usize,
}

/// Feeds happenings into a world.
#[derive(Debug, Default)]
pub struct Ingest {
    pub stats: Stats,
    now: Time,
    known: BTreeSet<ActorId>,
    opened: BTreeSet<ChangeId>,
    merged: BTreeSet<ChangeId>,
    /// Each reviewer's standing review of each change.
    standing: BTreeMap<(ChangeId, ActorId), EvidenceId>,
    /// How many acts have been made (keeps otherwise identical attestations distinct).
    serial: u64,
}

pub(crate) const WEEK: u64 = 7 * 86_400;

impl Ingest {
    fn note(&mut self, entries: &[rigor_waist::Entry<Event>]) {
        self.stats.refused +=
            entries.iter().filter(|e| matches!(e.event, Event::Denied { .. } | Event::NormRejected { .. } | Event::EvidenceRejected { .. })).count();
    }

    fn admit(&mut self, w: &mut World, who: &ActorId) -> Result<(), Error> {
        if self.known.insert(who.clone()) {
            let member = Member { id: who.clone(), kind: MemberKind::Human, principal: None, role: None };
            let entries = w.act_with(&REGISTRAR, &CapId::Charter(ADMISSIONS), Act::Admit(member))?;
            self.note(&entries);
            self.stats.members_admitted += 1;
        }
        Ok(())
    }

    fn attest(&mut self, w: &mut World, by: &ActorId, about: About, method: Method) -> Result<Option<EvidenceId>, Error> {
        self.serial += 1;
        let a = Attestation {
            about,
            method,
            derived_from: BTreeSet::new(),
            artifacts: BTreeMap::new(),
            content: Content::Note(format!("happening {}", self.serial)),
        };
        let entries = w.act(by, Act::Attest(a))?;
        self.note(&entries);
        Ok(entries.iter().find_map(|e| match &e.event {
            Event::EvidenceRecorded(ev) => Some(ev.id),
            _ => None,
        }))
    }

    /// Feed one happening.
    ///
    /// # Errors
    /// On a kernel fault.
    pub fn happen(&mut self, w: &mut World, h: &Happening) -> Result<(), Error> {
        self.stats.happenings += 1;
        if h.at > self.now {
            self.now = h.at;
            let _ = w.tick(h.at.0)?;
        }
        let sound = Prop::Sound(h.change.clone());
        match h.what {
            What::Opened => {
                if self.opened.insert(h.change.clone()) {
                    self.stats.changes_opened += 1;
                    self.admit(w, &h.who)?;
                    let offer = Offer {
                        creditor: PROJECT,
                        ant: None,
                        cons: Condition::Holds(sound.clone()),
                        deadline: self.now.plus(WEEK),
                        tags: std::iter::once(Tag::Change).collect(),
                        parent: None,
                        terms: None,
                    };
                    let made = w.act(&h.who, Act::Commit(offer))?;
                    self.note(&made);
                    let claimed = w.act(&h.who, Act::Claim { prop: sound })?;
                    self.note(&claimed);
                }
            }
            What::Reviewed(verdict) => {
                self.admit(w, &h.who)?;
                let key = (h.change.clone(), h.who.clone());
                // Whatever this reviewer said before no longer stands.
                if let Some(earlier) = self.standing.remove(&key) {
                    let entries = w.act(&h.who, Act::Retract { evidence: earlier })?;
                    self.note(&entries);
                    self.stats.reviews_withdrawn += 1;
                }
                let about = match verdict {
                    Verdict::Approved => {
                        self.stats.approvals += 1;
                        Some(About::Supports(sound))
                    }
                    Verdict::ChangesRequested => {
                        self.stats.change_requests += 1;
                        Some(About::Rebuts(sound))
                    }
                    Verdict::Dismissed => {
                        self.stats.dismissals += 1;
                        None
                    }
                };
                if let Some(about) = about {
                    if let Some(id) = self.attest(w, &h.who, about, Method::Ack)? {
                        let _ = self.standing.insert(key, id);
                    }
                }
            }
            What::Merged => {
                if self.merged.insert(h.change.clone()) {
                    self.stats.merges += 1;
                    let _ = self.attest(w, &FORGE, About::Supports(Prop::Applied(h.change.clone())), Method::Receipt)?;
                }
            }
        }
        Ok(())
    }

    /// The changes merged in the window.
    #[must_use]
    pub const fn merged(&self) -> &BTreeSet<ChangeId> {
        &self.merged
    }
}

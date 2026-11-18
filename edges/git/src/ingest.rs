//! Turning commits into acts: admissions, changes, acknowledgements, reverts as refutations.

use crate::cast::*;
use crate::commit::*;
use rigor_institution::model::{
    About, Act, ActionKind, ActorId, Attestation, CapId, ChangeId, Condition, Content, Event, EvidenceId, Member, MemberKind, Method, Offer, Prop, Tag, Time,
};
use rigor_institution::world::{Error, World};
use std::collections::{BTreeMap, BTreeSet};

/// What ingestion met in the data.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Stats {
    pub commits: usize,
    pub members_admitted: usize,
    /// ...of whom this many were admitted by a sponsor under delegated authority.
    pub admitted_by_sponsor: usize,
    pub sponsors_empowered: usize,
    pub automated_members: usize,
    pub independently_acknowledged: usize,
    /// Commits whose message records at least one reviewer other than the author and committer.
    pub reviewed: usize,
    pub reviews: usize,
    /// Standing objections recorded by a forge (a reviewer requested changes and never approved).
    pub objections: usize,
    /// Rule changes the reflector proposed, which the project then approved.
    pub reflector_proposals_approved: usize,
    pub reverts_declared: usize,
    /// Reverts naming a hash that matches no earlier commit, or more than one.
    pub reverts_unresolved: usize,
    pub reverts_of_reverts: usize,
    /// Commits whose committer clock was behind the latest time already seen.
    pub clock_regressions: usize,
    /// Commits dated so far back that their soak period would already be over on arrival;
    /// their period is counted from arrival instead.
    pub backdated: usize,
    /// Attempts the institution refused (there should be none: every act is meant to be authorised).
    pub denied: usize,
    pub self_reverts: usize,
}

/// Incremental ingestion of a history into a world. The world is passed in, so several
/// histories can be fed into differently scoped parts of one world.
#[derive(Debug)]
pub struct Ingest {
    pub stats: Stats,
    /// A fixed period, or `None` to use whatever the project's rules currently prescribe.
    fixed_period: Option<u64>,
    /// Whether the project approves what its reflector proposes (standing in for the human vote).
    pub approve_reflector: bool,
    pending: Vec<rigor_institution::model::ProposalId>,
    now: Time,
    known: BTreeSet<ActorId>,
    /// The admission capability each sponsor holds by delegation.
    sponsors: BTreeMap<ActorId, CapId>,
    seen: BTreeMap<ChangeId, ActorId>,
    /// Evidence each commit produced by reverting something (to be undercut if it is reverted).
    produced: BTreeMap<ChangeId, Vec<EvidenceId>>,
}

impl Ingest {
    /// Use a fixed maintenance period, ignoring the rules.
    #[must_use]
    pub fn new(soak: u64) -> Self {
        Self { fixed_period: Some(soak), ..Self::by_the_rules() }
    }

    /// Use the maintenance period the project's rules prescribe at the time of each commit.
    #[must_use]
    pub fn by_the_rules() -> Self {
        Self {
            stats: Stats::default(),
            fixed_period: None,
            approve_reflector: false,
            pending: Vec::new(),
            now: Time(0),
            known: BTreeSet::new(),
            sponsors: BTreeMap::new(),
            seen: BTreeMap::new(),
            produced: BTreeMap::new(),
        }
    }

    fn note(&mut self, entries: &[rigor_waist::Entry<Event>]) {
        self.stats.denied += entries.iter().filter(|e| matches!(peel(&e.event), Event::Denied { .. } | Event::NormRejected { .. })).count();
        self.pending.extend(entries.iter().filter_map(|e| match peel(&e.event) {
            Event::Proposed { id, by, .. } if *by == REFLECTOR => Some(*id),
            _ => None,
        }));
    }

    fn admit(&mut self, w: &mut World, who: &Identity, sponsor: Option<&ActorId>) -> Result<(), Error> {
        if !self.known.insert(who.id.clone()) {
            return Ok(());
        }
        let kind = if who.automated { MemberKind::Service } else { MemberKind::Human };
        let member = Member { id: who.id.clone(), kind, principal: None, role: None };
        let entries = match sponsor.and_then(|s| self.sponsors.get(s).map(|cap| (s.clone(), cap.clone()))) {
            Some((s, cap)) => {
                self.stats.admitted_by_sponsor += 1;
                w.act_with(&s, &cap, Act::Admit(member))?
            }
            None => w.act_with(&REGISTRAR, &CapId::Charter(ADMISSIONS), Act::Admit(member))?,
        };
        self.note(&entries);
        self.stats.members_admitted += 1;
        self.stats.automated_members += usize::from(who.automated);
        Ok(())
    }

    /// The registrar delegates (only) the power to admit to someone who commits others' work.
    fn empower(&mut self, w: &mut World, sponsor: &ActorId) -> Result<(), Error> {
        if self.sponsors.contains_key(sponsor) {
            return Ok(());
        }
        let grant = rigor_institution::model::Grant {
            to: sponsor.clone(),
            actions: std::iter::once(ActionKind::Admit).collect(),
            scope: rigor_institution::model::Scope::default(),
            limits: BTreeMap::new(),
            expires: None,
            delegable: false,
        };
        let entries = w.act_with(&REGISTRAR, &CapId::Charter(ADMISSIONS), Act::Delegate(grant))?;
        self.note(&entries);
        if let Some(cap) = entries.iter().find_map(|e| match peel(&e.event) {
            Event::Delegated(c) => Some(c.id.clone()),
            _ => None,
        }) {
            let _ = self.sponsors.insert(sponsor.clone(), cap);
            self.stats.sponsors_empowered += 1;
        }
        Ok(())
    }

    fn attest(&mut self, w: &mut World, by: &ActorId, about: About, method: Method, note: &ChangeId) -> Result<Option<EvidenceId>, Error> {
        let a = Attestation { about, method, derived_from: BTreeSet::new(), artifacts: BTreeMap::new(), content: Content::Note(note.to_string()) };
        let entries = w.act(by, Act::Attest(a))?;
        self.note(&entries);
        Ok(entries.iter().find_map(|e| match peel(&e.event) {
            Event::EvidenceRecorded(ev) => Some(ev.id),
            _ => None,
        }))
    }

    /// The unique earlier commit a written hash refers to.
    fn resolve(&self, written: &str) -> Option<ChangeId> {
        let mut hits = self.seen.keys().filter(|c| c.0.as_str().starts_with(written));
        match (hits.next(), hits.next()) {
            (Some(one), None) => Some(one.clone()),
            _ => None,
        }
    }

    /// Feed one commit.
    ///
    /// # Errors
    /// On a kernel fault.
    pub fn commit(&mut self, w: &mut World, c: &Commit) -> Result<(), Error> {
        self.stats.commits += 1;
        if c.committed > self.now {
            self.now = c.committed;
            let _ = w.tick(c.committed.0)?;
        } else if c.committed < self.now {
            self.stats.clock_regressions += 1;
        }
        let applied_by_other = c.committer.id != c.author.id;
        if applied_by_other {
            self.admit(w, &c.committer, None)?;
            self.empower(w, &c.committer.id)?;
        }
        self.admit(w, &c.author, applied_by_other.then_some(&c.committer.id))?;
        // Policy for backdated records: the period runs from arrival, never from a date
        // already in the past.
        let period = match self.fixed_period {
            Some(p) => p,
            None => w.gov()?.rules().map_or(1, |r| r.norms.maintenance_period),
        };
        self.stats.backdated += usize::from(c.committed.plus(period) < self.now);
        let sound = Prop::Sound(c.id.clone());
        let offer = Offer {
            creditor: PROJECT,
            ant: None,
            cons: Condition::Unrefuted(sound.clone()),
            deadline: self.now.max(c.committed).plus(period),
            tags: std::iter::once(Tag::Change).collect(),
            parent: None,
            terms: None,
        };
        let made = w.act(&c.author.id, Act::Commit(offer))?;
        self.note(&made);
        let claimed = w.act(&c.author.id, Act::Claim { prop: sound.clone() })?;
        self.note(&claimed);
        if applied_by_other {
            let _ = self.attest(w, &c.committer.id, About::Supports(sound), Method::Ack, &c.id)?;
            self.stats.independently_acknowledged += 1;
        }
        let others: Vec<&ActorId> = c.reviewers.iter().filter(|r| **r != c.committer.id).collect();
        self.stats.reviewed += usize::from(!others.is_empty());
        for reviewer in others {
            self.admit(w, &Identity { id: reviewer.clone(), automated: false }, None)?;
            let _ = self.attest(w, reviewer, About::Supports(Prop::Sound(c.id.clone())), Method::Ack, &c.id)?;
            self.stats.reviews += 1;
        }
        for objector in &c.objectors {
            self.admit(w, &Identity { id: objector.clone(), automated: false }, None)?;
            let _ = self.attest(w, objector, About::Rebuts(Prop::Sound(c.id.clone())), Method::Ack, &c.id)?;
            self.stats.objections += 1;
        }
        for written in &c.reverts {
            self.stats.reverts_declared += 1;
            let Some(target) = self.resolve(written) else {
                self.stats.reverts_unresolved += 1;
                continue;
            };
            self.stats.self_reverts += usize::from(self.seen.get(&target) == Some(&c.author.id));
            let mut mine = Vec::new();
            mine.extend(self.attest(w, &c.author.id, About::Rebuts(Prop::Sound(target.clone())), Method::Reversal, &c.id)?);
            // Reverting a commit also withdraws whatever that commit had itself reverted.
            let theirs = self.produced.get(&target).cloned().unwrap_or_default();
            self.stats.reverts_of_reverts += usize::from(!theirs.is_empty());
            for e in theirs {
                mine.extend(self.attest(w, &c.author.id, About::Undercuts(e), Method::Reversal, &c.id)?);
            }
            self.produced.entry(c.id.clone()).or_default().extend(mine);
        }
        let _ = self.seen.insert(c.id.clone(), c.author.id.clone());
        if self.approve_reflector {
            for id in std::mem::take(&mut self.pending) {
                // Two holders of the power to approve; the reflector's proposing counts for nothing.
                let _ = w.act_with(&REGISTRAR, &CapId::Charter(GOVERN_REGISTRAR), Act::Approve { id })?;
                let entries = w.act_with(&PROJECT, &CapId::Charter(GOVERN_PROJECT), Act::Approve { id })?;
                self.stats.reflector_proposals_approved += usize::from(entries.iter().any(|e| matches!(peel(&e.event), Event::RulesChanged(_))));
            }
        }
        Ok(())
    }

    /// Advance the clock past every soak period.
    ///
    /// # Errors
    /// On a kernel fault.
    pub fn close(&mut self, w: &mut World) -> Result<(), Error> {
        let longest = match self.fixed_period {
            Some(p) => p,
            None => w.gov()?.rules().map_or(1, |r| r.norms.maintenance_period),
        };
        let _ = w.tick(self.now.plus(longest).plus(1).0)?;
        Ok(())
    }
}

/// The event inside any number of boundary wrappers.
pub(crate) fn peel(e: &Event) -> &Event {
    let mut cur = e;
    while let Event::Scoped { inner, .. } = cur {
        cur = inner;
    }
    cur
}

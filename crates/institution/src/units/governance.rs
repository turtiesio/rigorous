//! Governance: the rules are data, changed only by the rules' own procedure.
//!
//! `Propose` then approvals from `quorum` distinct members yields `RulesChanged { version + 1 }`.
//! Every proposal is validated before it can be enacted ([`Rules::validate`]), and may not touch
//! an entrenched path. The quorum and the entrenchment list are themselves rules.

use crate::model::{Act, ActorId, Amendment, ContentDigest, Event, GovError, Kind, ProposalId, RulePath, Rules, RulesChanged, Time, Version};
use crate::model::{Failure, LeafError};
use rigor_kernel::{digest_of, Envelope, Leaf, Name};
use std::collections::{BTreeMap, BTreeSet};

pub const GOV: Name = Name::lit("gov");

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
enum Stage {
    Open,
    Enacted,
    Stale,
}

#[derive(Clone, Debug, Hash, serde::Serialize, serde::Deserialize)]
struct Proposal {
    amendments: Vec<Amendment>,
    base: Version,
    approvals: BTreeSet<ActorId>,
    stage: Stage,
}

/// One entry in the history of the rule set.
#[derive(Clone, Debug, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Enactment {
    pub version: Version,
    pub digest: ContentDigest,
    pub at: Time,
    pub by: BTreeSet<ActorId>,
}

#[derive(Clone, Debug, Hash, Default, serde::Serialize, serde::Deserialize)]
pub struct Governance {
    now: Time,
    version: Version,
    rules: Option<Rules>,
    history: Vec<Enactment>,
    proposals: BTreeMap<ProposalId, Proposal>,
}

impl Governance {
    #[must_use]
    pub const fn version(&self) -> Version {
        self.version
    }
    #[must_use]
    pub const fn rules(&self) -> Option<&Rules> {
        self.rules.as_ref()
    }
    #[must_use]
    pub fn history(&self) -> &[Enactment] {
        &self.history
    }

    fn enact(&mut self, new: Rules, changed: BTreeSet<RulePath>, by: BTreeSet<ActorId>) -> Result<Event, LeafError> {
        let prev = self.history.last().map(|e| e.digest);
        let digest = ContentDigest(digest_of(&new)?);
        self.version = Version(self.version.0.saturating_add(1));
        self.history.push(Enactment { version: self.version, digest, at: self.now, by: by.clone() });
        self.rules = Some(new.clone());
        Ok(Event::RulesChanged(Box::new(RulesChanged { version: self.version, rules: new, digest, prev, changed, by })))
    }

    fn propose(&mut self, by: &ActorId, amendments: &[Amendment], out: &mut Vec<Event>) -> Result<(), LeafError> {
        let Some(rules) = &self.rules else { return Ok(()) };
        let reject = |reason| Event::GovRejected { by: by.clone(), reason };
        if amendments.is_empty() {
            out.push(reject(GovError::EmptyProposal));
            return Ok(());
        }
        let paths: BTreeSet<RulePath> = amendments.iter().map(Amendment::path).collect();
        if let Some(p) = paths.iter().find(|p| rules.governance.entrenched.contains(p)) {
            out.push(reject(GovError::Entrenched(*p)));
            return Ok(());
        }
        if let Err(e) = rules.amended(amendments).validate() {
            out.push(reject(GovError::Invalid(e)));
            return Ok(());
        }
        let id = ProposalId(digest_of(&(self.version, amendments))?);
        if self.proposals.contains_key(&id) {
            out.push(reject(GovError::DuplicateProposal));
            return Ok(());
        }
        let p = Proposal { amendments: amendments.to_vec(), base: self.version, approvals: BTreeSet::new(), stage: Stage::Open };
        let _ = self.proposals.insert(id, p);
        out.push(Event::Proposed { id, by: by.clone(), base: self.version, paths });
        self.approve(by, id, out)
    }

    fn approve(&mut self, by: &ActorId, id: ProposalId, out: &mut Vec<Event>) -> Result<(), LeafError> {
        let reject = |reason| Event::GovRejected { by: by.clone(), reason };
        let version = self.version;
        let Some(rules) = self.rules.clone() else { return Ok(()) };
        let Some(p) = self.proposals.get_mut(&id).filter(|p| p.stage == Stage::Open) else {
            out.push(reject(GovError::NoSuchOpenProposal));
            return Ok(());
        };
        if p.base != version {
            p.stage = Stage::Stale;
            out.push(reject(GovError::Stale));
            return Ok(());
        }
        let _ = p.approvals.insert(by.clone());
        out.push(Event::Approved { id, by: by.clone(), approvals: p.approvals.clone() });
        let enough = u32::try_from(p.approvals.len()).unwrap_or(u32::MAX) >= rules.governance.quorum;
        if enough {
            p.stage = Stage::Enacted;
            let (amendments, approvals) = (p.amendments.clone(), p.approvals.clone());
            let changed = amendments.iter().map(Amendment::path).collect();
            out.push(self.enact(rules.amended(&amendments), changed, approvals)?);
        }
        Ok(())
    }
}

impl Leaf<Event> for Governance {
    fn name(&self) -> Name {
        GOV
    }
    fn inputs(&self) -> BTreeSet<Kind> {
        [Kind::Genesis, Kind::Tick, Kind::Ok].into_iter().collect()
    }
    fn outputs(&self) -> BTreeSet<Kind> {
        [Kind::RulesChanged, Kind::Proposed, Kind::Approved, Kind::GovRejected].into_iter().collect()
    }
    fn on(&mut self, env: &Envelope<'_, Event>) -> Result<Vec<Event>, LeafError> {
        let mut out = Vec::new();
        match env.event {
            Event::Tick(t) => self.now = self.now.max(*t),
            Event::Genesis(c) if self.rules.is_none() => {
                // An invalid constitution is not a constitution.
                c.rules.validate().map_err(|e| LeafError::Broken(Failure::InvalidConstitution(e)))?;
                let all = [
                    RulePath::Ranks,
                    RulePath::Trust,
                    RulePath::SelfSupport,
                    RulePath::Templates,
                    RulePath::Period,
                    RulePath::Responsibility,
                    RulePath::Retroactive,
                    RulePath::Quorum,
                    RulePath::Entrenched,
                ];
                out.push(self.enact(c.rules.clone(), all.into_iter().collect(), BTreeSet::new())?);
            }
            Event::Ok { by, act: Act::Propose { amendments }, .. } => self.propose(by, amendments, &mut out)?,
            Event::Ok { by, act: Act::Approve { id }, .. } => self.approve(by, *id, &mut out)?,
            _ => {}
        }
        Ok(out)
    }
}

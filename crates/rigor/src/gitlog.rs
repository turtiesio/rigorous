//! Adapter: a real git history as a stream of typed institutional events.
//!
//! This is where untyped real-world text is turned into the closed vocabulary, once, at the
//! boundary. What is read from each commit:
//!
//! * who authored it and who committed it (pseudonymised: an actor id is a digest of the
//!   e-mail address, so no personal data enters the log);
//! * when it was committed (clocks in real histories are not monotone);
//! * whether it declares "This reverts commit <hash>".
//!
//! The mapping into the model:
//!
//! * a newcomer is admitted by whoever first committed their work (their sponsor), acting
//!   under admission authority DELEGATED by the registrar; someone who commits their own first
//!   change is admitted by the registrar itself;
//! * the author undertakes a MAINTENANCE commitment to the project: the change stays
//!   unrefuted for a soak period, counted from when the record arrives (a backdated record
//!   does not get a period that is already over);
//! * a committer other than the author is an independent acknowledgement (supporting evidence);
//! * a revert is rebutting evidence against the reverted change, and undercuts every piece of
//!   evidence the reverted commit had itself produced (so a revert of a revert reinstates).
//!
//! Nothing here judges people. The findings the rules derive are about pseudonymous ids and are
//! only ever reported in aggregate.

use crate::kernel::{digest_bytes, Name, NameError};
use crate::model::{
    About, Act, ActionKind, ActorId, Assessment, Attestation, CapId, ChangeId, Charter, CharterName, Condition, Constitution, Content, Event, EvidenceId,
    Member, MemberKind, Method, Offer, Prop, Rank, Rules, Status, Tag, Time, Trust,
};
use crate::org::{org, standard_rules, Trusted};
use crate::world::{Error, World, CONSTITUTION};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::process::Command;

pub const PROJECT: ActorId = ActorId::lit("project");
pub const REGISTRAR: ActorId = ActorId::lit("registrar");
pub const ADMISSIONS: CharterName = CharterName::lit("admissions");
pub const GOVERN_PROJECT: CharterName = CharterName::lit("govern-project");
pub const GOVERN_REGISTRAR: CharterName = CharterName::lit("govern-registrar");
pub const REFLECTOR: ActorId = ActorId::lit("reflector");
pub const REFLECT: CharterName = CharterName::lit("reflect");

/// A project's organisation: the institutional units plus a reflector that watches how each
/// method of evidence fares and may propose (never enact) a change of rank. It judges a method
/// after 200 acceptances and proposes when 5% or more were overturned; it judges the maintenance
/// period after 50 observed refutations.
///
/// # Errors
/// If the units cannot be composed.
pub fn project_org() -> Result<crate::kernel::Par<Event>, crate::model::Fault> {
    let reflector = crate::units::reflector::Reflector::new(REFLECTOR, CapId::Charter(REFLECT), 200, 5, 50);
    org(vec![Box::new(crate::kernel::Cell(reflector))])
}

/// Why a history could not be read or ingested.
#[derive(Debug)]
pub enum GitError {
    Io(std::io::Error),
    /// `git log` exited unsuccessfully.
    Git(i32),
    /// A record did not have the expected fields.
    Malformed {
        record: usize,
    },
    Name(NameError),
    World(Error),
    Adapter(crate::adapters::AdapterError),
}

impl From<Error> for GitError {
    fn from(e: Error) -> Self {
        Self::World(e)
    }
}
impl From<crate::adapters::AdapterError> for GitError {
    fn from(e: crate::adapters::AdapterError) -> Self {
        Self::Adapter(e)
    }
}
impl From<NameError> for GitError {
    fn from(e: NameError) -> Self {
        Self::Name(e)
    }
}

/// Someone who appears in a history.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Identity {
    pub id: ActorId,
    /// Declared to be an automated account by its own name or address.
    pub automated: bool,
}

/// One commit, reduced to what the model uses.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Commit {
    pub id: ChangeId,
    pub author: Identity,
    pub committer: Identity,
    pub committed: Time,
    /// Hash prefixes this commit says it reverts (as written; possibly abbreviated or unknown).
    pub reverts: Vec<String>,
    /// People the commit message records as having reviewed or acknowledged the change
    /// (`Reviewed-by:` / `Acked-by:` trailers), other than its author.
    pub reviewers: Vec<ActorId>,
}

fn identity(email: &str, name: &str) -> Result<Identity, NameError> {
    let key = email.trim().to_lowercase();
    let id = ActorId(Name::new(&format!("u-{}", digest_bytes(key.as_bytes())))?);
    let automated = [email, name].iter().any(|s| s.contains("[bot]") || s.to_lowercase().contains("noreply@github"));
    Ok(Identity { id, automated })
}

/// The hash prefixes following each "This reverts commit " in a message.
fn reverts_in(message: &str) -> Vec<String> {
    const MARK: &str = "This reverts commit ";
    message
        .match_indices(MARK)
        .filter_map(|(i, _)| message.get(i + MARK.len()..))
        .map(|rest| rest.chars().take_while(char::is_ascii_hexdigit).collect::<String>().to_lowercase())
        .filter(|h| h.len() >= 7)
        .collect()
}

/// The people named in `Reviewed-by:` and `Acked-by:` trailers, by address.
fn reviewers_in(message: &str) -> Vec<ActorId> {
    let mut out = Vec::new();
    for line in message.lines() {
        let lower = line.trim_start().to_lowercase();
        if !(lower.starts_with("reviewed-by:") || lower.starts_with("acked-by:")) {
            continue;
        }
        let address = line.split_once('<').and_then(|(_, rest)| rest.split_once('>')).map(|(a, _)| a);
        if let Some(id) = address.and_then(|a| identity(a, "").ok()) {
            if !out.contains(&id.id) {
                out.push(id.id);
            }
        }
    }
    out
}

/// Parse the output of [`LOG_FORMAT`].
///
/// # Errors
/// If a record is malformed.
pub fn parse(output: &str) -> Result<Vec<Commit>, GitError> {
    let mut commits = Vec::new();
    for (i, record) in output.split('\u{1e}').map(str::trim_start).filter(|r| !r.is_empty()).enumerate() {
        let fields: Vec<&str> = record.splitn(7, '\u{1f}').collect();
        let [hash, author_email, author_name, committer_email, committer_name, committed, message] = fields.as_slice() else {
            return Err(GitError::Malformed { record: i });
        };
        let author = identity(author_email, author_name)?;
        let reviewers = reviewers_in(message).into_iter().filter(|r| *r != author.id).collect();
        commits.push(Commit {
            id: ChangeId(Name::new(hash)?),
            author,
            committer: identity(committer_email, committer_name)?,
            committed: Time(committed.trim().parse::<u64>().map_err(|_| GitError::Malformed { record: i })?),
            reverts: reverts_in(message),
            reviewers,
        });
    }
    Ok(commits)
}

/// Fields separated by U+001F, records by U+001E. `%aE`/`%cE` apply the repository's mailmap,
/// so one person with several addresses is one identity wherever the project has said so.
pub const LOG_FORMAT: &str = "--format=%H%x1f%aE%x1f%aN%x1f%cE%x1f%cN%x1f%ct%x1f%B%x1e";

/// A history in reduced, pseudonymous form: one line per commit, fields separated by spaces:
/// hash, author id, author automated (0/1), committer id, committer automated, time, reverts
/// (comma-separated, or `-`), reviewers (comma-separated ids, or `-`). This is what the test fixture holds: real structure, no personal data.
#[must_use]
pub fn export(commits: &[Commit]) -> String {
    let mut out = String::new();
    for c in commits {
        let reverts = if c.reverts.is_empty() { "-".to_owned() } else { c.reverts.join(",") };
        let reviewers = if c.reviewers.is_empty() { "-".to_owned() } else { c.reviewers.iter().map(ToString::to_string).collect::<Vec<_>>().join(",") };
        out.push_str(&format!(
            "{} {} {} {} {} {} {} {}\n",
            c.id,
            c.author.id,
            u8::from(c.author.automated),
            c.committer.id,
            u8::from(c.committer.automated),
            c.committed.0,
            reverts,
            reviewers
        ));
    }
    out
}

/// Read back what [`export`] wrote.
///
/// # Errors
/// If a line is malformed.
pub fn import(text: &str) -> Result<Vec<Commit>, GitError> {
    let mut commits = Vec::new();
    for (record, line) in text.lines().enumerate() {
        let fields: Vec<&str> = line.split(' ').collect();
        let [hash, author, a_auto, committer, c_auto, time, reverts, reviewers] = fields.as_slice() else {
            return Err(GitError::Malformed { record });
        };
        let who = |id: &str, auto: &str| -> Result<Identity, GitError> { Ok(Identity { id: ActorId(Name::new(id)?), automated: auto == "1" }) };
        commits.push(Commit {
            id: ChangeId(Name::new(hash)?),
            author: who(author, a_auto)?,
            committer: who(committer, c_auto)?,
            committed: Time(time.parse::<u64>().map_err(|_| GitError::Malformed { record })?),
            reverts: if *reverts == "-" { Vec::new() } else { reverts.split(',').map(str::to_owned).collect() },
            reviewers: if *reviewers == "-" { Vec::new() } else { reviewers.split(',').map(|r| Name::new(r).map(ActorId)).collect::<Result<_, _>>()? },
        });
    }
    Ok(commits)
}

/// Read a repository's history, oldest first.
///
/// # Errors
/// If git cannot be run or its output cannot be parsed.
pub fn read(repo: &Path, limit: Option<usize>) -> Result<Vec<Commit>, GitError> {
    let mut cmd = Command::new("git");
    let _ = cmd.arg("--git-dir").arg(repo).arg("log").arg("--reverse").arg(LOG_FORMAT);
    let out = cmd.output().map_err(GitError::Io)?;
    if !out.status.success() {
        return Err(GitError::Git(out.status.code().unwrap_or(-1)));
    }
    let mut commits = parse(&String::from_utf8_lossy(&out.stdout))?;
    if let Some(n) = limit {
        commits.truncate(n);
    }
    Ok(commits)
}

/// Rules for a body of work: acknowledgements and reversals from anyone, a reversal outranking
/// an acknowledgement.
#[must_use]
pub fn rules() -> Rules {
    let mut r = standard_rules(&Trusted { receipts: PROJECT, probes: PROJECT, tests: PROJECT, audits: PROJECT, acks: PROJECT });
    r.evidence.ranks = [(Method::Ack, Rank(2)), (Method::Reversal, Rank(3))].into_iter().collect();
    r.evidence.trusted = [(Method::Ack, Trust::Anyone), (Method::Reversal, Trust::Anyone)].into_iter().collect();
    r
}

/// Found the project's institution in whatever scope the world currently addresses.
///
/// # Errors
/// On a kernel fault.
pub fn found_in(w: &mut World) -> Result<(), Error> {
    let _ = w.inject(&CONSTITUTION, Event::Genesis(Box::new(constitution())))?;
    Ok(())
}

/// A founded world with a project and a registrar who may admit members.
///
/// # Errors
/// On a kernel fault.
pub fn found() -> Result<World, Error> {
    let mut w = World::new(Box::new(project_org()?));
    found_in(&mut w)?;
    Ok(w)
}

/// Several projects as separately bounded organisations inside one world: each has its own
/// institution, members, rules and clock; nothing crosses a boundary.
///
/// # Errors
/// If the units cannot be composed.
pub fn federation(names: &[Name]) -> Result<World, Error> {
    let mut parts: Vec<Box<dyn crate::kernel::Unit<Event>>> = Vec::new();
    for n in names {
        parts.push(Box::new(crate::kernel::Entity::new(n.clone(), Box::new(project_org()?), BTreeSet::new(), BTreeSet::new())));
    }
    Ok(World::new(Box::new(crate::kernel::Par::new(parts, &crate::org::open_kinds())?)))
}

fn constitution() -> Constitution {
    let m = |id: ActorId, kind| Member { id, kind, principal: None, role: None };
    let govern: BTreeSet<ActionKind> = [ActionKind::Propose, ActionKind::Approve].into_iter().collect();
    Constitution {
        members: vec![m(PROJECT, MemberKind::Org), m(REGISTRAR, MemberKind::Service), m(REFLECTOR, MemberKind::Service)],
        charters: vec![
            Charter { name: ADMISSIONS, holder: REGISTRAR, actions: std::iter::once(ActionKind::Admit).collect(), delegable: true },
            Charter { name: GOVERN_PROJECT, holder: PROJECT, actions: govern.clone(), delegable: false },
            Charter { name: GOVERN_REGISTRAR, holder: REGISTRAR, actions: govern, delegable: false },
            Charter { name: REFLECT, holder: REFLECTOR, actions: std::iter::once(ActionKind::Propose).collect(), delegable: false },
        ],
        balances: Vec::new(),
        rules: rules(),
    }
}

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
    pending: Vec<crate::model::ProposalId>,
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

    fn note(&mut self, entries: &[crate::kernel::Entry<Event>]) {
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
        let grant = crate::model::Grant {
            to: sponsor.clone(),
            actions: std::iter::once(ActionKind::Admit).collect(),
            scope: crate::model::Scope::default(),
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
fn peel(e: &Event) -> &Event {
    let mut cur = e;
    while let Event::Scoped { inner, .. } = cur {
        cur = inner;
    }
    cur
}

/// An independent computation of which changes end up refuted, by a different algorithm from
/// the system's (plain recursion on "is this commit still in effect"), for cross-checking.
///
/// A commit is in effect unless some commit that reverts it is itself in effect. A change is
/// refuted exactly when it is not in effect.
#[must_use]
pub fn oracle(commits: &[Commit]) -> BTreeSet<ChangeId> {
    let mut seen: BTreeSet<&ChangeId> = BTreeSet::new();
    let mut reverted_by: BTreeMap<ChangeId, Vec<&ChangeId>> = BTreeMap::new();
    for c in commits {
        for written in &c.reverts {
            let mut hits = seen.iter().filter(|k| k.0.as_str().starts_with(written.as_str()));
            if let (Some(one), None) = (hits.next(), hits.next()) {
                reverted_by.entry((*one).clone()).or_default().push(&c.id);
            }
        }
        let _ = seen.insert(&c.id);
    }
    // Reverts always come later, so one pass from newest to oldest settles everything.
    let mut in_effect: BTreeMap<&ChangeId, bool> = BTreeMap::new();
    for c in commits.iter().rev() {
        let undone = reverted_by.get(&c.id).is_some_and(|rs| rs.iter().any(|r| in_effect.get(r).copied().unwrap_or(true)));
        let _ = in_effect.insert(&c.id, !undone);
    }
    in_effect.into_iter().filter(|(_, ok)| !ok).map(|(c, _)| c.clone()).collect()
}

/// What the system concluded, in aggregate.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Conclusions {
    pub refuted: BTreeSet<ChangeId>,
    pub by_status: BTreeMap<Status, usize>,
    pub commitments_with_findings: usize,
}

/// Read the system's conclusions out of a world.
///
/// # Errors
/// If the world lacks an institutional unit.
pub fn conclusions(w: &World) -> Result<Conclusions, Error> {
    let mut c = Conclusions::default();
    for (p, s) in w.epi()?.standings() {
        if let (Prop::Sound(id), Assessment::Refuted) = (p, s.assessment) {
            let _ = c.refuted.insert(id.clone());
        }
    }
    for r in w.norms()?.records() {
        if let Some(s) = r.status {
            *c.by_status.entry(s).or_default() += 1;
        }
    }
    c.commitments_with_findings = w.adj()?.verdicts().count();
    Ok(c)
}

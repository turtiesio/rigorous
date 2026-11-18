//! Adapter: pull requests and their reviews, from a forge's public event archive.
//!
//! A fourth kind of real record, and the first with real RETRACTION AT SCALE: a reviewer's new
//! review supersedes their earlier one, and a review can be dismissed.
//!
//! The mapping:
//!
//! * opening a pull request: its author undertakes that the change will be found sound, and
//!   claims that it is;
//! * an approving review supports that; a review requesting changes rebuts it;
//! * a reviewer's later review withdraws their earlier one; a dismissal withdraws it too;
//! * a merge is the forge's own record that the change was applied.
//!
//! People and repositories are pseudonymised at the boundary. Only a window of the archive is
//! read, so a review given before the window is invisible: "merged with no approval on record"
//! means on record in the window, nothing more.

use flate2::read::GzDecoder;
use rigor_institution::model::{
    About, Act, ActionKind, ActorId, Assessment, Attestation, CapId, ChangeId, Charter, CharterName, Condition, Constitution, Content, Event, EvidenceId,
    Member, MemberKind, Method, Offer, Prop, Rank, Tag, Time, Trust,
};
use rigor_institution::org::{org, standard_rules, Trusted};
use rigor_institution::world::{Error, World, CONSTITUTION};
use rigor_kernel::{digest_bytes, Name, NameError};
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
use std::io::{BufRead, BufReader};
use std::path::Path;

pub const PROJECT: ActorId = ActorId::lit("the-forge-users");
pub const REGISTRAR: ActorId = ActorId::lit("the-registrar");
pub const FORGE: ActorId = ActorId::lit("the-forge");
pub const ADMISSIONS: CharterName = CharterName::lit("admissions");

/// What a review said.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    Approved,
    ChangesRequested,
    /// The reviewer's earlier review was dismissed.
    Dismissed,
}

/// One thing that happened to a pull request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum What {
    Opened,
    Reviewed(Verdict),
    Merged,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Happening {
    pub change: ChangeId,
    /// The author, the reviewer whose review this is, or whoever merged.
    pub who: ActorId,
    pub what: What,
    pub at: Time,
}

/// Why archive data could not be read.
#[derive(Debug)]
pub enum ForgeError {
    Io(std::io::Error),
    Malformed { line: usize },
    Name(NameError),
    World(Error),
}
impl From<Error> for ForgeError {
    fn from(e: Error) -> Self {
        Self::World(e)
    }
}
impl From<NameError> for ForgeError {
    fn from(e: NameError) -> Self {
        Self::Name(e)
    }
}

#[derive(Deserialize)]
struct Login {
    login: String,
}
#[derive(Deserialize)]
struct Repo {
    name: String,
}
#[derive(Deserialize)]
struct Numbered {
    number: u64,
}
#[derive(Deserialize)]
struct Review {
    state: String,
    user: Option<Login>,
}
#[derive(Deserialize)]
struct Payload {
    action: Option<String>,
    number: Option<u64>,
    review: Option<Review>,
    pull_request: Option<Numbered>,
}
#[derive(Deserialize)]
struct Raw {
    #[serde(rename = "type")]
    kind: String,
    actor: Login,
    repo: Repo,
    created_at: String,
    payload: Payload,
}

fn person(login: &str) -> Result<ActorId, NameError> {
    Ok(ActorId(Name::new(&format!("u-{}", digest_bytes(login.to_lowercase().as_bytes())))?))
}

fn change(repo: &str, number: u64) -> Result<ChangeId, NameError> {
    Ok(ChangeId(Name::new(&format!("pr-{:016x}", digest_bytes(format!("{repo}#{number}").as_bytes()).0))?))
}

/// Seconds since the epoch for a UTC timestamp `YYYY-MM-DDTHH:MM:SSZ`.
fn seconds(stamp: &str) -> Option<u64> {
    let n = |range: std::ops::Range<usize>| stamp.get(range).and_then(|s| s.parse::<u64>().ok());
    let (y, m, d, h, mi, s) = (n(0..4)?, n(5..7)?, n(8..10)?, n(11..13)?, n(14..16)?, n(17..19)?);
    // Days from civil date (proleptic Gregorian), for years after 1970.
    let (y2, m2) = if m <= 2 { (y - 1, m + 9) } else { (y, m - 3) };
    let days = 365 * y2 + y2 / 4 - y2 / 100 + y2 / 400 + (153 * m2 + 2) / 5 + d - 1;
    let epoch = 365 * 1969 + 1969 / 4 - 1969 / 100 + 1969 / 400 + (153 * 10 + 2) / 5; // 1970-01-01 in the same count
    Some((days.checked_sub(epoch)?) * 86_400 + h * 3600 + mi * 60 + s)
}

/// Read one hourly archive file (gzip, one JSON event per line), keeping only what the model uses.
///
/// # Errors
/// If the file cannot be read. Lines that are not recognised events are skipped.
pub fn read_archive(path: &Path, into: &mut Vec<Happening>) -> Result<(), ForgeError> {
    let file = std::fs::File::open(path).map_err(ForgeError::Io)?;
    for line in BufReader::new(GzDecoder::new(file)).lines() {
        let Ok(line) = line else { break };
        let Ok(raw) = serde_json::from_str::<Raw>(&line) else { continue };
        let Some(at) = seconds(&raw.created_at).map(Time) else { continue };
        let found = match raw.kind.as_str() {
            "PullRequestEvent" => raw.payload.number.and_then(|n| match raw.payload.action.as_deref() {
                Some("opened") => Some((n, raw.actor.login.clone(), What::Opened)),
                Some("merged") => Some((n, raw.actor.login.clone(), What::Merged)),
                _ => None,
            }),
            "PullRequestReviewEvent" => raw.payload.pull_request.as_ref().zip(raw.payload.review.as_ref()).and_then(|(pr, review)| {
                let reviewer = review.user.as_ref().map_or_else(|| raw.actor.login.clone(), |u| u.login.clone());
                let verdict = match review.state.as_str() {
                    "approved" => Some(Verdict::Approved),
                    "changes_requested" => Some(Verdict::ChangesRequested),
                    "dismissed" => Some(Verdict::Dismissed),
                    _ => None,
                };
                verdict.map(|v| (pr.number, reviewer, What::Reviewed(v)))
            }),
            _ => None,
        };
        if let Some((number, login, what)) = found {
            if let (Ok(change), Ok(who)) = (change(&raw.repo.name, number), person(&login)) {
                into.push(Happening { change, who, what, at });
            }
        }
    }
    Ok(())
}

/// The reduced, pseudonymous form: one happening per line, `<O|A|C|D|M> <change> <who> <seconds>`.
#[must_use]
pub fn export(all: &[Happening]) -> String {
    let mut out = String::new();
    for h in all {
        let letter = match h.what {
            What::Opened => 'O',
            What::Reviewed(Verdict::Approved) => 'A',
            What::Reviewed(Verdict::ChangesRequested) => 'C',
            What::Reviewed(Verdict::Dismissed) => 'D',
            What::Merged => 'M',
        };
        out.push_str(&format!("{letter} {} {} {}\n", h.change, h.who, h.at.0));
    }
    out
}

/// Read back what [`export`] wrote.
///
/// # Errors
/// If a line is malformed.
pub fn import(text: &str) -> Result<Vec<Happening>, ForgeError> {
    let mut out = Vec::new();
    for (i, line) in text.lines().enumerate() {
        let bad = || ForgeError::Malformed { line: i + 1 };
        let fields: Vec<&str> = line.split(' ').collect();
        let [letter, change, who, at] = fields.as_slice() else { return Err(bad()) };
        let what = match *letter {
            "O" => What::Opened,
            "A" => What::Reviewed(Verdict::Approved),
            "C" => What::Reviewed(Verdict::ChangesRequested),
            "D" => What::Reviewed(Verdict::Dismissed),
            "M" => What::Merged,
            _ => return Err(bad()),
        };
        out.push(Happening { change: ChangeId(Name::new(change)?), who: ActorId(Name::new(who)?), what, at: Time(at.parse().map_err(|_| bad())?) });
    }
    Ok(out)
}

/// Independent computation of how each change stands: each reviewer's LATEST review is what
/// counts; a dismissal removes it; the author's own approval counts for nothing.
#[must_use]
pub fn oracle(all: &[Happening]) -> BTreeMap<ChangeId, Assessment> {
    let mut author: BTreeMap<&ChangeId, &ActorId> = BTreeMap::new();
    let mut latest: BTreeMap<&ChangeId, BTreeMap<&ActorId, Verdict>> = BTreeMap::new();
    for h in all {
        match h.what {
            What::Opened => {
                let _ = author.entry(&h.change).or_insert(&h.who);
            }
            What::Reviewed(Verdict::Dismissed) => {
                let _ = latest.entry(&h.change).or_default().remove(&h.who);
            }
            What::Reviewed(v) => {
                let _ = latest.entry(&h.change).or_default().insert(&h.who, v);
            }
            What::Merged => {}
        }
    }
    let mut out = BTreeMap::new();
    for (change, reviews) in latest {
        let for_it = reviews.iter().any(|(who, v)| *v == Verdict::Approved && author.get(change) != Some(who));
        let against = reviews.values().any(|v| *v == Verdict::ChangesRequested);
        let standing = match (for_it, against) {
            (true, false) => Assessment::Accepted,
            (false, true) => Assessment::Refuted,
            (true, true) => Assessment::Contested,
            (false, false) => Assessment::Unsupported,
        };
        let _ = out.insert(change.clone(), standing);
    }
    out
}

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

/// A founded world for a forge.
///
/// # Errors
/// On a kernel fault.
pub fn found() -> Result<World, Error> {
    let m = |id: ActorId, kind| Member { id, kind, principal: None, role: None };
    let mut rules = standard_rules(&Trusted { receipts: FORGE, probes: FORGE, tests: FORGE, audits: FORGE, acks: FORGE });
    rules.evidence.ranks = [(Method::Ack, Rank(2)), (Method::Receipt, Rank(1))].into_iter().collect();
    rules.evidence.trusted = [(Method::Ack, Trust::Anyone), (Method::Receipt, Trust::Only(std::iter::once(FORGE).collect()))].into_iter().collect();
    let constitution = Constitution {
        members: vec![m(PROJECT, MemberKind::Org), m(REGISTRAR, MemberKind::Service), m(FORGE, MemberKind::Service)],
        charters: vec![Charter { name: ADMISSIONS, holder: REGISTRAR, actions: std::iter::once(ActionKind::Admit).collect(), delegable: false }],
        balances: Vec::new(),
        rules,
    };
    let mut w = World::new(Box::new(org(Vec::new())?));
    let _ = w.inject(&CONSTITUTION, Event::Genesis(Box::new(constitution)))?;
    Ok(w)
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

const WEEK: u64 = 7 * 86_400;

impl Ingest {
    fn note(&mut self, entries: &[rigor_kernel::Entry<Event>]) {
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

/// How the system currently assesses every change that was reviewed.
///
/// # Errors
/// If the world lacks an institutional unit.
pub fn conclusions(w: &World, reviewed: &BTreeMap<ChangeId, Assessment>) -> Result<BTreeMap<ChangeId, Assessment>, Error> {
    let epi = w.epi()?;
    Ok(reviewed.keys().map(|c| (c.clone(), epi.assessment(&Prop::Sound(c.clone())))).collect())
}

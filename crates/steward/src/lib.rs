//! Self-stewardship: the system governing the repository it lives in.
//!
//! The project's standing claims about itself ("the tests pass", "the lints are clean", "the
//! formatting is canonical", "the proofs check") are propositions. Each day it is run, the
//! steward has the maintainer undertake that they hold that day; evidence comes only from
//! really running the checks, bound to digests of the real files. So:
//!
//! * evidence from an earlier run still counts while the files it was about are unchanged
//!   (nothing is re-run needlessly);
//! * the moment a file changes, the evidence about it is stale and the claim is unsupported
//!   until the check is run again;
//! * a day on which a claim did not hold stays in the log as a violated commitment, with a
//!   verdict: the project's record of itself cannot be quietly rewritten.
//!
//! The roadmap is in the same world: each open item is a commitment with a review-by day.
//!
//! What this is not: the steward does not write code. It notices, records, and holds to account.

pub mod change;
pub mod dashboard;
pub mod research;
pub mod selfmodel;

use rigor_adapter_host::{files_under, migrate, refound, restore, run_check, save, AdapterError, ArtifactWatcher};
use rigor_institution::model::{
    Act, ActorId, ArtifactId, Assessment, Commitment, Condition, Constitution, Event, Member, MemberKind, Method, Offer, Prop, Requirement, Status, Tag, Time,
};
use rigor_institution::org::{org, standard_rules, Trusted};
use rigor_institution::world::attest;
use rigor_institution::world::{Error, World, CONSTITUTION};
use rigor_kernel::LogError;
use rigor_kernel::Unit;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

pub const PROJECT: ActorId = ActorId::lit("project");
pub const MAINTAINER: ActorId = ActorId::lit("maintainer");
pub const CI: ActorId = ActorId::lit("ci");
/// The person the project answers to. Decisions that are the owner's are never made by the system.
pub const OWNER: ActorId = ActorId::lit("owner");
/// A language model that drafts (answers, proposals). What it says is a claim, never evidence.
pub const RESEARCHER: ActorId = ActorId::lit("researcher");
/// A second, different language model that reviews what the first one drafted.
pub const REVIEWER: ActorId = ActorId::lit("reviewer");
/// Where money spent on consulting models goes.
pub const GATEWAY: ActorId = ActorId::lit("gateway");
/// Millionths of a US dollar.
pub const MICRO_USD: rigor_institution::model::Resource = rigor_institution::model::Resource::lit("micro-usd");
pub const OWNERSHIP: rigor_institution::model::CharterName = rigor_institution::model::CharterName::lit("ownership");

pub const SOURCES: ArtifactId = ArtifactId::lit("rust-sources");
pub const PROOFS: ArtifactId = ArtifactId::lit("lean-proofs");

/// A check the project runs against itself.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Check {
    Tests,
    Lints,
    Format,
    Proofs,
}

impl Check {
    pub const ALL: [Self; 4] = [Self::Tests, Self::Lints, Self::Format, Self::Proofs];

    #[must_use]
    pub const fn requirement(self) -> Requirement {
        match self {
            Self::Tests => Requirement::lit("tests-pass"),
            Self::Lints => Requirement::lit("lints-clean"),
            Self::Format => Requirement::lit("format-canonical"),
            Self::Proofs => Requirement::lit("proofs-check"),
        }
    }

    /// What the check is about: its evidence goes stale when this changes.
    #[must_use]
    pub const fn artifact(self) -> ArtifactId {
        match self {
            Self::Tests | Self::Lints | Self::Format => SOURCES,
            Self::Proofs => PROOFS,
        }
    }

    /// The real command.
    #[must_use]
    pub fn command(self, root: &Path) -> Command {
        let mut c = match self {
            Self::Tests | Self::Lints | Self::Format => Command::new("cargo"),
            Self::Proofs => Command::new("lean"),
        };
        let _ = match self {
            Self::Tests => c.args(["test", "--quiet"]),
            Self::Lints => c.args(["clippy", "--quiet", "--all-targets"]),
            Self::Format => c.args(["fmt", "--check"]),
            Self::Proofs => c.arg("proofs/Rigor.lean"),
        };
        let _ = c.current_dir(root);
        c
    }

    #[must_use]
    pub fn prop(self) -> Prop {
        Prop::Conforms(self.requirement())
    }
}

/// An open item of the roadmap: something the project has undertaken to resolve or re-decide.
#[derive(Clone, Debug)]
pub struct BacklogItem {
    pub requirement: Requirement,
    /// Days after founding by which it must be done or consciously re-planned.
    pub review_within: u64,
}

/// The roadmap's "address next" list (docs/05-roadmap.md), as obligations. An item is
/// undertaken the first time the steward runs with it in this list; items that were closed
/// earlier stay in the log as satisfied commitments.
pub const BACKLOG: [BacklogItem; 14] = [
    BacklogItem { requirement: Requirement::lit("snapshots-are-attested-not-trusted"), review_within: 45 },
    BacklogItem { requirement: Requirement::lit("forge-review-evidence-adapter"), review_within: 60 },
    BacklogItem { requirement: Requirement::lit("real-data-for-transfers-and-budgets"), review_within: 60 },
    BacklogItem { requirement: Requirement::lit("expulsion-and-role-change"), review_within: 60 },
    BacklogItem { requirement: Requirement::lit("organisations-interacting-across-boundaries"), review_within: 75 },
    BacklogItem { requirement: Requirement::lit("a-second-kind-of-real-record"), review_within: 75 },
    BacklogItem { requirement: Requirement::lit("open-vocabulary-by-type-parameters"), review_within: 90 },
    BacklogItem { requirement: Requirement::lit("forge-reviews-at-scale-not-a-sample"), review_within: 90 },
    BacklogItem { requirement: Requirement::lit("a-human-in-the-loop-for-reflector-proposals"), review_within: 90 },
    BacklogItem { requirement: Requirement::lit("kernel-verified-against-lean-model"), review_within: 180 },
    // Recommended by the system's own research ("what-should-change-next") and adopted by the owner.
    BacklogItem { requirement: Requirement::lit("controlled-editing-of-existing-files"), review_within: 30 },
    BacklogItem { requirement: Requirement::lit("controlled-changes-to-code-not-only-notes"), review_within: 60 },
    BacklogItem { requirement: Requirement::lit("capabilities-prohibitions-and-lacks-as-distinct-records"), review_within: 60 },
    BacklogItem { requirement: Requirement::lit("checks-that-cover-documents-not-only-code"), review_within: 45 },
];

/// Why the steward could not do its job (as opposed to finding the project wanting).
#[derive(Debug)]
pub enum StewardError {
    Adapter(AdapterError),
    World(Error),
    Clock,
    Mind(rigor_mind::MindError),
    /// A name the system built at run time was not a valid name.
    Name(rigor_kernel::NameError),
    /// A stored change request no longer matches the identity it was verified and approved under.
    ChangeAltered(rigor_institution::model::ChangeId),
}

impl From<rigor_mind::MindError> for StewardError {
    fn from(e: rigor_mind::MindError) -> Self {
        Self::Mind(e)
    }
}
impl From<rigor_kernel::NameError> for StewardError {
    fn from(e: rigor_kernel::NameError) -> Self {
        Self::Name(e)
    }
}

impl From<AdapterError> for StewardError {
    fn from(e: AdapterError) -> Self {
        Self::Adapter(e)
    }
}
impl From<Error> for StewardError {
    fn from(e: Error) -> Self {
        Self::World(e)
    }
}

/// The current day, as whole days since the Unix epoch.
///
/// # Errors
/// If the system clock is before the epoch.
pub fn today() -> Result<u64, StewardError> {
    Ok(SystemTime::now().duration_since(UNIX_EPOCH).map_err(|_| StewardError::Clock)?.as_secs() / 86_400)
}

fn root_unit() -> Result<Box<dyn Unit<Event>>, Error> {
    Ok(Box::new(org(Vec::new())?))
}

/// The project's constitution: who its founding members are and what they may do.
#[must_use]
pub fn constitution() -> Constitution {
    use rigor_institution::model::{ActionKind, Amount, Charter, Method, Trust};
    let m = |id: ActorId, kind| Member { id, kind, principal: None, role: None };
    let ai = |id: ActorId| Member { id, kind: MemberKind::Ai, principal: Some(OWNER), role: None };
    let mut rules = standard_rules(&Trusted { receipts: CI, probes: CI, tests: CI, audits: OWNER, acks: PROJECT });
    // An acknowledgement counts from the project, the owner, or the reviewing model -- never
    // from the drafting model, and (by the self-support rule) never from whoever made the claim.
    let _ = rules.evidence.trusted.insert(Method::Ack, Trust::Only([PROJECT, OWNER, REVIEWER].into_iter().collect()));
    Constitution {
        members: vec![
            m(PROJECT, MemberKind::Org),
            m(OWNER, MemberKind::Human),
            m(MAINTAINER, MemberKind::Human),
            m(CI, MemberKind::Service),
            ai(RESEARCHER),
            ai(REVIEWER),
            m(GATEWAY, MemberKind::Service),
        ],
        charters: vec![Charter {
            name: OWNERSHIP,
            holder: OWNER,
            actions: [ActionKind::Propose, ActionKind::Approve, ActionKind::Admit, ActionKind::Expel, ActionKind::SetRole].into_iter().collect(),
            delegable: false,
        }],
        // What the owner has set aside for the project to spend on consulting models.
        balances: vec![(OWNER, MICRO_USD, Amount(5_000_000))],
        rules,
    }
}

fn found_world(day: u64) -> Result<World, Error> {
    let mut w = World::new(root_unit()?);
    let _ = w.inject(&CONSTITUTION, Event::Genesis(Box::new(constitution())))?;
    let _ = w.tick(day)?;
    Ok(w)
}

fn undertaking(prop: Prop, deadline: Time, tag: Tag) -> Offer {
    Offer { creditor: PROJECT, ant: None, cons: Condition::Holds(prop), deadline, tags: std::iter::once(tag).collect(), parent: None, terms: None }
}

/// What happened to one check in a run.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    /// Evidence from an earlier run is still about the current files: nothing was run.
    StillEvidenced,
    RanAndPassed,
    RanAndFailed,
    /// The check could not be run at all (for example the tool is missing): nothing is established.
    CouldNotObserve,
    Skipped,
}

/// A steward bound to a repository and its persisted log.
#[derive(Debug)]
pub struct Steward {
    pub world: World,
    root: PathBuf,
    log: PathBuf,
    watcher: ArtifactWatcher,
    /// Set when a stored log existed but could not be used (and so was set aside).
    pub previous_log_rejected: Option<PathBuf>,
    /// Set when the stored log was in an older schema and was migrated (the old schema number).
    pub migrated_from: Option<u32>,
    /// Set when the stored log was founded under an older constitution and was re-founded.
    pub refounded: bool,
    /// Set when the record on disk does not reach, or does not match, the point last anchored.
    pub anchor_problem: Option<AnchorProblem>,
}

/// The last point of the record the steward wrote down outside the record itself
/// (`.rigor-anchor` at the repository root, meant to be kept under version control). A log is
/// self-consistent however much of its end is cut off; the anchor is what a cut is measured against.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Anchor {
    pub events: u64,
    pub head: rigor_kernel::Digest,
}

/// How the record fails to meet its anchor.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnchorProblem {
    /// The record has fewer events than were anchored: its end is missing (or all of it).
    Shorter { anchored: u64, found: u64 },
    /// The record has a different event at the anchored position: it was rewritten.
    Different,
}

impl Steward {
    /// Open the steward's world: replay the stored log if there is a usable one, else found anew.
    ///
    /// # Errors
    /// On I/O failure or a kernel fault.
    pub fn open(root: &Path, log: &Path, day: u64) -> Result<Self, StewardError> {
        let mut rejected = None;
        let mut migrated = None;
        let world = if log.exists() {
            match restore(root_unit()?, log) {
                Ok(w) => w,
                Err(AdapterError::Io(e)) => return Err(StewardError::Adapter(AdapterError::Io(e))),
                // Written under an older format: its chain is verified and its history carried over.
                Err(AdapterError::Log(LogError::Schema { found })) => match migrate(root_unit()?, log) {
                    Ok(w) => {
                        let kept = log.with_extension(format!("schema-{found}.jsonl"));
                        let _ = std::fs::copy(log, &kept).map_err(AdapterError::Io)?;
                        migrated = Some(found);
                        w
                    }
                    Err(AdapterError::Io(e)) => return Err(StewardError::Adapter(AdapterError::Io(e))),
                    Err(_) => {
                        let aside = log.with_extension(format!("rejected-day-{day}.jsonl"));
                        std::fs::rename(log, &aside).map_err(AdapterError::Io)?;
                        rejected = Some(aside);
                        found_world(day)?
                    }
                },
                Err(_) => {
                    // Tampered with, or written by an incompatible version of the vocabulary.
                    // History is never silently discarded: it is set aside, and a new one begins.
                    let aside = log.with_extension(format!("rejected-day-{day}.jsonl"));
                    std::fs::rename(log, &aside).map_err(AdapterError::Io)?;
                    rejected = Some(aside);
                    found_world(day)?
                }
            }
        } else {
            found_world(day)?
        };
        // A log founded before the project had an owner, models and a budget is carried over
        // under the current constitution, in the open (the old file is kept).
        let mut refounded = false;
        let world = if log.exists() && !world.inst()?.valid(&rigor_institution::model::CapId::Base(OWNER)) {
            let kept = log.with_extension(format!("before-refounding-day-{day}.jsonl"));
            let _ = std::fs::copy(log, &kept).map_err(AdapterError::Io)?;
            if migrated.is_some() {
                // The migrated world is only in memory: write it so that re-founding reads current-schema bytes.
                save(&world, log)?;
            }
            refounded = true;
            refound(root_unit()?, log, &constitution())?
        } else {
            world
        };
        // Measure the record against its anchor, unless it was just rebuilt in the open.
        let anchor: Option<Anchor> = std::fs::read_to_string(root.join(".rigor-anchor")).ok().and_then(|t| serde_json::from_str(&t).ok());
        let anchor_problem = match anchor {
            Some(a) if migrated.is_none() && !refounded && rejected.is_none() => {
                let found = world.rt.log().next_seq();
                let at_anchor = a.events.checked_sub(1).and_then(|i| usize::try_from(i).ok()).and_then(|i| world.rt.log().entries().get(i));
                if found < a.events {
                    Some(AnchorProblem::Shorter { anchored: a.events, found })
                } else if at_anchor.is_some_and(|e| e.id != a.head) {
                    Some(AnchorProblem::Different)
                } else {
                    None
                }
            }
            Some(_) | None => None,
        };
        let rust = [files_under(&root.join("crates"), &[])?, vec![root.join("Cargo.toml"), root.join("Cargo.lock"), root.join("rustfmt.toml")]].concat();
        let watcher = ArtifactWatcher::of_sets([(SOURCES, rust), (PROOFS, files_under(&root.join("proofs"), &["lean"])?)]);
        Ok(Self {
            world,
            root: root.to_path_buf(),
            log: log.to_path_buf(),
            watcher,
            previous_log_rejected: rejected,
            migrated_from: migrated,
            refounded,
            anchor_problem,
        })
    }

    /// One pass: observe the files, undertake today's upkeep, run only the checks whose claims
    /// are not currently evidenced, and persist the log.
    ///
    /// # Errors
    /// On I/O failure or a kernel fault. A failing check is not an error: it is a finding.
    pub fn run(&mut self, day: u64, force: bool, skip: &BTreeSet<Check>) -> Result<Vec<(Check, Outcome)>, StewardError> {
        let _ = self.world.tick(day)?;
        // Undertake any roadmap item not yet undertaken (whatever its status now).
        let undertaken: BTreeSet<Prop> = self
            .world
            .tagged(Tag::Task)?
            .into_iter()
            .filter_map(|r| match &r.terms().cons {
                Condition::Holds(p) => Some(p.clone()),
                Condition::Unrefuted(_) | Condition::Paid { .. } => None,
            })
            .collect();
        for item in &BACKLOG {
            let prop = Prop::Conforms(item.requirement.clone());
            if !undertaken.contains(&prop) {
                let _ = self.world.act(&MAINTAINER, Act::Commit(undertaking(prop, Time(day).plus(item.review_within), Tag::Task)))?;
            }
        }
        let _ = self.watcher.scan(&mut self.world)?;
        let mut outcomes = Vec::new();
        for check in Check::ALL {
            // Two propositions. "It conforms" is about the files as they are now: its evidence is
            // bound to their digests and goes stale when they change. "It held on day d" is a
            // fact about the past: nothing that happens later can make it stale, so a good day
            // stays good and a bad day stays bad whatever is done to the files afterwards.
            let now_prop = check.prop();
            let day_prop = Prop::HeldOn { requirement: check.requirement(), day };
            // A duplicate (a second run on the same day) is refused by the norms unit; that is fine.
            let _ = self.world.act(&MAINTAINER, Act::Commit(undertaking(day_prop.clone(), Time(day), Tag::Upkeep)))?;
            let outcome = if skip.contains(&check) {
                Outcome::Skipped
            } else if !force && self.evidenced(check)? {
                Outcome::StillEvidenced
            } else {
                match run_check(&mut self.world, &mut self.watcher, &mut check.command(&self.root), &now_prop, &[check.artifact()], &CI) {
                    Ok(true) => Outcome::RanAndPassed,
                    Ok(false) => Outcome::RanAndFailed,
                    Err(AdapterError::Io(_)) => Outcome::CouldNotObserve,
                    Err(e) => return Err(e.into()),
                }
            };
            // The day's fact: what a real check, on the files as they are now, shows as of this run.
            if !matches!(outcome, Outcome::Skipped | Outcome::CouldNotObserve) {
                use rigor_institution::model::About;
                let holds = self.evidenced(check)?;
                let recorded = self.world.epi()?.assessment(&day_prop);
                if (recorded == Assessment::Accepted) != holds || recorded == Assessment::Unsupported {
                    let on_it = self.world.epi()?.evidence_on(&day_prop);
                    let mine: Vec<_> = self.world.epi()?.standing_evidence().filter(|e| on_it.contains(&e.id) && e.source == CI).map(|e| e.id).collect();
                    for e in mine {
                        let _ = self.world.act(&CI, Act::Retract { evidence: e })?;
                    }
                    let about = if holds { About::Supports(day_prop) } else { About::Rebuts(day_prop) };
                    let _ = self.world.act(&CI, attest(about, Method::Test, &format!("as of this run on day {day}")))?;
                }
            }
            outcomes.push((check, outcome));
        }
        // A lapse is put right when the requirement that lapsed is evidenced again: say so, as
        // evidence, so that the remediation duty the lapse created can be discharged.
        let mut put_right = Vec::new();
        for r in self.world.tagged(Tag::Remediation)? {
            let lapsed = r.terms().basis.as_ref().and_then(|b| self.world.norms().ok()?.record(&b.of).map(|x| x.terms().cons.clone()));
            let check = match lapsed {
                Some(Condition::Holds(Prop::HeldOn { requirement, .. } | Prop::Conforms(requirement))) => {
                    Check::ALL.into_iter().find(|c| c.requirement() == requirement)
                }
                _ => None,
            };
            if let (Some(check), Condition::Holds(remediated), Some(Status::Detached)) = (check, &r.terms().cons, r.status) {
                if self.evidenced(check)? {
                    put_right.push(remediated.clone());
                }
            }
        }
        for p in put_right {
            let _ = self.world.act(&CI, attest(rigor_institution::model::About::Supports(p), Method::Test, "the lapsed requirement is evidenced again"))?;
        }
        self.save()?;
        Ok(outcomes)
    }

    /// The project accepts that a backlog item is done (or consciously closed).
    ///
    /// # Errors
    /// On I/O failure or a kernel fault.
    pub fn accept(&mut self, requirement: Requirement, day: u64) -> Result<bool, StewardError> {
        // Only an undertaking on the roadmap can be accepted as done. The project's claims about
        // itself cannot: for those the only evidence is a real run of the check.
        let is_roadmap_item = self.world.tagged(Tag::Task)?.iter().any(|r| r.terms().cons == Condition::Holds(Prop::Conforms(requirement.clone())));
        if !is_roadmap_item || Check::ALL.iter().any(|c| c.requirement() == requirement) {
            return Ok(false);
        }
        let _ = self.world.tick(day)?;
        let _ = self.world.act(&MAINTAINER, Act::Claim { prop: Prop::Conforms(requirement.clone()) })?;
        let _ =
            self.world.act(&PROJECT, attest(rigor_institution::model::About::Supports(Prop::Conforms(requirement)), Method::Ack, "accepted by the project"))?;
        self.save()?;
        Ok(true)
    }

    /// The repository this steward looks after.
    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Where the steward keeps what it writes (answers, change requests, the dashboard).
    #[must_use]
    pub fn home(&self) -> PathBuf {
        self.log.parent().map_or_else(|| self.root.join(".rigor"), Path::to_path_buf)
    }

    /// The owner decides a question put to them. Only ever called on the owner's instruction.
    ///
    /// # Errors
    /// On I/O failure or a kernel fault.
    pub fn owner_decides(&mut self, requirement: Requirement, yes: bool, day: u64) -> Result<(), StewardError> {
        use rigor_institution::model::About;
        let _ = self.world.tick(day)?;
        let prop = Prop::Conforms(requirement);
        let about = if yes { About::Supports(prop) } else { About::Rebuts(prop) };
        let _ = self.world.act(&OWNER, attest(about, Method::Audit, "decided by the owner"))?;
        self.save()
    }

    /// The owner sets aside a budget for consulting models: a capability to transfer from the
    /// owner's account, limited to `micro_usd`, held by the steward's service account.
    ///
    /// # Errors
    /// On I/O failure or a kernel fault.
    pub fn owner_grants_budget(&mut self, micro_usd: u64) -> Result<bool, StewardError> {
        use rigor_institution::model::{ActionKind, Amount, CapId, Grant, Scope};
        let grant = Grant {
            to: CI,
            actions: std::iter::once(ActionKind::Transfer).collect(),
            scope: Scope { from: Some(OWNER), purpose: None },
            limits: std::iter::once((MICRO_USD, Amount(micro_usd))).collect(),
            expires: None,
            delegable: false,
        };
        let entries = self.world.act_with(&OWNER, &CapId::Own(OWNER), Act::Delegate(grant))?;
        self.save()?;
        Ok(entries.iter().any(|e| matches!(e.event, Event::Delegated(_))))
    }

    /// The budgets the steward currently holds: capability, limit, and how much is spent.
    ///
    /// # Errors
    /// If the world lacks an institutional unit.
    pub fn budgets(&self) -> Result<Vec<(rigor_institution::model::CapId, u64, u64)>, StewardError> {
        let inst = self.world.inst()?;
        Ok(inst
            .caps()
            .values()
            .filter(|c| c.holder == CI && c.scope.from == Some(OWNER) && inst.valid(&c.id))
            .filter_map(|c| c.limits.get(&MICRO_USD).map(|lim| (c.id.clone(), lim.0, inst.spent(&c.id, &MICRO_USD).0)))
            .collect())
    }

    /// What is left to spend, across budgets.
    ///
    /// # Errors
    /// If the world lacks an institutional unit.
    pub fn budget_left(&self) -> Result<u64, StewardError> {
        Ok(self.budgets()?.iter().map(|(_, limit, spent)| limit.saturating_sub(*spent)).sum())
    }

    /// Pay for a consultation out of the budget. The institution's own authority and ledger
    /// units enforce the limit: returns false if the payment was refused.
    ///
    /// # Errors
    /// On a kernel fault.
    pub fn pay(&mut self, micro_usd: u64) -> Result<bool, StewardError> {
        use rigor_institution::model::{Amount, Memo, Transfer};
        if micro_usd == 0 {
            return Ok(true);
        }
        for (cap, limit, spent) in self.budgets()? {
            if limit.saturating_sub(spent) >= micro_usd {
                let t = Transfer { from: OWNER, to: GATEWAY, resource: MICRO_USD, amount: Amount(micro_usd), memo: Memo::None };
                let entries = self.world.act_with(&CI, &cap, Act::Transfer(t))?;
                return Ok(entries.iter().any(|e| matches!(e.event, Event::Transferred { .. })));
            }
        }
        Ok(false)
    }

    /// The owner accepts the record as it now is, after an anchor failure has been looked into.
    ///
    /// # Errors
    /// On I/O failure.
    pub fn owner_reanchors(&mut self) -> Result<(), StewardError> {
        self.anchor_problem = None;
        self.save()
    }

    /// Persist the log.
    ///
    /// # Errors
    /// On I/O failure.
    pub fn persist(&self) -> Result<(), StewardError> {
        self.save()
    }

    fn save(&self) -> Result<(), StewardError> {
        if let Some(dir) = self.log.parent() {
            std::fs::create_dir_all(dir).map_err(AdapterError::Io)?;
        }
        save(&self.world, &self.log)?;
        // The anchor follows the record -- except when the record has failed to meet it: then
        // it is left as it is, so that the failure cannot be papered over by simply running again.
        if self.anchor_problem.is_none() {
            let anchor = Anchor { events: self.world.rt.log().next_seq(), head: self.world.rt.log().head() };
            let text = serde_json::to_string(&anchor).map_err(|e| StewardError::Adapter(AdapterError::Snapshot(rigor_kernel::EncodingError(e.to_string()))))?;
            std::fs::write(self.root.join(".rigor-anchor"), text).map_err(AdapterError::Io)?;
        }
        Ok(())
    }

    /// Whether a self-claim is currently evidenced THE ONLY WAY IT CAN BE: by a real run of its
    /// check, by the service account, bound to the digest of the files as they now are.
    /// Nobody's say-so counts, whatever their standing.
    ///
    /// # Errors
    /// If the world lacks an institutional unit.
    pub fn evidenced(&self, check: Check) -> Result<bool, StewardError> {
        let epi = self.world.epi()?;
        let standing = epi.standing(&check.prop());
        Ok(standing.assessment == Assessment::Accepted
            && epi.standing_evidence().any(|e| standing.basis.contains(&e.id) && e.source == CI && e.method == Method::Test && !e.artifacts.is_empty()))
    }

    /// Whether every self-claim that was not skipped is currently evidenced by a real check,
    /// and the record reaches its anchor.
    ///
    /// # Errors
    /// If the world lacks an institutional unit.
    pub fn conformant(&self, skip: &BTreeSet<Check>) -> Result<bool, StewardError> {
        let mut all = self.anchor_problem.is_none();
        for c in Check::ALL.into_iter().filter(|c| !skip.contains(c)) {
            all = all && self.evidenced(c)?;
        }
        Ok(all)
    }

    /// Commitments carrying a tag, with their current status.
    ///
    /// # Errors
    /// If the world lacks an institutional unit.
    pub fn commitments(&self, tag: Tag) -> Result<Vec<(Commitment, Status)>, StewardError> {
        Ok(self.world.tagged(tag)?.into_iter().filter_map(|r| r.status.map(|s| (r.commitment.clone(), s))).collect())
    }
}

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

use crate::adapters::{files_under, migrate, restore, run_check, save, AdapterError, ArtifactWatcher};
use crate::kernel::LogError;
use crate::kernel::Unit;
use crate::model::{
    Act, ActorId, ArtifactId, Assessment, Commitment, Condition, Constitution, Event, Member, MemberKind, Method, Offer, Prop, Requirement, Status, Tag, Time,
};
use crate::org::{org, standard_rules, Trusted};
use crate::scenario::attest;
use crate::world::{Error, World, CONSTITUTION};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

pub const PROJECT: ActorId = ActorId::lit("project");
pub const MAINTAINER: ActorId = ActorId::lit("maintainer");
pub const CI: ActorId = ActorId::lit("ci");

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
pub const BACKLOG: [BacklogItem; 10] = [
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
];

/// Why the steward could not do its job (as opposed to finding the project wanting).
#[derive(Debug)]
pub enum StewardError {
    Adapter(AdapterError),
    World(Error),
    Clock,
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

fn found_world(day: u64) -> Result<World, Error> {
    let m = |id: ActorId, kind| Member { id, kind, principal: None, role: None };
    let constitution = Constitution {
        members: vec![m(PROJECT, MemberKind::Org), m(MAINTAINER, MemberKind::Human), m(CI, MemberKind::Service)],
        charters: Vec::new(),
        balances: Vec::new(),
        rules: standard_rules(&Trusted { receipts: CI, probes: CI, tests: CI, audits: PROJECT, acks: PROJECT }),
    };
    let mut w = World::new(root_unit()?);
    let _ = w.inject(&CONSTITUTION, Event::Genesis(Box::new(constitution)))?;
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
        let rust =
            [files_under(&root.join("crates"), &["rs", "toml"])?, vec![root.join("Cargo.toml"), root.join("Cargo.lock"), root.join("rustfmt.toml")]].concat();
        let watcher = ArtifactWatcher::of_sets([(SOURCES, rust), (PROOFS, files_under(&root.join("proofs"), &["lean"])?)]);
        Ok(Self { world, root: root.to_path_buf(), log: log.to_path_buf(), watcher, previous_log_rejected: rejected, migrated_from: migrated })
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
            let prop = check.prop();
            // A duplicate (a second run on the same day) is refused by the norms unit; that is fine.
            let _ = self.world.act(&MAINTAINER, Act::Commit(undertaking(prop.clone(), Time(day), Tag::Upkeep)))?;
            let outcome = if skip.contains(&check) {
                Outcome::Skipped
            } else if !force && self.world.epi()?.assessment(&prop) == Assessment::Accepted {
                Outcome::StillEvidenced
            } else {
                match run_check(&mut self.world, &mut self.watcher, &mut check.command(&self.root), &prop, &[check.artifact()], &CI) {
                    Ok(true) => Outcome::RanAndPassed,
                    Ok(false) => Outcome::RanAndFailed,
                    Err(AdapterError::Io(_)) => Outcome::CouldNotObserve,
                    Err(e) => return Err(e.into()),
                }
            };
            outcomes.push((check, outcome));
        }
        // A lapse is put right when the requirement that lapsed is evidenced again: say so, as
        // evidence, so that the remediation duty the lapse created can be discharged.
        let mut put_right = Vec::new();
        for r in self.world.tagged(Tag::Remediation)? {
            let lapsed = r.terms().basis.as_ref().and_then(|b| self.world.norms().ok()?.record(&b.of).map(|x| x.terms().cons.clone()));
            if let (Some(Condition::Holds(req)), Condition::Holds(remediated), Some(Status::Detached)) = (lapsed, &r.terms().cons, r.status) {
                if self.world.epi()?.assessment(&req) == Assessment::Accepted {
                    put_right.push(remediated.clone());
                }
            }
        }
        for p in put_right {
            let _ = self.world.act(&CI, attest(crate::model::About::Supports(p), Method::Test, "the lapsed requirement is evidenced again"))?;
        }
        self.save()?;
        Ok(outcomes)
    }

    /// The project accepts that a backlog item is done (or consciously closed).
    ///
    /// # Errors
    /// On I/O failure or a kernel fault.
    pub fn accept(&mut self, requirement: Requirement, day: u64) -> Result<(), StewardError> {
        let _ = self.world.tick(day)?;
        let _ = self.world.act(&MAINTAINER, Act::Claim { prop: Prop::Conforms(requirement.clone()) })?;
        let _ = self.world.act(&PROJECT, attest(crate::model::About::Supports(Prop::Conforms(requirement)), Method::Ack, "accepted by the project"))?;
        self.save()
    }

    fn save(&self) -> Result<(), StewardError> {
        if let Some(dir) = self.log.parent() {
            std::fs::create_dir_all(dir).map_err(AdapterError::Io)?;
        }
        Ok(save(&self.world, &self.log)?)
    }

    /// Whether every self-claim that was not skipped is currently evidenced.
    ///
    /// # Errors
    /// If the world lacks an institutional unit.
    pub fn conformant(&self, skip: &BTreeSet<Check>) -> Result<bool, StewardError> {
        let epi = self.world.epi()?;
        Ok(Check::ALL.iter().filter(|c| !skip.contains(c)).all(|c| epi.assessment(&c.prop()) == Assessment::Accepted))
    }

    /// Commitments carrying a tag, with their current status.
    ///
    /// # Errors
    /// If the world lacks an institutional unit.
    pub fn commitments(&self, tag: Tag) -> Result<Vec<(Commitment, Status)>, StewardError> {
        Ok(self.world.tagged(tag)?.into_iter().filter_map(|r| r.status.map(|s| (r.commitment.clone(), s))).collect())
    }
}

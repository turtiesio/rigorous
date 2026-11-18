//! What the system can do, what it cannot, and what each thing needs.
//!
//! Every ability is declared with its needs. Each need is PROBED for real (is the program
//! there, is there a credential, does the gateway answer, is there budget, has the owner
//! decided) and the result is recorded as evidence about the proposition "the system can do
//! this". So the system's account of itself is assessed like any other claim, changes when the
//! facts change, and names exactly what is lacking.

use crate::{Steward, StewardError, CI};
use rigor_institution::model::{About, Act, Assessment, Method, Prop, Requirement};
use rigor_institution::world::attest;
use rigor_kernel::Name;
use rigor_mind::Gateway;
use std::process::{Command, Stdio};

/// A program the system may need on the machine it runs on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Program {
    Cargo,
    Clippy,
    Rustfmt,
    Lean,
    Git,
    /// A tool able to verify Rust code against a specification.
    RustVerifier,
}

impl Program {
    fn present(self) -> bool {
        let run =
            |program: &str, args: &[&str]| Command::new(program).args(args).stdout(Stdio::null()).stderr(Stdio::null()).status().is_ok_and(|s| s.success());
        match self {
            Self::Cargo => run("cargo", &["--version"]),
            Self::Clippy => run("cargo", &["clippy", "--version"]),
            Self::Rustfmt => run("cargo", &["fmt", "--version"]),
            Self::Lean => run("lean", &["--version"]),
            Self::Git => run("git", &["--version"]),
            Self::RustVerifier => run("verus", &["--version"]) || run("aeneas", &["--version"]) || run("cargo", &["creusot", "--version"]),
        }
    }
}

/// Something that is not built, and why that is the present position.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unbuilt {
    /// There is one log and one clock; nothing here is a step towards agreement between machines.
    ManyLogsOneTruth,
    /// A change may add files; nothing yet edits or deletes an existing one under control.
    EditingExistingFiles,
    /// Nothing reads a forge's pull requests wholesale.
    ForgeCrawl,
    /// No tool chain connects the Rust code to the Lean model.
    CodeToModelProof,
}

/// One thing an ability depends on.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Need {
    Program(Program),
    /// A credential for the model gateway can be found.
    Credential,
    /// The model gateway answers.
    GatewayAnswers,
    /// The owner has set aside money that is not yet spent.
    Budget,
    /// The owner has decided this question in favour.
    OwnerSaysYes(Requirement),
    /// The owner approves each instance (so the ability exists only one approval at a time).
    OwnerApprovesEach,
    Ability(Ability),
    Unbuilt(Unbuilt),
    /// Not a gap: the system is designed not to do this.
    ByDesignNever,
}

/// The things the system might be asked to do.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Ability {
    CheckItself,
    CheckItsProofs,
    ReadHistories,
    ConsultModels,
    Research,
    ProposeChanges,
    VerifyChanges,
    ApplyChanges,
    ChangeItselfUnsupervised,
    EditExistingFiles,
    ProveCodeMatchesModel,
    ReadAForgeAtScale,
    RunAcrossMachines,
    EstablishTruth,
}

impl Ability {
    pub const ALL: [Self; 14] = [
        Self::CheckItself,
        Self::CheckItsProofs,
        Self::ReadHistories,
        Self::ConsultModels,
        Self::Research,
        Self::ProposeChanges,
        Self::VerifyChanges,
        Self::ApplyChanges,
        Self::ChangeItselfUnsupervised,
        Self::EditExistingFiles,
        Self::ProveCodeMatchesModel,
        Self::ReadAForgeAtScale,
        Self::RunAcrossMachines,
        Self::EstablishTruth,
    ];

    /// The name under which the ability appears in the log.
    #[must_use]
    pub const fn name(self) -> Name {
        Name::lit(match self {
            Self::CheckItself => "check-itself",
            Self::CheckItsProofs => "check-its-proofs",
            Self::ReadHistories => "read-histories",
            Self::ConsultModels => "consult-models",
            Self::Research => "research",
            Self::ProposeChanges => "propose-changes",
            Self::VerifyChanges => "verify-changes",
            Self::ApplyChanges => "apply-changes",
            Self::ChangeItselfUnsupervised => "change-itself-unsupervised",
            Self::EditExistingFiles => "edit-existing-files",
            Self::ProveCodeMatchesModel => "prove-code-matches-model",
            Self::ReadAForgeAtScale => "read-a-forge-at-scale",
            Self::RunAcrossMachines => "run-across-machines",
            Self::EstablishTruth => "establish-truth",
        })
    }

    /// What it means, for a person reading the dashboard.
    #[must_use]
    pub const fn meaning(self) -> &'static str {
        match self {
            Self::CheckItself => "Run its own tests, lints and format check, and record the evidence",
            Self::CheckItsProofs => "Re-check the machine proofs about its model",
            Self::ReadHistories => "Read version-control histories as records of what people did",
            Self::ConsultModels => "Ask language models, within the owner's budget",
            Self::Research => "Put a question to one model, have another review the answer, and record both",
            Self::ProposeChanges => "Draft a change to the things it manages, as a request",
            Self::VerifyChanges => "Try a proposed change in a sandbox and record whether the checks still pass",
            Self::ApplyChanges => "Apply a change that is verified and approved",
            Self::ChangeItselfUnsupervised => "Apply changes to itself without the owner's approval",
            Self::EditExistingFiles => "Change or delete existing files under the same control as additions",
            Self::ProveCodeMatchesModel => "Prove that its code does what its proved model says",
            Self::ReadAForgeAtScale => "Read all of a project's reviews and check runs from its forge",
            Self::RunAcrossMachines => "Keep one consistent record across several machines",
            Self::EstablishTruth => "Establish that a claim is true, as opposed to assessing the evidence for it",
        }
    }

    /// What it depends on.
    #[must_use]
    pub fn needs(self) -> Vec<Need> {
        use Need::{ByDesignNever, Credential, GatewayAnswers, OwnerApprovesEach, OwnerSaysYes};
        match self {
            Self::CheckItself => vec![Need::Program(Program::Cargo), Need::Program(Program::Clippy), Need::Program(Program::Rustfmt)],
            Self::CheckItsProofs => vec![Need::Program(Program::Lean)],
            Self::ReadHistories => vec![Need::Program(Program::Git)],
            Self::ConsultModels => vec![Credential, GatewayAnswers, Need::Budget],
            Self::Research => vec![Need::Ability(Self::ConsultModels)],
            Self::ProposeChanges => vec![Need::Ability(Self::ConsultModels)],
            Self::VerifyChanges => vec![Need::Ability(Self::CheckItself)],
            Self::ApplyChanges => vec![Need::Ability(Self::VerifyChanges), OwnerApprovesEach],
            Self::ChangeItselfUnsupervised => vec![Need::Ability(Self::VerifyChanges), OwnerSaysYes(Requirement::lit("unsupervised-changes-allowed"))],
            Self::EditExistingFiles => vec![Need::Unbuilt(Unbuilt::EditingExistingFiles)],
            Self::ProveCodeMatchesModel => vec![Need::Program(Program::RustVerifier), Need::Unbuilt(Unbuilt::CodeToModelProof)],
            Self::ReadAForgeAtScale => vec![OwnerSaysYes(Requirement::lit("use-the-owners-forge-credential")), Need::Unbuilt(Unbuilt::ForgeCrawl)],
            Self::RunAcrossMachines => vec![Need::Unbuilt(Unbuilt::ManyLogsOneTruth)],
            Self::EstablishTruth => vec![ByDesignNever],
        }
    }
}

/// Where an ability stands.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Standing {
    /// Every need is met.
    Can,
    /// Every need is met, and each use still waits for the owner.
    CanWithApproval,
    /// These needs are not met.
    Cannot(Vec<Need>),
}

/// The system's account of itself at one moment.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelfReport {
    pub abilities: Vec<(Ability, Standing)>,
}

impl SelfReport {
    #[must_use]
    pub fn standing(&self, a: Ability) -> Option<&Standing> {
        self.abilities.iter().find(|(x, _)| *x == a).map(|(_, s)| s)
    }

    #[must_use]
    pub fn can(&self, a: Ability) -> bool {
        matches!(self.standing(a), Some(Standing::Can | Standing::CanWithApproval))
    }

    /// A plain-text account, for giving to a model as context.
    #[must_use]
    pub fn describe(&self) -> String {
        let mut out = String::new();
        for (a, s) in &self.abilities {
            let line = match s {
                Standing::Can => format!("CAN {}: {}\n", a.name(), a.meaning()),
                Standing::CanWithApproval => format!("CAN, WITH THE OWNER'S APPROVAL EACH TIME, {}: {}\n", a.name(), a.meaning()),
                Standing::Cannot(lacking) => {
                    format!("CANNOT {}: {} -- lacking: {}\n", a.name(), a.meaning(), lacking.iter().map(describe_need).collect::<Vec<_>>().join("; "))
                }
            };
            out.push_str(&line);
        }
        out
    }
}

/// A need in words.
#[must_use]
pub fn describe_need(n: &Need) -> String {
    match n {
        Need::Program(p) => format!("the program {p:?} on this machine"),
        Need::Credential => "a credential for the model gateway".to_owned(),
        Need::GatewayAnswers => "the model gateway answering".to_owned(),
        Need::Budget => "unspent budget set aside by the owner".to_owned(),
        Need::OwnerSaysYes(r) => format!("the owner's decision in favour of '{r}'"),
        Need::OwnerApprovesEach => "the owner's approval, each time".to_owned(),
        Need::Ability(a) => format!("the ability '{}'", a.name()),
        Need::Unbuilt(u) => match u {
            Unbuilt::ManyLogsOneTruth => "agreement between machines on one order of events (not built)".to_owned(),
            Unbuilt::EditingExistingFiles => "controlled editing of existing files (not built: changes can only add files)".to_owned(),
            Unbuilt::ForgeCrawl => "a crawler for a forge's reviews (not built)".to_owned(),
            Unbuilt::CodeToModelProof => "a proof connecting the code to the model (not built)".to_owned(),
        },
        Need::ByDesignNever => "nothing: by design it never does this".to_owned(),
    }
}

impl Steward {
    fn met(&self, need: &Need, gateway: &Gateway, gateway_answers: bool, depth: u8) -> Result<bool, StewardError> {
        Ok(match need {
            Need::Program(p) => p.present(),
            Need::Credential => gateway.has_credential(),
            Need::GatewayAnswers => gateway_answers,
            Need::Budget => self.budget_left()? > 0,
            Need::OwnerSaysYes(r) => self.world.epi()?.assessment(&Prop::Conforms(r.clone())) == Assessment::Accepted,
            Need::OwnerApprovesEach => true,
            Need::Ability(a) if depth < 8 => {
                let mut all = true;
                for n in a.needs() {
                    all = all && self.met(&n, gateway, gateway_answers, depth + 1)?;
                }
                all
            }
            Need::Ability(_) | Need::Unbuilt(_) | Need::ByDesignNever => false,
        })
    }

    /// Probe every need of every ability, and record what changed as evidence.
    ///
    /// # Errors
    /// On a kernel fault.
    pub fn examine_itself(&mut self, gateway: &Gateway) -> Result<SelfReport, StewardError> {
        let gateway_answers = gateway.has_credential() && gateway.reachable().is_ok();
        let mut abilities = Vec::new();
        for a in Ability::ALL {
            let mut lacking = Vec::new();
            for n in a.needs() {
                if !self.met(&n, gateway, gateway_answers, 0)? {
                    lacking.push(n);
                }
            }
            let standing = if !lacking.is_empty() {
                Standing::Cannot(lacking)
            } else if a.needs().contains(&Need::OwnerApprovesEach) {
                Standing::CanWithApproval
            } else {
                Standing::Can
            };
            // Record only a change: withdraw the probe that no longer describes the facts, attest the new one.
            let prop = Prop::Can(a.name());
            let can = !matches!(standing, Standing::Cannot(_));
            let recorded = self.world.epi()?.standing(&prop);
            let was = match recorded.assessment {
                Assessment::Accepted => Some(true),
                Assessment::Refuted => Some(false),
                Assessment::Contested | Assessment::Unsupported => None,
            };
            if was != Some(can) {
                for old in recorded.basis {
                    let _ = self.world.act(&CI, Act::Retract { evidence: old })?;
                }
                let about = if can { About::Supports(prop) } else { About::Rebuts(prop) };
                let _ = self.world.act(&CI, attest(about, Method::Probe, "probed every need of this ability"))?;
            }
            abilities.push((a, standing));
        }
        Ok(SelfReport { abilities })
    }
}

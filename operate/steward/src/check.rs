//! The checks the project runs on itself, and the proposition each one is evidence for.

use crate::cast::*;
use rigor_institution::model::{ArtifactId, Prop, Requirement};
use std::path::Path;
use std::process::Command;

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

//! Who may say what from outside: the few voices of the open world, and what each may say.

use super::Event;
use rigor_waist::{Name, Path};

/// The clock: the only voice that says what time it is.
pub const CLOCK: Name = Name::lit("clock");
/// The founding: the only voice that gives a constitution.
pub const CONSTITUTION: Name = Name::lit("constitution");
/// The observer of artifacts: the only voice that says a file changed.
pub const FILESYSTEM: Name = Name::lit("fs");
/// The migrator: the only voice that records that a history was carried over.
pub const MIGRATOR: Name = Name::lit("migrator");

impl Event {
    /// Whether the open world may say this, as this actor. That the actor IS who it is named
    /// as is assumed (authentication is outside the model); that the clock's words come only
    /// from the clock is not.
    #[must_use]
    pub fn admissible_from(&self, actor: &Path) -> bool {
        let said_by = |voice: &Name| actor.0.last() == Some(voice);
        match self {
            Self::Genesis(_) => said_by(&CONSTITUTION),
            Self::Tick(_) => said_by(&CLOCK),
            Self::ArtifactChanged { .. } => said_by(&FILESYSTEM),
            Self::Migrated { .. } => said_by(&MIGRATOR),
            // An actor's attempt, or a message between actors: anyone may try; the units decide.
            Self::Do { .. } | Self::Signal { .. } => true,
            // Addressed to a nested organisation: judged by what it carries.
            Self::Scoped { inner, .. } => inner.admissible_from(actor),
            // Everything else is the word of a unit and can only be produced by one.
            _ => false,
        }
    }
}

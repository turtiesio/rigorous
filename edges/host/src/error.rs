//! Why the host adapter could not observe or persist.

use rigor_institution::model::ArtifactId;
use rigor_institution::world::Error;
use rigor_waist::LogError;

/// Why an adapter could not observe.
#[derive(Debug)]
pub enum AdapterError {
    Io(std::io::Error),
    World(Error),
    Log(LogError),
    /// The regenerated log differs from the stored one.
    ReplayDiverged,
    UnknownArtifact(ArtifactId),
    /// A stimulus in an old log has no form in the current vocabulary.
    Unmigratable {
        index: usize,
    },
    Snapshot(rigor_waist::EncodingError),
    /// The world no longer holds the history a snapshot audit must replay.
    NotAuditable,
    /// Nobody trusted has shown this snapshot to be the state its log produces.
    SnapshotNotAttested,
}

impl From<Error> for AdapterError {
    fn from(e: Error) -> Self {
        Self::World(e)
    }
}

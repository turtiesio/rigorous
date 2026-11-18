//! Why the steward could not do something.

use rigor_adapter_host::AdapterError;
use rigor_institution::world::Error;

/// Why the steward could not do its job (as opposed to finding the project wanting).
#[derive(Debug)]
pub enum StewardError {
    Adapter(AdapterError),
    World(Error),
    Clock,
    Mind(rigor_mind::MindError),
    /// A name the system built at run time was not a valid name.
    Name(rigor_waist::NameError),
    /// A stored change request no longer matches the identity it was verified and approved under.
    ChangeAltered(rigor_institution::model::ChangeId),
}

impl From<rigor_mind::MindError> for StewardError {
    fn from(e: rigor_mind::MindError) -> Self {
        Self::Mind(e)
    }
}
impl From<rigor_waist::NameError> for StewardError {
    fn from(e: rigor_waist::NameError) -> Self {
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

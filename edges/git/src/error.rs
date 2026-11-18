//! Why a history could not be read or ingested.

use rigor_institution::world::Error;
use rigor_waist::NameError;

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
    Adapter(rigor_adapter_host::AdapterError),
}

impl From<Error> for GitError {
    fn from(e: Error) -> Self {
        Self::World(e)
    }
}

impl From<rigor_adapter_host::AdapterError> for GitError {
    fn from(e: rigor_adapter_host::AdapterError) -> Self {
        Self::Adapter(e)
    }
}

impl From<NameError> for GitError {
    fn from(e: NameError) -> Self {
        Self::Name(e)
    }
}

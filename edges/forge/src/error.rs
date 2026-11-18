//! Why an archive could not be read or ingested.

use rigor_institution::world::Error;
use rigor_waist::NameError;

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

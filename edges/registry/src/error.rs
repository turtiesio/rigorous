//! Why an index could not be read or ingested.

use rigor_institution::world::Error;
use rigor_waist::NameError;

/// Why registry data could not be read.
#[derive(Debug)]
pub enum RegistryError {
    Malformed { line: usize },
    Name(NameError),
    World(Error),
}

impl From<Error> for RegistryError {
    fn from(e: Error) -> Self {
        Self::World(e)
    }
}

//! Why blocks could not be read or settled.

use rigor_institution::world::Error;
use rigor_waist::NameError;

/// Why a block file could not be read or settled.
#[derive(Debug)]
pub enum ChainError {
    Malformed { line: usize },
    Name(NameError),
    World(Error),
}

impl From<Error> for ChainError {
    fn from(e: Error) -> Self {
        Self::World(e)
    }
}

impl From<NameError> for ChainError {
    fn from(e: NameError) -> Self {
        Self::Name(e)
    }
}

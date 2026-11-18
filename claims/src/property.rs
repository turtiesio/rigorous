//! The six classes a property is placed in. A property usually sits in several: conservation is
//! preserved by composition AND enforced by the ledger when a transfer happens.

use crate::claim::Id;

/// What is said about a property in one class, and the claims that say it rests on.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Facet {
    pub note: &'static str,
    pub rests_on: &'static [Id],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Property {
    pub name: &'static str,
    /// Preserved by composition, under the stated assumptions.
    pub composition: Option<Facet>,
    /// Statically provable or model-checkable.
    pub statically: Option<Facet>,
    /// Enforceable at run time.
    pub at_run_time: Option<Facet>,
    /// Observable but not preventable.
    pub observable_only: Option<Facet>,
    /// Dependent on external trust assumptions.
    pub external_trust: Option<Facet>,
    /// Not established.
    pub not_established: Option<Facet>,
}

impl Property {
    /// The classes this property is placed in, in the order above.
    #[must_use]
    pub fn facets(&self) -> [Option<&Facet>; 6] {
        [
            self.composition.as_ref(),
            self.statically.as_ref(),
            self.at_run_time.as_ref(),
            self.observable_only.as_ref(),
            self.external_trust.as_ref(),
            self.not_established.as_ref(),
        ]
    }
}

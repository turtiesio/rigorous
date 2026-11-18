//! What a claim is, and the closed set of ways one can be supported.

use rigor_waist::Name;

/// A claim's identifier.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id(pub Name);

/// An identifier from a literal (an invalid one does not compile when used in a constant).
#[must_use]
pub const fn id(s: &'static str) -> Id {
    Id(Name::lit(s))
}

/// An axiom a machine-checked proof depends on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Axiom {
    Propext,
    QuotSound,
}

/// How a claim is supported. Exactly one, by construction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Support {
    /// Machine-checked in Lean, about the mathematical model. Says nothing directly about the code.
    Proved { theorems: &'static [&'static str], axioms: &'static [Axiom] },
    /// The compiler or a denied lint rejects the program: holds for every execution.
    Compiler { how: &'static str },
    /// Exhaustive exploration of a finite instance to a fixed point.
    ModelChecked { instance: &'static str, tests: &'static [&'static str] },
    /// Tests: evidence, not proof.
    Tested { extent: &'static str, tests: &'static [&'static str], shown_by: &'static str },
    /// Measured on external data; reproducible, but not part of the test suite.
    Measured { extent: &'static str, how: &'static str },
    /// Taken on trust from outside the system.
    Assumed { breaks: &'static str },
    /// Not established here.
    Unresolved,
    /// Something that does NOT hold, with the assumptions under which it fails.
    Counterexample { assuming: &'static str, tests: &'static [&'static str], shown_by: &'static str },
}

impl Support {
    /// The tests this support cites by name.
    #[must_use]
    pub const fn tests(&self) -> &'static [&'static str] {
        match self {
            Self::ModelChecked { tests, .. } | Self::Tested { tests, .. } | Self::Counterexample { tests, .. } => tests,
            Self::Proved { .. } | Self::Compiler { .. } | Self::Measured { .. } | Self::Assumed { .. } | Self::Unresolved => &[],
        }
    }

    /// The theorems this support cites by name.
    #[must_use]
    pub const fn theorems(&self) -> &'static [&'static str] {
        match self {
            Self::Proved { theorems, .. } => theorems,
            _ => &[],
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Claim {
    pub id: Id,
    pub statement: &'static str,
    pub support: Support,
}

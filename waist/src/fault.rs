//! How the machinery itself can fail (as opposed to refusing, which is an ordinary event).

use crate::digest::EncodingError;
use crate::name::Name;
use crate::payload::Payload;

/// The fault type of an event type.
pub type FaultOf<E> = Fault<<E as Payload>::Kind, <E as Payload>::Failure>;

/// A fault in the machinery (as opposed to a refusal, which is an ordinary event).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Fault<K, F> {
    /// A unit emitted a kind it did not declare.
    Undeclared {
        unit: Name,
        kind: K,
    },
    /// Two parts would own the same output kind.
    OutputClash {
        kind: K,
        a: Name,
        b: Name,
    },
    DuplicateName(Name),
    /// A non-boundary unit declared that it listens to scoped (internal) events.
    ListensInsideBoundary(Name),
    /// A non-boundary unit declared that it emits scoped (internal) events: only a boundary may.
    EmitsInsideBoundary(Name),
    /// An event of a kind only units may produce was offered as an external stimulus.
    NotAStimulus(K),
    /// The cascade did not reach quiescence within the fuel bound.
    Divergence {
        fuel: u64,
    },
    Encoding(EncodingError),
    /// A unit could not process an event it accepted.
    Unit {
        unit: Name,
        failure: F,
    },
}

impl<K, F> From<EncodingError> for Fault<K, F> {
    fn from(e: EncodingError) -> Self {
        Self::Encoding(e)
    }
}

/// Why a leaf could not process an event it accepted.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LeafError<F> {
    Encoding(EncodingError),
    /// The unit cannot proceed, for one of its enumerated reasons.
    Broken(F),
}

impl<F> From<EncodingError> for LeafError<F> {
    fn from(e: EncodingError) -> Self {
        Self::Encoding(e)
    }
}

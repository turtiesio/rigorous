//! What the waist needs to know about an event type, and nothing more.

use crate::name::Name;
use crate::path::Path;
use std::fmt::Debug;

/// What the kernel needs to know about an event type.
pub trait Payload: Clone + Debug + serde::Serialize + 'static {
    /// The closed set of event kinds: interfaces are sets of these.
    type Kind: Copy + Ord + Debug + 'static;
    /// The enumerated ways a unit can be unable to proceed (as opposed to refusing, which is an event).
    type Failure: Clone + Debug + PartialEq + 'static;
    fn kind(&self) -> Self::Kind;
    /// The marker the runtime issues when a cascade has reached quiescence.
    fn settle() -> Self;
    fn settle_kind() -> Self::Kind;
    /// Wrap an event as internal to the entity `entity`.
    fn scoped(entity: Name, inner: Self) -> Self;
    fn as_scoped(&self) -> Option<(&Name, &Self)>;
    fn scoped_kind() -> Self::Kind;
    /// Mark an event as having come in from outside a federated boundary.
    fn foreign(inner: Self) -> Self;
    fn foreign_kind() -> Self::Kind;
    /// If this event is addressed to the world outside a federated boundary, what it carries.
    fn as_abroad(&self) -> Option<&Self>;
    /// Whether this event may arrive from outside the system, said by this actor. Everything
    /// else can only ever be produced by a unit: the runtime refuses it as a stimulus.
    fn may_come_from(&self, actor: &Path) -> bool;
}

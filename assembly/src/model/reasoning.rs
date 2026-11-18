//! The language responsibility rules are written in: predicates, sorts and constants.

use super::*;
use rigor_logic::datalog::{self, Atom, Fact, Literal, Rule, RuleError, Signature, Sorted, Term};
use serde::{Deserialize, Serialize};

/// Predicates of the responsibility language. The first block is supplied as facts by the
/// adjudicator; the second is derived by rules.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Pred {
    Status,
    Debtor,
    Creditor,
    /// `Parent(child, parent)`: the child commitment was made in service of the parent.
    Parent,
    Cons,
    HasTag,
    Claimed,
    Notified,
    Assessed,
    Kind,
    Principal,
    HasRole,

    Accountable,
    SubFailed,
    HasSubFailure,
    Knew,
    Misreported,
    Culpable,
    Negligent,
    Excused,
    Contributed,
    AtFault,
    IsAi,
    Liable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sort {
    Actor,
    Commitment,
    Prop,
    Status,
    Tag,
    Kind,
    Role,
    Bool,
    Assessment,
}

impl Signature for Pred {
    type Sort = Sort;
    fn signature(self) -> &'static [Sort] {
        use Sort::{Actor, Assessment, Bool, Commitment, Kind, Prop, Role, Status, Tag};
        match self {
            Self::Status => &[Commitment, Status],
            Self::Debtor | Self::Creditor => &[Commitment, Actor],
            Self::Parent | Self::SubFailed => &[Commitment, Commitment],
            Self::Cons => &[Commitment, Prop],
            Self::HasTag => &[Commitment, Tag],
            Self::Claimed => &[Actor, Prop],
            Self::Notified => &[Actor, Prop, Bool],
            Self::Assessed => &[Prop, Assessment],
            Self::Kind => &[Actor, Kind],
            Self::Principal => &[Actor, Actor],
            Self::HasRole => &[Actor, Role],
            Self::Accountable
            | Self::Knew
            | Self::Misreported
            | Self::Culpable
            | Self::Negligent
            | Self::Excused
            | Self::Contributed
            | Self::AtFault
            | Self::Liable => &[Actor, Commitment],
            Self::HasSubFailure => &[Commitment],
            Self::IsAi => &[Actor],
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Const {
    Actor(ActorId),
    Commitment(CommitmentId),
    Prop(Prop),
    Status(Status),
    Tag(Tag),
    Kind(MemberKind),
    Role(Role),
    Bool(bool),
    Assessment(Assessment),
}

impl Sorted<Sort> for Const {
    fn sort(&self) -> Sort {
        match self {
            Self::Actor(_) => Sort::Actor,
            Self::Commitment(_) => Sort::Commitment,
            Self::Prop(_) => Sort::Prop,
            Self::Status(_) => Sort::Status,
            Self::Tag(_) => Sort::Tag,
            Self::Kind(_) => Sort::Kind,
            Self::Role(_) => Sort::Role,
            Self::Bool(_) => Sort::Bool,
            Self::Assessment(_) => Sort::Assessment,
        }
    }
}

pub type RRule = Rule<RuleId, Pred, Const>;
pub type RFact = Fact<Pred, Const>;
pub type RAtom = Atom<Pred, Const>;
pub type RLiteral = Literal<Pred, Const>;
pub type RTerm = Term<Const>;
pub type RProof = datalog::Proof<RuleId, Pred, Const>;
pub type RRuleError = RuleError<RuleId, Pred>;

impl Finding {
    #[must_use]
    pub const fn pred(self) -> Pred {
        match self {
            Self::Accountable => Pred::Accountable,
            Self::Culpable => Pred::Culpable,
            Self::Negligent => Pred::Negligent,
            Self::Excused => Pred::Excused,
            Self::Contributed => Pred::Contributed,
            Self::Liable => Pred::Liable,
        }
    }
    pub const ALL: [Self; 6] = [Self::Accountable, Self::Culpable, Self::Negligent, Self::Excused, Self::Contributed, Self::Liable];
}

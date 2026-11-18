//! What can be claimed, evidenced and assessed. Structured, never parsed.

#[allow(clippy::wildcard_imports)]
use super::*;
use rigor_waist::Name;
use serde::{Deserialize, Serialize};
use std::fmt;

/// A piece of work, possibly one step of a larger one.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Task {
    pub name: TaskName,
    pub step: Option<TaskName>,
}

impl Task {
    #[must_use]
    pub const fn whole(name: TaskName) -> Self {
        Self { name, step: None }
    }
    #[must_use]
    pub fn step(&self, step: TaskName) -> Self {
        Self { name: self.name.clone(), step: Some(step) }
    }
    /// The task this is a step of, if it is a step.
    #[must_use]
    pub fn parent(&self) -> Option<Self> {
        self.step.as_ref().map(|_| Self { name: self.name.clone(), step: None })
    }
}

/// One published version of a package.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Release {
    pub package: Name,
    pub version: (u64, u64, u64),
}

/// Something that can be claimed, evidenced and assessed. Structured, never parsed.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Prop {
    Done(Task),
    Remediated(CommitmentId),
    Audited(CommitmentId),
    Conforms(Requirement),
    /// The change is sound (should stand).
    Sound(ChangeId),
    /// The snapshot with this digest is the state its log really produces.
    Faithful(ContentDigest),
    /// The release is fit to depend on.
    Fit(Release),
    /// The system is able, now, to do the named thing (every need of it is met).
    Can(Name),
    /// The document with this digest adequately answers the named question.
    Answers {
        question: Name,
        answer: ContentDigest,
    },
    /// The change has been applied to the thing it changes.
    Applied(ChangeId),
    /// The named question has an answer that was accepted on review.
    Answered(Name),
    /// The requirement held on that day. A fact about the past: nothing that happens later to
    /// the thing it was about makes it stale.
    HeldOn {
        requirement: Requirement,
        day: u64,
    },
    Fact(Name),
}

impl fmt::Display for Prop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Done(Task { name, step: None }) => write!(f, "done({name})"),
            Self::Done(Task { name, step: Some(s) }) => write!(f, "done({name}.{s})"),
            Self::Remediated(c) => write!(f, "remediated({c})"),
            Self::Audited(c) => write!(f, "audited({c})"),
            Self::Conforms(r) => write!(f, "conforms({r})"),
            Self::Sound(c) => write!(f, "sound({c})"),
            Self::Faithful(d) => write!(f, "faithful({d})"),
            Self::Fit(r) => write!(f, "fit({} {}.{}.{})", r.package, r.version.0, r.version.1, r.version.2),
            Self::Can(n) => write!(f, "can({n})"),
            Self::Answers { question, answer } => write!(f, "answers({question}, {answer})"),
            Self::Applied(c) => write!(f, "applied({c})"),
            Self::Answered(q) => write!(f, "answered({q})"),
            Self::HeldOn { requirement, day } => write!(f, "held({requirement}, day {day})"),
            Self::Fact(n) => write!(f, "fact({n})"),
        }
    }
}

impl Prop {
    /// Whether this proposition may be INFERRED from evidence supporting `premise`. Inference is
    /// only ever along these fixed lines; nothing else follows from anything.
    ///
    /// * a whole task is done, from a step of that same task being done;
    /// * a question is answered, from a document being accepted as answering that question.
    #[must_use]
    pub fn follows_from(&self, premise: &Self) -> bool {
        match (self, premise) {
            (Self::Done(whole), Self::Done(part)) => whole.step.is_none() && part.step.is_some() && whole.name == part.name,
            (Self::Answered(q), Self::Answers { question, .. }) => q == question,
            _ => false,
        }
    }
}

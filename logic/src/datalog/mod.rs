//! Sorted, stratified Datalog with negation and why-provenance.
//!
//! Generic in the predicate type `P` and the constant type `C`. Every predicate has a fixed
//! signature of sorts; [`check`] rejects rules that are ill-sorted, unsafe or not stratifiable,
//! so an accepted program has exactly one model, computable in polynomial time.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;

/// A predicate vocabulary with sorts.
pub trait Signature: Copy + Ord + Debug {
    type Sort: Copy + Eq + Debug + 'static;
    fn signature(self) -> &'static [Self::Sort];
}

/// A constant that knows its sort.
pub trait Sorted<S>: Clone + Ord + Debug {
    fn sort(&self) -> S;
}

mod check;
mod proof;
mod solve;

pub use check::*;
pub use proof::*;
pub use solve::*;

/// A rule variable. Variables are numbered, not named.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Var(pub u8);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Term<C> {
    Var(Var),
    Const(C),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Atom<P, C> {
    pub pred: P,
    pub args: Vec<Term<C>>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Literal<P, C> {
    Pos(Atom<P, C>),
    Neg(Atom<P, C>),
}

impl<P, C> Literal<P, C> {
    pub const fn atom(&self) -> &Atom<P, C> {
        match self {
            Self::Pos(a) | Self::Neg(a) => a,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Rule<I, P, C> {
    pub id: I,
    pub head: Atom<P, C>,
    pub body: Vec<Literal<P, C>>,
}

/// A ground atom.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Fact<P, C> {
    pub pred: P,
    pub args: Vec<C>,
}

/// Why a rule set was rejected.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RuleError<I, P> {
    DuplicateId(I),
    Arity { rule: I, pred: P },
    IllSorted { rule: I, pred: P },
    UnsafeHead { rule: I },
    UnsafeNegation { rule: I, pred: P },
    NotStratifiable,
}

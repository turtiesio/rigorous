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

/// Validate; return the stratum of every predicate that occurs.
///
/// # Errors
/// See [`RuleError`].
pub fn check<I, P, C>(rules: &[Rule<I, P, C>]) -> Result<BTreeMap<P, usize>, RuleError<I, P>>
where
    I: Clone + Ord,
    P: Signature,
    C: Sorted<P::Sort>,
{
    let mut ids = BTreeSet::new();
    let mut strata: BTreeMap<P, usize> = BTreeMap::new();
    for r in rules {
        if !ids.insert(r.id.clone()) {
            return Err(RuleError::DuplicateId(r.id.clone()));
        }
        let mut sorts: BTreeMap<Var, P::Sort> = BTreeMap::new();
        let mut bound: BTreeSet<Var> = BTreeSet::new();
        for (atom, positive) in r.body.iter().map(|l| (l.atom(), matches!(l, Literal::Pos(_)))).chain(std::iter::once((&r.head, false))) {
            let sig = atom.pred.signature();
            if sig.len() != atom.args.len() {
                return Err(RuleError::Arity { rule: r.id.clone(), pred: atom.pred });
            }
            for (t, s) in atom.args.iter().zip(sig) {
                let ok = match t {
                    Term::Const(c) => c.sort() == *s,
                    Term::Var(v) => {
                        if positive {
                            let _ = bound.insert(*v);
                        }
                        *sorts.entry(*v).or_insert(*s) == *s
                    }
                };
                if !ok {
                    return Err(RuleError::IllSorted { rule: r.id.clone(), pred: atom.pred });
                }
            }
            let _ = strata.entry(atom.pred).or_insert(0);
        }
        let vars = |a: &Atom<P, C>| -> Vec<Var> {
            a.args
                .iter()
                .filter_map(|t| match t {
                    Term::Var(v) => Some(*v),
                    Term::Const(_) => None,
                })
                .collect()
        };
        if !vars(&r.head).iter().all(|v| bound.contains(v)) {
            return Err(RuleError::UnsafeHead { rule: r.id.clone() });
        }
        for l in &r.body {
            if let Literal::Neg(a) = l {
                if !vars(a).iter().all(|v| bound.contains(v)) {
                    return Err(RuleError::UnsafeNegation { rule: r.id.clone(), pred: a.pred });
                }
            }
        }
    }
    let limit = strata.len() + 1;
    let mut rounds = 0usize;
    loop {
        let mut changed = false;
        rounds += 1;
        if rounds > limit {
            return Err(RuleError::NotStratifiable);
        }
        for r in rules {
            for l in &r.body {
                let dep = strata.get(&l.atom().pred).copied().unwrap_or(0);
                let need = match l {
                    Literal::Pos(_) => dep,
                    Literal::Neg(_) => dep + 1,
                };
                let h = strata.entry(r.head.pred).or_insert(0);
                if *h < need {
                    *h = need;
                    changed = true;
                }
            }
        }
        if !changed {
            return Ok(strata);
        }
    }
}

/// How a derived fact was obtained.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Why<I, P, C> {
    pub rule: I,
    pub positive: Vec<Fact<P, C>>,
    /// Facts whose ABSENCE the step relied on (closed-world assumptions).
    pub absent: Vec<Fact<P, C>>,
}

/// A derivation tree.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Proof<I, P, C> {
    Given(Fact<P, C>),
    Derived { fact: Fact<P, C>, rule: I, from: Vec<Proof<I, P, C>>, absent: Vec<Fact<P, C>> },
}

#[derive(Clone, Debug)]
pub struct Model<I, P, C> {
    pub facts: BTreeSet<Fact<P, C>>,
    why: BTreeMap<Fact<P, C>, Why<I, P, C>>,
}

impl<I: Clone, P: Copy + Ord, C: Clone + Ord> Model<I, P, C> {
    #[must_use]
    pub fn holds(&self, f: &Fact<P, C>) -> bool {
        self.facts.contains(f)
    }

    pub fn of(&self, pred: P) -> impl Iterator<Item = &Fact<P, C>> {
        self.facts.iter().filter(move |f| f.pred == pred)
    }

    #[must_use]
    pub fn proof(&self, f: &Fact<P, C>) -> Proof<I, P, C> {
        match self.why.get(f) {
            None => Proof::Given(f.clone()),
            Some(w) => {
                Proof::Derived { fact: f.clone(), rule: w.rule.clone(), from: w.positive.iter().map(|p| self.proof(p)).collect(), absent: w.absent.clone() }
            }
        }
    }

    /// Every absent fact some step of the derivation of `f` relied on.
    #[must_use]
    pub fn assumptions(&self, f: &Fact<P, C>) -> BTreeSet<Fact<P, C>> {
        let mut out = BTreeSet::new();
        let mut seen = BTreeSet::new();
        let mut stack = vec![f];
        while let Some(x) = stack.pop() {
            if !seen.insert(x) {
                continue;
            }
            if let Some(w) = self.why.get(x) {
                out.extend(w.absent.iter().cloned());
                stack.extend(w.positive.iter());
            }
        }
        out
    }
}

type Env<C> = BTreeMap<Var, C>;
type Partial<P, C> = Vec<(Env<C>, Vec<Fact<P, C>>)>;
type Derivations<I, P, C> = Vec<(Fact<P, C>, Why<I, P, C>)>;

fn unify<P: Copy + Eq, C: Clone + Eq>(atom: &Atom<P, C>, fact: &Fact<P, C>, env: &Env<C>) -> Option<Env<C>> {
    if atom.pred != fact.pred || atom.args.len() != fact.args.len() {
        return None;
    }
    let mut env = env.clone();
    for (t, v) in atom.args.iter().zip(&fact.args) {
        match t {
            Term::Const(c) if c == v => {}
            Term::Const(_) => return None,
            Term::Var(x) => match env.get(x) {
                Some(bound) if bound == v => {}
                Some(_) => return None,
                None => {
                    let _ = env.insert(*x, v.clone());
                }
            },
        }
    }
    Some(env)
}

fn ground<P: Copy, C: Clone>(atom: &Atom<P, C>, env: &Env<C>) -> Option<Fact<P, C>> {
    let args: Option<Vec<C>> = atom
        .args
        .iter()
        .map(|t| match t {
            Term::Const(c) => Some(c.clone()),
            Term::Var(v) => env.get(v).cloned(),
        })
        .collect();
    args.map(|args| Fact { pred: atom.pred, args })
}

/// Compute the unique model of a (checked) program over the given facts.
///
/// # Errors
/// If the program does not pass [`check`].
pub fn solve<I, P, C>(rules: &[Rule<I, P, C>], facts: impl IntoIterator<Item = Fact<P, C>>) -> Result<Model<I, P, C>, RuleError<I, P>>
where
    I: Clone + Ord,
    P: Signature,
    C: Sorted<P::Sort>,
{
    let strata = check(rules)?;
    let mut db: BTreeSet<Fact<P, C>> = facts.into_iter().collect();
    let mut why: BTreeMap<Fact<P, C>, Why<I, P, C>> = BTreeMap::new();
    let levels: BTreeSet<usize> = strata.values().copied().collect();
    for level in levels {
        let layer: Vec<&Rule<I, P, C>> = rules.iter().filter(|r| strata.get(&r.head.pred) == Some(&level)).collect();
        loop {
            let mut new: Derivations<I, P, C> = Vec::new();
            for r in &layer {
                let mut envs: Partial<P, C> = vec![(Env::new(), Vec::new())];
                for l in &r.body {
                    if let Literal::Pos(atom) = l {
                        let mut next = Vec::new();
                        for (env, used) in &envs {
                            for f in db.iter().filter(|f| f.pred == atom.pred) {
                                if let Some(e2) = unify(atom, f, env) {
                                    let mut u = used.clone();
                                    u.push(f.clone());
                                    next.push((e2, u));
                                }
                            }
                        }
                        envs = next;
                    }
                }
                for (env, used) in envs {
                    let absent: Vec<Fact<P, C>> = r
                        .body
                        .iter()
                        .filter_map(|l| match l {
                            Literal::Neg(a) => ground(a, &env),
                            Literal::Pos(_) => None,
                        })
                        .collect();
                    if absent.iter().any(|f| db.contains(f)) {
                        continue;
                    }
                    if let Some(head) = ground(&r.head, &env) {
                        if !db.contains(&head) {
                            new.push((head, Why { rule: r.id.clone(), positive: used, absent }));
                        }
                    }
                }
            }
            if new.is_empty() {
                break;
            }
            for (head, w) in new {
                if db.insert(head.clone()) {
                    let _ = why.insert(head, w);
                }
            }
        }
    }
    Ok(Model { facts: db, why })
}

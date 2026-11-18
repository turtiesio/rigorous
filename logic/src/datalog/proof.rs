//! What was derived, and why: a model with a proof for every fact.

use super::*;

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
    pub(crate) why: BTreeMap<Fact<P, C>, Why<I, P, C>>,
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

//! Which rule sets have exactly one meaning: sorted, safe and stratifiable.

use super::*;

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

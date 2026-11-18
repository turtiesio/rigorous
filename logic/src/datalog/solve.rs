//! Evaluation, stratum by stratum, to the least model.

use super::*;

pub(crate) type Env<C> = BTreeMap<Var, C>;
pub(crate) type Partial<P, C> = Vec<(Env<C>, Vec<Fact<P, C>>)>;
pub(crate) type Derivations<I, P, C> = Vec<(Fact<P, C>, Why<I, P, C>)>;

pub(crate) fn unify<P: Copy + Eq, C: Clone + Eq>(atom: &Atom<P, C>, fact: &Fact<P, C>, env: &Env<C>) -> Option<Env<C>> {
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

pub(crate) fn ground<P: Copy, C: Clone>(atom: &Atom<P, C>, env: &Env<C>) -> Option<Fact<P, C>> {
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
    let mut by: BTreeMap<P, Vec<Fact<P, C>>> = BTreeMap::new();
    for f in &db {
        by.entry(f.pred).or_default().push(f.clone());
    }
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
                            for f in by.get(&atom.pred).into_iter().flatten() {
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
                    by.entry(head.pred).or_default().push(head.clone());
                    let _ = why.insert(head, w);
                }
            }
        }
    }
    Ok(Model { facts: db, why })
}

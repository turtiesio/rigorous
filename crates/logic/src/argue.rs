//! Dung abstract argumentation under grounded semantics (unique, polynomial).

use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Label {
    In,
    Out,
    Undecided,
}

/// Label every node. Attacks mentioning unknown nodes are ignored. O(V + E log V).
#[must_use]
pub fn grounded<N: Ord + Clone>(nodes: &BTreeSet<N>, attacks: &BTreeSet<(N, N)>) -> BTreeMap<N, Label> {
    let mut attackers: BTreeMap<&N, usize> = nodes.iter().map(|n| (n, 0)).collect();
    let mut targets: BTreeMap<&N, Vec<&N>> = BTreeMap::new();
    for (a, b) in attacks {
        if nodes.contains(a) && nodes.contains(b) {
            if let Some(c) = attackers.get_mut(b) {
                *c += 1;
            }
            targets.entry(a).or_default().push(b);
        }
    }
    let mut label: BTreeMap<N, Label> = BTreeMap::new();
    let mut todo: Vec<&N> = attackers.iter().filter(|(_, c)| **c == 0).map(|(n, _)| *n).collect();
    while let Some(n) = todo.pop() {
        if label.contains_key(n) {
            continue;
        }
        let _ = label.insert(n.clone(), Label::In);
        for m in targets.get(n).into_iter().flatten() {
            if label.contains_key(*m) {
                continue;
            }
            let _ = label.insert((*m).clone(), Label::Out);
            for k in targets.get(*m).into_iter().flatten() {
                if let Some(c) = attackers.get_mut(*k) {
                    *c = c.saturating_sub(1);
                    if *c == 0 && !label.contains_key(*k) {
                        todo.push(*k);
                    }
                }
            }
        }
    }
    for n in nodes {
        let _ = label.entry(n.clone()).or_insert(Label::Undecided);
    }
    label
}

/// The textbook definition: least fixed point of F(S) = { a | S defends a }. For cross-checking.
#[must_use]
pub fn grounded_reference<N: Ord + Clone>(nodes: &BTreeSet<N>, attacks: &BTreeSet<(N, N)>) -> BTreeMap<N, Label> {
    let att: BTreeSet<&(N, N)> = attacks.iter().filter(|(a, b)| nodes.contains(a) && nodes.contains(b)).collect();
    let mut s: BTreeSet<&N> = BTreeSet::new();
    loop {
        let next: BTreeSet<&N> =
            nodes.iter().filter(|a| att.iter().filter(|(_, x)| x == *a).all(|(b, _)| att.iter().any(|(d, t)| t == b && s.contains(d)))).collect();
        if next == s {
            break;
        }
        s = next;
    }
    nodes
        .iter()
        .map(|n| {
            let l = if s.contains(n) {
                Label::In
            } else if att.iter().any(|(a, b)| b == n && s.contains(a)) {
                Label::Out
            } else {
                Label::Undecided
            };
            (n.clone(), l)
        })
        .collect()
}

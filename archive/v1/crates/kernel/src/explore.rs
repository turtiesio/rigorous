//! Bounded explicit-state exploration.
//!
//! A harness, not a boundary: it feeds a unit whatever its neighbours could have produced, so it
//! delivers alphabets without the check that a stimulus is of a kind the outside may send.

use super::base::Digest;
use super::log::Entry;
use super::runtime::Runtime;
use super::unit::{FaultOf, Payload};
use super::Name;
use std::collections::BTreeSet;

/// A sequence of external stimuli.
pub type Trace<E> = Vec<(Name, E)>;

/// Outcome of [`reach`].
#[derive(Debug)]
pub struct Reach<E: Payload, V> {
    pub states: usize,
    pub transitions: usize,
    /// True if a fixed point was reached before the depth bound.
    pub complete: bool,
    /// The first violation found, with the stimuli that lead to it.
    pub violation: Option<(V, Trace<E>)>,
}

/// Breadth-first search over runtime states under an alphabet of external stimuli.
/// `check(before, stimulus, entries, after)` returns a violation value when a property fails.
///
/// # Errors
/// See [`Fault`].
pub fn reach<E: Payload, V>(
    start: &Runtime<E>,
    alphabet: &[(Name, E)],
    depth: usize,
    mut check: impl FnMut(&Runtime<E>, &(Name, E), &[Entry<E>], &Runtime<E>) -> Option<V>,
) -> Result<Reach<E, V>, FaultOf<E>> {
    let mut seen: BTreeSet<Digest> = BTreeSet::new();
    let _ = seen.insert(start.digest());
    let mut frontier: Vec<(Runtime<E>, Trace<E>)> = vec![(start.fork(), Vec::new())];
    let mut transitions = 0usize;
    for _ in 0..depth {
        let mut next = Vec::new();
        for (rt, trace) in &frontier {
            for a in alphabet {
                let mut r = rt.fork();
                let entries = r.inject_unchecked(super::Path::of(&a.0), a.1.clone())?;
                transitions += 1;
                let mut t = trace.clone();
                t.push(a.clone());
                if let Some(msg) = check(rt, a, &entries, &r) {
                    return Ok(Reach { states: seen.len(), transitions, complete: false, violation: Some((msg, t)) });
                }
                if seen.insert(r.digest()) {
                    next.push((r, t));
                }
            }
        }
        frontier = next;
        if frontier.is_empty() {
            break;
        }
    }
    Ok(Reach { states: seen.len(), transitions, complete: frontier.is_empty(), violation: None })
}

/// Run every ordering of `stimuli` from `start`; return the distinct final state digests.
///
/// # Errors
/// See [`Fault`].
pub fn interleavings<E: Payload>(start: &Runtime<E>, stimuli: &[(Name, E)]) -> Result<BTreeSet<Digest>, FaultOf<E>> {
    fn go<E: Payload>(rt: &Runtime<E>, rest: &[(Name, E)], out: &mut BTreeSet<Digest>) -> Result<(), FaultOf<E>> {
        if rest.is_empty() {
            let _ = out.insert(rt.digest());
            return Ok(());
        }
        for (i, s) in rest.iter().enumerate() {
            let mut r = rt.fork();
            let _ = r.inject_unchecked(super::Path::of(&s.0), s.1.clone())?;
            let others: Vec<(Name, E)> = rest.iter().enumerate().filter(|(j, _)| *j != i).map(|(_, x)| x.clone()).collect();
            go(&r, &others, out)?;
        }
        Ok(())
    }
    let mut out = BTreeSet::new();
    go(start, stimuli, &mut out)?;
    Ok(out)
}

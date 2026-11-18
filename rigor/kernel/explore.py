"""Bounded explicit-state exploration (model checking to a depth bound)."""
from __future__ import annotations

from itertools import permutations


def reach(make_rt, alphabet, depth, on_step=None, max_states=500_000):
    """BFS over runtime states under an alphabet of external stimuli.

    on_step(before, action, events, after) -> None | str   (a violation message)
    Returns {'states', 'transitions', 'counterexample', 'violation', 'complete'}.
    `complete` is True if a fixed point was reached before the depth bound.
    """
    rt0 = make_rt()
    seen = {rt0.digest()}
    frontier = [(rt0, ())]
    transitions = 0
    for _ in range(depth):
        nxt = []
        for rt, trace in frontier:
            for a in alphabet:
                r = rt.fork()
                evs = r.inject(*a)
                transitions += 1
                if on_step:
                    bad = on_step(rt, a, evs, r)
                    if bad:
                        return {"states": len(seen), "transitions": transitions,
                                "counterexample": trace + (a,), "violation": bad, "complete": False}
                h = r.digest()
                if h not in seen:
                    seen.add(h)
                    nxt.append((r, trace + (a,)))
                    if len(seen) > max_states:
                        raise MemoryError("state bound exceeded")
        frontier = nxt
        if not frontier:
            break
    return {"states": len(seen), "transitions": transitions, "counterexample": None,
            "violation": None, "complete": not frontier}


def interleavings(make_rt, stimuli):
    """Run every ordering of `stimuli`; return {final state digest: one ordering}."""
    finals = {}
    for perm in permutations(range(len(stimuli))):
        rt = make_rt()
        for i in perm:
            rt.inject(*stimuli[i])
        finals.setdefault(rt.digest(), perm)
    return finals

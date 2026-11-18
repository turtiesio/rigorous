"""Impact analysis: what does a change touch?

  cone(rt, change)   static over-approximation, by walking dependency indices
                     (evidence -> propositions -> commitments -> families ->
                     verdicts -> derived commitments).  No execution.
  what_if(rt, ...)   exact: fork the world, apply the stimulus, diff judgments.

Soundness (tested, not proved): every judgment that what_if reports as changed
lies inside the cone of the change.
"""
from __future__ import annotations

from .norms.epistemic import component
from .norms.institution import valid


def judgments(rt) -> dict:
    s, j = rt.state, {}
    for p, st in s["epi"]["status"].items():
        j[("prop", p)] = st["status"]
    for cid, c in s["norms"]["c"].items():
        j[("commitment", cid)] = c["status"]
    for cid, f in s["adj"]["verdicts"].items():
        if f:
            j[("verdict", cid)] = tuple(map(tuple, f))
    for cid in s["inst"]["caps"]:
        j[("cap", cid)] = valid(s["inst"], cid)
    return j


def what_if(rt, kind, payload, actor="env") -> dict:
    before = judgments(rt)
    fork = rt.fork()
    fork.inject(kind, payload, actor)
    after = judgments(fork)
    return {k: (before.get(k), after.get(k)) for k in set(before) | set(after) if before.get(k) != after.get(k)}


def _commitment_closure(s, cids) -> set:
    norms, adj = s["norms"], s["adj"]
    out, stack = set(), list(cids)
    while stack:
        c = stack.pop()
        if c in out or c not in norms["c"]:
            continue
        out.add(c)
        root = c
        while adj["c"].get(root, {}).get("parent") in adj["c"]:
            root = adj["c"][root]["parent"]
        fam, st = [], [root]
        while st:
            x = st.pop()
            fam.append(x)
            st.extend(adj["kids"].get(x, {}))
        stack.extend(fam)
        stack.extend(norms["derived"].get(c, {}).values())
    return out


def cone(rt, change) -> set:
    """change: ('evidence', id) | ('prop', p) | ('artifact', name) | ('cap', id) | ('rules', top-level key)"""
    s = rt.state
    kind, x = change
    epi = s["epi"]
    out: set = set()
    if kind in ("evidence", "prop", "artifact") or (kind == "rules" and x == "evidence"):
        if kind == "evidence":
            seeds, props = [x], set()
        elif kind == "prop":
            seeds, props = list(epi["by_prop"].get(x, {})), {x}
        elif kind == "artifact":
            seeds, props = list(epi["by_art"].get(x, {})), set()
        else:
            seeds, props = list(epi["ev"]), set(epi["status"])
        comp = component(epi, seeds)
        props |= {epi["ev"][e]["prop"] for e in comp if epi["ev"][e]["prop"]}
        out |= {("evidence", e) for e in comp} | {("prop", p) for p in props}
        cids = {c for p in props for c in s["norms"]["by_prop"].get(p, {})}
    elif kind == "rules" and x == "responsibility":
        cids = set(s["norms"]["c"])
    elif kind == "cap":
        kids, grew = {x}, True
        while grew:
            more = {c["id"] for c in s["inst"]["caps"].values() if c["parent"] in kids} - kids
            kids |= more
            grew = bool(more)
        return {("cap", c) for c in kids}
    else:
        cids = set()
    closure = _commitment_closure(s, cids)
    out |= {("commitment", c) for c in closure} | {("verdict", c) for c in closure}
    return out

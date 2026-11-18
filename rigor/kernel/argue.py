"""Dung abstract argumentation, grounded semantics (unique, polynomial)."""
from __future__ import annotations


def grounded(nodes, attacks) -> dict:
    """Label every node 'in' | 'out' | 'undec'.  O(V+E)."""
    nodes = list(nodes)
    att = {n: set() for n in nodes}   # attackers of n
    tgt = {n: set() for n in nodes}   # nodes n attacks
    for a, b in attacks:
        if a in att and b in att:
            att[b].add(a)
            tgt[a].add(b)
    live = {n: len(att[n]) for n in nodes}   # attackers not yet 'out'
    label: dict = {}
    todo = [n for n in nodes if live[n] == 0]
    while todo:
        n = todo.pop()
        if n in label:
            continue
        label[n] = "in"
        for m in tgt[n]:
            if m in label:
                continue
            label[m] = "out"
            for k in tgt[m]:
                live[k] -= 1
                if live[k] == 0 and k not in label:
                    todo.append(k)
    for n in nodes:
        label.setdefault(n, "undec")
    return label


def grounded_reference(nodes, attacks) -> dict:
    """Textbook definition: least fixed point of F(S) = {a | S defends a}."""
    nodes = list(nodes)
    attacks = {(a, b) for a, b in attacks if a in nodes and b in nodes}
    s: set = set()
    while True:
        new = {a for a in nodes
               if all(any((d, b) in attacks for d in s) for (b, x) in attacks if x == a)}
        if new == s:
            break
        s = new
    out = {b for (a, b) in attacks if a in s}
    return {n: "in" if n in s else "out" if n in out else "undec" for n in nodes}

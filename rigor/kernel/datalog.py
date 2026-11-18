"""Stratified Datalog with negation and why-provenance.

Rules are plain JSON data so they can be carried in events, hashed, versioned
and amended:   {"id": "r1", "head": ["p", "?X"], "body": [["q", "?X"], ["not", "r", "?X"]]}
Variables start with '?'.  `check` rejects unsafe or unstratifiable programs,
so every accepted rule set has a unique model computable in polynomial time.
"""
from __future__ import annotations


class RuleError(Exception):
    pass


def is_var(t) -> bool:
    return isinstance(t, str) and t.startswith("?")


def _split(lit):
    return (True, lit[1:]) if lit and lit[0] == "not" else (False, lit)


def check(rules) -> dict:
    """Validate; return {predicate: stratum}."""
    if not isinstance(rules, list):
        raise RuleError("rules must be a list")
    ids = set()
    strat: dict = {}
    for r in rules:
        if not isinstance(r, dict) or not {"id", "head", "body"} <= set(r):
            raise RuleError(f"malformed rule: {r!r}")
        if r["id"] in ids:
            raise RuleError(f"duplicate rule id {r['id']}")
        ids.add(r["id"])
        head, body = r["head"], r["body"]
        if not head or not all(isinstance(t, str) for t in head) or head[0] == "not":
            raise RuleError(f"{r['id']}: bad head")
        bound = set()
        for lit in body:
            neg, atom = _split(lit)
            if not atom or not all(isinstance(t, str) for t in atom):
                raise RuleError(f"{r['id']}: bad literal {lit!r}")
            if not neg:
                bound |= {t for t in atom[1:] if is_var(t)}
            strat.setdefault(atom[0], 0)
        for lit in body:
            neg, atom = _split(lit)
            if neg and not {t for t in atom[1:] if is_var(t)} <= bound:
                raise RuleError(f"{r['id']}: unsafe negation {lit!r}")
        if not {t for t in head[1:] if is_var(t)} <= bound:
            raise RuleError(f"{r['id']}: unsafe head variable")
        strat.setdefault(head[0], 0)
    changed, rounds = True, 0
    while changed:
        changed = False
        rounds += 1
        if rounds > len(strat) + 1:
            raise RuleError("not stratifiable (recursion through negation)")
        for r in rules:
            h = r["head"][0]
            for lit in r["body"]:
                neg, atom = _split(lit)
                need = strat[atom[0]] + (1 if neg else 0)
                if strat[h] < need:
                    strat[h] = need
                    changed = True
    return strat


class Model:
    def __init__(self, facts, why):
        self.facts, self.why = facts, why

    def __contains__(self, f):
        return tuple(f) in self.facts

    def query(self, pred):
        return sorted(f for f in self.facts if f[0] == pred)

    def proof(self, fact):
        """Nested [fact, rule_id, [subproofs], [negated atoms]]; base facts have rule 'fact'."""
        fact = tuple(fact)
        w = self.why.get(fact)
        if w is None:
            return [list(fact), "fact", [], []]
        rid, pos, neg = w
        return [list(fact), rid, [self.proof(p) for p in pos], [list(n) for n in neg]]

    def assumptions(self, fact) -> set:
        """Atoms whose ABSENCE the derivation relies on (closed-world steps)."""
        out, stack, seen = set(), [tuple(fact)], set()
        while stack:
            f = stack.pop()
            if f in seen or f not in self.why:
                continue
            seen.add(f)
            _, pos, neg = self.why[f]
            out |= set(neg)
            stack.extend(pos)
        return out


def _match(atom, fact, env):
    if len(atom) != len(fact) or atom[0] != fact[0]:
        return None
    env = dict(env)
    for t, v in zip(atom[1:], fact[1:]):
        if is_var(t):
            if env.setdefault(t, v) != v:
                return None
        elif t != v:
            return None
    return env


def solve(rules, facts) -> Model:
    strat = check(rules)
    db: set = {tuple(f) for f in facts}
    by: dict = {}
    for f in db:
        by.setdefault(f[0], set()).add(f)
    why: dict = {}
    for level in sorted(set(strat.values())):
        layer = [r for r in rules if strat[r["head"][0]] == level]
        while True:
            new = []
            for r in layer:
                pos = [a for n, a in map(_split, r["body"]) if not n]
                neg = [a for n, a in map(_split, r["body"]) if n]
                envs = [({}, [])]
                for atom in pos:
                    envs = [(e2, used + [f]) for e, used in envs for f in by.get(atom[0], ())
                            if (e2 := _match(atom, f, e)) is not None]
                for env, used in envs:
                    negs = [tuple(env.get(t, t) for t in a) for a in neg]
                    if any(n in db for n in negs):
                        continue
                    head = tuple(env.get(t, t) for t in r["head"])
                    if head not in db:
                        new.append((head, (r["id"], used, negs)))
            if not new:
                break
            for head, w in new:
                if head not in db:
                    db.add(head)
                    by.setdefault(head[0], set()).add(head)
                    why[head] = w
    return Model(db, why)

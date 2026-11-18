"""Claims, evidence, provenance and their revisable assessment.

A proposition is never "true" here.  It has an ASSESSMENT -- accepted, refuted,
contested or unsupported -- computed from the admissible evidence by grounded
argumentation under the evidence rules in force.  New evidence, retraction,
a changed artifact or a changed rule all revise the assessment.

  support(e,P) vs rebut(e',P): attack each other; the strictly stronger method wins
  undercut(e, e'):             e attacks the evidence e' itself (always succeeds)
  derived_from:                any successful attack on a parent hits the child
  stale:                       evidence bound to an artifact digest that changed
  self-support:                support rooted in the claimant's own word is inadmissible

Reassessment is local: only the connected component of the changed evidence is
recomputed (grounded semantics decomposes over connected components).
"""
from __future__ import annotations

from ..kernel.argue import grounded
from ..kernel.event import clone, digest
from ..kernel.unit import Leaf

DEFAULT = {"status": "unsupported", "since": None, "basis": []}


def admissible(s, eid, memo=None) -> bool:
    memo = {} if memo is None else memo
    if eid in memo:
        return memo[eid]
    e, rules = s["ev"][eid], s["rules"]
    trusted = rules["trusted"].get(e["method"], [])
    ok = (not e["retracted"]
          and e["method"] in rules["ranks"]
          and (trusted == "*" or e["source"] in trusted)
          and all(s["art"].get(a) == d for a, d in e["artifact"].items())
          and all(admissible(s, p, memo) for p in e["derived_from"]))
    if ok and e["pol"] == "+" and e["prop"] and not rules.get("self_support"):
        ok = not (roots(s, eid) & set(s["claims"].get(e["prop"], {})))
    memo[eid] = ok
    return ok


def roots(s, eid) -> set:
    e = s["ev"][eid]
    if not e["derived_from"]:
        return {e["source"]}
    return set().union(*(roots(s, p) for p in e["derived_from"]))


def rank(s, eid) -> int:
    e = s["ev"][eid]
    return min([s["rules"]["ranks"][e["method"]]] + [rank(s, p) for p in e["derived_from"]])


def descendants(s, eid) -> set:
    out, stack = set(), [eid]
    while stack:
        for k in s["kids"].get(stack.pop(), {}):
            if k not in out:
                out.add(k)
                stack.append(k)
    return out


def component(s, seeds) -> set:
    out, stack = set(), [x for x in seeds if x in s["ev"]]
    while stack:
        x = stack.pop()
        if x in out:
            continue
        out.add(x)
        e = s["ev"][x]
        nb = list(e["derived_from"]) + list(s["kids"].get(x, {})) + list(s["cutters"].get(x, {}))
        if e["target"]:
            nb.append(e["target"])
        if e["prop"]:
            nb += list(s["by_prop"].get(e["prop"], {}))
        stack.extend(n for n in nb if n not in out)
    return out


def label(s, eids) -> dict:
    memo: dict = {}
    adm = sorted(e for e in eids if admissible(s, e, memo))
    aset = set(adm)
    rk = {e: rank(s, e) for e in adm}
    direct = []
    by_prop: dict = {}
    for e in adm:
        ev = s["ev"][e]
        if ev["prop"]:
            by_prop.setdefault(ev["prop"], {"+": [], "-": []})[ev["pol"]].append(e)
        elif ev["target"] in aset:
            direct.append((e, ev["target"]))
    for g in by_prop.values():
        for a in g["+"]:
            for b in g["-"]:
                if rk[b] <= rk[a]:
                    direct.append((a, b))
                if rk[a] <= rk[b]:
                    direct.append((b, a))
    attacks = set(direct)
    for a, b in direct:
        attacks |= {(a, d) for d in descendants(s, b) if d in aset}
    return grounded(adm, attacks)


def prop_status(s, prop, labels) -> dict:
    es = s["by_prop"].get(prop, {})
    sup = sorted(e for e in es if labels.get(e) == "in" and s["ev"][e]["pol"] == "+")
    reb = sorted(e for e in es if labels.get(e) == "in" and s["ev"][e]["pol"] == "-")
    if reb:
        return {"status": "refuted", "since": None, "basis": reb}
    if sup:
        return {"status": "accepted", "since": min(s["ev"][e]["at"] for e in sup), "basis": sup}
    if any(labels.get(e) == "undec" for e in es):
        return {"status": "contested", "since": None, "basis": []}
    return dict(DEFAULT)


def full_status(s) -> dict:
    """From-scratch assessment of every proposition (reference for the incremental path)."""
    labels = label(s, list(s["ev"]))
    props = set(s["by_prop"]) | set(s["status"])
    return {p: prop_status(s, p, labels) for p in props}


class Epistemic(Leaf):
    name = "epi"
    inputs = ("tick", "rules.changed", "ok.claim", "ok.attest", "ok.retract", "artifact.changed")
    outputs = ("prop.status", "ev.*")

    def init(self):
        return {"now": 0, "version": 0, "rules": {"ranks": {}, "trusted": {}, "self_support": False},
                "claims": {}, "ev": {}, "by_prop": {}, "kids": {}, "cutters": {}, "art": {}, "by_art": {},
                "status": {}}

    work = 0   # instrumentation only (not state): evidence nodes re-labelled so far

    def _reassess(self, s, eids, extra=()):
        self.work += len(eids)
        labels = label(s, eids)
        props = {s["ev"][e]["prop"] for e in eids if s["ev"][e]["prop"]} | set(extra)
        for p in sorted(props):
            new, old = prop_status(s, p, labels), s["status"].get(p, DEFAULT)
            if new != old:
                s["status"][p] = new
                yield "prop.status", {"prop": p, **new, "was": old["status"], "rules_version": s["version"]}

    def on(self, s, ev):
        p = ev.payload
        if ev.kind == "tick":
            s["now"] = max(s["now"], p["t"])
        elif ev.kind == "rules.changed":
            s["rules"], s["version"] = clone(p["rules"]["evidence"]), p["version"]
            yield from self._reassess(s, list(s["ev"]), list(s["status"]))
        elif ev.kind == "ok.claim":
            prop = p.get("prop")
            if isinstance(prop, str) and p["by"] not in s["claims"].setdefault(prop, {}):
                s["claims"][prop][p["by"]] = s["now"]
                yield from self._reassess(s, component(s, s["by_prop"].get(prop, {})), [prop])
        elif ev.kind == "artifact.changed":
            if s["art"].get(p["name"]) != p["digest"]:
                s["art"][p["name"]] = p["digest"]
                yield from self._reassess(s, component(s, s["by_art"].get(p["name"], {})))
        elif ev.kind == "ok.retract":
            e = s["ev"].get(p.get("evidence"))
            if e is None or e["source"] != p["by"]:
                yield "ev.rejected", {"by": p["by"], "reason": "only the source may retract its evidence"}
            elif not e["retracted"]:
                e["retracted"] = True
                yield "ev.retracted", {"id": e["id"], "by": p["by"]}
                yield from self._reassess(s, component(s, [e["id"]]))
        elif ev.kind == "ok.attest":
            prop, target = p.get("prop"), p.get("target")
            parents = sorted(set(p.get("derived_from") or []))
            method = p.get("method")
            bad = None
            if (prop is None) == (target is None):
                bad = "exactly one of prop/target required"
            elif target is not None and target not in s["ev"]:
                bad = "unknown target evidence"
            elif any(x not in s["ev"] for x in parents):
                bad = "unknown provenance"
            elif method == "inference" and not parents:
                bad = "an inference must cite what it is derived from"
            elif not isinstance(method, str):
                bad = "method required"
            if bad:
                yield "ev.rejected", {"by": p["by"], "reason": bad}
                return
            e = {"source": p["by"], "prop": prop, "target": target, "pol": "-" if target else p.get("pol", "+"),
                 "method": method, "derived_from": parents, "artifact": clone(p.get("artifact") or {}),
                 "content": p.get("content"), "at": s["now"], "retracted": False}
            e["id"] = "e-" + digest(e, 10)
            if e["id"] in s["ev"]:
                yield "ev.rejected", {"by": p["by"], "reason": "duplicate evidence"}
                return
            s["ev"][e["id"]] = e
            if prop:
                s["by_prop"].setdefault(prop, {})[e["id"]] = 1
            if target:
                s["cutters"].setdefault(target, {})[e["id"]] = 1
            for x in parents:
                s["kids"].setdefault(x, {})[e["id"]] = 1
            for a in e["artifact"]:
                s["by_art"].setdefault(a, {})[e["id"]] = 1
            yield "ev.recorded", {k: e[k] for k in ("id", "source", "prop", "target", "pol", "method", "derived_from", "at")}
            yield from self._reassess(s, component(s, [e["id"]]))

"""Directed conditional commitments -- the single normative primitive.

  C(debtor, creditor, antecedent, consequent, deadline)

A condition is either {"prop": P} (holds while P is ASSESSED accepted -- hence
revisable) or {"event": {"kind","match"}} (holds once a matching institutional
event has occurred -- irrevocable).

The status is not stored truth but a FUNCTION of the current facts:

  released | cancelled | void          explicit operations (absorbing)
  satisfied    consequent held by the deadline
  violated     antecedent held, deadline passed (or debtor repudiated)
  expired      antecedent never held by the deadline
  detached     antecedent holds: an unconditional obligation is in force
  conditional  otherwise

So when a claim is refuted, `satisfied` falls back to `detached`/`violated` by
re-evaluation, and everything that was derived from a violation that no longer
stands is voided.  Violations and verdicts instantiate secondary commitments
from rule templates (contrary-to-duty obligations: remediation, refund, audit).
"""
from __future__ import annotations

import re

from ..kernel.event import clone, digest
from ..kernel.unit import Leaf

STATUSES = {"conditional", "detached", "satisfied", "violated", "expired", "released", "cancelled", "void"}
ABSORBING = {"released", "cancelled", "void"}


class Unresolved(Exception):
    pass


def subst(x, env):
    if isinstance(x, dict):
        return {k: subst(v, env) for k, v in x.items()}
    if isinstance(x, list):
        return [subst(v, env) for v in x]
    if not isinstance(x, str) or "$" not in x:
        return x

    def look(name):
        if name not in env or env[name] is None:
            raise Unresolved(name)
        return env[name]
    m = re.fullmatch(r"\$([\w.:]+)", x)
    if m:
        return look(m.group(1))
    return re.sub(r"\$([\w.:]+)", lambda m: str(look(m.group(1))), x)


def cond_ok(c) -> bool:
    if not isinstance(c, dict) or len(c) != 1:
        return False
    if "prop" in c:
        return isinstance(c["prop"], str)
    e = c.get("event")
    return isinstance(e, dict) and isinstance(e.get("kind"), str) and isinstance(e.get("match", {}), dict)


def ev_match(m, payload) -> bool:
    for k, v in m.items():
        if isinstance(v, dict) and "ge" in v:
            got = payload.get(k)
            if not isinstance(got, int) or got < v["ge"]:
                return False
        elif payload.get(k) != v:
            return False
    return True


def evaluate(s, c) -> str:
    if c["op"] in ("released", "cancelled", "void"):
        return c["op"]

    def when(which):
        cond = c[which]
        if cond is None:
            return c["created_at"]
        if "prop" in cond:
            st = s["prop"].get(cond["prop"])
            return st["since"] if st and st["status"] == "accepted" else None
        return c[which + "_at"]
    ct, at = when("cons"), when("ant")
    if ct is not None and ct <= c["deadline"]:
        return "satisfied"
    if c["op"] == "breach":
        return "violated"
    if at is None or at > c["deadline"]:
        return "expired" if s["now"] > c["deadline"] else "conditional"
    return "violated" if s["now"] > c["deadline"] else "detached"


class Norms(Leaf):
    name = "norms"
    outputs = ("norm.*",)

    def __init__(self, watch=("res.transferred",)):
        self.watch = tuple(watch)
        self.inputs = ("inst.member", "tick", "rules.changed", "ok.commit", "ok.release", "ok.cancel",
                       "ok.assign", "prop.status", "verdict") + self.watch

    def init(self):
        return {"now": 0, "version": 0, "rules": {"on_violation": [], "on_verdict": []},
                "c": {}, "prop": {}, "by_prop": {}, "watch": {}, "due": {}, "roles": {}, "derived": {}}

    # -- helpers (append to `out`) -------------------------------------------------
    def _create(self, s, out, spec, source, basis=None, by=None):
        why = None
        if not cond_ok(spec.get("cons")) or not (spec.get("ant") is None or cond_ok(spec["ant"])):
            why = "malformed condition"
        elif not isinstance(spec.get("deadline"), int) or spec["deadline"] < s["now"]:
            why = "deadline missing or in the past"
        elif not isinstance(spec.get("debtor"), str) or not isinstance(spec.get("creditor"), str):
            why = "debtor and creditor required"
        elif spec.get("parent") is not None and spec["parent"] not in s["c"]:
            why = "unknown parent commitment"
        if why:
            out.append(("norm.rejected", {"by": by, "reason": why}))
            return None
        c = {"debtor": spec["debtor"], "creditor": spec["creditor"], "ant": clone(spec.get("ant")),
             "cons": clone(spec["cons"]), "deadline": spec["deadline"], "tags": sorted(spec.get("tags") or []),
             "meta": clone(spec.get("meta") or {}), "parent": spec.get("parent"), "basis": basis}
        cid = "c-" + digest(c, 10)
        if cid in s["c"]:
            out.append(("norm.rejected", {"by": by, "reason": "duplicate commitment", "id": cid}))
            return None
        public = {"id": cid, **c, "source": source, "rules_version": s["version"]}
        c.update(id=cid, source=source, rules_version=s["version"], op=None, status=None,
                 created_at=s["now"], ant_at=None, cons_at=None)
        s["c"][cid] = c
        for cond in (c["ant"], c["cons"]):
            if cond and "prop" in cond:
                s["by_prop"].setdefault(cond["prop"], {})[cid] = 1
            elif cond:
                s["watch"][cid] = 1
        s["due"].setdefault(str(c["deadline"]), {})[cid] = 1
        out.append(("norm.created", public))
        self._refresh(s, out, [cid])
        return cid

    def _env(self, s, c, extra=None):
        env = {"debtor": c["debtor"], "creditor": c["creditor"], "id": c["id"], "now": s["now"],
               "prop": (c["cons"] or {}).get("prop")}
        env.update({"meta." + k: v for k, v in c["meta"].items()})
        env.update({"role:" + r: m for r, m in s["roles"].items()})
        env.update(extra or {})
        return env

    def _derive(self, s, out, parent, key, template, env, basis):
        """Ensure the secondary commitment `key` of `parent` exists and is not void."""
        have = s["derived"].setdefault(parent["id"], {})
        if key in have:
            child = s["c"][have[key]]
            if child["op"] == "void":
                child["op"] = None
                self._refresh(s, out, [child["id"]])
            return
        try:
            spec = subst(template, env)
        except Unresolved as u:
            out.append(("norm.skipped", {"of": parent["id"], "key": key, "reason": f"unresolved ${u}"}))
            return
        d = spec.get("deadline")
        if isinstance(d, str) and d.startswith("+"):
            spec["deadline"] = s["now"] + int(d[1:])
        cid = self._create(s, out, spec, "rule", {**basis, "of": parent["id"], "key": key})
        if cid:
            have[key] = cid

    def _void(self, s, out, parent_id, keep=lambda key: False):
        for key, cid in sorted(s["derived"].get(parent_id, {}).items()):
            child = s["c"][cid]
            if not keep(key) and child["op"] is None:
                child["op"] = "void"
                self._refresh(s, out, [cid])

    def _refresh(self, s, out, ids):
        for cid in sorted(ids):
            c = s["c"][cid]
            new, old = evaluate(s, c), c["status"]
            if new == old:
                continue
            c["status"] = new
            out.append(("norm.status", {"id": cid, "was": old, "status": new, "at": s["now"],
                                        "debtor": c["debtor"], "creditor": c["creditor"], "tags": c["tags"]}))
            if new == "violated":
                for i, t in enumerate(s["rules"]["on_violation"]):
                    if t["when"].get("tag") in c["tags"]:
                        self._derive(s, out, c, f"v:{i}", t["create"], self._env(s, c), {"kind": "violation"})
            elif old == "violated":
                self._void(s, out, cid)          # the violation no longer stands: so do its consequences

    # -- transition function -------------------------------------------------------
    def on(self, s, ev):
        out: list = []
        p, k = ev.payload, ev.kind
        if k == "inst.member":
            if p.get("role"):
                s["roles"][p["role"]] = p["id"]
        elif k == "rules.changed":
            s["rules"], s["version"] = clone(p["rules"]["norms"]), p["version"]
        elif k == "tick":
            s["now"] = max(s["now"], p["t"])
            due = [d for d in s["due"] if int(d) < s["now"]]
            ids = [cid for d in due for cid in s["due"].pop(d)]
            self._refresh(s, out, ids)
        elif k == "ok.commit":
            self._create(s, out, {**p, "debtor": p["by"]}, "act", by=p["by"])
        elif k in ("ok.release", "ok.cancel", "ok.assign"):
            c = s["c"].get(p.get("id"))
            if c is None or c["status"] in ABSORBING:
                out.append(("norm.rejected", {"by": p["by"], "reason": "no such live commitment"}))
            elif k == "ok.release" and p["by"] == c["creditor"]:
                c["op"] = "released"
            elif k == "ok.cancel" and p["by"] == c["debtor"]:
                c["op"] = "cancelled" if c["status"] == "conditional" else "breach"
            elif k == "ok.assign" and p["by"] == c["creditor"] and isinstance(p.get("to"), str):
                c["creditor"] = p["to"]
                out.append(("norm.assigned", {"id": c["id"], "creditor": p["to"], "by": p["by"]}))
            else:
                out.append(("norm.rejected", {"by": p["by"], "reason": f"{k[3:]} not within the actor's power"}))
            if c is not None:
                self._refresh(s, out, [c["id"]])
        elif k == "prop.status":
            s["prop"][p["prop"]] = {"status": p["status"], "since": p["since"]}
            self._refresh(s, out, list(s["by_prop"].get(p["prop"], {})))
        elif k == "verdict":
            c = s["c"].get(p["commitment"])
            if c is not None:
                want = {}
                for i, t in enumerate(s["rules"]["on_verdict"]):
                    for finding, party in p["findings"]:
                        if t["when"].get("finding") == finding and t["when"]["tag"] in c["tags"]:
                            want[f"j:{i}:{finding}:{party}"] = (t, finding, party)
                self._void(s, out, c["id"], keep=lambda key: not key.startswith("j:") or key in want)
                for key, (t, finding, party) in sorted(want.items()):
                    self._derive(s, out, c, key, t["create"], self._env(s, c, {"party": party}),
                                 {"kind": "verdict", "finding": finding, "party": party})
        if k in self.watch:
            hit = []
            for cid in list(s["watch"]):
                c = s["c"][cid]
                for which in ("ant", "cons"):
                    cond = c[which]
                    if cond and "event" in cond and c[which + "_at"] is None \
                            and cond["event"]["kind"] == k and ev_match(cond["event"].get("match", {}), p):
                        c[which + "_at"] = s["now"]
                        hit.append(cid)
                if all(not (c[w] and "event" in c[w]) or c[w + "_at"] is not None for w in ("ant", "cons")):
                    del s["watch"][cid]
            self._refresh(s, out, hit)
        return out

"""Governance: the rules are data, changed only by the rules' own procedure.

  propose{patch} -> approvals >= quorum -> rules.changed{version+1}

Every proposal is statically validated before it can be enacted:
  * entrenched paths are immutable (the trust anchor of self-amendment),
  * responsibility rules must be safe and stratified (unique model, PTIME),
  * violation/verdict templates must form an acyclic tag graph
    (so chains of secondary obligations are finite),
  * the quorum must stay >= 1.
The quorum and the entrenchment list are themselves rules.
"""
from __future__ import annotations

from ..kernel import datalog
from ..kernel.event import clone, digest
from ..kernel.unit import Leaf


class InvalidRules(Exception):
    pass


def validate_rules(r) -> None:
    try:
        ev, no, re_, go = r["evidence"], r["norms"], r["responsibility"], r["governance"]
        if not all(isinstance(v, int) for v in ev["ranks"].values()) or not isinstance(ev["trusted"], dict):
            raise InvalidRules("evidence: ranks must be integers, trusted a map")
        datalog.check(re_["rules"])
        if not isinstance(re_["findings"], list):
            raise InvalidRules("responsibility.findings must be a list")
        if not isinstance(go["quorum"], int) or go["quorum"] < 1:
            raise InvalidRules("quorum must be >= 1")
        edges: dict = {}
        for t in no["on_violation"] + no["on_verdict"]:
            edges.setdefault(t["when"]["tag"], set()).update(t["create"].get("tags", []))
            if "cons" not in t["create"] or "deadline" not in t["create"]:
                raise InvalidRules("template needs cons and deadline")
    except datalog.RuleError as e:
        raise InvalidRules(f"responsibility rules: {e}") from None
    except (KeyError, TypeError, AttributeError) as e:
        raise InvalidRules(f"malformed rule set: {e!r}") from None
    seen, done = set(), set()

    def visit(tag):
        if tag in done:
            return
        if tag in seen:
            raise InvalidRules(f"template cycle through tag {tag!r}: obligations could regress forever")
        seen.add(tag)
        for nxt in edges.get(tag, ()):
            visit(nxt)
        done.add(tag)
    for tag in list(edges):
        visit(tag)


def apply_patch(rules, patch):
    """patch: {"dotted.path": value}; a path ending in '+' appends to a list."""
    new = clone(rules)
    for path, value in sorted(patch.items()):
        append = path.endswith("+")
        keys = path.rstrip("+").split(".")
        d = new
        for k in keys[:-1]:
            d = d[k]
        if append:
            d[keys[-1]] = d[keys[-1]] + clone(value)
        else:
            d[keys[-1]] = clone(value)
    return new


class Governance(Leaf):
    name = "gov"
    inputs = ("genesis", "tick", "ok.propose", "ok.approve")
    outputs = ("rules.changed", "gov.*")

    def init(self):
        return {"now": 0, "version": 0, "rules": None, "history": [], "proposals": {}}

    def _enact(self, s, new, changed, by):
        prev = digest(s["rules"]) if s["rules"] else None
        s["rules"], s["version"] = new, s["version"] + 1
        s["history"].append({"version": s["version"], "digest": digest(new), "at": s["now"], "by": by})
        return "rules.changed", {"version": s["version"], "rules": clone(new), "digest": digest(new),
                                 "prev": prev, "changed": changed, "by": by}

    def on(self, s, ev):
        p = ev.payload
        if ev.kind == "tick":
            s["now"] = max(s["now"], p["t"])
        elif ev.kind == "genesis":
            if s["rules"] is None:
                validate_rules(p["rules"])          # an invalid constitution is not a constitution
                yield self._enact(s, clone(p["rules"]), ["*"], ["constitution"])
        elif ev.kind == "ok.propose":
            patch = p.get("patch")
            try:
                if not isinstance(patch, dict) or not patch:
                    raise InvalidRules("empty patch")
                for path in patch:
                    q = path.rstrip("+")
                    for e in s["rules"]["governance"].get("entrenched", []):
                        if q == e or q.startswith(e + ".") or e.startswith(q + "."):
                            raise InvalidRules(f"{e} is entrenched")
                validate_rules(apply_patch(s["rules"], patch))
            except InvalidRules as e:
                yield "gov.rejected", {"by": p["by"], "reason": str(e)}
                return
            except (KeyError, TypeError) as e:
                yield "gov.rejected", {"by": p["by"], "reason": f"bad patch path: {e!r}"}
                return
            pid = "p-" + digest([s["version"], patch], 10)
            if pid in s["proposals"]:
                yield "gov.rejected", {"by": p["by"], "reason": "duplicate proposal"}
                return
            s["proposals"][pid] = {"patch": clone(patch), "base": s["version"], "approvals": {}, "status": "open"}
            yield "gov.proposed", {"id": pid, "by": p["by"], "base": s["version"], "paths": sorted(patch)}
            yield from self._approve(s, pid, p["by"])
        elif ev.kind == "ok.approve":
            yield from self._approve(s, p.get("id"), p["by"])

    def _approve(self, s, pid, by):
        pr = s["proposals"].get(pid)
        if pr is None or pr["status"] != "open":
            yield "gov.rejected", {"by": by, "reason": "no such open proposal"}
            return
        if pr["base"] != s["version"]:
            pr["status"] = "stale"
            yield "gov.rejected", {"by": by, "reason": "proposal was made against an older rule version"}
            return
        pr["approvals"][by] = 1
        yield "gov.approved", {"id": pid, "by": by, "approvals": sorted(pr["approvals"])}
        if len(pr["approvals"]) >= s["rules"]["governance"]["quorum"]:
            pr["status"] = "enacted"
            yield self._enact(s, apply_patch(s["rules"], pr["patch"]), sorted(pr["patch"]), sorted(pr["approvals"]))

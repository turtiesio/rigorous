"""Responsibility: findings derived by explicit, versioned rules -- with proofs.

The adjudicator holds no opinions.  It projects the events it has seen into
facts, evaluates the responsibility rules in force (stratified Datalog), and
publishes for each violated commitment the findings, the derivation of each
finding, and the closed-world assumptions that derivation relied on.

Work is local: only the family (delegation tree) of a touched commitment is
re-adjudicated, and only once the cascade that touched it has settled.
"""
from __future__ import annotations

from ..kernel import datalog
from ..kernel.event import clone
from ..kernel.unit import Leaf

STANDING_ASSUMPTIONS = [
    "A-AUTH: actors are who the log says they are (authentication is external)",
    "A-OBS: 'notified'/'claimed' are records of messages, not of what anyone actually knew",
    "A-ASSESS: 'violated' rests on the current assessment of evidence, which is revisable",
]


class Adjudicator(Leaf):
    name = "adj"
    inputs = ("inst.member", "rules.changed", "norm.created", "norm.status", "norm.assigned",
              "ok.claim", "prop.status", "signal", "settle")
    outputs = ("verdict",)
    work = 0   # instrumentation only: families re-adjudicated

    def init(self):
        return {"version": 0, "rules": {"rules": [], "findings": [], "retroactive": True}, "versions": {},
                "members": {}, "c": {}, "kids": {}, "claims": {}, "notified": {}, "prop": {}, "by_prop": {},
                "verdicts": {}, "arose": {}, "dirty": {}}

    def _root(self, s, cid):
        while s["c"][cid]["parent"] in s["c"]:
            cid = s["c"][cid]["parent"]
        return cid

    def _family(self, s, root):
        out, stack = [], [root]
        while stack:
            x = stack.pop()
            out.append(x)
            stack.extend(s["kids"].get(x, {}))
        return sorted(out)

    def facts(self, s, fam):
        f = []
        for m, info in s["members"].items():
            for key in ("kind", "principal", "role"):
                if info.get(key):
                    f.append((key, m, info[key]))
        for cid in fam:
            c = s["c"][cid]
            f += [("debtor", cid, c["debtor"]), ("creditor", cid, c["creditor"]), ("status", cid, str(c["status"]))]
            f += [("tag", cid, t) for t in c["tags"]]
            if c["parent"] in s["c"]:
                f.append(("parent", cid, c["parent"]))
            if c["prop"]:
                p = c["prop"]
                f.append(("cons", cid, p))
                f.append(("prop", p, s["prop"].get(p, "unsupported")))
                f += [("claimed", x, p) for x in s["claims"].get(p, {})]
                f += [("notified", x, p, v) for x, v in s["notified"].get(p, {}).items()]
        return f

    def _adjudicate(self, s, root):
        self.work += 1
        fam = self._family(s, root)
        ver = s["version"] if s["rules"].get("retroactive", True) else s["arose"].get(root, s["version"])
        rules = s["versions"].get(str(ver), s["rules"])
        model = datalog.solve(rules["rules"], self.facts(s, fam))
        for cid in fam:
            hits = [f for f in model.facts if len(f) == 3 and f[0] in rules["findings"] and f[2] == cid] \
                if s["c"][cid]["status"] == "violated" else []
            findings = sorted([f[0], f[1]] for f in hits)
            if findings == s["verdicts"].get(cid, []):
                continue
            s["verdicts"][cid] = findings
            cwa = sorted({"CWA: no record that " + _show(a) for f in hits for a in model.assumptions(f)})
            yield "verdict", {"commitment": cid, "findings": findings, "rules_version": ver,
                              "proofs": {_show(f): model.proof(f) for f in sorted(hits)},
                              "assumptions": STANDING_ASSUMPTIONS + cwa}

    def on(self, s, ev):
        p, k = ev.payload, ev.kind
        touched: set = set()
        props = []
        if k == "inst.member":
            s["members"][p["id"]] = {x: p.get(x) for x in ("kind", "principal", "role")}
        elif k == "rules.changed":
            s["rules"], s["version"] = clone(p["rules"]["responsibility"]), p["version"]
            s["versions"][str(p["version"])] = s["rules"]
            touched = {self._root(s, c) for c in s["c"]}
        elif k == "norm.created":
            prop = (p["cons"] or {}).get("prop")
            s["c"][p["id"]] = {"debtor": p["debtor"], "creditor": p["creditor"], "parent": p["parent"],
                               "prop": prop, "tags": p["tags"], "status": None}
            if p["parent"]:
                s["kids"].setdefault(p["parent"], {})[p["id"]] = 1
            if prop:
                s["by_prop"].setdefault(prop, {})[p["id"]] = 1
        elif k == "norm.status":
            s["c"][p["id"]]["status"] = p["status"]
            root = self._root(s, p["id"])
            if p["status"] == "violated":
                s["arose"].setdefault(root, s["version"])
            touched = {root}
        elif k == "norm.assigned":
            s["c"][p["id"]]["creditor"] = p["creditor"]
            touched = {self._root(s, p["id"])}
        elif k == "ok.claim":
            if isinstance(p.get("prop"), str):
                s["claims"].setdefault(p["prop"], {})[p["by"]] = 1
                props = [p["prop"]]
        elif k == "prop.status":
            s["prop"][p["prop"]] = p["status"]
            props = [p["prop"]]
        elif k == "signal":
            if isinstance(p.get("prop"), str) and isinstance(p.get("to"), str):
                s["notified"].setdefault(p["prop"], {})[p["to"]] = "true" if p.get("value") else "false"
                props = [p["prop"]]
        for prop in props:
            touched |= {self._root(s, c) for c in s["by_prop"].get(prop, {})}
        for root in touched:
            s["dirty"][root] = 1
        if k == "settle":                       # judge only quiescent states (no glitches)
            for root in sorted(s["dirty"]):
                yield from self._adjudicate(s, root)
            s["dirty"] = {}


def _show(atom) -> str:
    return f"{atom[0]}({', '.join(atom[1:])})"

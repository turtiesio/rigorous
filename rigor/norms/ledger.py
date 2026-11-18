"""Resources: a ledger over a commutative monoid (N per resource type).

Invariant (conservation): for every resource type, the sum of balances equals
the amount minted at genesis; balances never go negative.
"""
from __future__ import annotations

from ..kernel.event import clone
from ..kernel.unit import Leaf


def conserved(s) -> bool:
    tot: dict = {}
    for h, rs in s["bal"].items():
        for r, n in rs.items():
            if n < 0:
                return False
            tot[r] = tot.get(r, 0) + n
    return all(tot.get(r, 0) == n for r, n in s["minted"].items()) and set(tot) <= set(s["minted"])


class Ledger(Leaf):
    name = "ledger"
    inputs = ("genesis", "ok.transfer")
    outputs = ("res.*",)

    def init(self):
        return {"bal": {}, "minted": {}, "genesis": False}

    def on(self, s, ev):
        p = ev.payload
        if ev.kind == "genesis":
            if s["genesis"]:
                return
            s["genesis"] = True
            for m in p["members"]:
                s["bal"][m] = {}
            for h, rs in clone(p.get("balances", {})).items():
                for r, n in rs.items():
                    s["bal"].setdefault(h, {})[r] = n
                    s["minted"][r] = s["minted"].get(r, 0) + n
        elif ev.kind == "ok.transfer":
            f, t, r, n = p.get("from"), p.get("to"), p.get("rtype"), p.get("amount")
            info = {"from": f, "to": t, "rtype": r, "amount": n, "by": p["by"], "via": p["via"], "memo": p.get("memo")}
            if f not in s["bal"] or t not in s["bal"]:
                yield "res.failed", {**info, "reason": "unknown account"}
            elif s["bal"][f].get(r, 0) < n:
                yield "res.failed", {**info, "reason": "insufficient funds"}
            else:
                s["bal"][f][r] -= n
                s["bal"][t][r] = s["bal"][t].get(r, 0) + n
                yield "res.transferred", info

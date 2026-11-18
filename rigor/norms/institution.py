"""Authority: the count-as gate.

An actor's *attempt* is a brute event  do{action,args,cap}.  It COUNTS AS the
institutional event  ok.<action>  iff the actor holds a capability whose
delegation chain is rooted in the constitution (genesis), is unrevoked and
unexpired along its whole length, covers the action and scope, and has budget.
Otherwise the attempt is recorded as `denied`.  Other institutional units only
ever listen to ok.* -- whose sole emitter (checked by Par) is this unit.

Delegation must attenuate (actions subset, scope superset, limits <=, expiry <=),
so a chain can never carry more authority than its root (proved: proofs/Rigor.lean).
"""
from __future__ import annotations

from ..kernel.event import clone, digest
from ..kernel.unit import Leaf

BASE = ["claim", "attest", "retract", "commit", "release", "cancel", "assign", "request", "revoke"]


def chain(s, cid):
    """Return ([cap leaf..root], None) or (None, reason)."""
    out = []
    while cid is not None:
        c = s["caps"].get(cid)
        if c is None:
            return None, f"unknown cap {cid}"
        if c["revoked"]:
            return None, f"revoked: {cid}"
        if c["expires"] is not None and s["now"] > c["expires"]:
            return None, f"expired: {cid}"
        out.append(c)
        cid = c["parent"]
    return out, None


def valid(s, cid) -> bool:
    return chain(s, cid)[1] is None


def attenuates(child, parent) -> str | None:
    if not set(child["actions"]) <= set(parent["actions"]):
        return "actions exceed parent"
    if any(child["scope"].get(k) != v for k, v in parent["scope"].items()):
        return "scope wider than parent"
    for r, lim in parent["limits"].items():
        if r not in child["limits"] or child["limits"][r] > lim:
            return f"limit on {r} exceeds parent"
    if parent["expires"] is not None and (child["expires"] is None or child["expires"] > parent["expires"]):
        return "expiry exceeds parent"
    return None


def audit_caps(s) -> list:
    """Invariant check: every delegated cap attenuates its parent; spending within limits."""
    bad = []
    for c in s["caps"].values():
        if c["parent"] is not None:
            why = attenuates(c, s["caps"][c["parent"]])
            if why:
                bad.append((c["id"], why))
        for r, lim in c["limits"].items():
            if s["spent"].get(c["id"], {}).get(r, 0) > lim:
                bad.append((c["id"], f"overspent {r}"))
    return bad


class Institution(Leaf):
    name = "inst"
    inputs = ("genesis", "tick", "do", "res.failed")
    outputs = ("ok.*", "denied", "inst.*")

    def init(self):
        return {"now": 0, "genesis": None, "members": {}, "caps": {}, "spent": {}}

    @staticmethod
    def _root(s, cid, holder, actions, scope=None, limits=None, delegable=False):
        s["caps"][cid] = {"id": cid, "issuer": "constitution", "holder": holder, "actions": sorted(actions),
                          "scope": scope or {}, "limits": limits or {}, "expires": None,
                          "delegable": delegable, "parent": None, "revoked": False}

    def on(self, s, ev):
        p = ev.payload
        if ev.kind == "genesis":
            if s["genesis"]:
                return
            s["genesis"] = ev.id
            for m, info in sorted(p["members"].items()):
                s["members"][m] = clone(info)
                self._root(s, "base:" + m, m, BASE)
                self._root(s, "own:" + m, m, ["transfer"], {"from": m}, delegable=True)
                yield "inst.member", {"id": m, **info}
            for c in p.get("caps", []):
                self._root(s, c["id"], c["holder"], c["actions"], clone(c.get("scope", {})),
                           clone(c.get("limits", {})), c.get("delegable", False))
        elif ev.kind == "tick":
            s["now"] = max(s["now"], p["t"])
        elif ev.kind == "res.failed":
            ch, _ = chain(s, p.get("via"))
            for c in ch or []:
                sp = s["spent"].get(c["id"], {})
                if p["rtype"] in sp:
                    sp[p["rtype"]] -= p["amount"]
        elif ev.kind == "do":
            yield self._do(s, ev.actor, p)

    def _do(self, s, actor, p):
        action, args = p.get("action"), clone(p.get("args") or {})
        cid = p.get("cap") or "base:" + actor

        def deny(reason):
            return "denied", {"by": actor, "action": action, "args": args, "via": cid, "reason": reason}

        if actor not in s["members"]:
            return deny("not a member")
        if not isinstance(action, str) or "by" in args or "via" in args:
            return deny("malformed")
        ch, why = chain(s, cid)
        if why:
            return deny(why)
        cap = ch[0]
        if cap["holder"] != actor:
            return deny("cap not held by actor")

        if action == "delegate":
            if not cap["delegable"]:
                return deny("cap not delegable")
            if args.get("to") not in s["members"]:
                return deny("delegatee not a member")
            new = {"issuer": actor, "holder": args["to"], "actions": sorted(args.get("actions") or []),
                   "scope": args.get("scope") or {}, "limits": args.get("limits") or {},
                   "expires": args.get("expires"), "delegable": bool(args.get("delegable")),
                   "parent": cid, "revoked": False}
            if not new["actions"]:
                return deny("empty delegation")
            why = attenuates(new, cap)
            if why:
                return deny("not an attenuation: " + why)
            new["id"] = "cap-" + digest(new, 10)
            if new["id"] in s["caps"]:
                return deny("duplicate delegation")
            s["caps"][new["id"]] = new
            return "ok.delegate", {"id": new["id"], "to": new["holder"], "actions": new["actions"],
                                   "scope": new["scope"], "limits": new["limits"], "expires": new["expires"],
                                   "delegable": new["delegable"], "parent": cid, "by": actor, "via": cid}

        if action not in cap["actions"]:
            return deny("action not granted")
        if any(args.get(k) != v for k, v in cap["scope"].items()):
            return deny("out of scope")

        if action == "revoke":
            target = s["caps"].get(args.get("target"))
            if target is None:
                return deny("unknown target")
            anc, c = [], target
            while c is not None:
                anc.append(c)
                c = s["caps"].get(c["parent"])
            if not any(a["issuer"] == actor for a in anc):
                return deny("only an issuer on the chain may revoke")
            target["revoked"] = True
            return "ok.revoke", {"target": target["id"], "by": actor, "via": cid}

        if action == "transfer":
            amt, r = args.get("amount"), args.get("rtype")
            if not isinstance(amt, int) or isinstance(amt, bool) or amt <= 0 or not isinstance(r, str):
                return deny("malformed transfer")
            for c in ch:
                lim = c["limits"].get(r)
                if lim is not None and s["spent"].get(c["id"], {}).get(r, 0) + amt > lim:
                    return deny(f"budget exceeded on {c['id']}")
            for c in ch:
                if r in c["limits"]:
                    sp = s["spent"].setdefault(c["id"], {})
                    sp[r] = sp.get(r, 0) + amt
        return "ok." + action, {**args, "by": actor, "via": cid}

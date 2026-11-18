"""Synthetic worlds for scaling measurements and locality tests."""
from __future__ import annotations

from .kernel.unit import Runtime
from .norms.org import DEFAULT_RULES, make_org

MEMBERS = {"boss": {"kind": "human"}, "w": {"kind": "human"}, "deployer": {"kind": "service"},
           "monitor": {"kind": "service"}, "auditor": {"kind": "human", "role": "auditor"}}


def world_n(n: int, chain: int = 1) -> Runtime:
    """n independent tasks; each is a delegation chain of `chain` commitments
    (t{i}.0 <- t{i}.1 <- ...), every link supported by its own receipt."""
    rt = Runtime(make_org())
    rt.inject("genesis", {"members": MEMBERS, "rules": DEFAULT_RULES, "balances": {"boss": {"credit": 10}}},
              "constitution")
    for i in range(n):
        parent = None
        for d in range(chain):
            p = f"done:t{i}.{d}"
            evs = rt.do("w", "commit", creditor="boss", cons={"prop": p}, deadline=5, tags=["task"], parent=parent)
            parent = next(e.payload["id"] for e in evs if e.kind == "norm.created")
            rt.do("deployer", "attest", prop=p, pol="+", method="receipt", content=f"r{i}.{d}")
            rt.do("w", "claim", prop=p)
    rt.tick(6)
    return rt


def refute(rt: Runtime, i: int, d: int = 0):
    return rt.do("monitor", "attest", prop=f"done:t{i}.{d}", pol="-", method="probe", content="probe")

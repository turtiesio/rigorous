"""An organisation = the six institutional units composed in parallel.

`make_org` is used unchanged for a team, a company, or a company nested inside
another (wrap it in an Entity): the same semantics at every scale.
"""
from __future__ import annotations

from ..kernel.unit import Par
from .adjudicator import Adjudicator
from .commitments import Norms
from .epistemic import Epistemic
from .governance import Governance
from .institution import Institution
from .ledger import Ledger


def make_org(*members):
    return Par(Institution(), Ledger(), Epistemic(), Norms(), Adjudicator(), Governance(), *members)


def R(rid, head, *body):
    return {"id": rid, "head": head, "body": [list(b) for b in body]}


# Responsibility rules, version 1.  Read "?X is <finding> for commitment ?C".
RESPONSIBILITY_V1 = [
    R("acc",   ["accountable", "?X", "?C"], ["status", "?C", "violated"], ["debtor", "?C", "?X"]),
    R("sub",   ["sub_failed", "?C", "?D"], ["parent", "?D", "?C"], ["status", "?D", "violated"]),
    R("hsub",  ["has_sub_failure", "?C"], ["sub_failed", "?C", "?D"]),
    R("knew",  ["knew", "?X", "?C"], ["debtor", "?C", "?X"], ["sub_failed", "?C", "?D"],
               ["cons", "?D", "?Q"], ["notified", "?X", "?Q", "false"]),
    R("mis",   ["misreported", "?X", "?C"], ["status", "?C", "violated"], ["debtor", "?C", "?X"],
               ["cons", "?C", "?P"], ["claimed", "?X", "?P"]),
    R("culp",  ["culpable", "?X", "?C"], ["misreported", "?X", "?C"], ["knew", "?X", "?C"]),
    R("exc",   ["excused", "?X", "?C"], ["accountable", "?X", "?C"], ["sub_failed", "?C", "?D"],
               ["not", "misreported", "?X", "?C"]),
    R("contr", ["contributed", "?Y", "?C"], ["sub_failed", "?C", "?D"], ["debtor", "?D", "?Y"]),
    R("flt1",  ["at_fault", "?X", "?C"], ["culpable", "?X", "?C"]),
    R("flt2",  ["at_fault", "?X", "?C"], ["accountable", "?X", "?C"], ["not", "has_sub_failure", "?C"]),
    R("ai",    ["is_ai", "?X"], ["kind", "?X", "ai"]),
    # An AI agent can be at fault but cannot bear liability: it lifts to its principal.
    R("liab1", ["liable", "?P", "?C"], ["at_fault", "?X", "?C"], ["kind", "?X", "ai"], ["principal", "?X", "?P"]),
    R("liab2", ["liable", "?X", "?C"], ["at_fault", "?X", "?C"], ["not", "is_ai", "?X"]),
]

# Amendment used in the demo: a duty to verify before reporting success.
RESPONSIBILITY_V2_EXTRA = [
    R("negl",  ["negligent", "?X", "?C"], ["misreported", "?X", "?C"], ["not", "knew", "?X", "?C"]),
    R("flt3",  ["at_fault", "?X", "?C"], ["negligent", "?X", "?C"]),
]

DEFAULT_RULES = {
    "evidence": {
        "ranks": {"receipt": 1, "ack": 2, "test": 2, "probe": 3, "audit": 4, "inference": 9},
        "trusted": {"receipt": ["deployer"], "probe": ["monitor"], "test": ["ci"], "audit": ["auditor"],
                    "ack": ["alice"], "inference": "*"},
        "self_support": False,
    },
    "norms": {
        "on_violation": [
            {"when": {"tag": "task"},
             "create": {"debtor": "$role:auditor", "creditor": "$creditor", "cons": {"prop": "audited:$id"},
                        "deadline": "+10", "tags": ["audit"]}},
        ],
        "on_verdict": [
            {"when": {"finding": "liable", "tag": "task"},
             "create": {"debtor": "$party", "creditor": "$creditor", "cons": {"prop": "remediated:$id"},
                        "deadline": "+5", "tags": ["remediation"]}},
            {"when": {"finding": "liable", "tag": "paid"},
             "create": {"debtor": "$party", "creditor": "$meta.payer",
                        "cons": {"event": {"kind": "res.transferred",
                                           "match": {"from": "$party", "to": "$meta.payer", "rtype": "credit",
                                                     "amount": {"ge": "$meta.price"}}}},
                        "deadline": "+5", "tags": ["refund"]}},
        ],
    },
    "responsibility": {
        "rules": RESPONSIBILITY_V1,
        "findings": ["accountable", "culpable", "negligent", "excused", "contributed", "liable"],
        "retroactive": True,
    },
    "governance": {"quorum": 2, "entrenched": ["governance.entrenched", "evidence.self_support"]},
}

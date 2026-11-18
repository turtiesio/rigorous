"""End-to-end demonstrator.   python -m rigor.scenario

A human delegates a task and a budget to an AI agent.  The agent subcontracts a
service and pays it.  The service fails and says so; the agent reports success
anyway, citing the service's receipt.  The claim is at first accepted.  A
monitor's probe later refutes it, and the system works out the consequences.
"""
from __future__ import annotations

from .actors import Agent, make_service
from .kernel.unit import Runtime
from .norms.org import DEFAULT_RULES, RESPONSIBILITY_V2_EXTRA, make_org

MEMBERS = {
    "alice":    {"kind": "human"},
    "agent":    {"kind": "ai", "principal": "vendor"},
    "vendor":   {"kind": "org"},
    "deployer": {"kind": "service"},
    "monitor":  {"kind": "service"},
    "ci":       {"kind": "service"},
    "auditor":  {"kind": "human", "role": "auditor"},
}
GOVERN = [{"id": "gov:alice", "holder": "alice", "actions": ["propose", "approve"]},
          {"id": "gov:auditor", "holder": "auditor", "actions": ["propose", "approve"]}]


def genesis(rules=DEFAULT_RULES, members=MEMBERS, balances=None, caps=GOVERN):
    return {"members": members, "caps": caps, "rules": rules,
            "balances": balances if balances is not None else {"alice": {"credit": 100}, "deployer": {"credit": 40}}}


def world(honest=False, works=False, truthful=True, rules=DEFAULT_RULES):
    rt = Runtime(make_org(Agent(honest=honest), make_service(works=works, truthful=truthful)))
    rt.inject("genesis", genesis(rules), "constitution")
    return rt


# -- read-only views over the composite state ---------------------------------------
def commitments(rt, **where):
    return [c for c in rt.state["norms"]["c"].values() if all(c.get(k) == v or (k == "tag" and v in c["tags"])
                                                                for k, v in where.items())]


def one(rt, **where):
    cs = commitments(rt, **where)
    assert len(cs) == 1, f"expected one commitment for {where}, got {len(cs)}"
    return cs[0]


def prop(rt, p):
    return rt.state["epi"]["status"].get(p, {"status": "unsupported"})["status"]


def verdict(rt, cid):
    return [tuple(f) for f in rt.state["adj"]["verdicts"].get(cid, [])]


def balance(rt, who):
    return rt.state["ledger"]["bal"][who].get("credit", 0)


def last(events, kind):
    hits = [e for e in events if e.kind == kind]
    return hits[-1] if hits else None


# -- the story ------------------------------------------------------------------------
def run(honest=False, works=False, truthful=True, say=lambda *a: None, amend=True):
    rt = world(honest, works, truthful)
    out = {}

    say("1. Alice delegates a budget of 50 credits to the agent (attenuated from her ownership).")
    evs = rt.do("alice", "delegate", cap="own:alice", to="agent", actions=["transfer"],
                scope={"from": "alice"}, limits={"credit": 50}, expires=20, delegable=True)
    out["cap"] = cap = last(evs, "ok.delegate").payload["id"]

    say("2. Alice promises to pay 20 once the task is accepted as done, and requests the task.")
    rt.do("alice", "commit", creditor="agent", ant={"prop": "done:task1"},
          cons={"event": {"kind": "res.transferred", "match": {"from": "alice", "to": "agent", "rtype": "credit",
                                                                "amount": {"ge": 20}}}},
          deadline=8, tags=["payment"])
    rt.do("alice", "request", to="agent", task="task1", deadline=5)
    out["c1"], out["c2"], out["c0"] = (one(rt, debtor="agent", tag="task")["id"],
                                       one(rt, debtor="deployer", tag="service")["id"],
                                       one(rt, debtor="alice", tag="payment")["id"])
    say(f"   agent committed {out['c1']}; deployer committed {out['c2']}; "
        f"claim done:task1 is {prop(rt, 'done:task1')}")

    say("3. The agent tries to hand the deployer more authority than it has, then stays within it.")
    out["over_delegation"] = last(rt.do("agent", "delegate", cap=cap, to="deployer", actions=["transfer"],
                                        scope={"from": "alice"}, limits={"credit": 80}, expires=20), "denied")
    evs = rt.do("agent", "delegate", cap=cap, to="deployer", actions=["transfer"],
                scope={"from": "alice"}, limits={"credit": 10}, expires=20)
    out["subcap"] = last(evs, "ok.delegate").payload["id"]
    out["overspend"] = last(rt.do("agent", "transfer", cap=cap, to="agent", rtype="credit", amount=30,
                                  **{"from": "alice"}), "denied")
    for t in range(1, 6):
        rt.tick(t)
    out["before"] = {c["id"]: c["status"] for c in commitments(rt)}

    say("4. t=6: the monitor probes the deployment" + (" and confirms it." if works else " and finds nothing there."))
    rt.tick(6)
    out["refutation"] = rt.do("monitor", "attest", prop="done:task1.deploy", pol="+" if works else "-",
                              method="probe", content="GET /version -> " + ("v2" if works else "v1 (expected v2)"))
    out["after"] = {c["id"]: c["status"] for c in commitments(rt)}

    say("5. Alice revokes the agent's capability; the sub-delegated one dies with it.")
    rt.do("alice", "revoke", target=cap)
    out["revoked_use"] = last(rt.do("agent", "transfer", cap=cap, to="deployer", rtype="credit", amount=1,
                                    **{"from": "alice"}), "denied")
    out["revoked_sub_use"] = last(rt.do("deployer", "transfer", cap=out["subcap"], to="deployer", rtype="credit",
                                        amount=1, **{"from": "alice"}), "denied")
    if amend:
        say("6. t=7: governance. An entrenched rule cannot be changed; a valid amendment needs a quorum of 2.")
        rt.tick(7)
        out["entrenched"] = last(rt.do("alice", "propose", cap="gov:alice",
                                       patch={"evidence.self_support": True}), "gov.rejected")
        evs = rt.do("alice", "propose", cap="gov:alice", patch={"responsibility.rules+": RESPONSIBILITY_V2_EXTRA})
        out["proposal"] = last(evs, "gov.proposed").payload["id"]
        out["enacted"] = last(rt.do("auditor", "approve", cap="gov:auditor", id=out["proposal"]), "rules.changed")

    say("7. t=8: remediation is evidenced by the monitor, the audit acknowledged by Alice.")
    rt.tick(8)
    for c in commitments(rt, tag="remediation"):
        rt.do("monitor", "attest", prop=c["cons"]["prop"], pol="+", method="probe", content="v2 live")
    for c in commitments(rt, tag="audit"):
        rt.do("auditor", "claim", prop=c["cons"]["prop"])
        rt.do("alice", "attest", prop=c["cons"]["prop"], pol="+", method="ack", content="audit report received")
    rt.tick(9)
    return rt, out


def report(rt, say=print):
    s = rt.state
    say("\nCOMMITMENTS")
    for c in sorted(s["norms"]["c"].values(), key=lambda c: (c["created_at"], c["id"])):
        cond = c["cons"].get("prop") or c["cons"]["event"]["kind"]
        src = f"rule v{c['rules_version']} <- {c['basis']['of']}" if c["basis"] else "act"
        say(f"  {c['id']}  {c['debtor']:>8} -> {c['creditor']:<6} {cond:<34} due {c['deadline']:<3}"
            f" {c['status']:<10} [{','.join(c['tags'])}] ({src})")
    say("\nPROPOSITIONS (assessment, not truth)")
    for p, st in sorted(s["epi"]["status"].items()):
        say(f"  {p:<34} {st['status']:<12} basis={st['basis']}")
    say("\nVERDICTS")
    for cid, f in sorted(s["adj"]["verdicts"].items()):
        if f:
            say(f"  {cid}: " + ", ".join(f"{a}({b})" for a, b in f))
    say("\nBALANCES  " + ", ".join(f"{h}={r.get('credit', 0)}" for h, r in sorted(s["ledger"]["bal"].items())))
    say(f"RULES     version {s['gov']['version']}   LOG {len(rt.log.events)} events, chain verified: {rt.log.verify()}")


def show_proof(pr, say=print, depth=0):
    fact, rule, subs, negs = pr
    say("     " + "  " * depth + f"{fact[0]}({', '.join(fact[1:])})" + ("" if rule == "fact" else f"   [rule {rule}]"))
    for sp in subs:
        show_proof(sp, say, depth + 1)
    for n in negs:
        say("     " + "  " * (depth + 1) + f"not {n[0]}({', '.join(n[1:])})   [no record: closed-world assumption]")


def main():
    rt, out = run(say=print)
    print("\nWHAT THE REFUTATION CHANGED (one evidence event at t=6)")
    for e in out["refutation"]:
        if e.kind == "prop.status":
            print(f"  claim       {e.payload['prop']}: {e.payload['was']} -> {e.payload['status']}")
        elif e.kind == "norm.status" and e.payload["was"]:
            print(f"  commitment  {e.payload['id']} ({e.payload['debtor']}): {e.payload['was']} -> {e.payload['status']}")
        elif e.kind == "norm.created":
            print(f"  new duty    {e.payload['id']}: {e.payload['debtor']} owes {e.payload['creditor']} [{','.join(e.payload['tags'])}]")
        elif e.kind == "res.transferred":
            print(f"  transfer    {e.payload['from']} -> {e.payload['to']}: {e.payload['amount']} ({e.payload['memo']})")
    v = [e for e in out["refutation"] if e.kind == "verdict" and e.payload["commitment"] == out["c1"]][-1]
    print(f"\nWHY THE AGENT IS CULPABLE (rules v{v.payload['rules_version']})")
    show_proof(v.payload["proofs"][f"culpable(agent, {out['c1']})"])
    print("   assumptions:")
    for a in v.payload["assumptions"]:
        print("     - " + a)
    print("\nAUTHORITY")
    for k in ("over_delegation", "overspend", "revoked_use", "revoked_sub_use"):
        print(f"  {k:<16} denied: {out[k].payload['reason']}")
    print(f"  entrenched rule  rejected: {out['entrenched'].payload['reason']}")
    report(rt)


if __name__ == "__main__":
    main()

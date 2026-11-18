"""Randomised invariants (seeded, reproducible) and bounded model checks."""
import random
import unittest

from rigor.kernel.explore import interleavings, reach
from rigor.kernel.unit import Par, Runtime
from rigor.norms.commitments import ABSORBING, STATUSES, Norms
from rigor.norms.epistemic import DEFAULT, Epistemic, full_status
from rigor.norms.institution import audit_caps, chain
from rigor.norms.ledger import conserved
from rigor.norms.org import DEFAULT_RULES, make_org
from rigor.scenario import genesis, world
from rigor.synth import refute, world_n


class TestAuthorityAndResources(unittest.TestCase):
    def test_random_delegation_trees_never_amplify_authority(self):
        """Whatever members attempt: every cap attenuates its parent, no chain overspends any
        ancestor's limit, a revoked subtree never acts again, and credit is conserved."""
        who = ["alice", "agent", "vendor", "deployer", "monitor"]
        for seed in range(25):
            rnd = random.Random(seed)
            rt = Runtime(make_org())
            rt.inject("genesis", genesis(balances={m: {"credit": 60} for m in who}), "constitution")
            inst = rt.state["inst"]
            dead: set = set()
            for step in range(120):
                caps = sorted(inst["caps"])
                a, cid = rnd.choice(who), rnd.choice(caps)
                r = rnd.random()
                if r < 0.35:
                    evs = rt.do(a, "delegate", cap=cid, to=rnd.choice(who), actions=["transfer"],
                                scope={"from": rnd.choice(who)}, limits={"credit": rnd.randrange(1, 80)},
                                expires=rnd.choice([None, 5, 50]), delegable=rnd.random() < 0.8)
                elif r < 0.85:
                    evs = rt.do(a, "transfer", cap=cid, to=rnd.choice(who), rtype="credit",
                                amount=rnd.randrange(-2, 40), **{"from": rnd.choice(who)})
                elif r < 0.95:
                    evs = rt.do(a, "revoke", target=cid)
                    for e in evs:
                        if e.kind == "ok.revoke":
                            dead.add(e.payload["target"])
                else:
                    evs = rt.tick(inst["now"] + 1)
                for e in evs:
                    if e.kind.startswith("ok.") and e.kind != "ok.revoke":
                        lineage, c = [], inst["caps"][e.payload["via"]]
                        while c:
                            lineage.append(c["id"])
                            c = inst["caps"].get(c["parent"])
                        self.assertFalse(dead & set(lineage), f"seed {seed}: revoked authority was exercised")
                        self.assertEqual(inst["caps"][e.payload["via"]]["holder"], e.payload["by"])
                    if e.kind == "ok.transfer":
                        root = inst["caps"][lineage[-1]]
                        self.assertEqual(root["scope"]["from"], e.payload["from"])   # only the owner's line moves funds
                self.assertEqual(audit_caps(inst), [], f"seed {seed} step {step}")
                self.assertTrue(conserved(rt.state["ledger"]), f"seed {seed} step {step}")


class TestEvidence(unittest.TestCase):
    def test_incremental_assessment_equals_from_scratch(self):
        """After every event, the locally-revised assessment equals a full recomputation."""
        sources = ["deployer", "monitor", "alice", "auditor", "ci"]
        methods = ["receipt", "probe", "ack", "audit", "test", "inference", "rumour"]
        for seed in range(40):
            rnd = random.Random(seed)
            rt = Runtime(Par(Epistemic()))
            rules = {"evidence": {**DEFAULT_RULES["evidence"],
                                  "trusted": {m: "*" for m in methods if m != "rumour"}}}
            rt.inject("rules.changed", {"rules": rules, "version": 1})
            epi = rt.state["epi"]
            for step in range(60):
                ids = sorted(epi["ev"])
                r = rnd.random()
                by = rnd.choice(sources)
                if r < 0.55 or not ids:
                    p = {"by": by, "method": rnd.choice(methods), "content": step,
                         "derived_from": rnd.sample(ids, min(len(ids), rnd.choice([0, 0, 1, 2]))),
                         "artifact": {"f": rnd.choice("xy")} if rnd.random() < 0.2 else {}}
                    if ids and rnd.random() < 0.2:
                        p["target"] = rnd.choice(ids)
                    else:
                        p.update(prop=f"p{rnd.randrange(4)}", pol=rnd.choice("+-"))
                    if p["method"] == "inference" and not p["derived_from"]:
                        p["method"] = "ack"
                    rt.inject("ok.attest", p)
                elif r < 0.7:
                    rt.inject("ok.claim", {"by": by, "prop": f"p{rnd.randrange(4)}"})
                elif r < 0.8:
                    e = epi["ev"][rnd.choice(ids)]
                    rt.inject("ok.retract", {"by": e["source"], "evidence": e["id"]})
                elif r < 0.9:
                    rt.inject("artifact.changed", {"name": "f", "digest": rnd.choice("xy")})
                else:
                    rt.inject("tick", {"t": epi["now"] + 1})
                full = full_status(epi)
                for prop, st in full.items():
                    self.assertEqual(epi["status"].get(prop, DEFAULT), st, f"seed {seed} step {step} {prop}")


def lifecycle_rt():
    rt = Runtime(Par(Norms()))
    rt.inject("ok.commit", {"by": "d", "creditor": "c", "ant": {"prop": "a"}, "cons": {"prop": "p"}, "deadline": 2})
    return rt


class TestCommitmentLifecycle(unittest.TestCase):
    def test_model_check_single_commitment(self):
        """Exhaustive exploration (to a fixed point) of one commitment under every interleaving of
        time, antecedent/consequent (re)assessment, release and cancel."""
        cid = next(iter(lifecycle_rt().state["norms"]["c"]))
        alphabet = [("tick", {"t": t}) for t in (1, 2, 3)]
        alphabet += [("prop.status", {"prop": q, "status": st, "since": since})
                     for q in "ap" for st, since in (("accepted", 1), ("accepted", 3), ("refuted", None))]
        alphabet += [("ok.release", {"by": "c", "id": cid}), ("ok.cancel", {"by": "d", "id": cid}),
                     ("ok.release", {"by": "d", "id": cid})]          # the debtor cannot release itself
        seen = set()

        def on_step(before, action, events, after):
            b, a = before.state["norms"]["c"][cid], after.state["norms"]["c"][cid]
            if a["status"] not in STATUSES:
                return f"unknown status {a['status']}"
            if b["status"] in ABSORBING and a["status"] != b["status"]:
                return f"left absorbing status {b['status']}"
            if b["status"] == "satisfied" and a["status"] != "satisfied" and action[0] not in ("prop.status", "ok.release"):
                return f"satisfaction undone by {action[0]} (only reassessment or release may)"
            if after.state["norms"]["now"] > a["deadline"] and a["status"] in ("conditional", "detached"):
                return "still pending after the deadline"
            if action == ("ok.release", {"by": "d", "id": cid}) and a["status"] != b["status"]:
                return "debtor released its own commitment"
            if b["status"] != a["status"]:
                seen.add((b["status"], a["status"]))
            return None
        res = reach(lifecycle_rt, alphabet, depth=12, on_step=on_step)
        self.assertIsNone(res["violation"], res)
        self.assertTrue(res["complete"], "state space not exhausted within the bound")
        self.assertIn(("satisfied", "violated"), seen)           # revision is real
        self.assertIn(("violated", "satisfied"), seen)           # ...in both directions
        self.assertFalse({s for s, _ in seen} & ABSORBING)
        self.states, self.transitions = res["states"], seen

    def test_model_checker_finds_seeded_bug(self):
        """Sanity of the checker itself: a false invariant yields a concrete counterexample trace."""
        cid = next(iter(lifecycle_rt().state["norms"]["c"]))
        alphabet = [("tick", {"t": 3}), ("prop.status", {"prop": "a", "status": "accepted", "since": 1})]
        res = reach(lifecycle_rt, alphabet, depth=4,
                    on_step=lambda b, a, e, after: "violated" if after.state["norms"]["c"][cid]["status"] == "violated" else None)
        self.assertEqual(len(res["counterexample"]), 2)


class TestPartialOrder(unittest.TestCase):
    STIM = [("do", {"action": "attest", "args": {"prop": "A", "pol": "+", "method": "probe"}}, "monitor"),
            ("do", {"action": "attest", "args": {"prop": "B", "pol": "+", "method": "receipt"}}, "deployer"),
            ("do", {"action": "transfer", "cap": "own:alice", "args": {"from": "alice", "to": "vendor", "rtype": "credit", "amount": 5}}, "alice"),
            ("do", {"action": "claim", "args": {"prop": "C"}}, "auditor"),
            ("do", {"action": "commit", "args": {"creditor": "alice", "cons": {"prop": "A"}, "deadline": 9}}, "vendor")]

    def test_independent_stimuli_commute(self):
        """5 stimuli touching disjoint facts: all 120 orderings reach one state."""
        self.assertEqual(len(interleavings(world, self.STIM)), 1)

    def test_dependent_stimuli_do_not(self):
        """A use and a revocation of the same capability are order-dependent: the log order IS the fact."""
        def make():
            rt = world()
            rt.do("alice", "delegate", cap="own:alice", to="agent", actions=["transfer"], scope={"from": "alice"},
                  limits={"credit": 50})
            return rt
        cap = [c for c in make().state["inst"]["caps"] if c.startswith("cap-")][0]
        race = [("do", {"action": "revoke", "args": {"target": cap}}, "alice"),
                ("do", {"action": "transfer", "cap": cap, "args": {"from": "alice", "to": "agent", "rtype": "credit", "amount": 5}}, "agent")]
        self.assertEqual(len(interleavings(make, race)), 2)


class TestLocality(unittest.TestCase):
    def test_revision_work_is_independent_of_world_size(self):
        costs = {}
        for n in (20, 200):
            rt = world_n(n, chain=2)
            epi = next(u for u in rt.root.parts if u.name == "epi")
            adj = next(u for u in rt.root.parts if u.name == "adj")
            w0, a0 = epi.work, adj.work
            before = dict(rt.state["norms"]["c"].items())
            snap = {k: v["status"] for k, v in before.items()}
            evs = refute(rt, n // 2, 1)
            changed = {k for k, v in rt.state["norms"]["c"].items() if snap.get(k) != v["status"]}
            costs[n] = (len(evs), epi.work - w0, adj.work - a0, len(changed))
        self.assertEqual(costs[20], costs[200])


if __name__ == "__main__":
    unittest.main()

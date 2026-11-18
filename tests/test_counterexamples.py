"""Negative results and adversarial cases, kept as executable facts.

Each test pins down something the foundation does NOT give you, or an attack it
must (and does) withstand.  See docs/03-status-ledger.md.
"""
import copy
import unittest
from collections import deque

from rigor.impact import cone, what_if
from rigor.kernel.unit import Leaf, Par, Runtime
from rigor.norms.org import DEFAULT_RULES
from rigor.scenario import commitments, genesis, one, prop, run, verdict, world


class Fwd(Leaf):
    def __init__(self, name, inp, outs):
        self.name, self.inputs, self.outputs, self._outs = name, (inp,), tuple(outs), outs

    def on(self, s, ev):
        s.setdefault("seen", []).append(ev.kind)
        for o in self._outs:
            yield o, {}


class Rec(Leaf):
    def __init__(self, name, *inp):
        self.name, self.inputs = name, inp

    def on(self, s, ev):
        s.setdefault("seen", []).append(ev.kind)
        return ()


class RTC(Par):
    """The tempting alternative: a composite that runs its internal feedback to completion."""
    def step(self, state, ev):
        outs, q = [], deque([ev])
        while q:
            e = q.popleft()
            for o in Par.step(self, state, e):
                outs.append(o)
                q.append(type(ev)(0, "", o.kind, o.actor, o.payload, None, o.src))
        return outs


class TestCompositionCounterexamples(unittest.TestCase):
    def units(self):
        return Fwd("p", "go", ["m1"]), Fwd("q", "m1", ["m2"]), Fwd("r", "go", ["m3"]), Rec("s", "m2", "m3")

    def test_run_to_completion_composition_is_not_associative(self):
        """Why Par delivers one event at a time and leaves scheduling to the single runtime."""
        p, q, r, s = self.units()
        flat = RTC(p, q, r, s)
        st = flat.init()
        flat.step(st, Runtime(flat).log.append("go", "env", {}))
        p, q, r, s = self.units()
        nested = RTC(RTC(p, q), r, s)
        st2 = nested.init()
        nested.step(st2, Runtime(nested).log.append("go", "env", {}))
        self.assertEqual(st["s"]["seen"], ["m3", "m2"])
        self.assertEqual(st2["s"]["seen"][:2], ["m2", "m3"])        # same units, different bracketing
        self.assertEqual(st2["s"]["seen"].count("m2"), 2)           # ...and the nested loop even re-delivers

    def test_par_is_associative_on_the_same_units(self):
        logs = []
        for build in (lambda p, q, r, s: Par(p, q, r, s), lambda p, q, r, s: Par(Par(p, q), r, s),
                      lambda p, q, r, s: Par(s, Par(r, Par(q, p)))):
            rt = Runtime(build(*self.units()))
            rt.inject("go")
            logs.append([e.kind for e in rt.log.events])
        self.assertTrue(logs[0] == logs[1] == logs[2])

    def test_liveness_is_not_compositional(self):
        """Each unit answers whenever it is asked (a liveness guarantee given its assumption).
        Composed, each waits for the other: nothing ever happens.  Circular assume-guarantee
        reasoning is unsound for liveness; only safety survives composition unconditionally."""
        a, b = Fwd("a", "b.asks", ["a.asks"]), Fwd("b", "a.asks", ["b.asks"])
        alone = Runtime(Par(a))
        self.assertEqual([e.kind for e in alone.inject("b.asks")], ["b.asks", "a.asks"])    # a is live
        both = Runtime(Par(Fwd("a", "b.asks", ["a.asks"]), Fwd("b", "a.asks", ["b.asks"])))
        self.assertEqual([e.kind for e in both.inject("start")], ["start"])                 # deadlock


class TestEpistemicCounterexamples(unittest.TestCase):
    def test_no_self_certification(self):
        rt = world()
        rt.do("agent", "claim", prop="done:x")
        rt.do("agent", "attest", prop="done:x", pol="+", method="ack", content="trust me")
        self.assertEqual(prop(rt, "done:x"), "unsupported")
        # ...and laundering it through an 'inference' from its own word does not help:
        e = next(iter(rt.state["epi"]["ev"]))
        rt.do("agent", "attest", prop="done:x", pol="+", method="inference", derived_from=[e], content="so it is done")
        self.assertEqual(prop(rt, "done:x"), "unsupported")
        # ...and the rule that forbids it cannot be amended away:
        evs = rt.do("alice", "propose", cap="gov:alice", patch={"evidence.self_support": True})
        self.assertEqual(evs[-2].kind if evs[-1].kind == "settle" else evs[-1].kind, "gov.rejected")

    def test_untrusted_source_and_unknown_method_carry_no_weight(self):
        rt = world()
        rt.do("vendor", "attest", prop="q", pol="+", method="probe", content="x")     # vendor is not a trusted prober
        rt.do("monitor", "attest", prop="q", pol="+", method="vibes", content="x")
        self.assertEqual(prop(rt, "q"), "unsupported")

    def test_equal_evidence_stays_contested_no_forced_decision(self):
        rt = world()
        rt.do("vendor", "commit", creditor="alice", cons={"prop": "q"}, deadline=3)
        rt.do("monitor", "attest", prop="q", pol="+", method="probe", content="up")
        self.assertEqual(one(rt, debtor="vendor")["status"], "satisfied")
        rt.do("monitor", "attest", prop="q", pol="-", method="probe", content="down")
        self.assertEqual(prop(rt, "q"), "contested")
        self.assertEqual(one(rt, debtor="vendor")["status"], "detached")     # undecided is not satisfied
        rt.tick(4)
        self.assertEqual(one(rt, debtor="vendor")["status"], "violated")     # ...and the debtor bears the doubt

    def test_late_evidence_does_not_cure_a_missed_deadline(self):
        rt = world()
        rt.do("vendor", "commit", creditor="alice", cons={"prop": "q"}, deadline=3)
        rt.tick(5)
        rt.do("monitor", "attest", prop="q", pol="+", method="probe", content="up now")
        self.assertEqual(prop(rt, "q"), "accepted")
        self.assertEqual(one(rt, debtor="vendor")["status"], "violated")

    def test_retracting_the_refutation_reinstates_everything_except_what_already_happened(self):
        """Revision runs in both directions.  When the monitor withdraws its probe, the claim is
        accepted again, the violations and verdicts are withdrawn and the duties they created are
        voided.  The refund that was already paid is NOT undone: effects are irreversible; only
        judgments are revisable."""
        rt, o = run(amend=False)
        probe = [e for e in o["refutation"] if e.kind == "ev.recorded"][0].payload["id"]
        self.assertEqual(one(rt, tag="refund")["status"], "satisfied")
        rt.do("monitor", "retract", evidence=probe)
        self.assertEqual(prop(rt, "done:task1"), "accepted")
        self.assertEqual({c["status"] for c in commitments(rt, tag="task")}, {"satisfied"})
        self.assertEqual(verdict(rt, o["c1"]), [])
        self.assertEqual(one(rt, tag="refund")["status"], "void")
        moved = [(e.payload["from"], e.payload["amount"]) for e in rt.log.events if e.kind == "res.transferred"]
        self.assertEqual(moved, [("alice", 30), ("deployer", 30)])           # the money stays moved


class TestGovernanceCounterexamples(unittest.TestCase):
    def propose(self, patch):
        rt = world()
        evs = rt.do("alice", "propose", cap="gov:alice", patch=patch)
        return next(e for e in evs if e.kind in ("gov.rejected", "gov.proposed"))

    def test_obligation_regress_is_rejected_statically(self):
        """'Violating an audit duty creates an audit duty' would regress forever."""
        loop = [{"when": {"tag": "audit"}, "create": {"debtor": "$role:auditor", "creditor": "$creditor",
                                                       "cons": {"prop": "audited:$id"}, "deadline": "+1", "tags": ["audit"]}}]
        e = self.propose({"norms.on_violation+": loop})
        self.assertIn("cycle", e.payload["reason"])

    def test_rules_without_a_unique_model_are_rejected(self):
        bad = [{"id": "x1", "head": ["liable", "?X", "?C"], "body": [["debtor", "?C", "?X"], ["not", "innocent", "?X", "?C"]]},
               {"id": "x2", "head": ["innocent", "?X", "?C"], "body": [["debtor", "?C", "?X"], ["not", "liable", "?X", "?C"]]}]
        self.assertIn("stratifiable", self.propose({"responsibility.rules+": bad}).payload["reason"])

    def test_quorum_and_authority(self):
        rt = world()
        self.assertEqual([e.payload["reason"] for e in rt.do("agent", "propose", patch={"governance.quorum": 1}) if e.kind == "denied"],
                         ["action not granted"])
        evs = rt.do("alice", "propose", cap="gov:alice", patch={"governance.quorum": 1})
        self.assertEqual(rt.state["gov"]["version"], 1)                # one voice is not a quorum of 2
        pid = next(e for e in evs if e.kind == "gov.proposed").payload["id"]
        rt.do("alice", "approve", cap="gov:alice", id=pid)             # voting twice does not help
        self.assertEqual(rt.state["gov"]["version"], 1)
        self.assertIn(">= 1", self.propose({"governance.quorum": 0}).payload["reason"])
        rt.do("auditor", "approve", cap="gov:auditor", id=pid)         # the rules amend their own amendment rule
        self.assertEqual((rt.state["gov"]["version"], rt.state["gov"]["rules"]["governance"]["quorum"]), (2, 1))

    def test_invalid_constitution_is_refused(self):
        from rigor.norms.governance import InvalidRules
        from rigor.norms.org import make_org
        rules = copy.deepcopy(DEFAULT_RULES)
        rules["governance"]["quorum"] = 0
        with self.assertRaises(InvalidRules):
            Runtime(make_org()).inject("genesis", genesis(rules), "constitution")


class TestNormCounterexamples(unittest.TestCase):
    def test_nobody_can_bind_or_release_another(self):
        rt = world()
        rt.do("vendor", "commit", creditor="alice", cons={"prop": "q"}, deadline=3)
        c = one(rt, debtor="vendor")
        why = lambda evs: [e.payload["reason"] for e in evs if e.kind == "norm.rejected"]
        self.assertTrue(why(rt.do("vendor", "release", id=c["id"])))    # debtor cannot release itself
        self.assertTrue(why(rt.do("agent", "release", id=c["id"])))     # nor can a third party
        self.assertTrue(why(rt.do("agent", "assign", id=c["id"], to="agent")))
        rt.do("alice", "assign", id=c["id"], to="auditor")              # the creditor may assign the claim
        self.assertEqual(one(rt, debtor="vendor")["creditor"], "auditor")
        rt.do("vendor", "cancel", id=c["id"])                           # repudiating a live duty is a breach
        self.assertEqual(one(rt, debtor="vendor")["status"], "violated")

    def test_contrary_to_duty_without_paradox(self):
        """Chisholm's scenario: the primary duty is violated AND the secondary duty is in force.
        In standard deontic logic this is inconsistent; with commitments as objects it is just two
        records with different statuses."""
        rt, o = run(amend=False)
        st = {c["id"]: c["status"] for c in commitments(rt)}
        self.assertEqual(st[o["c1"]], "violated")
        self.assertEqual(one(rt, tag="remediation")["basis"]["of"], o["c1"])
        self.assertEqual(one(rt, tag="remediation")["status"], "satisfied")
        self.assertEqual(st[o["c1"]], "violated")                       # remediation does not erase the violation


class TestImpact(unittest.TestCase):
    def test_exact_impact_lies_inside_the_static_cone(self):
        rt, _ = run(amend=False)
        stim = rt.externals()
        i = next(i for i, s in enumerate(stim) if s[2] == "monitor")
        from rigor.actors import Agent, make_service
        from rigor.norms.org import make_org
        pre = Runtime.replay(make_org(Agent(honest=False), make_service()), stim[:i])
        static = cone(pre, ("prop", "done:task1.deploy"))
        exact = what_if(pre, *stim[i])
        existing = {k for k, (b, a) in exact.items() if b is not None}
        self.assertTrue(existing and existing <= static, existing - static)
        after = pre.fork()
        after.inject(*stim[i])
        for k, (b, a) in exact.items():                                 # new judgments hang off the cone
            if b is None and k[0] == "commitment":
                self.assertIn(("commitment", after.state["norms"]["c"][k[1]]["basis"]["of"]), static)
            if b is None and k[0] == "verdict":
                self.assertIn(k, static)
        # and the cone is far from "everything":
        rt2 = world()
        rt2.do("vendor", "commit", creditor="alice", cons={"prop": "unrelated"}, deadline=9)
        self.assertEqual({k for k in cone(rt2, ("prop", "done:task1.deploy")) if k[0] == "commitment"}, set())
        self.assertEqual(what_if(pre, "tick", {"t": 5}, "clock"), {})   # a what-if never touches the real world
        self.assertEqual(pre.log.head, Runtime.replay(make_org(Agent(honest=False), make_service()), stim[:i]).log.head)

    def test_capability_cone(self):
        rt, o = run(amend=False)
        self.assertEqual(cone(rt, ("cap", o["cap"])), {("cap", o["cap"]), ("cap", o["subcap"])})


if __name__ == "__main__":
    unittest.main()

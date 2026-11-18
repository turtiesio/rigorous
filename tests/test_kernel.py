import dataclasses
import random
import unittest

from rigor.actors import Agent, make_service
from rigor.kernel import datalog
from rigor.kernel.argue import grounded, grounded_reference
from rigor.kernel.unit import Divergence, Entity, InterfaceError, Leaf, Par, Runtime
from rigor.norms.adjudicator import Adjudicator
from rigor.norms.commitments import Norms
from rigor.norms.epistemic import Epistemic
from rigor.norms.governance import Governance
from rigor.norms.institution import Institution
from rigor.norms.ledger import Ledger
from rigor.norms.org import make_org
from rigor.scenario import run


class Echo(Leaf):
    def __init__(self, name, inp, out):
        self.name, self.inputs, self.outputs, self._out = name, (inp,), (out,), out

    def on(self, s, ev):
        yield self._out, {}


class TestLog(unittest.TestCase):
    def test_chain_detects_tampering(self):
        rt, _ = run()
        self.assertTrue(rt.log.verify())
        e = rt.log.events[10]
        rt.log.events[10] = dataclasses.replace(e, payload={**e.payload, "x": 1})
        self.assertFalse(rt.log.verify())

    def test_replay_is_deterministic(self):
        rt, _ = run()
        again = Runtime.replay(make_org(Agent(honest=False), make_service()), rt.externals())
        self.assertEqual(again.log.head, rt.log.head)
        self.assertEqual(again.digest(), rt.digest())


class TestComposition(unittest.TestCase):
    def test_par_is_associative_and_commutative(self):
        """Any bracketing / ordering of the same units yields the identical event log."""
        rt, _ = run()
        stim = rt.externals()
        parts = lambda: [Institution(), Ledger(), Epistemic(), Norms(), Adjudicator(), Governance(),
                         Agent(honest=False), make_service()]
        rnd = random.Random(7)
        for _ in range(8):
            us = parts()
            rnd.shuffle(us)
            k = rnd.randrange(1, len(us) - 1)
            j = rnd.randrange(0, k)
            root = Par(Par(Par(*us[:j]), *us[j:k]), Par(*us[k:])) if j else Par(Par(*us[:k]), Par(*us[k:]))
            self.assertEqual(Runtime.replay(root, stim).log.head, rt.log.head)

    def test_entity_scoping_is_functorial(self):
        """A whole organisation nested 1 and 2 boundaries deep behaves exactly as it does
        standalone, up to the namespace prefix (the same system at every scale)."""
        rt, _ = run()
        flat = [(e.kind, e.actor, e.payload) for e in rt.log.events]
        for prefix, wrap in (("acme/", lambda u: Entity("acme", u)),
                             ("holding/acme/", lambda u: Par(Entity("holding", Par(Entity("acme", u)))))):
            nested = Runtime(wrap(make_org(Agent(honest=False), make_service())))
            for kind, payload, actor in rt.externals():
                nested.inject(prefix + kind, payload, prefix + actor)
            strip = lambda x: x[len(prefix):] if x.startswith(prefix) else x     # `settle` is kernel-level
            got = [(strip(e.kind), strip(e.actor), e.payload) for e in nested.log.events]
            self.assertEqual(got, flat)

    def test_composite_can_be_substituted_by_an_atomic_unit(self):
        """Abstraction: replace the two-part service by a one-state monolith with the same
        boundary behaviour.  Every other unit ends in exactly the same state."""
        class Mono(Leaf):
            name, inputs, outputs = "deployer", ("ok.request", "norm.created"), ("do", "signal")

            def on(self, s, ev):
                p = ev.payload
                if ev.kind == "ok.request" and p.get("to") == "deployer":
                    prop = "done:" + p["task"]
                    yield "do", {"action": "commit", "args": {
                        "creditor": p["by"], "cons": {"prop": prop}, "deadline": p["deadline"],
                        "tags": ["paid", "service"], "parent": p.get("parent"), "meta": p.get("meta") or {}}}
                    yield "do", {"action": "attest", "args": {"prop": prop, "pol": "+", "method": "receipt",
                                                              "content": "job accepted"}}
                    yield "signal", {"to": p["by"], "prop": prop, "value": False}
                elif ev.kind == "norm.created" and p["debtor"] == "deployer" and "refund" in p["tags"]:
                    m = p["cons"]["event"]["match"]
                    yield "do", {"action": "transfer", "cap": "own:deployer", "args": {
                        "from": "deployer", "to": m["to"], "rtype": m["rtype"], "amount": m["amount"]["ge"],
                        "memo": "refund " + p["id"]}}
        rt, _ = run()
        mono = Runtime.replay(make_org(Agent(honest=False), Mono()), rt.externals())
        rest = lambda r: {k: v for k, v in r.state.items() if k != "deployer"}
        self.assertEqual(rest(mono), rest(rt))
        self.assertNotEqual([e.kind for e in mono.log.events], [e.kind for e in rt.log.events])   # traces differ; outcomes do not

    def test_boundary_hides_internals(self):
        rt, _ = run()
        jobs = [e for e in rt.log.events if e.kind.startswith("deployer/")]
        self.assertTrue(jobs and all(e.actor.startswith("deployer/") for e in jobs))
        self.assertTrue(all(e.actor == "deployer" for e in rt.log.events if e.src.startswith("deployer/") and e.kind == "do"))
        with self.assertRaises(InterfaceError):          # nobody may listen inside the boundary
            Par(make_service(), Echo("spy", "deployer/job.failed", "spy.out"))

    def test_interface_enforcement(self):
        with self.assertRaises(InterfaceError):          # duplicate identity
            Par(Ledger(), Ledger())
        with self.assertRaises(InterfaceError):          # forged institutional output: two owners of ok.*
            Par(Institution(), Echo("rogue", "tick", "ok.transfer"))
        bad = Echo("bad", "go", "declared")
        bad._out = "undeclared"
        with self.assertRaises(InterfaceError):
            Runtime(Par(bad)).inject("go")

    def test_unit_cannot_impersonate(self):
        """The kernel stamps the actor; a unit that is not a member is denied whatever it says."""
        class Rogue(Leaf):
            name, inputs, outputs = "mallory", ("go",), ("do",)

            def on(self, s, ev):
                yield "do", {"action": "transfer", "cap": "own:alice", "actor": "alice",
                             "args": {"from": "alice", "to": "mallory", "rtype": "credit", "amount": 5}}
        from rigor.scenario import genesis
        rt = Runtime(make_org(Rogue()))
        rt.inject("genesis", genesis(), "constitution")
        evs = rt.inject("go")
        self.assertEqual([e.payload["reason"] for e in evs if e.kind == "denied"], ["not a member"])

    def test_divergence_is_detected_not_hidden(self):
        rt = Runtime(Par(Echo("ping", "pong", "ping"), Echo("pong", "ping", "pong")), fuel=200)
        with self.assertRaises(Divergence):
            rt.inject("ping")


class TestArgumentation(unittest.TestCase):
    def test_matches_textbook_definition_on_random_graphs(self):
        rnd = random.Random(1)
        for _ in range(300):
            n = rnd.randrange(1, 9)
            nodes = list(range(n))
            att = {(rnd.randrange(n), rnd.randrange(n)) for _ in range(rnd.randrange(0, 2 * n))}
            self.assertEqual(grounded(nodes, att), grounded_reference(nodes, att))

    def test_reinstatement_and_stalemate(self):
        self.assertEqual(grounded("abc", [("a", "b"), ("b", "c")]), {"a": "in", "b": "out", "c": "in"})
        self.assertEqual(grounded("ab", [("a", "b"), ("b", "a")]), {"a": "undec", "b": "undec"})


class TestDatalog(unittest.TestCase):
    R = staticmethod(lambda i, h, *b: {"id": i, "head": h, "body": [list(x) for x in b]})

    def test_negation_and_proof(self):
        R = self.R
        rules = [R("1", ["reach", "?X", "?Y"], ["edge", "?X", "?Y"]),
                 R("2", ["reach", "?X", "?Z"], ["reach", "?X", "?Y"], ["edge", "?Y", "?Z"]),
                 R("3", ["isolated", "?X"], ["node", "?X"], ["not", "reach", "a", "?X"])]
        m = datalog.solve(rules, [("edge", "a", "b"), ("edge", "b", "c"), ("node", "c"), ("node", "d")])
        self.assertIn(("reach", "a", "c"), m)
        self.assertEqual(m.query("isolated"), [("isolated", "d")])
        self.assertEqual(m.assumptions(("isolated", "d")), {("reach", "a", "d")})
        self.assertEqual(m.proof(("reach", "a", "c"))[1], "2")

    def test_rejects_unstratifiable_and_unsafe(self):
        R = self.R
        with self.assertRaises(datalog.RuleError):       # p :- not q.  q :- not p.   (no unique model)
            datalog.check([R("1", ["p", "?X"], ["n", "?X"], ["not", "q", "?X"]),
                           R("2", ["q", "?X"], ["n", "?X"], ["not", "p", "?X"])])
        with self.assertRaises(datalog.RuleError):
            datalog.check([R("1", ["p", "?X"], ["not", "q", "?X"])])
        with self.assertRaises(datalog.RuleError):
            datalog.check([R("1", ["p", "?X", "?Y"], ["q", "?X"])])


if __name__ == "__main__":
    unittest.main()

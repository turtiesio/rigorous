"""Continuous conformance against real files and a real subprocess."""
import os
import sys
import tempfile
import unittest

from rigor.adapters import ArtifactWatcher, restore, run_check, save
from rigor.kernel.unit import Runtime
from rigor.norms.org import make_org
from rigor.scenario import one, prop, run, world

IMPL_OK = "def price(qty):\n    return qty * 3\n"
IMPL_BAD = "def price(qty):\n    return qty * 4\n"
TEST = "import sys; sys.path.insert(0, sys.argv[1])\nfrom impl import price\nassert price(2) == 6\n"


class TestConformance(unittest.TestCase):
    def test_evidence_is_bound_to_the_artifact_it_was_about(self):
        with tempfile.TemporaryDirectory() as d:
            impl, test = os.path.join(d, "impl.py"), os.path.join(d, "check.py")
            open(impl, "w").write(IMPL_OK)
            open(test, "w").write(TEST)
            rt = world()
            w = ArtifactWatcher(rt, {"impl": impl, "check": test})
            cmd = [sys.executable, "-B", test, d]
            rt.do("vendor", "commit", creditor="alice", cons={"prop": "conforms:R1"}, deadline=50, tags=["conformance"])
            rt.do("vendor", "claim", prop="conforms:R1")
            self.assertEqual(one(rt, tag="conformance")["status"], "detached")      # a claim alone is nothing

            code, _ = run_check(rt, w, cmd, "conforms:R1", ["impl", "check"])       # real subprocess
            self.assertEqual((code, prop(rt, "conforms:R1")), (0, "accepted"))
            self.assertEqual(one(rt, tag="conformance")["status"], "satisfied")

            open(impl, "w").write(IMPL_BAD)                                         # the implementation changes
            self.assertEqual(w.scan(), ["impl"])
            self.assertEqual(prop(rt, "conforms:R1"), "unsupported")                # old pass is stale, not false
            self.assertEqual(one(rt, tag="conformance")["status"], "detached")

            code, _ = run_check(rt, w, cmd, "conforms:R1", ["impl", "check"])
            self.assertEqual((code, prop(rt, "conforms:R1")), (1, "refuted"))

            open(impl, "w").write(IMPL_OK)                                          # reverting restores the old evidence
            w.scan()
            self.assertEqual(prop(rt, "conforms:R1"), "accepted")
            self.assertEqual(one(rt, tag="conformance")["status"], "satisfied")

            # A passed test is evidence, not proof: an independent observation of production outranks it.
            rt.do("monitor", "attest", prop="conforms:R1", pol="-", method="probe", content="prod returns 8 for qty=2")
            self.assertEqual(prop(rt, "conforms:R1"), "refuted")
            self.assertEqual(one(rt, tag="conformance")["status"], "detached")

    def test_log_persistence_and_tamper_detection(self):
        from rigor.actors import Agent, make_service
        rt, _ = run()
        root = lambda: make_org(Agent(honest=False), make_service())
        with tempfile.TemporaryDirectory() as d:
            path = os.path.join(d, "log.jsonl")
            save(rt, path)
            self.assertEqual(restore(root(), path).digest(), rt.digest())
            import json
            lines = open(path).read().splitlines()
            i = next(i for i, l in enumerate(lines) if '"res.transferred"' in l)
            rec = json.loads(lines[i])
            rec["payload"]["amount"] = 3                                   # rewrite history: 30 -> 3
            lines[i] = json.dumps(rec)
            open(path, "w").write("\n".join(lines) + "\n")
            with self.assertRaises(ValueError):
                restore(root(), path)


if __name__ == "__main__":
    unittest.main()

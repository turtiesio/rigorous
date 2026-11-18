"""The end-to-end scenario, asserted step by step, plus its variants."""
import unittest

from rigor.norms.institution import audit_caps
from rigor.norms.ledger import conserved
from rigor.scenario import balance, commitments, one, prop, run, verdict


def kinds(evs, kind):
    return [e for e in evs if e.kind == kind]


class TestMainScenario(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.rt, cls.o = run()

    def test_1_state_transitions_before_refutation(self):
        o = self.o
        self.assertEqual(o["before"][o["c1"]], "satisfied")     # on the strength of the claim
        self.assertEqual(o["before"][o["c2"]], "satisfied")
        self.assertEqual(o["before"][o["c0"]], "detached")      # Alice now owed the payment

    def test_2_refutation_revises_claim_and_obligations(self):
        o, ref = self.o, self.o["refutation"]
        st = {e.payload["prop"]: (e.payload["was"], e.payload["status"]) for e in kinds(ref, "prop.status")}
        self.assertEqual(st["done:task1.deploy"], ("accepted", "refuted"))
        self.assertEqual(st["done:task1"], ("accepted", "unsupported"))    # its only support fell with the receipt
        self.assertEqual(o["after"][o["c1"]], "violated")
        self.assertEqual(o["after"][o["c2"]], "violated")
        self.assertEqual(o["after"][o["c0"]], "conditional")    # the duty to pay no longer exists

    def test_3_secondary_obligations_are_created_and_tracked(self):
        rt, o = self.rt, self.o
        audit, rem, refund = (one(rt, tag=t) for t in ("audit", "remediation", "refund"))
        self.assertEqual((audit["debtor"], audit["basis"]["of"]), ("auditor", o["c1"]))
        self.assertEqual((rem["debtor"], rem["creditor"], rem["basis"]["finding"]), ("vendor", "alice", "liable"))
        self.assertEqual((refund["debtor"], refund["creditor"], refund["status"]), ("deployer", "alice", "satisfied"))
        self.assertEqual({c["status"] for c in (audit, rem)}, {"satisfied"})
        self.assertEqual(one(rt, tag="payment")["status"], "expired")

    def test_4_authority_and_resources(self):
        rt, o = self.rt, self.o
        self.assertIn("exceeds parent", o["over_delegation"].payload["reason"])
        self.assertIn("budget exceeded", o["overspend"].payload["reason"])
        self.assertIn("revoked", o["revoked_use"].payload["reason"])
        self.assertIn("revoked", o["revoked_sub_use"].payload["reason"])   # cascade through the chain
        self.assertEqual(audit_caps(rt.state["inst"]), [])
        self.assertTrue(conserved(rt.state["ledger"]))
        self.assertEqual((balance(rt, "alice"), balance(rt, "deployer"), balance(rt, "agent")), (100, 40, 0))
        moved = [(e.payload["from"], e.payload["to"], e.payload["amount"]) for e in kinds(rt.log.events, "res.transferred")]
        self.assertEqual(moved, [("alice", "deployer", 30), ("deployer", "alice", 30)])

    def test_5_responsibility_with_proof_and_assumptions(self):
        rt, o = self.rt, self.o
        self.assertEqual(verdict(rt, o["c1"]), [("accountable", "agent"), ("contributed", "deployer"),
                                                ("culpable", "agent"), ("liable", "vendor")])
        self.assertEqual(verdict(rt, o["c2"]), [("accountable", "deployer"), ("liable", "deployer")])
        v = [e for e in kinds(o["refutation"], "verdict") if e.payload["commitment"] == o["c1"]][-1].payload
        self.assertEqual(v["rules_version"], 1)
        self.assertEqual(v["proofs"][f"liable(vendor, {o['c1']})"][1], "liab1")      # AI liability lifts to principal
        self.assertTrue(any(a.startswith("A-OBS") for a in v["assumptions"]))

    def test_6_governance_is_versioned_and_guarded(self):
        rt, o = self.rt, self.o
        self.assertIn("entrenched", o["entrenched"].payload["reason"])
        self.assertEqual(o["enacted"].payload["version"], 2)
        self.assertEqual(o["enacted"].payload["by"], ["alice", "auditor"])
        self.assertEqual([h["version"] for h in rt.state["gov"]["history"]], [1, 2])
        self.assertEqual(o["enacted"].payload["prev"], rt.state["gov"]["history"][0]["digest"])

    def test_7_every_institutional_event_was_authorised(self):
        """Each ok.* event is caused by a `do` of the same actor (count-as, no shortcuts)."""
        by_id = {e.id: e for e in self.rt.log.events}
        for e in self.rt.log.events:
            if e.kind.startswith("ok."):
                cause = by_id[e.cause]
                self.assertEqual((cause.kind, cause.actor, e.actor), ("do", e.payload["by"], "inst"))


class TestVariants(unittest.TestCase):
    def test_honest_agent_is_excused_and_its_liability_is_withdrawn(self):
        """The agent reports the failure.  At t=6 its own commitment lapses and, with the service's
        receipt still standing, the rules hold the vendor liable.  When the probe then shows the
        service failed, that finding is withdrawn and the remediation duty it created is voided."""
        rt, o = run(honest=True, amend=False)
        self.assertEqual(prop(rt, "done:task1"), "unsupported")
        self.assertEqual(verdict(rt, o["c1"]), [("accountable", "agent"), ("contributed", "deployer"),
                                                ("excused", "agent")])
        self.assertEqual(one(rt, tag="remediation")["status"], "void")
        self.assertEqual(one(rt, tag="refund")["status"], "satisfied")
        seq = [tuple(map(tuple, e.payload["findings"])) for e in rt.log.events
               if e.kind == "verdict" and e.payload["commitment"] == o["c1"]]
        self.assertIn(("liable", "vendor"), seq[0])
        self.assertNotIn(("liable", "vendor"), seq[-1])

    def test_rule_change_closes_a_responsibility_gap(self):
        """The service lies (says done).  Under v1 nobody is at fault for the agent's false report;
        the v2 amendment (duty to verify) makes it negligent, and re-adjudication follows."""
        rt, o = run(truthful=False, amend=True)
        vs = [e.payload for e in rt.log.events if e.kind == "verdict" and e.payload["commitment"] == o["c1"]]
        self.assertEqual([v["rules_version"] for v in vs], [1, 2])
        self.assertNotIn(["liable", "vendor"], vs[0]["findings"])
        self.assertIn(["negligent", "agent"], vs[1]["findings"])
        self.assertIn(["liable", "vendor"], vs[1]["findings"])
        self.assertTrue(any(a.startswith("CWA: no record that knew(agent") for a in vs[1]["assumptions"]))
        rem = one(rt, tag="remediation")
        self.assertEqual((rem["rules_version"], rem["created_at"]), (2, 7))

    def test_non_retroactive_rules_do_not_reach_back(self):
        import copy
        from rigor.norms.org import DEFAULT_RULES, RESPONSIBILITY_V2_EXTRA
        from rigor.scenario import world
        rules = copy.deepcopy(DEFAULT_RULES)
        rules["responsibility"]["retroactive"] = False
        rt = world(truthful=False, rules=rules)
        rt.do("alice", "delegate", cap="own:alice", to="agent", actions=["transfer"], scope={"from": "alice"},
              limits={"credit": 50}, delegable=True)
        rt.do("alice", "request", to="agent", task="task1", deadline=5)
        rt.tick(6)
        rt.do("monitor", "attest", prop="done:task1.deploy", pol="-", method="probe", content="x")
        evs = rt.do("alice", "propose", cap="gov:alice", patch={"responsibility.rules+": RESPONSIBILITY_V2_EXTRA})
        pid = [e for e in evs if e.kind == "gov.proposed"][0].payload["id"]
        evs = rt.do("auditor", "approve", cap="gov:auditor", id=pid)
        self.assertEqual(rt.state["gov"]["version"], 2)
        self.assertEqual(kinds(evs, "verdict"), [])
        self.assertEqual(commitments(rt, tag="remediation"), [])

    def test_working_service_everything_is_satisfied(self):
        rt, o = run(works=True, honest=True, amend=False)
        self.assertEqual(prop(rt, "done:task1"), "accepted")
        self.assertEqual(o["before"][o["c1"]], "satisfied")


if __name__ == "__main__":
    unittest.main()

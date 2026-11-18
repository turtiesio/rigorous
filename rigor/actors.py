"""Behavioural units: an AI agent and a service that is itself a composite.

They are ordinary units.  Nothing in the kernel or the institution knows what
kind of thing they are; that difference lives in member attributes
(kind/principal/role) which only the *rules* read.
"""
from __future__ import annotations

from .kernel.unit import Entity, Leaf, Par


class Agent(Leaf):
    """Accepts tasks, subcontracts a service, pays from a delegated budget, reports.

    honest=False models an agent that reports success whatever the service said.
    """
    inputs = ("ok.request", "ok.delegate", "norm.created", "signal", "ev.recorded")
    outputs = ("do", "signal")

    def __init__(self, name="agent", service="deployer", price=30, honest=True):
        self.name, self.service, self.price, self.honest = name, service, price, honest

    def init(self):
        return {"cap": None, "payer": None, "tasks": {}, "subs": {}, "receipts": {}, "signals": {}, "reported": {}}

    def _report(self, s, sub):
        task = s["subs"].get(sub)
        prop = "done:" + sub
        if task is None or task in s["reported"] or prop not in s["signals"]:
            return
        ok = s["signals"][prop]
        if ok or not self.honest:
            if prop not in s["receipts"]:
                return                                   # wait for something to cite
            s["reported"][task] = "success"
            yield "do", {"action": "claim", "args": {"prop": "done:" + task}}
            yield "do", {"action": "attest", "args": {"prop": "done:" + task, "pol": "+", "method": "inference",
                                                      "derived_from": [s["receipts"][prop]],
                                                      "content": f"{sub} reported complete"}}
        else:
            s["reported"][task] = "failure"
            yield "signal", {"to": s["tasks"][task]["creditor"], "prop": "done:" + task, "value": False}

    def on(self, s, ev):
        p, me = ev.payload, self.name
        if ev.kind == "ok.delegate" and p["to"] == me and "transfer" in p["actions"]:
            s["cap"], s["payer"] = p["id"], p["scope"].get("from")
        elif ev.kind == "ok.request" and p.get("to") == me:
            s["tasks"][p["task"]] = {"creditor": p["by"], "deadline": p["deadline"]}
            yield "do", {"action": "commit", "args": {
                "creditor": p["by"], "cons": {"prop": "done:" + p["task"]}, "deadline": p["deadline"],
                "tags": ["task"], "meta": {"task": p["task"]}}}
        elif ev.kind == "norm.created" and p["debtor"] == me and p["meta"].get("task") in s["tasks"] \
                and p["source"] == "act":
            task = p["meta"]["task"]
            sub = task + ".deploy"
            s["subs"][sub] = task
            yield "do", {"action": "request", "args": {
                "to": self.service, "task": sub, "deadline": p["deadline"] - 2, "parent": p["id"],
                "meta": {"price": self.price, "payer": s["payer"]}}}
            yield "do", {"action": "transfer", "cap": s["cap"], "args": {
                "from": s["payer"], "to": self.service, "rtype": "credit", "amount": self.price,
                "memo": sub}}
        elif ev.kind == "ev.recorded" and p["pol"] == "+" and p["source"] != me and p["prop"]:
            s["receipts"].setdefault(p["prop"], p["id"])
            yield from self._report(s, p["prop"][5:])
        elif ev.kind == "signal" and p.get("to") == me and isinstance(p.get("prop"), str):
            s["signals"][p["prop"]] = bool(p.get("value"))
            yield from self._report(s, p["prop"][5:])


class ServiceApi(Leaf):
    """Front of the service: accepts jobs, issues receipts, reports outcomes, pays refunds."""
    name = "api"
    inputs = ("ok.request", "job.done", "job.failed", "norm.created")
    outputs = ("do", "signal", "job")

    def __init__(self, entity="deployer", truthful=True):
        self.entity, self.truthful = entity, truthful

    def on(self, s, ev):
        p = ev.payload
        if ev.kind == "ok.request" and p.get("to") == self.entity:
            prop = "done:" + p["task"]
            yield "do", {"action": "commit", "args": {
                "creditor": p["by"], "cons": {"prop": prop}, "deadline": p["deadline"],
                "tags": ["paid", "service"], "parent": p.get("parent"), "meta": p.get("meta") or {}}}
            yield "job", {"task": p["task"], "for": p["by"]}
            yield "do", {"action": "attest", "args": {"prop": prop, "pol": "+", "method": "receipt",
                                                      "content": "job accepted"}}
        elif ev.kind == "job.done":
            yield "signal", {"to": p["for"], "prop": "done:" + p["task"], "value": True}
        elif ev.kind == "job.failed":
            yield "signal", {"to": p["for"], "prop": "done:" + p["task"], "value": not self.truthful}
        elif ev.kind == "norm.created" and p["debtor"] == self.entity and "refund" in p["tags"]:
            m = p["cons"]["event"]["match"]
            yield "do", {"action": "transfer", "cap": "own:" + self.entity, "args": {
                "from": self.entity, "to": m["to"], "rtype": m["rtype"], "amount": m["amount"]["ge"],
                "memo": "refund " + p["id"]}}


class Worker(Leaf):
    """Back of the service: where the work really happens (or does not)."""
    name = "worker"
    inputs = ("job",)
    outputs = ("job.done", "job.failed")

    def __init__(self, works=False):
        self.works = works

    def on(self, s, ev):
        yield ("job.done" if self.works else "job.failed"), dict(ev.payload)


def make_service(name="deployer", works=False, truthful=True):
    """A two-part system that the outside sees as one actor."""
    return Entity(name, Par(ServiceApi(name, truthful), Worker(works)),
                  imports={"ok.request": "ok.request", "norm.created": "norm.created"},
                  exports={"do": "do", "signal": "signal"})

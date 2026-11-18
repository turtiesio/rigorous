"""Events and the append-only, hash-chained log.

An event is the only thing that ever "happens" in the system.  The log is the
single source of truth; every unit state is a fold over it (event sourcing).
"""
from __future__ import annotations

import hashlib
import json
from dataclasses import dataclass, asdict


def _default(o):
    if isinstance(o, (set, frozenset)):
        return sorted(o)
    raise TypeError(f"not canonicalisable: {type(o)}")


def canon(x) -> str:
    """Canonical JSON: the identity of a value is the identity of this string."""
    return json.dumps(x, sort_keys=True, separators=(",", ":"), default=_default)


def digest(x, n: int = 16) -> str:
    return hashlib.sha256(canon(x).encode()).hexdigest()[:n]


def clone(x):
    """Deep copy of a JSON value (units must never alias event payloads)."""
    return json.loads(canon(x))


@dataclass(frozen=True)
class Event:
    seq: int
    id: str
    kind: str
    actor: str          # who performed it (stamped by the kernel, not the unit)
    payload: dict
    cause: str | None   # id of the event whose processing emitted this one
    src: str            # full path of the emitting leaf (provenance)

    def get(self, k, d=None):
        return self.payload.get(k, d)


GENESIS_HEAD = "0" * 16


class Log:
    def __init__(self):
        self.events: list[Event] = []
        self.head = GENESIS_HEAD

    def append(self, kind, actor, payload, cause=None, src="") -> Event:
        seq = len(self.events)
        eid = digest([self.head, seq, kind, actor, payload, cause, src])
        ev = Event(seq, eid, kind, actor, payload, cause, src)
        self.events.append(ev)
        self.head = eid
        return ev

    def verify(self) -> bool:
        """Recompute the hash chain; False if any event was altered/reordered."""
        head = GENESIS_HEAD
        for i, e in enumerate(self.events):
            if e.seq != i or e.id != digest([head, i, e.kind, e.actor, e.payload, e.cause, e.src]):
                return False
            head = e.id
        return head == self.head

    def copy(self) -> "Log":
        l = Log()
        l.events = list(self.events)
        l.head = self.head
        return l

    def dump(self, path):
        with open(path, "w") as f:
            for e in self.events:
                f.write(canon(asdict(e)) + "\n")

    @staticmethod
    def load(path) -> "Log":
        l = Log()
        with open(path) as f:
            for line in f:
                l.events.append(Event(**json.loads(line)))
        l.head = l.events[-1].id if l.events else GENESIS_HEAD
        return l

"""Units: deterministic open transition systems, closed under composition.

  Unit  U = (S, s0, In, Out, step)      step : S x Event -> S x Emit*

Three constructors, and nothing else:

  Leaf    a primitive unit (its `on` method is the transition function)
  Par     parallel composition; an event is delivered to every part that
          accepts its kind (I/O-automata style broadcast synchronisation).
          Par is *transparent*: it adds no identity, so Par(Par(a,b),c) and
          Par(a,b,c) produce identical logs.
  Entity  a boundary: wraps a unit as ONE named actor.  Internal kinds are
          namespaced "<name>/<kind>" (hidden), only declared exports leave
          stamped with the entity's name, only declared imports enter.

The Runtime is the closed-world executor: a FIFO queue over the root unit.
When the queue drains it issues one `settle` event, so units whose judgments
read several facts at once (the adjudicator) judge only quiescent states.
`step` may mutate the state it is given; purity is recovered by Runtime.fork().
"""
from __future__ import annotations

import copy
from collections import deque
from dataclasses import dataclass, replace

from .event import Event, Log, digest


SETTLE = "settle"   # kernel-issued marker: the cascade has reached quiescence


class InterfaceError(Exception):
    pass


class Divergence(Exception):
    pass


def matches(pattern: str, kind: str) -> bool:
    return kind.startswith(pattern[:-1]) if pattern.endswith("*") else pattern == kind


def overlaps(p: str, q: str) -> bool:
    pw, qw = p.endswith("*"), q.endswith("*")
    a, b = p.rstrip("*"), q.rstrip("*")
    if pw and qw:
        return a.startswith(b) or b.startswith(a)
    if pw:
        return q.startswith(a)
    if qw:
        return p.startswith(b)
    return p == q


@dataclass(frozen=True)
class Emit:
    kind: str
    payload: dict
    actor: str = ""
    src: str = ""


class Unit:
    name: str = ""
    inputs: tuple = ()
    outputs: tuple = ()

    def init(self) -> dict:
        return {}

    def step(self, state: dict, ev: Event) -> list[Emit]:
        raise NotImplementedError

    def accepts(self, kind: str) -> bool:
        return any(matches(p, kind) for p in self.inputs)

    def names(self) -> set:
        return {self.name}


class Leaf(Unit):
    def on(self, s: dict, ev: Event):
        """Yield (kind, payload) pairs; may mutate s."""
        return ()

    def step(self, state, ev):
        outs = []
        for kind, payload in self.on(state, ev) or ():
            if not any(matches(p, kind) for p in self.outputs):
                raise InterfaceError(f"{self.name} emitted undeclared kind {kind!r}")
            outs.append(Emit(kind, payload, self.name, self.name))
        return outs


class Par(Unit):
    def __init__(self, *parts: Unit, open_kinds=("do", "signal")):
        self.parts = sorted(parts, key=lambda u: (u.name, id(u)))
        seen: set = set()
        for u in self.parts:
            ns = u.names()
            if seen & ns or (not isinstance(u, Par) and not u.name):
                raise InterfaceError(f"duplicate or empty unit name: {sorted(seen & ns) or u}")
            seen |= ns
        self._names = seen
        # I/O-automata compatibility: every non-open output kind has one owner.
        for i, a in enumerate(self.parts):
            for b in self.parts[i + 1:]:
                for p in a.outputs:
                    for q in b.outputs:
                        if overlaps(p, q) and not any(matches(o, p.rstrip("*")) and matches(o, q.rstrip("*")) for o in open_kinds):
                            raise InterfaceError(f"output {p!r}/{q!r} owned by both {a.name or 'par'} and {b.name or 'par'}")
        # Boundary privacy: nobody listens inside another entity's namespace.
        ents = [u.name for u in self._flat() if isinstance(u, Entity)]
        for u in self._flat():
            for p in u.inputs:
                for e in ents:
                    if u.name != e and p.startswith(e + "/"):
                        raise InterfaceError(f"{u.name} listens inside boundary of {e}")
        self.inputs = tuple(sorted({p for u in self.parts for p in u.inputs}))
        self.outputs = tuple(sorted({p for u in self.parts for p in u.outputs}))
        self.open_kinds = open_kinds

    def _flat(self):
        for u in self.parts:
            if isinstance(u, Par):
                yield from u._flat()
            else:
                yield u

    def names(self):
        return self._names

    def _key(self, i, u):
        return u.name or f"par{i}"

    def init(self):
        return {self._key(i, u): u.init() for i, u in enumerate(self.parts)}

    def step(self, state, ev):
        outs = []
        for i, u in enumerate(self.parts):
            if u.accepts(ev.kind):
                outs.extend(u.step(state[self._key(i, u)], ev))
        return outs


class Entity(Unit):
    """A system-of-systems seen from outside as one actor."""

    def __init__(self, name: str, inner: Unit, imports: dict | None = None, exports: dict | None = None):
        self.name, self.inner = name, inner
        self.imports = dict(imports or {})   # outer kind -> inner kind
        self.exports = dict(exports or {})   # inner kind -> outer kind
        self.inputs = (name + "/*",) + tuple(self.imports) + ((SETTLE,) if inner.accepts(SETTLE) else ())
        self.outputs = (name + "/*",) + tuple(self.exports.values())

    def init(self):
        return self.inner.init()

    def step(self, state, ev):
        pre = self.name + "/"
        if ev.kind.startswith(pre):
            actor = ev.actor[len(pre):] if ev.actor.startswith(pre) else "^" + ev.actor
            inner = replace(ev, kind=ev.kind[len(pre):], actor=actor)
        elif ev.kind == SETTLE:
            inner = ev
        elif ev.kind in self.imports:
            inner = replace(ev, kind=self.imports[ev.kind], actor="^" + ev.actor)
        else:
            return []
        if not self.inner.accepts(inner.kind):
            return []
        outs = []
        for o in self.inner.step(state, inner):
            if o.kind in self.exports:
                outs.append(Emit(self.exports[o.kind], o.payload, self.name, pre + o.src))
            else:
                outs.append(Emit(pre + o.kind, o.payload, pre + o.actor, pre + o.src))
        return outs


class Runtime:
    def __init__(self, root: Unit, fuel: int = 1_000_000):
        self.root, self.fuel = root, fuel
        self.state = root.init()
        self.log = Log()

    def inject(self, kind: str, payload: dict | None = None, actor: str = "env") -> list[Event]:
        """An external stimulus (the open world speaking). Runs to quiescence."""
        start = len(self.log.events)
        q = deque([(kind, payload or {}, actor, None, "ext:" + actor)])
        steps, dirty, settles = 0, False, self.root.accepts(SETTLE)
        while q:
            steps += 1
            if steps > self.fuel:
                raise Divergence(f"no quiescence after {self.fuel} events")
            k, p, a, cause, src = q.popleft()
            ev = self.log.append(k, a, p, cause, src)
            dirty = dirty or k != SETTLE
            if self.root.accepts(k):
                outs = self.root.step(self.state, ev)
                outs.sort(key=lambda e: e.src)   # stable: per-leaf order kept, cross-leaf order canonical
                q.extend((o.kind, o.payload, o.actor, ev.id, o.src) for o in outs)
            if not q and dirty and settles:
                # Quiescence: let units that judge whole situations (not single events) do so now,
                # so that no judgment is ever issued on a half-propagated state.
                dirty = False
                q.append((SETTLE, {}, "kernel", None, "kernel"))
        return self.log.events[start:]

    def do(self, actor: str, action: str, cap: str | None = None, **args) -> list[Event]:
        return self.inject("do", {"action": action, "args": args, "cap": cap}, actor)

    def tick(self, t: int) -> list[Event]:
        return self.inject("tick", {"t": t}, "clock")

    def fork(self) -> "Runtime":
        r = Runtime.__new__(Runtime)
        r.root, r.fuel = self.root, self.fuel
        r.state = copy.deepcopy(self.state)
        r.log = self.log.copy()
        return r

    def digest(self) -> str:
        return digest(self.state)

    def externals(self) -> list[tuple]:
        return [(e.kind, e.payload, e.actor) for e in self.log.events if e.cause is None and e.kind != SETTLE]

    @staticmethod
    def replay(root: Unit, stimuli) -> "Runtime":
        rt = Runtime(root)
        for kind, payload, actor in stimuli:
            rt.inject(kind, payload, actor)
        return rt

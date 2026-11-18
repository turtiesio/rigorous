"""Adapters: where the formal system touches real software.

An adapter turns something that happened outside (a file changed, a command
exited 0) into an EVENT ABOUT AN OBSERVATION.  It never asserts the fact itself:
"the test command exited 0 against artifact digests D" is evidence bound to D,
and it goes stale the moment an artifact's digest differs.
"""
from __future__ import annotations

import hashlib
import subprocess

from .kernel.event import Log
from .kernel.unit import Runtime


def file_digest(path) -> str:
    with open(path, "rb") as f:
        return hashlib.sha256(f.read()).hexdigest()[:16]


class ArtifactWatcher:
    """Observes real files; reports digest changes as artifact.changed events."""

    def __init__(self, rt: Runtime, paths: dict):
        self.rt, self.paths, self.seen = rt, dict(paths), {}

    def scan(self) -> list:
        changed = []
        for name, path in sorted(self.paths.items()):
            d = file_digest(path)
            if self.seen.get(name) != d:
                self.seen[name] = d
                self.rt.inject("artifact.changed", {"name": name, "digest": d}, "fs")
                changed.append(name)
        return changed


def run_check(rt: Runtime, watcher: ArtifactWatcher, cmd: list, prop: str, artifacts: list,
              actor: str = "ci", method: str = "test", timeout: int = 60):
    """Run a real command; attest its outcome as evidence bound to the artifact digests."""
    watcher.scan()
    proc = subprocess.run(cmd, capture_output=True, text=True, timeout=timeout)
    out = hashlib.sha256((proc.stdout + proc.stderr).encode()).hexdigest()[:16]
    events = rt.do(actor, "attest", prop=prop, pol="+" if proc.returncode == 0 else "-", method=method,
                   artifact={a: watcher.seen[a] for a in artifacts},
                   content={"cmd": cmd, "exit": proc.returncode, "output_digest": out})
    return proc.returncode, events


def save(rt: Runtime, path) -> None:
    rt.log.dump(path)


def restore(root, path) -> Runtime:
    """Rebuild a runtime from a persisted log by re-running its external stimuli,
    and insist that the regenerated log is bit-identical (tamper + determinism check)."""
    log = Log.load(path)
    if not log.verify():
        raise ValueError("log hash chain is broken")
    rt = Runtime.replay(root, [(e.kind, e.payload, e.actor) for e in log.events if e.cause is None and e.kind != "settle"])
    if rt.log.head != log.head:
        raise ValueError("replay diverged from the recorded log")
    return rt

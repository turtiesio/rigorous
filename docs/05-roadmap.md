# Roadmap

Ordered by how much each item reduces what is currently assumed or unresolved (ids from `03-status-ledger.md`).

## Close the gaps in what is already claimed

1. **Model ↔ code (U1).** Either extract the kernel from Lean, or write the kernel's `Par`/`Runtime` in a form
   small enough to transcribe line by line and prove the scheduler (queue + `settle`) preserves T1–T5.
2. **Enforce A-UNIT.** Run units in a restricted interpreter (or WebAssembly) so that determinism and
   state-locality are guaranteed rather than assumed.
3. **Termination (U2).** Give each unit a declared rank on the kinds it emits and require emissions to
   descend; reject compositions whose kind graph has an unranked cycle.
4. **Quiescent judgment for all multi-fact units (U4).** Generalise `settle` into strata, as in synchronous
   languages, and prove glitch-freedom.
5. **Impact soundness (U3).** Property-test cone ⊇ exact over random worlds, then prove it from the indices.
6. **Semi-naive, incremental Datalog (N13).**

## Extend the semantics

7. **Independence analysis and partial-order reduction (U5).** Declare footprints per event kind; derive the
   independence relation; use it in the explorer. The measurements show the saving is k! → 1 when it applies.
8. **Automatic refinement checking (U7)** for finite units by bounded bisimulation.
9. **Distribution (U8).** Replace the single log by per-entity logs with causal links; classify each event
   kind as monotone (mergeable without coordination) or not (needs agreement), following the CALM line of work.
   N3 shows revocation and budgets are in the second class.
10. **Federation (U9).** A gateway unit that turns an inner institution's decisions into outer claims and
    evidence, with identity translation across the boundary.
11. **Dynamic membership and restructuring (U12).** `admit`, `expel` and re-wiring as governed acts.
12. **Time-indexed propositions and entailment (U14).**
13. **Quantitative evidence (U10)** — only with a counterexample showing ordinal ranks are insufficient.

## Additional domains (each should need rules and adapters only, no kernel change)

| Domain | New adapters | New rules | Risk to the narrow waist |
|---|---|---|---|
| Software delivery (spec → code → deploy → runtime) | git, CI, deploy API, metrics | conformance templates | low: prototyped in `tests/test_adapters.py` |
| Procurement and payment | bank/ledger feed | payment and refund templates | low |
| Incident response | paging, monitoring | escalation templates, time-boxed duties | medium: needs dynamic roles (item 11) |
| AI-agent oversight | tool-call logs, model output capture | duty-to-verify, liability lifting | low: the demonstrator's subject |
| Regulatory compliance | document registries | rule sets per regulation, with versions | medium: needs federation (item 10) |
| Multi-organisation contracts | counterpart's log | cross-boundary evidence rules | high: needs items 9 and 10 |

The test for every extension is the same: if it requires a change to `rigor/kernel/`, record the
counterexample that forced it in `01-prior-art.md` §3 before making the change.

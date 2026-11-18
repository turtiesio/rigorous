# Step 0, group A: norms, priority, commitments and rule change

**Status.** This was an attempt to settle the plan's first group of design decisions (D1 to D7 and D14) with a small home-made rule language, and to show the conformance, rule-change and entrenchment targets (R3, R4, R5) as design rules. Two independent reviewers attacked the first draft once ([groupA_attack_astra.md](/root/mic/reports/groupA_attack_astra.md), [groupA_attack_fable.md](/root/mic/reports/groupA_attack_fable.md)). Between them they found four results false with counterexamples and fifteen further rules that fail on concrete cases. This document now records the outcome. It does not offer a repaired design, for the reason given at the end.

## Outcome

1. **The attempt failed.** The selection procedure can end with a set of norms that fails its own test, the feasibility test is sound only for a much narrower class than claimed, and R3, R4 and R5 as stated are false.
2. **A few pieces survived both reviews.** They are listed below and are all small.
3. **The rule language was not new, and it was worse than what exists.** Both reviewers placed it next to published systems that already settle the points it got wrong.
4. **The decision for this group is to stop designing a rule language** and to carry out step 0 for norms inside an existing one, adding only what the plan needs on top.

## What was shown false

| Claim in the draft | Counterexample, in brief |
|---|---|
| R5: no amendment can take a unit outside its authorities' norms, so entrenchment needs no check | A unit relabels the data a norm's pattern reads (files a vendor under an approved class, or gives a purchase another purpose), or acts through a unit it creates, which no authority edge reaches. Neither is a norm that conflicts with anything, so the selection does nothing |
| R5: every amendment is accepted | The language has no construct for who may change a rule, so there is no amendment procedure to accept or amend. If amending is an effect, a prohibition can forbid it; if it is not, nothing governs it |
| R3: a unit that emits only derived effects conforms to every norm | A unit that emits nothing satisfies the premise and misses every obligation |
| R4: the feasibility test holds at every change of version | A cap lowered below what is already committed is added without a test, and the set then fails |
| The selection is unique and consistent | An obligation is tested before the lower-priority permissions and caps it depends on and is never re-tested, so outcomes depend on the order of norms that do not conflict; the cap-raising rule can never fire; routed norms tie |
| The feasibility test is sound | It ignores deadlines, repeated triggers, capacities that are not caps, costs above the declared maximum, and effects that depend on a lapsed lease |

## Rules that failed on a case

- **Exceptions to a prohibition.** When one prohibition blocks every way of meeting a higher obligation, the rule either excepts the whole pattern for ever (and a lowest-priority obligation then rides through), picks one way arbitrarily, or excepts nothing and leaves an obligation that cannot be met. An exception has to belong to one instance of the obligation and lapse when it is discharged.
- **Commitments.** Having no priority, a commitment made first outranks every later norm. A unit can commit its whole budget to a party it controls and so make itself immune to later obligations. If the later norm is declared to apply, the "issuer pays" rule pays the unit for obeying.
- **Escape by refusal.** A unit that waits until a vendor will refuse, then orders and gets a certified refusal, is acquitted. A unit that orders in time and gets no answer is found against. The draft uses "the obligation is to emit an order" and "the obligation is to have an order accepted" interchangeably.
- **Leases.** When an intermediate authority changes a value it exports, what binds the lower unit changes with no selection, no flag and no bearer. If it stops renewing its lease, every effect of the lower unit is refused, so a low-ranked intermediate can halt the obligations of higher authorities.
- **Routing.** Taking the lowest priority on a route stops an intermediate from tightening a rule in its superior's name. It does not stop loosening: the intermediate can declare a mapping under which nothing falls in the forbidden class.
- **Rank.** Rank is the only condition on an authority edge, so any higher-ranked unit can make itself an authority over any lower one.

## What survived

- **Strictly decreasing rank on every authority edge rules out cycles of any length,** however declarations interleave, provided ranks are certified and do not change. Both reviewers confirmed this.
- **Reserving disjoint amounts is a sound way to guarantee a set of obligations** in the narrow case where each fires once, each has an effect that can really be emitted, all interference between them goes through resources with certified capacities, and time does not matter. It can be arbitrarily wasteful: with n mutually exclusive branches each costing the whole budget, it admits one.
- **A norm that travels through an intermediate should not carry more than the intermediate's priority.** This is a cost choice the reviewers accepted, with the consequence that a superior who wants its own priority must bind a unit directly.
- **Not re-running the selection when the environment changes** is a defensible policy, as long as acquittal for a broken assumption is not automatic.
- **The ledger arithmetic** from the first pass still holds.

## Where the pieces already exist

The reviewers checked these at abstract level; I have not read them.

| What the plan needs | Closest existing work |
|---|---|
| Permissions and prohibitions with a default of refusal | Cedar, where a forbid always overrides a permit ([semantics](https://docs.cedarpolicy.com/other/security.html)); XACML's combining algorithms ([specification](https://docs.oasis-open.org/xacml/3.0/xacml-3.0-core-spec-os-en.pdf)) |
| Choosing among conflicting norms by priority | Prioritised normative systems with three published procedures ([Liao et al.](https://arxiv.org/abs/1709.08034)); defeasible deontic logic, which separates kinds of permission ([Governatori et al.](https://arxiv.org/abs/1212.0079)); input/output logic with constraints |
| Obligations, events that count as other events, and the power to change rules | Institutional action languages, with a formal model and a translation to answer-set programs ([InstAL](https://purehost.bath.ac.uk/ws/files/148119668/instal.pdf)) |
| Aggregate limits under concurrent use | The escrow method ([O'Neil](https://www.cs.umb.edu/~poneil/EscrowTM.pdf)) and bounded counters ([Balegas et al.](https://arxiv.org/abs/1503.09052)) |

The draft lacked exactly the construct the institutional languages have and it needed most: a power, meaning a rule about who can bring a norm or an amendment into effect and by what event. Without it there are no charters, no amendment procedures and no way to say an authority exceeded its own.

## What the plan needs that these do not give

Astra's reading, which I accept, is that the only possibly distinctive content is the contract between the pieces:

- how a norm's priority is reduced along a route between authorities;
- exceptions that are scoped to one instance of an obligation and are determined;
- obligation instances that persist across a change of rule version, without being erased, restarted or counted twice;
- rule changes whose cost to existing commitments is financed by a named party before they take effect;
- what an intermediate's exported, leased context means over time.

Each of these was found incomplete or inconsistent in the draft. None has been shown impossible.

## The decision

Step 0 for this group is redone on a different footing.

1. **Take an institutional action language as the norm layer,** with its existing semantics for obligations, constitutive rules and powers, and take forbid-overrides-permit as the rule for permissions and prohibitions.
2. **Encode the benchmark's authority graph and variants in it and run them.** A machine check finds the kind of error the reviewers found here (an obligation tested before its permission, a rule that never fires) immediately and without argument.
3. **Add only the five items above,** one at a time, each as a stated extension with the cases it must pass.
4. **Restate R3, R4 and R5 afterwards,** against what the chosen language already guarantees, so that what is claimed as new is only the extension.
5. **Three requirements from the reviews become fixed inputs** to that work:
   - every attribute a norm reads is either computed from the effect by the trusted interpreter or certified by a party outside the unit, and norms over other attributes are detected after the fact and never prevented;
   - a created unit inherits its creator's authority edges, and limits on a creator count its whole subtree;
   - a commitment to a party under common control does not reserve against an authority's norms.

## Why no repaired design is offered here

Three rounds of prose design in this group (the first pass, its correction, and this draft) each produced a rule set that failed on small cases within one review. The errors were not subtle: a loop that tests things in the wrong order, a rule whose condition can never hold. A fourth prose design would fail the same way. The remaining work for this group is executable, and belongs in step 0 proper.

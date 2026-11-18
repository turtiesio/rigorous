# Step 0: an executable norm layer, first increment

**Superseded.** Two independent testers ran this model and found eight ways it gave wrong outcomes. It has been replaced by an encoding in InstAL itself, described in [Composition contract.md](/root/mic/reports/Composition contract.md); the scenarios here and the testers' counterexamples are its regression tests. What follows is kept as a record.

**Status.** [Step 0 group A norms.md](/root/mic/reports/Step 0 group A norms.md) records a prose rule language that failed review and the decision to redo the work as an executable model. This is the first increment of that. It is a small program in answer-set programming, shaped like an institutional action language: events, norms in force over time, obligations with a lifecycle, and powers that decide which acts of issuing and amending take effect. It runs under the clingo solver, which is unpacked in /root/mic/vendor. It has not been reviewed.

- Rules: [norms.lp](/root/mic/model/norms.lp), about a hundred lines.
- Scenarios: [test_norms.py](/root/mic/model/test_norms.py), seventeen, all passing. Run with `PYTHONPATH=/root/mic/vendor python3 test_norms.py` in /root/mic/model.

## What the increment covers

One unit or a creator and its created unit, under several authorities, over a few time steps. It covers permissions, prohibitions and obligations, priority among authorities, the lifecycle of an obligation, powers, and the amendment procedure. It does not cover resources, caps, commitments, routes through intermediate authorities, leases, or evidence.

## The rules, in words

**Which norms apply.** A norm applies to the unit it names. A created unit inherits every norm that an authority placed on its creator. A unit's own rules are not inherited.

**Which acts of issuing take effect.** An authority may issue a norm over a unit it has an edge to. A unit may issue norms over itself, by the procedure its charter has at that moment. A norm issued without the power, against the issuer's own charter, or by a procedure no longer in force, is void. Issuing against one's own charter is also a finding against the issuer.

**Deciding an effect.** An effect is refused unless something permits it. A permission is a permit norm in force, or a live obligation that the effect would meet: an obligation carries the permission for its own pattern. A prohibition in force refuses a permitted effect, with one exception below.

**Obligations.**
- An obligation is overridden, and not owed, while every way of meeting it is blocked by a prohibition of the same or higher priority.
- Otherwise it is live from issue to its deadline, until one allowed effect meets it.
- A prohibition of strictly lower priority gives way to an emission that meets a live obligation, but only if the obligation has no way that no prohibition touches. The exception belongs to that obligation instance, and ends when the obligation is discharged.
- An obligation still in force and not overridden at its deadline, and never discharged, is a finding against the unit.

**Amendment.** Changing the charter's procedure is itself an effect. It needs the procedure in force at that moment, and an authority may forbid it. The amendment rule can be amended, and the new one then binds.

**Attributes.** A norm that reads an attribute the unit declares for itself is marked as detectable only. The model does not treat it as preventing anything.

## The scenarios

| Scenario | Outcome the model gives |
|---|---|
| A report the regulator requires and a lower authority forbids | The first report is allowed by exception and discharges the duty; a second report and a report to anyone else are refused |
| The same with the prohibition ranked higher | The duty is overridden; no finding |
| The permission ranks below the obligation, issued before it or after it | The same outcome both ways: the report is allowed |
| A lower authority withdraws the permission | The duty remains, and missing it is a finding |
| One prohibition blocks both ways of meeting a duty | Whichever way the unit uses first is allowed; the other is then refused |
| A unit's own low-priority duty with the same pattern | It is overridden and cannot use the higher duty's exception |
| A duty with one forbidden way and one free way | The forbidden way stays refused; the free way discharges it |
| A unit that never acts | A finding |
| A unit orders through a unit it created | The created unit is refused the forbidden order |
| A norm from a body with no edge to the unit | Void |
| A norm its issuer's charter forbids | Void, and a finding against the issuer |
| A prohibition over a purpose the unit declares for itself | Marked detectable only; the relabelled payment goes through |
| Amending the amendment rule | Accepted under the rule in force; an attempt under the old rule is then refused |
| An own rule issued under a superseded procedure | Void |
| An authority forbids amendment | The amendment is refused |
| No permission | Refused |

Nine of these are the counterexamples the reviewers raised against the prose version. The model gives the outcome the reviewers said was required in each.

## What this does and does not show

It shows that the corrected rules are consistent on these cases and that the order in which norms are issued does not change the outcome in the cases where the prose version depended on it. A program with one answer set per scenario cannot test a rule in the wrong order or leave a rule that never fires without it showing.

It does not show that the rules are right in general. Seventeen scenarios written by the author of the rules are weak evidence. The next step is for independent reviewers to write scenarios that the model gets wrong.

## Known gaps, to be added as later increments

1. Resources: declared needs, reservation, caps as prohibitions on totals, and the ledger of [ledger.py](/root/mic/model/ledger.py).
2. Commitments, their admission, and what happens to them when a norm changes.
3. Routes through an intermediate authority, the priority a routed norm carries, and exported context under a lease.
4. Obligations triggered by an event, and repeated triggers.
5. An act window ending before the deadline, and what a refusal or silence from a counterparty means.
6. Creation as an event in time, with limits on a creator that count its whole subtree.
7. Units under common control.

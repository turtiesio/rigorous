# how-to-decide-what-to-defer

**Question.** By what explicit criteria should work be sorted into: do now, defer with a stated trigger, and deliberately not do? What makes a deferral honest instead of a way of losing things?

*Drafted by a language model and accepted by a second, different model acting as reviewer. It is an answer the project has adopted as a working position, not established fact.*

## Sorting criteria

**Do now** — any of these, alone, is sufficient:
- It blocks work that is already in progress (someone is waiting or a task is stalled).
- Delaying it makes the work measurably more expensive or risky, and you can say how.
- It is small enough to finish in the current working session without displacing anything.
- It is a decision only the owner can make, and other work depends on it.

"Do now" should not be reserved for trivial items; high-value work the owner has explicitly prioritised belongs here too. The honest test is: *if this is not done in this session, something concretely worse happens, or it is the owner's stated priority.*

**Defer with a stated trigger** — the work is real and intended, but not now. A deferral is honest when it records, in the tracking system itself:
1. **What the trigger is.** A trigger may be an observable event (a dependency lands, a failure appears), a date, or a scheduled review. A date is acceptable; the point is that the item resurfaces without anyone having to remember it.
2. **What should happen when the trigger fires.** This need not be immediate action — it may be "reassess whether this is still worth doing." But the required response must be written down, so firing the trigger forces a decision rather than silence.
3. **Who or what notices.** The trigger must be checked by something that actually runs: a periodic review of the deferred list, the owner, or a check the system performs. If nothing is responsible for noticing, the trigger is decorative.
4. **Why now is the wrong time.** One sentence. This is what distinguishes deferral from avoidance.

**Deliberately not do** — record the item and the reason, then stop carrying it:
- The cost or risk exceeds the value, on evidence rather than impression.
- It duplicates something already done or planned.
- It depends on a capability that does not exist and is not planned, and no decision has been taken to build or acquire it.
- The owner has decided against it.

The record should state the reason and, where sensible, what would change the verdict. "Not do" is a decision, not a graveyard: it can be revisited, but only deliberately.

## What makes deferral honest

A deferral is honest when the deferred item remains visible and self-resurfacing. The failure mode is a list that only shrinks by forgetting. Three safeguards:

- **No trigger-less deferrals.** Every deferred item has a date, an event, or a scheduled review — otherwise it is a "not do" in disguise and should be recorded as one.
- **Firing a trigger obligates a recorded outcome** — done, re-deferred with a new trigger, or moved to "not do" with a reason. Silence is not an outcome.
- **Periodic audit.** Review the deferred list on a fixed cadence; anything whose trigger has passed without a recorded outcome is treated as a defect in the system, not as the item quietly disappearing.

What I am unsure of: the right cadence for the audit, and whether "not do" items need re-confirmation or should simply persist until someone reopens them.

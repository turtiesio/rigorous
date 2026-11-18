# State of the investigation

**The question.** Is there a smaller, more fundamental, mathematically rigorous and compositional foundation from which systems, software, organisations, institutions, humans, agents and their interactions can be constructed and governed, recursively from resource to nation, self-reflecting, and scaling to about 10^10 actors while staying controlled?

**The answer so far.** Small rigorous substrates exist and each composes with itself. Nobody has composed them into one foundation, and nothing shows it cannot be done. An attempt to do so here has produced one consistent piece (how to compute a verdict when a chain of delegated commitments fails), one failed piece (a rule language for norms and authorities), and a clear finding about method: the pieces are all published, and what is missing is the contract between them.

## Where the work now is

The goal has been narrowed, at the user's direction, to a rigorous contract between existing formalisms, using an established institutional action language and claiming nothing as new. That work is in [Composition contract.md](/root/mic/reports/Composition contract.md), with executable models in /root/mic/model. It supersedes the sections below on the norm layer and on the time bar, and it is the document to read first.

**Where that stands (draft 12, after ten rounds of attack).** Confirmed defects per round, by tester: 21, 15, 6, 5, 6, 3, 3, 1, 1, 2 and 8, 6, 3, 2, 2, 1, 2, 3, 0, 1. Rounds 9 and 10 found no wrong outcome of the models inside the stated scope; their findings were false statements in the document about newly written work. Neither count fell in round 10, so round 11 decides the contract's stop condition on convergence. On whether the verdict layer can stay free of norm rules, the finding is in two parts: no rule about what is permitted has had to be put in the verdict layer, in any case tried; but InstAL has no clock or arithmetic, so every time and amount in a norm is held outside the norm layer and used by the verdict. The contract holds as a division of labour and not as the claim that the norm layer contains the norms.

## The executable milestone

At the user's direction the work moved from contract drafts to running code. [/root/mic/runtime](/root/mic/runtime/README.md) is a minimal runtime in which the norm, resource, evidence and verdict models run in one execution path: units compose recursively behind one interface, every outward effect goes through a guard that consults a certified log, the ledger and InstAL, and deadlines are placed as they fall due. The evidence layer, which the contract only assumed, is implemented there. Its README separates what is implemented and tested, proved elsewhere, assumed, conventional and unsupported. It does not show scale.

The contract and its eleven drafts, with every counterexample and attack report, are kept as they were. Round 11 of review was stopped unfinished when the direction changed.

## What has been produced

| Document | What it is | How far it has been checked |
|---|---|---|
| [Compositional foundations for governed systems.md](/root/mic/reports/Compositional foundations for governed systems.md) | The prior-art survey across seven fields, with a comparison of candidate foundations and the limits any foundation must respect | Four rounds of independent review by two reviewers; no critical error in the last two. Most classical citations checked only at identifier or abstract level |
| [Integrated foundation plan.md](/root/mic/reports/Integrated foundation plan.md) | A plan to find out whether an integrated foundation can be built: a fixed architecture, sixteen open design decisions with their known problems and test cases, five targets, stop rules | Seven rounds; the last found no high or critical defect |
| [Step 0 worked on paper.md](/root/mic/reports/Step 0 worked on paper.md) | A first pass through the design decisions and the accountability target | One round, then corrected; not re-reviewed |
| [Step 0 group B accountability.md](/root/mic/reports/Step 0 group B accountability.md) | Evidence, verdicts and loss allocation, rebuilt after review | One round on the first draft, which found it wrong in most particulars; the rebuilt version is checked by an executable model and not re-reviewed |
| [Step 0 group A norms.md](/root/mic/reports/Step 0 group A norms.md) | A record of a failed attempt at a rule language, with what survived and what to do instead | One round, which broke it; the document now reports that outcome |
| [verdict.py](/root/mic/model/verdict.py), [ledger.py](/root/mic/model/ledger.py) and their tests | Executable models of the verdict and of the spending-cap ledger | All cases from the documents and the reviews pass, with random-history checks of the stated invariants |

All reviews are in /root/mic/reports, in files whose names contain "attack".

## What is established, and how firmly

- **The survey's conclusions.** Closure under composition is available by construction in several formalisms. Guarantees are conditional and have to be shown property by property. Formal coverage is strong for specified components and thin for autonomous principals and for authority that is constituted and not delegated. These held up under four rounds of attack.
- **A verdict rule for delegation chains.** When a chain of time-bound commitments fails, the amount each unit's lateness added to the miss can be computed from each unit's own certified records and the public contracts. A unit that was not late is charged nothing for lateness, the shares add up to the miss, and small changes in the facts make small changes in the shares. This rests on a declared convention about counterfactuals, and it is an assembly of known pieces. The model confirms the arithmetic on twelve cases and 300 random chains.
- **A ledger rule for a spending cap that changes between rule versions.** Rights stay reserved after their lease expires until a final balance arrives; a raised cap frees only what the total leaves room for; spending never exceeds the cap in force when rights were last issued. The model confirms this on the six cases the reviews raised and 2000 random histories.
- **Two small lemmas.** Authority edges that always run from a higher rank to a lower one cannot form a cycle. With a delivery bound among a unit's witnesses, at most one log entry per position can be certified.

## What failed

- **The plan's central target as first worded.** It asked for the minimal sets of parties whose deviations suffice for a failure, computed from per-unit summaries. Whether that is true depends on a causal definition the plan had left open, and under the natural reading it is false: two histories with the same boundary events can have different causes. It was replaced by the verdict rule above, which is a different object.
- **A home-made rule language for norms.** Its selection procedure could return a rule set that failed its own test, and its entrenchment claim was false: a unit could step outside its authorities' rules by relabelling the data a rule reads or by acting through a unit it creates.
- **Every claim of the form "this problem no longer arises".** Each one made during the worked passes was wrong.

## What is open, in order of how much depends on it

1. **The norm layer.** The plan needs obligations, constitutive rules and the power to change rules. These exist in institutional action languages. The next step is to encode the benchmark in one and run it, and to add only the five things those languages lack: how priority falls along a route between authorities, exceptions tied to one instance of an obligation, obligations that persist across a rule change, rule changes whose cost is financed before they take effect, and the meaning over time of context exported under a lease. This needs an answer-set solver, which is not installed in this environment.
2. **The authority analysis** (can a principal ever obtain a right, given quantities and revocation). Not attempted. It depends on the norm layer.
3. **Evidence between units.** Verdicts are final only by a time bar, and a unit whose summary arrives late bears a loss it may not have caused. Whether that is acceptable is a question for the people the foundation would govern.
4. **Everything beyond time.** The verdict rule is worked out only for lateness. Quality bounds and spending limits need their own definition of shortfall.
5. **Scale.** Nothing has been measured. The plan's scale step has not started.

## Whether to go on

By the plan's own stop rule, work continues only if the accountability target survives and either the authority analysis or the conformance target improves on its baseline. The accountability target survives only in a replaced form that is reuse; the other two are untested. So the honest position is that the paper stage is unfinished and the evidence so far leans towards "a careful assembly of known results, with the contract between them still to be written". That contract is the possible contribution. It is small, specific and testable, and it is not the grand foundation the question describes.

## What the process showed

- Reviewing prose found one or two serious errors per round and each prose fix introduced the next. Working small cases found thirteen in one pass. An executable model found a wrong test input in seconds and then confirmed twelve cases that had taken two reviewers to establish.
- Reviewer suggestions copied in without checking caused as many errors as they fixed.
- Two independent reviewers of different model families agreed on every major finding and each caught things the other missed.

The remaining work should be done as executable models with cases first, and reviewed after.

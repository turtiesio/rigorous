# Adversarial review, round 3: "Small rigorous substrates exist; the integrated foundation is unbuilt"

Reviewer: a fresh Claude Fable instance. I formed a view of the current report first, then read the round-2 reviews to avoid repeating fixed points and to check the fixes.

**What was checked this round.**

- All 29 DOI links were resolved through doi.org metadata. Every one returns the claimed work, authors and year, including the 28 dagger-marked ones.
- All 44 arXiv ids were resolved through the arXiv API. Every one exists with the claimed title and authors, apart from one author-order slip (finding 16).
- All 86 other links were status-checked. Three are dead (finding 15).
- Four sources behind the newest material were read in full text or abstract: Chakarov et al. (AWS), Minsky 2014, RFC 8693 and the UCAN spec. Others were checked at search-excerpt level and are marked as such.

**Result in one paragraph.** No fabricated source and no misstated theorem that changes a conclusion. The damage is in the editor-written synthesis. The "demonstrated around 10^9" list in the conclusion mislabels its main evidence and mixes units. The "two changes of kind" claim behind the ten-level table is contradicted by the report's own examples. Several new cells and sentences overstate or contradict the notes.

Labels used below: **VERIFIED WRONG** (a source contradicts the claim), **COULD NOT VERIFY**, **JUDGEMENT**.

---

## 1. HIGH: the "demonstrated for populations around 10^9" list mislabels its evidence and mixes units

**Claim attacked (conclusion).** "Demonstrated for populations around 10^9, each for a restricted function: closure with hierarchical naming; capability-style authority evaluated by runtime monitors over decidable policy languages, in one case with a verified engine; safety invariants under a single operator; conserved balances; and federated routing."

**VERIFIED WRONG: "capability-style".** Neither system the body cites for authority at scale is a capability system.

- The AWS engine evaluates customer-written policies with explicit deny. The paper states "An explicit deny in any policy overrides" allows and proves a lemma named `DenyTrumpsAllow` ([Chakarov et al., full text](https://cdn.amazon.science/bb/40/22ac44f84f6d8eb625ac9666a00f/formally-verified-cloud-scale-authorization.pdf), read this session). That is identity- and policy-based access control with a non-monotone override.
- Zanzibar "stores and evaluates access control lists" ([Pang et al.](https://www.usenix.org/conference/atc19/presentation/pang)).
- The report's own third element defines capability authority as an unforgeable reference that attenuates under delegation. Access control lists and deny-overrides policies are the design the object-capability literature defines itself against.
- The capability standards the report does cite (UCAN, OCapN) are described in the body as drafts with no scale evidence.

So capability-style authority is not demonstrated at 10^9 by anything in the report. What is demonstrated is centrally evaluated, policy-based authorization.

**VERIFIED WRONG: "populations around 10^9 ... with a verified engine".** The AWS figure is an invocation rate, "invoked 1 billion times per second", not a population. The report's own body warns that its scale figures "count different units" and "do not establish a common capacity limit measured in actors". The conclusion then files a decisions-per-second figure under "populations".

**JUDGEMENT: two items have no identifiable evidence.**

- "Safety invariants under a single operator" has no source in the body at population 10^9. seL4 and CompCert are single artefacts.
- "Closure with hierarchical naming" treats delegated naming as evidence for closure in the operadic sense. Naming delegation is recursive; it is not a demonstration that a composite presents the same governed interface as its parts.

**Provenance of the error.** The phrase "capability-style authority" was suggested in round 2 (attack_fable_r2.md, finding 12) and copied into the conclusion without a check against the body.

**Recommended rewording.** "Demonstrated at large scale, each for a restricted function and in different units: delegated hierarchical naming and federated routing (about 6 billion users, 78,000 autonomous systems); centrally evaluated policy-based authorization (access control lists for billions of users in Zanzibar; about 10^9 decisions per second in AWS, with an engine proved against a formal specification); unique centrally issued identity (1.45 billion identifiers); and account balances under a single payment operator (25 billion transactions a month). Capability-style delegation with attenuation has kernel-scale and draft-standard evidence only." Delete "safety invariants under a single operator" or name the evidence. Make the same change to the opening where it leans on this list.

---

## 2. HIGH: "what is being governed changes kind twice" is asserted without a source and contradicted by the report's own examples

**Claims attacked.**

- Opening: "at the level of government, authority originates instead of being delegated."
- Ten-level section: "At the level of government, authority stops being something delegated downward and attenuated and becomes something created by collective recognition."
- Same section: "At the human level the unit stops being something whose behaviour can be specified in advance."

This is new editor-written analysis and is now a headline. No source is cited for it.

**Against the government break.**

- VERIFIED: most governments hold delegated authority. The report's own example is China's 34 provincial, 333 prefecture, 2,843 county and 38,602 township divisions, a chain of authority delegated downward. The table itself places "a root with no superior enforcer" at the Nation row, not the Government row.
- VERIFIED: some governments above the nation hold authority that is delegated upward and enumerated. Under the principle of conferral the EU "shall act only within the limits of the competences conferred upon it by the Member States", and competences not conferred "remain with the Member States" ([TEU Article 5](https://eur-lex.europa.eu/eli/treaty/teu_2016/art_5/oj/eng), confirmed by search excerpt). That is delegation with attenuation at the top of the stack.
- JUDGEMENT: authority created by collective recognition is not special to government. It is how institutional power arises at every level from the team upward. Jones and Sergot's "counts as" account is about institutions in general ([Jones & Sergot 1996](https://doi.org/10.1093/jigpal/4.3.427), DOI resolved). The report's own thin-protocol federation (Internet standards and routing) holds authority that no state delegated.
- The report's third element already says capability systems have rights amplification, so authority that does not merely attenuate appears at the program level too.

**Against the human break.**

- JUDGEMENT: the break is not at "human". Units that cannot be specified in advance appear at the service level (third-party and Byzantine components, which the report covers under agreement) and at the AI-agent level, which the table lists after human with the same gap.
- The real distinction is between specified components and autonomous principals. It cuts across levels instead of sitting between level three and level four.

**Why HIGH and not CRITICAL.** The weaker conclusion survives: formal coverage is thinner for autonomous principals, and root authority is assumed. The stronger "two changes of kind at two named levels" does not.

**Recommended rewording.** "Two distinctions cut across the levels. The first is between components whose behaviour is specified and principals whose behaviour is not (humans, learned agents, third-party services); for the second group prevention gives way to monitoring, incentives and accountability after the fact. The second is between authority that is delegated and attenuated and authority that is constituted by recognition; constitution occurs wherever an institution is founded, and only a sovereign root lacks a superior. No candidate reviewed treats both explicitly." Change the opening sentence to match and retitle the section "Formal coverage is strong for specified components and thin for autonomous principals and constituted authority".

---

## 3. MEDIUM: "No composition result for safety of learned components" is false as written

**Claim attacked (ten-level table, AI agent row, "Main gap").** "No composition result for safety of learned components".

**VERIFIED WRONG (at abstract level).**

- Shielding gives safety guarantees for learned policies by construction. Factored shields give them compositionally for multi-agent learning: one shield per subset of agents, running concurrently, and the approach "can guarantee the safety of agents during learning" ([Elsayed-Aly et al.](https://arxiv.org/abs/2101.11196), abstract fetched).
- An assume-guarantee framework with contracts on deep-network components exists ([Păsăreanu et al.](https://arxiv.org/abs/1810.08303), abstract fetched; it is a framework illustrated on one example, not a general theorem).
- The notes record shielding (`ai_agent_governance_frontier.md` line 209). The editor's cell dropped it.
- The report's own body cites a capability-based design with "provable security" for LLM agents (Debenedetti et al.), and its composition table lists single-trace safety properties as composing.

What is missing is a composition result for the intrinsic, unenforced safety or alignment of learned components. The body says that correctly: "no general theorem that alignment fails to compose; the evidence is empirical".

**Recommended rewording.** "Enforced safety composes through shields and monitors; no composition result exists for the unenforced behaviour or alignment of learned components; Sybil-resistant identity."

---

## 4. MEDIUM: the opening's three "prices" omit the mechanism the conclusion relies on most

**Claim attacked (opening and conclusion).** "guarantees over a universal language are paid for with proof effort, conservatism or runtime enforcement."

**JUDGEMENT (internal inconsistency).**

- The conclusion's own design mixture has a fourth component: "accountability after the fact for everything else, which includes most of what humans do".
- The body says runtime monitors enforce at most safety plus some liveness and "the rest needs deadlines or sanctions".
- So deterrence through audit and sanction is a fourth price. It is the one that carries the levels from the human upward, which the report says are the hard ones.
- Statistical assurance (sampled audit, testing) is a fifth, discussed in the arithmetic section.

The three-way list is presented as exhaustive in two headline positions and is not.

**Recommended rewording.** "paid for with proof effort, conservatism, runtime enforcement, or accountability after the fact (audit, sanction and liability), and the last gives deterrence and not a guarantee."

---

## 5. MEDIUM: law-governed interaction is presented as a "formal result" and under "what it proves"; the cited paper proves nothing

**Claims attacked.**

- Opening: "Partial formal results on governance across levels exist, notably multi-level norm compliance checking and law-governed conformance hierarchies."
- Comparison table, column "What it proves about composition": "hierarchies of laws with conformance by construction".

**VERIFIED (full text of [Minsky 2014](https://arxiv.org/pdf/1403.4321) read locally).**

- The paper is "Dependable Management of Untrusted Distributed Systems". It contains no theorem, lemma or proof (zero matches in the extracted text).
- It describes itself as "work in progress" with "open problems", reporting an "experimental prototype".
- It describes conformance hierarchies in one paragraph: "the hierarchy is enforced by its very construction". It then defers the mechanism: "The manner this is done has been defined in [2], and it is too complex to describe here". Reference [2] is Ao and Minsky's 2003 paper on regulation of distributed coalitions, which the report does not cite.
- Enforcement depends on controllers "trusted" to mediate every message. The report's table does say this.

The citation is real and the description is fair as a design property. It is an enforcement architecture, not a formal result. Only King et al. is one: "a sound and complete computational framework" for compliance between institution levels ([King et al., JAAMAS 2017](https://orca.cardiff.ac.uk/130977), confirmed by search excerpt).

**Recommended rewording.**

- Opening: "One formal result on governance across levels exists (sound and complete multi-level norm compliance checking for institutional models), alongside an enforcement architecture in which subordinate laws conform to superior laws by construction."
- Table cell: "Decentralised enforcement by trusted controllers; conformance of subordinate laws by construction (a design property, not a proved theorem); sound and complete multi-level compliance checking in the institutional-model line."
- Cite Ao and Minsky 2003 for the hierarchy mechanism, or mark it as not fetched.

---

## 6. MEDIUM: the "Limits on any central controller" row cites limits that arise from the absence of a central controller

**Claim attacked (requirements table).** "Limits on any central controller | Proven theorems | Requisite variety; data-rate; Bode; NEXP, PSPACE and undecidability results".

**VERIFIED against the report's own body.**

- The NEXP result is for decentralised control: "Finite-horizon decentralised control with as few as two agents is NEXP-hard".
- The undecidability result is for distributed synthesis with an information fork. [Finkbeiner & Schewe](https://lics.siglog.org/2005/FinkbeinerSchewe-UniformDistributedS.html) confirm the criterion at search-excerpt level.
- The body says "The decisive variable for control is the information structure".
- These results say that removing the central controller makes optimal control harder. They are not limits on a central controller. The same problems with centralised information are far easier (from memory: polynomial for fully observed Markov decision processes, PSPACE for the partially observed finite-horizon case).

Only requisite variety, the data-rate theorem and Bode's integral bound any controller. The row as written supports a reading ("theorems show central control cannot work") that is the opposite of what the cited results show.

**Recommended fix.** Split the row:

- "Limits on any controller | Proven theorems | Requisite variety; data-rate; Bode".
- "Cost of decentralising control | Proven worst-case hardness | NEXP for decentralised control, PSPACE for modular supervision, undecidability with information forks".

---

## 7. MEDIUM: the ten-level table understates formal treatment at the human and team levels, and its sourcing claim is false

**Claims attacked.**

- "The table below walks them once, using only material from the sections above."
- Human row: "Treated as environment in assume-guarantee reasoning; incentives through mechanism design".
- Team row: "Commitment protocols, organisational multi-agent models, supervision".

**VERIFIED (against the report and notes).**

- Several cells use material that appears nowhere above: "electronic institutions", "organisational multi-agent models", "constitutive ('counts as') rules" and "probabilistic assume-guarantee" (first cited after the table). A grep of the report confirms it.
- The notes contain the strongest formal treatments of what coalitions of autonomous agents can enforce, and the report drops them. Coalition logic and alternating-time temporal logic are at `logic_formal_methods.md` line 134 ([Alur, Henzinger & Kupferman 2002](https://doi.org/10.1145/585265.585270), DOI resolved). The report has zero mentions of either.

**JUDGEMENT.**

- For the Human and Team rows the strongest existing formal treatments are game-theoretic and strategic-ability logics, decentralised partially observable decision models (which the body already uses for hardness), and epistemic logic (which the body uses through Halpern and Moses).
- "Treated as environment" is the weakest available treatment, not the strongest.
- This does not overturn "thin from the human upward", because none of these carries guarantees about real humans. It does make the table's "strongest existing" column wrong in two rows.

**Recommended fix.**

- Human row: "Game theory and mechanism design; strategic-ability and epistemic logics; treated as environment in assume-guarantee reasoning".
- Team row: add "alternating-time temporal logic and coalition logic; decentralised decision models".
- Replace "using only material from the sections above" with "drawing on the sections above and the researchers' notes".

---

## 8. MEDIUM: Sybil resistance is listed as "shown impossible"; the source shows less

**Claims attacked.**

- Conclusion, under "Shown impossible, under the hypotheses of the cited theorems": "a permissionless population that is also Sybil-proof without an identity authority or scarce resource".
- Body: "One entity cannot be prevented from presenting many identities without a logically central authority or a scarce resource".

**VERIFIED (abstract level).**

- Douceur's abstract says that without a logically centralised authority Sybil attacks are "always possible except under extreme and unrealistic assumptions of resource parity and coordination among entities" ([Douceur](https://www.microsoft.com/en-us/research/publication/the-sybil-attack/), confirmed by search excerpt). It is an argument in a specific model of resource-challenge validation, not an impossibility theorem of the Gödel or Apt-Kozen kind.
- Decentralised bounded resistance exists. SybilLimit needs no central authority and bounds accepted Sybil identities to O(log n) per attack edge under a fast-mixing social-graph assumption ([Yu et al.](https://www.comp.nus.edu.sg/~yuhf/yuh-sybillimit.pdf), search excerpt).
- "Sybil-proof" in the strict sense stays out of reach; "Sybil-bounded" does not. A reader will take the list item as ruling out both.

**Recommended rewording.** In the conclusion: "strict Sybil-proofness in a permissionless population without an identity authority or a scarce resource (Douceur's argument; bounded Sybil resistance is achievable under trust-graph assumptions)". Move it out of the theorem list or mark it as an argument in a model.

---

## 9. MEDIUM: the conclusion's first sentence and its "two lists" do not answer the question asked

**Claims attacked.**

- "Smaller rigorous substrates exist and compose."
- "The answer to the existence question therefore comes in two lists."

**JUDGEMENT.**

- "and compose" is ambiguous. If it means each substrate is closed under its own composition, it is true by construction. If it means the substrates compose with each other, it contradicts the first open problem ("no published formalism integrates interface, boundary, authority, resource, commitment and monotone state with a proved congruence or compositionality result") and the section title "integration is the open problem".
- The two lists do not bear on the existence question. Nothing on the impossible list is something the question asked for: the question did not ask for unconditional self-certification, complete automatic verification or all the social-choice axioms at once. Nothing on the demonstrated list is fractal, self-reflecting or cross-level. The whole question sits in the "open part" between them.
- So the honest answer is still "unknown; not built; not ruled out", which the report says. The word "therefore" claims more.
- The report also still names no strongest candidate and runs no candidate against the levels. It admits this three times, so I do not rate it higher. It remains the comparison the brief required.

**Recommended rewording.** "Smaller rigorous substrates exist, and each is closed under its own composition; they have not been composed with each other." And: "The existence question stays open. Two lists bound it: what the cited theorems exclude, none of which the question strictly requires, and what has been run at scale, none of which is recursive across levels."

---

## 10. MEDIUM: "no interoperability protocol specifies attenuation across further agent-to-agent hops" is contradicted by a spec the report itself cites

**Claim attacked.** "no interoperability protocol specifies attenuation across further agent-to-agent hops, and none addresses liability."

**VERIFIED WRONG (first half).**

- The UCAN specification, which the report cites elsewhere as a draft capability standard, specifies exactly this: "Each direct delegation MUST either directly restate or attenuate (diminish) its capabilities". It defines "Agent" as its general class of principals and gives a multi-hop chain example ([ucan-wg/spec](https://github.com/ucan-wg/spec), fetched).
- OAuth token exchange expresses multi-hop chains: "A chain of delegation can be expressed by nesting one act claim within another". It adds that prior actors are informational only and must not drive access-control decisions ([RFC 8693, section 4.1](https://www.rfc-editor.org/rfc/rfc8693.html), fetched).

The sentence was written by the editor from a round-2 rewording and over-generalises it. The accurate statement is about adoption, not specification. The liability half stands.

**Recommended rewording.** "Multi-hop attenuated delegation is specified in capability-token drafts such as UCAN and is expressible, for information only, in OAuth token exchange; none of the agent protocols in deployment mandates it across agent-to-agent hops, and none addresses liability."

---

## LOW findings

**11. The conclusion's split of locally checkable invariants is partly inconsistent with the body (JUDGEMENT).**

- "append-only provenance ... need an identity authority, an owner or agreement": the report's own proof-carrying-data row says a message can carry a proof that "its whole history" complies, checked locally. What needs agreement is the absence of forks, not provenance.
- "bounded authority ... can be checked locally": this holds only without revocation, which the third element says "needs an online check".
- Fix: "bounded authority (absent revocation), contract refinement and history compliance can be checked locally; uniqueness of identity, absence of forks, revocation and non-negative shared balances need an authority, an owner or agreement."

**12. Red flag "a mechanism that is at once fair, efficient, strategy-proof and budget-balanced" matches no theorem cited in the body (JUDGEMENT, from memory).**

- The cited results are Arrow, Gibbard-Satterthwaite, Sen, List-Pettit, Myerson-Satterthwaite and Holmström.
- Myerson-Satterthwaite's fourth condition is individual rationality, not fairness.
- Fix: "efficient, incentive-compatible, individually rational and budget-balanced", or cite a fairness impossibility.

**13. "none supplies a composition operator showing that an entity built from entities obeys the same laws" is too strong for accounting (JUDGEMENT).**

- The report's own composition table lists "Double-entry conservation across entities" as composing, and the fifth element says consolidation "is addition with elimination of internal balances".
- For the conservation law the composite provably obeys the same law, since a sum of zero-sum vectors is zero-sum.
- Fix: add "except the conservation identity, which consolidates by addition" to the body sentence and to the Organisation row.

**14. Proof-carrying data "composes along arbitrary message graphs" (COULD NOT VERIFY).**

- The cited project page says proofs attest to a message "and all of its history" in a model where the prover has access to a signature-card-like functionality ([PCD page](http://projects.csail.mit.edu/pcd/), fetched). It does not say "arbitrary graphs".
- The follow-up paper's abstract speaks of PCD for "shallow distributed computations" as the base construction ([Bitansky et al.](https://eprint.iacr.org/2012/095), fetched).
- Fix: "along directed acyclic message graphs, with depth limits or stronger assumptions depending on the construction."

**15. Three dead links (VERIFIED, HTTP 404 this session).**

- `finkbeiner.groups.cispa.de/publications/FS05a.html`. Working replacement: https://finkbeiner.groups.cispa.de/publications/2005-uniform-distributed-synthesis/
- `finkbeiner.groups.cispa.de/publications/BJKKRVW16.pdf`.
- `web.mst.edu/~ff/classes/6600/mclean-sp94.pdf`.
- The last two were reported dead in round 2 and not replaced.

**16. "Yoshida & Hou" has the wrong author order and an old version (VERIFIED).**

- arXiv 2402.16741 is by Ping Hou, Nobuko Yoshida and Iona Kuhn, now at v6 ([arXiv](https://arxiv.org/abs/2402.16741)).
- Fix: "Hou, Yoshida & Kuhn".

**17. "The evidence at scale is for the last two" sits oddly with its own example (JUDGEMENT).**

- The AWS engine is evidence for proof effort (four years, Dafny) applied to the monitor. So the scaled pattern is proof for the small checker plus runtime enforcement for the actors.
- "Verified engine" also has a stated trust base: the specification was reverse-engineered from the old engine and validated by differential testing against 10^15 production samples, and the deployed Java comes from a custom Dafny-to-Java compiler that is tested, not proved ([Chakarov et al.](https://cdn.amazon.science/bb/40/22ac44f84f6d8eb625ac9666a00f/formally-verified-cloud-scale-authorization.pdf), read).
- Fix: "the evidence at scale is for runtime enforcement of restricted policy languages, with proof effort spent on the enforcing engine."

**18. Kim et al. is still cited at v2 (VERIFIED).**

- The arXiv API shows v3 as current. The report hedges this in a parenthesis.
- Fix: cite v3 directly.

---

## Fidelity of the round-2 fixes

**Applied faithfully (spot-checked against sources).**

- The AWS Dafny engine and "a billion times per second": abstract and body read.
- Proof-carrying design demoted to "research-stage option".
- "At least four shapes".
- Aggregation and agreement impossibilities no longer said to soften with population.
- Closure and substitutivity separated.
- Hierarchical supervision now attributed to Zhong & Wonham and to Wong & Wonham.
- Timed common knowledge; MetaCoq scope; Willard; the edit-automata wording; the says/speaks-for attribution.
- Jordan with Mount-Reiter; the Gurevich, Lynch-Tuttle and TLA rows; King et al.
- Kwiatkowska et al. is the TACAS 2010 paper as cited.
- Saoud et al. recover small-gain "as a particular case" (search excerpt).
- Bliudze & Sifakis show BIP glue has the same strong expressiveness as SOS-definable glue, which is a completeness-style result that fits its row (search excerpt).

**Applied wrongly or over-applied.**

- The two lists in the conclusion (finding 1; a reviewer's loose phrase was copied in).
- The agent-hop sentence (finding 10).
- The law-governed interaction result promoted to "formal result" (finding 5).
- The ten-level table (findings 2, 3 and 7), which is new and had no researcher behind it.

---

## Verdict on the headline conclusions

| Headline (opening or conclusion) | Verdict |
|---|---|
| Small rigorous compositional substrates exist | Established. |
| None, alone or combined, is shown to deliver the whole; impossibility not shown | True, as an absence after limited search, which the report says. |
| Closure by construction; substitutivity under side conditions | Established; the first half is definitional. |
| Guarantees over a universal language cost proof effort, conservatism or runtime enforcement | Incomplete: omits after-the-fact accountability, which the conclusion itself relies on (finding 4). |
| Evidence at scale is for restricted policy languages with runtime monitors, one with a verified engine | Supported for policy-based authorization; not for "capability-style" authority, and the unit is decisions per second (findings 1, 17). |
| Formal coverage strong at the bottom three levels, thin from the human upward | Roughly right as a gradient; the two named "changes of kind" are not supported (finding 2), and three table cells are wrong (findings 3, 7). |
| Nine elements are a checklist, not a kernel | Established and candid. |
| "Shown impossible" list | Three items are theorem-grade; the Sybil item is not (finding 8); none is something the question required (finding 9). |
| "Demonstrated around 10^9" list | Wrong as worded (finding 1). |
| Four coordination shapes at 10^9 | Supported as a lower bound. |

**Does it answer the question?** Partly, and honestly. The answer is "no such foundation exists today, none is ruled out, and here is what is priced by theorem". It still does not compare the strongest candidates against each other or against the levels, which it admits. The citation record is now sound: every DOI and arXiv id resolves to the claimed work. The remaining defects are confined to editor-written synthesis and are all fixable by rewording. Findings 1 and 2 need the most care because they sit in the opening and the conclusion.

CRITICAL: 0  HIGH: 2  MEDIUM: 8  LOW: 8

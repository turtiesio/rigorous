# Adversarial review, round 4: "Small rigorous substrates exist; the integrated foundation is unbuilt"

Reviewer: Claude Fable (fresh instance). Date: 17 November 2026. Report reviewed: `/root/mic/reports/Compositional foundations for governed systems.md` (fourth version).

**What was checked this round.** I read the whole report and formed a view before opening the earlier reviews. I then concentrated on the text written after round 3:

- the opening paragraph;
- the bottom rows of the comparison table and the paragraph after it;
- the ten-level section;
- the two requirements-table rows on controller limits and decentralisation;
- the requirements and red-flags paragraphs;
- the conclusion and its two lists.

Sources fetched or searched this session: King et al., Chakarov et al., Ivanov et al., Bliudze & Sifakis, Rohloff & Lafortune, Kim et al. v3 (full HTML), the UPI article, the Zanzibar abstract, RFC 6487, and 21 link status checks. I re-did the report's arithmetic for 10^10 actors (bits, pairs, tree depths, fidelity, attention ratio, China spans) and found it correct.

**Result in one paragraph.** No fabricated source and no misstated theorem that changes a conclusion. The newly added citations say what the report says they say. The remaining damage is again in editor-written synthesis, and most of it sits in the conclusion's "runs at large scale" list. That list says nothing at scale is recursive, which its own first item and the report's body contradict. It says attenuated delegation has only kernel and draft evidence, which deployed systems contradict. Two phrases that round-3 reviewers proposed were copied in without a check and are themselves wrong. The central answer (substrates exist, integration is unbuilt, impossibility is not shown) survives.

Labels used below: **VERIFIED WRONG** (a source contradicts the text), **COULD NOT VERIFY**, **JUDGEMENT**.

---

## 1. HIGH: "none recursive across levels" is contradicted by the list's own first item and by the report's body

**Claim attacked (conclusion).** "Run at large scale, each for a restricted function, in different units, and none recursive across levels: delegated hierarchical naming and federated routing ...".

**VERIFIED WRONG for naming and number delegation.**

- DNS delegation is the same structure at every depth. A zone delegates a sub-zone, which is again a zone with the same record types, the same authority semantics and the same right to delegate further ([RFC 1034](https://www.rfc-editor.org/rfc/rfc1034.html), sections 4.2 and 4.2.1). The report's own label for it is "delegated hierarchical naming".
- Internet number resources are delegated recursively with machine-checked containment. RFC 6487 requires that, for every adjacent pair of certificates in a path, the issuer's resources "encompass" the subject's ([RFC 6487](https://www.rfc-editor.org/rfc/rfc6487.txt), sections 2 and 7.1, text read this session). That is recursion of authority through registry, provider and customer with one rule at every level.
- Measured coverage is large: about 67% of routes had a signed origin authorization in June 2026 ([Hurricane Electric figures as reported](https://ipregistry.co/blog/rpki-blind-spots), secondary source; [IPinfo](https://ipinfo.io/reports/rpki-country) gives 65% of routed IPv4 address space).

**VERIFIED against the report's own body (internal contradiction).**

- "Law is the one field that has run a recursive, self-modifying governance system on billions of people."
- The supervision tree is "the one deployed mechanism that is literally recursive governance", in services the comparison table credits with 10^8 to 10^9 users.
- "the legal entity nests", with 3.1 million identified entities.

**Why it matters.** The question asks whether a recursive structure can run at scale. The conclusion answers that nothing run at scale is recursive. The evidence says something narrower and more useful: recursion at planetary scale exists for single restricted functions (naming authority, number-resource authority, restart supervision, legal entity nesting). What does not exist is one recursion carrying authority, obligation and liability together.

**Charitable reading.** The editor may have meant "none spans the question's ten levels from resource to nation". That is true, but it is not what the sentence says, and the next sentence already makes that point.

**Provenance.** The phrase is not in the round-3 recommended wording (attack_fable_r3.md, finding 1). The editor added it.

**Why HIGH and not CRITICAL.** The headline (integration unbuilt, open part is carrying authority, obligation and liability through every level) survives with rewording.

**Recommended rewording.** "Run at large scale, each for a restricted function and in different units. Some are recursive for that one function: naming and number-resource authority are delegated by the same rule at every depth, supervision trees nest, and legal entities nest. None carries authority, obligation and liability together through the levels the question names: ..."

---

## 2. MEDIUM: "Capability-style delegation with attenuation has kernel-scale and draft-standard evidence only" understates deployed evidence

**Claim attacked (conclusion).** As quoted.

**VERIFIED WRONG in the word "only".**

- RPKI resource certificates are attenuating delegation with offline-verifiable chains, standardised (not draft) and deployed across most of the routing table. See finding 1 for [RFC 6487](https://www.rfc-editor.org/rfc/rfc6487.txt) and coverage.
- Macaroons are bearer tokens whose holder can add caveats offline and hand the narrowed token on ([Birgisson et al., NDSS 2014](https://research.google/pubs/macaroons-cookies-with-contextual-caveats-for-decentralized-authorization-in-the-cloud/)). The researchers' concurrency notes cite this paper (line 157) and say macaroons "can be verified offline" (line 163). The report dropped it.
- Macaroons are in production: PyPI API tokens are macaroons, and a holder can narrow them without contacting PyPI ([pypitoken documentation](https://pypitoken.readthedocs.io/)).
- AWS session policies narrow a delegated role session to the intersection of the role's policy and the passed policy ([AWS IAM documentation](https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html); page reachable, the intersection rule is stated from memory of that page).

**JUDGEMENT.** None of these is an object-capability system in Miller's sense, since they are tokens and certificates checked by a verifier. The sentence says "capability-style delegation with attenuation", and that description fits all of them.

**Provenance.** The sentence is the round-3 reviewer's suggested wording (attack_fable_r3.md, finding 1), copied without a check.

**Recommended rewording.** "Attenuating delegation is deployed at Internet scale in narrow vocabularies (number-resource certificates, caveated bearer tokens, scoped cloud sessions). Object-capability systems with general authority have kernel-scale evidence, and the open multi-hop agent standards are drafts."

---

## 3. MEDIUM: "No candidate reviewed treats both distinctions explicitly" is contradicted by a strand in the comparison table

**Claim attacked (paragraph after the ten-level table).** "No candidate reviewed treats both distinctions explicitly." The two distinctions are specified components versus autonomous principals, and delegated versus constituted authority.

**VERIFIED (abstract level) that the normative multi-agent strand treats constituted authority formally.**

- Boella and van der Torre separate regulative norms from constitutive "counts-as" norms that create institutional facts and modify the normative system itself ([KR 2004](https://iris.unito.it/handle/2318/28822), abstract via search).
- King et al., which the report cites, opens by saying an institution comprises constitutive rules ([TU Delft record](https://research.tudelft.nl/en/publications/automated-multi-level-governance-compliance-checking/), abstract read).
- The researchers' own notes cite Jones and Sergot's 1996 logic of institutionalised power (organisation notes line 162; logic notes line 125). The report's body never mentions it.

**JUDGEMENT on the first distinction.** The same literature is built on agents that can violate norms, and it separates regimentation (making violation impossible) from enforcement by sanction. That is the report's specified-versus-autonomous distinction under another name. I did not fetch a primary source for this, so treat it as from memory.

**Missed prior art that bears on the ninth element and on "dynamic structure".**

- Artikis formalises protocols whose rules the participants change at run time through a meta-protocol, executable in C+ ([arXiv 1005.4815](https://arxiv.org/abs/1005.4815)). This is Hart's rule of change as a specification.
- Pitt, Schaumeier and Artikis axiomatise six of Ostrom's eight principles in the Event Calculus (ACM TAAS 2012, doi 10.1145/2382570.2382575; abstract via search, publisher page returned 403).

Neither proves safety or accountability for self-rewiring organisations, so the report's fourth open problem stands. They do make "thin" a matter of degree and the "no candidate" sentence false.

**Recommended rewording.** "The normative multi-agent literature treats both distinctions (constitutive rules and institutionalised power; regimentation versus sanction), at the scale of research prototypes and without composition theorems across levels. No candidate reviewed combines them with the resource, contract and concurrency layers."

---

## 4. MEDIUM: the "Cost of decentralising control" row files a result that is not about decentralisation

**Claim attacked (requirements table).** "Cost of decentralising control | Proven worst-case hardness | NEXP for decentralised control; PSPACE for modular supervision; undecidability with information forks".

**VERIFIED WRONG for the middle item.** Rohloff and Lafortune's abstract concerns plants given as parallel compositions of automata. The result is that deciding whether a supervisor exists for the modular system against a global specification is PSPACE-complete ([abstract](https://researchwith.njit.edu/en/publications/pspace-completeness-of-modular-supervisory-control-problems/), fetched). The hardness comes from the succinct description of the plant. It applies to a single global supervisor and says nothing about splitting control among several.

**Consequence in the body.** The sentence "The decisive variable for control is the information structure rather than the number of agents" is followed by this result, which depends on the number of modules and not on information structure.

**Provenance.** The row was created by the round-3 split (attack_fable_r3.md, finding 6), which moved all three results together.

**Recommended fix.** Row: "NEXP for decentralised control; undecidability with information forks". Move the PSPACE result to a row or clause on "cost of modular description (state explosion), which applies to a central supervisor too".

---

## 5. MEDIUM: "Enforced safety composes through shields and monitors" is unconditional, against the report's own control section

**Claim attacked (ten-level table, AI agent row).** "Enforced safety composes through shields and monitors".

**VERIFIED against the report's own body.** The cybernetics section says "A conjunction of supervisors is valid when their languages are non-conflicting". A shield that overrides actions is a supervisor. Two shields can each be safe and jointly leave no permitted action, or block progress.

**JUDGEMENT.** The unconditional statement is right for monitors that only halt, since the conjunction of two safety properties is a safety property ([Schneider](https://doi.org/10.1145/353323.353382)). It is not right for shields that substitute actions, or for several agents shielded separately. Multi-agent shielding work treats factoring a shield as a problem with its own conditions (ElSayed-Aly et al., AAMAS 2021, arXiv 2101.11196, cited from memory and not fetched).

**Provenance.** The wording is the round-3 reviewer's (attack_fable_r3.md, finding 3), copied in.

**Recommended rewording.** "Enforced safety composes by conjunction for halting monitors, and for action-substituting shields when they are non-conflicting; conditional results exist for learned controllers with verified mode contracts; ..."

---

## 6. MEDIUM: the opening understates static admission checking, which runs at scale for unrestricted languages

**Claims attacked (opening).** "The evidence at scale is for runtime enforcement of restricted policy languages, with proof effort spent on the enforcing engine". "Proof-carrying admission (unrestricted actors, a small trusted checker) is a research-stage option".

**JUDGEMENT, with deployed counter-examples.**

- The Linux kernel admits untrusted programs only after a static verifier establishes memory safety and termination ([kernel documentation](https://docs.kernel.org/bpf/verifier.html)).
- WebAssembly modules are validated by a small type checker before execution in every major browser, and the specification states a soundness property for that type system (WebAssembly core specification, appendix on soundness; the page did not load this session, so this is from memory).
- Both are "untrusted producer, small checker, designated safety invariant" for a general-purpose language, on the order of 10^9 devices.

**Why it matters.** The opening builds a contrast between restricted languages with runtime monitors (evidence at scale) and admission by checker (research stage). The middle case, admission by a type or safety checker, is deployed at the same scale. It is the stronger precedent for governing arbitrary agents, because the actor language is not restricted. Full proof-carrying data with recursive proofs is correctly described as early.

**Recommended rewording.** "The evidence at scale is for two patterns: runtime enforcement of restricted policy languages, and static admission of unrestricted code by a small checker for a fixed safety invariant (bytecode and kernel-extension verifiers). Admission with actor-supplied proofs of richer invariants is at research stage."

---

## 7. MEDIUM: the four "prices" omit weakening the guarantee to a probabilistic or computational one

**Claim attacked (opening and conclusion).** "guarantees over a universal language are paid for with proof effort, conservatism, runtime enforcement, or accountability after the fact".

**JUDGEMENT (internal inconsistency).** The body escapes several of its own impossibilities by a fifth route:

- randomised protocols escape the quadratic message bound (King & Saia);
- proof-carrying data has "computational" soundness under setup assumptions;
- differential privacy composes as an additive probabilistic budget;
- sampled audit gives a statistical guarantee;
- probabilistic assume-guarantee reasoning is listed as the relevant theory for LLM agents.

None of these is proof effort, conservatism, runtime enforcement or after-the-fact accountability. The report also names "probability" as something its checklist omits. This is the same kind of gap round 3 found when the list had three items.

**Recommended rewording.** "... paid for with proof effort, conservatism, runtime enforcement, a guarantee that holds only with stated probability or under computational assumptions, or accountability after the fact."

---

## 8. MEDIUM: "account balances under a single payment operator" mislabels UPI

**Claim attacked (conclusion).** "account balances under a single payment operator (about 25 billion transactions a month)".

**VERIFIED WRONG.**

- The transaction figure is right: 25.47 billion in October 2026 ([Free Press Journal](https://www.freepressjournal.in/business/upi-transactions-hit-2547-billion-in-october-value-soars-to-3153-lakh-crore), fetched).
- UPI is an interbank scheme. A government factsheet reports 741 banks live on it in mid-2026 ([PIB document, August 2026](https://static.pib.gov.in/WriteReadData/specificdocs/documents/2026/aug/doc2026824968201.pdf), via search summary). The operator runs the central switch, and the payer's and payee's banks debit and credit the accounts.
- So balances sit with hundreds of banks. What is central is message routing and the scheme rules.

**Why it matters.** UPI is a mixed shape (central switch, federated account-holding), which supports the report's own remark that "the shapes combine". The mislabel hides that.

**Provenance.** Round-3 reviewer's wording (attack_fable_r3.md, finding 1), copied in.

**Recommended rewording.** "interbank account transfers routed through one central switch under one rulebook (about 25 billion transactions a month across some 740 banks)".

---

## LOW findings

**9. "One formal result on governance across levels exists" claims uniqueness the survey cannot support (JUDGEMENT; abstract VERIFIED).**

- King et al. say their formal framework "is complemented by a sound and complete computational framework" ([TU Delft record](https://research.tudelft.nl/en/publications/automated-multi-level-governance-compliance-checking/)). So soundness and completeness are of the implementation relative to the authors' own definition of compliance.
- The same group's earlier AAMAS 2015 paper and the works in finding 3 are further formal results on cross-level governance.
- Fix: "At least one formal result ... was found (a computational compliance checker proved sound and complete against its authors' formal semantics for institutional models)".

**10. Opening: "Substitutivity ... holds only under side conditions" contradicts the body (VERIFIED against the report).**

- The body says strong bisimilarity is a congruence in CCS, with no side condition, and that contract refinement gives independent implementability.
- Fix: "holds outright in some settings (strong bisimilarity in CCS, contract refinement) and under side conditions in others (weak equivalences, name passing, bigraphs)".

**11. Two dead links survive a third round (VERIFIED, HTTP 404 this session).**

- `finkbeiner.groups.cispa.de/publications/BJKKRVW16.pdf` and `web.mst.edu/~ff/classes/6600/mclean-sp94.pdf`.
- Both were reported in rounds 2 and 3. The revision note says round-3 findings "are corrected here".
- Fix: cite by DOI. From memory and not resolved this session: Bloem et al., "Decidability in Parameterized Verification", SIGACT News 47(2), 2016; McLean, IEEE Symposium on Security and Privacy 1994.

**12. Kim et al. is still cited at v2 (VERIFIED).**

- v3 (8 April 2026) reports 260 configurations and keeps 17.2 times, 4.4 times and the 39% to 70% planning degradation ([arXiv 2512.08296](https://arxiv.org/abs/2512.08296); v3 HTML read).
- The report's parenthesis is accurate. Citing v3 directly removes the hedge.

**13. Round-3 fix half applied: the Organisation row (VERIFIED against the report).**

- The body now says no tradition supplies a composition operator "apart from the accounting identity". The Organisation row of the ten-level table still says "No theorem that an entity built from entities obeys the same laws" with no exception.
- Fix: add "beyond the accounting identity" to the row.

**14. "Limits on any controller" and "any number of voters" are slightly wider than the theorems (JUDGEMENT).**

- The data-rate theorem and the Bode integral are stated for linear plants and loops. Only requisite variety is model-free. Fix: "Limits on controllers (general for variety; linear-system results for data rate and sensitivity)".
- Arrow's theorem holds for any finite number of voters. Fix: add "finite". This does not matter at 10^10.

---

## Fidelity of the round-3 fixes

**Applied faithfully (checked against the round-3 reviews and, where marked, against sources).**

- The two lists in the conclusion were rewritten with units stated. The AWS figure is now a rate, and Zanzibar is described as policy-based (Zanzibar abstract fetched: trillions of access lists, millions of requests per second).
- "Changes kind twice" was replaced by two cross-cutting distinctions.
- The AI-agent row now cites Ivanov et al., and the paper is as described: compositional learning and verification of neural-network controllers by mode (search excerpt and authors' PDF listing).
- After-the-fact accountability was added as a fourth price.
- Law-governed interaction is marked "a design property, not a proved theorem".
- The controller-limits row was split (but see finding 4).
- Sybil-proofness moved out of the theorem list. The red flag now reads "efficient, incentive-compatible, individually rational and budget-balanced".
- "Unboundedly many actors" now carries "the general problem" and "restricted families remain decidable".
- Radner, the UCAN and OAuth sentence, the proof-carrying data wording, the locally checkable invariants and the Hou, Yoshida & Kuhn author order are all as recommended.
- Chakarov et al.: Dafny engine, "invoked 1 billion times per second", specification validated by differential and shadow testing against 10^15 samples ([ICSE page](https://conf.researchr.org/details/icse-2025/icse-2025-research-track/119/Formally-Verified-Cloud-Scale-Authorization), fetched). "Proved against a formal specification" is fair.
- Bliudze & Sifakis: record 170493 is "A Notion of Glue Expressiveness for Component-Based Systems", CONCUR 2008, and fits its row (search excerpt; the record page returned 429).

**Applied wrongly, or reviewer wording copied in without a check.** Findings 1 (editor addition), 2, 5 and 8 (reviewer phrases), 4 (side effect of a split), 11 and 13 (not applied).

**Could not verify, not counted as errors.** The King et al. full text (Cardiff PDF returned 403; abstract confirmed elsewhere). The Künnemann et al. page (403; the paper exists as CSF 2019 from memory). The absence claims, which the report labels as limited-search absences.

---

## Verdict on the headline conclusions

| Headline (opening or conclusion) | Verdict |
|---|---|
| Small rigorous compositional substrates exist; none alone or combined delivers the whole; impossibility not shown | Established as an absence after limited search, which the report says. Close to trivially true while "the whole" and "controlled" stay undefined. |
| Closure by construction; substitutivity under side conditions | Established; the second half is slightly overstated (finding 10). |
| One formal cross-level governance result plus a conformance architecture | Sources say this; "one" should be "at least one" (findings 3, 9). |
| Impossibility results do not rule out a universal foundation; they price guarantees | Established. The price list is still incomplete (finding 7). |
| Evidence at scale is for runtime enforcement of restricted policy languages; proof-carrying admission is research-stage | Half right. Static admission of unrestricted code by a small checker is also at scale (finding 6). |
| Incentive impossibilities soften with population; aggregation and agreement do not | Supported as stated. |
| Formal coverage strong for specified components, thin for autonomous principals and constituted authority | Right as a gradient. "No candidate treats both distinctions" is wrong (finding 3). |
| Nine elements are a checklist, not a kernel | Established and candid. |
| "Excluded" list | Sound. Each item is theorem-grade under its hypotheses, and Sybil is correctly set apart. |
| "Run at large scale" list | Figures right. Two characterisations are wrong ("none recursive", finding 1; "single payment operator", finding 8) and the capability sentence understates (finding 2). |
| Open part: authority, obligation and liability through every level with a composition law | Survives every finding here. |

**Strongest counter-argument to the main answer.** The report treats "integration unbuilt" as the finding. A critic could say the integration already exists in the only form it ever will: a transition-system or type-theoretic host in which each element is an encoding, with the thin recursive delegations of finding 1 as the deployed waist. On that view the open problem is the choice of invariants and not a missing formalism. The report half-concedes this ("all nine can be encoded in an open transition system") and proposes the worked example that would settle it. It does not refute it.

**Does it answer the question?** Yes, as far as the evidence allows, and honestly. The answer is that no such foundation exists today, none is ruled out, the theorems price what one could promise, and recursion at scale exists only for single narrow functions. The last clause is what the current conclusion gets wrong. The candidate-by-candidate comparison is still missing, which the report states openly.

**Final verdict.** No critical error. The citation record held up again: every newly added source I could reach says what is claimed. One significant supporting claim in the conclusion is false as written and contradicts the body (finding 1). Seven local claims need rewording, four of them inherited from reviewers' suggested text. The fixes are all rewordings. After this round, new wording should be checked against the body before it is pasted in, whoever proposed it.

CRITICAL: 0  HIGH: 1  MEDIUM: 7  LOW: 6

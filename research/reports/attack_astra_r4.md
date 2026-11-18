# Adversarial review, round 4

Reviewed 17 November 2026. I read the current report before consulting earlier attacks, inspected supporting passages and conclusions across all seven researcher notes, and then checked the earlier reviews, especially round 3. This is a targeted source audit of the newest synthesis, not a fresh verification of every citation. I did not modify the report or notes. Only the five numbered findings are counted.

## 1. HIGH — Attenuated capability delegation has deployed evidence beyond kernels and drafts

**Claim attacked:** “Capability-style delegation with attenuation has kernel-scale and draft-standard evidence only.”

**Verified wrong.** Macaroons are an explicit counterexample to this new exclusion. The original paper defines decentralised delegation through bearer credentials with caveats that restrict authority, and supplies an authorization-logic formalisation. It is already mentioned in the concurrency research notes. More decisively, Storj documents actual access grants, restriction commands and sharing, rather than merely a proposed standard. Its implementation documentation says successive restrictions intersect the permissions in the construction chain. Canonical independently documents macaroon authentication in the Snap Store and Enterprise Store.

**Primary evidence:** [Birgisson et al., NDSS 2014](https://research.google/pubs/macaroons-cookies-with-contextual-caveats-for-decentralized-authorization-in-the-cloud/); [Storj access-grant product documentation](https://storj.dev/dcs/access); [Storj's explanation of restricted delegated access](https://www.storj.io/blog/secure-access-control-in-the-decentralized-cloud/); [Storj grant API, Restrict](https://pkg.go.dev/storj.io/common@v0.0.0-20260928130847-58b262d1c082/grant); [Canonical API authentication](https://ubuntu.com/enterprise-store/docs/reference/api-authentication/). These establish operational capability-style attenuation, not a billion-principal benchmark or a verified whole-system authority theorem.

**My judgement:** This is a significant false maturity assessment of a central design candidate. Round 3 correctly rejected using AWS policy authorization as evidence for capability confinement. The revision overcorrected from “that example does not establish it” to “only kernels and drafts establish it.” The main unknown-integration conclusion survives; this supporting exclusion does not.

**Exact fix:** “Capability-style attenuated delegation also has deployed service evidence, including macaroon-based access grants in Storj. This survey has not established a comparable billion-principal benchmark or an end-to-end verified governance guarantee for those deployments.” Add macaroons to the comparison's deployment evidence and retain the distinction between credentials with attenuation and object-capability confinement.

## 2. MEDIUM — The learned-controller correction still excludes what its new citation proves

**Claim attacked:** AI-agent row: “Enforced safety composes through shields and monitors, and conditional results exist for learned controllers with verified mode contracts …; none for unenforced behaviour or alignment”.

**Verified wrong under the ordinary runtime-enforcement reading.** Ivanov et al. verify the learned controllers themselves and their learned mode predictor. Their architecture is learning followed by verification, not a shield intercepting unsafe proposed actions. Theorem 3.9 establishes safety and liveness for the hybrid automaton when its compatibility and verification conditions hold. Consequently, absence of an external runtime enforcer is not absence of a compositional behavioural guarantee. If “unenforced” instead means “neither verified nor otherwise constrained,” the sentence needs to say so; that is a different distinction.

**Primary evidence:** [Authors' full paper, Figure 1, §3 and Theorem 3.9](https://cs.rpi.edu/~ivanor/cpub/emsoft21-ivanov.pdf); [publication DOI](https://doi.org/10.1145/3477023). I downloaded and inspected the paper, including the theorem. Its result concerns a specified dynamical model and controller class, not arbitrary LLM organisations.

**My judgement:** This is a residual error in an otherwise substantial correction, hence MEDIUM rather than the previous round's HIGH. The editor combined a sound counterexample from one reviewer with the other reviewer's overbroad “unenforced” exclusion. Proof and runtime enforcement are alternative ways to establish some guarantees; the report itself makes that point elsewhere.

**Exact fix:** “Conditional composition results exist both for runtime-enforced safety and for directly verified learned controllers with compatible mode contracts. No result reviewed here establishes general safety or alignment of open-ended, heterogeneous, self-modifying LLM-agent organisations. Sybil-resistant identity is a separate gap.”

## 3. MEDIUM — The new two-distinction exclusion understates the institutional model already cited

**Claim attacked:** After distinguishing specified components from autonomous principals, and delegated authority from constituted authority: “No candidate reviewed treats both distinctions explicitly.”

**Verified source content:** King et al. explicitly distinguish regulation from regimentation because institutional designers retain autonomy (§2.2). Their semantics distinguishes observable actions from institutionally recognised actions, introduces constitutive rules, and separates normative obligations/prohibitions from institutional empowerments (§3.1). An auction participant can physically raise a hand without having the institutional standing to bid. This is already formal treatment of autonomous action versus normative constraints, and of constituted powers versus mere physical ability. It is not confined to delegated access rights.

**Primary evidence:** [King et al., full accepted manuscript, §§2.2 and 3.1](https://repository.tudelft.nl/file/File_f9ba14ab-06a6-4d9b-839c-fa789c227f0f), [DOI](https://doi.org/10.1007/s10458-017-9363-y). The Cardiff download returned HTTP 403; I obtained and inspected the Delft copy instead.

**My judgement:** The categorical exclusion is weak and misleading, rather than a source-demonstrated literal contradiction of a formally defined test: the report never operationalises what would count as treating “both distinctions.” King does not derive social recognition or legitimacy from first principles, supply complete implementation models for all participants, or integrate attenuating capabilities with liability across ten levels. But those stronger requirements are different from explicitly representing the distinctions. Turning an integration gap into an absence of representation erases relevant existing work.

**Exact fix:** “Institutional models already distinguish autonomous actions, normative constraints and constituted powers; enforcement architectures separately distinguish untrusted actors from trusted controllers. This survey has not demonstrated a single construction integrating those distinctions with attenuating delegation and cross-level liability.” Retain the two distinctions as an analytical checklist, not a proved exclusion of every reviewed candidate.

## 4. MEDIUM — UPI is interbank payment coordination, not a single operator's account balances

**Claim attacked:** The large-scale conclusion lists “account balances under a single payment operator (about 25 billion transactions a month).”

**Verified wrong as an account-balance architecture.** NPCI operates UPI, but customer balances are held and debited/credited by participating banks. NPCI's settlement document describes interbank clearing and settlement using member settlement accounts at RBI. Calling NPCI the network operator is correct; treating UPI's transaction volume as evidence for balances maintained by one operator collapses distinct institutional and technical responsibilities. Transaction volume also does not demonstrate a proved balance invariant.

**Primary evidence:** [NPCI settlement process, pp. 2–3](https://www.npci.org.in/uploads/UPI_Settlement_Process_256f73e1df.pdf); [RBI, §7.33, participants and architecture](https://www.rbi.org.in/scripts/PublicationsView.aspx?id=20315); [participating bank's definitions of remitter and beneficiary banks](https://centralbank.bank.in/sites/default/files/UPI_TermsAndConditions_V1.pdf).

**My judgement:** The report already allows hybrid coordination shapes, so this does not overturn its large-scale conclusion. It does matter to its purpose: UPI is evidence for interoperable organisations with a shared network and settlement authority, not just another single-operator ledger. This finding challenges the architecture, not the November-2026 transaction count, which I did not independently confirm from a primary monthly release.

**Exact fix:** “Interbank payment routing, clearing and settlement through UPI (about 25 billion transactions a month, according to the cited report), with customer accounts held by participating banks and member settlement accounts at RBI.” Do not describe the volume figure as evidence of formally conserved balances.

## 5. MEDIUM — Splitting the controller-limit row did not restore the control theorems' model assumptions

**Claims attacked:** “Limits on any controller | Proven theorems | Requisite variety; data-rate; Bode”; and the checklist's “Its necessary conditions are theorem-grade … conserved robustness”.

**Verified overstatement:** The cited Bode sensitivity integral constrains a specified feedback-system class and a particular frequency-domain quantity. It is not a conservation law for an arbitrary organisation's robustness. Stein's presentation explicitly concerns SISO sensitivity functions and finite-bandwidth plant/controller pairs; the notes themselves classify extension beyond linear time-invariant systems as analogy. The report drops that qualification precisely where it turns the result into a requirement on any candidate foundation.

**Primary evidence:** [Stein, “Respect the Unstable,” pp. 14–15, Bode Integrals sidebar](https://ieeecss.org/sites/ieeecss/files/2021-09/Gunter%20Stein%20IEEE_2003.pdf). The familiar zero/unstable-pole-sum formula requires the appropriate high-frequency decay; it does not apply unchanged even to every rational linear loop.

**Concrete check, my derivation:** For loop gain L(s)=k/(s+1), k>0, sensitivity S(s)=(s+1)/(s+1+k) has magnitude strictly below one at every finite frequency. It has no compensating sensitivity peak. This does not contradict Bode: this loop lacks the relative-degree condition for that version of the conservation formula. It does contradict treating the slogan as assumption-free.

**Exact fix:** Change the row to “Model-dependent controller limits | Proven within stated plant, channel and feedback assumptions | Requisite-variety bounds; data-rate theorems; Bode sensitivity integrals.” Replace “conserved robustness” with “sensitivity trade-offs for applicable feedback models; organisational extrapolation is a heuristic.” Keep the decentralised-hardness row separate: that round-3 correction was appropriate.

## Revision fidelity and verification limits

The major round-3 fixes were applied: the opening now quantifies over the general parameterised problem, human guarantees are restricted by model and interface coverage rather than ruled out, governments and nations are an overlay rather than a universal two-breakpoint hierarchy, the AWS quantity is decisions per second, and LGI conformance is identified as a design property rather than a proved theorem. Radner is now tied to a particular aggregation model and sufficient parallel resources. I do not re-count those resolved objections.

The defects above illustrate why faithfully copying review recommendations is insufficient. The capability exclusion, “unenforced” exclusion and universal controller row inherit or combine overly compact reviewer wording. The institutional exclusion is another new inference not established by the underlying source. The report should not repair these by adding different universal negative claims.

**Verified surviving evidence:** AWS's primary conference abstract expressly reports an authorization engine invoked a billion times per second, replacement using Dafny, deployment in 2024, and extensive differential testing. This supports the revised opening's distinction between proof effort on the engine and runtime enforcement for requests. [Chakarov et al.](https://conf.researchr.org/details/icse-2025/icse-2025-research-track/119/Formally-Verified-Cloud-Scale-Authorization). King supplies an actual formal/computational correspondence result. Minsky's [2014 paper](https://arxiv.org/pdf/1403.4321), which I downloaded, explicitly distinguishes unspecified actors from trusted interaction controllers; describing it as an enforcement architecture is appropriate.

**Could not verify; not counted as errors:** exhaustive nonexistence of the proposed integration; all scale ceilings and 2026 counts; all dagger citations; or the claimed absence of every possible cross-principal liability semantics. Those would require much broader searches. I did not find a source establishing the full requested foundation. I also did not independently audit every hypothesis behind the compact theorem list. The report's explicit limited-search and incomplete-comparison caveats should remain.

## Headline audit and strongest counterargument

| Opening/conclusion claim | Assessment |
|---|---|
| Small rigorous compositional substrates exist | Established. “Smaller” lacks a common cost metric, so existence does not establish which is most fundamental. |
| Closure is available by construction | Established and substantially definitional. It proves closure of the selected model class, not adequacy of a uniform real-world representation. |
| No reviewed work delivers the whole; impossibility is not established | Defensible as a report of this search. It is not an established theorem of global nonexistence. |
| The listed impossibilities exclude certain unrestricted demands, not the user's foundation | Substantially sound with the stated hypotheses. The user did not require those unrestricted demands. |
| Guarantees are conditional and property-specific | Established but nearly tautological until properties, models and hypotheses are fixed. The body provides useful instances. Accountability is correctly distinguished from a guarantee in the opening; preserve that distinction in the conclusion too. |
| Verified restricted authorization runs at enormous scale | Supported by the AWS source, in decisions per second. No inference to ten billion independent principals follows. |
| Capability delegation has only kernel/draft evidence | Wrong; finding 1. |
| Formal coverage is thinner for autonomous principals and constituted authority | Plausible comparative judgement, not a theorem. Findings 2–3 correct unjustified absence claims used to support it. |
| Nine elements are a nonminimal, incomplete checklist; at least four coordination shapes exist | Reasonable synthesis rather than a fundamental theorem. Neither is offered as exhaustive. |
| The substrates “have not been composed with each other” | Read charitably as “not into the complete requested foundation.” Literally it denies the partial integrations the body already acknowledges. Use that narrower wording; this is not a separate counted finding. |

The strongest counterargument is constructive: start with open transition systems or a proof-assistant metalanguage, represent unknown principals nondeterministically, compose them through contracts, assign constitutive institutional semantics to observed events, and mediate selected actions through enforceable policies. Nothing in the impossibility list forbids this. The report has not specified the adequacy, governance or minimality criterion by which such a construction would count or fail, and it has not established a lower bound excluding ten-billion-principal implementations. Conversely, naming that construction does not prove model adequacy, scalable implementation, legitimate authority or propagation of real liability. This is why “not demonstrated here” is supportable and “no such foundation exists” would not be.

## Verdict

The main answer survives: rigorous compositional ingredients exist, a complete integration at the requested scale is not demonstrated, and the cited impossibilities do not rule it out. I found one significant false maturity claim and four narrower scope or architecture errors, chiefly in the newly edited synthesis; no finding establishes a false central existence answer or a fabricated source. The report answers the question partially and candidly, but remains a literature map and research agenda rather than the requested common-basis comparison or a construction. Its admitted lack of a common worked example is not counted as a new high-severity defect.

CRITICAL: 0  HIGH: 1  MEDIUM: 4  LOW: 0

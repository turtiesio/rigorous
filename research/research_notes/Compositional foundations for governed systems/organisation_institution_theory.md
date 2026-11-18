# Organisational theory, institutional economics, political theory, law and social ontology as compositional foundations

> Verification note for the report writer. Figures marked **[searched 2026-11]** were checked by web search this session (search-result summaries, not full-text reads). Everything else is cited to the primary paper, lecture or encyclopedia entry from bibliographic knowledge; those links (mostly DOIs, JSTOR, SEP, nobelprize.org) were not re-fetched this session, so quote page-level details only after opening them. Labels used: **[Theorem]** = formal result, **[Empirical]** = observed regularity, **[Framework]** = practitioner or conceptual scheme.

## 1. Elinor Ostrom: IAD, rule types, design principles, polycentricity, the Grammar of Institutions

### Takeaway
Ostrom's programme is the most empirically grounded account of nested self-governance, and its ADICO grammar (now Institutional Grammar 2.0) is the closest thing social science has to a formal syntax for rules. It gives a rule taxonomy and a nesting principle but no composition calculus or theorems.

### Cited Findings
- [Framework] The IAD framework's unit of analysis is the **action situation**: participants in positions choose among actions, given information, control over outcomes, and costs/benefits attached to outcomes. Exogenous inputs are biophysical conditions, community attributes and rules-in-use. — [Ostrom 2011, Policy Studies Journal](https://doi.org/10.1111/j.1541-0072.2010.00394.x)
- [Framework] Seven rule types map one-to-one onto the parts of an action situation: **position** (which positions exist), **boundary** (who enters/leaves positions), **choice** (actions assigned to positions), **aggregation** (how individual choices combine into outcomes), **information** (channels), **payoff** (costs/benefits), **scope** (permitted outcomes). — [Ostrom Nobel lecture 2009](https://www.nobelprize.org/prizes/economic-sciences/2009/ostrom/lecture/)
- [Framework] Rules are layered: operational, collective-choice, constitutional (and meta-constitutional) levels; rules at one level are made in action situations at the next level up. This is Ostrom's recursion and self-modification device. — [Ostrom 2011](https://doi.org/10.1111/j.1541-0072.2010.00394.x)
- [Empirical] The eight design principles for long-enduring commons institutions: clear boundaries; congruence of rules with local conditions; collective-choice arrangements; monitoring; graduated sanctions; conflict-resolution mechanisms; minimal recognition of rights to organise; and, for larger systems, **nested enterprises** (governance activities organised in multiple nested layers). — [Ostrom Nobel lecture 2009](https://www.nobelprize.org/prizes/economic-sciences/2009/ostrom/lecture/)
- [Empirical] Cox, Arnold and Villamayor-Tomás reviewed 91 studies and found the principles "well supported empirically"; they proposed a reformulation that splits principles 1, 2 and 4 into sub-parts (e.g. user boundaries vs resource boundaries). Criticisms recorded: the principles are structural and under-represent trust, legitimacy and other processes, and may not transfer outside the settings where they were identified. **[searched 2026-11]** — [Cox, Arnold & Villamayor-Tomás 2010, Ecology and Society 15(4):38](https://www.ecologyandsociety.org/vol15/iss4/art38/)
- [Framework] Polycentricity: Ostrom, Tiebout and Warren (1961) described metropolitan governance as "many centers of decision-making which are formally independent of each other" that nonetheless function as a system through competition, contracting and conflict resolution. — [Ostrom, Tiebout & Warren 1961, APSR](https://doi.org/10.2307/1952530)
- [Framework] Crawford and Ostrom's **ADICO** syntax: **A**ttributes (to whom), **D**eontic (may/must/must not), a**I**m (action), **C**onditions (when/where), **O**r else (sanction). AIC = shared strategy; ADIC = norm; ADICO = rule. — [Crawford & Ostrom 1995, APSR](https://doi.org/10.2307/2082975)
- [Framework] Institutional Grammar 2.0 (Frantz and Siddiki) adds an Object component (ABDICO), splits conditions into activation conditions and execution constraints, adds **constitutive** statements alongside regulative ones, supports **nested statements** (a component can itself be an institutional statement, e.g. an "Or else" that is a full rule binding a monitor), and defines three expressiveness levels (IG Core, IG Extended, IG Logico). **[searched 2026-11]** — [Frantz & Siddiki, IG 2.0 Codebook, arXiv:2008.08937](https://arxiv.org/abs/2008.08937); [Frantz & Siddiki 2021, Public Administration](https://doi.org/10.1111/padm.12719)
- [Framework] Tooling exists through 2025: IG Parser encodes statements in "IG Script" and treats nesting as components composed of components. **[searched 2026-11]** — [Frantz, IG Parser, arXiv:2505.13393](https://arxiv.org/abs/2505.13393)

### Inferences
- Ostrom's contribution to a compositional foundation is threefold: (a) a rule typology indexed to the parts of a game; (b) a level structure in which rule-change is itself a governed action situation; (c) empirical evidence that monitoring, graduated sanctions, dispute resolution and nesting are necessary at scale.
- IG 2.0's constitutive/regulative split lines up with Searle (counts-as vs. deontic rules) and Hart (power-conferring vs. duty-imposing rules); the nested "Or else" lines up with Hurwicz's guardians problem (section 2). Three independent traditions converge on the same structure.
- The design principles are empirical regularities from small-to-medium commons (roughly 10^2 to 10^5 users). Principle 8 asserts nesting but does not specify span, depth or inter-level interfaces.

### Gaps
- No formal semantics for IAD action situations beyond game-theoretic sketches; no proof that the seven rule types are complete or minimal.
- I did not read the full Cox et al. text, so the per-principle support percentages are not reported here.
- Evidence for polycentric governance at national or planetary scale is mostly case-based; I found no quantitative test at that scale.

## 2. Economics of organisation: formal results and what they imply for hierarchy

### Takeaway
Economics supplies the theorems: no mechanism gets efficiency, incentive compatibility, voluntary participation and budget balance together, and no aggregation rule satisfies all reasonable fairness axioms. Hierarchy appears as the cost-minimising answer to bounded information processing, with depth growing logarithmically in size, and with cumulative control loss as the price.

### Cited Findings
**Why organisations exist and where their boundaries lie**
- [Framework] Coase: firms exist because using the price mechanism has costs; a firm expands until the cost of organising one more transaction internally equals the cost of doing it on the market. — [Coase 1937, Economica](https://doi.org/10.1111/j.1468-0335.1937.tb00002.x)
- [Framework/Empirical] Williamson: the transaction is the unit of analysis; governance form (market, hybrid, hierarchy) is matched to asset specificity, uncertainty and frequency, under bounded rationality and opportunism. — [Williamson Nobel lecture 2009](https://www.nobelprize.org/prizes/economic-sciences/2009/williamson/lecture/)
- [Theorem-style model] Grossman-Hart-Moore: contracts are necessarily incomplete, so ownership is the allocation of **residual control rights**; ownership should go to the party whose non-contractible investment matters most. — [Grossman & Hart 1986, JPE](https://doi.org/10.1086/261404); [Hart & Moore 1990, JPE](https://doi.org/10.1086/261729)
- [Framework] Hayek: the knowledge needed for economic order is dispersed, tacit and local ("knowledge of the particular circumstances of time and place"), so decisions must be decentralised to those who hold it and coordinated by compressed signals such as prices. — [Hayek 1945, AER](https://www.jstor.org/stable/1809376)

**Delegation and incentives**
- [Framework] Jensen and Meckling: the firm is a "nexus of contracts"; agency costs = monitoring + bonding + residual loss. — [Jensen & Meckling 1976, JFE](https://doi.org/10.1016/0304-405X(76)90026-X)
- [Theorem] Holmström's informativeness principle: any signal that is informative about the agent's action should enter the optimal contract. — [Holmström 1979, Bell Journal](https://doi.org/10.2307/3003320)
- [Theorem] Holmström's team impossibility: with joint output and unobserved individual effort, no sharing rule that is **budget-balanced** yields efficient effort as a Nash equilibrium. Efficiency needs a "budget breaker" (a residual claimant outside the team). — [Holmström 1982, Bell Journal](https://doi.org/10.2307/3003457)
- [Theorem-style model] Multitask principal-agent: strong incentives on measurable tasks crowd out unmeasured ones, so low-powered incentives, job design and limits on outside activity can be optimal. — [Holmström & Milgrom 1991, JLEO](https://doi.org/10.1093/jleo/7.special_issue.24)
- [Theorem-style model] Aghion and Tirole: formal authority (the right to decide) differs from real authority (effective control, held by whoever has the information); delegation raises agent initiative at the cost of principal control. — [Aghion & Tirole 1997, JPE](https://doi.org/10.1086/262063)
- [Framework] Team theory (Marschak and Radner) studies optimal decision rules and information structures when members share one objective but hold different information: the pure coordination problem with incentives removed. — [Marschak 1955, Management Science](https://doi.org/10.1287/mnsc.1.2.127)

**Mechanism design and impossibility**
- [Theorem] Revelation principle: any outcome achievable in equilibrium by any mechanism is achievable by a direct mechanism in which truthful reporting is an equilibrium. — [Myerson 1979, Econometrica](https://doi.org/10.2307/1912346)
- [Theorem] Hurwicz (1972): in standard exchange economies no mechanism is simultaneously incentive compatible, individually rational and Pareto efficient. Related results (Mount-Reiter, Jordan) show the competitive price mechanism uses the minimal message space among efficient decentralised mechanisms. — [Nobel Committee, Scientific Background 2007](https://www.nobelprize.org/uploads/2018/06/advanced-economicsciences2007.pdf)
- [Theorem] Myerson-Satterthwaite: for bilateral trade with private values and overlapping supports, no mechanism is at once Bayesian incentive compatible, individually rational, budget balanced and ex post efficient. — [Myerson & Satterthwaite 1983, JET](https://doi.org/10.1016/0022-0531(83)90048-0)
- [Theorem] Gibbard-Satterthwaite: with three or more outcomes, every strategy-proof, onto social choice function on an unrestricted domain is dictatorial. — [Gibbard 1973, Econometrica](https://doi.org/10.2307/1914083); [Satterthwaite 1975, JET](https://doi.org/10.1016/0022-0531(75)90050-2)
- [Theorem] Arrow: with three or more alternatives, no social welfare function satisfies unrestricted domain, weak Pareto, independence of irrelevant alternatives and non-dictatorship. — [SEP, Arrow's Theorem](https://plato.stanford.edu/entries/arrows-theorem/)
- [Theorem] Sen's liberal paradox: unrestricted domain, weak Pareto and even minimal individual decisiveness (two people each decisive over one pair) are jointly inconsistent. — [Sen 1970, JPE](https://doi.org/10.1086/259614)
- [Framework] Hurwicz's Nobel lecture "But Who Will Guard the Guardians?": a mechanism is only "implemented" if its rules are themselves enforced, so enforcers must be modelled as players in a larger game. The regress can stop where some guardians ("intervenors") act from internalised norms rather than payoff. — [Hurwicz Nobel lecture 2007](https://www.nobelprize.org/prizes/economic-sciences/2007/hurwicz/lecture/); [Hurwicz 2008, AER](https://doi.org/10.1257/aer.98.3.577)

**Formal theories of hierarchy shape**
- [Theorem-style model] Sah and Stiglitz: a **hierarchy** (serial approval) rejects more good projects; a **polyarchy** (any one unit may approve) accepts more bad ones. Which is better depends on the project mix and screening quality. Architecture is an error-type trade-off. — [Sah & Stiglitz 1986, AER](https://www.jstor.org/stable/1806069)
- [Theorem-style model] Radner: with processors that each take time per item, efficient networks for aggregating n items are hierarchical trees; minimum delay grows on the order of log n even with unlimited processors, and "reduced" (not strictly balanced) trees are efficient. — [Radner 1993, Econometrica](https://doi.org/10.2307/2951495)
- [Theorem-style model] Bolton and Dewatripont: specialisation lowers per-item processing cost but adds communication cost, and the efficient network is pyramidal. — [Bolton & Dewatripont 1994, QJE](https://doi.org/10.2307/2118349)
- [Theorem-style model] Van Zandt: when the environment changes while information is being processed, larger hierarchies decide on staler data, so real-time processing bounds useful scale and favours decentralised decisions on local information. — [Van Zandt 1999, Review of Economic Studies](https://doi.org/10.1111/1467-937X.00101)
- [Theorem-style model] Garicano: in a **knowledge hierarchy**, front-line workers learn the common problems and pass exceptions upward to successively scarcer experts. Cheaper communication widens spans and centralises; cheaper knowledge acquisition pushes decisions down. — [Garicano 2000, JPE](https://doi.org/10.1086/317671)
- [Theorem-style model] Control loss: if each level transmits intent with fidelity below one, loss compounds geometrically with depth, which bounds firm size (Williamson); Calvo and Wellisz show the bound depends on how supervision is modelled. — [Williamson 1967, JPE](https://doi.org/10.1086/259258); [Calvo & Wellisz 1978, JPE](https://doi.org/10.1086/260719)

### Inferences
- **Depth**: processing models make depth roughly logarithmic in size. With span s, depth for N actors is about log N / log s. For N = 10^10: span 7 gives about 12 levels, span 10 gives 10, span 30 gives about 7, span 150 gives about 4.6. (Arithmetic on the models above, not a cited result.)
- **Span**: no universal optimum. Span rises with cheaper communication and more routine work, and falls with exception rate and need for supervision (Garicano; Calvo-Wellisz).
- **Delegation**: the impossibility results mean every delegation leaves a residue (information rent, residual loss or inefficiency). A foundation therefore needs explicit primitives for residual control (who decides what the contract does not cover) and residual claim (who absorbs the surplus or deficit). Holmström 1982 says a team cannot close its own accounts efficiently; some party outside the team must hold the residual. That is a formal argument for nesting.
- **Architecture is plural**: Sah-Stiglitz plus Hayek imply a mix of hierarchy (for error-averse, aggregating decisions) and polyarchy/market (for exploratory, locally informed decisions), which matches Ostrom's polycentricity.
- **Guardians**: Hurwicz implies governance cannot be closed purely by incentives; the regress ends in norms, mutual monitoring cycles, or an external anchor.

### Gaps
- These models are stylised (homogeneous agents, single task types). I found no theorem giving optimal depth or span for heterogeneous human-plus-AI organisations.
- The Marschak-Radner 1972 monograph was not directly sourced; the 1955 paper is cited instead.
- Whether AI agents with cheap communication and cheap knowledge shift Garicano's trade-off toward flatter or more centralised forms is an open inference, not an established result.

## 3. Classical and modern organisation theory; self-management systems and how they performed

### Takeaway
Organisation theory converges on one explanation: structure exists to cope with bounded rationality and interdependence. It offers robust regularities (hierarchy is near-universal, spans are small, levels are few) but few formal laws. Self-management systems replace the manager hierarchy with a hierarchy of rules and nested circles; evidence of success is real but concentrated in a handful of firms, with visible reversals.

### Cited Findings
**Classical**
- [Framework] Weber's bureaucracy: fixed jurisdictional competences, office hierarchy, written files, expert training, impersonal rules, separation of office from person. Authority is legal-rational: it attaches to the office, not the holder. — [SEP, Max Weber](https://plato.stanford.edu/entries/weber/)
- [Framework] Simon: bounded rationality and satisficing; the organisation supplies decision premises, so that individuals can decide within a tractable sub-problem. — [Simon Nobel lecture 1978](https://www.nobelprize.org/prizes/economic-sciences/1978/simon/lecture/)
- [Framework] Galbraith: structure must match information-processing capacity to task uncertainty. Either reduce the need (slack resources, self-contained units) or raise capacity (vertical information systems, lateral relations). — [Galbraith 1974, Interfaces](https://doi.org/10.1287/inte.4.3.28)
- [Framework] Thompson: three interdependence types with matching coordination modes: pooled (standardisation), sequential (planning), reciprocal (mutual adjustment); group the most costly interdependencies first. — [James D. Thompson, Wikipedia summary of Organizations in Action 1967](https://en.wikipedia.org/wiki/James_D._Thompson)
- [Framework] Mintzberg: five parts (strategic apex, middle line, operating core, technostructure, support staff), five coordinating mechanisms, five configurations (simple structure, machine bureaucracy, professional bureaucracy, divisionalised form, adhocracy). — [Mintzberg 1980, Management Science](https://doi.org/10.1287/mnsc.26.3.322)
- [Framework] Conway's law: organisations produce designs that copy their communication structures. — [Conway 1968](http://www.melconway.com/Home/Committees_Paper.html)
- [Empirical] The "mirroring hypothesis" was supported in matched product pairs: loosely coupled (open-source) organisations produced markedly more modular software than tightly coupled firms. — [MacCormack, Baldwin & Rusnak 2012, Research Policy](https://doi.org/10.1016/j.respol.2012.04.011)
- [Framework] Team Topologies: four team types (stream-aligned, platform, enabling, complicated-subsystem) and three interaction modes (collaboration, X-as-a-service, facilitating), with team cognitive load as the sizing constraint. — [Team Topologies, Key Concepts](https://teamtopologies.com/key-concepts)
- [Framework] Jaques' Requisite Organization: work complexity is measured by **time-span of discretion** (longest task the role holder must complete unsupervised); roles fall into about seven or eight strata with fixed time-span bands; one managerial layer per stratum. — [Requisite organization, Wikipedia](https://en.wikipedia.org/wiki/Requisite_organization)

**Span, depth and group size**
- [Empirical] In large US firms from 1986 to 1998, the number of managers reporting directly to the CEO rose substantially and the number of levels between CEO and division heads fell; firms got flatter, with wider spans at the top. — [Rajan & Wulf 2006, Review of Economics and Statistics](https://doi.org/10.1162/rest.88.4.759)
- [Empirical] Dunbar's number (about 150) was extrapolated from a regression of primate group size on neocortex ratio. — [Dunbar 1992, Journal of Human Evolution](https://doi.org/10.1016/0047-2484(92)90081-J)
- [Empirical, critique] Lindenfors, Wartel and Lind re-ran the analysis with updated data and methods: 95% confidence intervals were 4-520 and 2-336 depending on method; they conclude a cognitive limit "cannot be derived in this manner", and note the estimate is of average, not maximum, group size. **[searched 2026-11]** — [Lindenfors, Wartel & Lind 2021, Biology Letters](https://pmc.ncbi.nlm.nih.gov/articles/PMC8103230)
- [Empirical] Human groupings show a discrete hierarchy of sizes with a scaling ratio near 3 (about 5, 15, 50, 150, 500, 1500); hunter-gatherer societies show nested groups with a branching ratio near 4. — [Zhou, Sornette, Hill & Dunbar 2005, Proc. R. Soc. B](https://doi.org/10.1098/rspb.2004.2970); [Hamilton et al. 2007, Proc. R. Soc. B](https://doi.org/10.1098/rspb.2007.0564)
- [Empirical] US firm sizes follow a Zipf distribution (power law with exponent near 1): very many tiny firms, a few giants, no characteristic size. — [Axtell 2001, Science](https://doi.org/10.1126/science.1062081)

**Self-management systems**
- [Framework] The Holacracy Constitution v5.0 has five articles (organisational structure, rules of cooperation, tactical meetings, distributed authority, governance process). A **role** is defined by purpose, domains and accountabilities; a **circle** is a role broken into sub-roles, so circles nest recursively; governance can be changed only through the governance process (proposal, clarifying questions, reactions, objections, integration). Version 5 allows article-by-article adoption. **[searched 2026-11]** — [Holacracy Constitution 5.0](https://www.holacracy.org/constitution/5-0/)
- [Framework] Lee and Edmondson define self-managing organisations as those that radically and formally decentralise authority throughout the organisation, and stress that they remove the manager-subordinate reporting relation, not structure or rules. — [Lee & Edmondson 2017, Research in Organizational Behavior](https://doi.org/10.1016/j.riob.2017.10.002)
- [Empirical, case] Zappos rolled out Holacracy from 2013 (about 460 circles and 4,700 roles); the 2015 "commit or leave" offer led to departures reported between 14% and 18% of staff, and by 2020 reporting described Zappos as quietly moving away from it. Medium abandoned it in 2016. **[searched 2026-11]** — [Quartz 2020](https://qz.com/work/1776841/zappos-has-quietly-backed-away-from-holacracy/); [Fortune 2016](https://fortune.com/2016/03/04/management-changes-at-medium/)
- [Empirical, case] HBR's assessment: self-management works best adopted in parts, where adaptability matters more than reliability; governance overhead (role proliferation, meeting load) is the main cost. — [Bernstein, Bunch, Canner & Lee 2016, HBR](https://hbr.org/2016/07/beyond-the-holacracy-hype)
- [Empirical, case] Haier's rendanheyi split the company into more than 4,000 microenterprises, most with 10-15 employees, linked by internal contracts and platforms. An earlier Haier-authored source says "more than 2,000". **[searched 2026-11; sources disagree on the count]** — [Hamel & Zanini 2018, HBR](https://hbr.org/2018/11/the-end-of-bureaucracy); [Zhang Ruimin 2018, strategy+business](https://www.strategy-business.com/article/Why-Haier-Is-Reorganizing-Itself-around-the-Internet-of-Things)
- [Framework, case] Buurtzorg organises district nursing in small self-managing teams with no middle management, supported by coaches and a small back office. — [Buurtzorg, the Buurtzorg model](https://www.buurtzorg.com/about-us/buurtzorgmodel/)

### Inferences
- Every "post-hierarchical" system examined keeps a containment hierarchy of units (circles, microenterprises, teams) and replaces personal authority with rule-based authority plus an explicit rule-change process. The recursion unit is "a role that may contain roles". This is Weber's office/person separation pushed further, not the abolition of hierarchy.
- Small-unit sizes cluster at 5-15 (Haier microenterprises, typical teams, the first Zhou et al. layers). Branching ratios of 3-4 in natural human groupings are well below managerial spans of 7-10, suggesting formal structure roughly doubles the span that unassisted social cognition supports.
- Dunbar's 150 should not be used as a design constant; the supportable claim is "layered groupings with a roughly constant scaling ratio", not a specific limit.
- Evidence on self-management is case-based with survivorship bias; no controlled comparison exists at scale.

### Gaps
- Buurtzorg's often-quoted figures (about 10,000 nurses, about 850 teams of up to 12, a back office of about 50) and W. L. Gore's lattice structure with plants capped near 150-200 people were not verified this session and are omitted from the findings.
- Sociocracy (consent decision-making, double-linking of circles) and Laloux's "Reinventing Organizations" were not separately sourced; Holacracy derives from sociocracy.
- Jaques' strata have contested empirical support; I did not locate a strong independent validation.
- Exact Rajan-Wulf magnitudes should be taken from the paper before quoting.

## 4. Formal enterprise and business ontologies

### Takeaway
Two small, formal cores stand out. DEMO reduces an enterprise to actors making commitments in a universal transaction pattern; REA reduces its economics to resources, events and agents linked by duality. Accounting adds a genuine conservation law. The broad frameworks (Zachman, TOGAF, ArchiMate, FIBO) are vocabularies, not calculi.

### Cited Findings
- [Framework, formal] DEMO / PSI theory: an enterprise is a network of **actor roles** that enter into and comply with **commitments** through **coordination acts**; production acts create the facts. The basic transaction pattern is **request, promise, (execute), state, accept**; the standard pattern adds decline and reject; the complete pattern adds four revocations. Each transaction kind has one executor role, and transactions nest into trees that mirror product structure. — [Dietz 2006, Enterprise Ontology, Springer](https://doi.org/10.1007/3-540-33149-2); [Dietz & Mulder 2020, Enterprise Ontology: A Human-Centric Approach](https://doi.org/10.1007/978-3-030-38854-6)
- [Framework] DEMO separates three human abilities (performa: commitments and original facts; informa: reasoning and computation; forma: storing and transmitting documents) and claims the performa-level "essential model" is coherent, comprehensive, consistent, concise and essential, and far smaller than conventional process models. — [Dietz 2006](https://doi.org/10.1007/3-540-33149-2)
- [Framework] The Language/Action Perspective treats organisations as networks of conversations for action, built on speech acts (request, promise, assertion, declaration). — [Winograd 1987, Human-Computer Interaction](https://doi.org/10.1207/s15327051hci0301_2); [SEP, Speech Acts](https://plato.stanford.edu/entries/speech-acts/)
- [Framework, formal] REA: accounting phenomena are modelled as economic **Resources**, **Events** and **Agents**; every increment event is paired with a decrement event by a **duality** relation (give/take); later extended with commitments, contracts and types (policy level). It was adopted as the Open-edi business transaction ontology in ISO/IEC 15944-4. — [McCarthy 1982, The Accounting Review](https://www.jstor.org/stable/246878); [REA, Wikipedia](https://en.wikipedia.org/wiki/Resources,_Events,_Agents)
- [Theorem] Double-entry bookkeeping has an exact algebraic form: accounts are ordered pairs (debit, credit) forming the "group of differences" (Pacioli group); a transaction is a zero-sum vector, so the accounting equation is an invariant. — [Ellerman 2014, arXiv:1407.1898](https://arxiv.org/abs/1407.1898)
- [Framework] Triple-entry accounting adds a third, cryptographically signed shared receipt to each party's two entries, making the transaction record common to both parties and a third party. — [Grigg 2005, Triple Entry Accounting](https://iang.org/papers/triple_entry.html)
- [Framework, formal] UFO is an axiomatised foundational ontology in three layers: UFO-A (endurants: kinds, roles, phases, relators), UFO-B (events), UFO-C (social and intentional entities: agents, commitments, claims, normative descriptions). A **relator** is the truth-maker of a material relation (e.g. an employment or a contract). gUFO is a lightweight OWL implementation. — [Guizzardi et al. 2022, Applied Ontology](https://doi.org/10.3233/AO-210256); [gUFO](https://nemo-ufes.github.io/gufo/)
- [Framework] Zachman: a 6x6 classification (what, how, where, who, when, why by stakeholder perspective). — [Zachman 1987, IBM Systems Journal](https://doi.org/10.1147/sj.263.0276)
- [Framework] ArchiMate: layered (business, application, technology) with active structure, behaviour and passive structure elements and a fixed relationship set. — [ArchiMate 3 specification](https://pubs.opengroup.org/architecture/archimate3-doc/)
- [Framework] FIBO: an OWL ontology of financial industry concepts including legal entities, ownership and control, and contracts. — [EDM Council, FIBO](https://spec.edmcouncil.org/fibo/)
- [Framework] Bunge-Wand-Weber: a model of information systems from Bunge's ontology (things, properties, states, events, transformations, lawful state spaces, systems and couplings), used to judge modelling grammars for construct deficit and overload. — [Wand & Weber 1990, IEEE TSE](https://doi.org/10.1109/32.60316)
- [Framework] BFO is a small top-level ontology (continuants vs occurrents), standardised as ISO/IEC 21838-2. — [Basic Formal Ontology](https://basic-formal-ontology.org/)

### Inferences
- DEMO and REA are complementary and jointly near-minimal. DEMO covers the **deontic/communicative** side (who commits to whom, with what state machine); REA covers the **economic/resource** side (what is conserved and exchanged). REA's later commitment and contract constructs meet DEMO's promise.
- Double-entry is the one true conservation law in this strand: each transaction sums to zero across accounts. It composes trivially (consolidation of sub-entity ledgers is addition with elimination of inter-entity balances), which is why legal-entity recursion is tractable in practice.
- DEMO's four basic acts are Searle's directive (request), commissive (promise), assertive (state) and declaration (accept): the transaction pattern is a protocol built from speech-act types.
- The enterprise-architecture frameworks give classification, not composition laws.

### Gaps
- DEMO's completeness claim and model-size reduction rest on case studies by its own community; I found no independent large-sample validation. Uptake is concentrated in a small academic and consulting community.
- The current edition and number of ISO/IEC 15944-4 were not verified against iso.org this session.
- No source found that formally unifies DEMO transactions, REA duality and double-entry in a single calculus.

## 5. Social ontology and law

### Takeaway
Law is the one field that has actually run a recursive, self-modifying governance system on billions of people. Its recurring structure is: constitutive rules create statuses, statuses carry Hohfeldian positions, powers are delegated downward under constraints, and secondary rules govern recognition, change and adjudication. Group agency is formally possible, but subject to an aggregation impossibility.

### Cited Findings
**Social ontology**
- [Framework] Searle: institutional facts are created by constitutive rules of the form "**X counts as Y in context C**"; in the later version, by status function declarations. Status functions carry **deontic powers** (rights, duties, permissions, authorisations) that give desire-independent reasons for action. Criticisms include "freestanding Y terms" (e.g. corporations or electronic money with no underlying X) and Epstein's distinction between grounding and anchoring social facts. — [SEP, Social Ontology](https://plato.stanford.edu/entries/social-ontology/)
- [Formal] Jones and Sergot formalised "counts as" as a conditional connective relative to an institution, combined with action and deontic operators, giving a logic of **institutionalised power**. — [Jones & Sergot 1996, Logic Journal of the IGPL](https://doi.org/10.1093/jigpal/4.3.427)
- [Framework] Gilbert: a plural subject is constituted by **joint commitment**, which creates directed obligations among the parties and cannot be rescinded unilaterally. Tuomela: we-mode vs I-mode intentions, with group ethos and authorised operative members. Bratman offers a reductive account via interlocking individual intentions. — [SEP, Collective Intentionality](https://plato.stanford.edu/entries/collective-intentionality/)
- [Theorem] Discursive dilemma / judgment aggregation: no aggregation function over logically connected propositions satisfies universal domain, collective rationality, anonymity and systematicity (proposition-wise majority voting can yield an inconsistent collective judgment set). — [List & Pettit 2002, Economics and Philosophy](https://doi.org/10.1017/S0266267102001098)
- [Framework] List and Pettit: a group is an agent when it has representational and motivational states and acts on them rationally; to stay rational it must relax an aggregation condition (e.g. premise-based procedures), which makes group attitudes not a simple function of member attitudes. Such groups can be held responsible. — [List & Pettit 2011, Group Agency, OUP](https://doi.org/10.1093/acprof:oso/9780199591565.001.0001)

**Jurisprudence**
- [Framework, formalisable] Hohfeld's eight jural positions in two squares. First order: **claim-right / duty**, **privilege (liberty) / no-right**. Second order: **power / liability**, **immunity / disability**. Each pair is correlative (one party's position entails the other's); powers are abilities to change first-order positions. — [Hohfeld 1913, Yale Law Journal](https://doi.org/10.2307/785533); [SEP, Rights](https://plato.stanford.edu/entries/rights/)
- [Framework] Hart: a legal system is the union of **primary rules** (duty-imposing) and **secondary rules**: the **rule of recognition** (what counts as valid law, curing uncertainty), **rules of change** (curing static rules) and **rules of adjudication** (curing inefficient enforcement). The rule of recognition exists as a social practice of officials. — [SEP, Legal Positivism](https://plato.stanford.edu/entries/legal-positivism/)
- [Framework] Kelsen: norms form a hierarchy (Stufenbau) in which each norm is valid because created under a higher norm, ending in a presupposed basic norm (Grundnorm). — [SEP, The Pure Theory of Law](https://plato.stanford.edu/entries/lawphil-theory/)

**Constitutional and administrative design**
- [Framework] Madison, Federalist 51: "Ambition must be made to counteract ambition"; government must be obliged to control itself through separated, mutually checking departments and a compound (federal) republic. — [Federalist No. 51](https://avalon.law.yale.edu/18th_century/fed51.asp)
- [Framework, legal text] Subsidiarity in EU law: in areas outside exclusive competence, the Union acts only if and in so far as objectives cannot be sufficiently achieved by Member States at central, regional or local level. — [Treaty on European Union, Article 5(3)](https://eur-lex.europa.eu/legal-content/EN/TXT/?uri=CELEX:12016M005)
- [Framework] Subsidiarity in Catholic social teaching: it is "an injustice ... to assign to a greater and higher association what lesser and subordinate organizations can do". — [Pius XI 1931, Quadragesimo Anno, para. 79](https://www.vatican.va/content/pius-xi/en/encyclicals/documents/hf_p-xi_enc_19310515_quadragesimo-anno.html)
- [Framework, legal doctrine] Non-delegation: a legislature may not hand its legislative power to another body without an intelligible principle. The private-law analogue is *delegata potestas non potest delegari* (a delegate cannot sub-delegate without authority). — [Cornell LII, Nondelegation doctrine](https://www.law.cornell.edu/wex/nondelegation_doctrine); [Delegata potestas non potest delegari, Wikipedia](https://en.wikipedia.org/wiki/Delegata_potestas_non_potest_delegari)

**Corporate law and the legal entity**
- [Framework] Hansmann and Kraakman: the essential contribution of organisational law is **asset partitioning**. "Affirmative" partitioning (entity shielding) protects firm assets from owners' personal creditors; "defensive" partitioning (limited liability) protects owners from firm creditors. Entity shielding cannot be replicated by contract alone. — [Hansmann & Kraakman 2000, Yale Law Journal](https://doi.org/10.2307/797521)
- [Empirical] Legal entities nest (parents, subsidiaries, holding structures). The Global LEI System identifies them and records direct and ultimate parent relationships. Active LEIs passed 3.02 million at end Q1 2026 and exceeded 3.1 million at end Q2 2026; total including lapsed/retired exceeded 3.35 million. **[searched 2026-11]** — [GLEIF, Q1 2026](https://www.gleif.org/en/newsroom/blog/the-lei-in-numbers-active-lei-population-surpasses-3-million-in-q1-2026); [GLEIF, Q2 2026](https://www.gleif.org/en/newsroom/blog/the-lei-in-numbers-q2-2026-signals-growing-regional-momentum-for-the-lei)

### Inferences
- **Hohfeld is a type system for relations between two parties.** First-order positions are constraints on actions; second-order positions are capabilities to rewrite first-order positions. This is the cleanest available primitive for "permission plus delegation", and it is already two-level reflective.
- **Hart's secondary rules are the reflective-governance kernel**: recognition (what is a valid rule), change (how rules are changed), adjudication (who decides disputes). Ostrom's constitutional level, Holacracy's governance process and Kelsen's Stufenbau are instances of it.
- Kelsen's Grundnorm, Hart's rule of recognition as social fact, and Hurwicz's intervenors are three statements of the same point: the validity chain bottoms out in acceptance, not in a further rule. A formal foundation must take a root of trust as given.
- The **legal entity** is law's recursive unit: a bundle of (a) a separate asset pool, (b) capacity to hold Hohfeldian positions and contract, (c) a governance rule set, (d) agents authorised to act for it, and (e) the capacity to own other entities. A system of entities is again an entity (the group or holding).
- Subsidiarity is a default allocation rule for a nested system: place each power at the lowest level able to exercise it, with upward escalation requiring justification. It is the normative counterpart of Hayek's knowledge argument and Garicano's management by exception.
- The discursive dilemma means a group agent's stance cannot in general be computed by aggregating members' stances proposition by proposition. A compositional foundation must give each unit its own decision procedure and state, which is how corporate law treats boards.

### Gaps
- Hohfeld formalisations (Kanger-Lindahl normative positions, Sergot) exist but were not individually sourced here.
- The LEI covers only entities that need one (mostly financial-market participants); it is a small fraction of all legal entities. I found no authoritative global count of all registered legal entities.
- Fiduciary duty and agency law (actual vs apparent authority) are covered only through the general sources above.

## 6. Normative multi-agent systems and computational organisation models

### Takeaway
The MAS community has already built executable organisation models from the same primitives (role, group, norm, commitment, protocol). They are the nearest thing to running code for this strand. Uptake outside academia is small, and they were designed for 10^1 to 10^3 agents.

### Cited Findings
- [Formal, software] **Electronic institutions** (Esteva, Sierra, Noriega and colleagues): an institution is specified by a dialogic framework (roles, ontology, illocutions), **scenes** (interaction protocols as state machines), a **performative structure** (a network of scenes with role-flow between them) and norms; ISLANDER is the specification editor and AMELI the runtime that enforces the rules through governors. — [Esteva, de la Cruz & Sierra 2002, AAMAS (ISLANDER)](https://doi.org/10.1145/545056.545069)
- [Formal, software] **MOISE+** separates three specifications: **structural** (roles, groups, links of authority/communication/acquaintance, cardinalities, compatibility), **functional** (social schemes: goal decomposition trees and missions) and **deontic/normative** (permissions and obligations tying roles to missions). — [Hübner, Sichman & Boissier 2007, IJAOSE](https://doi.org/10.1504/IJAOSE.2007.016266)
- [Software] **JaCaMo** combines Jason (BDI agents), CArtAgO (environment artifacts) and Moise (organisation) into one programming platform. — [Boissier et al. 2013, Science of Computer Programming](https://doi.org/10.1016/j.scico.2011.10.004)
- [Framework, formal] **OperA** (V. Dignum) has an organisational model (roles, objectives, norms, scene scripts), a social model (role-enactment contracts between agents and the organisation) and an interaction model (interaction contracts); it keeps agent autonomy by contracting rather than programming behaviour. — [Dignum 2004, PhD thesis, Utrecht](https://dspace.library.uu.nl/handle/1874/890)
- [Framework] **AGR** (Ferber, Gutknecht, Michel): three primitives only (agent, group, role); agents play roles in groups, groups may overlap, and nothing is assumed about agents' internals. — [Ferber, Gutknecht & Michel 2004, AOSE](https://doi.org/10.1007/978-3-540-24620-6_15)
- [Formal] **Social commitments** (Singh): C(debtor, creditor, antecedent, consequent) with operations create, discharge, cancel, release, **delegate** and **assign**; commitments are public and directed, unlike private intentions, and give protocols their meaning. — [Singh 1999, Artificial Intelligence and Law](https://doi.org/10.1023/A:1008319631231)
- [Protocol] **Contract Net** (Smith): task announcement, bids, award; a manager/contractor relation that can recurse, since a contractor may subcontract. — [Smith 1980, IEEE Transactions on Computers](https://doi.org/10.1109/TC.1980.1675516)
- [Survey] Horling and Lesser compare organisational paradigms (hierarchies, **holarchies**, coalitions, teams, congregations, societies, federations, markets, matrix organisations), concluding that no single form suits all settings, and each has characteristic strengths and costs. — [Horling & Lesser 2004, Knowledge Engineering Review](https://doi.org/10.1017/S0269888905000317)

### Inferences
- The MAS models independently rediscover the social-science primitives: scene = Ostrom's action situation; performative structure = network of action situations; MOISE's structural/functional/normative split = Mintzberg's structure, Thompson's workflow and Hohfeld/ADICO norms; Singh's commitment = DEMO's promise = Gilbert's directed obligation; Contract Net = a DEMO transaction with competitive selection.
- Singh's commitment with delegate/assign operations is the smallest formal object that captures delegation with accountability: the debtor changes but the creditor's claim persists.
- The models differ mainly on enforcement: regimentation (violations made impossible, as in AMELI) versus sanctioning (violations detected and punished). This is the same choice as hard constraints versus Ostrom's graduated sanctions.

### Gaps
- No evidence found of these platforms running at more than modest agent counts or in large production deployments; uptake appears academic.
- Castelfranchi's commitment and dependence theory, and OMNI as the merger of OperA and HarmonIA, were not individually sourced.
- Whether LLM-agent frameworks since 2023 have adopted these organisation models was left to the AI-governance strand.

## 7. Scale facts for the 10^10 target

### Takeaway
Humanity is already a recursive system of about 8.3 billion people, organised through roughly 200 states, hundreds of millions of firms and millions of formally identified legal entities. Real hierarchies are shallow (about 5 territorial levels for 1.4 billion people) with spans near 10, and people belong to several overlapping hierarchies at once.

### Cited Findings
- World population is about 8.3 billion in mid-2026 under the UN World Population Prospects 2024 medium variant (figure taken from secondary trackers citing the UN). **[searched 2026-11]** — [UN World Population Prospects](https://population.un.org/wpp/); [StatisticsTimes citing WPP 2024](https://statisticstimes.com/demographics/world-population.php)
- The UN has 193 member states. — [United Nations, Member States](https://www.un.org/en/about-us/member-states)
- Formal micro, small and medium enterprises: a World Bank analysis of 2019 data counted over 322 million, employing more than 705 million people. Including informal firms, secondary sources give roughly 400-500 million (weaker sourcing). **[searched 2026-11]** — [World Bank, MSME Economic Indicators Analysis Note](https://documents1.worldbank.org/curated/en/873301627470308867/pdf/Micro-Small-and-Medium-Enterprises-Economic-Indicators-MSME-EI-Analysis-Note.pdf)
- Legal entities with an active LEI: over 3.1 million at mid-2026. **[searched 2026-11]** — [GLEIF, Q2 2026](https://www.gleif.org/en/newsroom/blog/the-lei-in-numbers-q2-2026-signals-growing-regional-momentum-for-the-lei)
- China's territorial hierarchy (Ministry of Civil Affairs data for 2022, via a Hong Kong Education Bureau sheet): about 333 prefecture-level, 2,843 county-level and 38,602 township-level divisions, beneath 34 provincial-level units, with villages below. **[searched 2026-11]** — [HK Education Bureau, Administrative Division info sheet](https://www.edb.gov.hk/attachment/en/curriculum-development/kla/pshe/national-geography/national-geography-resource-portal/Info_Sheet_01-administrative_division-eng.pdf); [Administrative divisions of China, Wikipedia](https://en.wikipedia.org/wiki/Administrative_divisions_of_China)
- Chinese Communist Party: 101.29 million members and over 5.43 million primary-level organisations at end 2025 (Organisation Department report, 30 June 2026). **[searched 2026-11]** — [CGTN 2026](https://news.cgtn.com/news/2026-06-30/CPC-membership-reaches-101-286-mln-1OoiX3xQAog/p.html)
- Walmart: about 2.1 million associates worldwide (about 1.6 million in the US), fiscal 2026 revenue $713 billion. **[searched 2026-11]** — [Walmart 2026 Annual Report release](https://corporate.walmart.com/news/2026/04/23/walmart-releases-2026-annual-report-and-proxy-statement)
- Indian Railways: over 1.23 million employees (press report of government data, March 2025). **[searched 2026-11]** — [Free Press Journal 2025](https://www.freepressjournal.in/india/indian-railways-sees-steady-rise-in-women-participation-workforce-share-grows-to-82)
- Catholic Church: about 1.406 billion Catholics, 3,041 ecclesiastical jurisdictions and 406,996 priests at end 2023. **[searched 2026-11]** — [Vatican News 2025](https://www.vaticannews.va/en/vatican-city/news/2025-03/pontifical-yearbook-2025-priests-religious-statistics.html)
- India has on the order of 250,000 gram panchayats (village councils); district counts in the sources found were dated and conflicting (640 to 686). **[searched 2026-11; weak sourcing]** — [Administrative divisions of India, Wikipedia](https://en.wikipedia.org/wiki/Administrative_divisions_of_India)

### Inferences
- **China's spans** (derived from the counts above): 34 provinces, then about 9.8 prefectures per province, 8.5 counties per prefecture, 13.6 townships per county. Five formal levels (centre, province, prefecture, county, township) plus villages govern about 1.4 billion people; average township population is about 36,000.
- **CCP**: 101.29 million / 5.43 million gives about 19 members per primary organisation. The base unit of a 10^8-member organisation is team-sized.
- **Catholic Church**: 1.406 billion / 3,041 is about 460,000 Catholics per jurisdiction and about 3,450 per priest. The formal chain (pope, bishop, parish priest) is three levels with a top span near 3,000: an extremely flat hierarchy held together by doctrine (shared rules) rather than supervision.
- **Firms**: 8.3 billion people over 322-500 million enterprises gives about 17-26 people per enterprise on average, but the distribution is Zipf, so the median is far smaller.
- **Depth for 10^10**: at the spans actually observed (about 8-14), 10^10 actors need about 9-11 levels if strictly tree-shaped. Observed systems use fewer formal levels because the leaf units are large (townships, parishes) and because people sit in several overlapping trees (state, employer, party, church, family). Real structure is a polycentric overlay of shallow hierarchies, not one deep tree.
- Fewer than 200 top-level sovereign units, each with 3-6 levels, is the empirical shape of planetary governance; above the state there is no hierarchy, only treaty (contract) and polycentric bodies.

### Gaps
- PLA active strength (commonly given as about 2 million by IISS) was not verified this session.
- No authoritative global count of all registered legal entities (as opposed to LEI holders or MSMEs) was found.
- The population figure is from secondary trackers of UN WPP 2024; I did not open the UN data portal.
- Current official counts for India's districts, blocks and panchayats were not obtained; depth for India (union, state, district, block/tehsil, panchayat) is stated from general knowledge without a verified source.
- Catholic parish and bishop counts were not found in the results.

## 8. What minimal primitives recur across these theories

### Takeaway
About ten primitives recur in every tradition examined, and they collapse to a core of five: actor, role (status), norm (Hohfeldian relation), commitment (transaction), and the reflective triple of recognition, change and adjudication. No tradition supplies a full composition calculus; each supplies one piece.

### Cited Findings
- Roles as positions distinct from occupants appear as Ostrom's positions, Weber's office, Holacracy's role, AGR's and MOISE's role, DEMO's actor role and Searle's status function. — [Ostrom 2011](https://doi.org/10.1111/j.1541-0072.2010.00394.x); [SEP Weber](https://plato.stanford.edu/entries/weber/); [Holacracy Constitution 5.0](https://www.holacracy.org/constitution/5-0/); [Ferber et al. 2004](https://doi.org/10.1007/978-3-540-24620-6_15); [Dietz 2006](https://doi.org/10.1007/3-540-33149-2); [SEP Social Ontology](https://plato.stanford.edu/entries/social-ontology/)
- Norms with deontic force and a sanction clause appear as ADICO/IG 2.0 statements, Hohfeld's positions, MOISE's deontic specification and electronic-institution norms. — [Crawford & Ostrom 1995](https://doi.org/10.2307/2082975); [Hohfeld 1913](https://doi.org/10.2307/785533); [Hübner et al. 2007](https://doi.org/10.1504/IJAOSE.2007.016266)
- Directed commitment between two parties appears as DEMO's promise within a transaction, Singh's social commitment, Gilbert's joint commitment, REA's commitment and the contract of economics. — [Dietz 2006](https://doi.org/10.1007/3-540-33149-2); [Singh 1999](https://doi.org/10.1023/A:1008319631231); [SEP Collective Intentionality](https://plato.stanford.edu/entries/collective-intentionality/); [McCarthy 1982](https://www.jstor.org/stable/246878)
- Conserved resources moved by paired events appear in REA duality and double-entry bookkeeping. — [McCarthy 1982](https://www.jstor.org/stable/246878); [Ellerman 2014](https://arxiv.org/abs/1407.1898)
- Rules about rules appear as Hart's secondary rules, Ostrom's collective-choice and constitutional levels, Kelsen's norm hierarchy and Holacracy's governance process. — [SEP Legal Positivism](https://plato.stanford.edu/entries/legal-positivism/); [Ostrom 2011](https://doi.org/10.1111/j.1541-0072.2010.00394.x); [SEP Pure Theory of Law](https://plato.stanford.edu/entries/lawphil-theory/); [Holacracy Constitution 5.0](https://www.holacracy.org/constitution/5-0/)
- Monitoring, sanctioning and dispute resolution are required by Ostrom's principles 4-6, Hart's rule of adjudication and Hurwicz's enforcement argument. — [Ostrom Nobel lecture 2009](https://www.nobelprize.org/prizes/economic-sciences/2009/ostrom/lecture/); [Hurwicz 2008](https://doi.org/10.1257/aer.98.3.577)
- Nesting of units appears as Ostrom's nested enterprises, Holacracy circles, legal-entity ownership and subsidiarity. — [Ostrom Nobel lecture 2009](https://www.nobelprize.org/prizes/economic-sciences/2009/ostrom/lecture/); [Hansmann & Kraakman 2000](https://doi.org/10.2307/797521); [TEU Art. 5(3)](https://eur-lex.europa.eu/legal-content/EN/TXT/?uri=CELEX:12016M005)

### Inferences
**Comparison of recurring primitives** (synthesis; the mapping is mine, the components are from the sources above)

| Primitive | Ostrom / IG 2.0 | Economics | Organisation theory / self-management | DEMO / REA / accounting | Law / social ontology | Normative MAS |
|---|---|---|---|---|---|---|
| Actor (bearer of agency) | participant | agent, principal | member, employee | subject; economic agent | natural or legal person; group agent | agent |
| Role / status | position; Attribute | job, office | office (Weber); role (Holacracy) | actor role | status function (Y term) | role |
| Boundary / membership | boundary rules; principle 1 | firm boundary (Coase, GHM) | circle, unit, team | scope of the enterprise; entity | legal entity; citizenship; jurisdiction | group; scene entry |
| Norm (deontic relation) | Deontic + aIm + Conditions | contract terms | policy, accountability | business rules; action rules | Hohfeld's eight positions | obligation, permission, prohibition |
| Commitment / promise | (implicit in agreements) | contract | accountability | request-promise-state-accept; REA commitment | joint commitment; contract | social commitment |
| Resource + event (account) | payoff rules; resource units | assets; residual claim | budget | REA resource/event; ledger entry | property; asset partition | artifact; resource |
| Decision / aggregation | aggregation rules | mechanism; voting rule | consent process; authority | (outside scope) | voting; board procedure | scene protocol |
| Delegation + accountability | (via levels) | principal-agent; authority | lead link; manager; time-span | executor vs initiator of a transaction | power/liability; agency; sub-delegation | delegate/assign of commitments |
| Monitoring, sanction, dispute resolution | principles 4, 5, 6; Or else | monitoring; guardians | tension processing; escalation | decline/reject/revoke; audit | adjudication; courts | governors; sanctions |
| Rule-change rule (reflection) | collective-choice and constitutional levels | mechanism choice | governance process | (weak) | rules of recognition and change; Grundnorm | (weak; norm-change research) |
| Nesting / recursion | nested enterprises; polycentricity | hierarchy models | circles in circles; microenterprises | transaction trees; consolidation | entity owns entity; federalism; subsidiarity | holons; sub-contracting |

- **Smallest recurring set (proposed):**
  1. **Actor**: anything that can bear commitments, human, artificial or a group that has its own decision procedure.
  2. **Role (status)**: a position created by a constitutive rule ("X counts as Y in C"), separable from its occupant, carrying norms.
  3. **Norm as a Hohfeldian relation**: first-order (claim/duty, liberty/no-right) and second-order (power/liability, immunity/disability). Second-order positions give delegation and rule-change in the same vocabulary.
  4. **Commitment within a transaction protocol**: a directed, public promise with a lifecycle (request, promise, state, accept, plus decline/reject/revoke), delegable and assignable.
  5. **Resource and event under a conservation law**: double-entry/REA duality, giving accounts.
  6. **Boundary (entity)**: a unit with its own members, asset pool, rule set and decision procedure, which can itself be an actor. This is the recursion step, and it is what law calls a legal person.
  7. **Reflective triple**: recognition (what counts as a valid rule or status), change (who may alter rules, by what procedure) and adjudication (who resolves disputes and sanctions), with a root of acceptance that is not itself derived.
- Items 2, 3 and 7 can all be expressed with constitutive rules plus Hohfeldian powers, so a tighter core is **actor, constitutive rule, Hohfeldian relation, commitment, conserved resource**; boundary and reflection are constructions from these (an entity is a status; rule-change is exercise of a power).
- **What the theorems add as constraints on any such foundation:** (a) aggregation is not free: Arrow, Gibbard-Satterthwaite, Sen and List-Pettit mean each unit needs an explicit, imperfect decision procedure; (b) delegation always leaks: Holmström, Myerson-Satterthwaite and Hurwicz mean a residual claimant and residual controller must be named at every level; (c) enforcement is part of the system and its regress ends in an accepted root; (d) depth is logarithmic in size and control loss compounds with depth, so subsidiarity (decide at the lowest capable level, escalate exceptions) is the efficient default; (e) overlapping, polycentric membership is the empirical norm, so composition must be a lattice or graph of entities, not a single tree.
- **What is missing everywhere:** a proof of completeness or minimality of any primitive set; a formal composition operator showing that an entity built from entities satisfies the same laws (law and accounting do this in practice through consolidation and veil rules, not by theorem); and any validated design for 10^10 actors.

### Gaps
- The minimal set above is my synthesis across sources; no single cited source states or proves it.
- I did not find prior work that formally derives Ostrom's seven rule types, Hohfeld's positions and DEMO's transaction pattern from one another, though the correspondences are noted informally in the IG 2.0 and normative-MAS literatures.
- Rule-change (reflection) is weakly treated in the enterprise-ontology and MAS columns; whether DEMO or MOISE can express their own modification is not established here.

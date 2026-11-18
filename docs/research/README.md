# Research the system did on itself

Each file here is an answer the system obtained by its own process: a question from its agenda
(`crates/steward/src/research.rs`) was put to one language model, a second and different model reviewed the
answer, and only an accepted answer was proposed for inclusion, as a change request that was verified in a
sandbox, reviewed again, and applied only on the owner's approval.

They are **working positions the project has adopted, not established findings**. Cite them as "the
project's current position", not as external research.

## Standing reservations

When the change that added the first three answers was reviewed, the reviewing model objected. The owner
approved the change with these reservations kept on record:

- *Managing work beyond software*: "a contract is not executed continuously" was misleading. **Since
  corrected**, by the system itself, as an edit that went through the same control (change
  `21c61784ec07ac38`). Two earlier attempts were rejected by the owner: one would have replaced the whole
  document with the single changed line, and one over-generalised in the other direction.
- *What an operator must see*: "anything that failed stays visible" needs a boundary. Failures stay in the
  record for good; the main screen should surface the ones that still matter.

An earlier answer to "what to track to manage change" was accepted by the research reviewer but its change
request was **rejected by the owner**: it claimed its list was minimal without showing it, and it said an
append-only history prevents quiet rewriting, which it does not without an integrity check such as a hash
chain. That answer is not in this folder.

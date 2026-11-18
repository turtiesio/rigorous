# Adversarial review of rigor

Six findings, all confirmed by running Rust integration tests. Run all six from the repository root with `cargo test --test astra_review -- --ignored`. Each test asserts the documented result and fails on this checkout. The tests are opt-in. The library is unchanged.

### High — Corroborating a refutation discharges a violated maintenance duty

**Reproduce** with `cargo test --test astra_review corroboration -- --ignored`. A duty expires at time 10. A trusted probe refutes its proposition at time 5. At time 11 the duty is still violated. Add a second corroborating probe. Both probes are now in the basis and the proposition is still refuted.

**Observed** `left: Some(Satisfied)`, `right: Some(Violated)`. Epistemic emits an assessment when its basis changes. Norms overwrites the time the refutation was learned with the time of that update. Under §5.1 and P14, the uninterrupted, timely refutation should keep the duty violated.

### High — The adjudicator drops an auditor’s accountability until the auditor becomes the creditor

**Reproduce** with `cargo test --test astra_review auditor_finding -- --ignored`. Add the validated rule “the auditor is accountable for every violated commitment.” Let the agent miss a deadline owed to Alice, inspect the findings, then have Alice assign the overdue commitment to the auditor. Neither premise of the rule changes.

**Observed** auditor accountability before and after assignment was `(false, true)`, expected `(true, true)`. The adjudicator builds role facts only for actors it reaches from the family. That optimization omits the founding auditor from the first evaluation. A verdict can therefore omit a finding required by the enacted rules even after the cascade settles.

### High — A repudiated secondary duty survives withdrawal of its sole basis

**Reproduce** with `cargo test --test astra_review withdrawn_basis -- --ignored`. Refute a maintenance proposition to trigger an audit duty. Have the auditor repudiate the detached duty, then retract the original refutation. The parent returns to `Detached`.

**Observed** child status `left: Some(Violated)`, `right: Some(Void)`. In `Norms::void`, only a child with no recorded operation can become void. Repudiation sets its operation to `Breach`, leaving a live violation on the auditor’s record after its basis disappears, contrary to §5.4.

### High — The steward preserves passing test evidence after a test fixture changes and Cargo fails

**Reproduce** with `cargo test --test astra_review test_fixture_change -- --ignored`. In a temporary crate, a real Cargo test checks `include_str!("data.txt")` against `"ok"`. Run the steward successfully, change only that text file, run Cargo directly, then reopen the steward and run it again. The probe isolates the tests claim by skipping the other checks.

**Observed** Cargo failed, the steward returned `StillEvidenced`, and its conformance result versus Cargo’s success flag was `left: true`, `right: false`. The Rust artifact digest excludes text fixtures. This checkout itself compiles six text or JSONL fixtures into its real-world tests. A fixture-only regression can leave the tests claim accepted.

### Medium — Two passing steward runs manufacture a historical violation

**Reproduce** with `cargo test --test astra_review historical_upkeep -- --ignored`. Run a passing Cargo test on day 100 and verify that day’s upkeep is satisfied. Replace the source with another passing test. Reopen on day 101. Both direct Cargo execution and the steward’s fresh check pass.

**Observed** day 100 status `left: Some(Violated)`, `right: Some(Satisfied)`. Changing the source invalidates the old evidence. The new evidence is dated after the old deadline. Because both days’ commitments refer to the same current proposition, the upkeep report turns yesterday’s observed success into a lapse and creates a spurious remediation duty.

### Medium — A successful transfer destroys one credit while the conservation check passes

**Reproduce** with `cargo test --test astra_review ledger_total -- --ignored`. Found two accounts with balances 1 and `u64::MAX`. Transfer the one credit into the full account. The test verifies that the ledger emits `Transferred` and that `Ledger::conserved()` returns true, then independently sums the final balances in `u128`.

**Observed** total `left: 18446744073709551615`, `right: 18446744073709551616`. The debit succeeds and the credit saturates. The conservation check saturates both the minted amount and its sum, hiding the loss from P5’s invariant check. This requires an extreme but accepted genesis balance sheet.

### Checks that held up

- `incremental_assessment_equals_from_scratch` passed its 40 seeded runs of 60 steps. The corroboration failure above occurs in the commitment consuming the assessment.
- `retracting_the_refutation_reinstates_everything_except_what_already_happened` passed. In that scenario the paid refund stays paid and its duty becomes void. The separate repudiation case above exercises a different child operation.

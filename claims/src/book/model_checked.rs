//! Model-checked: exhaustive exploration of a finite instance.

use crate::claim::{id, Claim, Support};

pub const CLAIMS: &[Claim] = &[
    Claim { id: id("m1"), statement: "Absorbing statuses are never left; `satisfied` is undone only by reassessment or release; nothing is pending after its deadline; a debtor cannot release itself", support: Support::ModelChecked { instance: "one commitment; 12 stimuli; explored to a fixed point", tests: &["model_check_single_commitment"] } },
    Claim { id: id("m2"), statement: "Mutually independent stimuli reach one state under every ordering", support: Support::ModelChecked { instance: "5 stimuli (120 orders) in tests; 6 (720) in the measurements", tests: &["independent_stimuli_commute"] } },
    Claim { id: id("m3"), statement: "The explorer reports a shortest counterexample for a false invariant", support: Support::ModelChecked { instance: "seeded bug", tests: &["model_checker_finds_a_seeded_bug"] } },
    Claim {
        id: id("m4"),
        statement: "The ledger alone, whatever it is told and in whatever order: balances always sum to what was minted; a refused order moves nothing; every order is answered exactly once; only known accounts hold balances. Checked on the concept by itself, with no system assembled around it",
        support: Support::ModelChecked { instance: "3 accounts (one never opened), 2 resources, 12 inputs including zero, more-than-held and the largest number; every reachable state", tests: &["the_laws_hold_in_every_reachable_state_of_a_small_world"] },
    },
];

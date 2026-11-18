//! Proved: machine-checked theorems about the mathematical model (not about the Rust code).

use crate::claim::{id, Axiom, Claim, Support};

pub const CLAIMS: &[Claim] = &[
    Claim { id: id("t1"), statement: "Invariants of the parts, each inductive on its own unit, hold jointly on the parallel composite", support: Support::Proved { theorems: &["inv_par", "inv_run"], axioms: &[] } },
    Claim { id: id("t2"), statement: "Refinement (forward simulation with equal outputs) of each part gives refinement of the composite", support: Support::Proved { theorems: &["sim_par"], axioms: &[] } },
    Claim { id: id("t3"), statement: "Refinement implies equal output traces on every input sequence", support: Support::Proved { theorems: &["sim_trace"], axioms: &[] } },
    Claim { id: id("t4"), statement: "`(A ∥ B) ∥ C` and `A ∥ (B ∥ C)` simulate each other with identical outputs", support: Support::Proved { theorems: &["par_assoc"], axioms: &[Axiom::Propext] } },
    Claim { id: id("t5"), statement: "Two events outside each other's footprint commute on the composite state", support: Support::Proved { theorems: &["commute"], axioms: &[] } },
    Claim { id: id("t6"), statement: "The capability at the end of any attenuating chain grants no action and no limit beyond the root's", support: Support::Proved { theorems: &["no_amplification"], axioms: &[] } },
    Claim { id: id("t7"), statement: "An enabled transfer between distinct holders preserves the total", support: Support::Proved { theorems: &["transfer_conserves"], axioms: &[Axiom::Propext, Axiom::QuotSound] } },
    Claim { id: id("t8"), statement: "Refinement is preserved by the scheduler: two units related by a simulation produce the same log from the same stimuli, with every output fed back as a later input", support: Support::Proved { theorems: &["sim_drive"], axioms: &[Axiom::Propext] } },
    Claim { id: id("t9"), statement: "Under the scheduler, the bracketing of a composition cannot be observed in the log", support: Support::Proved { theorems: &["drive_assoc"], axioms: &[Axiom::Propext] } },
];

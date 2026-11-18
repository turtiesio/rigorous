//! Shared test helpers: a tiny deterministic generator (no dependencies) and unit builders.
#![allow(dead_code, clippy::unwrap_used, clippy::indexing_slicing)]

use rigor::actors::{service, Agent};
use rigor::model::{Amount, Event};
use rigor::scenario::{Cast, AGENT, DEPLOYER};
use rigor::units::{adjudicator::Adjudicator, epistemic::Epistemic, governance::Governance, institution::Institution, ledger::Ledger, norms::Norms};
use rigor::waist::{Cell, Unit};

/// xorshift64*: reproducible from a seed.
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        Self(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1)
    }
    pub fn next(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }
    pub fn below(&mut self, n: usize) -> usize {
        usize::try_from(self.next() % u64::try_from(n).unwrap()).unwrap()
    }
    pub fn below_u64(&mut self, n: u64) -> u64 {
        self.next() % n
    }
    pub fn percent(&mut self, p: u64) -> bool {
        self.next() % 100 < p
    }
    pub fn pick<'a, T>(&mut self, xs: &'a [T]) -> &'a T {
        &xs[self.below(xs.len())]
    }
    pub fn shuffle<T>(&mut self, xs: &mut [T]) {
        for i in (1..xs.len()).rev() {
            xs.swap(i, self.below(i + 1));
        }
    }
}

/// The eight units of the scenario's world, unassembled.
pub fn scenario_parts(cast: Cast) -> Vec<Box<dyn Unit<Event>>> {
    vec![
        Box::new(Cell(Institution::default())),
        Box::new(Cell(Ledger::default())),
        Box::new(Cell(Epistemic::default())),
        Box::new(Cell(Norms::default())),
        Box::new(Cell(Adjudicator::default())),
        Box::new(Cell(Governance::default())),
        Box::new(Cell(Agent::new(AGENT, DEPLOYER, Amount(30), cast.honest_agent))),
        Box::new(service(&DEPLOYER, cast.service_works, cast.service_truthful).unwrap()),
    ]
}

//! What the ledger guarantees, whatever it is told and in whatever order.
//!
//! Each law is a statement about the ledger's own state and words, so it is checked here,
//! in every state a small world can reach, without assembling any system around it.

/// A guarantee of this concept.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Law {
    /// Per resource, balances always sum to what was minted at the founding.
    Conserved,
    /// A refused order moves nothing.
    RefusalMovesNothing,
    /// Every order is answered exactly once: moved, or failed with a reason.
    EveryOrderIsAnswered,
    /// Only known accounts hold balances.
    OnlyAccountsHold,
}

pub const LAWS: [Law; 4] = [Law::Conserved, Law::RefusalMovesNothing, Law::EveryOrderIsAnswered, Law::OnlyAccountsHold];

#[cfg(test)]
mod check {
    use crate::{Heard, Ledger, Order, Quantity, Said, Terms};
    use rigor_waist::{state_digest, Concept, Path, Who};

    struct Small;
    impl Terms for Small {
        type Account = u8;
        type Resource = u8;
        type Context = ();
    }

    /// Every input the environment could send, over three accounts (one never opened), two
    /// resources and quantities that include zero, more than anyone has, and the largest number.
    fn alphabet() -> Vec<fn() -> Heard<Small>> {
        vec![
            || Heard::Founded { accounts: vec![0, 1], balances: vec![(0, 0, Quantity(3)), (0, 1, Quantity(3)), (1, 1, Quantity(u64::MAX - 1))] },
            || Heard::Founded { accounts: vec![2], balances: vec![(2, 0, Quantity(9))] },
            || Heard::Opened(1),
            || order(0, 1, 0, 2),
            || order(1, 0, 0, 1),
            || order(0, 0, 0, 1),
            || order(0, 2, 0, 1),
            || order(1, 0, 1, u64::MAX - 1),
            || order(0, 1, 1, 2),
            || order(0, 1, 1, 1),
            || order(0, 1, 0, 0),
            || order(0, 1, 0, u64::MAX),
        ]
    }

    fn order(from: u8, to: u8, resource: u8, n: u64) -> Heard<Small> {
        Heard::Ordered(Order { from, to, resource, quantity: Quantity(n), context: () })
    }

    #[test]
    fn the_laws_hold_in_every_reachable_state_of_a_small_world() -> Result<(), crate::Broken> {
        let here = Who::Here(Path::default());
        let mut frontier = vec![Ledger::<Small>::default()];
        let mut seen = std::collections::BTreeSet::new();
        let mut states = 0usize;
        for _ in 0..64 {
            if frontier.is_empty() {
                break;
            }
            let mut next = Vec::new();
            for before in &frontier {
                for input in alphabet() {
                    let mut after = before.clone();
                    let was_order = matches!(input(), Heard::Ordered(_));
                    let said = after.step(&here, input())?;
                    assert!(after.conserved(), "conserved");
                    assert!(after.only_accounts_hold(), "only accounts hold");
                    assert_eq!(said.len(), usize::from(was_order), "every order answered exactly once");
                    if matches!(said.first(), Some(Said::Failed { .. })) {
                        assert_eq!(state_digest(before), state_digest(&after), "a refusal moves nothing");
                    }
                    if seen.insert(state_digest(&after)) {
                        states += 1;
                        next.push(after);
                    }
                }
            }
            frontier = next;
        }
        assert!(frontier.is_empty(), "the reachable states were exhausted, not merely sampled");
        assert!(states > 10, "the search was not vacuous: {states} states");
        Ok(())
    }
}

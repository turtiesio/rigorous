//! How a run is reproduced: the recorded stimuli are everything needed.

use crate::fault::FaultOf;
use crate::path::Path;
use crate::payload::Payload;
use crate::runtime::Runtime;
use crate::unit::Unit;

impl<E: Payload> Runtime<E> {
    /// The external stimuli recorded so far (everything needed to replay).
    #[must_use]
    pub fn stimuli(&self) -> Vec<(Path, E)> {
        self.log.entries().iter().filter(|e| e.cause.is_none() && e.event.kind() != E::settle_kind()).map(|e| (e.actor.clone(), e.event.clone())).collect()
    }

    /// Rebuild by re-running stimuli against a fresh root.
    ///
    /// # Errors
    /// See [`Fault`].
    pub fn replay(root: Box<dyn Unit<E>>, stimuli: &[(Path, E)]) -> Result<Self, FaultOf<E>> {
        let mut rt = Self::new(root);
        for (actor, event) in stimuli {
            let _ = rt.inject_as(actor.clone(), event.clone())?;
        }
        Ok(rt)
    }
}

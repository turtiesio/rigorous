//! Parallel composition: an event goes to every part that accepts its kind.
//! Transparent (adds no identity), so bracketing cannot be observed.

use crate::digest::EncodingError;
use crate::envelope::{Emit, Envelope};
use crate::fault::{Fault, FaultOf};
use crate::name::Name;
use crate::payload::Payload;
use crate::unit::Unit;
use std::any::Any;
use std::collections::BTreeSet;
use std::hash::Hasher;

/// Parallel composition.
#[derive(Debug)]
pub struct Par<E: Payload> {
    /// Each part with its input interface, computed once: interfaces are fixed at construction.
    parts: Vec<Slot<E>>,
}

/// A part together with its input interface.
type Slot<E> = (BTreeSet<<E as Payload>::Kind>, Box<dyn Unit<E>>);

impl<E: Payload> Par<E> {
    /// Compose. `open` are the kinds any part may emit (authenticated by the kernel's actor
    /// stamp rather than by ownership); every other kind must have exactly one owner.
    ///
    /// # Errors
    /// On duplicate names, clashing output ownership, or a unit listening inside a boundary.
    pub fn new(mut parts: Vec<Box<dyn Unit<E>>>, open: &BTreeSet<E::Kind>) -> Result<Self, FaultOf<E>> {
        parts.sort_by_key(|u| u.names());
        let mut seen: BTreeSet<Name> = BTreeSet::new();
        for u in &parts {
            for n in u.names() {
                if !seen.insert(n.clone()) {
                    return Err(Fault::DuplicateName(n));
                }
            }
            if let Some(spy) = u.spies().into_iter().next() {
                return Err(Fault::ListensInsideBoundary(spy));
            }
            if let Some(forger) = u.forgers().into_iter().next() {
                return Err(Fault::EmitsInsideBoundary(forger));
            }
        }
        let anon = Name::lit("par");
        let mut owner: Vec<(E::Kind, Name)> = Vec::new();
        for u in &parts {
            let me = u.names().into_iter().next().unwrap_or_else(|| anon.clone());
            for k in u.outputs() {
                if open.contains(&k) || k == E::scoped_kind() {
                    continue;
                }
                if let Some((_, other)) = owner.iter().find(|(kind, _)| *kind == k) {
                    return Err(Fault::OutputClash { kind: k, a: other.clone(), b: me });
                }
                owner.push((k, me.clone()));
            }
        }
        Ok(Self { parts: parts.into_iter().map(|u| (u.inputs(), u)).collect() })
    }
}

impl<E: Payload> Unit<E> for Par<E> {
    fn names(&self) -> Vec<Name> {
        self.parts.iter().flat_map(|(_, u)| u.names()).collect()
    }
    fn inputs(&self) -> BTreeSet<E::Kind> {
        self.parts.iter().flat_map(|(i, _)| i.iter().copied()).collect()
    }
    fn outputs(&self) -> BTreeSet<E::Kind> {
        self.parts.iter().flat_map(|(_, u)| u.outputs()).collect()
    }
    fn spies(&self) -> Vec<Name> {
        self.parts.iter().flat_map(|(_, u)| u.spies()).collect()
    }
    fn forgers(&self) -> Vec<Name> {
        self.parts.iter().flat_map(|(_, u)| u.forgers()).collect()
    }
    fn step(&mut self, env: &Envelope<'_, E>) -> Result<Vec<Emit<E>>, FaultOf<E>> {
        let kind = env.event.kind();
        let mut out = Vec::new();
        for (inputs, u) in &mut self.parts {
            if inputs.contains(&kind) {
                out.extend(u.step(env)?);
            }
        }
        Ok(out)
    }
    fn fork(&self) -> Box<dyn Unit<E>> {
        Box::new(Self { parts: self.parts.iter().map(|(i, u)| (i.clone(), u.fork())).collect() })
    }
    fn hash_state(&self, h: &mut dyn Hasher) {
        // State digests are compared only between runtimes built the same way.
        for (_, u) in &self.parts {
            u.hash_state(h);
        }
    }
    fn save(&self) -> Result<Vec<u8>, EncodingError> {
        let parts: Result<Vec<Vec<u8>>, EncodingError> = self.parts.iter().map(|(_, u)| u.save()).collect();
        bincode::serialize(&parts?).map_err(|e| EncodingError(e.to_string()))
    }
    fn load(&mut self, bytes: &[u8]) -> Result<(), EncodingError> {
        let parts: Vec<Vec<u8>> = bincode::deserialize(bytes).map_err(|e| EncodingError(e.to_string()))?;
        if parts.len() != self.parts.len() {
            return Err(EncodingError("snapshot is of a differently composed unit".to_owned()));
        }
        for ((_, u), b) in self.parts.iter_mut().zip(&parts) {
            u.load(b)?;
        }
        Ok(())
    }
    fn find(&self, path: &[Name]) -> Option<&dyn Any> {
        self.parts.iter().find_map(|(_, u)| u.find(path))
    }
}

//! The boundary: a whole unit seen from outside as one actor, its internal events wrapped
//! so that nobody outside can react to them. A federated boundary also keeps vocabularies apart.

use crate::digest::EncodingError;
use crate::envelope::{Emit, Envelope, Who};
use crate::fault::FaultOf;
use crate::name::Name;
use crate::path::Path;
use crate::payload::Payload;
use crate::unit::Unit;
use std::any::Any;
use std::collections::BTreeSet;
use std::hash::Hasher;

/// A boundary: `inner` seen from outside as the single actor `name`.
#[derive(Debug)]
pub struct Entity<E: Payload> {
    name: Name,
    inner: Box<dyn Unit<E>>,
    inner_inputs: BTreeSet<E::Kind>,
    imports: BTreeSet<E::Kind>,
    exports: BTreeSet<E::Kind>,
    /// A federated boundary keeps the two sides' vocabularies apart: what comes in arrives
    /// marked as foreign (so the inside's own units do not mistake an outside act for one of
    /// theirs), and only what the inside explicitly addresses abroad goes out.
    federated: bool,
}

impl<E: Payload> Entity<E> {
    /// A boundary around an organisation that deals with the outside only through its own
    /// envoy: `imports` arrive wrapped as foreign; only events addressed abroad leave.
    #[must_use]
    pub fn federated(name: Name, inner: Box<dyn Unit<E>>, imports: BTreeSet<E::Kind>, exports: BTreeSet<E::Kind>) -> Self {
        Self { federated: true, ..Self::new(name, inner, imports, exports) }
    }

    #[must_use]
    pub fn new(name: Name, inner: Box<dyn Unit<E>>, imports: BTreeSet<E::Kind>, exports: BTreeSet<E::Kind>) -> Self {
        Self { name, inner_inputs: inner.inputs(), inner, imports, exports, federated: false }
    }
}

impl<E: Payload> Unit<E> for Entity<E> {
    fn names(&self) -> Vec<Name> {
        vec![self.name.clone()]
    }
    fn inputs(&self) -> BTreeSet<E::Kind> {
        let mut s = self.imports.clone();
        let _ = s.insert(E::scoped_kind());
        if self.inner_inputs.contains(&E::settle_kind()) {
            let _ = s.insert(E::settle_kind());
        }
        s
    }
    fn outputs(&self) -> BTreeSet<E::Kind> {
        let mut s = self.exports.clone();
        let _ = s.insert(E::scoped_kind());
        s
    }
    fn is_boundary(&self) -> bool {
        true
    }
    fn spies(&self) -> Vec<Name> {
        Vec::new()
    }
    fn forgers(&self) -> Vec<Name> {
        Vec::new()
    }
    fn step(&mut self, env: &Envelope<'_, E>) -> Result<Vec<Emit<E>>, FaultOf<E>> {
        let outer = |w: &Who| match w {
            Who::Here(p) | Who::Outer(p) => Who::Outer(p.clone()),
        };
        let wrapped;
        let (event, who) = match env.event.as_scoped() {
            Some((entity, inner)) if *entity == self.name => {
                // An internal event is one that came from inside: its actor is named under this
                // boundary. A scoped event carrying this entity's name but coming from anywhere
                // else is a forgery and is dropped.
                match &env.who {
                    Who::Here(p) => match p.strip(&self.name) {
                        Some(rest) => (inner, Who::Here(rest)),
                        None => return Ok(Vec::new()),
                    },
                    Who::Outer(_) => return Ok(Vec::new()),
                }
            }
            Some(_) => return Ok(Vec::new()),
            None if env.event.kind() == E::settle_kind() => (env.event, env.who.clone()),
            None if self.imports.contains(&env.event.kind()) && self.federated => {
                wrapped = E::foreign(env.event.clone());
                (&wrapped, outer(&env.who))
            }
            None if self.imports.contains(&env.event.kind()) => (env.event, outer(&env.who)),
            None => return Ok(Vec::new()),
        };
        if !self.inner_inputs.contains(&event.kind()) {
            return Ok(Vec::new());
        }
        let inner_env = Envelope { who, entry: env.entry, event };
        let mut out = Vec::new();
        for o in self.inner.step(&inner_env)? {
            let src = o.src.inside(&self.name);
            if self.federated {
                match o.event.as_abroad() {
                    Some(sent) if self.exports.contains(&sent.kind()) => out.push(Emit { event: sent.clone(), actor: Path::of(&self.name), src }),
                    Some(_) | None => out.push(Emit { event: E::scoped(self.name.clone(), o.event), actor: o.actor.inside(&self.name), src }),
                }
            } else if self.exports.contains(&o.event.kind()) && o.event.as_scoped().is_none() {
                out.push(Emit { event: o.event, actor: Path::of(&self.name), src });
            } else {
                out.push(Emit { event: E::scoped(self.name.clone(), o.event), actor: o.actor.inside(&self.name), src });
            }
        }
        Ok(out)
    }
    fn fork(&self) -> Box<dyn Unit<E>> {
        Box::new(Self {
            name: self.name.clone(),
            inner: self.inner.fork(),
            inner_inputs: self.inner_inputs.clone(),
            imports: self.imports.clone(),
            exports: self.exports.clone(),
            federated: self.federated,
        })
    }
    fn hash_state(&self, h: &mut dyn Hasher) {
        self.inner.hash_state(h);
    }
    fn save(&self) -> Result<Vec<u8>, EncodingError> {
        self.inner.save()
    }
    fn load(&mut self, bytes: &[u8]) -> Result<(), EncodingError> {
        self.inner.load(bytes)
    }
    fn find(&self, path: &[Name]) -> Option<&dyn Any> {
        match path.split_first() {
            Some((first, rest)) if *first == self.name => self.inner.find(rest),
            Some(_) | None => None,
        }
    }
}

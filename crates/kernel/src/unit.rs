//! Units: deterministic open transition systems, closed under composition.
//!
//! Three constructors and nothing else:
//!
//! * [`Cell`] wraps a [`Leaf`] (a primitive transition function);
//! * [`Par`] is parallel composition: an event goes to every part that accepts its kind.
//!   It is transparent (adds no identity), so bracketing cannot be observed;
//! * [`Entity`] is a boundary: a whole unit seen from outside as one actor, its internal
//!   events wrapped so that nobody outside can react to them.

use super::base::{EncodingError, Name, Path};
use super::log::Entry;
use std::any::Any;
use std::collections::BTreeSet;
use std::fmt::Debug;
use std::hash::Hasher;

/// What the kernel needs to know about an event type.
pub trait Payload: Clone + Debug + serde::Serialize + 'static {
    /// The closed set of event kinds: interfaces are sets of these.
    type Kind: Copy + Ord + Debug + 'static;
    /// The enumerated ways a unit can be unable to proceed (as opposed to refusing, which is an event).
    type Failure: Clone + Debug + PartialEq + 'static;
    fn kind(&self) -> Self::Kind;
    /// The marker the runtime issues when a cascade has reached quiescence.
    fn settle() -> Self;
    fn settle_kind() -> Self::Kind;
    /// Wrap an event as internal to the entity `entity`.
    fn scoped(entity: Name, inner: Self) -> Self;
    fn as_scoped(&self) -> Option<(&Name, &Self)>;
    fn scoped_kind() -> Self::Kind;
    /// Mark an event as having come in from outside a federated boundary.
    fn foreign(inner: Self) -> Self;
    fn foreign_kind() -> Self::Kind;
    /// If this event is addressed to the world outside a federated boundary, what it carries.
    fn as_abroad(&self) -> Option<&Self>;
}

/// From whose point of view an actor is named.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Who {
    /// An actor inside the current boundary, by its local path.
    Here(Path),
    /// An actor outside the current boundary.
    Outer(Path),
}

impl Who {
    /// The local actor's name, if it is a single local name.
    #[must_use]
    pub fn local(&self) -> Option<&Name> {
        match self {
            Self::Here(p) => p.only(),
            Self::Outer(_) => None,
        }
    }
}

/// An event as delivered to a unit.
#[derive(Debug)]
pub struct Envelope<'a, E> {
    pub who: Who,
    pub entry: &'a Entry<E>,
    pub event: &'a E,
}

/// An event a unit emits, with the provenance the kernel assigned to it.
#[derive(Clone, Debug)]
pub struct Emit<E> {
    pub event: E,
    pub actor: Path,
    pub src: Path,
}

/// The fault type of an event type.
pub type FaultOf<E> = Fault<<E as Payload>::Kind, <E as Payload>::Failure>;

/// A fault in the machinery (as opposed to a refusal, which is an ordinary event).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Fault<K, F> {
    /// A unit emitted a kind it did not declare.
    Undeclared {
        unit: Name,
        kind: K,
    },
    /// Two parts would own the same output kind.
    OutputClash {
        kind: K,
        a: Name,
        b: Name,
    },
    DuplicateName(Name),
    /// A non-boundary unit declared that it listens to scoped (internal) events.
    ListensInsideBoundary(Name),
    /// The cascade did not reach quiescence within the fuel bound.
    Divergence {
        fuel: u64,
    },
    Encoding(EncodingError),
    /// A unit could not process an event it accepted.
    Unit {
        unit: Name,
        failure: F,
    },
}

impl<K, F> From<EncodingError> for Fault<K, F> {
    fn from(e: EncodingError) -> Self {
        Self::Encoding(e)
    }
}

/// Why a leaf could not process an event it accepted.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LeafError<F> {
    Encoding(EncodingError),
    /// The unit cannot proceed, for one of its enumerated reasons.
    Broken(F),
}

impl<F> From<EncodingError> for LeafError<F> {
    fn from(e: EncodingError) -> Self {
        Self::Encoding(e)
    }
}

/// A primitive unit. `on` is its transition function; it owns its state.
pub trait Leaf<E: Payload>: Clone + std::hash::Hash + Debug + serde::Serialize + serde::de::DeserializeOwned + 'static {
    fn name(&self) -> Name;
    fn inputs(&self) -> BTreeSet<E::Kind>;
    fn outputs(&self) -> BTreeSet<E::Kind>;
    /// # Errors
    /// Only for faults; refusals are ordinary events.
    fn on(&mut self, env: &Envelope<'_, E>) -> Result<Vec<E>, LeafError<E::Failure>>;
}

/// The object-safe view of any unit, primitive or composite.
pub trait Unit<E: Payload>: Debug {
    /// Names of all leaves and entities directly visible at this level.
    fn names(&self) -> Vec<Name>;
    fn inputs(&self) -> BTreeSet<E::Kind>;
    fn outputs(&self) -> BTreeSet<E::Kind>;
    fn is_boundary(&self) -> bool {
        false
    }
    /// Units at this level (through transparent `Par`s) that listen to scoped events without being a boundary.
    fn spies(&self) -> Vec<Name>;
    /// # Errors
    /// See [`Fault`].
    fn step(&mut self, env: &Envelope<'_, E>) -> Result<Vec<Emit<E>>, FaultOf<E>>;
    fn fork(&self) -> Box<dyn Unit<E>>;
    fn hash_state(&self, h: &mut dyn Hasher);
    /// The unit's whole state, for a snapshot.
    ///
    /// # Errors
    /// If the state cannot be encoded.
    fn save(&self) -> Result<Vec<u8>, EncodingError>;
    /// Replace the unit's state with one produced by [`Unit::save`] on a unit built the same way.
    ///
    /// # Errors
    /// If the bytes are not such a state.
    fn load(&mut self, bytes: &[u8]) -> Result<(), EncodingError>;
    /// Typed read access to a leaf by local path (for views and tests; never for other units).
    fn find(&self, path: &[Name]) -> Option<&dyn Any>;
}

/// A leaf as a unit.
#[derive(Clone, Debug)]
pub struct Cell<L>(pub L);

impl<E: Payload, L: Leaf<E>> Unit<E> for Cell<L> {
    fn names(&self) -> Vec<Name> {
        vec![self.0.name()]
    }
    fn inputs(&self) -> BTreeSet<E::Kind> {
        self.0.inputs()
    }
    fn outputs(&self) -> BTreeSet<E::Kind> {
        self.0.outputs()
    }
    fn spies(&self) -> Vec<Name> {
        if self.0.inputs().contains(&E::scoped_kind()) {
            vec![self.0.name()]
        } else {
            Vec::new()
        }
    }
    fn step(&mut self, env: &Envelope<'_, E>) -> Result<Vec<Emit<E>>, FaultOf<E>> {
        let name = self.0.name();
        let declared = self.0.outputs();
        let path = Path::of(&name);
        let mut out = Vec::new();
        let events = self.0.on(env).map_err(|e| match e {
            LeafError::Encoding(x) => Fault::Encoding(x),
            LeafError::Broken(failure) => Fault::Unit { unit: name.clone(), failure },
        })?;
        for event in events {
            if !declared.contains(&event.kind()) {
                return Err(Fault::Undeclared { unit: name, kind: event.kind() });
            }
            out.push(Emit { event, actor: path.clone(), src: path.clone() });
        }
        Ok(out)
    }
    fn fork(&self) -> Box<dyn Unit<E>> {
        Box::new(self.clone())
    }
    fn hash_state(&self, mut h: &mut dyn Hasher) {
        std::hash::Hash::hash(&self.0, &mut h);
    }
    fn save(&self) -> Result<Vec<u8>, EncodingError> {
        bincode::serialize(&self.0).map_err(|e| EncodingError(e.to_string()))
    }
    fn load(&mut self, bytes: &[u8]) -> Result<(), EncodingError> {
        self.0 = bincode::deserialize(bytes).map_err(|e| EncodingError(e.to_string()))?;
        Ok(())
    }
    fn find(&self, path: &[Name]) -> Option<&dyn Any> {
        match path {
            [n] if *n == self.0.name() => Some(&self.0),
            _ => None,
        }
    }
}

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
    fn step(&mut self, env: &Envelope<'_, E>) -> Result<Vec<Emit<E>>, FaultOf<E>> {
        let outer = |w: &Who| match w {
            Who::Here(p) | Who::Outer(p) => Who::Outer(p.clone()),
        };
        let wrapped;
        let (event, who) = match env.event.as_scoped() {
            Some((entity, inner)) if *entity == self.name => {
                let who = match &env.who {
                    Who::Here(p) => p.strip(&self.name).map_or_else(|| Who::Outer(p.clone()), Who::Here),
                    Who::Outer(p) => Who::Outer(p.clone()),
                };
                (inner, who)
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

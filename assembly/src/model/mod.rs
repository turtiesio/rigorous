//! The typed vocabulary of the assembled system: one file per concept's words, and the event
//! type that sums them.

mod act;
mod amending;
mod amendment;
mod authority;
mod event;
mod evidence;
mod failure;
mod ids;
mod judgment;
mod membership;
mod migrate;
mod obligation;
mod outside;
mod proposition;
mod reasoning;
mod rules;
mod value;

pub use act::*;
pub use amending::*;
pub use amendment::*;
pub use authority::*;
pub use event::*;
pub use evidence::*;
pub use failure::*;
pub use ids::*;
pub use judgment::*;
pub use membership::*;
pub use migrate::*;
pub use obligation::*;
pub use outside::*;
pub use proposition::*;
pub use reasoning::*;
pub use rules::*;
pub use value::*;

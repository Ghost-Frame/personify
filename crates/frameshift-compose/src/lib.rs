//! Persona composition engine.
//!
//! Pillar 5(d) of frameshift's design: personas declare an optional `extends`
//! base and a list of `mixins`; this crate resolves those references into
//! concrete `PersonaSource` values, merges them in deterministic order, and
//! reports conflicts (rule id collisions, skill id collisions, voice
//! mismatches).
//!
//! Composition is layered:
//!   base (extends)  ->  mixins (in declared order)  ->  root persona
//!
//! Later layers override earlier ones on ID collision (last-write-wins),
//! with a `Conflict` record emitted so the user can see what was overridden.
//!
//! Most function bodies are `todo!("M1 impl")` -- only the pieces required
//! for WS-5 scaffolding are implemented (basic `merge_sources` with
//! `BaseFirst` order, `LocalResolver::resolve` against a base directory).

pub mod composed;
pub mod composer;
pub mod conflict;
pub mod error;
pub mod merge;
pub mod resolver;

pub use composed::{ComposedPersona, Layer as ComposedLayer, Provenance};
pub use composer::Composer;
pub use conflict::{detect_conflicts, Conflict};
pub use error::ComposeError;
pub use merge::{merge_sources, MergeOrder};
pub use resolver::{LocalResolver, SourceResolver};

//! Structured persona source.
//!
//! Persona source is a typed schema (TOML) split across three files:
//! `persona.toml`, `rules.toml`, `skills.toml`. Markdown is a *render
//! target* produced from this typed source -- agents and CLIs operate on
//! typed fields, never on string-replace-in-markdown.
//!
//! This crate owns:
//! - the TOML schema for each file (`persona`, `rules`, `skills`)
//! - the composite `PersonaSource` with load/write split across the three files
//! - deterministic markdown projection (`render`)
//! - typed patch operations (`patch`)
//! - semantic diff between two `PersonaSource` snapshots (`diff`)
//!
//! Most function bodies are `todo!("M1 impl")` -- only the pieces required
//! for WS-4 scaffolding are implemented (schema serde round-trip, load/write
//! split, simplest-correct markdown projection).

pub mod diff;
pub mod error;
pub mod patch;
pub mod persona;
pub mod render;
pub mod rules;
pub mod skills;
pub mod source;

pub use diff::{diff, SemanticDiff};
pub use error::SourceError;
pub use patch::{apply_patch, AnchorPosition, PatchOp};
pub use persona::{Anchor, DefaultQuestion, Persona};
pub use render::render_to_markdown;
pub use rules::{Layer, Rule, RuleSet};
pub use skills::{Skill, SkillSet};
pub use source::PersonaSource;

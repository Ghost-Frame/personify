use frameshift_source::{PatternSet, Persona, Rule, Skill};
use serde::{Deserialize, Serialize};

/// Identifies a composition layer that contributed a rule or skill.
///
/// `Base` is the persona named in `extends`; `Mixin(name)` is one of the
/// declared mixins; `Root` is the persona that initiated composition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Layer {
    Base(String),
    Mixin(String),
    Root,
}

/// Provenance tag attached to each merged rule/skill so consumers can tell
/// which layer in the composition stack contributed it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Provenance {
    pub layer: Layer,
}

/// A rule paired with the layer that contributed it.
#[derive(Debug, Clone)]
pub struct ProvenancedRule {
    pub rule: Rule,
    pub provenance: Provenance,
}

/// A skill paired with the layer that contributed it.
#[derive(Debug, Clone)]
pub struct ProvenancedSkill {
    pub skill: Skill,
    pub provenance: Provenance,
}

/// The merged result of composing a root persona with its base + mixins.
///
/// Same shape as `PersonaSource` from `frameshift-source`, but every rule
/// and skill carries provenance so callers can render "rule X came from
/// mixin Y" diagnostics. Patterns are merged by concatenation.
#[derive(Debug, Clone)]
pub struct ComposedPersona {
    /// Core persona metadata from the root layer.
    pub persona: Persona,
    /// Merged rules with provenance tags.
    pub rules: Vec<ProvenancedRule>,
    /// Merged skills with provenance tags.
    pub skills: Vec<ProvenancedSkill>,
    /// Merged patterns from all layers (concatenated, no deduplication).
    pub patterns: PatternSet,
}

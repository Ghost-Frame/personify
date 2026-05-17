use frameshift_source::PersonaSource;

use crate::composed::Layer;

/// A conflict detected during composition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Conflict {
    /// Two or more layers declared a rule with the same id.
    RuleIdCollision { id: String, layers: Vec<Layer> },

    /// Two or more layers declared a skill with the same id.
    SkillIdCollision { id: String, layers: Vec<Layer> },

    /// The voice block disagrees across layers.
    VoiceMismatch { layers: Vec<Layer> },
}

/// Inspect a set of persona source layers and report conflicts.
///
/// Stub for WS-5 -- real heuristics land in M1 once provenance tracking is
/// wired through `merge_sources`.
pub fn detect_conflicts(_layers: &[&PersonaSource]) -> Vec<Conflict> {
    todo!("M1 impl");
}

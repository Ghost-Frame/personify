use serde::{Deserialize, Serialize};

use crate::source::PersonaSource;

/// Typed diff between two `PersonaSource` snapshots.
///
/// Rules and skills are compared by exact `id` equality. `voice_changed`
/// is a plain string-equality check for now. `anchor_similarity` is an
/// `Option<f32>` slot in [0.0, 1.0] reserved for the M1+ embedding-based
/// semantic-similarity score on the L2 cascade anchor; until then the
/// implementation is `todo!()`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct SemanticDiff {
    pub added_rules: Vec<String>,
    pub removed_rules: Vec<String>,
    pub modified_rules: Vec<String>,
    pub added_skills: Vec<String>,
    pub removed_skills: Vec<String>,
    pub voice_changed: bool,
    pub anchor_similarity: Option<f32>,
}

pub fn diff(_a: &PersonaSource, _b: &PersonaSource) -> SemanticDiff {
    todo!("M1 impl -- exact-id rule/skill diff, voice string compare, anchor similarity stub")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn semantic_diff_json_roundtrip() {
        let d = SemanticDiff {
            added_rules: vec!["new-rule".to_string()],
            removed_rules: vec!["old-rule".to_string()],
            modified_rules: vec!["tweaked".to_string()],
            added_skills: vec!["s1".to_string()],
            removed_skills: vec![],
            voice_changed: true,
            anchor_similarity: Some(0.87),
        };

        let serialized = serde_json::to_string(&d).unwrap();
        let parsed: SemanticDiff = serde_json::from_str(&serialized).unwrap();
        assert_eq!(parsed, d);
    }

    #[test]
    fn default_semantic_diff_is_empty() {
        let d = SemanticDiff::default();
        assert!(d.added_rules.is_empty());
        assert!(d.removed_rules.is_empty());
        assert!(!d.voice_changed);
        assert!(d.anchor_similarity.is_none());
    }
}

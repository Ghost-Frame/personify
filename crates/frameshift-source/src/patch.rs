use serde::{Deserialize, Serialize};

use crate::error::SourceError;
use crate::rules::Rule;
use crate::skills::Skill;
use crate::source::PersonaSource;

/// Position of a cascade anchor block in the rendered output.
///
/// `Top` and `Recency` are the two priming positions documented in the
/// PLAN.md cascade-anchor design; `L2` is the body anchor used for the
/// main behavioral framing.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AnchorPosition {
    Top,
    L2,
    Recency,
}

/// Typed mutation against a `PersonaSource`. The publish flow can render
/// each `PatchOp` as a human-readable changelog entry instead of a text
/// diff -- this is the whole point of typed source.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PatchOp {
    RuleAdd(Rule),
    RuleRemove { id: String },
    SkillAdd(Skill),
    SkillRemove { id: String },
    CascadeAnchorSet {
        position: AnchorPosition,
        text: String,
    },
}

/// Apply a single patch op to a persona source. Stub for M1.
pub fn apply_patch(_src: &mut PersonaSource, _op: PatchOp) -> Result<(), SourceError> {
    todo!("M1 impl -- apply typed patch ops to PersonaSource")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::Layer;

    #[test]
    fn patch_op_json_roundtrip() {
        // PatchOp is exposed over MCP as JSON, so the JSON shape needs to
        // round-trip cleanly even though the at-rest format on disk is TOML.
        let ops = vec![
            PatchOp::RuleAdd(Rule {
                id: "new-rule".to_string(),
                layer: Layer::L2,
                text: "be kind".to_string(),
            }),
            PatchOp::RuleRemove {
                id: "old-rule".to_string(),
            },
            PatchOp::SkillAdd(Skill {
                id: "new-skill".to_string(),
                invoke_when: "always".to_string(),
            }),
            PatchOp::SkillRemove {
                id: "old-skill".to_string(),
            },
            PatchOp::CascadeAnchorSet {
                position: AnchorPosition::Recency,
                text: "final priming line".to_string(),
            },
        ];

        let serialized = serde_json::to_string(&ops).unwrap();
        let parsed: Vec<PatchOp> = serde_json::from_str(&serialized).unwrap();
        assert_eq!(parsed, ops);
    }
}

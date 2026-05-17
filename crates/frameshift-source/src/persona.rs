use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// The `persona.toml` file -- identity, voice, anchors, default questions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Persona {
    pub schema_version: u32,
    pub name: String,
    pub voice: String,
    /// Anchor blocks keyed by anchor name (e.g. "l2", "cascade_top",
    /// "cascade_recency"). Order is not significant here; render order is
    /// chosen by the projection layer.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub anchor: BTreeMap<String, Anchor>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub default_questions: Vec<DefaultQuestion>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Anchor {
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DefaultQuestion {
    pub question: String,
}

impl Persona {
    /// Minimal valid persona -- used as a default when scaffolding.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            schema_version: 1,
            name: name.into(),
            voice: String::new(),
            anchor: BTreeMap::new(),
            default_questions: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn persona_toml_roundtrip() {
        let mut anchor = BTreeMap::new();
        anchor.insert(
            "l2".to_string(),
            Anchor {
                text: "You are a cryptographic correctness practitioner.".to_string(),
            },
        );
        anchor.insert(
            "cascade_top".to_string(),
            Anchor {
                text: "specification first, reference implementation second".to_string(),
            },
        );

        let original = Persona {
            schema_version: 1,
            name: "cryptographic".to_string(),
            voice: "citation-driven, careful, willing to say I don't know".to_string(),
            anchor,
            default_questions: vec![DefaultQuestion {
                question: "Which specification or RFC governs this code?".to_string(),
            }],
        };

        let serialized = toml::to_string(&original).unwrap();
        let parsed: Persona = toml::from_str(&serialized).unwrap();
        assert_eq!(parsed, original);
    }
}

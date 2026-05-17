use serde::{Deserialize, Serialize};

/// The `rules.toml` file -- a flat list of `[[rule]]` entries.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct RuleSet {
    #[serde(default, rename = "rule", skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<Rule>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Rule {
    pub id: String,
    pub layer: Layer,
    pub text: String,
}

/// Behavioral layer for a rule.
///
/// - `L1` -- non-negotiable invariants
/// - `L2` -- contextual defaults, overridable with explicit justification
/// - `L3` -- preferences and stylistic guidance
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Layer {
    L1,
    L2,
    L3,
}

impl RuleSet {
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ruleset_toml_roundtrip() {
        let original = RuleSet {
            rules: vec![
                Rule {
                    id: "no-rolling-crypto".to_string(),
                    layer: Layer::L1,
                    text: "Never roll a new cryptographic primitive when an audited implementation exists.".to_string(),
                },
                Rule {
                    id: "preserve-constant-time".to_string(),
                    layer: Layer::L1,
                    text: "Never replace constant-time code with variable-time code.".to_string(),
                },
                Rule {
                    id: "prefer-rfc-citations".to_string(),
                    layer: Layer::L3,
                    text: "Cite the RFC or specification when discussing protocol behavior.".to_string(),
                },
            ],
        };

        let serialized = toml::to_string(&original).unwrap();
        let parsed: RuleSet = toml::from_str(&serialized).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn empty_ruleset_roundtrips() {
        let original = RuleSet::default();
        let serialized = toml::to_string(&original).unwrap();
        let parsed: RuleSet = toml::from_str(&serialized).unwrap();
        assert_eq!(parsed, original);
    }
}

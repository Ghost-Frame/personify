use serde::{Deserialize, Serialize};

/// The `skills.toml` file -- a flat list of `[[skill]]` entries.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct SkillSet {
    #[serde(default, rename = "skill", skip_serializing_if = "Vec::is_empty")]
    pub skills: Vec<Skill>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Skill {
    pub id: String,
    /// Free-text description of when this skill should be invoked.
    pub invoke_when: String,
}

impl SkillSet {
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skillset_toml_roundtrip() {
        let original = SkillSet {
            skills: vec![
                Skill {
                    id: "test-driven-development".to_string(),
                    invoke_when: "All cryptographic implementations -- tests BEFORE code".to_string(),
                },
                Skill {
                    id: "security-audit-remediation".to_string(),
                    invoke_when: "When CVE-class issues are reported against a primitive in use".to_string(),
                },
            ],
        };

        let serialized = toml::to_string(&original).unwrap();
        let parsed: SkillSet = toml::from_str(&serialized).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn empty_skillset_roundtrips() {
        let original = SkillSet::default();
        let serialized = toml::to_string(&original).unwrap();
        let parsed: SkillSet = toml::from_str(&serialized).unwrap();
        assert_eq!(parsed, original);
    }
}

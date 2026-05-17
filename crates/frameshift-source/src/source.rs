use std::fs;
use std::path::{Path, PathBuf};

use crate::error::SourceError;
use crate::persona::Persona;
use crate::rules::RuleSet;
use crate::skills::SkillSet;

const PERSONA_FILE: &str = "persona.toml";
const RULES_FILE: &str = "rules.toml";
const SKILLS_FILE: &str = "skills.toml";

/// Composite persona source. Read from / written to a directory containing
/// `persona.toml`, `rules.toml`, `skills.toml`.
#[derive(Debug, Clone, PartialEq)]
pub struct PersonaSource {
    pub persona: Persona,
    pub rules: RuleSet,
    pub skills: SkillSet,
}

impl PersonaSource {
    pub fn new(persona: Persona) -> Self {
        Self {
            persona,
            rules: RuleSet::default(),
            skills: SkillSet::default(),
        }
    }

    /// Load a persona source from a directory. The directory must contain
    /// `persona.toml`. `rules.toml` and `skills.toml` are optional -- absent
    /// files yield empty sets.
    pub fn load_from_dir(dir: &Path) -> Result<Self, SourceError> {
        let persona = load_required::<Persona>(&dir.join(PERSONA_FILE))?;
        let rules = load_optional::<RuleSet>(&dir.join(RULES_FILE))?.unwrap_or_default();
        let skills = load_optional::<SkillSet>(&dir.join(SKILLS_FILE))?.unwrap_or_default();
        Ok(Self {
            persona,
            rules,
            skills,
        })
    }

    /// Write a persona source as three TOML files in `dir`. Creates the
    /// directory if it does not exist. Always writes all three files
    /// (including empty `rules.toml` / `skills.toml` for predictable layout).
    pub fn write_to_dir(&self, dir: &Path) -> Result<(), SourceError> {
        fs::create_dir_all(dir).map_err(|source| SourceError::Io {
            path: dir.to_path_buf(),
            source,
        })?;

        write_toml(&dir.join(PERSONA_FILE), &self.persona)?;
        write_toml(&dir.join(RULES_FILE), &self.rules)?;
        write_toml(&dir.join(SKILLS_FILE), &self.skills)?;
        Ok(())
    }
}

fn load_required<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T, SourceError> {
    if !path.exists() {
        return Err(SourceError::MissingFile(path.to_path_buf()));
    }
    read_toml(path)
}

fn load_optional<T: serde::de::DeserializeOwned>(path: &Path) -> Result<Option<T>, SourceError> {
    if !path.exists() {
        return Ok(None);
    }
    read_toml(path).map(Some)
}

fn read_toml<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T, SourceError> {
    let raw = fs::read_to_string(path).map_err(|source| SourceError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    toml::from_str(&raw).map_err(|source| SourceError::TomlDeserialize {
        path: path.to_path_buf(),
        source,
    })
}

fn write_toml<T: serde::Serialize>(path: &PathBuf, value: &T) -> Result<(), SourceError> {
    let serialized = toml::to_string(value)?;
    fs::write(path, serialized).map_err(|source| SourceError::Io {
        path: path.clone(),
        source,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::persona::{Anchor, DefaultQuestion, Persona};
    use crate::rules::{Layer, Rule, RuleSet};
    use crate::skills::{Skill, SkillSet};
    use std::collections::BTreeMap;

    fn sample() -> PersonaSource {
        let mut anchor = BTreeMap::new();
        anchor.insert(
            "l2".to_string(),
            Anchor {
                text: "anchor body".to_string(),
            },
        );

        PersonaSource {
            persona: Persona {
                schema_version: 1,
                name: "demo".to_string(),
                voice: "demo voice".to_string(),
                anchor,
                default_questions: vec![DefaultQuestion {
                    question: "what is this?".to_string(),
                }],
            },
            rules: RuleSet {
                rules: vec![Rule {
                    id: "r1".to_string(),
                    layer: Layer::L1,
                    text: "rule one".to_string(),
                }],
            },
            skills: SkillSet {
                skills: vec![Skill {
                    id: "s1".to_string(),
                    invoke_when: "always".to_string(),
                }],
            },
        }
    }

    #[test]
    fn write_then_load_roundtrip() {
        let tmp = tempfile_dir();
        let original = sample();
        original.write_to_dir(&tmp).unwrap();

        let loaded = PersonaSource::load_from_dir(&tmp).unwrap();
        assert_eq!(loaded, original);
    }

    #[test]
    fn missing_persona_file_errors() {
        let tmp = tempfile_dir();
        let err = PersonaSource::load_from_dir(&tmp).unwrap_err();
        assert!(matches!(err, SourceError::MissingFile(_)));
    }

    /// Build a fresh empty temp dir without pulling tempfile as a dep --
    /// frameshift-source has no dev-deps and we want to keep it that way
    /// for this scaffolding milestone.
    fn tempfile_dir() -> PathBuf {
        let base = std::env::temp_dir();
        let unique = format!(
            "frameshift-source-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0)
        );
        let dir = base.join(unique);
        fs::create_dir_all(&dir).expect("create temp dir");
        dir
    }
}

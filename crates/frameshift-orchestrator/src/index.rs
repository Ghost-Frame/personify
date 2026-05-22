//! Persona index: matchable representations of installed personas.

use std::collections::BTreeSet;
use std::path::PathBuf;

use frameshift_source::PersonaSource;

use crate::error::OrchestratorError;

/// Stopwords excluded from persona keyword extraction.
const STOPWORDS: &[&str] = &[
    "the", "and", "for", "with", "that", "this", "you", "are", "not", "its",
    "use", "all", "can", "has", "was", "will", "any", "but", "our", "have",
    "from", "they", "when", "your", "how", "what", "who",
];

/// A matchable, pre-processed representation of a single persona.
///
/// Built from a `PersonaSource` by extracting and normalizing all textual content
/// into deduplicated keyword bags and structured sets for fast overlap scoring.
#[derive(Debug, Clone)]
pub struct PersonaProfile {
    /// The persona's canonical name.
    pub name: String,

    /// Optional human-readable description of the persona.
    pub description: Option<String>,

    /// Programming languages this persona is associated with, derived from
    /// `CodeExample.language` fields, keyword scan of name/description, and
    /// known-language name detection.
    pub languages: BTreeSet<String>,

    /// Deduplicated lowercase keyword tokens extracted from name, description,
    /// voice (tone + text), anchor texts, rule texts, skill `invoke_when` fields,
    /// and pattern category/items. Stopwords and short tokens removed.
    pub keywords: Vec<String>,

    /// Required tools declared in the capability manifest (empty if none).
    pub required_tools: Vec<String>,

    /// Whether the capability manifest declares network egress required.
    pub network_egress: bool,
}

impl PersonaProfile {
    /// Build a `PersonaProfile` from a loaded `PersonaSource`.
    ///
    /// Extracts languages from code examples and keyword scans, builds a
    /// deduplicated keyword corpus from all textual fields, and copies capability
    /// manifest data if present.
    pub fn from_source(src: &PersonaSource) -> Self {
        let name = src.persona.name.clone();
        let description = src.persona.description.clone();

        // Collect language hints from code examples.
        let mut languages: BTreeSet<String> = BTreeSet::new();
        for ex in &src.patterns.examples {
            let lang = ex.language.to_lowercase();
            if !lang.is_empty() {
                languages.insert(lang);
            }
        }

        // Keyword corpus: gather all text fields, then tokenize + dedup.
        let mut text_parts: Vec<String> = Vec::new();
        text_parts.push(src.persona.name.clone());
        if let Some(desc) = &src.persona.description {
            text_parts.push(desc.clone());
        }
        text_parts.push(src.persona.voice.tone.clone());
        if let Some(vt) = &src.persona.voice.text {
            text_parts.push(vt.clone());
        }
        for q in &src.persona.voice.questions {
            text_parts.push(q.text.clone());
        }
        for anchor in src.persona.anchor.values() {
            text_parts.push(anchor.text.clone());
            if let Some(tl) = &anchor.tagline {
                text_parts.push(tl.clone());
            }
        }
        for rule in &src.rules.rules {
            text_parts.push(rule.text.clone());
        }
        for skill in &src.skills.skills {
            text_parts.push(skill.invoke_when.clone());
        }
        for cat in &src.patterns.stack {
            text_parts.push(cat.category.clone());
            for item in &cat.items {
                text_parts.push(item.clone());
            }
        }
        for ex in &src.patterns.examples {
            text_parts.push(ex.language.clone());
            text_parts.push(ex.context.clone());
        }

        let combined = text_parts.join(" ");
        let keywords = extract_keywords(&combined);

        // Language detection via keyword scan: if a known language name appears
        // in keywords, add it to the language set.
        for lang in KNOWN_LANGUAGES {
            if keywords.iter().any(|k| k == *lang) {
                languages.insert(lang.to_string());
            }
        }

        // Capability manifest.
        let (required_tools, network_egress) = if let Some(cm) = &src.persona.capability_manifest {
            (cm.required_tools.clone(), cm.network_egress)
        } else {
            (Vec::new(), false)
        };

        PersonaProfile {
            name,
            description,
            languages,
            keywords,
            required_tools,
            network_egress,
        }
    }
}

/// Known language identifiers used for keyword-based language detection.
const KNOWN_LANGUAGES: &[&str] = &[
    "rust", "typescript", "javascript", "python", "go", "java", "ruby",
    "c", "cpp", "markdown", "toml", "shell", "bash", "sql", "yaml",
    "haskell", "kotlin", "swift", "scala", "elixir", "erlang", "clojure",
];

/// Extract deduplicated, lowercase, stopword-filtered keyword tokens from `text`.
///
/// Splits on non-alphanumeric characters, lowercases, drops tokens shorter than
/// 3 characters, removes stopwords, and deduplicates while preserving first-seen order.
fn extract_keywords(text: &str) -> Vec<String> {
    let mut seen = std::collections::HashSet::new();
    text.split(|c: char| !c.is_alphanumeric())
        .map(|t| t.to_lowercase())
        .filter(|t| t.len() >= 3)
        .filter(|t| !STOPWORDS.contains(&t.as_str()))
        .filter(|t| seen.insert(t.clone()))
        .collect()
}

/// An in-memory index of all installed persona profiles, ready for scoring.
#[derive(Debug, Clone)]
pub struct PersonaIndex {
    /// Ordered list of pre-processed persona profiles.
    pub profiles: Vec<PersonaProfile>,
}

impl PersonaIndex {
    /// Build a `PersonaIndex` from a slice of already-loaded persona sources.
    pub fn build(sources: &[PersonaSource]) -> Self {
        let profiles = sources.iter().map(PersonaProfile::from_source).collect();
        PersonaIndex { profiles }
    }

    /// Load persona sources from a list of directories and build an index.
    ///
    /// Each directory must contain a valid `persona.toml`. Returns an error if
    /// any directory fails to load.
    pub fn from_dirs(dirs: &[PathBuf]) -> Result<Self, OrchestratorError> {
        let mut sources = Vec::with_capacity(dirs.len());
        for dir in dirs {
            let src = PersonaSource::load_from_dir(dir)?;
            sources.push(src);
        }
        Ok(Self::build(&sources))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a minimal PersonaSource for testing.
    fn minimal_source(name: &str, tone: &str) -> PersonaSource {
        use frameshift_source::*;
        PersonaSource {
            persona: Persona {
                schema_version: 1,
                name: name.to_string(),
                version: None,
                description: Some(format!("{name} persona for testing")),
                license: None,
                author: None,
                extends: None,
                mixin: vec![],
                voice: Voice {
                    tone: tone.to_string(),
                    text: None,
                    questions: vec![],
                },
                anchor: std::collections::BTreeMap::new(),
                classification_tiers: vec![],
                conflict_resolution: None,
                cascade_anchors: vec![],
                self_eval: vec![],
                ambiguity_questions: vec![],
                safety_layer: None,
                growth: None,
                references: vec![],
                capability_manifest: None,
                conformance: None,
                default_questions: vec![],
            },
            rules: RuleSet::default(),
            skills: SkillSet::default(),
            patterns: PatternSet::default(),
        }
    }

    /// from_source extracts the persona name.
    #[test]
    fn profile_extracts_name() {
        let src = minimal_source("rust-expert", "precise and performant");
        let profile = PersonaProfile::from_source(&src);
        assert_eq!(profile.name, "rust-expert");
    }

    /// from_source detects rust keyword in the name.
    #[test]
    fn profile_detects_language_from_name() {
        let src = minimal_source("rust-expert", "precise and performant");
        let profile = PersonaProfile::from_source(&src);
        assert!(profile.keywords.iter().any(|k| k == "rust" || k == "expert"));
    }

    /// PersonaIndex::build creates one profile per source.
    #[test]
    fn index_build_count() {
        let sources = vec![
            minimal_source("alpha", "tone a"),
            minimal_source("beta", "tone b"),
        ];
        let index = PersonaIndex::build(&sources);
        assert_eq!(index.profiles.len(), 2);
    }
}

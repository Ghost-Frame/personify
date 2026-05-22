//! Shared selection helper: single entry point used by CLI, MCP, and daemon.
//!
//! All three surfaces call `select` with a `SelectionInputs` struct so that
//! parity is structural rather than duplicated across callers.

use std::path::{Path, PathBuf};

use crate::context::sense;
use crate::error::OrchestratorError;
use crate::feedback::Preferences;
use crate::index::PersonaIndex;
use crate::policy::{rank, PolicyWeights, Scored};

/// All inputs required to run a persona selection pass.
pub struct SelectionInputs<'a> {
    /// The project root directory used for context sensing.
    pub project_root: &'a Path,

    /// Optional task hint text to steer lexical scoring.
    pub task_hint: Option<&'a str>,

    /// List of persona source directories to index.
    /// Each must contain a valid `persona.toml` (loaded by `PersonaIndex::from_dirs`).
    pub source_dirs: Vec<PathBuf>,

    /// Per-persona scoring bias preferences.
    pub prefs: Preferences,

    /// Scoring weight configuration.
    pub weights: PolicyWeights,
}

/// Run a full persona selection pass and return ranked results.
///
/// Senses the project context from `inputs.project_root`, builds a
/// `PersonaIndex` from `inputs.source_dirs`, applies scoring weights and
/// preferences, and returns the sorted result from `policy::rank`.
///
/// Returns an empty `Vec` (not an error) when `source_dirs` is empty.
pub fn select(inputs: &SelectionInputs<'_>) -> Result<Vec<Scored>, OrchestratorError> {
    if inputs.source_dirs.is_empty() {
        return Ok(Vec::new());
    }

    let ctx = sense(inputs.project_root, inputs.task_hint);
    let index = PersonaIndex::from_dirs(&inputs.source_dirs)?;
    let ranked = rank(&ctx, &index, &inputs.weights, &inputs.prefs);
    Ok(ranked)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    /// Build a minimal persona source directory suitable for indexing.
    fn make_persona_source(dir: &Path, name: &str) {
        fs::create_dir_all(dir).unwrap();
        let toml = format!(
            r#"schema_version = 1
name = "{name}"
version = "0.1.0"
author_handle = "test"
author_pubkey = "local-unsigned"
description = "Test persona for {name}"

[voice]
tone = "precise"
"#
        );
        fs::write(dir.join("persona.toml"), toml).unwrap();
    }

    /// select() returns one entry per installed persona source.
    #[test]
    fn select_returns_all_personas() {
        let tmp = TempDir::new().unwrap();
        let dir_a = tmp.path().join("alpha");
        let dir_b = tmp.path().join("beta");
        make_persona_source(&dir_a, "alpha");
        make_persona_source(&dir_b, "beta");

        let project = TempDir::new().unwrap();
        let inputs = SelectionInputs {
            project_root: project.path(),
            task_hint: None,
            source_dirs: vec![dir_a, dir_b],
            prefs: Preferences::new(),
            weights: PolicyWeights::default(),
        };

        let ranked = select(&inputs).unwrap();
        assert_eq!(ranked.len(), 2, "expected one entry per persona");
    }

    /// select() with an empty source_dirs returns an empty vec, not an error.
    #[test]
    fn select_empty_source_dirs_returns_empty() {
        let project = TempDir::new().unwrap();
        let inputs = SelectionInputs {
            project_root: project.path(),
            task_hint: None,
            source_dirs: vec![],
            prefs: Preferences::new(),
            weights: PolicyWeights::default(),
        };

        let ranked = select(&inputs).unwrap();
        assert!(ranked.is_empty(), "expected empty result for no source dirs");
    }

    /// select() with a task hint passes tokens through to the lexical scorer.
    #[test]
    fn select_with_task_hint_does_not_error() {
        let tmp = TempDir::new().unwrap();
        let dir_a = tmp.path().join("alpha");
        make_persona_source(&dir_a, "alpha");

        let project = TempDir::new().unwrap();
        let inputs = SelectionInputs {
            project_root: project.path(),
            task_hint: Some("refactor rust module"),
            source_dirs: vec![dir_a],
            prefs: Preferences::new(),
            weights: PolicyWeights::default(),
        };

        let ranked = select(&inputs).unwrap();
        assert_eq!(ranked.len(), 1);
    }
}

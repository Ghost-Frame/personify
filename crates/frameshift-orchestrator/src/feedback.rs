//! Feedback loop: learn from user overrides to adjust per-persona preference bias.

use std::collections::BTreeMap;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::error::OrchestratorError;

/// Maximum absolute value for any persona bias entry.
const BIAS_MAX: f32 = 0.2;

/// Amount to increase the chosen persona's bias on each override.
const BUMP: f32 = 0.05;

/// Amount to decrease the auto-picked persona's bias on each override.
const DECAY: f32 = 0.03;

/// Per-persona additive scoring bias, persisted across sessions.
///
/// Bias values are clamped to `[-BIAS_MAX, BIAS_MAX]`. Positive bias means
/// the user has historically preferred this persona; negative means they have
/// been switched away from it.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Preferences {
    /// Map from persona name to additive score bias.
    pub bias: BTreeMap<String, f32>,
}

impl Preferences {
    /// Create a new empty `Preferences` with no recorded biases.
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a user override: bump `chosen` bias up and decay `auto_pick` bias down.
    ///
    /// Both adjustments are clamped to `[-BIAS_MAX, BIAS_MAX]`. If `auto_pick`
    /// is `None` or equals `chosen`, only the chosen persona is bumped.
    pub fn record_override(&mut self, auto_pick: Option<&str>, chosen: &str) {
        let chosen_bias = self.bias.entry(chosen.to_string()).or_insert(0.0);
        *chosen_bias = (*chosen_bias + BUMP).min(BIAS_MAX);

        if let Some(auto) = auto_pick {
            if auto != chosen {
                let auto_bias = self.bias.entry(auto.to_string()).or_insert(0.0);
                *auto_bias = (*auto_bias - DECAY).max(-BIAS_MAX);
            }
        }
    }

    /// Return the current additive bias for `persona`, or 0.0 if not recorded.
    pub fn bias_for(&self, persona: &str) -> f32 {
        self.bias.get(persona).copied().unwrap_or(0.0)
    }

    /// Load preferences from a JSON file.
    ///
    /// Returns an empty `Preferences` if the file does not exist.
    pub fn load(path: &Path) -> Result<Self, OrchestratorError> {
        if !path.exists() {
            return Ok(Self::new());
        }
        let data = std::fs::read_to_string(path)?;
        let prefs = serde_json::from_str(&data)?;
        Ok(prefs)
    }

    /// Persist preferences to a JSON file, creating parent directories as needed.
    pub fn save(&self, path: &Path) -> Result<(), OrchestratorError> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let data = serde_json::to_string_pretty(self)?;
        std::fs::write(path, data)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    /// record_override increases bias for the chosen persona.
    #[test]
    fn override_increases_chosen_bias() {
        let mut prefs = Preferences::new();
        prefs.record_override(Some("auto"), "chosen");
        assert!(prefs.bias_for("chosen") > 0.0);
    }

    /// record_override decreases bias for the auto-picked persona.
    #[test]
    fn override_decreases_auto_bias() {
        let mut prefs = Preferences::new();
        prefs.record_override(Some("auto"), "chosen");
        assert!(prefs.bias_for("auto") < 0.0);
    }

    /// Bias is clamped at BIAS_MAX even after many bumps.
    #[test]
    fn bias_clamped_at_max() {
        let mut prefs = Preferences::new();
        for _ in 0..100 {
            prefs.record_override(None, "target");
        }
        assert!(prefs.bias_for("target") <= BIAS_MAX + f32::EPSILON);
    }

    /// Bias is clamped at -BIAS_MAX even after many decays.
    #[test]
    fn bias_clamped_at_min() {
        let mut prefs = Preferences::new();
        for _ in 0..100 {
            prefs.record_override(Some("victim"), "other");
        }
        assert!(prefs.bias_for("victim") >= -BIAS_MAX - f32::EPSILON);
    }

    /// Load returns empty Preferences when the file does not exist.
    #[test]
    fn load_missing_returns_empty() {
        let tmp = TempDir::new().unwrap();
        let prefs = Preferences::load(&tmp.path().join("nonexistent.json")).unwrap();
        assert!(prefs.bias.is_empty());
    }

    /// Save and load round-trip preserves bias values.
    #[test]
    fn save_load_roundtrip() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("prefs.json");

        let mut prefs = Preferences::new();
        prefs.record_override(Some("a"), "b");
        prefs.save(&path).unwrap();

        let loaded = Preferences::load(&path).unwrap();
        assert!((loaded.bias_for("b") - prefs.bias_for("b")).abs() < f32::EPSILON);
        assert!((loaded.bias_for("a") - prefs.bias_for("a")).abs() < f32::EPSILON);
    }
}

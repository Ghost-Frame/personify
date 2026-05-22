//! Durable on/off state for automate mode.

use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::error::OrchestratorError;

/// Whether automate mode is enabled or disabled.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    /// Automate mode is disabled.
    Off,
    /// Automate mode is enabled.
    On,
}

/// Persisted automate mode state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModeState {
    /// The current mode (on or off).
    pub mode: Mode,
}

impl ModeState {
    /// Load mode state from a JSON file.
    ///
    /// Returns `ModeState { mode: Mode::Off }` if the file does not exist.
    pub fn load(path: &Path) -> Result<Self, OrchestratorError> {
        if !path.exists() {
            return Ok(ModeState { mode: Mode::Off });
        }
        let data = std::fs::read_to_string(path)?;
        let state = serde_json::from_str(&data)?;
        Ok(state)
    }

    /// Persist mode state to a JSON file, creating parent directories as needed.
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

    /// Load returns Off when file does not exist.
    #[test]
    fn load_missing_returns_off() {
        let tmp = TempDir::new().unwrap();
        let state = ModeState::load(&tmp.path().join("mode.json")).unwrap();
        assert_eq!(state.mode, Mode::Off);
    }

    /// Save then load round-trips Mode::On.
    #[test]
    fn save_load_roundtrip_on() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("mode.json");
        let state = ModeState { mode: Mode::On };
        state.save(&path).unwrap();
        let loaded = ModeState::load(&path).unwrap();
        assert_eq!(loaded.mode, Mode::On);
    }

    /// Save then load round-trips Mode::Off.
    #[test]
    fn save_load_roundtrip_off() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("mode.json");
        let state = ModeState { mode: Mode::Off };
        state.save(&path).unwrap();
        let loaded = ModeState::load(&path).unwrap();
        assert_eq!(loaded.mode, Mode::Off);
    }
}

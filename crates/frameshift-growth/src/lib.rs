//! Append-only local growth log for Frameshift personas.
//!
//! Each persona installation has a `growth.md` file in the central store.
//! This crate provides a single `append` function that adds timestamped
//! entries. Growth is local-only -- it never leaves the machine.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Errors from growth file operations.
#[derive(Debug, thiserror::Error)]
pub enum GrowthError {
    /// Failed to write to the growth file.
    #[error("failed to write to growth file at {path}: {source}")]
    Io {
        /// Path of the growth file.
        path: PathBuf,
        /// Underlying IO error.
        source: std::io::Error,
    },

    /// The persona name contains path traversal characters.
    #[error("invalid persona name: {0}")]
    InvalidPersonaName(String),

    /// The project ID contains path traversal characters.
    #[error("invalid project id: {0}")]
    InvalidProjectId(String),
}

/// Append a growth entry with the current UTC timestamp.
pub fn append(
    data_root: &Path,
    project_id: &str,
    persona_name: &str,
    entry_text: &str,
) -> Result<(), GrowthError> {
    let ts = format_utc_now();
    append_with_timestamp(data_root, project_id, persona_name, entry_text, &ts)
}

/// Append a growth entry with a caller-supplied timestamp string.
///
/// Exposed for test determinism -- production callers should use `append`.
pub fn append_with_timestamp(
    data_root: &Path,
    project_id: &str,
    persona_name: &str,
    entry_text: &str,
    timestamp: &str,
) -> Result<(), GrowthError> {
    validate_path_component(project_id)
        .map_err(|_| GrowthError::InvalidProjectId(project_id.to_string()))?;
    validate_path_component(persona_name)
        .map_err(|_| GrowthError::InvalidPersonaName(persona_name.to_string()))?;

    let growth_path = data_root
        .join("projects")
        .join(project_id)
        .join("personas")
        .join(persona_name)
        .join("growth.md");

    if let Some(parent) = growth_path.parent() {
        fs::create_dir_all(parent).map_err(|source| GrowthError::Io {
            path: growth_path.clone(),
            source,
        })?;
    }

    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&growth_path)
        .map_err(|source| GrowthError::Io {
            path: growth_path.clone(),
            source,
        })?;

    writeln!(file, "---\n<!-- growth: {} -->\n\n{}\n", timestamp, entry_text)
        .map_err(|source| GrowthError::Io {
            path: growth_path,
            source,
        })?;

    Ok(())
}

/// Reject path components containing traversal sequences or separators.
fn validate_path_component(s: &str) -> Result<(), ()> {
    if s.is_empty() || s.contains("..") || s.contains('/') || s.contains('\\') {
        return Err(());
    }
    Ok(())
}

/// Format the current UTC time as an RFC3339 timestamp.
fn format_utc_now() -> String {
    use std::time::SystemTime;
    let duration = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = duration.as_secs();
    let days = secs / 86400;
    let time_secs = secs % 86400;
    let hours = time_secs / 3600;
    let minutes = (time_secs % 3600) / 60;
    let seconds = time_secs % 60;
    let (year, month, day) = days_to_ymd(days);
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        year, month, day, hours, minutes, seconds
    )
}

/// Convert days since Unix epoch to (year, month, day).
///
/// Uses Howard Hinnant's civil_from_days algorithm.
fn days_to_ymd(days: u64) -> (u64, u64, u64) {
    let z = days + 719468;
    let era = z / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn append_creates_file_when_absent() {
        let tmp = tempfile::tempdir().unwrap();
        append_with_timestamp(
            tmp.path(),
            "proj1",
            "cryptographic",
            "first entry",
            "2026-01-01T00:00:00Z",
        )
        .unwrap();
        let path = tmp
            .path()
            .join("projects/proj1/personas/cryptographic/growth.md");
        assert!(path.exists());
        let content = fs::read_to_string(&path).unwrap();
        assert!(content.contains("first entry"));
    }

    #[test]
    fn append_accumulates_entries() {
        let tmp = tempfile::tempdir().unwrap();
        append_with_timestamp(
            tmp.path(),
            "proj1",
            "rust",
            "entry one",
            "2026-01-01T00:00:00Z",
        )
        .unwrap();
        append_with_timestamp(
            tmp.path(),
            "proj1",
            "rust",
            "entry two",
            "2026-01-02T00:00:00Z",
        )
        .unwrap();
        let path = tmp.path().join("projects/proj1/personas/rust/growth.md");
        let content = fs::read_to_string(&path).unwrap();
        assert!(content.contains("entry one"));
        assert!(content.contains("entry two"));
        assert!(content.find("entry one") < content.find("entry two"));
    }

    #[test]
    fn append_rejects_traversal_in_persona_name() {
        let tmp = tempfile::tempdir().unwrap();
        let result = append_with_timestamp(
            tmp.path(),
            "proj1",
            "../../etc/shadow",
            "evil",
            "2026-01-01T00:00:00Z",
        );
        assert!(result.is_err());
    }

    #[test]
    fn append_rejects_traversal_in_project_id() {
        let tmp = tempfile::tempdir().unwrap();
        let result = append_with_timestamp(
            tmp.path(),
            "../../etc",
            "persona",
            "evil",
            "2026-01-01T00:00:00Z",
        );
        assert!(result.is_err());
    }

    #[test]
    fn append_with_timestamp_inserts_header() {
        let tmp = tempfile::tempdir().unwrap();
        append_with_timestamp(
            tmp.path(),
            "p",
            "persona",
            "body text",
            "2026-01-01T00:00:00Z",
        )
        .unwrap();
        let path = tmp.path().join("projects/p/personas/persona/growth.md");
        let content = fs::read_to_string(&path).unwrap();
        assert!(content.contains("2026-01-01T00:00:00Z"));
        assert!(content.contains("body text"));
    }

    #[test]
    fn append_rejects_empty_persona_name() {
        let tmp = tempfile::tempdir().unwrap();
        let result = append_with_timestamp(tmp.path(), "proj", "", "text", "ts");
        assert!(result.is_err());
    }

    #[test]
    fn format_utc_now_produces_valid_timestamp() {
        let ts = format_utc_now();
        assert!(ts.ends_with('Z'));
        assert_eq!(ts.len(), 20);
    }
}

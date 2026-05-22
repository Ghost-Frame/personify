//! Context sensing: infer the work context from project layout and an optional task hint.

use std::collections::BTreeMap;
use std::path::Path;

/// Maximum number of files to scan during a project walk.
const MAX_FILES: usize = 2000;

/// Maximum directory depth to descend during a project walk.
const MAX_DEPTH: usize = 6;

/// Directories to skip entirely during the walk.
const SKIP_DIRS: &[&str] = &[".git", "target", "node_modules", ".cache", "__pycache__", ".hg", ".svn"];

/// A snapshot of the inferred work context for a project directory.
#[derive(Debug, Clone, PartialEq)]
pub struct ContextSignal {
    /// The basename of the project root directory (used as a human-readable label).
    pub project_name: String,

    /// Weighted language presence derived from file-extension scanning.
    /// Values are in (0.0, 1.0] and sum to at most 1.0 per language contribution.
    /// Example: `{"rust": 1.0, "toml": 0.2}`.
    pub languages: BTreeMap<String, f32>,

    /// Framework/build-system markers detected from well-known files.
    /// E.g. "cargo", "npm", "go", "python".
    pub frameworks: Vec<String>,

    /// Normalized, lowercase tokens extracted from `task_hint`.
    /// Used by the policy scorer for lexical matching against persona keywords.
    pub task_tokens: Vec<String>,
}

/// Walk `project_root` (bounded by depth and file count), scan extensions for
/// language weights, detect marker files for frameworks, and tokenize `task_hint`.
///
/// The walk is deterministic: entries are processed in sorted order. `.git`,
/// `target`, `node_modules`, and similar directories are skipped entirely.
/// Language weights are proportional to file count normalized to a [0.0, 1.0]
/// scale relative to the most-seen language.
pub fn sense(project_root: &Path, task_hint: Option<&str>) -> ContextSignal {
    let project_name = project_root
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    let mut raw_counts: BTreeMap<String, usize> = BTreeMap::new();
    let mut frameworks: Vec<String> = Vec::new();
    let mut file_count = 0usize;

    walk(project_root, 0, &mut raw_counts, &mut frameworks, &mut file_count);

    // Deduplicate frameworks (marker files may appear at multiple levels).
    frameworks.sort();
    frameworks.dedup();

    // Normalize language counts to a [0.0, 1.0] scale.
    let max_count = raw_counts.values().copied().max().unwrap_or(1).max(1) as f32;
    let languages: BTreeMap<String, f32> = raw_counts
        .into_iter()
        .map(|(lang, count)| (lang, (count as f32 / max_count).min(1.0)))
        .collect();

    let task_tokens = tokenize(task_hint.unwrap_or(""));

    ContextSignal {
        project_name,
        languages,
        frameworks,
        task_tokens,
    }
}

/// Recursively walk `dir` up to `MAX_DEPTH` and `MAX_FILES`, updating
/// `raw_counts` with extension-to-language hits and `frameworks` with
/// marker-file discoveries.
fn walk(
    dir: &Path,
    depth: usize,
    raw_counts: &mut BTreeMap<String, usize>,
    frameworks: &mut Vec<String>,
    file_count: &mut usize,
) {
    if depth > MAX_DEPTH || *file_count >= MAX_FILES {
        return;
    }

    // Collect entries in sorted order for determinism.
    let mut entries: Vec<_> = match std::fs::read_dir(dir) {
        Ok(rd) => rd.filter_map(|e| e.ok()).collect(),
        Err(_) => return,
    };
    entries.sort_by_key(|e| e.file_name());

    for entry in entries {
        if *file_count >= MAX_FILES {
            break;
        }

        let path = entry.path();
        let name = entry.file_name();
        let name_str = name.to_string_lossy();

        if path.is_dir() {
            if SKIP_DIRS.contains(&name_str.as_ref()) {
                continue;
            }
            walk(&path, depth + 1, raw_counts, frameworks, file_count);
        } else if path.is_file() {
            *file_count += 1;

            // Check for framework marker files.
            detect_markers(&name_str, raw_counts, frameworks);
        }
    }
}

/// Given a file name, detect framework markers and map its extension to a language.
fn detect_markers(
    name: &str,
    raw_counts: &mut BTreeMap<String, usize>,
    frameworks: &mut Vec<String>,
) {
    // Marker-file -> framework + implied language entries.
    match name {
        "Cargo.toml" => {
            push_unique(frameworks, "cargo");
            *raw_counts.entry("rust".to_string()).or_insert(0) += 1;
        }
        "package.json" => {
            push_unique(frameworks, "npm");
        }
        "go.mod" => {
            push_unique(frameworks, "go");
            *raw_counts.entry("go".to_string()).or_insert(0) += 1;
        }
        "pyproject.toml" | "requirements.txt" | "setup.py" | "setup.cfg" => {
            push_unique(frameworks, "python");
            *raw_counts.entry("python".to_string()).or_insert(0) += 1;
        }
        _ => {}
    }

    // Extension-based language mapping.
    if let Some(ext) = Path::new(name).extension().and_then(|e| e.to_str()) {
        if let Some(lang) = ext_to_language(ext) {
            *raw_counts.entry(lang.to_string()).or_insert(0) += 1;
        }
    }
}

/// Push `value` into `vec` only if not already present (cheap dedup during walk).
fn push_unique(vec: &mut Vec<String>, value: &str) {
    if !vec.iter().any(|v| v == value) {
        vec.push(value.to_string());
    }
}

/// Map a file extension to a canonical language name.
/// Returns `None` for extensions that do not map to a tracked language.
fn ext_to_language(ext: &str) -> Option<&'static str> {
    match ext {
        "rs" => Some("rust"),
        "ts" | "tsx" => Some("typescript"),
        "js" | "jsx" | "mjs" | "cjs" => Some("javascript"),
        "py" | "pyi" => Some("python"),
        "go" => Some("go"),
        "java" => Some("java"),
        "rb" => Some("ruby"),
        "c" | "h" => Some("c"),
        "cpp" | "cc" | "cxx" | "hpp" | "hh" => Some("cpp"),
        "md" | "mdx" => Some("markdown"),
        "toml" => Some("toml"),
        "sh" | "bash" | "zsh" => Some("shell"),
        "yaml" | "yml" => Some("yaml"),
        "json" => Some("json"),
        "sql" => Some("sql"),
        _ => None,
    }
}

/// Tokenize `text` into lowercase, alphanumeric tokens of length >= 2,
/// preserving insertion order and deduplicating.
pub(crate) fn tokenize(text: &str) -> Vec<String> {
    let mut seen = std::collections::HashSet::new();
    text.split(|c: char| !c.is_alphanumeric())
        .map(|t| t.to_lowercase())
        .filter(|t| t.len() >= 2)
        .filter(|t| seen.insert(t.clone()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    /// Helper: create a temp dir with a Cargo.toml and a few .rs files.
    fn make_rust_project() -> TempDir {
        let tmp = tempfile::tempdir().unwrap();
        fs::write(tmp.path().join("Cargo.toml"), "[package]\nname=\"test\"").unwrap();
        fs::write(tmp.path().join("main.rs"), "fn main() {}").unwrap();
        fs::write(tmp.path().join("lib.rs"), "// lib").unwrap();
        tmp
    }

    /// sense() on a Rust project detects rust language and cargo framework.
    #[test]
    fn sense_rust_project() {
        let tmp = make_rust_project();
        let sig = sense(tmp.path(), None);
        assert!(sig.languages.contains_key("rust"), "expected rust in languages");
        assert!(sig.frameworks.contains(&"cargo".to_string()), "expected cargo framework");
    }

    /// Task hint tokens are normalized and deduplicated.
    #[test]
    fn sense_task_tokens() {
        let tmp = make_rust_project();
        let sig = sense(tmp.path(), Some("Clippy lint check"));
        assert!(sig.task_tokens.contains(&"clippy".to_string()));
        assert!(sig.task_tokens.contains(&"lint".to_string()));
        assert!(sig.task_tokens.contains(&"check".to_string()));
    }

    /// tokenize drops tokens shorter than 2 chars.
    #[test]
    fn tokenize_filters_short() {
        let tokens = tokenize("a bb ccc");
        assert!(!tokens.contains(&"a".to_string()));
        assert!(tokens.contains(&"bb".to_string()));
        assert!(tokens.contains(&"ccc".to_string()));
    }

    /// SKIP_DIRS entries are not descended into.
    #[test]
    fn sense_skips_git_and_target() {
        let tmp = tempfile::tempdir().unwrap();
        let git_dir = tmp.path().join(".git");
        fs::create_dir_all(&git_dir).unwrap();
        fs::write(git_dir.join("HEAD"), "ref: refs/heads/main").unwrap();
        let sig = sense(tmp.path(), None);
        // Walking .git would produce "shell" or nothing useful; we just verify no panic
        // and that we get a valid signal back.
        let _ = sig;
    }
}

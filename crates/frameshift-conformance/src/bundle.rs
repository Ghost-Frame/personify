use crate::case::TestCase;
use crate::error::ConformanceError;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::Path;

/// A named, versioned collection of [`TestCase`]s a persona ships with.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TestBundle {
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub tests: Vec<TestCase>,
}

/// Canonical sha256 hex digest of the bundle's TOML serialization.
///
/// The hash is stable as long as `toml::to_string` produces a deterministic
/// rendering of the typed structure (it does for the fields we use). The same
/// value is stored in [`frameshift_pack::ConformanceBaseline::bundle_hash`].
pub fn bundle_hash(bundle: &TestBundle) -> Result<String, ConformanceError> {
    let canonical = toml::to_string(bundle)?;
    let mut hasher = Sha256::new();
    hasher.update(canonical.as_bytes());
    Ok(hex::encode(hasher.finalize()))
}

/// Load `bundle.toml` from a directory.
pub fn load_from_dir(dir: &Path) -> Result<TestBundle, ConformanceError> {
    let path = dir.join("bundle.toml");
    if !path.exists() {
        return Err(ConformanceError::MissingBundle(dir.to_path_buf()));
    }
    let raw = std::fs::read_to_string(&path).map_err(|source| ConformanceError::Io {
        path: path.clone(),
        source,
    })?;
    let bundle: TestBundle = toml::from_str(&raw)?;
    Ok(bundle)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::case::{ExpectedBehavior, ScorerKind};

    #[test]
    fn bundle_round_trip_toml() {
        let bundle = TestBundle {
            name: "cryptographic".to_string(),
            version: "0.1.0".to_string(),
            tests: vec![TestCase {
                id: "rejects-md5".to_string(),
                prompt: "Should I use MD5 for password hashing?".to_string(),
                expected: ExpectedBehavior::Contains {
                    value: "argon2".to_string(),
                },
                scorer: ScorerKind::Substring,
            }],
        };

        let serialized = toml::to_string(&bundle).expect("serialize");
        let parsed: TestBundle = toml::from_str(&serialized).expect("parse");
        assert_eq!(bundle, parsed);

        let h1 = bundle_hash(&bundle).expect("hash");
        let h2 = bundle_hash(&parsed).expect("hash");
        assert_eq!(h1, h2);
        assert_eq!(h1.len(), 64);
    }
}

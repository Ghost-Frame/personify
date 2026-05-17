use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum ConformanceError {
    #[error("missing bundle.toml in {0}")]
    MissingBundle(PathBuf),

    #[error("failed to read {path}: {source}")]
    Io {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("failed to parse bundle.toml: {0}")]
    BundleParse(#[from] toml::de::Error),

    #[error("failed to serialize bundle: {0}")]
    BundleSerialize(#[from] toml::ser::Error),

    #[error("runner failure: {0}")]
    Runner(String),
}

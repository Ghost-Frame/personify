use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum SourceError {
    #[error("i/o error at {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("toml deserialize error in {path}: {source}")]
    TomlDeserialize {
        path: PathBuf,
        #[source]
        source: toml::de::Error,
    },

    #[error("toml serialize error: {0}")]
    TomlSerialize(#[from] toml::ser::Error),

    #[error("required persona source file missing: {0}")]
    MissingFile(PathBuf),

    #[error("invalid layer '{0}' -- expected one of L1, L2, L3")]
    InvalidLayer(String),
}

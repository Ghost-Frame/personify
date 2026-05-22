/// Errors produced by daemon operations including I/O, JSON parsing,
/// client calls, growth file writes, and file watcher setup.
#[derive(Debug, thiserror::Error)]
pub enum DaemonError {
    /// Underlying I/O failure.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON serialization or deserialization failure.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Error propagated from the frameshift-client library.
    #[error("client error: {0}")]
    Client(String),

    /// Error propagated from the frameshift-growth library.
    #[error("growth error: {0}")]
    Growth(String),

    /// Error from the notify file watcher setup or event stream.
    #[error("watcher error: {0}")]
    Watcher(String),
}

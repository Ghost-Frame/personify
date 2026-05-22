/// Errors from MCP server operations.
#[derive(Debug, thiserror::Error)]
pub enum McpError {
    /// Wraps standard I/O failures from stdin/stdout handling.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    /// Wraps JSON serialization/deserialization failures.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    /// Wraps errors returned by the frameshift-client library.
    #[error("client error: {0}")]
    Client(String),
    /// Wraps errors returned by the frameshift-growth library.
    #[error("growth error: {0}")]
    Growth(String),
}

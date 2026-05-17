use thiserror::Error;

/// Errors raised by the capability sandbox runtime.
#[derive(Debug, Error)]
pub enum CapabilityError {
    /// A tool was invoked that requires a capability the pack manifest did not declare.
    #[error("undeclared capability invoked: {0}")]
    UndeclaredCapability(String),
    /// The manifest itself was malformed or internally inconsistent.
    #[error("invalid capability manifest: {0}")]
    InvalidManifest(String),
}

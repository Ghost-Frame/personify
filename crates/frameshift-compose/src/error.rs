use frameshift_source::SourceError;

#[derive(Debug, thiserror::Error)]
pub enum ComposeError {
    #[error("source error: {0}")]
    Source(#[from] SourceError),

    #[error("could not resolve persona spec '{spec}': {reason}")]
    Unresolved { spec: String, reason: String },

    #[error("invalid persona spec '{0}' -- expected '<name>' or '<name>@<version>'")]
    InvalidSpec(String),

    #[error("composition conflict(s) detected: {count}")]
    Conflicted { count: usize },
}

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

    /// A mixin or the root persona (without `override_inherited = true`) attempted
    /// to override an L1 rule from the base persona. Per threat model SD6, only
    /// the root persona can override inherited L1 rules, and only with the
    /// explicit opt-in flag set.
    #[error("L1 rule '{rule_id}' from {base_layer} cannot be overridden by {mixin_layer}")]
    L1Override {
        /// The `id` of the L1 rule that was targeted.
        rule_id: String,
        /// Human-readable description of the layer that owns the L1 rule.
        base_layer: String,
        /// Human-readable description of the layer that attempted the override.
        mixin_layer: String,
    },
}

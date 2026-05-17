use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// Summary of which declared capabilities were actually exercised at runtime.
///
/// `unused` is the difference `declared \ used` and represents tightening
/// candidates: capabilities the manifest could safely drop on the next version.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct UsageReport {
    pub declared: BTreeSet<String>,
    pub used: BTreeSet<String>,
    pub unused: BTreeSet<String>,
}

impl UsageReport {
    /// Build a report from a declared and an observed-used set, deriving `unused`.
    pub fn new(declared: BTreeSet<String>, used: BTreeSet<String>) -> Self {
        let unused: BTreeSet<String> = declared.difference(&used).cloned().collect();
        Self {
            declared,
            used,
            unused,
        }
    }

    /// Declared capabilities that were never invoked. Tightening candidates.
    pub fn unused_capabilities(&self) -> &BTreeSet<String> {
        &self.unused
    }

    /// Capabilities that were invoked but never declared. Sandbox violations.
    pub fn undeclared_invocations(&self) -> BTreeSet<String> {
        self.used.difference(&self.declared).cloned().collect()
    }
}

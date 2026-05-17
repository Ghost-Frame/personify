use serde::{Deserialize, Serialize};

use crate::report::UsageReport;

/// Advisory signal derived from a [`UsageReport`]. These are not errors: they
/// surface tightening opportunities and sandbox drift to operators.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Warning {
    /// A capability was declared in the manifest but never invoked. Candidate
    /// for removal on the next pack version.
    UnusedCapability(String),
    /// A capability was invoked at runtime but was not declared in the manifest.
    /// Indicates either a sandbox bypass or a manifest that needs widening.
    UndeclaredCapabilityInvoked(String),
}

/// Derive the full set of advisory warnings from a usage report.
pub fn emit_warnings(report: &UsageReport) -> Vec<Warning> {
    let mut warnings = Vec::new();
    for cap in report.unused_capabilities() {
        warnings.push(Warning::UnusedCapability(cap.clone()));
    }
    for cap in report.undeclared_invocations() {
        warnings.push(Warning::UndeclaredCapabilityInvoked(cap));
    }
    warnings
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn emit_warnings_covers_both_categories() {
        let declared: BTreeSet<String> = ["Read", "Edit"].into_iter().map(String::from).collect();
        let used: BTreeSet<String> = ["Read", "Bash"].into_iter().map(String::from).collect();
        let report = UsageReport::new(declared, used);

        let warnings = emit_warnings(&report);
        assert!(warnings.contains(&Warning::UnusedCapability("Edit".to_string())));
        assert!(warnings.contains(&Warning::UndeclaredCapabilityInvoked("Bash".to_string())));
        assert_eq!(warnings.len(), 2);
    }
}

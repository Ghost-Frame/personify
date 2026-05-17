//! Runtime capability sandbox for frameshift personas.
//!
//! The pack manifest (see [`frameshift_pack::CapabilityManifest`]) is the static
//! declaration of what a persona is allowed to do. This crate provides the runtime side
//! that consumes it: filtering tool lists down to declared capabilities and tracking
//! actual invocations so we can report tightening candidates (declared but unused).

pub mod error;
pub mod filter;
pub mod report;
pub mod tracker;
pub mod warning;

pub use error::CapabilityError;
pub use filter::{CapabilityFilter, Tool};
pub use report::UsageReport;
pub use tracker::UsageTracker;
pub use warning::{emit_warnings, Warning};

// Re-export the static manifest type so downstream consumers do not have to depend on
// frameshift-pack directly just to construct a filter.
pub use frameshift_pack::CapabilityManifest;

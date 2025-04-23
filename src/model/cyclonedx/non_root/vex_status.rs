//! VEX status definitions according to the CycloneDX specification.
//!
//! These status values indicate whether a component is affected by a vulnerability,
//! following both the CycloneDX 1.4 and 1.5 specifications.
//!
use serde_derive::{Deserialize, Serialize};

/// Status of a component with respect to a vulnerability.
///
/// Values follow the CycloneDX VEX format specification, which includes
/// both the original VEX statuses and the newer statuses added in CycloneDX 1.4.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VexStatus {
    Affected,
    Unaffected,
    Unknown,
    NotAffected,        // CycloneDX 1.4
    Fixed,              // CycloneDX 1.4
    UnderInvestigation, // CycloneDX 1.4
}

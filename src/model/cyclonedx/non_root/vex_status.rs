use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VexStatus {
    Affected,
    Unaffected,
    Unknown,
    NotAffected, // CycloneDX 1.4
    Fixed, // CycloneDX 1.4
    UnderInvestigation, // CycloneDX 1.4
}

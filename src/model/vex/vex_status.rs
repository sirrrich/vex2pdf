use serde_derive::{Deserialize, Serialize};


#[derive(Debug, Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VexStatus {
    Affected,
    Fixed,
    UnderInvestigation,
    NotAffected,
}

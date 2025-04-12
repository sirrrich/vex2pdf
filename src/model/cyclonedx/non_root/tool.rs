use super::*;
use hash::Hash;
use external_reference::ExternalReference;
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tool {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hashes: Option<Vec<Hash>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub externalReferences: Option<Vec<ExternalReference>>,
}
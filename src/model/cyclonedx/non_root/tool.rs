use super::*;
use component::component::Component;
use external_reference::ExternalReference;
use hash::Hash;
use service::Service;

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

    #[serde(rename = "externalReferences", skip_serializing_if = "Option::is_none")]
    pub external_references: Option<Vec<ExternalReference>>,
}

// Add a new struct for the modern format
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModernTools {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<Component>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<Service>>,
}

// Create an enum to handle both formats with serde
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Tools {
    Modern(ModernTools),
    Legacy(Vec<Tool>),
}

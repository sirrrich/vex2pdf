use super::*;
use organizational_entity::OrganizationalEntity;
use hash::Hash;
use license::Licenses;
use swid::Swid;
use pedigree::Pedigree;
use external_reference::ExternalReference;
use property::Property;
use evidence::Evidence;
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Component {
    #[serde(rename = "type")]
    pub component_type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bom_ref: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplier: Option<OrganizationalEntity>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,

    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hashes: Option<Vec<Hash>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub licenses: Option<Licenses>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub copyright: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpe: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub purl: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub swid: Option<Swid>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pedigree: Option<Pedigree>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub externalReferences: Option<Vec<ExternalReference>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<Property>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<Component>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence: Option<Evidence>,
}






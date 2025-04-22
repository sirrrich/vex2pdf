use super::*;
use data_classification::DataClassification;
use external_reference::ExternalReference;
use license::Licenses;
use property::Property;
use organizational_entity::OrganizationalEntity;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Service {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bom_ref: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<OrganizationalEntity>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,

    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub authenticated: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_trust_boundary: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<DataClassification>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub licenses: Option<Licenses>,

    #[serde(rename="externalReferences",skip_serializing_if = "Option::is_none")]
    pub external_references: Option<Vec<ExternalReference>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<Property>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<Service>>,
}


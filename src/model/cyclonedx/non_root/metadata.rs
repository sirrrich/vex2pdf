use super::*;
use component::component::Component;
use document_metadata::DocumentMetadata;
use license::*;
use person::Person;
use product::Product;
use property::Property;
use tool::Tool;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub authors: Option<Vec<Person>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<Component>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacture: Option<Person>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplier: Option<Person>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub licenses: Option<Licenses>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<Property>>,

    // These appear to be custom fields for your VEX implementation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<DocumentMetadata>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<Product>,
}

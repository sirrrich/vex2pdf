use super::*;
use contact::Contact;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationalEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Vec<Contact>>,
}

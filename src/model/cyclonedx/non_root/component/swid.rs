use super::*;
use attached_text::AttachedText;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Swid {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tagId: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tagVersion: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<AttachedText>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

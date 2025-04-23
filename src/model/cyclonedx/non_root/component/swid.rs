use super::*;
use attached_text::AttachedText;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Swid {
    #[serde(rename = "tagId", skip_serializing_if = "Option::is_none")]
    pub tag_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    #[serde(rename = "tagVersion", skip_serializing_if = "Option::is_none")]
    pub tag_version: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<AttachedText>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

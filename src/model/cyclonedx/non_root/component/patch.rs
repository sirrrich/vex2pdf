use super::*;
use attached_text::AttachedText;
use issue::Issue;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Patch {
    #[serde(rename = "type")]
    pub patch_type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub diff: Option<AttachedText>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolves: Option<Vec<Issue>>,
}

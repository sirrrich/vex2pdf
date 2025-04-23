use super::*;
use identifiable_action::IdentifiableAction;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Commit {
    pub uid: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<IdentifiableAction>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub committer: Option<IdentifiableAction>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

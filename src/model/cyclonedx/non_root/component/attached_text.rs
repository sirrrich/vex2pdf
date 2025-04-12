use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttachedText {
    pub content: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub contentType: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
}


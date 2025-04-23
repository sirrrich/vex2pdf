use super::*;
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CopyrightEvidence {
    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
}

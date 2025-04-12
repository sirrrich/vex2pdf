use super::*;
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Affects {
    pub ref_: String,

    #[serde(rename = "ref")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}
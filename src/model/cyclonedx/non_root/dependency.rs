use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Dependency {
    
    #[serde(rename="ref")]
    pub ref_field: String,
    #[serde(rename = "dependsOn", skip_serializing_if = "Option::is_none")]
    pub depends_on: Option<Vec<String>>,
}
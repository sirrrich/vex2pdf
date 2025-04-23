use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Composition {
    pub aggregate: String,
    pub assemblies: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<String>>,
    pub completeness: Completeness,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Completeness {
    /// Specifies if components are known to be complete or incomplete
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<CompletenessType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<CompletenessType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<CompletenessType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vulnerabilities: Option<CompletenessType>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompletenessType {
    #[serde(rename = "complete")]
    Complete,

    #[serde(rename = "incomplete")]
    Incomplete,

    #[serde(rename = "unknown")]
    Unknown,

    #[serde(rename = "not_specified")]
    NotSpecified,
}

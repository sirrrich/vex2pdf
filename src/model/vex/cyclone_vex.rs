use serde_derive::{Deserialize, Serialize};
use super::document_metadata::DocumentMetadata;
use super::vulnerability_statement::VulnerabilityStatement;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CycloneVex {
    // Document metadata
    pub document: DocumentMetadata,
    // Vulnerability statements
    pub vulnerability_statements: Vec<VulnerabilityStatement>,
}

impl CycloneVex {
    pub fn from_json(json_str: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_str)
    }

    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file_content = std::fs::read_to_string(path)?;
        let vex = Self::from_json(&file_content)?;
        Ok(vex)
    }

    /// Convert to JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

}

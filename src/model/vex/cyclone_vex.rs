use super::document_metadata::DocumentMetadata;
use super::vulnerability_statement::VulnerabilityStatement;
use serde_derive::{Deserialize, Serialize};
use std::path::Path;
use std::{fs, io};

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

    /// Parse CycloneVex from a file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, io::Error> {
        let content = fs::read_to_string(path)?;
        Self::from_json(&content).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    /// Convert to JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

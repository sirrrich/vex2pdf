//! Core data structure representing a CycloneDX VEX document.
//!
//! This module implements the root structure for CycloneDX VEX format according to
//! the CycloneDX 1.5 specification.

/// The root structure of a CycloneDX VEX document.
///
/// Represents a complete VEX document with all components, vulnerabilities,
/// and metadata according to the CycloneDX 1.5 specification.
use super::*;
use non_root::component::component::Component;
use non_root::composition::Composition;
use non_root::dependency::Dependency;
use non_root::external_reference::ExternalReference;
use non_root::metadata::Metadata;
use non_root::service::Service;
use non_root::vulnerability::Vulnerability;

use std::path::Path;
use std::{fs, io};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CycloneDxVex {
    // Required fields
    #[serde(rename = "bomFormat")]
    pub bom_format: String, // Must be "CycloneDX"
    #[serde(rename = "specVersion")]
    pub spec_version: String, // Like "1.4" or "1.5"
    pub version: i32, // Document version, default 1

    // Optional fields
    #[serde(rename = "serialNumber", skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>, // RFC-4122 UUID

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<Component>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<Service>>,

    #[serde(rename = "externalReferences", skip_serializing_if = "Option::is_none")]
    pub external_references: Option<Vec<ExternalReference>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<Dependency>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub compositions: Option<Vec<Composition>>,

    // VEX-specific section
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vulnerabilities: Option<Vec<Vulnerability>>,
}

impl CycloneDxVex {
    pub fn from_json(json_str: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_str)
    }

    /// Parse CycloneDxVex from a file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, io::Error> {
        let content = fs::read_to_string(path)?;
        Self::from_json(&content).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    /// Convert to JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Validate against the CycloneDX schema
    pub fn validate(&self) -> Result<(), String> {
        // This is a placeholder implementation
        if self.bom_format != "CycloneDX" {
            return Err("bomFormat must be 'CycloneDX'".to_string());
        }

        // Validate other required fields and format requirements

        todo!();
    }
}

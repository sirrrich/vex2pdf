//! # vex2pdf library
//!
//! Core functionality for converting CycloneDX VEX documents to PDF format.
//!
//! This library provides:
//! - CycloneDX VEX data model definitions
//! - PDF generation capabilities
//! - Utilities for processing VEX documents
//!
//! ## Architecture
//!
//! The library is organized into modules:
//! - `model`: Data structures representing the CycloneDX VEX format
//! - `pdf`: PDF generation functionality

pub mod model {
    pub mod cyclonedx {

        pub mod root {
            use super::non_root;
            use serde_derive::{Deserialize, Serialize};
            pub mod cyclone_vex;
        }

        pub mod non_root {
            use serde_derive::{Deserialize, Serialize};

            pub mod advisory;
            pub mod affects;
            pub mod composition;
            pub mod credits;
            pub mod data_classification;
            pub mod dependency;
            pub mod document_metadata;
            pub mod external_reference;
            pub mod hash;
            pub mod individual;
            pub mod license;
            pub mod metadata;
            pub mod organization;
            pub mod organizational_entity;
            pub mod person;
            pub mod product;
            pub mod property;
            pub mod service;
            pub mod source;
            pub mod tool;
            pub mod tracking_info;
            pub mod vex_status;
            pub mod vulnerability;
            pub mod vulnerability_analysis;
            pub mod vulnerability_rating;
            pub mod vulnerability_reference;
            pub mod vulnerability_statement;

            pub mod component {
                use super::external_reference;
                use super::hash;
                use super::license;
                use super::property;
                use serde_derive::{Deserialize, Serialize};
                pub mod attached_text;
                pub mod commit;
                pub mod component;
                pub mod contact;
                pub mod copyright_evidence;
                pub mod evidence;
                pub mod identifiable_action;
                pub mod issue;
                pub mod license_evidence;
                pub mod patch;
                pub mod pedigree;
                pub mod source;
                pub mod swid;
            }
        }
    }
}

pub mod pdf {
    pub mod generator;
}

#[cfg(test)]
mod model_tests {
    use crate::model::cyclonedx::non_root::metadata::Metadata;
    use crate::model::cyclonedx::non_root::tool::Tools;
    use crate::model::cyclonedx::non_root::vex_status::VexStatus::{
        Affected, NotAffected, Unknown,
    };
    use crate::model::cyclonedx::non_root::{
        source::Source, vex_status::VexStatus, vulnerability::Vulnerability,
        vulnerability_analysis::VulnerabilityAnalysis, vulnerability_rating::VulnerabilityRating,
        vulnerability_reference::VulnerabilityReference,
    };
    use crate::model::cyclonedx::root::cyclone_vex::CycloneDxVex;

    fn create_sample_vex() -> CycloneDxVex {
        // Create a VEX document following CycloneDX 1.5 structure
        CycloneDxVex {
            bom_format: "CycloneDX".to_string(),
            spec_version: "1.5".to_string(),
            version: 1,
            serial_number: Some("urn:uuid:3e671687-395b-41f5-a30f-a58921a69b79".to_string()),
            metadata: Some(Metadata {
                timestamp: Some("2023-07-15T10:30:00Z".to_string()),
                tools: Some(Tools::Legacy(vec![])),
                authors: Some(vec![]),
                component: None,
                // other metadata fields
                manufacture: None,
                supplier: None,
                licenses: None,
                properties: None,
                document: None,
                product: None,
            }),
            vulnerabilities: Some(vec![
                Vulnerability {
                    id: "CVE-2023-12345".to_string(),
                    source: Some(Source {
                        name: "NVD".to_string(),
                        url: Some("https://nvd.nist.gov/vuln/detail/CVE-2023-12345".to_string()),
                    }),
                    status: Some(NotAffected),
                    references: Some(vec![
                        VulnerabilityReference {
                            id: "GHSA-vh95-rmgr-6w4m".to_string(),
                            source: Some(Source {
                                name: "GitHub".to_string(),
                                url: Some("https://github.com/advisories/GHSA-vh95-rmgr-6w4m".to_string()),
                            }),
                        }
                    ]),
                    ratings: Some(vec![
                        VulnerabilityRating {
                            source: Some(Source {
                                name: "NVD".to_string(),
                                url: Some("https://nvd.nist.gov/vuln-metrics/cvss/v3-calculator".to_string()),
                            }),
                            score: 8.8,
                            severity: "high".to_string(),
                            method: "CVSSv3.1".to_string(),
                            vector: Some("CVSS:3.1/AV:N/AC:L/PR:N/UI:R/S:U/C:H/I:H/A:H".to_string()),
                        }
                    ]),
                    cwes: Some(vec![79, 89]),
                    description: Some("Known vulnerability in library that allows unauthorized access".to_string()),
                    detail: Some("Detailed explanation of the vulnerability and its potential impact.".to_string()),
                    recommendation: Some("Upgrade to version 1.2.4 or later".to_string()),
                    advisories: None,
                    created: Some("2023-01-10T14:30:00Z".to_string()),
                    published: Some("2023-01-15T08:15:00Z".to_string()),
                    updated: Some("2023-07-10T08:15:00Z".to_string()),
                    credits: None,
                    analysis: Some(VulnerabilityAnalysis {
                        state: Affected,
                        justification: Some("Code is vulnerable as confirmed by security testing".to_string()),
                        response: Some(vec!["will_fix".to_string()]),
                        detail: Some("Security testing confirmed the vulnerability is present and exploitable.".to_string()),
                    }),
                    affects: Some(vec![
                        // Define which components are affected
                    ]),
                    properties: None,
                },
                Vulnerability {
                    id: "CVE-2023-67890".to_string(),
                    source: Some(Source {
                        name: "NVD".to_string(),
                        url: Some("https://nvd.nist.gov/vuln/detail/CVE-2023-67890".to_string()),
                    }),
                    status: Some(Affected),
                    references: None,
                    ratings: Some(vec![
                        VulnerabilityRating {
                            source: Some(Source {
                                name: "NVD".to_string(),
                                url: None,
                            }),
                            score: 6.5,
                            severity: "medium".to_string(),
                            method: "CVSSv3.1".to_string(),
                            vector: Some("CVSS:3.1/AV:N/AC:L/PR:L/UI:N/S:U/C:L/I:L/A:L".to_string()),
                        }
                    ]),
                    cwes: None,
                    description: Some("Component does not use the affected library".to_string()),
                    detail: None,
                    recommendation: None,
                    advisories: None,
                    created: Some("2023-02-20T11:45:00Z".to_string()),
                    published: Some("2023-02-25T16:30:00Z".to_string()),
                    updated: Some("2023-07-10T09:20:00Z".to_string()),
                    credits: None,
                    analysis: Some(VulnerabilityAnalysis {
                        state: NotAffected,
                        justification: Some("code_not_present".to_string()),
                        response: None,
                        detail: Some("This component does not contain the vulnerable code.".to_string()),
                    }),
                    affects: None,
                    properties: None,
                },
                Vulnerability {
                    id: "CVE-2022-98765".to_string(),
                    source: Some(Source {
                        name: "NVD".to_string(),
                        url: Some("https://nvd.nist.gov/vuln/detail/CVE-2022-98765".to_string()),
                    }),
                    status: Some(Unknown),
                    references: None,
                    ratings: Some(vec![
                        VulnerabilityRating {
                            source: Some(Source {
                                name: "NVD".to_string(),
                                url: None,
                            }),
                            score: 7.2,
                            severity: "high".to_string(),
                            method: "CVSSv3.1".to_string(),
                            vector: Some("CVSS:3.1/AV:N/AC:L/PR:H/UI:N/S:U/C:H/I:H/A:H".to_string()),
                        }
                    ]),
                    cwes: Some(vec![89]),
                    description: Some("SQL injection vulnerability in query parameter handling".to_string()),
                    detail: None,
                    recommendation: Some("Fixed in patch 0.9.5-p1".to_string()),
                    advisories: None,
                    created: Some("2022-11-10T08:30:00Z".to_string()),
                    published: Some("2022-11-15T14:45:00Z".to_string()),
                    updated: Some("2023-06-15T14:22:00Z".to_string()),
                    credits: None,
                    analysis: Some(VulnerabilityAnalysis {
                        state: VexStatus::Fixed,
                        justification: Some("Code has been fixed in the current version".to_string()),
                        response: Some(vec!["fix".to_string()]),
                        detail: Some("The vulnerability was fixed in this version with a proper input validation.".to_string()),
                    }),
                    affects: None,
                    properties: None,
                },
                Vulnerability {
                    id: "CVE-2023-54321".to_string(),
                    source: Some(Source {
                        name: "NVD".to_string(),
                        url: Some("https://nvd.nist.gov/vuln/detail/CVE-2023-54321".to_string()),
                    }),
                    status: None,
                    references: None,
                    ratings: Some(vec![
                        VulnerabilityRating {
                            source: Some(Source {
                                name: "Internal".to_string(),
                                url: None,
                            }),
                            score: 5.0,
                            severity: "medium".to_string(),
                            method: "CVSSv3.1".to_string(),
                            vector: Some("CVSS:3.1/AV:N/AC:L/PR:H/UI:N/S:U/C:H/I:N/A:N".to_string()),
                        }
                    ]),
                    cwes: None,
                    description: Some("Investigating reports of sensitive data exposure".to_string()),
                    detail: Some("Potential information disclosure under investigation".to_string()),
                    recommendation: None,
                    advisories: None,
                    created: Some("2023-07-10T09:15:00Z".to_string()),
                    published: Some("2023-07-12T11:30:00Z".to_string()),
                    updated: Some("2023-07-14T11:05:00Z".to_string()),
                    credits: None,
                    analysis: Some(VulnerabilityAnalysis {
                        state: VexStatus::UnderInvestigation,
                        justification: None,
                        response: Some(vec!["investigating".to_string()]),
                        detail: Some("Security team is actively investigating this vulnerability.".to_string()),
                    }),
                    affects: None,
                    properties: None,
                },
            ]),
            components: None,
            services: None,
            external_references: None,
            dependencies: None,
            compositions: None,
        }
    }

    #[test]
    fn test_vex_serialization() {
        let vex = create_sample_vex();

        // Test serialization
        let json = serde_json::to_string_pretty(&vex).expect("Failed to serialize to JSON");
        println!("Serialized VEX: {}", json);

        // Test deserialization
        let deserialized: CycloneDxVex =
            serde_json::from_str(&json).expect("Failed to deserialize from JSON");

        // Verify the round trip works
        assert_eq!(vex.bom_format, deserialized.bom_format);
        assert_eq!(vex.spec_version, deserialized.spec_version);
        assert_eq!(vex.version, deserialized.version);
        // More assertions as needed
    }

    #[test]
    fn test_vex_status_serialization() {
        let statuses = vec![
            VexStatus::Affected,
            VexStatus::Unaffected,
            VexStatus::Unknown,
            VexStatus::NotAffected,        // CycloneDX 1.4
            VexStatus::Fixed,              // CycloneDX 1.4
            VexStatus::UnderInvestigation, // CycloneDX 1.4
        ];

        for status in statuses {
            let json = serde_json::to_string(&status).expect("Failed to serialize VexStatus");
            let deserialized: VexStatus =
                serde_json::from_str(&json).expect("Failed to deserialize VexStatus");
            assert_eq!(status, deserialized);
        }
    }

    #[test]
    fn test_vex_file_io() {
        use std::fs;
        use std::io::Write;

        let vex = create_sample_vex();

        // Create a temporary file
        let mut temp_file = std::env::temp_dir();
        temp_file.push("test_vex.json");

        // Write the VEX to the file
        let json = serde_json::to_string_pretty(&vex).expect("Failed to serialize to JSON");
        let mut file = fs::File::create(&temp_file).expect("Failed to create temp file");
        file.write_all(json.as_bytes())
            .expect("Failed to write to temp file");

        // Read it back
        let content = fs::read_to_string(&temp_file).expect("Failed to read temp file");
        let loaded_vex: CycloneDxVex =
            serde_json::from_str(&content).expect("Failed to parse JSON");

        // Clean up
        fs::remove_file(&temp_file).expect("Failed to remove temp file");

        // Verify
        assert_eq!(vex.bom_format, loaded_vex.bom_format);
        assert_eq!(vex.spec_version, loaded_vex.spec_version);
    }

    #[test]
    fn test_generate_sample_file() {
        use std::fs;
        let vex = create_sample_vex();
        let json = serde_json::to_string_pretty(&vex).expect("Failed to serialize to JSON");

        // Create a sample file in the current directory
        fs::write("sample_vex.json", json).expect("Failed to write sample file");

        println!("Sample VEX file created at sample_vex.json");
    }
}

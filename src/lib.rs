pub mod model {
    pub mod cyclonedx {
        
        pub mod root {
            use serde_derive::{Deserialize, Serialize};
            use super::non_root;
            pub mod cyclone_vex;
        };
        
        pub mod non_root {
            use serde_derive::{Deserialize, Serialize};
            
            pub mod document_metadata;
            pub mod product;
            pub mod tracking_info;
            pub mod vex_status;
            pub mod vulnerability_statement;
            pub mod advisory;
            pub mod affects;
            pub mod credits;
            pub mod individual;
            pub mod organization;
            pub mod property;
            pub mod source;
            pub mod vulnerability;
            pub mod vulnerability_analysis;
            pub mod vulnerability_rating;
            pub mod vulnerability_reference;
            
        }


    }
}

pub mod pdf {
    pub mod generator;
    pub mod styling;
}

#[cfg(test)]
mod model_tests {
    use crate::model::vex::{
        cyclone_vex::CycloneVex, document_metadata::DocumentMetadata, product::Product,
        vex_status::VexStatus, vulnerability_statement::VulnerabilityStatement,
    };

    fn create_sample_vex() -> CycloneVex {
        use crate::model::vex::document_metadata::TrackingInfo;

        CycloneVex {
            document: DocumentMetadata {
                id: "example-vex-123".to_string(),
                version: "1.0".to_string(),
                author: "Test Author".to_string(),
                timestamp: "2023-07-15T10:30:00Z".to_string(),
                title: Some("Vulnerability Disclosure Report".to_string()),
                tracking: Some(TrackingInfo {
                    id: "TRACK-2023-001".to_string(),
                    status: Some("final".to_string()),
                    version: Some(1),
                    timestamp: Some("2023-07-15T10:30:00Z".to_string()),
                }),
            },
            vulnerability_statements: vec![
                VulnerabilityStatement {
                    vulnerability_id: "CVE-2023-12345".to_string(),
                    product: Product {
                        id: "product-123".to_string(),
                        name: "Example Product".to_string(),
                        version: "1.2.3".to_string(),
                        purl: Some("pkg:maven/org.example/library@1.2.3".to_string()),
                        cpe: Some("cpe:2.3:a:example:product:1.2.3:*:*:*:*:*:*:*".to_string()),
                        supplier: Some("Example Inc.".to_string()),
                    },
                    status: VexStatus::Affected,
                    justification: Some("This version contains the vulnerability".to_string()),
                    impact_statement: Some("Remote code execution is possible".to_string()),
                    timestamp: "2023-07-10T08:15:00Z".to_string(),
                    description: Some("Known vulnerability in library that allows unauthorized access".to_string()),
                    remediation: Some("Upgrade to version 1.2.4 or later".to_string()),
                },
                VulnerabilityStatement {
                    vulnerability_id: "CVE-2023-67890".to_string(),
                    product: Product {
                        id: "product-456".to_string(),
                        name: "Another Component".to_string(),
                        version: "2.0.1".to_string(),
                        purl: Some("pkg:npm/example-package@2.0.1".to_string()),
                        cpe: Some("cpe:2.3:a:examplevendor:component:2.0.1:*:*:*:*:*:*:*".to_string()),
                        supplier: Some("Example Vendor LLC".to_string()),
                    },
                    status: VexStatus::NotAffected,
                    justification: Some("This component is not vulnerable".to_string()),
                    impact_statement: None,
                    timestamp: "2023-07-10T09:20:00Z".to_string(),
                    description: Some("Component does not use the affected library".to_string()),
                    remediation: None,
                },
                VulnerabilityStatement {
                    vulnerability_id: "CVE-2022-98765".to_string(),
                    product: Product {
                        id: "product-789".to_string(),
                        name: "Legacy Service".to_string(),
                        version: "0.9.5".to_string(),
                        purl: Some("pkg:golang/github.com/example/service@0.9.5".to_string()),
                        cpe: Some("cpe:2.3:a:example:service:0.9.5:*:*:*:*:*:*:*".to_string()),
                        supplier: None,
                    },
                    status: VexStatus::Fixed,
                    justification: Some("Patched in this version".to_string()),
                    impact_statement: Some("Previously allowed SQL injection".to_string()),
                    timestamp: "2023-06-15T14:22:00Z".to_string(),
                    description: Some("SQL injection vulnerability in query parameter handling".to_string()),
                    remediation: Some("Fixed in patch 0.9.5-p1".to_string()),
                },
                VulnerabilityStatement {
                    vulnerability_id: "CVE-2023-54321".to_string(),
                    product: Product {
                        id: "product-101".to_string(),
                        name: "Microservice API".to_string(),
                        version: "3.1.0".to_string(),
                        purl: Some("pkg:docker/example/api@sha256:7834d71e1256457954a9987cc68c72de8b43f9876543210".to_string()),
                        cpe: None, // Some products might not have CPE identifiers
                        supplier: Some("Internal Development".to_string()),
                    },
                    status: VexStatus::UnderInvestigation,
                    justification: None,
                    impact_statement: Some("Potential information disclosure".to_string()),
                    timestamp: "2023-07-14T11:05:00Z".to_string(),
                    description: Some("Investigating reports of sensitive data exposure".to_string()),
                    remediation: None,
                },
            ],
        }
    }

    #[test]
    fn test_vex_serialization() {
        let vex = create_sample_vex();

        // Test serialization
        let json = vex.to_json().expect("Failed to serialize to JSON");
        println!("Serialized VEX: {}", json);

        // Test deserialization
        let deserialized = CycloneVex::from_json(&json).expect("Failed to deserialize from JSON");

        // Verify the round trip works
        assert_eq!(vex, deserialized);
    }

    #[test]
    fn test_vex_status_serialization() {
        let statuses = vec![
            VexStatus::Affected,
            VexStatus::Fixed,
            VexStatus::UnderInvestigation,
            VexStatus::NotAffected,
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
        let json = vex.to_json().expect("Failed to serialize to JSON");
        let mut file = fs::File::create(&temp_file).expect("Failed to create temp file");
        file.write_all(json.as_bytes())
            .expect("Failed to write to temp file");

        // Read it back
        let loaded_vex = CycloneVex::from_file(&temp_file).expect("Failed to load from file");

        // Clean up
        fs::remove_file(&temp_file).expect("Failed to remove temp file");

        // Verify
        assert_eq!(vex, loaded_vex);
    }

    #[test]
    fn test_generate_sample_file() {
        use std::fs;
        let vex = create_sample_vex();
        let json = vex.to_json().expect("Failed to serialize to JSON");

        // Create a sample file in the current directory
        fs::write("sample_vex.json", json).expect("Failed to write sample file");

        println!("Sample VEX file created at sample_vex.json");
    }
}

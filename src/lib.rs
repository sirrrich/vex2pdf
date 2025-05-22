//! # vex2pdf library
//!
//! Core functionality for converting CycloneDX VEX documents to PDF format.
//!
//! ## CycloneDX Compatibility
//!
//! This library fully supports CycloneDX schema version 1.5 and provides compatibility
//! for version 1.6 documents that only use 1.5 fields. Documents using 1.6-specific
//! fields may not process correctly.
//!
//! ## Features
//!
//! This library provides:
//! - PDF generation capabilities for CycloneDX VEX documents
//! - Support for various VEX elements including vulnerabilities, components, and metadata
//! - Flexible font configuration with environment variable support
//!
//! ## Vulnerabilities Section Behavior
//!
//! By default, the library will:
//! - Display a "Vulnerabilities" section with vulnerability details when vulnerabilities exist
//! - Display a "Vulnerabilities" section with a "No Vulnerabilities reported" message when no vulnerabilities exist
//! - The "No Vulnerabilities" message display can be controlled with the `VEX2PDF_NOVULNS_MSG` environment variable
//!   (set to "false" to hide the section entirely when no vulnerabilities exist)

//!
//! ## Font Configuration
//!
//! The library searches for Liberation Sans fonts in these locations (in order of precedence):
//! 1. Custom directory specified via `VEX2PDF_FONTS_PATH` environment variable (if set)
//! 2. Project-local directory `./fonts/liberation-fonts` (if it exists)
//! 3. User's local fonts directory `~/.local/share/fonts/liberation-fonts` (if it exists)
//! 4. System-wide directory `/usr/share/fonts/liberation-fonts`
//!
//! ## Architecture
//!
//! The library is organized into modules:
//! - `pdf`: PDF generation functionality
//!   - `font_config`: Font configuration and discovery
//!   - `generator`: PDF document generation
//! - `lib_utils`: Utilities and data models used in this library and accompanying runnable
//!

// Re-export cyclonedx-bom models for use by consumers of this library
pub use cyclonedx_bom as model;

pub mod pdf {
    pub mod font_config;
    pub mod generator;
}

pub mod lib_utils {
    pub mod config;
    pub mod env_vars;
    pub mod input_file_type;
    pub mod run_utils;
}

use crate::lib_utils::run_utils::print_copyright;
use lib_utils::config::Config;
use lib_utils::input_file_type::InputFileType;
use lib_utils::run_utils::{find_files, parse_files};
use pdf::generator::PdfGenerator;
use std::error::Error;

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let pdf_generator = PdfGenerator::new();

    if config.show_oss_licenses {
        // show OSS licenses and return
        print_copyright();
        let main_license_text = include_bytes!("../LICENSE.md");
        let trimmed_license =
            String::from_utf8_lossy(main_license_text).replacen("# MIT License", "", 1);
        println!("{}", trimmed_license);
        println!();
        println!("-----------------------------------------------------------------------------\n");
        println!("This software makes use of Liberation Fonts licensed under SIL as follows : ");
        println!();
        let sil_license_text = include_bytes!("../external/fonts/liberation-fonts/LICENSE");

        println!("{}", String::from_utf8_lossy(sil_license_text));

        // abort any processing
        return Ok(());
    }

    // Find json files
    let json_files = find_files(config, InputFileType::JSON)?;
    // Generate PDFs out of given json files
    parse_files(&pdf_generator, &json_files, InputFileType::JSON);

    // Find xml files and parse them
    let xml_files = find_files(config, InputFileType::XML)?;
    // Generate PDFs out of given xml files
    parse_files(&pdf_generator, &xml_files, InputFileType::XML);

    Ok(())
}
#[cfg(test)]
mod tests {
    use cyclonedx_bom::models::bom::Bom;
    use cyclonedx_bom::models::metadata::Metadata;
    use cyclonedx_bom::models::tool::{Tool, Tools};
    use cyclonedx_bom::models::vulnerability::{Vulnerabilities, Vulnerability};

    use cyclonedx_bom::models::vulnerability_rating::{
        Score, ScoreMethod, Severity, VulnerabilityRating, VulnerabilityRatings,
    };
    use cyclonedx_bom::prelude::{DateTime, NormalizedString, SpecVersion, UrnUuid};
    use std::fs;
    use std::io::BufReader;

    fn create_sample_vex() -> Bom {
        // Create a VEX document following CycloneDX structure

        Bom {
            spec_version: SpecVersion::V1_5,
            version: 1,
            serial_number: Some(
                UrnUuid::new("urn:uuid:3e671687-395b-41f5-a30f-a58921a69b79".to_string())
                    .expect("Unable to create urn:uuid"),
            ),
            metadata: Some(Metadata {
                timestamp: Some(DateTime::now().expect("failed to convert date")),
                tools: Some(Tools::List(vec![Tool {
                    name: Some(NormalizedString::new("my_tool")),
                    ..Tool::default()
                }])),
                ..Metadata::default()
            }),
            vulnerabilities: Some(Vulnerabilities(vec![
                Vulnerability {
                    bom_ref: None,
                    id: None,
                    vulnerability_source: None,
                    description: Some(
                        "Known vulnerability in library that allows unauthorized access"
                            .to_string(),
                    ),
                    detail: Some(
                        "Detailed explanation of the vulnerability and its potential impact."
                            .to_string(),
                    ),
                    recommendation: Some("Upgrade to version 1.2.4 or later".to_string()),
                    workaround: None,
                    proof_of_concept: None,
                    advisories: None,
                    created: None,
                    published: None,
                    updated: None,
                    rejected: None,
                    vulnerability_credits: None,
                    tools: None,
                    vulnerability_analysis: None,
                    vulnerability_targets: None,
                    vulnerability_ratings: Some(VulnerabilityRatings(vec![VulnerabilityRating {
                        score: Some(Score::from(8.1)),
                        severity: Some(Severity::High),
                        score_method: Some(ScoreMethod::CVSSv31),
                        vector: Some(NormalizedString::new(
                            "CVSS:3.1/AV:N/AC:L/PR:N/UI:R/S:U/C:H/I:H/A:H",
                        )),
                        vulnerability_source: None,
                        justification: None,
                    }])),

                    vulnerability_references: None,
                    cwes: None,
                    properties: None,
                },
                Vulnerability {
                    bom_ref: None,
                    id: None,
                    vulnerability_source: None,
                    description: Some("Component does not use the affected library".to_string()),
                    detail: Some(
                        "Detailed explanation of the vulnerability and its potential impact."
                            .to_string(),
                    ),
                    recommendation: Some("Upgrade to version 1.2.3 or later".to_string()),
                    workaround: None,
                    proof_of_concept: None,
                    advisories: None,
                    created: None,
                    published: None,
                    updated: None,
                    rejected: None,
                    vulnerability_credits: None,
                    tools: None,
                    vulnerability_analysis: None,
                    vulnerability_targets: None,
                    vulnerability_ratings: Some(VulnerabilityRatings(vec![VulnerabilityRating {
                        score: Some(Score::from(6.5)),
                        severity: Some(Severity::High),
                        score_method: Some(ScoreMethod::CVSSv31),
                        vector: Some(NormalizedString::new(
                            "CVSS:3.1/AV:N/AC:L/PR:L/UI:N/S:U/C:L/I:L/A:L",
                        )),
                        vulnerability_source: None,
                        justification: None,
                    }])),

                    vulnerability_references: None,
                    cwes: None,
                    properties: None,
                },
            ])),
            ..Bom::default()
        }
    }

    #[test]
    fn test_vex_serialization() {
        let vex = create_sample_vex();

        // Test serialization
        let mut output = Vec::<u8>::new();
        vex.clone()
            .output_as_json_v1_5(&mut output)
            .expect("failed to read vex object");

        let json_str = String::from_utf8(output).expect("failed to serialize json object");

        println!("Serialized VEX: {}", json_str);

        let parsed_json =
            serde_json::from_str(&json_str).expect("serde failed to read json from string object");
        let deserialization_result = Bom::parse_json_value(parsed_json);

        // Test deserialization
        match deserialization_result {
            Ok(deserialized) => {
                println!("Deserialized CycloneDX: {:?}", deserialized);
                // Verify the round trip works
                assert_eq!(vex.serial_number, deserialized.serial_number);
                assert_eq!(vex.spec_version, deserialized.spec_version);
            }
            Err(err) => {
                panic!("Deserialization failed: {:?}", err);
            }
        }
    }

    #[test]
    fn test_vex_json_file_io() {
        use std::io::Write;

        let vex = create_sample_vex();
        let mut output = Vec::<u8>::new();
        vex.clone()
            .output_as_json_v1_5(&mut output)
            .expect("failed to read vex object");
        let json_str = String::from_utf8(output).expect("failed to serialize json object");

        // Create a temporary file
        let mut temp_file = std::env::temp_dir();
        temp_file.push("test_vex.json");

        // Write the VEX to the file
        let mut file = fs::File::create(&temp_file).expect("Failed to create temp file");
        file.write_all(json_str.as_bytes())
            .expect("Failed to write to temp file");

        // Read it back
        let content_reader =
            BufReader::new(fs::File::open(&temp_file).expect("failed to open file"));
        let loaded_vex: Bom = Bom::parse_from_json(content_reader).expect("Failed to parse JSON");

        // Clean up
        fs::remove_file(&temp_file).expect("Failed to remove temp file");

        // Verify
        assert_eq!(vex.serial_number, loaded_vex.serial_number);
    }

    #[test]
    fn test_vex_xml_file_io() {
        use std::io::Write;

        let vex = create_sample_vex();
        let mut output = Vec::<u8>::new();
        vex.clone()
            .output_as_xml_v1_5(&mut output)
            .expect("failed to read vex object");
        let xml_str = String::from_utf8(output).expect("failed to serialize json object");

        // Create a temporary file
        let mut temp_file = std::env::temp_dir();
        temp_file.push("test_vex.xml");

        // Write the VEX to the file
        let mut file = fs::File::create(&temp_file).expect("Failed to create temp file");
        file.write_all(xml_str.as_bytes())
            .expect("Failed to write to temp file");

        // Read it back
        let content_reader =
            BufReader::new(fs::File::open(&temp_file).expect("failed to open file"));
        let loaded_vex: Bom =
            Bom::parse_from_xml_v1_5(content_reader).expect("Failed to parse JSON");

        // Clean up
        fs::remove_file(&temp_file).expect("Failed to remove temp file");

        // Verify
        assert_eq!(vex.serial_number, loaded_vex.serial_number);
    }

    #[test]
    fn test_generate_sample_file() {
        let vex = create_sample_vex();
        let mut output = Vec::<u8>::new();
        vex.clone()
            .output_as_json_v1_5(&mut output)
            .expect("failed to read vex object");
        let json_str = String::from_utf8(output).expect("failed to serialize json object");

        // Create a sample file in the current directory
        fs::write("sample_vex.json", json_str).expect("Failed to write sample file");

        println!("Sample VEX file created at sample_vex.json");
    }

    // font config tests

    #[test]
    fn test_font_dirs_construction() {
        use crate::pdf::font_config::FontsDir;
        use std::path::PathBuf;

        let font_dir = FontsDir::new("test-fonts", None);

        // Test system path
        assert_eq!(
            font_dir.system,
            PathBuf::from("/usr/share/fonts/test-fonts")
        );

        // Test project path
        assert_eq!(font_dir.project, PathBuf::from("./fonts/test-fonts"));

        // Test local path (need to account for HOME expansion)
        if let Ok(home) = std::env::var("HOME") {
            let expected_local_path =
                PathBuf::from(format!("{}/.local/share/fonts/test-fonts", home));
            assert_eq!(font_dir.local, expected_local_path);
        }
    }

    #[test]
    fn test_font_dir_precedence() {
        use crate::pdf::font_config::FontsDir;
        use std::env;
        use std::fs;
        use std::path::PathBuf;

        // Create a test struct with paths we can control
        struct TestFontDir {
            font_dir: FontsDir,
        }

        impl TestFontDir {
            fn new() -> Self {
                // Create a FontsDir with custom paths for testing
                let mut font_dir = FontsDir::new("test-fonts", None);

                // Override with test paths
                let temp_dir = env::temp_dir();
                font_dir.system = temp_dir.join("system-fonts");
                font_dir.local = temp_dir.join("local-fonts");
                font_dir.project = temp_dir.join("project-fonts");

                Self { font_dir }
            }

            fn create_dir(&self, path: &PathBuf) {
                let _ = fs::create_dir_all(path);
            }

            fn remove_dir(&self, path: &PathBuf) {
                let _ = fs::remove_dir_all(path);
            }

            fn clean_up(&self) {
                self.remove_dir(&self.font_dir.system);
                self.remove_dir(&self.font_dir.local);
                self.remove_dir(&self.font_dir.project);
            }
        }

        let test = TestFontDir::new();

        // Test 1: Only system directory exists
        test.create_dir(&test.font_dir.system);
        assert_eq!(test.font_dir.get_active_font_dir(), &test.font_dir.system);
        test.remove_dir(&test.font_dir.system);

        // Test 2: Only local directory exists
        test.create_dir(&test.font_dir.local);
        assert_eq!(test.font_dir.get_active_font_dir(), &test.font_dir.local);
        test.remove_dir(&test.font_dir.local);

        // Test 3: Only project directory exists
        test.create_dir(&test.font_dir.project);
        assert_eq!(test.font_dir.get_active_font_dir(), &test.font_dir.project);
        test.remove_dir(&test.font_dir.project);

        // Test 4: All directories exist (should choose project)
        test.create_dir(&test.font_dir.system);
        test.create_dir(&test.font_dir.local);
        test.create_dir(&test.font_dir.project);
        assert_eq!(test.font_dir.get_active_font_dir(), &test.font_dir.project);

        // Test 5: System and local exist (should choose local)
        test.remove_dir(&test.font_dir.project);
        assert_eq!(test.font_dir.get_active_font_dir(), &test.font_dir.local);

        // Clean up
        test.clean_up();
    }

    #[test]
    fn test_font_dir_default() {
        use crate::pdf::font_config::FontsDir;

        let default_font_dir = FontsDir::default();
        let liberation_font_dir = FontsDir::new("liberation-fonts", None);

        // Default should use "liberation-fonts"
        assert_eq!(
            default_font_dir.system.to_string_lossy(),
            liberation_font_dir.system.to_string_lossy()
        );
    }

    #[test]
    fn test_font_dir_env_var() {
        use crate::lib_utils::env_vars::EnvVarNames;
        use crate::pdf::font_config::FontsDir;
        use std::env;
        use std::path::PathBuf;

        // Save original env var value
        let original = env::var(EnvVarNames::FontsPath.as_str()).ok();

        // Set custom path
        let test_path = "/tmp/test-fonts-path";
        env::set_var(EnvVarNames::FontsPath.as_str(), test_path);

        // Test that default constructor picks up the env var
        let default_fonts = FontsDir::default();
        assert_eq!(default_fonts.custom, Some(PathBuf::from(test_path)));

        // Clean up
        if let Some(val) = original {
            env::set_var(EnvVarNames::FontsPath.as_str(), val);
        } else {
            env::remove_var(EnvVarNames::FontsPath.as_str());
        }
    }

    #[test]
    fn test_novulns_msg_env_var_handling() {
        use crate::lib_utils::env_vars::EnvVarNames;
        use std::env;

        // Save original env var value
        let original = env::var(EnvVarNames::NoVulnsMsg.as_str()).ok();

        // Test setting and retrieving the env var
        env::remove_var(EnvVarNames::NoVulnsMsg.as_str());
        assert_eq!(
            env::var(EnvVarNames::NoVulnsMsg.as_str()).is_err(),
            true,
            "Env var should not exist initially"
        );

        env::set_var(EnvVarNames::NoVulnsMsg.as_str(), "false");
        assert_eq!(
            env::var(EnvVarNames::NoVulnsMsg.as_str()).unwrap(),
            "false",
            "Env var should be retrievable with correct value"
        );

        // Clean up
        if let Some(val) = original {
            env::set_var(EnvVarNames::NoVulnsMsg.as_str(), val);
        } else {
            env::remove_var(EnvVarNames::NoVulnsMsg.as_str());
        }
    }

    #[test]
    fn test_embedded_fonts_load_correctly() {
        use crate::pdf::font_config::FontsDir;

        let fonts_dir = FontsDir::default();
        let result = fonts_dir.load_embedded_font_family();
        assert!(
            result.is_ok(),
            "Failed to load embedded fonts: {:?}",
            result.err()
        );

        // Verify we have all font variants
        let font_family = result;

        assert!(font_family.is_ok());
    }

    #[cfg(test)]
    mod tests {
        use crate::lib_utils::env_vars::EnvVarNames;
        use std::env;

        #[test]
        fn test_env_var_behavior() {
            // Use a different enum variant for each test section to avoid interference

            // Test is_on when var not set
            {
                let var = EnvVarNames::ProcessXml;
                env::remove_var(var.as_str());
                assert_eq!(
                    var.is_on(),
                    false,
                    "is_on() should return false when var not set"
                );
            }

            // Test is_on with true values
            {
                let var = EnvVarNames::ProcessXml;
                for value in &["true", "True", "TRUE", "yes", "YES", "1", "on", "ON"] {
                    env::set_var(var.as_str(), value);
                    assert_eq!(var.is_on(), true, "is_on() failed for value: {}", value);
                    env::remove_var(var.as_str()); // Clean up after each test
                }
            }

            // Test is_on with false values
            {
                let var = EnvVarNames::ProcessXml;
                for value in &["false", "False", "FALSE", "no", "NO", "0", "off", "OFF"] {
                    env::set_var(var.as_str(), value);
                    assert_eq!(var.is_on(), false, "is_on() failed for value: {}", value);
                    env::remove_var(var.as_str()); // Clean up after each test
                }
            }

            // Test is_on_or_unset when var not set
            {
                let var = EnvVarNames::ProcessXml;
                env::remove_var(var.as_str());
                assert_eq!(
                    var.is_on_or_unset(),
                    true,
                    "is_on_or_unset() should return true when var not set"
                );
            }

            // Test is_on_or_unset with true values
            {
                let var = EnvVarNames::ProcessXml;
                for value in &["true", "True", "TRUE", "yes", "YES", "1", "on", "ON"] {
                    env::set_var(var.as_str(), value);
                    assert_eq!(
                        var.is_on_or_unset(),
                        true,
                        "is_on_or_unset() failed for value: {}",
                        value
                    );
                    env::remove_var(var.as_str()); // Clean up after each test
                }
            }

            // Test is_on_or_unset with false values
            {
                let var = EnvVarNames::ProcessXml;
                for value in &["false", "False", "FALSE", "no", "NO", "0", "off", "OFF"] {
                    env::set_var(var.as_str(), value);
                    assert_eq!(
                        var.is_on_or_unset(),
                        false,
                        "is_on_or_unset() failed for value: {}",
                        value
                    );
                    env::remove_var(var.as_str()); // Clean up after each test
                }
            }
        }
    }
}

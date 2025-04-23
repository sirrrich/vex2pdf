//! # vex2pdf
//!
//! A command-line tool that converts CycloneDX VEX JSON documents to PDF reports.
//!
//! ## Usage
//!
//! Run the tool in a directory containing VEX JSON files:
//!
//! ```
//! vex2pdf
//! ```
//!
//! The tool will scan for JSON files, process any valid VEX documents, and generate
//! corresponding PDF reports with the same filename but with a .pdf extension.
//!
//! ## Font Requirements
//!
//! This tool requires Liberation Sans fonts to render PDFs correctly.
//! See the README for details on setting up fonts.

use std::error::Error;
use std::fs::{self, File};
use std::io::BufReader;
use std::path::{Path, PathBuf};
use vex2pdf::model::cyclonedx::root::cyclone_vex::CycloneDxVex;
use vex2pdf::pdf::generator::PdfGenerator;

fn main() -> Result<(), Box<dyn Error>> {
    // Get the current directory
    let current_dir = std::env::current_dir()?;
    println!("Scanning for JSON files in: {}", current_dir.display());

    // Find all JSON files in the current directory
    let json_files = find_json_files(&current_dir)?;

    if json_files.is_empty() {
        println!("No JSON files found in the current directory.");
        return Ok(());
    }

    println!("Found {} JSON files", json_files.len());

    // Create PDF generator
    let pdf_generator = PdfGenerator::new();

    // Process each JSON file
    for json_path in json_files {
        println!("Processing: {}", json_path.display());

        // Try to parse the JSON file as a CycloneDxVex
        match parse_vex_json(&json_path) {
            Ok(vex) => {
                // Generate output PDF path with same base name
                let output_path = get_output_pdf_path(&json_path);

                println!("Generating PDF: {}", output_path.display());

                // Generate the PDF
                match pdf_generator.generate_pdf(&vex, &output_path) {
                    Ok(_) => println!("Successfully generated PDF: {}", output_path.display()),
                    Err(e) => println!("Failed to generate PDF for {}: {}", json_path.display(), e),
                }
            }
            Err(e) => println!("Failed to parse {}: {}", json_path.display(), e),
        }
    }

    Ok(())
}

// Find all JSON files in the given directory
fn find_json_files(dir: &Path) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut json_files = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension.to_string_lossy().to_lowercase() == "json" {
                    json_files.push(path);
                }
            }
        }
    }

    Ok(json_files)
}

// Parse a JSON file as CycloneDxVex
fn parse_vex_json(path: &Path) -> Result<CycloneDxVex, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let vex: CycloneDxVex = serde_json::from_reader(reader)?;
    Ok(vex)
}

// Get the output PDF path with the same base name as the JSON file
fn get_output_pdf_path(json_path: &Path) -> PathBuf {
    if let Some(file_stem) = json_path.file_stem() {
        json_path.with_file_name(format!("{}.pdf", file_stem.to_string_lossy()))
    } else {
        json_path.with_extension("pdf")
    }
}

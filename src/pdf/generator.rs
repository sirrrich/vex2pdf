use std::fmt::format;
use std::{error, io};
// src/pdf/generator.rs
use crate::model::cyclonedx::root::cyclone_vex::CycloneDxVex;
use genpdf::elements::{Paragraph, Text};
use genpdf::fonts::{Builtin, FontFamily};
use genpdf::style::{Color, Style, StyledString};
use genpdf::{Document, Element, SimplePageDecorator};
use std::path::Path;

pub struct PdfGenerator {
    // Define default styles as constants or struct fields
    title_style: Style,
    header_style: Style,
    normal_style: Style,
    indent_style: Style,
}

impl PdfGenerator {
    pub fn new() -> Self {
        // Initialize with default styles
        let title_style = Style::new()
            .with_font_size(18)
            .with_color(Color::Rgb(0, 0, 80));

        let header_style = Style::new()
            .with_font_size(14)
            .with_color(Color::Rgb(0, 0, 80));

        let normal_style = Style::new().with_font_size(11);

        let indent_style = Style::new()
            .with_font_size(10)
            .with_color(Color::Rgb(40, 40, 40));

        Self {
            title_style,
            header_style,
            normal_style,
            indent_style,
        }
    }

    pub fn generate_pdf<P: AsRef<Path>>(
        &self,
        vex: &CycloneDxVex,
        output_path: P,
    ) -> Result<(), io::Error> {
        // Set up the document with default fonts
        let font_family = genpdf::fonts::from_files("/usr/share/fonts/liberation-fonts", "LiberationSans", None)
            .expect("Failed to load font family");
        let mut doc = Document::new(font_family);

        doc.set_title("VEX Report");
        doc.set_page_decorator(SimplePageDecorator::new());

        // Add title and basic information
        doc.push(Paragraph::default().styled_string("CycloneDX VEX Document", self.title_style.clone()));
        doc.push(genpdf::elements::Break::new(1.0));

        // Add metadata if available
        if let Some(metadata) = &vex.metadata {
            
            if let Some(doc_metadata) = &metadata.document {
                if let Some(title) = &doc_metadata.title {
                    doc.push(Paragraph::default().styled_string(title, self.title_style.clone()));
                    doc.push(genpdf::elements::Break::new(1.0));
                }                
            }


            doc.push(Paragraph::default().styled_string("Document Information", self.header_style.clone()));

            // Add timestamp if available
            if let Some(timestamp) = &metadata.timestamp {
                doc.push(Paragraph::default().styled_string(
                    format!("Date: {}", timestamp),
                    self.normal_style.clone(),
                ));
            }

            // Add tools information if available
            if let Some(tools) = &metadata.tools {
                doc.push(Paragraph::default().styled_string(
                    "Tools:",
                    self.normal_style.clone(),
                ));

                for tool in tools {
                    if let Some(tool_name) = &tool.name {
                        doc.push(Paragraph::default().styled_string(
                            format!("  - {}", tool_name),
                            self.indent_style.clone(),
                        ));                        
                    }
                }
            }

            doc.push(genpdf::elements::Break::new(1.0));
        }

        // Add basic BOM information
        doc.push(Paragraph::default().styled_string(
            format!("BOM Format: {}", vex.bom_format),
            self.normal_style.clone(),
        ));
        doc.push(Paragraph::default().styled_string(
            format!("Specification Version: {}", vex.spec_version),
            self.normal_style.clone(),
        ));
        doc.push(Paragraph::default().styled_string(
            format!("Version: {}", vex.version),
            self.normal_style.clone(),
        ));

        if let Some(serial) = &vex.serial_number {
            doc.push(Paragraph::default().styled_string(
                format!("Serial Number: {}", serial),
                self.normal_style.clone(),
            ));
        }

        doc.push(genpdf::elements::Break::new(1.0));

        // Add Vulnerabilities section if available
        if let Some(vulnerabilities) = &vex.vulnerabilities {
            doc.push(Paragraph::default().styled_string("Vulnerabilities", self.header_style.clone()));
            doc.push(genpdf::elements::Break::new(0.5));

            // Add each vulnerability
            for vuln in vulnerabilities {
                doc.push(Paragraph::default().styled_string(
                    format!("ID: {}", vuln.id),
                    self.normal_style.clone(),
                ));

                if let Some(desc) = &vuln.description {
                    doc.push(
                        Paragraph::default()
                            .styled_string(format!("Description: {}", desc), self.indent_style.clone()),
                    );
                }

                if let Some(ratings) = &vuln.ratings {
                    for rating in ratings {
                        doc.push(Paragraph::default().styled_string(
                            format!("Severity: {}", rating.severity),
                            self.indent_style.clone(),
                        ));
                    }
                }

                doc.push(genpdf::elements::Break::new(0.5));
            }
        }

        // Add Components section if available
        if let Some(components) = &vex.components {
            doc.push(Paragraph::default().styled_string("Components", self.header_style.clone()));
            doc.push(genpdf::elements::Break::new(0.5));

            for component in components {
                doc.push(Paragraph::default().styled_string(
                    format!("Name: {}", component.name),
                    self.normal_style.clone(),
                ));

                if let Some(version) = &component.version {
                    doc.push(Paragraph::default().styled_string(
                        format!("Version: {}", version),
                        self.indent_style.clone(),
                    ));
                }

                doc.push(genpdf::elements::Break::new(0.5));
            }
        }

        // Render the document
        doc.render_to_file(output_path)
            .expect("failed to write file");

        Ok(())
    }
}
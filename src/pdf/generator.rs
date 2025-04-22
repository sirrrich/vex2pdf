//! PDF generation functionality for VEX reports.
//!
//! This module handles the conversion from CycloneDX VEX data structures to
//! formatted PDF documents using the genpdf library.
//!
//! The generator supports various VEX elements including vulnerabilities,
//! components, and document metadata.
//! 


use crate::model::cyclonedx::root::cyclone_vex::CycloneDxVex;
use genpdf::elements::Paragraph;
use genpdf::style::{Color, Style};
use genpdf::{Alignment, Document, Element};
use std::path::Path;
use std::io;

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


    /// Generates a PDF report from a CycloneDX VEX document.
    ///
    /// # Arguments
    ///
    /// * `vex` - The CycloneDX VEX document to convert
    /// * `output_path` - Path where the PDF report will be saved
    ///
    /// # Returns
    ///
    /// Result indicating success or an error with details
    pub fn generate_pdf<P: AsRef<Path>>(
        &self,
        vex: &CycloneDxVex,
        output_path: P,
    ) -> Result<(), io::Error> {
        // Set up the document with default fonts
        let mut fonts_dir = std::path::Path::new("/usr/share/fonts/liberation-sfonts");
        if !fonts_dir.exists() {
            fonts_dir = std::path::Path::new("./fonts/liberation-fonts");
        }
        let font_family = genpdf::fonts::from_files(&fonts_dir, "LiberationSans", None)
            .expect("Failed to load font family if you are on windows make sure a fonts directory is on the same level as the application and put Liberation fonts there");
        let document_title  = "Vulnerability Report Document";
        let pdf_title = "VEX Vulnerability Report";
        let mut doc = Document::new(font_family);
        
        doc.set_title(pdf_title);
        let mut decorator = genpdf::SimplePageDecorator::new();
        decorator.set_margins(10);
        decorator.set_header(|page| {
            let mut layout = genpdf::elements::LinearLayout::vertical();
            if page > 1 {
                layout.push(
                    Paragraph::new("Vulnerability report".to_string())
                        .aligned(Alignment::Left),
                );
                
                layout.push(
                    Paragraph::new(format!("Page {}", page)).aligned(Alignment::Center),
                );
                layout.push(genpdf::elements::Break::new(2));
            }
            layout.styled(Style::new()
                .with_font_size(10)
                .with_color(Color::Rgb(0, 0, 80)))
        });
        
        doc.set_page_decorator(decorator);

        // Add title and basic information
        doc.push(Paragraph::default().styled_string(document_title, self.title_style.clone()));
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
            doc.push(genpdf::elements::Break::new(1));

            // Add timestamp if available
            if let Some(timestamp) = &metadata.timestamp {
                doc.push(Paragraph::default().styled_string(
                    format!("Date: {}", timestamp),
                    self.normal_style.clone(),
                ));
            }
            
            doc.push(genpdf::elements::Break::new(1));

            // Add tools information if available
            if let Some(tools) = &metadata.tools {
                doc.push(Paragraph::default().styled_string(
                    "Tools:",
                    self.normal_style.clone(),
                ));
                
                let mut ul_tools = genpdf::elements::UnorderedList::new();
                for tool in tools {
                    if let Some(tool_name) = &tool.name {
                        ul_tools.push(Paragraph::default().styled_string(
                            tool_name,
                            self.indent_style.clone(),
                        ));                        
                    }
                }
                doc.push(ul_tools);
                doc.push(genpdf::elements::Break::new(1));

            }
            
            if let Some(component) = &metadata.component {
                doc.push(Paragraph::default()
                    .styled_string("Component name : ",self.normal_style.clone())
                    .styled_string(&component.name,self.indent_style.clone()));                
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

        doc.push(genpdf::elements::Break::new(2.0));

        // Add Vulnerabilities section if available
        if let Some(vulnerabilities) = &vex.vulnerabilities {
            doc.push(Paragraph::default().styled_string("Vulnerabilities", self.header_style.clone()));
            doc.push(genpdf::elements::Break::new(1.0));
            
            let mut ordered_list = genpdf::elements::OrderedList::new();


            // Add each vulnerability
            for vuln in vulnerabilities {
                let mut vuln_layout = genpdf::elements::LinearLayout::vertical()
                    .element(Paragraph::default().styled_string("ID: ",self.normal_style.clone())
                                                 .styled_string(format!("{}", vuln.id),self.normal_style.clone()))
                    .element(genpdf::elements::Break::new(0.5));

                if let Some(desc) = &vuln.description {
                    vuln_layout.push(
                        Paragraph::default()
                            .styled_string( "Description: ", self.indent_style.clone().bold())
                            .styled_string(format!("{}", desc),self.indent_style));
                    vuln_layout.push(genpdf::elements::Break::new(0.5));
                }
                
                let mut ratings_list = genpdf::elements::UnorderedList::new();

                if let Some(ratings) = &vuln.ratings {
                    for rating in ratings {
                        ratings_list.push( Paragraph::default().styled_string(format!("Severity: {} ({})", rating.severity,rating.method),
                                             self.indent_style.clone()));
                    }
                }
                vuln_layout.push(ratings_list);
                vuln_layout.push(genpdf::elements::Break::new(1));
                ordered_list.push(vuln_layout);
            }
            
            // list_layout.push(ordered_list);
            doc.push(ordered_list);
            doc.push(genpdf::elements::Break::new(0.5));
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
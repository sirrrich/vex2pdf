//! PDF generation functionality for CycloneDX (VEX) reports.
//!
//! This module handles the conversion from CycloneDX (VEX) data structures to
//! formatted PDF documents using the genpdf library.
//!
//! The generator supports various VEX elements, including vulnerabilities,
//! components, and document metadata.
//!

use crate::lib_utils::env_vars::EnvVarNames;
use crate::pdf::font_config::FontsDir;
use cyclonedx_bom::models::tool::Tools;
use cyclonedx_bom::prelude::Bom;
use genpdf::elements::Paragraph;
use genpdf::style::{Color, Style};
use genpdf::{Alignment, Document, Element};
use std::io;
use std::path::Path;

pub struct PdfGenerator<'a> {
    title_style: Style,
    header_style: Style,
    normal_style: Style,
    indent_style: Style,
    /// This is the title of the report; which is the first heading
    /// in the first page if no value is given a default title is used.
    report_title: Option<&'a str>,
    /// Sets the PDF document's metadata title that appears in PDF reader applications.
    /// This is distinct from the filename on disk (which typically matches the original file with a .pdf extension).
    /// If not specified, a default title will be used.
    pdf_meta_name: Option<&'a str>,
}

impl Default for PdfGenerator<'_> {
    /// Creates a new PDF generator with default report and PDF titles.
    ///
    /// This implementation of the `Default` trait provides a convenient way to create a
    /// `PdfGenerator` with standard titles suitable for vulnerability reports.
    ///
    /// # Returns
    ///
    /// A new `PdfGenerator` instance with:
    /// - Report title: "Vulnerability Report Document"
    /// - PDF title: "VEX Vulnerability Report"
    /// - Default styling for all text elements
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vex2pdf::pdf::generator::PdfGenerator;
    /// use std::default::Default;
    ///
    /// // Create a generator with default settings
    /// let generator = PdfGenerator::default();
    ///
    /// // Alternatively
    /// let generator: PdfGenerator = Default::default();
    /// ```
    fn default() -> Self {
        Self::new(
            Some(Self::get_default_report_title()),
            Some(Self::get_default_pdf_meta_name()),
        )
    }
}

impl<'a> PdfGenerator<'a> {
    /// Creates a new PDF generator with custom report and PDF titles.
    ///
    /// # Arguments
    ///
    /// * `report_title` - The title displayed as the main heading on the first page of the report
    /// * `pdf_title` - The title displayed in the PDF reader window/tab when the document is opened
    ///
    /// # Returns
    ///
    /// A new `PdfGenerator` instance with default styles and the specified titles
    ///
    /// # Examples
    ///
    /// ```rust
    /// use vex2pdf::pdf::generator::PdfGenerator;
    ///
    /// // Create a generator with custom titles
    /// let generator = PdfGenerator::new(Some("Security Analysis Results"), Some("Product Security Report"));
    /// ```
    pub fn new(report_title: Option<&'a str>, pdf_meta_name: Option<&'a str>) -> Self {
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
            report_title,
            pdf_meta_name,
        }
    }

    /// Gets the default title for the pdf metadata
    fn get_default_pdf_meta_name() -> &'static str {
        "VEX Vulnerability Report"
    }

    /// Gets the default title of the report which shows on the first page
    fn get_default_report_title() -> &'static str {
        "Vulnerability Report Document"
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
    pub fn generate_pdf<P: AsRef<Path>>(&self, vex: &Bom, output_path: P) -> Result<(), io::Error> {
        // Set up the document with default fonts

        let document_title = self
            .report_title
            .unwrap_or(Self::get_default_report_title());
        let pdf_title = self
            .pdf_meta_name
            .unwrap_or(Self::get_default_pdf_meta_name());

        let mut doc = Document::new(FontsDir::build().font_family);

        doc.set_title(pdf_title);
        let mut decorator = genpdf::SimplePageDecorator::new();
        decorator.set_margins(10);
        let header_title = document_title.to_string();
        decorator.set_header(move |page| {
            let mut layout = genpdf::elements::LinearLayout::vertical();
            if page > 1 {
                layout.push(
                    Paragraph::new(&header_title).aligned(Alignment::Left),
                );

                layout.push(Paragraph::new(format!("Page {}", page)).aligned(Alignment::Center));
                layout.push(genpdf::elements::Break::new(2));
            }
            layout.styled(
                Style::new()
                    .with_font_size(10)
                    .with_color(Color::Rgb(0, 0, 80)),
            )
        });

        doc.set_page_decorator(decorator);

        // Add title and basic information
        doc.push(Paragraph::default().styled_string(document_title, self.title_style));
        doc.push(genpdf::elements::Break::new(1.0));

        // Add metadata if available
        if let Some(metadata) = &vex.metadata {
            doc.push(Paragraph::default().styled_string("Document Information", self.header_style));
            doc.push(genpdf::elements::Break::new(1));

            // Add timestamp if available
            if let Some(timestamp) = &metadata.timestamp {
                doc.push(
                    Paragraph::default()
                        .styled_string(format!("Date: {}", timestamp), self.normal_style),
                );
            }

            doc.push(genpdf::elements::Break::new(1));

            // Add tools information if available
            if let Some(tools) = &metadata.tools {
                doc.push(Paragraph::default().styled_string("Tools:", self.normal_style));

                let mut ul_tools = genpdf::elements::UnorderedList::new();

                match tools {
                    Tools::List(tools_list) => {
                        for tool in tools_list {
                            if let Some(tool_name) = &tool.name {
                                ul_tools.push(
                                    Paragraph::default()
                                        .styled_string(tool_name.to_string(), self.indent_style),
                                );
                            }
                        }
                    }
                    Tools::Object {
                        services: services_obj,
                        components: components_obj,
                    } => {
                        // Handle components used as tools
                        if let Some(components) = &components_obj {
                            for component in &components.0 {
                                let component_name = &component.name;
                                let display_name = if let Some(version) = &component.version {
                                    format!("{} (v{})", component_name, version)
                                } else {
                                    component_name.clone().to_string()
                                };

                                ul_tools.push(
                                    Paragraph::default()
                                        .styled_string(&display_name, self.indent_style),
                                );
                            }
                        }

                        // Handle services used as tools
                        if let Some(services) = &services_obj {
                            for service in &services.0 {
                                let service_name = &service.name;
                                let display_name = if let Some(version) = &service.version {
                                    format!("{} (v{})", service_name, version)
                                } else {
                                    service_name.clone().to_string()
                                };

                                ul_tools.push(
                                    Paragraph::default()
                                        .styled_string(&display_name, self.indent_style),
                                );
                            }
                        }
                    }
                }

                doc.push(ul_tools);
                doc.push(genpdf::elements::Break::new(1));
            }

            if let Some(component) = &metadata.component {
                doc.push(
                    Paragraph::default()
                        .styled_string("Component name : ", self.normal_style)
                        .styled_string(component.name.to_string(), self.indent_style),
                );
            }

            doc.push(genpdf::elements::Break::new(1.0));
        }

        // Add basic BOM information
        doc.push(Paragraph::default().styled_string("BOM Format: CycloneDX", self.normal_style));
        doc.push(Paragraph::default().styled_string(
            format!("Specification Version: {}", vex.spec_version),
            self.normal_style,
        ));
        doc.push(
            Paragraph::default()
                .styled_string(format!("Version: {}", vex.version), self.normal_style),
        );

        if let Some(serial) = &vex.serial_number {
            doc.push(
                Paragraph::default()
                    .styled_string(format!("Serial Number: {}", serial), self.normal_style),
            );
        }

        doc.push(genpdf::elements::Break::new(2.0));

        // Default to showing the message (true by default)
        let mut show_novulns_msg = true;

        // Only check environment variable to see if we should disable the default behavior
        if let Ok(vuln_msg_env) = std::env::var(EnvVarNames::NoVulnsMsg.as_str()) {
            // Check for negative values that would disable the message
            if vuln_msg_env.eq_ignore_ascii_case("false")
                || vuln_msg_env.eq_ignore_ascii_case("off")
                || vuln_msg_env.eq_ignore_ascii_case("no")
                || vuln_msg_env.eq_ignore_ascii_case("0")
            {
                show_novulns_msg = false;
            }
            // For any other value, keep the default (true)
        }

        // First determine if vulnerabilities exist
        let mut vulns_available = false;
        if let Some(vulnerabilities) = &vex.vulnerabilities {
            vulns_available = !vulnerabilities.0.is_empty();
        }

        // Decide if we should show the vulnerabilities section at all
        let show_vulns_section = vulns_available || show_novulns_msg;

        if show_vulns_section {
            doc.push(Paragraph::default().styled_string("Vulnerabilities", self.header_style));
            doc.push(genpdf::elements::Break::new(1.0));
        }

        if let Some(vulnerabilities) = &vex.vulnerabilities {
            let mut ordered_list = genpdf::elements::OrderedList::new();

            // Add each vulnerability
            for vuln in &vulnerabilities.0 {
                let mut vuln_layout = genpdf::elements::LinearLayout::vertical();

                let id_paragraph = if let Some(vuln_id) = &vuln.id {
                    Paragraph::default()
                        .styled_string("ID: ", self.normal_style)
                        .styled_string(format!("{}", vuln_id), self.normal_style)
                } else {
                    Paragraph::default().styled_string("ID: N/A", self.normal_style)
                };

                vuln_layout.push(id_paragraph);

                let desc_paragraph = if let Some(desc) = &vuln.description {
                    Paragraph::default()
                        .styled_string("Description: ", self.indent_style.bold())
                        .styled_string(desc, self.indent_style)
                } else {
                    Paragraph::default()
                        .styled_string("Description: ", self.indent_style.bold())
                        .styled_string("N/A", self.indent_style)
                };

                vuln_layout.push(desc_paragraph);
                vuln_layout.push(genpdf::elements::Break::new(0.5));

                let mut ratings_list = genpdf::elements::UnorderedList::new();

                if let Some(ratings) = &vuln.vulnerability_ratings {
                    for rating in &ratings.0 {
                        let rating_method = if let Some(method) = &rating.score_method {
                            method.to_string()
                        } else {
                            "N/A".to_string()
                        };
                        if let Some(severity) = &rating.severity {
                            ratings_list.push(Paragraph::default().styled_string(
                                format!("Severity: {} ({})", severity, rating_method),
                                self.indent_style,
                            ));
                        }
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

        //Add message if vulns are not available
        if !vulns_available && show_novulns_msg {
            let vulns_style = Style::new()
                .bold()
                .with_font_size(16)
                .with_color(Color::Rgb(0, 100, 0));

            doc.push(
                Paragraph::new("No Vulnerabilities reported")
                    .aligned(Alignment::Center)
                    .padded(genpdf::Margins::vh(10, 0))
                    .framed()
                    .styled(vulns_style),
            );
            doc.push(genpdf::elements::Break::new(1.0));
        }

        // Add Components section if available
        if let Some(components) = &vex.components {
            doc.push(Paragraph::default().styled_string("Components", self.header_style));
            doc.push(genpdf::elements::Break::new(0.5));

            for component in &components.0 {
                doc.push(
                    Paragraph::default()
                        .styled_string(format!("Name: {}", component.name), self.normal_style),
                );

                if let Some(version) = &component.version {
                    doc.push(
                        Paragraph::default()
                            .styled_string(format!("Version: {}", version), self.indent_style),
                    );
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

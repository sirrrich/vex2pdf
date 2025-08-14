//! PDF generation functionality for CycloneDX (VEX) reports.
//!
//! This module handles the conversion from CycloneDX (VEX) data structures to
//! formatted PDF documents using the genpdf library.
//!
//! The generator supports various VEX elements, including vulnerabilities,
//! components, and document metadata.
//!

use crate::pdf::font_config::FontsDir;
use cyclonedx_bom::models::tool::Tools;
use cyclonedx_bom::prelude::Bom;
use cyclonedx_bom::models::vulnerability::AnalysisState;
use genpdf::elements::Paragraph;
use genpdf::style::{Color, Style};
use genpdf::{Alignment, Document, Element};
use std::io;
use std::path::Path;

fn fmt_analysis_state(state: &AnalysisState) -> &'static str {
    match state {
        AnalysisState::NotAffected   => "not_affected",
        AnalysisState::Exploitable   => "exploitable",
        AnalysisState::InTriage      => "in_triage",
        AnalysisState::Resolved      => "resolved",
        AnalysisState::FalsePositive => "false_positive",
        _ => "unknown",
    }
}

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
    /// Controls whether a `No Vulnerabilities` message is shown if there are no vulnerabilities
    show_novulns_msg: bool,
    /// Controls whether the components section is shown
    show_components: bool,
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
            true,
            true,
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
    /// * `show_novulns_msg` - Whether the `No Vulnerabilities reported` message is shown when no vulnerabilities are available
    /// * `show_components` - Whether the `components section` is shown
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
    /// // Create a generator with custom titles showing a No Vulnerabilities message if
    /// // the vulnerabilities array is empty and hiding the components section
    /// let generator = PdfGenerator::new(Some("Security Analysis Results"), Some("Product Security Report"),true,false);
    /// ```
    pub fn new(
        report_title: Option<&'a str>,
        pdf_meta_name: Option<&'a str>,
        show_novulns_msg: bool,
        show_components: bool,
    ) -> Self {
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
            show_novulns_msg,
            show_components,
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
                layout.push(Paragraph::new(&header_title).aligned(Alignment::Left));

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

        // First determine if vulnerabilities exist
        let mut vulns_available = false;
        if let Some(vulnerabilities) = &vex.vulnerabilities {
            vulns_available = !vulnerabilities.0.is_empty();
        }

        // Decide if we should show the vulnerabilities section at all
        let show_vulns_section = vulns_available || self.show_novulns_msg;

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

                // --- Analysis (CycloneDX 'analysis' / Rust: vulnerability_analysis) ---
                if let Some(analysis) = &vuln.vulnerability_analysis {
                    // Überschrift „Analysis:“ (optisch etwas eingerückt/fett)
                    vuln_layout.push(
                        Paragraph::default()
                            .styled_string("Analysis:", self.indent_style.bold())
                    );
                
                    // state
                    if let Some(state) = &analysis.state {
                        vuln_layout.push(
                            Paragraph::default()
                                .styled_string("  state: ", self.indent_style.bold())
                                .styled_string(fmt_analysis_state(state), self.indent_style)
                        );
                    }
                
                    // detail
                    if let Some(detail) = analysis.detail.as_deref() {
                        if !detail.is_empty() {
                            vuln_layout.push(
                                Paragraph::default()
                                    .styled_string("  detail: ", self.indent_style.bold())
                                    .styled_string(detail, self.indent_style)
                            );
                        }
                    }
                
                    // (Optional) justification / response, falls gewünscht:
                    // if let Some(j) = analysis.justification.as_ref() {
                    //     vuln_layout.push(
                    //         Paragraph::default()
                    //             .styled_string("  justification: ", self.indent_style.bold())
                    //             .styled_string(j.to_string(), self.indent_style)
                    //     );
                    // }
                    // if let Some(rsp) = analysis.response.as_ref() {
                    //     let joined = rsp.iter().map(ToString::to_string).collect::<Vec<_>>().join(", ");
                    //     if !joined.is_empty() {
                    //         vuln_layout.push(
                    //             Paragraph::default()
                    //                 .styled_string("  response: ", self.indent_style.bold())
                    //                 .styled_string(joined, self.indent_style)
                    //         );
                    //     }
                    // }
                
                    // kleiner Abstand zur nächsten Sektion
                    vuln_layout.push(genpdf::elements::Break::new(0.5));
                }
                
                let mut ratings_list = genpdf::elements::UnorderedList::new();

                if let Some(ratings) = &vuln.vulnerability_ratings {
                    for rating in &ratings.0 {
                        let rating_method = if let Some(method) = &rating.score_method {
                            method.to_string()
                        } else {
                            "N/A".to_string()
                        };

                        let source_str: Option<String> =
                            rating.vulnerability_source.as_ref().and_then(|source| {
                                source
                                    .name
                                    .as_ref()
                                    .map(|source_name| source_name.to_string())
                            });

                        if let Some(severity) = &rating.severity {
                            // add Severity ratings and sources

                            let mut severity_par = Paragraph::default()
                                .styled_string("Severity: ", self.indent_style.bold())
                                .styled_string(
                                    format!("{} ({}", severity, rating_method),
                                    self.indent_style,
                                );

                            if let Some(source_name) = source_str {
                                severity_par = severity_par
                                    .styled_string(" — Source: ", self.indent_style)
                                    .styled_string(source_name, self.indent_style);
                            }

                            severity_par = severity_par.styled_string(")", self.indent_style);
                            ratings_list.push(severity_par);
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
        if !vulns_available && self.show_novulns_msg {
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
        if self.show_components {
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
        }

        // Render the document
        doc.render_to_file(output_path)
            .expect("failed to write file");

        Ok(())
    }
}

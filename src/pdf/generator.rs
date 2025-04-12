use std::fmt::format;
use std::{error, io};
// src/pdf/generator.rs
use crate::model::vex::cyclone_vex::CycloneVex;
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
        vex: &CycloneVex,
        output_path: P,
    ) -> Result<(), io::Error> {
        // Set up the document with default fonts
        let font_family = genpdf::fonts::from_files("/usr/share/fonts/liberation-fonts", "LiberationSans", None)
            .expect("Failed to load font family");
        let mut doc = Document::new(font_family);

        doc.set_title("VEX Report");
        doc.set_page_decorator(SimplePageDecorator::new());

        // Add title

        if let Some(title) = &vex.document.title {
            doc.push(Paragraph::default().styled_string(title, self.title_style.clone()));
            doc.push(genpdf::elements::Break::new(1.0));
        }

        // Add metadata
        // Corrected code for document metadata section
        doc.push(Paragraph::default().styled_string(
            format!("Document ID: {}", vex.document.id),
            self.normal_style.clone(),
        ));
        doc.push(Paragraph::default().styled_string(
            format!("Version: {}", vex.document.version),
            self.normal_style.clone(),
        ));
        doc.push(Paragraph::default().styled_string(
            format!("Author: {}", vex.document.author),
            self.normal_style.clone(),
        ));
        doc.push(Paragraph::default().styled_string(
            format!("Date: {}", vex.document.timestamp),
            self.normal_style.clone(),
        ));
        doc.push(genpdf::elements::Break::new(1.0));

        // Add Vulnerability Statements section
        doc.push(Paragraph::default().styled_string("Vulnerability Statements", self.header_style));
        doc.push(genpdf::elements::Break::new(0.5));

        // Add each vulnerability statement
        for statement in &vex.vulnerability_statements {
            doc.push(Paragraph::default().styled_string(
                format!("Vulnerability ID: {}", statement.vulnerability_id),
                self.normal_style.clone(),
            ));

            doc.push(Paragraph::default().styled_string(
                format!(
                    "Product: {} ({})",
                    statement.product.name, statement.product.version
                ),
                self.indent_style.clone(),
            ));

            if let Some(desc) = &statement.description {
                doc.push(
                    Paragraph::default()
                        .styled_string(format!("Description: {}", desc), self.indent_style.clone()),
                );
            }

            if let Some(justification) = &statement.justification {
                doc.push(Paragraph::default().styled_string(
                    format!("Justification: {}", justification),
                    self.indent_style.clone(),
                ));
            }

            doc.push(genpdf::elements::Break::new(0.5));
        }

        // Render the document
        doc.render_to_file(output_path)
            .expect("failed to write file");

        Ok(())
    }
}

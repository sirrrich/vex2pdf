use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use vex_to_pdf;

use vex_to_pdf::model::vex::cyclone_vex::CycloneVex;
use vex_to_pdf::pdf::generator::PdfGenerator;

fn main() -> Result<(), Box<dyn Error>> {
    // Path to sample JSON file
    let json_path = "/home/salem/git_repos/IntellijProjects/vex-to-pdf/sample_vex.json";

    // Parse the JSON file
    let file = File::open(json_path)?;
    let reader = BufReader::new(file);
    let vex: CycloneVex = serde_json::from_reader(reader)?;

    println!(
        "Loaded VEX data: {} by {}",
        vex.document.title.as_deref().unwrap_or("Untitled"),
        vex.document.author
    );

    // Create PDF generator
    let pdf_generator = PdfGenerator::new();

    // Generate the PDF
    let output_path = "output_vex_report.pdf";
    match pdf_generator.generate_pdf(&vex, output_path) {
        Ok(_) => println!("Successfully generated PDF at: {}", output_path),
        Err(e) => println!("Failed to generate PDF: {}", e),
    }

    Ok(())
}

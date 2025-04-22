# CycloneDX (VEX) to PDF Converter

A command-line tool to convert CycloneDX VEX (Vulnerability Exploitability eXchange) documents in JSON format to PDF reports.

## ⚠️ Font Requirement
This application requires Liberation Sans fonts to properly render PDF documents. The fonts are NOT included in this repository due to licensing considerations.
**Before using the application, you must:**
1. Download the Liberation Sans TTF fonts from the [official repository](https://github.com/liberationfonts/liberation-fonts/releases)
2. Create a directory in the project root `fonts/liberation-fonts`
3. Place the following TTF files in that directory:
  - LiberationSans-Regular.ttf
  - LiberationSans-Bold.ttf
  - LiberationSans-Italic.ttf
  - LiberationSans-BoldItalic.ttf

**Alternative options:**
- On Linux systems, if the fonts are installed system-wide (typically in `/usr/share/fonts/liberation-sans`), the application will attempt to use them.
- Set the font path through an environment variable (not implemented yet).


> Note: The Liberation Fonts are licensed under the SIL Open Font License, not MIT. Please respect their license terms when using them.
>

## Overview
VEX-to-PDF is a Rust application that scans the current directory for CycloneDX VEX JSON files and converts them to human-readable PDF reports. It supports the CycloneDX VEX schema version 1.5 and handles various elements of the VEX documentation format including vulnerabilities, components, metadata, and more.
## Features
- Automatically scans directories for JSON files with VEX data
- Converts VEX documents to structured PDF reports
- Preserves all key VEX information including:
  - Document metadata and timestamps
  - Vulnerability details with severity ratings
  - Component information
  - Tools used to generate the VEX document

- Cross-platform support (Linux, Windows)

## Installation
### Prerequisites
- Rust and Cargo (latest stable version)
- Liberation Sans fonts (must be obtained separately as described above)

### From Source
Clone the repository, download and place Liberation fonts as described in the Font Requirement section, then build the application with . The binary will be available at target/release/vex2pdf. `cargo build --release`
### Windows Users
Windows users can either:
1. Build using Rust for Windows
2. Use a pre-built binary (when available)

## Usage
Run the application in a directory containing CycloneDX VEX JSON files:
``` 
./vex2pdf
```
The tool will:
1. Scan the current directory for JSON files
2. Attempt to parse each file as a CycloneDX VEX document
3. Generate a PDF report with the same name as the original file (with .pdf extension)
4. Display progress and results in the console

## Example
``` 
$ ./vex2pdf
Scanning for JSON files in: /home/user/vex-documents
Found 3 JSON files
Processing: sample_vex.json
Generating PDF: sample_vex.pdf
Successfully generated PDF: sample_vex.pdf
Processing: second-sample.json
Generating PDF: second-sample.pdf
Successfully generated PDF: UTF-8VEX.pdf
```
## Configuration
No configuration is currently required. The application will:
- Look for Liberation Sans fonts in `/usr/share/fonts/liberation-sfonts`
- Fall back to the included directory `./fonts/liberation-fonts`

## Documentation

> **Note**: Rust documentation is a work in progress. Please refer to the code comments for details on specific functions and data structures.
>

To generate documentation:
``` 
cargo doc --open
```
## CycloneDX VEX Format
This tool complies with the CycloneDX VEX schema version 1.5. For more information about the CycloneDX VEX format, see:
- [CycloneDX VEX Specification](https://cyclonedx.org/capabilities/vex/)
- [CycloneDX VEX Schema](https://cyclonedx.org/docs/1.5/json/)

## Security Considerations
- The application reads and processes files from the current directory
- No network connections are established
- Input validation is performed on all JSON files
- Font paths are validated before use

## License
This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.
The Liberation Fonts used by this application are licensed under the SIL Open Font License and must be obtained separately.
## Contributing
Contributions are welcome! Please feel free to submit a Pull Request.
1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Acknowledgments
- [CycloneDX](https://cyclonedx.org/) for the VEX specification
- [genpdf](https://crates.io/crates/genpdf) for PDF generation
- [Liberation Fonts](https://github.com/liberationfonts/liberation-fonts) for the PDF rendering fonts

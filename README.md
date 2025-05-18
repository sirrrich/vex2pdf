# CycloneDX (VEX) to PDF Converter

[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org/)
[![Documentation](https://docs.rs/vex2pdf/badge.svg)](https://docs.rs/vex2pdf)
[![Crates.io](https://img.shields.io/crates/v/vex2pdf.svg)](https://crates.io/crates/vex2pdf)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE.md)
[![CI](https://github.com/jurassicLizard/vex2pdf/actions/workflows/rust.yml/badge.svg)](https://github.com/jurassicLizard/vex2pdf/actions/workflows/rust.yml)
[![GitHub Release](https://img.shields.io/github/v/release/jurassicLizard/vex2pdf)](https://github.com/jurassicLizard/vex2pdf/releases/latest)

A command-line tool to convert CycloneDX VEX (Vulnerability Exploitability eXchange) documents in JSON or XML format to PDF reports.

<!-- TOC -->
* [CycloneDX (VEX) to PDF Converter](#cyclonedx-vex-to-pdf-converter)
  * [⚠️ Font Requirement](#-font-requirement)
  * [Overview](#overview)
  * [Features](#features)
  * [Installation](#installation)
    * [Prerequisites](#prerequisites)
    * [Via Cargo](#via-cargo)
    * [From Source](#from-source)
    * [Windows Users](#windows-users)
  * [Usage](#usage)
  * [Example](#example)
  * [Configuration](#configuration)
    * [Font Path Configuration](#font-path-configuration)
      * [Linux](#linux)
      * [Windows (PowerShell)](#windows-powershell)
    * [Environment Variables](#environment-variables)
      * [VEX2PDF_NOVULNS_MSG](#vex2pdf_novulns_msg)
  * [Documentation](#documentation)
  * [CycloneDX VEX Format](#cyclonedx-vex-format)
    * [Version 1.6 Compatibility Mode](#version-16-compatibility-mode)
  * [Security Considerations](#security-considerations)
  * [License](#license)
  * [Acknowledgments](#acknowledgments)
<!-- TOC -->

## ⚠️ Font Requirement

This application requires Liberation Sans fonts to properly render PDF documents.

The Liberation Sans fonts are NOT included in this repository due to licensing considerations. To set up the required fonts:

1. Download the Liberation Sans TTF fonts from the [official repository](https://github.com/liberationfonts/liberation-fonts/releases)
2. Create a directory in the project root: `fonts/liberation-fonts`
3. Place the following TTF files in that directory:
  - LiberationSans-Regular.ttf
  - LiberationSans-Bold.ttf
  - LiberationSans-Italic.ttf
  - LiberationSans-BoldItalic.ttf


**Alternative options:**
- On Linux systems, if the fonts are installed system-wide (typically in `/usr/share/fonts/liberation-sans`), the application will attempt to use them.
- Set the font path through the `VEX2PDF_FONTS_PATH` environment variable.

> Note: The Liberation Fonts are licensed under the SIL Open Font License, not MIT. Please respect their license terms when using them.
>

## Overview

VEX2PDF is a Rust application that scans the current directory for CycloneDX VEX files (JSON and XML) and converts them to human-readable PDF reports. It fully supports the CycloneDX VEX schema version 1.5 and provides compatibility for version 1.6 documents that only use 1.5 fields. Documents using 1.6-specific fields may not process correctly. The tool handles various elements of the VEX documentation format including vulnerabilities, components, metadata, and more.

## Features
- Automatically scans directories for JSON and XML files with VEX data
- Converts VEX documents to structured PDF reports
- Supports both JSON and XML CycloneDX formats
- Preserves all key VEX information including:
  - Document metadata and timestamps
  - Vulnerability details with severity ratings
  - Component information
  - Tools used to generate the VEX document
- Cross-platform support (Linux, Windows)

## Installation
### Prerequisites
- Rust and Cargo (latest stable version)
- Liberation Sans fonts (must be obtained separately as described in the [Font Requirement](#-font-requirement) section)

### Via Cargo
The easiest way to install VEX2PDF is directly from crates.io:

```bash
cargo install vex2pdf
```

After installation, the `vex2pdf` binary will be available in your Cargo bin directory.

> ⚠️ **Important**: You'll still need to set up the Liberation Sans fonts as described in the [Font Requirement](#-font-requirement) section. When installing via Cargo, fonts should be placed in one of the paths listed in the [Font Path Configuration](#font-path-configuration) section.



### From Source
Clone the repository, then build the application with `cargo build --release`. The binary will be available at target/release/vex2pdf.
### Windows Users
Windows users can either:
1. Install via Cargo as described above
2. Build using Rust for Windows from source
3. Use a pre-built binary (GitHub Releases Section)


## Usage

> ⚠️ Liberation fonts are needed (Check the [Font Path Configuration](#font-path-configuration) section above for more details)

Run the application in a directory containing CycloneDX VEX files (JSON or XML):

```shell 
./vex2pdf
```
The tool will:
1. Scan the current directory for JSON and XML files
2. Attempt to parse each file as a CycloneDX VEX document
3. Generate a PDF report with the same name as the original file (with .pdf extension)
4. Display progress and results in the console


## Example
``` 
$ ./vex2pdf
vex2pdf v0.6.0 - CycloneDX (VEX) to PDF Converter
Copyright (c) 2025 jurassicLizard - MIT License

Active font path: /usr/share/fonts/liberation-fonts

Scanning for JSON files in: ./documents
Found 2 JSON files
Processing: ./documents/example1.json
Generating PDF: ./documents/example1.pdf
Successfully generated PDF: ./documents/example1.pdf
Processing: ./documents/example2.json
Generating PDF: ./documents/example2.pdf
Successfully generated PDF: ./documents/example2.pdf

Scanning for XML files in: ./documents
Found 5 XML files
Processing: ./documents/example1.xml
Generating PDF: ./documents/example1.pdf
Successfully generated PDF: ./documents/example1.pdf
Processing: ./documents/example2.xml
Generating PDF: ./documents/example2.pdf
Successfully generated PDF: ./documents/example2.pdf
Processing: ./documents/example3.xml

NOTE: Downgrading CycloneDX BOM from spec version 1.6 to 1.5
Reason: Current implementation does not yet support spec version 1.6
Warning: This compatibility mode only works for BOMs that don't utilize 1.6-specific fields
         Processing will fail if 1.6-specific fields are encountered

Generating PDF: ./documents/example3.pdf
Successfully generated PDF: ./documents/example3.pdf
```
## Configuration

No configuration files are required. However the application has some customization options available

### Font Path Configuration

The application uses these locations for fonts in order of precedence:

1. Custom directory specified via `VEX2PDF_FONTS_PATH` environment variable (if set)
2. Project-local directory `./fonts/liberation-fonts` (if it exists)
3. User's local fonts directory `~/.local/share/fonts/liberation-fonts` (if it exists)
4. System-wide directory `/usr/share/fonts/liberation-fonts`

You can customize the font path by setting the `VEX2PDF_FONTS_PATH` environment variable:
The specified directory should contain the Liberation Sans font files directly (not in a subdirectory).

For example, if your fonts are in `/path/to/your/liberation-fonts/`, set:

#### Linux

```bash
export VEX2PDF_FONTS_PATH=/path/to/your/liberation-fonts
 ./vex2pdf
```

#### Windows (PowerShell)

```bash
$env:VEX2PDF_FONTS_PATH="C:\path\to\your\liberation-fonts" .\vex2pdf.exe
```




The specified directory should contain these font files:
- LiberationSans-Regular.ttf
- LiberationSans-Bold.ttf
- LiberationSans-Italic.ttf
- LiberationSans-BoldItalic.ttf


### Environment Variables

The following environment variables can be used to customize behavior:

| Variable             | Purpose                                                            | Default                           |
|----------------------|--------------------------------------------------------------------|-----------------------------------|
| VEX2PDF_FONTS_PATH   | Custom path to look for font files                                 | Check [Font Path Configuration](#font-path-configuration) |
| VEX2PDF_NOVULNS_MSG  | Controls the "No Vulnerabilities reported" message display         | true                              |


#### VEX2PDF_NOVULNS_MSG

This variable controls how the Vulnerabilities section appears when no vulnerabilities exist:
- When set to "true" or not set (default): A "Vulnerabilities" section will be shown with a "No Vulnerabilities reported" message
- When set to "false": The Vulnerabilities section will be completely omitted from the PDF

Example:
```bash
# To hide the Vulnerabilities section when no vulnerabilities exist this is mostly useful when a report for a bom is generated
VEX2PDF_NOVULNS_MSG=false vex2pdf
```


## Documentation


For full API documentation, please visit:
- [vex2pdf on docs.rs](https://docs.rs/vex2pdf)

> **Note**: Rust documentation is a work in progress. Please refer to the code comments for details on specific functions and data structures.

To generate documentation locally:
```bash
cargo doc --open
```


## CycloneDX VEX Format
This tool fully supports CycloneDX VEX schema version 1.5 and provides compatibility for version 1.6 documents that only use 1.5 fields. Documents using 1.6-specific fields may not process correctly. For more information about the CycloneDX VEX format, see:
- [CycloneDX VEX Specification](https://cyclonedx.org/capabilities/vex/)
- [CycloneDX VEX Schema](https://cyclonedx.org/docs/1.5/json/)

### Version 1.6 Compatibility Mode

This tool implements a special compatibility mode for CycloneDX 1.6 documents:

- When the tool encounters a document with `specVersion: "1.6"`, it will:
  1. Display a notification about downgrading to 1.5
  2. Automatically modify the document's spec version to "1.5"
  3. Attempt to process it using the 1.5 schema parser

This compatibility approach works well for documents that don't use 1.6-specific fields but allows the tool to process newer documents without requiring users to manually modify them.

**Limitations:**
- Documents that use 1.6-specific fields or structures may fail during processing
- No validation is performed for 1.6-specific features
- This is a temporary solution until full 1.6 support is implemented in the underlying cyclonedx-bom library

When processing 1.6 documents, you'll see console messages indicating the compatibility mode is active.

## Security Considerations
- The application reads and processes files from the current directory
- No network connections are established
- Input validation is performed on all JSON files
- Font paths are validated before use

## License
This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.
The Liberation Fonts used by this application are licensed under the SIL Open Font License and must be obtained separately.


## Acknowledgments
- [CycloneDX](https://cyclonedx.org/) for the VEX specification
- [cyclonedx-bom](https://crates.io/crates/cyclonedx-bom) for CycloneDX parsing
- [genpdf](https://crates.io/crates/genpdf) for PDF generation
- [serde_json](https://crates.io/crates/serde_json) for JSON processing
- [Liberation Fonts](https://github.com/liberationfonts/liberation-fonts) for the PDF rendering fonts

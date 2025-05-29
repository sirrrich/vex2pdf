use super::super::pdf::font_config::FontsDir;
use super::env_vars::EnvVarNames;
use super::input_file_type::InputFileType;
use crate::lib_utils::run_utils::print_copyright;
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

pub struct Config {
    pub working_dir: PathBuf,
    pub show_novulns_msg: bool, //FIXME still unused
    pub file_types_to_process: HashMap<InputFileType, bool>, //FIXME still unused
    pub show_oss_licenses: bool,
    pub report_title: Option<String>,
    pub pdf_meta_name: Option<String>,
}

impl Config {
    pub fn build() -> Result<Self, Box<dyn Error>> {
        let working_dir = std::env::current_dir()?;
        let show_novulns_msg = EnvVarNames::NoVulnsMsg.is_on_or_unset();
        let mut process_json = EnvVarNames::ProcessJson.is_on_or_unset();
        let process_xml = EnvVarNames::ProcessXml.is_on_or_unset();
        let show_oss_licenses = EnvVarNames::ShowOssLicenses.is_on();

        // print version info if requested
        if EnvVarNames::VersionInfo.is_on() {
            print_copyright();
        }

        if ! show_oss_licenses {
            // print init information only if show oss licenses is off
            FontsDir::print_fonts_info();
            // print default titles details only if show oss licenses is off
            EnvVarNames::print_report_titles_info();

        }

        // validate
        if !(process_json || process_xml) {
            println!("**** WARNING: we cannot have both json and xml deactivated. defaulting to json processing");
            process_json = true;
        }

        // init result map
        let mut file_types_to_process: HashMap<InputFileType, bool> = HashMap::new();
        file_types_to_process.insert(InputFileType::JSON, process_json);
        file_types_to_process.insert(InputFileType::XML, process_xml);

        let config = Config {
            working_dir,
            show_novulns_msg,
            file_types_to_process,
            show_oss_licenses,
            report_title: EnvVarNames::ReportTitle.get_value(),
            pdf_meta_name: EnvVarNames::PdfName.get_value(),
        };

        Ok(config)
    }
}

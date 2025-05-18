use super::super::pdf::font_config::FontsDir;
use super::env_vars::EnvVarNames;
use super::input_file_type::InputFileType;
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

pub struct Config {
    pub working_dir: PathBuf,
    pub fonts_dir: FontsDir,    //FIXME still unused
    pub show_novulns_msg: bool, //FIXME still unused
    pub file_types_to_process: HashMap<InputFileType, bool>, //FIXME still unused
}

impl Config {
    pub fn build() -> Result<Self, Box<dyn Error>> {
        let working_dir = std::env::current_dir()?;
        let fonts_dir = FontsDir::default();
        let show_novulns_msg = EnvVarNames::NoVulnsMsg.is_on_or_unset();
        let mut process_json = EnvVarNames::ProcessJson.is_on_or_unset();
        let process_xml = EnvVarNames::ProcessXml.is_on_or_unset();

        // print init information
        fonts_dir.print_fonts_info();

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
            fonts_dir,
            show_novulns_msg,
            file_types_to_process,
        };

        Ok(config)
    }
}

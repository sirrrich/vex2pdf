/// Environment variable names used in the application
pub enum EnvVarNames {
    /// Standard HOME environment variable
    Home,

    /// Custom path to look for font files
    /// When set, this is checked before standard system locations
    FontsPath,

    /// Controls whether to display "No Vulnerabilities reported" message
    /// When set to "false", the Vulnerabilities section will be omitted completely
    /// if no vulnerabilities exist.
    /// When set to "true" or not set (default), the "No Vulnerabilities reported"
    /// message will be displayed when no vulnerabilities are present.
    NoVulnsMsg,
}

impl EnvVarNames {
    pub fn as_str(&self) -> &'static str {
        match self {
            EnvVarNames::Home => "HOME",
            EnvVarNames::FontsPath => "VEX2PDF_FONTS_PATH",
            EnvVarNames::NoVulnsMsg => "VEX2PDF_NOVULNS_MSG",
        }
    }
}

/// Represents the supported input file types for VEX document processing.
///
/// This enum defines the file formats that can be processed by the vex2pdf application.
/// Currently, two formats are supported:
/// - XML: For VEX documents in XML format
/// - JSON: For VEX documents in JSON format
///
/// The enum implements methods to obtain string representations of the file type
/// for various use cases like file extension matching, logging, or error messages.
///
/// # Examples
///
/// ```rust
/// use vex2pdf::lib_utils::input_file_type::InputFileType;
///
/// // Get lowercase representation for file extension matching
/// assert_eq!(InputFileType::XML.as_str_lowercase(), "xml");
/// assert_eq!(InputFileType::JSON.as_str_lowercase(), "json");
///
/// // Get uppercase representation for display or logging
/// assert_eq!(InputFileType::XML.as_str_uppercase(), "XML");
/// assert_eq!(InputFileType::JSON.as_str_uppercase(), "JSON");
/// ```
#[derive(Eq, Hash, PartialEq)]
pub enum InputFileType {
    /// Represents an XML format VEX document
    XML,
    /// Represents a JSON format VEX document
    JSON,
}

impl InputFileType {
    /// Returns a lowercase string representation of the file type.
    ///
    /// This is useful for file extension matching or generating paths
    /// where lowercase is preferred.
    ///
    /// # Returns
    ///
    /// A lowercase static string representation of the file type:
    /// - `"xml"` for `InputFileType::XML`
    /// - `"json"` for `InputFileType::JSON`
    ///
    /// # Examples
    ///
    /// ```
    /// use vex2pdf::lib_utils::input_file_type::InputFileType;
    ///
    /// let file_extension = InputFileType::XML.as_str_lowercase();
    /// assert_eq!(file_extension, "xml");
    ///
    /// // Can be used for file filtering
    /// let path = "document.json";
    /// if path.ends_with(InputFileType::JSON.as_str_lowercase()) {
    ///     // Process JSON file
    /// }
    /// ```
    pub fn as_str_lowercase(&self) -> &'static str {
        match self {
            InputFileType::XML => "xml",
            InputFileType::JSON => "json",
        }
    }

    /// Returns an uppercase string representation of the file type.
    ///
    /// This is useful for display purposes, logging, or error messages
    /// where uppercase is typically used for file type names.
    ///
    /// # Returns
    ///
    /// An uppercase static string representation of the file type:
    /// - `"XML"` for `InputFileType::XML`
    /// - `"JSON"` for `InputFileType::JSON`
    ///
    /// # Examples
    ///
    /// ```
    /// use vex2pdf::lib_utils::input_file_type::InputFileType;
    ///
    /// let file_type = InputFileType::JSON;
    /// println!("Processing {} file", file_type.as_str_uppercase());
    /// // Outputs: "Processing JSON file"
    ///
    /// // Can be used in error messages
    /// let error_msg = format!("Failed to parse {} document", file_type.as_str_uppercase());
    /// assert_eq!(error_msg, "Failed to parse JSON document");
    /// ```
    pub fn as_str_uppercase(&self) -> &'static str {
        match self {
            InputFileType::XML => "XML",
            InputFileType::JSON => "JSON",
        }
    }
}

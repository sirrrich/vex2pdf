use std::env;
use std::path::PathBuf;

use super::super::lib_utils::env_vars::EnvVarNames;

//FIXME add enum for different linux distribution fonts
//FIXME make FontsDir be called by the main function and passed to the generator pdf
//FIXME create environment variables enum variants
/// `FontsDir` manages font directory paths across different locations in the system.
///
/// This struct tracks three possible locations for fonts:
/// - System-wide fonts (typically in `/usr/share/fonts/`)
/// - User-local fonts (typically in `~/.local/share/fonts/`)
/// - Project-specific fonts (in `./fonts/` relative to the current directory)
///
/// It provides methods to locate and select the appropriate font directory
/// based on availability and precedence rules.
pub struct FontsDir {
    /// Path to system-wide font directory
    pub system: PathBuf,

    /// Path to user's local font directory
    pub local: PathBuf,

    /// Path to project-specific font directory
    pub project: PathBuf,

    /// Path to custom-defined path via environment variable VEX2PDF_FONTS_PATH
    pub custom: Option<PathBuf>,
}

impl FontsDir {
    /// Creates a new `FontsDir` instance with paths configured for the specified font.
    ///
    /// # Arguments
    ///
    /// * `font_name` - The name of the font directory to look for (e.g., "liberation-fonts")
    ///
    /// # Returns
    ///
    /// A new `FontsDir` instance with configured paths for the specified font.
    ///
    /// # Examples
    ///
    /// ```
    /// use vex2pdf::pdf::font_config::FontsDir;
    /// let liberation_fonts = FontsDir::new("liberation-fonts", None);
    /// ```
    pub fn new(font_name: &str, fonts_path_str: Option<&str>) -> Self {
        // Calculate proper local path (expanding ~ to home directory)
        let local_path = if let Ok(home) = env::var("HOME") {
            PathBuf::from(format!("{}/.local/share/fonts/{}", home, font_name))
        } else {
            PathBuf::from(format!("~/.local/share/fonts/{}", font_name))
        };

        FontsDir {
            system: PathBuf::from(format!("{}{}", "/usr/share/fonts/", font_name)),
            local: local_path,
            project: PathBuf::from(format!("{}{}", "./fonts/", font_name)),
            custom: fonts_path_str.map(PathBuf::from),
        }
    }

    /// Returns the most appropriate font directory based on precedence rules.
    ///
    /// The precedence order is:
    /// 1. Project directory (if it exists)
    /// 2. User-local directory (if it exists)
    /// 3. System directory (fallback)
    ///
    /// # Returns
    ///
    /// A reference to the PathBuf of the selected font directory.
    ///
    /// # Examples
    ///
    /// ```
    /// use vex2pdf::pdf::font_config::FontsDir;
    /// let fonts = FontsDir::default();
    /// let font_path = fonts.get_active_font_dir();
    /// // Use font_path to load fonts
    /// ```
    pub fn get_active_font_dir(&self) -> &PathBuf {
        let active_path: &PathBuf;
        if let Some(custom_path) = &self.custom {
            if !custom_path.exists() {
                println!("WARNING: Non existing or Invalid Custom fonts path defined over environment variable ignoring")
                // we continue checking the standard paths after this
            } else {
                return custom_path;
            }
        }

        if self.project.exists() {
            active_path = &self.project;
        } else if self.local.exists() {
            active_path = &self.local;
        } else {
            active_path = &self.system;
        }

        active_path
    }

    /// Prints information about the active font directory to standard output.
    ///
    /// This method displays the path of the currently active font directory
    /// as determined by the `get_active_font_dir()` method. It converts the
    /// path to a string for display purposes, and will panic if the path
    /// cannot be converted to a valid UTF-8 string.
    ///
    /// # Panics
    ///
    /// This method will panic if the active font path contains invalid UTF-8 characters
    /// that cannot be converted to a Rust string.
    ///
    /// # Examples
    ///
    /// ```
    /// use vex2pdf::pdf::font_config::FontsDir;
    ///
    /// let fonts = FontsDir::default();
    /// fonts.print_fonts_info(); // Prints the active font path to stdout
    /// ```
    ///
    /// # Output Example
    ///
    /// ```text
    /// Active font path: /usr/share/fonts/liberation-fonts
    ///
    /// ```
    pub fn print_fonts_info(&self) {
        println!(
            "Active font path: {}",
            self.get_active_font_dir()
                .as_os_str()
                .to_str()
                .expect("unable to convert path to something usable")
        );
        println!();
    }
}

impl Default for FontsDir {
    /// Creates a default `FontsDir` instance configured for "liberation-fonts".
    /// Uses the VEX2PDF_FONTS_PATH environment Variable if present
    ///
    /// # Returns
    ///
    /// A new `FontsDir` instance for the Liberation fonts family.
    ///
    /// # Examples
    ///
    /// ```
    /// use vex2pdf::pdf::font_config::FontsDir;
    /// let default_fonts = FontsDir::default();
    /// ```
    fn default() -> Self {
        // Check for environment variable by default
        let custom_path = env::var(EnvVarNames::FontsPath.as_str()).ok();
        FontsDir::new("liberation-fonts", custom_path.as_deref())
    }
}

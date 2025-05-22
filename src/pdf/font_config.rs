use super::super::lib_utils::env_vars::EnvVarNames;
use genpdf::fonts::{FontData, FontFamily};
use std::env;
use std::error::Error;
use std::path::PathBuf;

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
            if custom_path.exists() {
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

    /// Determines whether a custom path override via VEX2PDF_FONTS_PATH has been set
    pub fn has_custom_path_override(&self) -> bool {
        self.custom.is_some()
    }

    /// Prints information about the active font source to standard output.
    ///
    /// This method first attempts to load embedded fonts. If successful, it will
    /// indicate that embedded fonts are being used. If embedded fonts cannot be loaded,
    /// it falls back to the external font directory as determined by the
    /// `get_active_font_dir()` method.
    ///
    /// The output will include either:
    /// - A message indicating embedded fonts are in use (with optional license information)
    /// - The path to the external font directory being used
    ///
    /// # Panics
    ///
    /// This method will panic if an error happens while loading embedded fonts AND
    /// the active font path contains invalid UTF-8 characters that cannot be converted to a Rust string.
    ///
    /// # Examples
    ///
    /// ```
    /// use vex2pdf::pdf::font_config::FontsDir;
    ///
    /// let fonts = FontsDir::default();
    /// fonts.print_fonts_info(); // Prints the active font source to stdout
    /// ```
    ///
    /// # Output Example
    ///
    /// When using embedded fonts:
    /// ```text
    /// Active font path: <embedded liberationSans fonts> -- the env variable VEX2PDF_SHOW_OSS_LICENSES=true shows Font license details
    ///
    /// ```
    ///
    /// When falling back to external fonts:
    /// ```text
    /// Active font path: /usr/share/fonts/liberation-fonts
    ///
    /// ```
    pub fn print_fonts_info(&self) {
        // THIS SECTION WILL BE REMOVED in 0.7.0 DUE TO DEPRECATION OF FONT PATH OVERRIDES
        let is_valid_custom = if let Some(custom) = &self.custom {
            println!("**** WARNING: Overriding embedded fonts ! this is deprecated and will be removed soon in favor for embedded fonts in the binary"); //FIXME remove this upon deprecation of FONT_PATH_OVERRIDES
            custom.exists()
        } else {
            false
        };
        let font_path = if is_valid_custom || self.load_embedded_font_family().is_err() {
            self.get_active_font_dir()
                .as_os_str()
                .to_str()
                .expect("unable to convert path to something usable")
        // END OF SECTION TO BE DELETED
        } else {
            "<embedded liberationSans fonts> -- the env variable VEX2PDF_SHOW_OSS_LICENSES=true shows Font license details"
        };

        println!("Active font path: {}", font_path);
        println!();
    }

    /// Loads a complete font family using embedded LiberationSans font data.
    ///
    /// This method creates a `FontFamily` containing all four standard LiberationSans font variants
    /// (regular, bold, italic, and bold-italic) that are embedded directly in the binary.
    /// This approach eliminates the need for external font files on the host system.
    ///
    /// # Returns
    ///
    /// A `Result` containing either:
    /// - `Ok(FontFamily<FontData>)` - A complete LiberationSans font family with all four variants
    /// - `Err(Box<dyn Error>)` - If any font variant fails to load or parse
    ///
    /// # Usage in PdfGenerator
    ///
    /// This method is intended to replace the traditional filesystem font loading:
    /// ```rust
    /// use vex2pdf::pdf::font_config::FontsDir;
    ///
    /// let fonts_dir = FontsDir::default();
    /// 
    /// let font_family = fonts_dir.load_embedded_font_family()
    ///     .unwrap_or_else(|_| {
    ///         // Fall back to loading fonts from filesystem
    ///         genpdf::fonts::from_files(fonts_dir.get_active_font_dir(), "LiberationSans", None)
    ///             .expect("Failed to load Liberation Sans fonts...")
    ///     });
    /// ```
    ///
    /// # Advantages
    ///
    /// - Works without access to the filesystem
    /// - Consistent font rendering across all platforms
    /// - No dependency on system-installed fonts
    /// - Eliminates the need for users to install fonts separately
    ///
    /// # Example Integration
    ///
    /// ```rust
    /// use vex2pdf::pdf::font_config::FontsDir;
    ///
    /// let fonts_dir = FontsDir::default();
    /// fonts_dir.print_fonts_info(); // Shows whether embedded or external fonts are used
    ///
    /// // Try embedded fonts first, fall back to filesystem if needed
    /// let font_family = fonts_dir.load_embedded_font_family()
    ///     .unwrap_or_else(|_| {
    ///         genpdf::fonts::from_files(fonts_dir.get_active_font_dir(), "LiberationSans", None)
    ///             .expect("Failed to load Liberation Sans fonts from any location.")
    ///     });
    ///
    /// // Use the font_family with genpdf
    /// let mut doc = genpdf::Document::new(font_family);
    /// // ... continue with document generation
    /// ```
    ///
    /// # License Information
    ///
    /// The embedded LiberationSans fonts are licensed under the SIL Open Font License.
    /// Set the environment variable `VEX2PDF_SHOW_OSS_LICENSES=true` to display full license details.
    pub fn load_embedded_font_family(&self) -> Result<FontFamily<FontData>, Box<dyn Error>> {
        let regular_font_data =
            include_bytes!("../../external/fonts/liberation-fonts/LiberationSans-Regular.ttf");
        let bold_font_data =
            include_bytes!("../../external/fonts/liberation-fonts/LiberationSans-Bold.ttf");
        let italic_font_data =
            include_bytes!("../../external/fonts/liberation-fonts/LiberationSans-Italic.ttf");
        let bold_italic_font_data =
            include_bytes!("../../external/fonts/liberation-fonts/LiberationSans-BoldItalic.ttf");

        // Create FontData objects from the byte arrays
        let regular = FontData::new(regular_font_data.to_vec(), None)?;
        let bold = FontData::new(bold_font_data.to_vec(), None)?;
        let italic = FontData::new(italic_font_data.to_vec(), None)?;
        let bold_italic = FontData::new(bold_italic_font_data.to_vec(), None)?;

        // Create a FontFamily with FontData
        Ok(FontFamily {
            regular,
            bold,
            italic,
            bold_italic,
        })
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

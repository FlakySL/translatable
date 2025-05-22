//! User configuration module.
//!
//! This module defines the structures and
//! helper functions for parsing and loading
//! user configuration files.

use std::env::var;
use std::fs::read_to_string;
use std::io::Error as IoError;
use std::sync::OnceLock;

use strum::EnumString;
use thiserror::Error;
use toml_edit::{DocumentMut, Table, TomlError};
use translatable_shared::misc::language::Language;

/// Configuration error enum.
///
/// Used for compile-time configuration
/// errors, such as errors while opening
/// files or parsing a file format.
///
/// The errors from this enum are directly
/// shown in rust-analyzer.
#[derive(Error, Debug)]
pub enum ConfigError {
    /// IO error derivations.
    ///
    /// Usually errors while interacting
    /// with the file system.
    ///
    /// [`Display`] forwards the inner error [`Display`]
    /// value with some prefix text.
    ///
    /// The enum implements [`From<std::io::Error>`] to
    /// allow conversion from [`std::io::Error`].
    ///
    /// **Parameters**
    /// * `0` - The IO error derivation.
    ///
    /// [`Display`]: std::fmt::Display
    /// [`From<std::io::Error>`]: std::io::Error
    #[error("IO error reading configuration: {0:#}")]
    Io(#[from] IoError),

    /// TOML deserialization error derivations.
    ///
    /// The configuration file contents could
    /// not be parsed as TOML.
    ///
    /// The error is formatted displaying
    /// the file name hardcoded as `./translatable.toml`
    /// and appended with the line and character.
    ///
    /// The enum implements [`From<toml::de::Error>`] to
    /// allow conversion from [`toml::de::Error`]
    ///
    /// **Parameters**
    /// * `0` - The TOML deserialization error derivation.
    ///
    /// [`From<toml::de::Error>`]: toml::de::Error
    #[error(
        "TOML parse error '{}'{}",
        .0.message(),
        .0.span()
            .map(|l| format!(" in ./translatable.toml:{}:{}", l.start, l.end))
            .unwrap_or_else(|| "".into())
    )]
    ParseToml(#[from] TomlError),

    /// Parse value error.
    ///
    /// There was an error while parsing
    /// a specific configuration entry,
    /// since these are mapped to enums in
    /// most cases.
    ///
    /// The error has a custom format
    /// displaying the key and value
    /// that should have been parsed.
    ///
    /// **Parameters**
    /// * `0` - The configuration key for which the entry
    /// could not be parsed.
    /// * `1` - The configuration value that couldn't be
    /// parsed.
    #[error("Couldn't parse configuration entry '{1}' for '{0}'")]
    InvalidValue(String, String),
}

/// Defines the search strategy for configuration files.
///
/// Represents the possible values of the parsed `seek_mode`
/// field, which determine the order in which file paths
/// are considered when opening configuration files.
#[derive(Default, Clone, Copy, EnumString)]
pub enum SeekMode {
    /// Alphabetical order (default)
    #[default]
    Alphabetical,

    /// Reverse alphabetical order
    Unalphabetical,
}

/// Strategy for resolving translation conflicts.
///
/// This enum defines how overlapping translations
/// are handled when multiple sources provide values
/// for the same key. The selected strategy determines
/// whether newer translations replace existing ones or
/// if the first encountered translation is preserved.
#[derive(Default, Clone, Copy, EnumString)]
pub enum TranslationOverlap {
    /// Last found translation overwrites previous ones (default)
    #[default]
    Overwrite,

    /// First found translation is preserved
    Ignore,
}

/// Main configuration structure for the translation system.
///
/// Holds all the core parameters used to control how translation files are
/// located, processed, and how conflicts are resolved between overlapping
/// translations.
pub struct MacroConfig {
    /// Path to the directory containing translation files.
    ///
    /// Specifies the base location where the system will search for
    /// translation files.
    ///
    /// # Example
    /// ```toml
    /// path = "./locales"
    /// ```
    path: String,

    /// File processing order strategy.
    ///
    /// Defines the order in which translation files are processed.
    /// Default: alphabetical order.
    seek_mode: SeekMode,

    /// Translation conflict resolution strategy.
    ///
    /// Determines the behavior when multiple files contain the same
    /// translation key.
    overlap: TranslationOverlap,

    /// Translation default language.
    ///
    /// This will be used as default language if the overriden language
    /// is not available, will automatically unwrap outputs as they will
    /// be pre-handled by this.
    fallback_language: Option<Language>,
}

impl MacroConfig {
    /// Get reference to the configured locales path.
    ///
    /// **Returns**
    /// The path to the directory where translation files are expected
    /// to be located.
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Get the current seek mode strategy.
    ///
    /// **Returns**
    /// The configured strategy used to determine the order in which
    /// translation files are processed.
    pub fn seek_mode(&self) -> SeekMode {
        self.seek_mode
    }

    /// Get the current overlap resolution strategy.
    ///
    /// **Returns**
    /// The configured strategy for resolving translation conflicts
    /// when multiple files define the same key.
    pub fn overlap(&self) -> TranslationOverlap {
        self.overlap
    }

    /// Get the fallback language.
    ///
    /// **Returns**
    /// Optionally the parsed format language from the configuration.
    pub fn fallback_language(&self) -> Option<Language> {
        self.fallback_language
    }
}

/// Global configuration cache.
///
/// Stores the initialized [`MacroConfig`] instance, which holds the
/// configuration for the translation system. The [`OnceLock`] ensures the
/// configuration is initialized only once and can be safely accessed across
/// multiple threads after that initialization.
static TRANSLATABLE_CONFIG: OnceLock<MacroConfig> = OnceLock::new();

/// Load the global translation configuration.
///
/// Initializes and returns a reference to the shared [`MacroConfig`] instance.
/// Configuration values are loaded in the following priority order:
/// environment variables override `translatable.toml`, and missing values fall
/// back to hardcoded defaults.
///
/// The configuration is cached after the first successful load, and reused on
/// subsequent calls.
///
/// **Returns**
/// A `Result` containing either:
/// * [`Ok(&MacroConfig)`] — The loaded configuration as a reference to the
///   cached macro configuration.
/// * [`Err(ConfigError)`] — An error because environment couldn't be read or
///   `translatable.toml` couldn't be read.
///
/// [`Ok(&MacroConfig)`]: MacroConfig
/// [`Err(ConfigError)`]: ConfigError
pub fn load_config() -> Result<&'static MacroConfig, ConfigError> {
    if let Some(config) = TRANSLATABLE_CONFIG.get() {
        return Ok(config);
    }

    let toml_content = read_to_string("./translatable.toml")
        .unwrap_or_default()
        .parse::<DocumentMut>()?;

    fn config_value(toml: &Table, key: &str) -> Option<String> {
        var(format!("TRANSLATABLE_{}", key.to_uppercase()))
            .ok()
            .or_else(|| {
                toml.get(key)
                    .and_then(|v| v.as_str())
                    .map(|v| v.to_string())
            })
    }

    macro_rules! parsed_config_value {
        ($key:literal) => {
            config_value(&toml_content, $key)
                .map(|s| {
                    s.parse()
                        .map_err(|_| ConfigError::InvalidValue($key.into(), s))
                })
                .transpose()
        };
    }

    let config = MacroConfig {
        path: config_value(&toml_content, "locales_path").unwrap_or("./translations".into()),

        overlap: parsed_config_value!("overlap")?.unwrap_or(TranslationOverlap::Ignore),

        seek_mode: parsed_config_value!("seek_mode")?.unwrap_or(SeekMode::Alphabetical),

        fallback_language: parsed_config_value!("fallback_language")?,
    };

    Ok(TRANSLATABLE_CONFIG.get_or_init(|| config))
}

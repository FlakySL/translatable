//! Configuration loading and handling for translatable content
//!
//! This module provides functionality to load and manage configuration
//! settings for localization/translation workflows from a TOML file.

use std::env::var;
use std::fs::read_to_string;
use std::io::Error as IoError;
use std::sync::OnceLock;

use strum::EnumString;
use thiserror::Error;
use toml::Table;
use toml::de::Error as TomlError;

/// Errors that can occur during configuration loading
#[derive(Error, Debug)]
pub enum ConfigError {
    /// IO error occurred while reading configuration file
    #[error("IO error reading configuration: {0:#}")]
    Io(#[from] IoError),

    /// TOML parsing error with location information
    #[error(
        "TOML parse error '{}'{}",
        .0.message(),
        .0.span().map(|l| format!(" in ./translatable.toml:{}:{}", l.start, l.end))
            .unwrap_or_else(|| "".into())
    )]
    ParseToml(#[from] TomlError),

    /// Invalid environment variable value for configuration options
    #[error("Couldn't parse configuration entry '{1}' for '{0}'")]
    InvalidValue(String, String),
}

/// File search order strategy
#[derive(Default, Clone, Copy, EnumString)]
pub enum SeekMode {
    /// Alphabetical order (default)
    #[default]
    Alphabetical,

    /// Reverse alphabetical order
    Unalphabetical,
}

/// Translation conflict resolution strategy
#[derive(Default, Clone, Copy, EnumString)]
pub enum TranslationOverlap {
    /// Last found translation overwrites previous ones (default)
    #[default]
    Overwrite,

    /// First found translation is preserved
    Ignore,
}

/// Main configuration structure for translation system
pub struct MacroConfig {
    /// Path to directory containing translation files
    ///
    /// # Example
    /// ```toml
    /// path = "./locales"
    /// ```
    path: String,

    /// File processing order strategy
    ///
    /// Default: alphabetical file processing
    seek_mode: SeekMode,

    /// Translation conflict resolution strategy
    ///
    /// Determines behavior when multiple files contain the same translation
    /// path
    overlap: TranslationOverlap,
}

impl MacroConfig {
    /// Get reference to configured locales path
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Get current seek mode strategy
    pub fn seek_mode(&self) -> SeekMode {
        self.seek_mode
    }

    /// Get current overlap resolution strategy
    pub fn overlap(&self) -> TranslationOverlap {
        self.overlap
    }
}

/// Global configuration cache
static TRANSLATABLE_CONFIG: OnceLock<MacroConfig> = OnceLock::new();

/// Load configuration from file or use defaults
///
/// # Implementation Notes
/// - Uses `OnceLock` for thread-safe singleton initialization
/// - Missing config file is not considered an error
/// - Config file must be named `translatable.toml` in root directory
/// - Environment variables take precedence over TOML configuration
/// - Supported environment variables:
///   - `TRANSLATABLE_LOCALES_PATH`: Overrides translation directory path
///   - `TRANSLATABLE_SEEK_MODE`: Sets file processing order ("alphabetical" or
///     "unalphabetical")
///   - `TRANSLATABLE_OVERLAP`: Sets conflict strategy ("overwrite" or "ignore")
///
/// # Panics
/// Will not panic but returns ConfigError for:
/// - Malformed TOML syntax
/// - Filesystem permission issues
/// - Invalid environment variable values
pub fn load_config() -> Result<&'static MacroConfig, ConfigError> {
    if let Some(config) = TRANSLATABLE_CONFIG.get() {
        return Ok(config);
    }

    // Load base configuration from TOML file
    let toml_content = read_to_string("./translatable.toml")
        .unwrap_or_default()
        .parse::<Table>()?;

    macro_rules! config_value {
        ($env_var:expr, $key:expr, $default:expr) => {
            var($env_var)
                .ok()
                .or_else(|| {
                    toml_content
                        .get($key)
                        .and_then(|v| v.as_str())
                        .map(|v| v.to_string())
                })
                .unwrap_or_else(|| $default.into())
        };

        (parse($env_var:expr, $key:expr, $default:expr)) => {{
            let value = var($env_var)
                .ok()
                .or_else(|| {
                    toml_content
                        .get($key)
                        .and_then(|v| v.as_str())
                        .map(|v| v.to_string())
                });

            if let Some(value) = value {
                value
                    .parse()
                    .map_err(|_| ConfigError::InvalidValue($key.into(), value.into()))
            } else {
                Ok($default)
            }
        }};
    }

    let config = MacroConfig {
        path: config_value!("TRANSLATABLE_LOCALES_PATH", "path", "./translations"),
        overlap: config_value!(parse(
            "TRANSLATABLE_OVERLAP",
            "overlap",
            TranslationOverlap::Ignore
        ))?,
        seek_mode: config_value!(parse(
            "TRANSLATABLE_SEEK_MODE",
            "seek_mode",
            SeekMode::Alphabetical
        ))?,
    };

    // Freeze configuration in global cache
    Ok(TRANSLATABLE_CONFIG.get_or_init(|| config))
}

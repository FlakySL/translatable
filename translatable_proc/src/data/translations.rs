use std::fs::{read_dir, read_to_string};
use std::io::Error as IoError;
use std::sync::OnceLock;

use thiserror::Error;
use toml::Table;
use toml::de::Error as TomlError;
use translatable_shared::translations::collection::TranslationNodeCollection;
use translatable_shared::translations::node::{TranslationNode, TranslationNodeError};

use super::config::{ConfigError, SeekMode, TranslationOverlap, load_config};

/// Contains error definitions for what may go wrong while
/// loading a translation.
#[derive(Error, Debug)]
pub enum TranslationDataError {
    /// Represents a generic IO error, if a file couldn't
    /// be opened, a path could not be found... This error
    /// will be inferred from an `std::io::Error`.
    #[error("There was a problem with an IO operation: {0:#}")]
    SystemIo(#[from] IoError),

    /// Used to convert any configuration loading error
    /// into a `TranslationDataError`, error messages are
    /// handled by the `ConfigError` itself.
    #[error("{0:#}")]
    LoadConfig(#[from] ConfigError),

    /// Specific conversion error for when a path can't be converted
    /// to a manipulable string because it contains invalid
    /// unicode characters.
    #[error("Couldn't open path, found invalid unicode characters")]
    InvalidUnicode,

    /// Represents a TOML deserialization error, this happens while
    /// loading files and converting their content to TOML.
    ///
    /// # Arguments
    /// * `.0` - The RAW toml::de::Error returned by the deserialization
    /// function.
    /// * `.1` - The path where the file was originally found.
    #[error(
        "TOML Deserialization error '{reason}' {span} in {1}",
        reason = _0.message(),
        span = _0
            .span()
            .map(|range| format!("on {}:{}", range.start, range.end))
            .unwrap_or_else(String::new)
    )]
    ParseToml(TomlError, String),

    #[error("{0:#}")]
    Node(#[from] TranslationNodeError),
}

/// Global thread-safe cache for loaded translations
static TRANSLATIONS: OnceLock<TranslationNodeCollection> = OnceLock::new();

/// Recursively walks directory to find all translation files
///
/// # Arguments
/// * `path` - Root directory to scan
///
/// # Returns
/// Vec of file paths or TranslationError
fn walk_dir(path: &str) -> Result<Vec<String>, TranslationDataError> {
    let mut stack = vec![path.to_string()];
    let mut result = Vec::new();

    // Use iterative approach to avoid recursion depth limits
    while let Some(current_path) = stack.pop() {
        let directory = read_dir(&current_path)?.collect::<Result<Vec<_>, _>>()?;

        for entry in directory {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path.to_str().ok_or(TranslationDataError::InvalidUnicode)?.to_string());
            } else {
                result.push(path.to_string_lossy().to_string());
            }
        }
    }

    Ok(result)
}

/// Loads and caches translations from configured directory
///
/// # Returns
/// Reference to cached translations or TranslationError
///
/// # Implementation Details
/// - Uses OnceLock for thread-safe initialization
/// - Applies sorting based on configuration
/// - Handles file parsing and validation
pub fn load_translations() -> Result<&'static TranslationNodeCollection, TranslationDataError> {
    if let Some(translations) = TRANSLATIONS.get() {
        return Ok(translations);
    }

    let config = load_config()?;
    let mut translation_paths = walk_dir(config.path())?;

    // Apply sorting based on configuration
    translation_paths.sort_by_key(|path| path.to_lowercase());
    if matches!(config.seek_mode(), SeekMode::Unalphabetical)
        || matches!(config.overlap(), TranslationOverlap::Overwrite)
    {
        translation_paths.reverse();
    }

    let translations = translation_paths
        .iter()
        .map(|path| {
            let table = read_to_string(path)?
                .parse::<Table>()
                .map_err(|err| TranslationDataError::ParseToml(err, path.clone()))?;

            Ok((path.clone(), TranslationNode::try_from(table)?))
        })
        .collect::<Result<TranslationNodeCollection, TranslationDataError>>()?;

    Ok(TRANSLATIONS.get_or_init(|| translations))
}

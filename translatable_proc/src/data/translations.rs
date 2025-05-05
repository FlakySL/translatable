//! Translation obtention module.
//!
//! This module is used to obtain
//! translations from their respective files.
//!
//! This module uses `crate::data::config` to
//! to load the translations and order them
//! based on the configuration provided
//! by the module.

use std::fs::{read_dir, read_to_string};
use std::io::Error as IoError;
use std::sync::OnceLock;

use thiserror::Error;
use toml_edit::{DocumentMut, TomlError};
use translatable_shared::translations::collection::TranslationNodeCollection;
use translatable_shared::translations::node::{TranslationNode, TranslationNodeError};

use super::config::{ConfigError, SeekMode, TranslationOverlap, load_config};

/// Translation retrieval error enum.
///
/// Represents errors that can occur during compile-time translation
/// retrieval. This includes I/O issues, configuration loading failures,
/// TOML deserialization errors, and translation node parsing errors.
///
/// The errors from this enum are directly surfaced in `rust-analyzer`
/// to assist with early detection and debugging.
#[derive(Error, Debug)]
pub enum TranslationDataError {
    /// I/O error derivation.
    ///
    /// Raised when an I/O operation fails during translation
    /// retrieval, typically caused by filesystem-level issues.
    ///
    /// [`Display`] will forward the inner [`std::io::Error`]
    /// representation prefixed with additional context.
    ///
    /// The enum implements [`From<std::io::Error>`] to allow
    /// automatic conversion from `IoError`.
    ///
    /// **Parameters**
    /// * `0` — The underlying I/O error.
    ///
    /// [`From<std::io::Error>`]: std::io::Error
    /// [`Display`]: std::fmt::Display
    #[error("IO Error: \"{0:#}\". Please check the specified path in your configuration file.")]
    Io(#[from] IoError),

    /// Configuration loading failure.
    ///
    /// Raised when the translation configuration cannot be loaded
    /// successfully, typically due to invalid values or missing
    /// configuration data.
    ///
    /// [`Display`] will forward the inner [`ConfigError`] message.
    ///
    /// The enum implements [`From<ConfigError>`] to allow automatic
    /// conversion from the underlying error.
    ///
    /// **Parameters**
    /// * `0` — The configuration error encountered.
    ///
    /// [`Display`]: std::fmt::Display
    #[error("{0:#}")]
    LoadConfig(#[from] ConfigError),

    /// Invalid Unicode path.
    ///
    /// Raised when a filesystem path cannot be processed due to
    /// invalid Unicode characters.
    ///
    /// This error signals that the translation system cannot proceed
    /// with a non-Unicode-compatible path.
    #[error("Couldn't open path, found invalid unicode characters")]
    InvalidUnicode,

    /// TOML deserialization failure.
    ///
    /// Raised when the contents of a translation file cannot be
    /// parsed as valid TOML data.
    ///
    /// The formatted error message includes the deserialization reason,
    /// the location within the file (if available), and the file path.
    ///
    /// **Parameters**
    /// * `0` — The [`toml::de::Error`] carrying the underlying deserialization
    ///   error.
    /// * `1` — The file path of the TOML file being parsed.
    #[error(
        "TOML Deserialization error '{reason}' {span} in {1}",
        reason = _0.message(),
        span = _0
            .span()
            .map(|range| format!("on {}:{}", range.start, range.end))
            .unwrap_or_else(String::new)
    )]
    ParseToml(TomlError, String),

    /// Translation node parsing failure.
    ///
    /// Raised when the translation system cannot correctly parse
    /// a translation node, typically due to invalid formatting
    /// or missing expected data.
    ///
    /// The enum implements [`From<TranslationNodeError>`] for
    /// seamless conversion.
    ///
    /// **Parameters**
    /// * `0` — The translation node error encountered.
    #[error("{0:#}")]
    Node(#[from] TranslationNodeError),
}

/// Global thread-safe cache for loaded translations.
///
/// Stores all parsed translations in memory after the first
/// successful load. Uses [`OnceLock`] to ensure that the translation
/// data is initialized only once in a thread-safe manner.
static TRANSLATIONS: OnceLock<TranslationNodeCollection> = OnceLock::new();

/// Recursively walks the target directory to discover all translation files.
///
/// Uses an iterative traversal strategy to avoid recursion depth limitations.
/// Paths are returned as [`String`] values, ready for processing.
///
/// Any filesystem errors, invalid paths, or read failures are reported
/// via `TranslationDataError`.
///
/// **Arguments**
/// * `path` — Root directory to scan for translation files.
///
/// **Returns**
/// A `Result` containing either:
/// * [`Ok(Vec<String>)`] — A flat list of discovered file paths.
/// * [`Err(TranslationDataError)`] — If traversal fails at any point.
///
/// [`Ok(Vec<String>)`]: std::vec::Vec<String>
/// [`Err(TranslationDataError)`]: TranslationDataError
fn walk_dir(path: &str) -> Result<Vec<String>, TranslationDataError> {
    let mut stack = vec![path.to_string()];
    let mut result = Vec::new();

    while let Some(current_path) = stack.pop() {
        let directory = read_dir(&current_path)?.collect::<Result<Vec<_>, _>>()?;

        for entry in directory {
            let path = entry.path();
            if path.is_dir() {
                stack.push(
                    path.to_str()
                        .ok_or(TranslationDataError::InvalidUnicode)?
                        .to_string(),
                );
            } else {
                result.push(
                    path.to_string_lossy()
                        .to_string(),
                );
            }
        }
    }

    Ok(result)
}

/// Loads and caches translations from the configured directory.
///
/// On the first invocation, this function:
/// - Reads the translation directory path from the loaded configuration.
/// - Recursively walks the directory to discover all translation files.
/// - Sorts the file list according to the configured `seek_mode`.
/// - Parses each file and validates its content.
///
/// Once successfully loaded, the parsed translations are stored
/// in a global [`OnceLock`]-backed cache and reused for the lifetime
/// of the process.
///
/// This function will return a reference to the cached translations
/// on every subsequent call.
///
/// **Returns**
/// A [`Result`] containing either:
/// * [`Ok(&TranslationNodeCollection)`] — The parsed and cached translations.
/// * [`Err(TranslationDataError)`] — An error because any of the translation
///   files couldn't be read.
///
/// [`Ok(&TranslationNodeCollection)`]: TranslationNodeCollection
/// [`Err(TranslationDataError)`]: TranslationDataError
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
                .parse::<DocumentMut>()
                .map_err(|err| TranslationDataError::ParseToml(err, path.clone()))?;

            Ok((path.clone(), TranslationNode::try_from(table.as_table())?))
        })
        .collect::<Result<TranslationNodeCollection, TranslationDataError>>()?;

    Ok(TRANSLATIONS.get_or_init(|| translations))
}

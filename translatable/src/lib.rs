//! # Translatable
//!
//! A robust internationalization solution for
//! Rust featuring compile-time validation,
//! ISO 639-1 compliance, and TOML-based
//! translation management.

#![warn(missing_docs)]

mod error;

/// Runtime error re-export.
///
/// This `use` statement renames
/// the run time error as a common
/// error by rust practice and exports
/// it.
#[rustfmt::skip]
pub use error::RuntimeError as Error;

/// Macro re-exports.
///
/// This `use` statement re-exports
/// all the macros on `translatable_proc`
/// which only work if included from
/// this module due to path generation.
#[rustfmt::skip]
pub use translatable_proc::translation;

#[rustfmt::skip]
pub use translatable_proc::TranslationContext;

/// Language enum re-export.
///
/// This `use` statement re-exports
/// from the hidden shared re-export
/// for user convenience on parsing.
#[rustfmt::skip]
pub use shared::misc::language::Language;

#[doc(hidden)]
#[rustfmt::skip]
pub use translatable_shared as shared;

//! # Translatable
//!
//! A robust internationalization solution for
//! Rust featuring compile-time validation,
//! ISO 639-1 compliance, and TOML-based
//! translation management.

mod error;

/// Runtime error re-export.
///
/// This `use` statement renames
/// the run time error as a common
/// error by rust practice and exports
/// it.
pub use error::RuntimeError as Error;
/// User-facing util re-exports.
///
/// This `use` statement re-exports
/// all the shared module items that
/// are useful for the end-user.
pub use shared::misc::language::Language;
/// Macro re-exports.
///
/// This `use` statement re-exports
/// all the macros on `translatable_proc`
/// which only work if included from
/// this module due to path generation.
pub use translatable_proc::translation;
#[doc(hidden)]
pub use translatable_shared as shared;

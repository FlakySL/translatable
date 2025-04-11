//! Internationalization library providing compile-time and runtime translation
//! facilities
//!
//! # Features
//! - TOML-based translation files
//! - ISO 639-1 language validation
//! - Configurable loading strategies
//! - Procedural macro for compile-time checking

use proc_macro::TokenStream;
use syn::parse_macro_input;

use crate::macro_generation::translation::translation_macro;
use crate::macro_input::translation::TranslationMacroArgs;

mod data;
mod macro_generation;
mod macro_input;

/// Procedural macro for compile-time translation validation
///
/// # Usage
/// ```ignore
/// translation!("en", static some::path)
/// ```
///
/// # Parameters
/// - Language code/literal
/// - Translation path (supports static analysis)
#[proc_macro]
pub fn translation(input: TokenStream) -> TokenStream {
    translation_macro(parse_macro_input!(input as TranslationMacroArgs).into()).into()
}

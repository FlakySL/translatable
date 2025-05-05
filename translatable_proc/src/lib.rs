//! Macro declarations for the `translatable` crate.
//!
//! This crate shouldn't be used by itself,
//! since the macros generate code with the context
//! of the `translatable` library.
//!
//! The `translatable` library re-exports the macros
//! declared in this crate.

#![warn(missing_docs)]

use macro_generation::context::context_macro;
use macro_generation::translation::translation_macro;
use macro_input::context::{ContextMacroArgs, ContextMacroStruct};
use macro_input::translation::TranslationMacroArgs;
use proc_macro::TokenStream;
use syn::parse_macro_input;

mod data;
mod macro_generation;
mod macro_input;

/// # Translation obtention macro.
///
/// This macro generates the way to obtain a translation
/// from the translation files in the directory defined
/// in the `translatable.toml` file.
///
/// **Parameters**
/// * `language` - A string literal for static inference or an instance of
/// `translatable::Language` for dynamic inference.
/// * `path` - A pat prefixed with `static` for static inference or a `Vec<impl
///   ToString>`
/// for dynamic inference.
/// * `replacements` - Arguments similar to python's `kwargs` for the
///   translation replacements.
///
/// This macro provides optimizations depending on the dynamism
/// of the parameters while calling the macro.
///
/// The optimizations are described the following way
/// - If path is static, no runtime lookup will be required
/// - If the path is dynamic, the file structure will be hardcoded.
///
/// - If the language is static, the validation will be reported by
///   `rust-analyzer`.
/// - If the language is dynamic the validation will be reported in runtime in
///   the `Err` branch.
///
/// - If both are dynamic a single [`String`] will be generated.
///
/// Independently of any other parameter, the `replacements` parameter
/// is always dynamic (context based).
///
/// You can shorten it's invocation if a similar identifier is on scope,
/// for example `x = x` can be shortened with `x`.
///
/// Replacement parameters are not validated, if a parameter exists it will be
/// replaced otherwise it won't.
///
/// **Returns**
/// A `Result` containing either:
/// * `Ok(String)` - If the invocation is successful.
/// * `Err(translatable::Error)` - If the invocation fails with a runtime error.
#[proc_macro]
pub fn translation(input: TokenStream) -> TokenStream {
    translation_macro(parse_macro_input!(input as TranslationMacroArgs).into()).into()
}

/// # Translation context macro
///
/// This macro converts a struct into a translation context.
///
/// By definition that struct shouldn't be used for anything else,
/// but nothing stops you from doing so.
///
/// This macro applies a rule to the struct. All fields must be
/// a `String` or `&str`.
///
/// You can configure some parameters as a punctuated [`MetaNameValue`],
/// these are
/// - `base_path`: A path that gets prepended to all fields.
/// - `fallback_language`: A language that must be available for all
/// paths and changes the return type of the `load_translations` method.
///
/// All the fields on the struct now point to paths in your translation
/// files, you can extend these paths applying the `#[path()]` attribute
/// with a [`TranslationPath`]. Otherwise the path will be appended as
/// the field identifier.
///
/// The field and struct visibility are kept as original.
///
/// This macro also generates a method called `load_translations` dynamically
/// that loads all translations and returns an instance of the struct,
/// optionally wrapped on a result depending on the `fallback_language`
/// parameter value.
///
/// [`MetaNameValue`]: syn::MetaNameValue
/// [`TranslationPath`]: macro_input::utils::translation_path::TranslationPath
#[proc_macro_attribute]
pub fn translation_context(attr: TokenStream, item: TokenStream) -> TokenStream {
    context_macro(
        parse_macro_input!(attr as ContextMacroArgs),
        parse_macro_input!(item as ContextMacroStruct),
    )
    .into()
}

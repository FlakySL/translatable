//! Macro declarations for the `translatable` crate.
//!
//! This crate shouldn't be used by itself,
//! since the macros generate code with the context
//! of the `translatable` library.
//!
//! The `translatable` library re-exports the macros
//! declared in this crate.

use macro_generation::context::context_macro;
use macro_input::context::{ContextMacroArgs, ContextMacroStruct};
use proc_macro::TokenStream;
use syn::parse_macro_input;

use crate::macro_generation::translation::translation_macro;
use crate::macro_input::translation::TranslationMacroArgs;

mod data;
mod macro_generation;
mod macro_input;

/// **translation obtention macro.**
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

#[proc_macro_attribute]
pub fn translation_context(attr: TokenStream, item: TokenStream) -> TokenStream {
    context_macro(
        parse_macro_input!(attr as ContextMacroArgs),
        parse_macro_input!(item as ContextMacroStruct)
    )
        .into()
}

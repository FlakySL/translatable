//! [`translation!()`] macro output module.
//!
//! This module contains the required for
//! the generation of the `translation!()` macro tokens
//! with intrinsics from `macro_input::translation.rs`.
//!
//! [`translation!()`]: crate::translation

use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use thiserror::Error;
use translatable_shared::handle_macro_result;
use translatable_shared::macros::collections::{map_to_tokens, map_transform_to_tokens};
use translatable_shared::misc::language::Language;

use crate::data::translations::load_translations;
use crate::macro_input::input_type::InputType;
use crate::macro_input::translation::TranslationMacroArgs;

/// Macro compile-time translation resolution error.
///
/// Represents errors that can occur while compiling the [`translation!()`].
/// This includes cases where a translation path cannot be found or
/// a language variant is unavailable at the specified path.
///
/// These errors are reported at compile-time by `rust-analyzer`
/// for immediate feedback while invoking the [`translation!()`] macro.
///
/// [`translation!()`]: crate::translation
#[derive(Error, Debug)]
enum MacroCompileError {
    /// The requested translation path could not be found.
    ///
    /// **Parameters**
    /// * `0` — The translation path, displayed in `::` notation.
    #[error("The path '{0}' could not be found")]
    PathNotFound(String),

    /// The requested language is not available for the provided translation
    /// path.
    ///
    /// **Parameters**
    /// * `0` — The requested `Language`.
    /// * `1` — The translation path where the language was expected.
    #[error("The language '{0:?}' ('{0:#}') is not available for the path '{1}'")]
    LanguageNotAvailable(Language, String),
}

/// `translation!()` macro output generation.
///
/// Expands into code that resolves a translation string based on the input
/// language and translation path, performing placeholder substitutions
/// if applicable.
///
/// If the language and path are fully static, the translation will be resolved
/// during macro expansion. Otherwise, the generated code will include runtime
/// resolution logic.
///
/// If the path or language is invalid at compile time, an appropriate
/// `MacroCompileError` will be reported.
///
/// **Arguments**
/// * `input` — Structured arguments defining the translation path, language,
/// and any placeholder replacements obtained from `macro_input::translation`.
///
/// **Returns**
/// Generated `TokenStream2` representing the resolved translation string or
/// runtime lookup logic.
pub fn translation_macro(input: TranslationMacroArgs) -> TokenStream2 {
    let translations = handle_macro_result!(load_translations());

    let template_replacements = map_transform_to_tokens(
        input.replacements(),
        |key, value| quote! { (stringify!(#key).to_string(), #value.to_string()) },
    );

    if let InputType::Static(language) = input.language() {
        if let InputType::Static(path) = input.path() {
            let static_path_display = path.join("::");

            let translation_object = translations
                .find_path(path)
                .ok_or_else(|| MacroCompileError::PathNotFound(static_path_display.clone()));

            let translation = handle_macro_result!(
                handle_macro_result!(translation_object)
                    .get(language)
                    .ok_or_else(|| {
                        MacroCompileError::LanguageNotAvailable(
                            language.clone(),
                            static_path_display.clone(),
                        )
                    })
            );

            return quote! {
                #translation
                    .replace_with(#template_replacements)
            };
        }
    }

    let language = match input.language() {
        InputType::Static(language) => language
            .clone()
            .to_token_stream(),
        InputType::Dynamic(language) => quote! {
            translatable::shared::misc::language::Language::from(#language)
        },
    };

    let translation_object = match input.path() {
        InputType::Static(path) => {
            let static_path_display = path.join("::");

            let translation_object = translations
                .find_path(path)
                .ok_or_else(|| MacroCompileError::PathNotFound(static_path_display.clone()));

            let translations_tokens = map_to_tokens(handle_macro_result!(translation_object));

            quote! {
                #[doc(hidden)]
                let path: Vec<_> = vec![#(#path.to_string()),*];

                #translations_tokens
            }
        },

        InputType::Dynamic(path) => {
            let translations_tokens = translations.to_token_stream();

            quote! {
                #[doc(hidden)]
                let path: Vec<_> = #path;

                #translations_tokens
                    .find_path(&path)
                    .ok_or_else(|| translatable::Error::PathNotFound(path.join("::")))?
            }
        },
    };

    quote! {
        (|| -> Result<String, translatable::Error> {
            std::result::Result::Ok({
                #[doc(hidden)]
                let language = #language;

                #translation_object
                    .get(&language)
                    .ok_or_else(|| translatable::Error::LanguageNotAvailable(language, path.join("::")))?
                    .replace_with(#template_replacements)
            })
        })()
    }
}

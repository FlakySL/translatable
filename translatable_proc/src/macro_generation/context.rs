//! [`TranslationContext`] derive macro output module.
//!
//! This module contains the required for
//! the generation of the [`TranslationContext`] derive macro tokens
//! with intrinsics from `macro_input::context`.
//!
//! [`TranslationContext`]: crate::translation_context

use core::panic;

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use thiserror::Error;
use translatable_shared::macros::collections::map_to_tokens;
use translatable_shared::{handle_macro_result, inline_quote};

use crate::data::translations::load_translations;
use crate::macro_input::context::ContextMacroInput;

/// Macro compile-time translation resolution error.
///
/// Represents errors that can occur while compiling the
/// [`TranslationContext`] derive macro. This includes cases where a translation
/// path cannot be found or fallback is not available for all the translations
/// in the context.
///
/// These errors are reported at compile-time by `rust-analyzer`
/// for immediate feedback while invoking the [`TranslationContext`]
/// macro.
///
/// [`TranslationContext`]: crate::translation_context
#[derive(Error, Debug)]
enum MacroCompileError {
    /// The requested translation path could not be found.
    ///
    /// **Parameters**
    /// * `0` — The translation path, displayed in `::` notation.
    #[error("A translation with the path '{0}' could not be found")]
    TranslationNotFound(String),

    /// A fallback is not available for a specified translation path.
    #[error("One of the translations doesn't have the fallback language available")]
    FallbackNotAvailable,

    /// One of the fields type is not a &str or String.
    #[error("Only String' and '&str' is allowed for translation contexts")]
    TypeNotAllowed,
}

/// [`TranslationContext`] derive macro output generation.
///
/// Expands into a struct that implements structured translation
/// loading.
///
/// If there is a fallback language configured, this is checked
/// with all the paths and then the `load_translations` generated
/// method will return the same structure instead of a Result.
///
/// **Arguments**
/// * `macro_input` - The parsed macro tokens themselves.
///
/// **Returns**
/// A TokenStream representing the implementation.
///
/// [`TranslationContext`]: crate::translation_context
pub fn context_macro(macro_input: ContextMacroInput) -> TokenStream2 {
    let translations = handle_macro_result!(out load_translations());

    let quoted_fields = macro_input
        .fields()
        .iter()
        .map(|field| {
            let path = field.path();
            translations
                .find_path(path.segments())
                .ok_or_else(|| MacroCompileError::TranslationNotFound(path.static_display()))
                .map(|translations| inline_quote! {
                    #{field.name()}: #{map_to_tokens(translations)}
                        .get(&language)
                })
        })
        .collect::<Result<Vec<_>, _>>();

    let quoted_fields = handle_macro_result!(out quoted_fields);
    let quoted_fields = quote! { #(#quoted_fields),* };

    inline_quote! {
        impl #{macro_input.ident()} {
            #[doc(hidden)]
            pub fn __translations(language: &::translatable::Language) -> Self {
                Self {
                    #quoted_fields
                }
            }
        }
    }
}

//! [`#\[translation_context\]`] macro output module.
//!
//! This module contains the required for
//! the generation of the [`#\[translation_context\]`] macro tokens
//! with intrinsics from `macro_input::context`.
//!
//! [`#\[translation_context\]`]: crate::translation_context

use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use thiserror::Error;
use translatable_shared::handle_macro_result;
use translatable_shared::macros::collections::map_to_tokens;

use crate::data::translations::load_translations;
use crate::macro_input::context::{ContextMacroArgs, ContextMacroStruct};

/// Macro compile-time translation resolution error.
///
/// Represents errors that can occur while compiling the [`#\[translation_context\]`]
/// macro. This includes cases where a translation path cannot be found or
/// fallback is not available for all the translations in the context.
///
/// These errors are reported at compile-time by `rust-analyzer`
/// for immediate feedback while invoking the [`#\[translation_context\]`] macro.
///
/// [`#\[translation_context\]`]: crate::translation_context
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

/// [`#\[translation_context\]`] macro output generation.
///
/// Expands into a struct that implements structured translation
/// loading.
///
/// If there is a fallback language configured, this is checked
/// with all the paths and then the `load_translations` generated
/// method will return the same structure instead of a Result.
///
/// **Arguments**
/// * `macro_args` - The parsed arguments for the macro invocation.
/// * `macro_input` - The parsed macro tokens themselves.
///
/// **Returns**
/// A TokenStream representing the implementation.
///
/// [`#\[translation_context\]`]: crate::translation_context
pub fn context_macro(
    macro_args: ContextMacroArgs,
    macro_input: ContextMacroStruct,
) -> TokenStream2 {
    let translations = handle_macro_result!(out load_translations());
    let base_path = macro_args.base_path();

    let struct_pub = macro_input.visibility();
    let struct_ident = macro_input.ident();

    let struct_fields = handle_macro_result!(out
        macro_input
            .fields()
            .iter()
            .map(|field| {
                let field_ty = field.ty().to_token_stream().to_string();
                if matches!(field_ty.as_str(), "String" | "&str") {
                    Ok(field)
                } else {
                    Err(MacroCompileError::TypeNotAllowed)
                }
            })
            .collect::<Result<Vec<_>, _>>()
    );

    let loadable_translations = handle_macro_result!(out
        macro_input
            .fields()
            .iter()
            .map(|field| {
                let path_segments = base_path
                    .merge(&field.path());

                let path_segments_display = path_segments
                    .join("::");

                let translation = translations
                    .find_path(&path_segments)
                    .ok_or(MacroCompileError::TranslationNotFound(path_segments.join("::")))?;

                let translation_tokens = map_to_tokens(translation);
                let ident = field.ident();

                let handler = if let Some(fallback_language) = macro_args.fallback_language() {
                    if let Some(translation) = translation.get(&fallback_language) {
                        quote! {
                            .unwrap_or(&#translation)
                        }
                    } else {
                        return Err(MacroCompileError::FallbackNotAvailable);
                    }
                } else {
                    quote! {
                        .ok_or_else(|| translatable::Error::LanguageNotAvailable(
                            language.clone(),
                            #path_segments_display.to_string()
                        ))?
                    }
                };

                Ok(quote! {
                    #ident: #translation_tokens
                        .get(&language)
                        #handler
                        .replace_with(&replacements)
                })
            })
            .collect::<Result<Vec<TokenStream2>, MacroCompileError>>()
    );

    let is_lang_some = macro_args
        .fallback_language()
        .is_some();

    let load_ret_ty = if is_lang_some {
        quote! { Self }
    } else {
        quote! { Result<Self, translatable::Error> }
    };

    let load_ret_stmnt = if is_lang_some {
        quote! {
            Self {
                #(#loadable_translations),*
            }
        }
    } else {
        quote! {
            Ok(Self {
                #(#loadable_translations),*
            })
        }
    };

    quote! {
        #struct_pub struct #struct_ident {
            #(#struct_fields),*
        }

        impl #struct_ident {
            #struct_pub fn load_translations<K: ToString, V: ToString>(
                language: translatable::Language,
                replacements: &std::collections::HashMap<K, V>
            ) -> #load_ret_ty {
                let replacements = replacements
                    .iter()
                    .map(|(key, value)| (key.to_string(), value.to_string()))
                    .collect::<std::collections::HashMap<String, String>>();

                #load_ret_stmnt
            }
        }
    }
}

//! [`translation!()`] macro output module.
//!
//! This module contains the required for
//! the generation of the [`translation!()`] macro tokens
//! with intrinsics from [`macro_input::translation`].
//!
//! The macro is separated by 4 functions, the main
//! function [`translation_macro`] which is exported
//! handles the context and conditionally calls the
//! other 3 functions based on the macro intrinsics.
//!
//! The language is really only used statically
//! if the path and language are both static, otherwise
//! static path is only validated but not used further
//! in compile time.
//!
//! [`translation!()`]: crate::translation
//! [`macro_input::translation`]: super::super::macro_input::translation

use std::collections::HashMap;

use syn::Ident;
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use thiserror::Error;
use translatable_shared::macros::collections::{map_to_tokens, map_transform_to_tokens};
use translatable_shared::macros::option::LiteralOption;
use translatable_shared::misc::language::Language;
use translatable_shared::misc::templating::FormatString;
use translatable_shared::translations::collection::TranslationNodeCollection;
use translatable_shared::translations::node::TranslationObject;
use translatable_shared::{handle_macro_result, inline_quote};

use crate::data::config::load_config;
use crate::data::translations::load_translations;
use crate::macro_input::translation::TranslationMacroArgs;
use crate::macro_input::utils::input_type::InputType;
use crate::macro_input::utils::translation_path::TranslationPath;

/// Macro compile-time translation resolution error.
///
/// Represents errors that can occur while compiling the [`translation!()`]
/// macro. This includes cases where a translation path cannot be found or
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

    /// The fallback language is not available for the provided translation
    /// path.
    ///
    /// **Parameters**
    /// * `0` - The translation path where the language was expected.
    #[error("The configured fallback language is not available for this '{0}'.")]
    FallbackNotAvailable(String),
}

/// Local macro generation context.
///
/// Macro generation context used for concern separation
/// on generating the [`translation!()`] macro generation separatedly
/// in multiple functions.
///
/// [`translation!()`]: crate::translation
struct GenerationContext<'i> {
    /// The translation fallback language
    /// whether it's available or not.
    fallback_language: Option<Language>,

    /// A reference to the translation nodes
    /// loaded from the file system.
    translations: &'i TranslationNodeCollection,

    /// The user provided template replacements.
    template_replacements: &'i HashMap<Ident, TokenStream2>,
}

/// Template replacement values to tokens.
///
/// Calls [`map_transform_to_tokens`] for the replacement values
/// in a specific maneer.
///
/// **Arguments**
/// * `replacements` - The replacement values.
///
/// **Returns**
/// The token stream with the converted [`HashMap`].
fn format_replacements(replacements: &HashMap<Ident, TokenStream2>) -> TokenStream2 {
    map_transform_to_tokens(
        replacements,
        |key, value| quote! { (stringify!(#key).to_string(), #value.to_string()) }
    )
}

/// [`TranslationObject`] static obtention helper.
///
/// Obtain a specific translation object in compile
/// time using macro intrinsics from a [`TranslationNodeCollection`]
/// and a [`TranslationPath`], converting the possible error
/// into the corresponding [`MacroCompileError`].
///
/// **Arguments**
/// * `translations` - The collection of files where to find the translation
///   object.
/// * `path` - The path to look for inside the collection.
///
/// **Returns**
/// A [`Result`] containing the translation object or a
/// [`MacroCompileError::PathNotFound`] error.
fn get_translation_object<'r>(
    translations: &'r TranslationNodeCollection,
    path: &TranslationPath,
) -> Result<&'r TranslationObject, MacroCompileError> {
    translations
        .find_path(path)
        .ok_or_else(|| MacroCompileError::PathNotFound(path.static_display()))
}

/// [`TranslationObject`] fallback helper.
///
/// Obtains the corresponding fallback translation
/// for a [`TranslationObject`], converting the possible
/// error to the corresponding [`MacroCompileError`].
///
/// **Arguments**
/// * `original_path` - The original path where the translation was found.
/// * `translation` - The translation object for where to find the fallback
///   translation.
/// * `fallback_language` - The fallback language to find the translation.
///
/// **Returns**
/// [`MacroCompileError::FallbackNotAvailable`] if there is a fallback
/// but is not available in the translation otherwise [`Ok`] whether there
/// was a fallback language specified or not.
fn get_fallback_translation<'r>(
    original_path: &TranslationPath,
    translation: &'r TranslationObject,
    fallback_language: Option<Language>,
) -> Result<Option<&'r FormatString>, MacroCompileError> {
    fallback_language
        .map(|lang| {
            translation
                .get(&lang)
                .ok_or_else(|| {
                    MacroCompileError::FallbackNotAvailable(original_path.static_display())
                })
        })
        .transpose()
}

/// Fully static arguments generation.
///
/// Concern separation for [`translation_macro`].
/// Generates the macro output taking all static arguments.
///
/// **Arguments**
/// * `ctx` - The macro generation context
/// * `language` - The static langauge argument
/// * `path` - The static path argument
///
/// **Returns**
/// Tokens to be directly returned for all static generation
#[inline(always)]
fn all_static(ctx: &GenerationContext, language: Language, path: &TranslationPath) -> TokenStream2 {
    let translation_object = handle_macro_result!(get_translation_object(ctx.translations, path));
    let fallback_translation = handle_macro_result!(
        get_fallback_translation(path, translation_object, ctx.fallback_language)
    );

    let translation = handle_macro_result!(
        translation_object
            .get(&language)
            .or(fallback_translation)
            .ok_or_else(|| MacroCompileError::LanguageNotAvailable(
                language,
                path.static_display()
            ))
    );

    inline_quote! {{
        #{translation}
            .replace_with(&#{format_replacements(ctx.template_replacements)})
    }}
}

/// Path static generation.
#[inline(always)]
fn path_static(
    ctx: &GenerationContext,
    language: &TokenStream2,
    path: &TranslationPath,
) -> TokenStream2 {
    let translation_object = handle_macro_result!(get_translation_object(ctx.translations, path));
    let fallback_translation: LiteralOption<_> = handle_macro_result!(
        get_fallback_translation(path, translation_object, ctx.fallback_language)
    )
        .into();

    inline_quote! {
        #{map_to_tokens(translation_object)}
            .get(&#{language})
            .or_else(|| #{fallback_translation})
            .ok_or_else(|| translatable::Error::LanguageNotAvailable(
                #{language},
                #{path.static_display()}.into()
            ))
            .map(|format_string| format_string
                .replace_with(&#{format_replacements(ctx.template_replacements)})
            )
    }
}

#[inline(always)]
fn all_dynamic(
    ctx: &GenerationContext,
    language: &TokenStream2,
    path: &TokenStream2,
) -> TokenStream2 {
    inline_quote! {
        (|| -> Result<std::string::String, translatable::Error> {
            // validation
            #[doc(hidden)]
            let __lang: translatable::Language = #{language};
            #[doc(hidden)]
            let __path: Vec<String> = {
                #[doc(hidden)]
                fn __to_vec<I: IntoIterator<Item = S>, S: ToString>(items: I) -> Vec<String> {
                    items.into_iter().map(|s| s.to_string()).collect()
                }
                __to_vec(#{path})
            };

            // sources
            #[doc(hidden)]
            let __translations = #{ctx.translations};
            #[doc(hidden)]
            let __found_path = __translations
                .find_path(&__path)
                .ok_or_else(|| translatable::Error::PathNotFound(__path.join("::")))?;

            // alternative
            #[doc(hidden)]
            let __fallback_translation = #{LiteralOption::from(ctx.fallback_language)}
                .map(|fallback| __found_path
                    .get(&fallback)
                    .ok_or_else(|| translatable::Error::FallbackNotAvailable(fallback, __path.join("::")))
                )
                .transpose()?;

            __translations
                .find_path(&__path)
                .and_then(|obj| obj
                    .get(&__lang)
                    .or(__fallback_translation)
                )
                .ok_or_else(|| translatable::Error::LanguageNotAvailable(__lang, __path.join("::")))
                .map(|format_string| format_string
                    .replace_with(&#{format_replacements(ctx.template_replacements)})
                )
        })()
    }
}

/// [`translation!()`] macro output generation.
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
/// and any placeholder replacements obtained from [`macro_input::translation`].
///
/// **Returns**
/// Generated `TokenStream2` representing the resolved translation string or
/// runtime lookup logic.
///
/// [`macro_input::translation`]: super::super::macro_input::translation
/// [`translation!()`]: crate::translation
pub fn translation_macro(input: TranslationMacroArgs) -> TokenStream2 {
    let config = handle_macro_result!(load_config());
    let translations = handle_macro_result!(load_translations());

    let ctx = GenerationContext {
        fallback_language: config.fallback_language(),
        translations,
        template_replacements: input.replacements(),
    };

    if let InputType::Static(language) = input.language() {
        if let InputType::Static(path) = input.path() {
            return all_static(&ctx, *language, path);
        }
    }

    let language = match input.language() {
        InputType::Static(language) => language.to_token_stream(),

        InputType::Dynamic(language) => quote! {
            translatable::shared::misc::language::Language::from(#language)
        },
    };

    match input.path() {
        InputType::Static(path) => path_static(&ctx, &language, &path),

        InputType::Dynamic(path) => all_dynamic(&ctx, &language, &path),
    }
}

use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use thiserror::Error;
use translatable_shared::handle_macro_result;
use translatable_shared::language::Language;
use translatable_shared::macros::collections::map_to_tokens;

use crate::data::translations::load_translations;
use crate::macro_input::input_type::InputType;
use crate::macro_input::translation::TranslationMacroArgs;

#[derive(Error, Debug)]
enum MacroCompileError {
    #[error("The path '{0}' could not be found")]
    PathNotFound(String),

    #[error("The language '{0:?}' ('{0:#}') is not available for the path '{1}'")]
    LanguageNotAvailable(Language, String),
}

pub fn translation_macro(input: TranslationMacroArgs) -> TokenStream2 {
    let translations = handle_macro_result!(load_translations());

    let template_replacements =
        input.replacements().iter().map(|(key, value)| quote! { #key = #value });

    if let InputType::Static(language) = input.language() {
        if let InputType::Static(path) = input.path() {
            let static_path_display = path.join("::");

            let translation_object = translations
                .find_path(&path)
                .ok_or_else(|| MacroCompileError::PathNotFound(static_path_display.clone()));

            let translation =
                handle_macro_result!(translation_object).get(language).ok_or_else(|| {
                    MacroCompileError::LanguageNotAvailable(
                        language.clone(),
                        static_path_display.clone(),
                    )
                });

            let translation = handle_macro_result!(translation).into_token_stream();

            return quote! {
                translatable::shared::replace_templates!(
                    #translation,
                    #(#template_replacements),*
                )
            };
        }
    }

    let language = match input.language() {
        InputType::Static(language) => language.clone().to_token_stream(),
        InputType::Dynamic(language) => quote! {
            translatable::shared::language::Language::from(#language)
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

                translatable::shared::replace_templates!(
                    {
                        #translation_object
                            .get(&language)
                            .ok_or_else(|| translatable::Error::LanguageNotAvailable(language, path.join("::")))?
                            .to_string()
                    },
                    #(#template_replacements),*
                )?
            })
        })()
    }
}

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use thiserror::Error;
use translatable_shared::handle_macro_result;
use translatable_shared::macros::collections::map_to_tokens;

use crate::data::translations::load_translations;
use crate::macro_input::context::{ContextMacroArgs, ContextMacroStruct};
use crate::macro_input::utils::translation_path::TranslationPath;

#[derive(Error, Debug)]
enum MacroCompileError {
    #[error("A translation with the path '{0}' could not be found.")]
    TranslationNotFound(String)
}

pub fn context_macro(base_path: ContextMacroArgs, macro_input: ContextMacroStruct) -> TokenStream2 {
    let translations = handle_macro_result!(load_translations());
    let base_path = base_path.into_inner().unwrap_or_else(|| TranslationPath::default());

    let struct_pub = macro_input.pub_state();
    let struct_ident = macro_input.ident();

    let struct_fields = macro_input
        .fields()
        .iter()
        .map(|field| {
            let field_ident = field.ident();
            quote! { #field_ident: String }
        });

    let loadable_translations = handle_macro_result!(
        macro_input
            .fields()
            .iter()
            .map(|field| {
                let path_segments = base_path
                    .merge(field.path());

                let path_segments_display = path_segments
                    .join("::");

                let translation = map_to_tokens(
                    translations
                        .find_path(&path_segments)
                        .ok_or(MacroCompileError::TranslationNotFound(path_segments.join("::")))?,
                );

                let ident = field.ident();

                Ok(quote! {
                    #ident: #translation
                        .get(&language)
                        .ok_or_else(|| translatable::Error::LanguageNotAvailable(
                            language.clone(),
                            #path_segments_display.to_string()
                        ))?
                        .replace_with(&replacements)
                })
            })
            .collect::<Result<Vec<TokenStream2>, MacroCompileError>>()
    );

    quote! {
        #struct_pub struct #struct_ident {
            #(#struct_fields),*
        }

        impl #struct_ident {
            #struct_pub fn load_translations<K: ToString, V: ToString>(
                language: translatable::Language,
                replacements: &std::collections::HashMap<K, V>
            ) -> Result<Self, translatable::Error> {
                let replacements = replacements
                    .iter()
                    .map(|(key, value)| (key.to_string(), value.to_string()))
                    .collect::<std::collections::HashMap<String, String>>();

                Ok(Self {
                    #(#loadable_translations),*
                })
            }
        }
    }
}

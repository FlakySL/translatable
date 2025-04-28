use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::Ident;
use thiserror::Error;
use translatable_shared::handle_macro_result;
use translatable_shared::macros::collections::map_to_tokens;

use crate::data::translations::load_translations;
use crate::macro_input::context::{ContextMacroArgs, ContextMacroStruct};

#[derive(Error, Debug)]
enum MacroCompileError {

}

macro_rules! pub_token {
    ($input:expr) => {
        if $input {
            quote! { pub }
        } else {
            quote! { }
        }
    };
}

pub fn context_macro(base_path: ContextMacroArgs, macro_input: ContextMacroStruct) -> TokenStream2 {
    let translations = handle_macro_result!(load_translations());

    let pub_token = pub_token!(macro_input.is_pub());
    let struct_ident = Ident::new(macro_input.ident(), Span::call_site());

    let base_path = base_path
        .or_empty()
        .segments()
        .to_vec();

    let translations = macro_input
        .fields()
        .iter()
        .map(|field| (
            field.is_pub(),
            field.ident(),
            {
                let path_segments = field
                    .path()
                    .segments()
                    .to_vec();

                let path = base_path
                    .iter()
                    .chain(&path_segments)
                    .collect();

                translations.find_path(&path)
            }
        ))
        .collect::<Vec<_>>();

    let struct_fields = macro_input
        .fields()
        .iter()
        .map(|field| {
            let pub_token = pub_token!(field.is_pub());

            let ident = Ident::new(
                field.ident(),
                Span::call_site()
            );

            quote! { #pub_token #ident: String }
        });

    let field_impls = translations
        .iter()
        .map(|(is_pub, ident, translation)| {
            let pub_token = pub_token!(*is_pub);

            let ident = Ident::new(
                ident,
                Span::call_site()
            );

            let templated_ident = format_ident!("templated_{ident}");

            let translation = translation
                .map(|translation| map_to_tokens(translation))
                .ok_or();

            quote! {
                #[inline]
                #pub_token fn #templated_ident(language: &translatable::Language)
                -> Option<translatable::shared::misc::templating::FormatString> {
                    #translation
                        .remove(language)
                }

                #[inline]
                #pub_token fn #ident(language: &translatable::Language) -> Option<String> {
                    Self::#templated_ident(language)
                        .map(|lang| lang.replace_with(std::collections::HashMap::new()))
                }
            }
        });

    quote! {
        #pub_token struct #struct_ident {
            #(#struct_fields),*
        }

        impl #struct_ident {
            #(#field_impls)*
        }
    }
}

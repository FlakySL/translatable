use std::collections::HashMap;

use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};

#[inline]
pub fn map_to_tokens<K: ToTokens, V: ToTokens>(map: &HashMap<K, V>) -> TokenStream2 {
    let map = map
        .iter()
        .map(|(key, value)| {
            let key = key.into_token_stream();
            let value = value.into_token_stream();

            quote! { (#key, #value) }
        });

    quote! {
        vec![#(#map),*]
            .into_iter()
            .collect::<std::collections::HashMap<_, _>>()
    }
}

#[inline]
pub fn map_transform_to_tokens<K, V, F>(map: &HashMap<K, V>, predicate: F) -> TokenStream2
where
    F: Fn(&K, &V) -> TokenStream2,
{
    let processed = map
        .iter()
        .map(|(key, value)| predicate(key, value));

    quote! {
        vec![#(#processed),*]
            .into_iter()
            .collect::<std::collections::HashMap<_, _>>()
    }
}

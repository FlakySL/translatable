use std::collections::HashMap;
use std::hash::Hash;

use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};

pub fn map_to_tokens<K: ToTokens, V: ToTokens>(map: &HashMap<K, V>) -> TokenStream2 {
    let map = map.into_iter().map(|(key, value)| {
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
pub fn map_transform_to_tokens<K, V, Kr, Vr, F>(map: &HashMap<K, V>, predicate: F) -> TokenStream2
where
    K: ToTokens,
    V: ToTokens,
    Kr: Eq + ToTokens + Hash,
    Vr: ToTokens + Hash,
    F: Fn(&K, &V) -> (Kr, Vr),
{
    map_to_tokens(
        &map
            .iter()
            .map(|(key, value)| predicate(key, value))
            .collect::<HashMap<_, _>>()
    )
}

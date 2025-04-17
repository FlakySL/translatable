//! Shared collection utils module.
//!
//! This module declares functions used by `translatable_proc`
//! and `translatable_shared` together, mostly used to convert
//! compile-time structures into runtime representations of
//! the same structures.

use std::collections::HashMap;

use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};

/// [`HashMap<K, V>`] runtime conversion.
///
/// This function converts a [`HashMap<K, V>`] into a [`TokenStream2`]
/// that when generated on a macro contains the same values as the initial
/// map.
///
/// The type of the keys and values of the map must implement [`ToTokens`].
///
/// **Parameters**
/// * `map` - The map to convert into tokens.
///
/// **Returns**
/// The provided `map` parameter represented as [`TokenStream2`].
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

/// [`HashMap<K, V>`] runtime conversion and mapping.
///
/// Similarly to [`map_to_tokens`] this function converts a [`HashMap<K, V>`]
/// into a [`TokenStream2`] that when generated on a macro contains the same
/// values as the original map. The difference is that in this function the keys
/// and values types don't need to implement [`ToTokens`], as this takes a
/// predicate which lets you modify values before converting it to tokens.
///
/// The predicate must return a [`TokenStream2`] containing tuples, the internal
/// conversion is as `vec![$($converted),*]` collected into a [`HashMap<K, V>`]
/// in runtime.
///
/// **Parameters**
/// * `map` - The map to convert into tokens.
/// * `predicate` - A predicate taking a key and a value that should return a
///   [`TokenStream2`]
/// containing a tuple of the key and the value transformed in any way.
///
/// **Returns**
/// The provided `map` parameter mutated with the `predicate` and converted to a
/// [`TokenStream2`].
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

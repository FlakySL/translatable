use std::collections::HashMap;

use quote::quote;
use translatable_shared::macros::collections::{map_to_tokens, map_transform_to_tokens};

#[test]
pub fn map_to_tokens_has_literals() {
    let tokens = map_to_tokens(&{
        let mut map = HashMap::new();
        map.insert("key1", 1);
        map.insert("key2", 2);

        map
    })
    .to_string();

    assert!(tokens.contains("\"key1\""));
    assert!(tokens.contains("1"));
    assert!(tokens.contains("\"key2\""));
    assert!(tokens.contains("2"));
}

#[test]
pub fn map_transform_to_tokens_has_literals() {
    let tokens = map_transform_to_tokens(
        &{
            let mut map = HashMap::new();
            map.insert("key1", 1i32);

            map
        },
        |key, value| quote! { (#key, #value.to_string()) },
    )
    .to_string()
    .replace(" ", ""); // normalize

    assert!(tokens.contains("vec![(\"key1\",1i32.to_string())]"));
}

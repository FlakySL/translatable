use std::collections::HashMap;

use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, TokenStreamExt, quote};

use super::node::{TranslationNode, TranslationObject};
use crate::macros::collections::map_transform_to_tokens;

/// Wraps a collection of translation nodes, these translation nodes
/// are usually directly loaded files, and the keys to access them
/// independently are the complete system path. The collection
/// permits searching for translations by iterating all the files
/// in the specified configuration order, so most likely you don't
/// need to seek for a translation independently.
pub struct TranslationNodeCollection(HashMap<String, TranslationNode>);

impl TranslationNodeCollection {
    pub fn new(collection: HashMap<String, TranslationNode>) -> Self {
        Self(collection)
    }

    /// This method may be used to load a translation
    /// independently, if you are looking for an independent
    /// translation you may want to call find_path instead.
    ///
    /// # Arguments
    /// * `path` - The OS path where the file was originally found.
    ///
    /// # Returns
    /// A top level translation node, containing all the translations
    /// in that specific file.
    #[allow(unused)]
    pub fn get_node(&self, path: &str) -> Option<&TranslationNode> {
        self.0
            .get(path)
    }

    /// This method is used to load a specific translation
    /// file agnostic from a "translation path" which consists
    /// of the necessary TOML object path to reach a specific
    /// translation object.
    ///
    /// # Arguments
    /// * `path` - The sections of the TOML path in order to access
    /// the desired translation object.
    ///
    /// # Returns
    /// A translation object containing a specific translation
    /// in all it's available languages.
    pub fn find_path<I: ToString>(&self, path: &Vec<I>) -> Option<&TranslationObject> {
        self.0
            .values()
            .find_map(|node| node.find_path(path))
    }
}

/// Abstraction to easily collect a `HashMap` and wrap it
/// in a `TranslationNodeCollection`.
impl FromIterator<(String, TranslationNode)> for TranslationNodeCollection {
    fn from_iter<T: IntoIterator<Item = (String, TranslationNode)>>(iter: T) -> Self {
        Self(
            iter.into_iter()
                .collect(),
        )
    }
}

impl ToTokens for TranslationNodeCollection {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let map =
            map_transform_to_tokens(&self.0, |key, value| quote! { (#key.to_string(), #value) });

        tokens.append_all(quote! {
            translatable::shared::translations::collection::TranslationNodeCollection::new(
                #map
            )
        });
    }
}

//! Translation file collection module.
//!
//! This module declares [`TranslationNodeCollection`]
//! a representation of each file found in the translations
//! folder defined in the configuration file.

use std::collections::HashMap;

use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, TokenStreamExt, quote};

use super::node::{TranslationNode, TranslationObject};
use crate::macros::collections::map_transform_to_tokens;

/// Translation file collection.
///
/// This tuple struct wraps a hashmap implementing
/// a lookup trough all the files in ascending order.
///
/// The internal hashmap contains the original file
/// paths along all the unmerged [`TranslationNode`]
/// found in each file.
pub struct TranslationNodeCollection(HashMap<String, TranslationNode>);

impl TranslationNodeCollection {
    /// Create a new [`TranslationNodeCollection`].
    ///
    /// By providing a populated hashmap, create a new
    /// [`TranslationNodeCollection`] structure.
    ///
    /// The file paths in the hashmap key aren't validated. This
    /// is usually called from a `to-runtime` implementation, if
    /// you want to obtain all the translation files use 
    ///
    /// **Arguments**
    /// * `collection` - An already populated collection for lookup.
    ///
    /// **Returns**
    /// The provided collection wrapped in a [`TranslationNodeCollection`].
    pub fn new(collection: HashMap<String, TranslationNode>) -> Self {
        Self(collection)
    }

    /// Get a node from a file path.
    ///
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

    /// Search a path trough all the nodes.
    ///
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

/// Hashmap wrapper implementation.
///
/// Abstraction to easily collect a [`HashMap<String, TranslationNode>`] and wrap it
/// in a [`TranslationNodeCollection`].
impl FromIterator<(String, TranslationNode)> for TranslationNodeCollection {
    fn from_iter<T: IntoIterator<Item = (String, TranslationNode)>>(iter: T) -> Self {
        Self(
            iter
                .into_iter()
                .collect(),
        )
    }
}

/// Compile-time to runtime implementation.
///
/// This implementation generates the call to [`new`] on
/// [`TranslationNodeCollection`] with the data from the current
/// instance to perform a compile-time to runtime conversion.
///
/// [`new`]: TranslationNodeCollection::new
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

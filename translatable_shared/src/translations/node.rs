//! Translation node declaration module.
//!
//! This module declares [`TranslationNode`] which
//! is a nested enum that behaves like a n-ary tree
//! for which each branch contains paths that might
//! lead to translation objects or other paths.

use std::collections::HashMap;

use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, TokenStreamExt, quote};
use strum::ParseError;
use thiserror::Error;
use toml::{Table, Value};

use crate::macros::collections::{map_to_tokens, map_transform_to_tokens};
use crate::misc::language::Language;
use crate::misc::templating::{FormatString, TemplateError};

/// [`TranslationNode`] errors.
///
/// This error is agnostic to the runtime, it is used
/// for errors while parsing a [`TranslationNode`] or
/// while trying seeking for it's content.
#[derive(Error, Debug)]
pub enum TranslationNodeError {
    /// Invalid object type error.
    ///
    /// This error signals that the nesting rules were
    /// broken, thus the parsing cannot continue.
    #[error("A nesting can only contain translation objects or other nestings")]
    InvalidNesting,

    /// Template validation error.
    ///
    /// This means there was an error while validating
    /// a translation templates, such as an invalid
    /// ident for its keys or unclosed templates.
    #[error("Template validation failed: {0:#}")]
    TemplateValidation(#[from] TemplateError),

    /// Invalid value found inside a nesting.
    ///
    /// This error signals that an invalid value was found
    /// inside a nesting, such as mixed values.
    #[error(
        "Mixed values are not allowed, a nesting can't contain strings and objects at the same \
         time"
    )]
    MixedValues,

    /// Invalid ISO-639-1 translation key.
    ///
    /// This error signals that an invalid key was found for a
    /// translation inside a translation object.
    ///
    /// Translation keys must follow the ISO-639-1 standard.
    #[error("Couldn't parse ISO 639-1 string for translation key")]
    LanguageParsing(#[from] ParseError),

    /// Empty translation file.
    ///
    /// This error signals that a created translation file
    /// is empty and cannot be parsed.
    #[error("A translation file cannot be empty")]
    EmptyTable
}

/// Nesting type alias.
///
/// This is one of the valid objects that might be found
/// on a translation file, this object might contain a translation
/// or another nesting.
pub type TranslationNesting = HashMap<String, TranslationNode>;

/// Object type alias.
///
/// This is one of the valid objects that might be found
/// on a translation file, this object contains only translations
/// keyed with their respective languages.
pub type TranslationObject = HashMap<Language, FormatString>;

/// Translation node structure.
///
/// This enum acts like an n-ary tree which
/// may contain [`TranslationNesting`] or
/// [`TranslationObject`] representing a tree
/// that follows the translation file rules.
pub enum TranslationNode {
    /// Branch containing a [`TranslationNesting`].
    ///
    /// Read the [`TranslationNesting`] documentation for
    /// more information.
    Nesting(TranslationNesting),

    /// Branch containing a [`TranslationObject`].
    ///
    /// Read the [`TranslationObject`] documentation for
    /// more information.
    Translation(TranslationObject),
}

impl TranslationNode {
    /// Resolves a translation path through the nesting hierarchy.
    ///
    /// **Arguments**
    /// * `path` - Slice of path segments to resolve.
    ///
    /// **Returns**
    /// A reference to translations if path exists and points to leaf node.
    pub fn find_path<I: ToString>(&self, path: &Vec<I>) -> Option<&TranslationObject> {
        let path = path
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>();

        match self {
            Self::Nesting(nested) => {
                let (first, rest) = path.split_first()?;
                nested
                    .get(first)?
                    .find_path(&rest.to_vec())
            },
            Self::Translation(translation) => path
                .is_empty()
                .then_some(translation),
        }
    }
}

/// Compile-time to runtime conversion implementation.
///
/// This implementation converts a [`TranslationNode`] into
/// runtime trough tokens by nesting calls depending on the
/// type inferred in compile-time.
///
/// This is usually used for dynamic paths.
impl ToTokens for TranslationNode {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            TranslationNode::Nesting(nesting) => {
                let map = map_transform_to_tokens(
                    nesting,
                    |key, value| quote! { (#key.to_string(), #value) },
                );

                tokens.append_all(quote! {
                    translatable::shared::translations::node::TranslationNode::Nesting(
                        #map
                    )
                });
            },

            TranslationNode::Translation(translation) => {
                let map = map_to_tokens(translation);

                tokens.append_all(quote! {
                    translatable::shared::translations::node::TranslationNode::Translation(
                        #map
                    )
                });
            },
        }
    }
}

/// TOML table parsing.
///
/// This implementation parses a TOML table object
/// into a [`TranslationNode`] for validation and
/// seeking the translations according to the rules.
impl TryFrom<Table> for TranslationNode {
    type Error = TranslationNodeError;

    // The top level can only contain objects is never enforced.
    fn try_from(value: Table) -> Result<Self, Self::Error> {
        let mut result = None;

        for (key, value) in value {
            match value {
                Value::String(translation_value) => {
                    match result.get_or_insert_with(|| Self::Translation(HashMap::new())) {
                        Self::Translation(translation) => {
                            translation.insert(key.parse()?, translation_value.parse()?);
                        },

                        Self::Nesting(_) => return Err(TranslationNodeError::MixedValues),
                    }
                },

                Value::Table(nesting_value) => {
                    match result.get_or_insert_with(|| Self::Nesting(HashMap::new())) {
                        Self::Nesting(nesting) => {
                            nesting.insert(key, Self::try_from(nesting_value)?);
                        },

                        Self::Translation(_) => return Err(TranslationNodeError::MixedValues),
                    }
                },

                _ => return Err(TranslationNodeError::InvalidNesting),
            }
        }

        result.ok_or(TranslationNodeError::EmptyTable)
    }
}

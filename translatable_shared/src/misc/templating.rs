//! String template generation module.
//!
//! This module declares the [`FormatString`]
//! which is a structure to parse templates
//! and generate strings of them with replaced
//! parameters.

use std::collections::HashMap;
use std::ops::Range;
use std::str::FromStr;

use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, TokenStreamExt, quote};
use syn::{Ident, parse_str};
use thiserror::Error;

use crate::macros::collections::map_transform_to_tokens;

/// Template parsing errors.
///
/// This error is used within [`FormatString`]
/// to represent parsing errors such as unclosed
/// unescaped tags or invalid identifiers.
#[derive(Error, Debug)]
pub enum TemplateError {
    /// Unclosed brace error.
    ///
    /// This error is returned when a brace
    /// that was considered unescaped
    /// was not closed after reaching the
    /// last character of the string.
    #[error("Found unclosed brace at index {0}")]
    Unclosed(usize),

    /// Invalid ident error.
    ///
    /// This error is returned when a key
    /// inside the braces couldn't be parsed
    /// as an [`Ident`], invalid identifiers
    /// are checked because of macro parsing.
    #[error("Found template with key '{0}' which is an invalid identifier")]
    InvalidIdent(String),
}

/// Format string wrapper struct.
///
/// This struct wraps a string and has
/// a counter of each template it has
/// with each respective position for
/// the sake of replacing these positions
/// with read data.
pub struct FormatString {
    /// Original templated string.
    ///
    /// This field contains the original
    /// string that aligns it's keyed templates
    /// with `self.spans`.
    ///
    /// This should never be mutated for the sake
    /// of keping the alignment with `self.spans`.
    original: String,

    /// Template spans.
    ///
    /// This hashmap contains the ranges
    /// of the templates found in the
    /// original string, for the sake
    /// of replacing them in a copy
    /// of the original string.
    spans: Vec<(String, Range<usize>)>,
}

impl FormatString {
    pub fn from_data(original: &str, spans: Vec<(String, Range<usize>)>) -> Self {
        Self { original: original.to_string(), spans }
    }

    pub fn replace_with(&self, values: HashMap<String, String>) -> String {
        let mut original = self
            .original
            .clone();

        let mut spans = self.spans.clone();
        spans.sort_by_key(|(_key, range)| range.start);

        let mut offset = 0isize;

        for (key, range) in spans {
            if let Some(value) = values.get(&key) {
                let start = (range.start as isize + offset) as usize;
                let end = (range.end as isize + offset) as usize;

                original.replace_range(start..end, value);

                offset += value.len() as isize - (range.end - range.start) as isize;
            }
        }

        original
    }
}

impl FromStr for FormatString {
    type Err = TemplateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let original = s.to_string();
        let mut spans = Vec::new();

        let char_to_byte = s
            .char_indices()
            .map(|(i, _)| i)
            .collect::<Vec<usize>>();

        let mut last_bracket_idx = None;
        let mut current_tmpl_key = String::new();
        for (char_idx, c) in original
            .chars()
            .enumerate()
        {
            match (c, last_bracket_idx) {
                // if last template index is the last character
                // ignore current as is escaped.
                ('{', Some(prev)) if prev == char_idx.saturating_sub(1) => last_bracket_idx = None,
                // if last template index is anything but the last character
                // set it as last index.
                ('{', _) => last_bracket_idx = Some(char_idx),

                // if last template index is not 0 and we find
                // a closing bracket complete a range.
                ('}', Some(open_idx)) => {
                    let key = current_tmpl_key.clone();

                    spans.push((
                        parse_str::<Ident>(&key)
                            .map_err(|_| TemplateError::InvalidIdent(key))?
                            .to_string(),
                        char_to_byte[open_idx]..char_to_byte[char_idx + 1], // inclusive
                    ));

                    last_bracket_idx = None;
                    current_tmpl_key.clear();
                },

                (c, Some(_)) => current_tmpl_key.push(c),

                _ => {},
            }
        }

        if let Some(lbi) = last_bracket_idx {
            Err(TemplateError::Unclosed(lbi))
        } else {
            Ok(FormatString { original, spans })
        }
    }
}

impl ToTokens for FormatString {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let original = &self.original;

        let span_map = self
            .spans
            .iter()
            .map(|(key, range)| {
                let start = range.start;
                let end = range.end;

                quote! { (#key.to_string(), #start..#end) }
            });

        tokens.append_all(quote! {
            translatable::shared::misc::templating::FormatString::from_data(
                #original,
                vec![#(#span_map),*]
            )
        });
    }
}

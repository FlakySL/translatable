//! [`TranslationPath`] module.
//!
//! This module declares an abstraction
//! to parse [`syn::Path`] disallowing
//! generic type arguments.
//!
//! This module doesn't have anything
//! to do with [`std::path`].

use proc_macro2::Span;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{Error as SynError, Path, PathArguments, Result as SynResult};

/// Static translation path parser.
///
/// This parser structure is an abstraction
/// of [`syn::Path`] but disallowing generic
/// types.
///
/// The structure is spanned preserving
/// the original path unless defaulted, otherwise
/// the span is callsite.
///
/// The structure is completly immutable.
#[derive(Clone)]
pub struct TranslationPath {
    /// The path segments.
    ///
    /// The segments are translated
    /// from a `syn::Path` as
    /// x::y -> vec!["x", "y"].
    segments: Vec<String>,

    /// The path original span
    /// unless default, then empty.
    span: Span,
}

/// [`TranslationPath`] macro parsing implementation.
///
/// Used to parse arguments with [`parse2`] or [`parse_macro_input!`]
/// within attribute arguments.
///
/// [`parse2`]: syn::parse2
/// [`parse_macro_input!`]: syn::parse_macro_input
impl Parse for TranslationPath {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let path = input.parse::<Path>()?;

        let span = path.span();
        let segments = path
            .segments
            .into_iter()
            .map(|segment| match segment.arguments {
                PathArguments::None => Ok(segment
                    .ident
                    .to_string()),

                error => Err(SynError::new_spanned(
                    error,
                    "A translation path can't contain generic arguments.",
                )),
            })
            .collect::<Result<_, _>>()?;

        Ok(Self { segments, span })
    }
}

/// Default implementation for [`TranslationPath`].
///
/// Used to create empty translation paths usually
/// for fallbacks with `Option::<TranslationPath>::unwrap_or_else()`.
///
/// The span generated for a [`TranslationPath::default`] call is
/// [`Span::call_site`].
impl Default for TranslationPath {
    fn default() -> Self {
        Self {
            segments: Vec::new(),
            span: Span::call_site(),
        }
    }
}

impl TranslationPath {
    /// Constructor function for [`TranslationPath`].
    ///
    /// This constructor function should be called with
    /// partial arguments from another function. Nothing
    /// happens if it's not.
    ///
    /// **Arguments**
    /// * `segments` - The segments this path is made of x::y -> vec!["x", "y"].
    /// * `span` - The original location or where this path should return errors
    /// if it may.
    ///
    /// **Returns**
    /// A constructed instance of [`TranslationPath`].
    #[inline]
    pub fn new(segments: Vec<String>, span: Span) -> Self {
        Self { segments, span }
    }

    /// Path merging helper method.
    ///
    /// This method takes both internal path segments and appends
    /// both making a vector out of the merge.
    ///
    /// Since spans cannot be split or we may not have multiple
    /// spans without having a complex structure then the span
    /// is directly not preserved.
    ///
    /// **Arguments**
    /// * `other` - The path this instance should be merged with.
    ///
    /// **Returns**
    /// A single vector with both internal paths merged.
    pub fn merge(&self, other: &Self) -> Vec<String> {
        // TODO: merge spans (not yet in #19)
        [
            self.segments().to_vec(),
            other.segments().to_vec()
        ]
            .concat()
    }

    /// Internal segments getter.
    ///
    /// **Returns**
    /// The internal segments.
    #[inline]
    #[allow(unused)]
    pub fn segments(&self) -> &Vec<String> {
        &self.segments
    }

    /// Internal span getter.
    ///
    /// **Returns**
    /// The internal span.
    #[inline]
    #[allow(unused)]
    pub fn span(&self) -> Span {
        // TODO: possibly implement Spanned
        self.span
    }
}

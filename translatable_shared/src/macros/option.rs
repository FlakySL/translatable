//! [`Option`] wrapper module.
//!
//! This module declares an option wrapper
//! that instead of conditionally rendering
//! renders literally.

use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};

/// [`Option`] wrapper for literal rendering.
///
/// This wrapper provides an interface to rendering
/// an [`Option`] instance literally such as
/// `std::option::Option::Some(..)`
/// or `std::option::Option::None` instead of
/// conditionally rendering as usual [`Option`].
pub struct LiteralOption<T: ToTokens>(Option<T>);

/// Wrapper main purpose implementation.
///
/// This implementation renders the inner
/// [`Option`] as a literal runtime call of
/// [`Option`] with it's literal path specified.
///
/// This does not support `core::option`. *yet*
impl<T: ToTokens> ToTokens for LiteralOption<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(
            match &self.0 {
                Some(value) => quote! { std::option::Option::Some(#value) },
                None => quote! { std::option::Option::None }
            }
        );
    }
}

/// Convenience [`From`] implementation.
///
/// Wraps the [`Option`] instance.
impl<T: ToTokens> From<Option<T>> for LiteralOption<T> {
    fn from(value: Option<T>) -> Self {
        Self(value)
    }
}

/// Conveniento [`Into`] implementation.
///
/// Unwraps the [`Option`] instance.
impl<T: ToTokens> Into<Option<T>> for LiteralOption<T> {
    fn into(self) -> Option<T> {
        self.0
    }
}

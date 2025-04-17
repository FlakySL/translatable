//! Input type abstraction for macro argument separation.
//!
//! This module defines the [`InputType`] enum,
//! which is used to distinguish between static
//! and dynamic values during macro input parsing.

use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;

/// Input type differentiation enum.
///
/// Represents whether an input is a static,
/// compile-time known value or a dynamic,
/// runtime expression. This differentiation
/// allows the translation system to apply
/// optimizations based on the input nature.
pub enum InputType<T: Sized> {
    /// Statically known value.
    ///
    /// The input is fully resolved at compile time, which allows
    /// the macro system to optimize for constant substitution and
    /// code simplification.
    ///
    /// **Parameters**
    /// * `0` — The static value.
    Static(T),

    /// Dynamically evaluated input.
    ///
    /// The input is represented as a [`TokenStream2`] expression,
    /// which is evaluated at runtime rather than compile time.
    ///
    /// **Parameters**
    /// * `0` — The dynamic [`TokenStream2`] expression.
    Dynamic(TokenStream2),
}

/// [`InputType`] runtime normalization implementation.
///
/// This implementation is used to convert [`InputType`]
/// into normalized runtime values in many aspects, only
/// if T implements [`ToTokens`].
impl<T: ToTokens> InputType<T> {
    /// [`InputType`] to [`TokenStream2`] conversion.
    ///
    /// This method takes an [`InputType`] and converts
    /// any of it's branches to a [`TokenStream2`] if
    /// available.
    ///
    /// **Returns**
    /// A [`TokenStream2`] representation of whatever the value
    /// is in the [`InputType`].
    #[inline]
    #[allow(unused)]
    fn dynamic(self) -> TokenStream2 {
        match self {
            Self::Static(value) => value.to_token_stream(),
            Self::Dynamic(value) => value,
        }
    }
}

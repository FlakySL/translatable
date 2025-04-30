//! Error utils module.
//!
//! This module declares blanket implementations
//! for error utils such as conversion to tokens
//! or other errors.

use std::fmt::Display;

use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::Error as SynError;

/// Error implementations for macro outputs.
///
/// This trait is meant to be implemented
/// as a blanket where every type that
/// implements [`Display`] can be converted
/// or either to a compile error or a [`SynError`].
pub trait IntoCompileError
where
    Self: Display + Sized,
{
    /// Convert error reference to runtime.
    ///
    /// Transforms the value into a string
    /// and wraps [`compile_error!`] into it
    /// for it to be returned when an error
    /// happens.
    ///
    /// The invocation happens inside a method
    /// for compatibility in both outside and
    /// inside functions.
    ///
    /// **Returns**
    /// A [`compile_error!`] wrapped `&str`.
    #[cold]
    fn to_compile_error(&self) -> TokenStream2 {
        let message = self.to_string();
        quote! { std::compile_error!(#message) }
    }

    fn to_out_compile_error(&self) -> TokenStream2 {
        let invocation = self.to_compile_error();
        quote! { fn __() { #invocation } }
    }

    /// Convert error reference to a spanned [`SynError`].
    ///
    /// Transforms the value into a string
    /// and creates a spanned [`SynError`]
    /// with the user provided span.
    ///
    /// **Parameters**
    /// * `span` - the error span for the `rust-analyzer` report.
    ///
    /// **Returns**
    /// A [`SynError`] with the value as a message and the provided `span`.
    #[cold]
    fn to_syn_error<T: ToTokens>(&self, span: T) -> SynError {
        SynError::new_spanned(span, self.to_string())
    }
}

/// [`IntoCompileError`] blanket implementation
/// for values that implement [`Display`].
impl<T: Display> IntoCompileError for T {}

/// [`to_compile_error`] conversion helper macro.
///
/// This macro takes a [`Result<T, E>`] where
/// `E` implements [`Display`] and generates
/// a match branch which directly returns the error
/// as a compile error.
///
/// This macro is meant to be called from a macro
/// generation function.
///
/// [`to_compile_error`]: IntoCompileError::to_compile_error
#[macro_export]
macro_rules! handle_macro_result {
    ($method:ident; $val:expr) => {{
        use $crate::macros::errors::IntoCompileError;

        match $val {
            std::result::Result::Ok(value) => value,
            std::result::Result::Err(error) => return error.$method(),
        }
    }};

    ($val:expr) => {
        $crate::handle_macro_result!(to_compile_error; $val)
    };

    (out $val:expr) => {
        $crate::handle_macro_result!(to_out_compile_error; $val)
    };
}

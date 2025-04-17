use std::fmt::Display;

use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::Error as SynError;

/// Implements a helper function to convert
/// anything that implements Display into
/// a generated `compile_error!` in macros.
pub trait IntoCompileError
where
    Self: Display + Sized,
{
    /// Transforms the value into a string
    /// and wraps `compile_error!` into it
    /// for it to be returned when an error
    /// happens
    fn to_compile_error(&self) -> TokenStream2 {
        let message = self.to_string();
        quote! { std::compile_error!(#message) }
    }

    fn into_syn_error<T: ToTokens>(self, span: T) -> SynError {
        SynError::new_spanned(span, self.to_string())
    }
}

impl<T: Display> IntoCompileError for T {}

#[macro_export]
macro_rules! handle_macro_result {
    ($val:expr) => {{
        use $crate::macros::errors::IntoCompileError;

        match $val {
            std::result::Result::Ok(value) => value,
            std::result::Result::Err(error) => return error.to_compile_error(),
        }
    }};
}

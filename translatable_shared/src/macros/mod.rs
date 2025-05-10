//! Macro helpers module.
//!
//! This module contains sub-modules
//! which or either help converting
//! compile-time structures into their
//! runtime representations with [`TokenStream2`]
//! or any other utils to generate
//! runtime code.
//!
//! [`TokenStream2`]: proc_macro2::TokenStream

pub mod collections;
pub mod errors;
pub mod inline_quote;

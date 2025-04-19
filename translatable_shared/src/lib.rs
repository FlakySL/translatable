//! Shared util declarations for `translatable` and `translatable_proc`
//!
//! This crate shouldn't be used by itself,
//! since it contains macro generation code which
//! relies on references from the `translatable` library.
//!
//! The `translatable` library re-exports the utils
//! declared in this crate and exposes the necessary
//! ones.

pub mod macros;
pub mod misc;
pub mod translations;

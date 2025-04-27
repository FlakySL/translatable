//! Macro input parsing module.
//!
//! This module contains the sub-modules
//! to parse macro input for specific
//! macros, the parsed input is usually
//! fed to [`macro_generation`] as intrinsics.
//!
//! Each sub-module represents a different macro,
//! except for separated utils.
//!
//! [`macro_generation`]: crate::macro_generation

pub mod context;
pub mod translation;
pub mod utils;

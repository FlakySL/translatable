//! Macro generation module.
//!
//! This module contains the sub-modules
//! to generate any kind of macro, in the
//! `lib.rs` file, a call to any of this
//! modules may be issued with intrinsics
//! from the [`macro_input`] module.
//!
//! Each module represents a single macro.
//!
//! [`macro_input`]: crate::macro_input

pub mod context;
pub mod translation;

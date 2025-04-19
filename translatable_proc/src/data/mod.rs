//! External data obtention module.
//!
//! This module contains the sub-modules
//! to obtain the translation data and
//! related configuration.
//!
//! The only thing that should possibly
//! be used outside is the [`translations`]
//! module, as the config is mostly
//! to read the translations from the files.

pub mod config;
pub mod translations;

mod error;

// Export the private error module
pub use error::RuntimeError as Error;
/// Re-export the language in the crate top level as
/// it's a macro parameter
pub use shared::Language;
/// Re-export the procedural macro for crate users
pub use translatable_proc::translation;

/// This module re-exports structures used by macros
/// that should not but could be used by the users
/// of the library
pub mod shared {
    /// Re-export utils used for both runtime and compile-time
    pub use translatable_shared::{
        Language,
        LanguageIter,
        TranslationNode,
        TranslationNodeCollection,
        TranslationNodeError,
    };
}

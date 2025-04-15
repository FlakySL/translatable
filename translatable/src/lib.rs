mod error;

// A re-export to the runtime error
// enum for user availabilty and
// debugging.
pub use error::RuntimeError as Error;
/// A re-export from the Language enum
/// for users to dynamically parse
/// when using dynamic arguments.
pub use shared::misc::language::Language;
/// A re-export to the translation macro
/// exported in the proc_macro module.
pub use translatable_proc::translation;
/// A re-export of all the shared modules
/// as declared in the shared crate used
/// for macro generation.
#[doc(hidden)]
pub use translatable_shared as shared;

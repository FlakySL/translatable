use thiserror::Error;
use translatable_shared::misc::language::Language;
use translatable_shared::misc::templating::TemplateError;
use translatable_shared::translations::node::TranslationNodeError;

/// Macro runtime error handling.
///
/// Used in `translation!(...)` invocations for non
/// compile time validations and errors.
///
/// Use the `Display` implementation to obtain the
/// error message, `self.cause()` is available as
/// a helper method for such purpose. Read it's
/// documentation before using.
#[derive(Error, Debug)]
pub enum RuntimeError {
    /// Translation node error derivations.
    ///
    /// `TranslationNode` construction failure,
    /// usually nesting missmatch, invalid
    /// template validation...
    ///
    /// `Display` directly forwards the inner
    /// error `Display` value.
    #[error("{0:#}")]
    TranslationNode(#[from] TranslationNodeError),

    /// Dynamic path resolve error.
    ///
    /// The specified path may not be found
    /// in any of the translation files.
    ///
    /// This is not related to run time language
    /// validity, check `Error::LanguageNotAvailable`
    /// for that purpose.
    #[error("The path '{0}' could not be found")]
    PathNotFound(String),

    /// Dynamic language obtention error.
    ///
    /// This specifically happens when a language
    /// is not available for a specific translation.
    ///
    /// Language parsing is delegated to the user,
    /// the language parameter must be a `Language`,
    /// if it's a &str the validation is made in compile
    /// time. In that case we don't reach run time.
    #[error("The language '{0:?}' ('{0:#}') is not available for the path '{1}'")]
    LanguageNotAvailable(Language, String),
}

impl RuntimeError {
    /// Runtime error display helper.
    ///
    /// This method is marked as `cold`
    /// so it should be called lazily with
    /// monads such as `ok_or_else` or any
    /// other `or_else` method.
    ///
    /// # Returns
    /// A heap allocated string containing
    /// the cause of the error.
    #[cold]
    #[inline]
    pub fn cause(&self) -> String {
        format!("{self:#}")
    }
}

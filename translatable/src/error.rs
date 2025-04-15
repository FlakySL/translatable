use thiserror::Error;
use translatable_shared::misc::language::Language;
use translatable_shared::misc::templating::TemplateError;
use translatable_shared::translations::node::TranslationNodeError;

/// This enum is used to debug runtime errors generated
/// by the macros in runtime, the error message can be obtained
/// using the `Display` trait of the enum itself.
#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("{0:#}")]
    TranslationNode(#[from] TranslationNodeError),

    #[error("The path '{0}' could not be found")]
    PathNotFound(String),

    #[error("The language '{0:?}' ('{0:#}') is not available for the path '{1}'")]
    LanguageNotAvailable(Language, String),

    #[error("An error has occurred while parsing templates: {0:#}")]
    TemplateMissmatch(#[from] TemplateError),
}

impl RuntimeError {
    /// This method makes use of the `Display` implemeted in
    /// `Error` to display the formatted cause String of
    /// the specific error.
    ///
    /// This method is marked as `cold`, because in the application
    /// there should be the least amount of errors possible,
    /// when displaying the error, please do in a lazy
    /// error handling method such as `ok_or_else` or `inspect_err`.
    ///
    /// # Returns
    /// The cause heap allocated String.
    #[cold]
    #[inline]
    pub fn cause(&self) -> String {
        format!("{self:#}")
    }
}

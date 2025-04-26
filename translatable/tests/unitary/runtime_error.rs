use translatable::{Error, Language};

#[test]
pub fn runtime_error_outputs() {
    assert_eq!(
        Error::PathNotFound("path::to::translation".into()).cause(),
        "The path 'path::to::translation' could not be found"
    );

    assert_eq!(
        Error::LanguageNotAvailable(Language::ES, "path::to::translation".into()).cause(),
        "The language 'ES' ('Spanish') is not available for the path 'path::to::translation'"
    )
}

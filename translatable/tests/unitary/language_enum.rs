use quote::ToTokens;
use translatable::Language;

#[test]
pub fn language_enum_parsing_case_insensitive() {
    let language_lower = "es".parse::<Language>();
    let language_upper = "ES".parse::<Language>();

    assert!(language_lower.is_ok());
    assert!(language_upper.is_ok());
}

#[test]
pub fn language_enum_to_tokens() {
    let language_tokens = Language::ES
        .into_token_stream()
        .to_string()
        .replace(" ", ""); // normalize the path.

    assert!(language_tokens.contains("translatable::shared::misc::language::Language::ES"));
}

#[test]
pub fn display_matches() {
    assert_eq!(Language::ES.to_string(), "Spanish");
}

use translatable::{Language, translation};

#[test]
fn test_both_dynamic() {
    let lang = Language::EN;
    let path = vec!["common", "greeting"];

    let result = translation!(lang, path, name = "Everyone");
    assert_eq!(result.unwrap(), "Hi Everyone!");
}

#[test]
fn test_dynamic_lang_static_path() {
    let lang = Language::ES;
    let result = translation!(lang, static common::greeting, name = "Amigo");
    assert_eq!(result.unwrap(), "¡Hola Amigo!");
}

#[test]
fn test_static_lang_dynamic_path() {
    let path = vec!["common", "greeting"];
    let result = translation!("en", path, name = "Friend");
    assert_eq!(result.unwrap(), "Hi Friend!");
}

#[test]
fn test_template_replacements() {
    let result = translation!("es", static common::greeting, name = "Carlos");
    assert_eq!(result, "¡Hola Carlos!");
}

#[test]
fn test_static_lang_static_path() {
    let result = translation!("en", static common::greeting, name = "Rustacean");
    assert_eq!(result, "Hi Rustacean!");
}

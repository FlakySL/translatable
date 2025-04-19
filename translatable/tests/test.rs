use translatable::{Language, translation};

const NAME: &str = "John";
const SURNAME: &str = "Doe";
const RESULT: &str = "¡Hola John Doe! Mi nombre es John Doe {{hola}}";

#[test]
fn both_static() {
    let result = translation!("es", static common::greeting, name = NAME, surname = SURNAME);
    assert!(result == RESULT);
}

#[test]
fn language_static_path_dynamic() {
    let result = translation!("es", vec!["common", "greeting"], name = NAME, surname = SURNAME);
    assert!(result.unwrap() == RESULT);
}

#[test]
fn language_dynamic_path_static() {
    let name = NAME;
    let surname = SURNAME;

    let result = translation!(Language::ES, static common::greeting, name, surname);
    assert!(result.unwrap() == RESULT);
}

#[test]
fn both_dynamic() {
    let result =
        translation!(Language::ES, vec!["common", "greeting"], name = NAME, surname = SURNAME);
    assert!(result.unwrap() == RESULT)
}

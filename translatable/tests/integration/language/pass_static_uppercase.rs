use translatable::translation;

#[cfg(test)]
#[test]
fn pass_static_uppercase() {
    let translation = translation!("ES", static greetings::formal);

    assert_eq!(translation, "Bueno conocerte.");
}

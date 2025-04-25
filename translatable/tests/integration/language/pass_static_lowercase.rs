use translatable::translation;

#[cfg(test)]
#[test]
fn pass_static_lowercase() {
    let translation = translation!("es", static greetings::formal);

    assert_eq!(translation, "Bueno conocerte.");
}

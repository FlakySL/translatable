use translatable::translation;

#[cfg(test)]
#[test]
pub fn pass_dynamic_expr() {
    let translation = translation!(
        "es".parse().expect("Expected language parsing to be OK"),
        static greetings::formal
    )
        .expect("Expected translation generation to be OK");

    assert_eq!(translation, "Bueno conocerte.");
}

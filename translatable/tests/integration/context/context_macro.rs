use std::collections::HashMap;

use translatable::{Language, translation_context};

#[translation_context(base_path = greetings, fallback_language = "es")]
pub struct TestContext {
    #[path(formal)]
    pub formal: String,
    informal: String,
}

#[test]
fn test() {
    let translations = TestContext::load_translations(
        Language::AA,
        &HashMap::from([
            ("user", "John")
        ])
    );

    assert_eq!(translations.informal, "Hey John, todo bien?");
    assert_eq!(translations.formal, "Bueno conocerte.")
}

use std::collections::HashMap;

use translatable::{Language, translation_context};

#[translation_context(greetings)]
pub struct TestContext {
    #[path(formal)]
    pub formal: String,
    informal: String,
}

#[test]
fn test() {
    let translations = TestContext::load_translations(
        Language::ES,
        &HashMap::from([
            ("user", "John")
        ])
    )
        .expect("Translations should be able to load");

    assert_eq!(translations.informal, "Hey John, todo bien?");
    assert_eq!(translations.formal, "Bueno conocerte.")
}

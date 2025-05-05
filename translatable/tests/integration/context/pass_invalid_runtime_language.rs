#![allow(dead_code)]

#[allow(unused_imports)] // trybuild
use ::{
    std::collections::HashMap,
    translatable::translation_context
};

#[translation_context(base_path = greetings)]
struct Context {
    formal: String,
    informal: String
}

#[test]
fn pass_invalid_runtime_language() {
    let translations = Context::load_translations(
        translatable::Language::AA,
        &HashMap::<String, String>::new()
    );

    assert!(translations.is_err());
}

#[allow(unused)]
fn main() {} // trybuild

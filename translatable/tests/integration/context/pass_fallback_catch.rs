#[allow(unused_imports)] // trybuild
use ::{std::collections::HashMap, translatable::TranslationContext};

#[derive(TranslationContext)]
#[base_path(greetings)]
struct Context {
    formal: String,
    informal: String,
}

#[test]
fn pass_fallback_catch() {
    let translations =
        Context::load_translations(translatable::Language::AA, &HashMap::from([("user", "John")]));

    assert_eq!(translations.formal, "Nice to meet you.");
    assert_eq!(translations.informal, "What's good John?");
}

#[allow(unused)]
fn main() {} // trybuild

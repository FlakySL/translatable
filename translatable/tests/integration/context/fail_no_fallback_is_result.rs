use std::collections::HashMap;
use translatable::{translation_context, Language};

#[translation_context(base_path = greetings)]
struct Context {
    formal: String,
    informal: String
}

fn main() {
    let ctx = Context::load_translations(Language::ES, &HashMap::new());
    assert!(ctx.formal); // invalid call
}

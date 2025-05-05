use translatable::{translation_context, Language};

#[translation_context(base_path = greetings, fallback_language = "en")]
struct Context {
    formal: String,
    informal: String
}

fn main() {
    let ctx = Context::load_translations(Language::ES, &HashMap::new());
    assert!(ctx.is_ok()); // invalid call
}

use translatable::translation_context;

#[translation_context(base_path = greetings, fallback_language = "invalid")]
struct Context {
    formal: String,
    informal: String
}

fn main() {}

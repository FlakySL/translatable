use translatable::translation_context;

#[translation_context(base_path = hello)]
struct Context {
    formal: String,
    informal: String
}

fn main() {}

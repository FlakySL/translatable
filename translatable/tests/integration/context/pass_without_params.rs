#[allow(unused_imports)] // trybuild
use translatable::translation_context;

#[translation_context(base_path = greetings)]
struct Context {
    formal: String,
    informal: String,
}

#[test]
fn pass_without_params() {}

#[allow(unused)]
fn main() {} // trybuild

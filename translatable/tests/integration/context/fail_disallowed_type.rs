#[allow(unused_imports)] // trybuild
use ::{std::collections::HashMap, translatable::translation_context};

#[translation_context(base_path = greetings)]
struct Context {
    formal: i32,
    informal: String,
}

#[allow(unused)]
fn main() {} // trybuild


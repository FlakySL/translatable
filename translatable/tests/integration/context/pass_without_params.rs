#[allow(unused_imports)] // trybuild
use translatable::translation_context;

#[translation_context]
struct Context {
    #[path(greetings::formal)]
    formal: String,
    #[path(greetings::informal)]
    informal: String,
}

#[test]
fn pass_without_params() {}

#[allow(unused)]
fn main() {} // trybuild

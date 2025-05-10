#[allow(unused_imports)] // trybuild
use translatable::translation;

#[cfg(test)]
#[test]
pub fn pass_dynamic_expr() {
    let translation = translation!(
        "es",
        "greetings.formal"
            .split(".")
    )
    .expect("Expected translation generation to be OK");

    assert_eq!(translation, "Bueno conocerte.");
}

#[allow(dead_code)]
fn main() {} // trybuild

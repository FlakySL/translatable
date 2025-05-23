#[allow(unused_imports)] // trybuild
use translatable::{Language, translation};

#[cfg(test)]
#[test]
pub fn pass_dynamic_enum() {
    let translation = translation!(Language::ES, static greetings::formal)
        .expect("Expected translation generation to be OK");

    assert_eq!(translation, "Bueno conocerte.");
}

#[allow(dead_code)]
fn main() {} // trybuild

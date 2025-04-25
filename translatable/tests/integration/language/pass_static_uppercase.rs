#[allow(unused_imports)] // trybuild
use translatable::translation;

#[cfg(test)]
#[test]
pub fn pass_static_uppercase() {
    let translation = translation!("ES", static greetings::formal);

    assert_eq!(translation, "Bueno conocerte.");
}

#[allow(dead_code)]
fn main() {} // trybuild

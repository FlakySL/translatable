#[allow(unused_imports)] // trybuild
use translatable::translation;

#[cfg(test)]
#[test]
pub fn pass_static_existing() {
    let translation = translation!("es", static greetings::formal);

    assert_eq!(translation, "Bueno conocerte.");
}

#[allow(dead_code)]
fn main() {} // trybuild

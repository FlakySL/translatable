#[allow(unused_imports)] // trybuild
use translatable::translation;

#[cfg(test)]
#[test]
pub fn pass_dynamic_expr() {
    let user = "Juan";

    let translation = translation!("es", static greetings::informal, user);

    assert_eq!(translation, "Hey Juan, todo bien?");
}

#[allow(dead_code)]
fn main() {} // trybuild

#[allow(unused_imports)] // trybuild
use translatable::{Language, translation};

#[cfg(test)]
#[test]
pub fn pass_dynamic_invalid_runtime() {
    let language = "invalid".parse::<Language>();

    assert!(language.is_err());
}

#[allow(dead_code)]
fn main() {} // trybuild

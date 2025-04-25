use translatable::{translation, Language};

#[cfg(test)]
#[test]
pub fn pass_dynamic_invalid_runtime() {
    let language = "invalid".parse::<Language>();

    assert!(language.is_err());

    translation!(language.unwrap(), static greetings::formal).ok();
}

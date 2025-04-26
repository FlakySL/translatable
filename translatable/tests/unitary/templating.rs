use std::collections::HashMap;
use std::str::FromStr;

use translatable_shared::misc::templating::FormatString;

#[test]
pub fn does_not_replace_not_found() {
    let result = FormatString::from_str("Hello {name}")
        .expect("Format string to be valid.")
        .replace_with(HashMap::new());

    assert_eq!(result, "Hello {name}");
}

#[test]
pub fn replaces_single_template() {
    let result = FormatString::from_str("Hello {name}")
        .expect("Format string to be valid.")
        .replace_with(HashMap::from([("name".into(), "Josh".into())]));

    assert_eq!(result, "Hello Josh");
}

#[test]
pub fn replaces_multiple_templates() {
    let result = FormatString::from_str("Hello {name} how are you doing {day}?")
        .expect("Format string to be valid.")
        .replace_with(HashMap::from([
            ("name".into(), "Josh".into()),
            ("day".into(), "today".into()),
        ]));

    assert_eq!(result, "Hello Josh how are you doing today?");
}

#[test]
pub fn replaces_mix_found_not_found() {
    let result = FormatString::from_str("Hello {name} how are you doing {day}?")
        .expect("Format string to be valid.")
        .replace_with(HashMap::from([("name".into(), "Josh".into())]));

    assert_eq!(result, "Hello Josh how are you doing {day}?");
}

#[test]
pub fn fails_unclosed_template() {
    let result = FormatString::from_str("Hello {");

    assert!(result.is_err());
}

#[test]
pub fn escapes_templates() {
    let result = FormatString::from_str("You write escaped templates like {{ this }}.")
        .expect("Format string to be valid.")
        .replace_with(HashMap::from([("this".into(), "not replaced".into())]));

    assert_eq!(result, "You write escaped templates like {{ this }}.")
}

use translatable::{Language, translation};

fn main() {
    let lang = Language::EN;
    let path = vec!["common", "greeting"];
    let name = "Juan";

    let result = translation!(lang, path, name);
    assert_eq!(result.unwrap(), "Hi Juan!");
}

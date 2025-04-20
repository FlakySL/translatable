use translatable::{Language, translation};

fn main() {
    let lang = Language::EN;
    let path = vec!["common", "greeting"];

    let res = translation!(lang, path);
    assert_eq!(res.unwrap(), "Hi {name}!");
}

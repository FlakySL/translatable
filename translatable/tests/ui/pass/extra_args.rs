use translatable::{Language, translation};

fn main() {
    let lang = Language::EN;
    let path = vec!["common", "greeting"];

    let res = translation!(lang, path, name = "Juan");
    assert_eq!(res.unwrap(), "Hi Juan!");
}

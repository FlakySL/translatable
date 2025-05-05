use std::collections::HashMap;

use toml::Table;
use translatable::Language;
use translatable_shared::translations::collection::TranslationNodeCollection;
use translatable_shared::translations::node::TranslationNode;

const FILE_1: &str = r#"
[greetings.formal]
es = "Hola"
en = "Hello"
"#;

const FILE_2: &str = r#"
[greetings.informal]
es = "Que haces?"
en = "Wyd?"
"#;

#[test]
pub fn loads_and_finds_collection() {
    let collection = TranslationNodeCollection::new(HashMap::from([
        (
            "a".into(),
            TranslationNode::try_from(
                FILE_1
                    .parse::<Table>()
                    .expect("TOML to be parsed correctly."),
            )
            .expect("TOML to follow the translation rules."),
        ),
        (
            "b".into(),
            TranslationNode::try_from(
                FILE_2
                    .parse::<Table>()
                    .expect("TOML to be parsed correctly."),
            )
            .expect("TOML to follow the translation rules."),
        ),
    ]));

    let translation = collection
        .find_path(
            &"greetings.formal"
                .split(".")
                .collect(),
        )
        .expect("Translation to be found.")
        .get(&Language::ES)
        .expect("Language to be available.")
        .replace_with(&HashMap::new());

    assert_eq!(translation, "Hola");
}

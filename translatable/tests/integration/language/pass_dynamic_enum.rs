use translatable::{translation, Language};

#[cfg(test)]
#[test]
fn pass_dynamic_enum() {
    translation!(Language::ES, static greetings::informal);
}


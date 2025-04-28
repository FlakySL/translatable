use translatable::{Language, translation_context};

#[translation_context(base::path)]
pub struct TestContext {
    pub xd: path::to::translation,
    lol: path::to::other_translation,
}

#[test]
fn test() {
    TestContext::lol(&Language::ES);
}

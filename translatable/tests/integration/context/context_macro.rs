use translatable::{translation_context, Language};

#[translation_context(base::path)]
pub struct TestContext {
    pub xd: path::to::translation,
    lol: path::to::other_translation,
}

#[test]
fn test() {
    TestContext::lol(&Language::ES);
}

use translatable::translation_context;

#[translation_context(base::path)]
pub struct TestContext {
    pub xd: path::to::translation,
    lol: path::to::other_translation,
}

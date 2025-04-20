use translatable::translation;

fn main() {
    // "zz" is not a valid ISO 639-1 language code
    let _ = translation!("zz", static common::greeting, name = "Alice");
}

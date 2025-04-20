use translatable::translation;

fn main() {
    let _ = translation!("en", static common::greeting, name = "Alice");
}

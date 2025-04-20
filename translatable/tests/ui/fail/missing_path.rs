use translatable::translation;

fn main() {
    // The path 'foo::bar' does not exist in TOML
    let _ = translation!("en", static foo::bar, name = "X");
}

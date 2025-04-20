use translatable::translation;

fn main() {
    let _ = translation!("en", static foo::Bar<T>, name = "X");
}

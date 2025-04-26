#[allow(unused_imports)]
use translatable::translation;

fn main() {
    translation!("es", static non::existing::path);
}

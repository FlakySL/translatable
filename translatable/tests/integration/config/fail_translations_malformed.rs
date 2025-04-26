// the macro isn't filled because the expected
// failure is on configuration.

#[allow(unused_imports)]
use translatable::{translation, Language};

fn main() {
    let _ = translation!(Language::ES, vec![""]);
}

#[allow(unused_imports)]
use translatable::translation;

struct NotDisplay;

fn main() {
    translation!("es", static greetings::informal, user = NotDisplay);
}

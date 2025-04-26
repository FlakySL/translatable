#[allow(unused_imports)]
use translatable::translation;

fn main() {
    translation!("es", static greetings::informal, %%$invalid = $ident);
}

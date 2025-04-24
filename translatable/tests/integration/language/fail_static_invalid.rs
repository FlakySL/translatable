use translatable::translation;

fn fail_static_invalid() {
    translation!("xx", static greetings::formal);
}

use translatable::translation;

fn fail_static_nonexistent() {
    translation!("es", static non::existing::path);
}

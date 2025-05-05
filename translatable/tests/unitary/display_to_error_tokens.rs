use translatable_shared::macros::errors::IntoCompileError;

#[test]
pub fn display_to_error_tokens() {
    let display = "test".to_string();

    let to_out_compile_error = display
        .to_out_compile_error()
        .to_string()
        .replace(" ", ""); // normalize

    assert!(to_out_compile_error.contains("fn"));
    assert!(to_out_compile_error.contains("__()"));
    assert!(to_out_compile_error.contains("std::compile_error!"));
}

use translatable_shared::inline_quote;
use quote::quote;

#[test]
pub fn interpolate_variable() {
    let a = 5;
    let result = inline_quote!(#a).to_string();

    assert_eq!(result, quote! {5i32}.to_string());
}

#[test]
pub fn evaluate_expression() {
    let result = inline_quote!(#{3 + 5 * 8}).to_string();

    assert_eq!(result, quote! {43i32}.to_string());
}

#[test]
pub fn interpolate_variable_inside_delimiters() {
    let x = 23;
    let parentheses = inline_quote!((#x)).to_string();
    let braces = inline_quote!([#x]).to_string();
    let brackets = inline_quote!({#x}).to_string();

    assert_eq!(parentheses, quote! {(23i32)}.to_string());
    assert_eq!(braces, quote! {[23i32]}.to_string());
    assert_eq!(brackets, quote! {{23i32}}.to_string())
}

#[test]
pub fn evaluate_expression_inside_delimiters() {
    let parentheses = inline_quote!((#{9 + 3})).to_string();
    let braces = inline_quote!([#{9 + 3}]).to_string();
    let brackets = inline_quote!({#{9 + 3}}).to_string();

    assert_eq!(parentheses, quote! {(12i32)}.to_string());
    assert_eq!(braces, quote! {[12i32]}.to_string());
    assert_eq!(brackets, quote! {{12i32}}.to_string());
}

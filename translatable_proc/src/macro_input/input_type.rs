use proc_macro2::TokenStream as TokenStream2;

/// This enum abstracts (in the programming sense)
/// the logic on separating between what's considered
/// dynamic and static while parsing the abstract
/// (in the conceptual sense) macro input.
pub enum InputType<T: Sized> {
    Static(T),
    Dynamic(TokenStream2),
}

impl<T: Into<TokenStream2>> InputType<T> {
    /// This method allows converting the
    /// enum value whether it's conceptually
    /// dynamic or static into its dynamic
    /// represented as a `TokenStream`
    #[inline]
    #[allow(unused)]
    fn dynamic(self) -> TokenStream2 {
        match self {
            Self::Static(value) => value.into(),
            Self::Dynamic(value) => value,
        }
    }
}

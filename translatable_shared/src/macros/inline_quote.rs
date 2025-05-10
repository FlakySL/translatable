//! [`inline_quote`] macro declaration.
//!
//! This module declares a macro to inline

///
#[macro_export]
macro_rules! inline_quote {
    ($($t:tt)*) => {{
        #[doc(hidden)]
        let mut tokens = quote::quote!();
        $crate::__inline_quote!(tokens => $($t)*);
        tokens
    }};
}

///
#[macro_export]
macro_rules! __inline_quote {
    ( $tokens:ident => #{ $e:expr } $($rest:tt)* ) => {{
        let __inline_tmp = $e;
        $tokens.extend(quote::quote! { #__inline_tmp });
        $crate::__inline_quote!($tokens => $($rest)*);
    }};

    ( $tokens:ident => $tt:tt $($rest:tt)* ) => {{
        $tokens.extend(quote::quote! { $tt });
        $crate::__inline_quote!($tokens => $($rest)*);
    }};

    ( $tokens:ident => ) => {};
}

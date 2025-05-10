//! [`inline_quote`] macro declaration.
//!
//! This module declares a macro to a quote version
//! that supports expansion of advanced expressions
//! with `#{}`.
//!
//! See https://github.com/dtolnay/quote/pull/296.

/// [`inline_quote`] macro.
///
/// A [`quote`] macro wrapper to support
/// expansion of advanced expressions with
/// `#{}`.
///
/// The invocation works the same as quote,
/// it is in fact backwards compatible if
/// the `#{}` templates are removed.
#[macro_export]
#[clippy::format_args]
macro_rules! inline_quote {
    ($($t:tt)*) => {{
        #[doc(hidden)]
        let mut tokens = quote::quote!();
        $crate::__inline_quote!(tokens => $($t)*);
        tokens
    }};
}

// inline_quote! dispatch macro.
#[doc(hidden)]
#[macro_export]
#[clippy::format_args]
macro_rules! __inline_quote {
    // template dispatch branch, if #{} found evaluate
    // and extend.
    ( $tokens:ident => #{ $e:expr } $($rest:tt)* ) => {{
        stringify!($e);
        let __inline_tmp = $e;
        $tokens.extend(quote::quote! { #__inline_tmp });
        $crate::__inline_quote!($tokens => $($rest)*);
    }};	

    ( $tokens:ident => { $($all:tt)* } $($rest:tt)* ) => {{
        let __inline_tmp = $crate::inline_quote!($($all)*);
        $tokens.extend(quote::quote! { { #__inline_tmp  } });
        $crate::__inline_quote!($tokens => $($rest)*);
    }};

    ( $tokens:ident => ( $($all:tt)* ) $($rest:tt)* ) => {{
        let __inline_tmp = $crate::inline_quote!($($all)*);
        $tokens.extend(quote::quote! { ( #__inline_tmp ) });
        $crate::__inline_quote!($tokens => $($rest)*);
    }};

    ( $tokens:ident => [ $($all:tt)* ] $($rest:tt)* ) => {{
        let __inline_tmp = $crate::inline_quote!($($all)*);
        $tokens.extend(quote::quote! { [ #__inline_tmp ] });
        $crate::__inline_quote!($tokens => $($rest)*);
    }};

    // any token dispatch branch, if something else is
    // found extend the existing source.
    ( $tokens:ident => $tt:tt $($rest:tt)* ) => {{
        $tokens.extend(quote::quote! { $tt });
        $crate::__inline_quote!($tokens => $($rest)*);
    }};

    // finsh loop
    ( $tokens:ident => ) => {};
}

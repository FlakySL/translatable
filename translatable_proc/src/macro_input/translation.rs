use std::collections::HashMap;

use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::token::Static;
use syn::{
    Error as SynError,
    Expr,
    ExprLit,
    Ident,
    Lit,
    Path,
    PathArguments,
    Result as SynResult,
    Token,
};
use thiserror::Error;
use translatable_shared::misc::language::Language;

use super::input_type::InputType;

/// Used to represent errors on parsing a `TranslationMacroArgs`
/// using `parse_macro_input!`.
///
/// The enum implements a helper function to convert itself
/// to a `syn` spanned error, so this enum isn't directly
/// exposed as the `syn::Error` instance takes place.
#[derive(Error, Debug)]
enum TranslationMacroArgsError {
    /// An error while parsing a compile-time String value
    /// was found.
    #[error("The literal '{0}' is an invalid ISO 639-1 string, and cannot be parsed")]
    InvalidIsoLiteral(String),

    /// Extra tokens were found while parsing a static path for
    /// the `translation!` macro, specifically generic arguments.
    #[error("This translation path contains generic arguments, and cannot be parsed")]
    InvalidPathContainsGenerics,
}

/// The `TranslationMacroArgs` struct is used to represent
/// the `translation!` macro parsed arguments, it's sole
/// purpose is to be used with `parse_macro_input!` with the
/// `Parse` implementation the structure has.
pub struct TranslationMacroArgs {
    /// Represents the user specified language
    /// which may be static if the specified language
    /// is a string literal or a `Language` enum tagged
    /// union instance, otherwise dynamic and represented
    /// as a `TokenStream`.
    language: InputType<Language>,

    /// Represents a toml path to find the translation
    /// object in the previously parsed TOML from the
    /// translation files, this can be static if specified
    /// as `static path::to::translation` or dynamic if
    /// it's another expression, this way represented as a
    /// `TokenStream`.
    path: InputType<Vec<String>>,

    /// Stores the replacement arguments for the translation
    /// templates such as `Hello {name}` if found on a translation.
    ///
    /// If a call such as `a` is found, it will be implicitly
    /// converted to `a = a` thus stored like so in the hash map.
    replacements: HashMap<Ident, TokenStream2>,
}

impl TranslationMacroArgsError {
    pub fn into_syn_error<T: ToTokens>(self, span: T) -> SynError {
        SynError::new_spanned(span, self.to_string())
    }
}

/// The implementation is used to achieve the
/// sole purpose this structure has, which is being
/// used in a `parse_macro_input!` call.
impl Parse for TranslationMacroArgs {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let parsed_language_arg = match input.parse::<Expr>()? {
            Expr::Lit(ExprLit { lit: Lit::Str(literal), .. }) => {
                match literal.value().parse::<Language>() {
                    Ok(language) => InputType::Static(language),

                    Err(_) => Err(TranslationMacroArgsError::InvalidIsoLiteral(literal.value())
                        .into_syn_error(literal))?,
                }
            },

            other => InputType::Dynamic(other.into_token_stream()),
        };

        input.parse::<Token![,]>()?;

        let parsed_path_arg = match input.parse::<Static>() {
            Ok(_) => {
                let language_arg = input
                    .parse::<Path>()?
                    .segments
                    .into_iter()
                    .map(|segment| match segment.arguments {
                        PathArguments::None => Ok(segment.ident.to_string()),

                        other => Err(TranslationMacroArgsError::InvalidPathContainsGenerics
                            .into_syn_error(other)),
                    })
                    .collect::<Result<Vec<_>, _>>()?;

                InputType::Static(language_arg)
            },

            Err(_) => InputType::Dynamic(input.parse::<Expr>()?.to_token_stream()),
        };

        let mut replacements = HashMap::new();
        if input.peek(Token![,]) {
            while !input.is_empty() {
                input.parse::<Token![,]>()?;
                let key = input.parse::<Ident>()?;
                let value = match input.parse::<Token![=]>() {
                    Ok(_) => input.parse::<Expr>()?.to_token_stream(),

                    Err(_) => key.clone().into_token_stream(),
                };

                replacements.insert(key, value);
            }
        }

        Ok(Self {
            language: parsed_language_arg,
            path: parsed_path_arg,
            replacements,
        })
    }
}

impl TranslationMacroArgs {
    #[inline]
    #[allow(unused)]
    pub fn language(&self) -> &InputType<Language> {
        &self.language
    }

    #[inline]
    #[allow(unused)]
    pub fn path(&self) -> &InputType<Vec<String>> {
        &self.path
    }

    #[inline]
    #[allow(unused)]
    pub fn replacements(&self) -> &HashMap<Ident, TokenStream2> {
        &self.replacements
    }
}

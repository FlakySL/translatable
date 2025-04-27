use proc_macro2::Span;
use syn::{braced, Ident, Path, PathArguments, Result as SynResult, Token};
use syn::parse::{Parse, ParseStream};
use thiserror::Error;
use translatable_shared::macros::errors::IntoCompileError;

#[derive(Error, Debug)]
enum ContextMacroArgsError {
    #[error("This translation path contains generic arguments, and cannot be parsed")]
    InvalidPathContainsGenerics,

    #[error("This macro must be applied on a struct")]
    NotAStruct
}

pub struct ContextMacroArgs {
    is_pub: bool,
    ident: String,
    fields: Vec<ContextMacroPathField>,
}

pub struct ContextMacroPathField {
    is_pub: bool,
    path: Vec<String>,
    ident: String,
}

impl Parse for ContextMacroPathField {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let mut is_pub = false;

        if input.peek(Token![pub]) {
            input.parse::<Token![pub]>()?;
            is_pub = true;
        }

        let ident = input.parse::<Ident>()?
            .to_string();

        input.parse::<Token![:]>()?;

        let path = input.parse::<Path>()?
            .segments
            .iter()
            .map(|segment| match &segment.arguments {
                PathArguments::None => Ok(segment.ident.to_string()),

                other => Err(ContextMacroArgsError::InvalidPathContainsGenerics
                    .to_syn_error(other)),
            })
            .collect::<Result<Vec<String>, _>>()?;

        Ok(Self { is_pub, path, ident })
    }
}

impl Parse for ContextMacroArgs {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let mut is_pub = false;

        if input.peek(Token![pub]) {
            input.parse::<Token![pub]>()?;
            is_pub = true;
        }

        if !input.peek(Token![struct]) {
            let dummy_ident = Ident::new("_", Span::call_site());
            return Err(
                ContextMacroArgsError::NotAStruct
                    .to_syn_error(dummy_ident)
            );
        }

        input.parse::<Token![struct]>()?;

        let ident = input.parse::<Ident>()?.to_string();

        let content;
        braced!(content in input);

        let fields = content
            .parse_terminated(ContextMacroPathField::parse, Token![,])?
            .into_iter()
            .collect::<Vec<_>>();

        Ok(Self { is_pub, ident, fields })
    }
}

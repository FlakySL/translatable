use std::str::FromStr;

use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::{parse2, Error as SynError, Expr, ExprLit, Field, Ident, ItemStruct, Lit, MetaNameValue, Result as SynResult, Token, Type, Visibility};
use thiserror::Error;
use translatable_shared::macros::errors::IntoCompileError;
use translatable_shared::misc::language::Language;

use super::utils::translation_path::TranslationPath;

#[derive(Error, Debug)]
enum MacroArgsError {
    #[error("Only named fields are allowed")]
    InvalidFieldType,

    #[error("Only a language literal is allowed")]
    OnlyLangLiteralAllowed,

    #[error("Invalid language literal '{0}' is not a valid ISO-639-1 language")]
    InvalidLanguageLiteral(String),

    #[error("Unknown key '{0}', allowed keys are 'fallback_language' and 'base_path'")]
    UnknownKey(String)
}

pub struct ContextMacroArgs{
    base_path: Option<TranslationPath>,
    fallback_language: Option<Language>
}

pub struct ContextMacroField {
    path: Option<TranslationPath>,
    pub_state: Visibility,
    ident: Ident,
    ty: Type,
}

pub struct ContextMacroStruct {
    pub_state: Visibility,
    ident: Ident,
    fields: Vec<ContextMacroField>,
}

impl ContextMacroArgs {
    #[inline]
    #[allow(unused)]
    pub fn base_path(&self) -> TranslationPath {
        self.base_path
            .clone()
            .unwrap_or_else(|| TranslationPath::default())
    }

    #[inline]
    #[allow(unused)]
    pub fn fallback_language(&self) -> Option<Language> {
        self.fallback_language
            .clone()
    }
}

impl Parse for ContextMacroArgs {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let values = input.parse_terminated(MetaNameValue::parse, Token![,])?;
        let mut base_path = None;
        let mut fallback_language = None;

        for kvp in values {
            let key = kvp.path
                .to_token_stream()
                .to_string();

            match key.as_str() {
                "base_path" => {
                    base_path = Some(
                        parse2::<TranslationPath>(kvp.value.to_token_stream())?
                    );
                }

                "fallback_language" => {
                    if let Expr::Lit(ExprLit { lit: Lit::Str(lit), .. }) = kvp.value {
                        fallback_language = Some(
                            Language::from_str(lit.value().as_str())
                                .map_err(|_|
                                    MacroArgsError::InvalidLanguageLiteral(lit.value())
                                        .to_syn_error(lit)
                                )?
                        );
                    } else {
                        return Err(
                            MacroArgsError::OnlyLangLiteralAllowed
                                .to_syn_error(kvp.value)
                        );
                    }
                }

                key => {
                    return Err(
                        MacroArgsError::UnknownKey(key.to_string())
                            .to_syn_error(kvp.path)
                    );
                }
            }
        }

        Ok(Self {
            base_path,
            fallback_language
        })
    }
}

impl ContextMacroField {
    #[inline]
    #[allow(unused)]
    pub fn path(&self) -> TranslationPath {
        self.path
            .clone()
            .unwrap_or_else(||
                TranslationPath::new(
                    vec![
                        self.ident
                            .to_string()
                    ],
                    self.ident
                        .span()
                )
            )
    }

    #[inline]
    #[allow(unused)]
    pub fn pub_state(&self) -> &Visibility {
        &self.pub_state
    }

    #[inline]
    #[allow(unused)]
    pub fn ident(&self) -> &Ident {
        &self.ident
    }

    #[inline]
    #[allow(unused)]
    pub fn ty(&self) -> &Type {
        &self.ty
    }
}

impl TryFrom<Field> for ContextMacroField {
    type Error = SynError;

    fn try_from(field: Field) -> Result<Self, Self::Error> {
        let path = field
            .attrs
            .iter()
            .find(|field| {
                field
                    .path()
                    .is_ident("path")
            })
            .map(|field| field.parse_args::<TranslationPath>())
            .transpose()?;

        let is_pub = field
            .vis
            .clone();

        let ident = field
            .ident
            .clone()
            .ok_or(MacroArgsError::InvalidFieldType.to_syn_error(&field))?;

        let ty = field.ty;

        Ok(Self { path, pub_state: is_pub, ident, ty })
    }
}

impl ContextMacroStruct {
    #[inline]
    #[allow(unused)]
    pub fn pub_state(&self) -> &Visibility {
        &self.pub_state
    }

    #[inline]
    #[allow(unused)]
    pub fn ident(&self) -> &Ident {
        &self.ident
    }

    #[inline]
    #[allow(unused)]
    pub fn fields(&self) -> &[ContextMacroField] {
        &self.fields
    }
}

impl Parse for ContextMacroStruct {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let structure = input.parse::<ItemStruct>()?;

        let is_pub = structure.vis;
        let ident = structure.ident;

        let fields = structure
            .fields
            .into_iter()
            .map(|field| ContextMacroField::try_from(field))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { pub_state: is_pub, ident, fields })
    }
}

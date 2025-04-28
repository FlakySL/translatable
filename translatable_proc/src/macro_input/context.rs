use syn::{parse::{Parse, ParseStream}, Field, Ident, ItemStruct, Result as SynResult, Type, Visibility, Error as SynError};
use thiserror::Error;
use translatable_shared::macros::errors::IntoCompileError;

use super::utils::translation_path::TranslationPath;

#[derive(Error, Debug)]
enum MacroArgsError {
    #[error("Only named fields are allowed")]
    InvalidFieldType
}

pub struct ContextMacroArgs(Option<TranslationPath>);

pub struct ContextMacroField {
    path: TranslationPath,
    is_pub: Visibility,
    ident: Ident,
    ty: Type
}

pub struct ContextMacroStruct {
    is_pub: Visibility,
    ident: Ident,
    fields: Vec<ContextMacroField>
}

impl Parse for ContextMacroArgs {
    fn parse(input: ParseStream) -> SynResult<Self> {
        Ok(Self(
            if !input.is_empty() {
                Some(input.parse::<TranslationPath>()?)
            } else {
                None
            }
        ))
    }
}

impl TryFrom<Field> for ContextMacroField {
    type Error = SynError;

    fn try_from(field: Field) -> Result<Self, Self::Error> {
        let path = field
            .attrs
            .iter()
            .find(|field| field.path().is_ident("path"))
            .map(|field| field.parse_args::<TranslationPath>())
            .transpose()?
            .unwrap_or_else(|| TranslationPath::default());

        let is_pub = field
            .vis
            .clone();

        let ident = field
            .ident
            .clone()
            .ok_or(
                MacroArgsError::InvalidFieldType
                    .to_syn_error(&field)
            )?;

        let ty = field
            .ty;

        Ok(Self {
            path,
            is_pub,
            ident,
            ty
        })
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

        Ok(Self {
            is_pub,
            ident,
            fields
        })
    }
}

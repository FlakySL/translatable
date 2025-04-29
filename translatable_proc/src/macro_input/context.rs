use syn::parse::{Parse, ParseStream};
use syn::{Error as SynError, Field, Ident, ItemStruct, Result as SynResult, Type, Visibility};
use thiserror::Error;
use translatable_shared::macros::errors::IntoCompileError;

use super::utils::translation_path::TranslationPath;

#[derive(Error, Debug)]
enum MacroArgsError {
    #[error("Only named fields are allowed")]
    InvalidFieldType,
}

pub struct ContextMacroArgs(Option<TranslationPath>);

pub struct ContextMacroField {
    path: TranslationPath,
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
    pub fn into_inner(self) -> Option<TranslationPath> {
        self.0
    }
}

impl Parse for ContextMacroArgs {
    fn parse(input: ParseStream) -> SynResult<Self> {
        Ok(Self(if !input.is_empty() { Some(input.parse::<TranslationPath>()?) } else { None }))
    }
}

impl ContextMacroField {
    #[inline]
    #[allow(unused)]
    pub fn path(&self) -> &TranslationPath {
        &self.path
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
            .transpose()?
            .unwrap_or_else(|| TranslationPath::default());

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

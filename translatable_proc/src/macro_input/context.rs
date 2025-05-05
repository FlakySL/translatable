//! [`#\[translation_context\]`] input parsing module.
//!
//! This module declares a structure that implements
//! [`Parse`] for it to be used with [`parse_macro_input`].
//!
//! [`#\[translation_context\]`]: crate::translation_context
//! [`parse_macro_input`]: syn::parse_macro_input

use std::str::FromStr;

use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt, quote};
use syn::parse::{Parse, ParseStream};
use syn::{
    Error as SynError,
    Expr,
    ExprLit,
    Field,
    Ident,
    ItemStruct,
    Lit,
    MetaNameValue,
    Result as SynResult,
    Token,
    Type,
    Visibility,
    parse2,
};
use thiserror::Error;
use translatable_shared::macros::errors::IntoCompileError;
use translatable_shared::misc::language::Language;

use super::utils::translation_path::TranslationPath;

/// Parse error for [`ContextMacroArgs`] and [`ContextMacroStruct`].
///
/// Represents errors that can occur while parsing the
/// [`#\[translation_context\]`] macro input. This error is only used while
/// parsing compile-time input, as runtime input is validated in runtime.
///
/// [`#\[translation_context\]`]: crate::translation_context
#[derive(Error, Debug)]
enum MacroArgsError {
    /// Invalid field type error.
    ///
    /// Usually from using an invalid struct type, such
    /// as tuple or unit.
    #[error("Only named fields are allowed")]
    InvalidFieldType,

    /// Invalid language parameter for fallback.
    ///
    /// Fallback only supports static language, same
    /// as the [`translation!()`] macro static language
    /// parameter.
    ///
    /// [`translation!()`]: crate::translation
    #[error("Only a language literal is allowed")]
    OnlyLangLiteralAllowed,

    /// Invalid ISO-639-1 language literal.
    ///
    /// Language literals must be ISO-639-1 compliant.
    ///
    /// **Parameters**
    /// * `0` - The invalid language literal.
    #[error("Invalid language literal '{0}' is not a valid ISO-639-1 language")]
    InvalidLanguageLiteral(String),

    /// Invalid macro parameter.
    ///
    /// **Parameters**
    /// * `0` - The unknown parameter key.
    #[error("Unknown key '{0}', allowed keys are 'fallback_language' and 'base_path'")]
    UnknownKey(String),
}

/// The arguments passed to the context macro.
///
/// These arguments are passed literally as a punctuated
/// [`MetaNameValue`] separated by `Token![,]`.
///
/// These act as configuration overrides for each context
/// struct.
pub struct ContextMacroArgs {
    /// Field base path.
    ///
    /// A base path to be prepended to all
    /// field paths.
    base_path: TranslationPath,

    /// Context fallback language.
    ///
    /// The fallback should be available
    /// in all the specified paths, removes
    /// the need to handle errors if a language
    /// is not available for a specific translation.
    fallback_language: Option<Language>,
}

/// A field inside a translation context struct.
///
/// Fields are parsed independently and moved
/// to a [`ContextMacroStruct`], this contains
/// data about how to load a translation.
pub struct ContextMacroField {
    /// The translation path.
    ///
    /// This path is appended to the
    /// path passed to the struct configuration.
    path: Option<TranslationPath>,

    /// The field visibility.
    ///
    /// This gets literally rendered as is.
    visibility: Visibility,

    /// The field name.
    ///
    /// This gets literally rendered as is.
    ident: Ident,

    /// The field type.
    ///
    /// Validated but rendered as is.
    ty: Type,
}

/// Translation context struct data.
///
/// This parses the struct necessary data
/// to re-generate it preparated to load
/// translations, loading [`ContextMacroField`]s
/// too.
pub struct ContextMacroStruct {
    /// The struct visibility.
    ///
    /// This gets literally rendered as is.
    visibility: Visibility,

    /// The struct name.
    ///
    /// This gets literally rendered as is.
    ident: Ident,

    /// The struct fields.
    ///
    /// Get rendered as specified in the
    /// [`ContextMacroField::to_tokens`] implementation.
    fields: Vec<ContextMacroField>,
}

impl ContextMacroArgs {
    /// Base path getter.
    ///
    /// **Returns**
    /// A reference to the `base_path`.
    #[inline]
    #[allow(unused)]
    pub fn base_path(&self) -> &TranslationPath {
        &self.base_path
    }

    /// Fallback language getter.
    ///
    /// **Returns**
    /// A reference o the `fallback_language`.
    #[inline]
    #[allow(unused)]
    pub fn fallback_language(&self) -> Option<Language> {
        self.fallback_language
            .clone()
    }
}

/// [`Parse`] implementation for [`ContextMacroArgs`].
///
/// This implementation is to be used within [`parse_macro_input!()`]
/// and parses the macro arguments to modify the macro behavior.
///
/// [`parse_macro_input!()`]: syn::parse_macro_input
impl Parse for ContextMacroArgs {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let values = input.parse_terminated(MetaNameValue::parse, Token![,])?;
        let mut base_path = None;
        let mut fallback_language = None;

        for kvp in values {
            let key = kvp
                .path
                .to_token_stream()
                .to_string();

            match key.as_str() {
                "base_path" => {
                    base_path = Some(parse2::<TranslationPath>(
                        kvp.value
                            .to_token_stream(),
                    )?);
                },

                "fallback_language" => {
                    if let Expr::Lit(ExprLit { lit: Lit::Str(lit), .. }) = kvp.value {
                        fallback_language = Some(
                            Language::from_str(
                                lit.value()
                                    .as_str(),
                            )
                            .map_err(|_| {
                                MacroArgsError::InvalidLanguageLiteral(lit.value())
                                    .to_syn_error(lit)
                            })?,
                        );
                    } else {
                        return Err(MacroArgsError::OnlyLangLiteralAllowed.to_syn_error(kvp.value));
                    }
                },

                key => {
                    return Err(MacroArgsError::UnknownKey(key.to_string()).to_syn_error(kvp.path));
                },
            }
        }

        let base_path = base_path.unwrap_or_else(|| TranslationPath::default());

        Ok(Self { base_path, fallback_language })
    }
}

impl ContextMacroField {
    /// Path getter.
    ///
    /// The path specified in the attribute
    /// otherwise a path with a single segment
    /// as the attribute ident. Alternative lazily
    /// evaluated.
    ///
    /// **Returns**
    /// The corresponding translation path for the field.
    #[inline]
    #[allow(unused)]
    pub fn path(&self) -> TranslationPath {
        self.path
            .clone()
            .unwrap_or_else(|| {
                TranslationPath::new(
                    vec![
                        self.ident
                            .to_string(),
                    ],
                    self.ident
                        .span(),
                )
            })
    }

    /// Visibility getter.
    ///
    /// **Returns**
    /// A reference to this field's visibility.
    #[inline]
    #[allow(unused)]
    pub fn visibility(&self) -> &Visibility {
        &self.visibility
    }

    /// Identifier getter.
    ///
    /// **Returns**
    /// A reference to this field's identifier.
    #[inline]
    #[allow(unused)]
    pub fn ident(&self) -> &Ident {
        &self.ident
    }

    /// Type getter.
    ///
    /// **Returns**
    /// A reference to this field's type.
    #[inline]
    #[allow(unused)]
    pub fn ty(&self) -> &Type {
        &self.ty
    }
}

/// [`ToTokens`] implementation for [`ContextMacroField`].
///
/// This implementation is used to convert the
/// data stored in this struct to the tokens
/// it represnets.
impl ToTokens for ContextMacroField {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let visibility = self.visibility();
        let ident = self.ident();
        let ty = self.ty();

        tokens.append_all(quote! {
            #visibility #ident: #ty
        });
    }
}

/// [`TryFrom<Field>`] implementation for [`ContextMacroField`].
///
/// This implementation is used to parse
/// the custom metadata from a struct field.
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

        Ok(Self { path, visibility: is_pub, ident, ty })
    }
}

impl ContextMacroStruct {
    /// Visibility getter.
    ///
    /// **Returns**
    /// A reference to this struct's visibility.
    #[inline]
    #[allow(unused)]
    pub fn visibility(&self) -> &Visibility {
        &self.visibility
    }

    /// Identifier getter.
    ///
    /// **Returns**
    /// A reference o this idenitifer visibility.
    #[inline]
    #[allow(unused)]
    pub fn ident(&self) -> &Ident {
        &self.ident
    }

    /// Fields getter.
    ///
    /// **Returns**
    /// A slice to all the fields in this struct.
    #[inline]
    #[allow(unused)]
    pub fn fields(&self) -> &[ContextMacroField] {
        &self.fields
    }
}

/// [`Parse`] implementation for [`ContextMacroStruct`].
///
/// This implementation is used to parse the struct
/// trough [`parse_macro_input!()`].
///
/// [`parse_macro_input!()`]: syn::parse_macro_input
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

        Ok(Self { visibility: is_pub, ident, fields })
    }
}

use syn::{parse::{Parse, ParseStream}, Error as SynError, Ident, ItemStruct, Result as SynResult, Type};

use super::utils::translation_path::TranslationPath;

#[derive(Debug)]
pub struct ContextMacroField {
    base_path: Option<TranslationPath>,
    path: Option<TranslationPath>,
    name: Ident
}

pub struct ContextMacroInput{
    ident: Ident,
    fields: Vec<ContextMacroField>
}


impl ContextMacroField {
    pub fn path(&self) -> TranslationPath {
        let field_path = self.path
            .clone()
            .unwrap_or_else(|| TranslationPath::new(
                vec![self.name.to_string()],
                self.name.span(),
                false
            ));

        self.base_path
            .clone()
            .map(|base_path| base_path.merge(&field_path))
            .unwrap_or(field_path)
    }

    pub fn name(&self) -> &Ident {
        &self.name
    }
}

impl ContextMacroInput {
    pub fn ident(&self) -> &Ident {
        &self.ident
    }

    pub fn fields(&self) -> &[ContextMacroField] {
        &self.fields
    }
}

impl Parse for ContextMacroInput {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let input_struct = input.parse::<ItemStruct>()?;

        let base_path = input_struct
            .attrs
            .iter()
            .find(|attr| attr.path().is_ident("base_path"))
            .map(|attr| attr.parse_args::<TranslationPath>())
            .transpose()?;

        let (fields, errors): (Vec<_>, Vec<_>) = input_struct
            .fields
            .iter()
            .map(|field| -> Result<ContextMacroField, SynError> {
                match &field.ty {
                    Type::Path(p) if p.path.is_ident("String") => {}
                    Type::Infer(_) => {},
                    _ => Err(SynError::new_spanned(field, "The type must be a String or inferred as '_'."))?
                }

                let field_path = field
                    .attrs
                    .iter()
                    .find(|attr| attr.path().is_ident("path"))
                    .map(|attr| attr.parse_args::<TranslationPath>())
                    .transpose()?;

                let field_name = field
                    .ident
                    .as_ref()
                    .cloned()
                    .ok_or_else(|| SynError::new_spanned(&field, "The field must be named."))?;

                Ok(ContextMacroField {
                    base_path: base_path.clone(),
                    path: field_path,
                    name: field_name,
                })
        })
        .partition::<Vec<_>, _>(Result::is_ok);

        let fields = fields.into_iter().collect::<Result<Vec<_>, _>>()?;
        let errors = errors.into_iter()
            .map(Result::unwrap_err)
            .reduce(|mut acc, err| {
                acc.combine(err);
                acc
            });

        if let Some(errors) = errors {
            Err(errors)
        } else {
            Ok(Self { ident: input_struct.ident, fields })
        }
    }
}

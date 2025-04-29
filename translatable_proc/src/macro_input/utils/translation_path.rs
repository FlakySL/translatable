use proc_macro2::Span;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{Error as SynError, Path, PathArguments, Result as SynResult};

#[derive(Clone)]
pub struct TranslationPath {
    segments: Vec<String>,
    span: Span,
}

impl Parse for TranslationPath {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let path = input.parse::<Path>()?;

        let span = path.span();
        let segments = path
            .segments
            .into_iter()
            .map(|segment| match segment.arguments {
                PathArguments::None => Ok(segment
                    .ident
                    .to_string()),

                error => Err(SynError::new_spanned(
                    error,
                    "A translation path can't contain generic arguments.",
                )),
            })
            .collect::<Result<_, _>>()?;

        Ok(Self { segments, span })
    }
}

impl Default for TranslationPath {
    fn default() -> Self {
        Self {
            segments: Vec::new(),
            span: Span::call_site(),
        }
    }
}

impl TranslationPath {
    pub fn new(segments: Vec<String>, span: Span) -> Self {
        Self { segments, span }
    }

    // TODO: merge spans (not yet in #19)
    pub fn merge(&self, other: &TranslationPath) -> Vec<String> {
        [
            self.segments().to_vec(),
            other.segments().to_vec()
        ]
            .concat()
    }

    #[inline]
    #[allow(unused)]
    pub fn segments(&self) -> &Vec<String> {
        &self.segments
    }

    #[inline]
    #[allow(unused)]
    pub fn span(&self) -> Span {
        self.span
    }
}

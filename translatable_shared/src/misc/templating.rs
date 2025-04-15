use std::collections::HashMap;
use std::ops::Range;
use std::str::FromStr;

use syn::{Ident, parse_str};
use thiserror::Error;
use unicode_xid::UnicodeXID;

#[derive(Error, Debug)]
pub enum TemplateError {
    // Runtime errors
    #[error("Found unclosed brace at index {0}")]
    Unclosed(usize),

    #[error("Found template with key '{0}' which is an invalid identifier")]
    InvalidIdent(String),

    // Compile errors
    #[error("Found template with a non compliant XID key, found invalid start character '{0}'")]
    InvalidxIdStart(char),

    #[error("Found template with a non compliant XID key, found invalid rest character '{0}'")]
    InvalidxIdRest(char),
}

pub struct FormatString {
    original: String,
    spans: HashMap<String, Range<usize>>,
}

impl FormatString {
    pub fn replace_with(mut self, values: HashMap<String, String>) -> String {
        let mut offset = 0isize;

        for (key, range) in self.spans {
            if let Some(value) = values.get(&key) {
                let start = (range.start as isize + offset) as usize;
                let end = (range.end as isize + offset) as usize;

                self.original.replace_range(start..end, value);

                offset += value.len() as isize - (range.end - range.start) as isize;
            }
        }

        self.original
    }
}

impl FromStr for FormatString {
    type Err = TemplateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let original = s.to_string();
        let mut spans = HashMap::new();

        let mut last_bracket_idx = 0usize;
        let mut current_tmpl_key = String::new();
        for (i, c) in original.chars().enumerate() {
            match (c, last_bracket_idx) {
                // if last template index is anything but the last character
                // set it as last index.
                ('{', lbi) if lbi != i.saturating_sub(1) => last_bracket_idx = i,
                // if last template index is the last character
                // ignore current as is escaped.
                ('{', lbi) if lbi == i.saturating_sub(1) => last_bracket_idx = 0,

                // if last template index is not 0 and we find
                // a closing bracket complete a range.
                ('}', lbi) if lbi != 0 => {
                    spans.insert(
                        parse_str::<Ident>(&current_tmpl_key)
                            .map_err(|_| TemplateError::InvalidIdent(current_tmpl_key))?
                            .to_string(),
                        (last_bracket_idx + 1)..(i + 2), // inclusive
                    );

                    last_bracket_idx = 0;
                    current_tmpl_key = String::new();
                },

                (c, lbi) if lbi != 0 => current_tmpl_key += &{ c.to_string() },

                _ => {},
            }
        }

        if last_bracket_idx != 0 {
            Err(TemplateError::Unclosed(last_bracket_idx))
        } else {
            Ok(FormatString { original, spans })
        }
    }
}

#[macro_export]
macro_rules! replace_templates {
    ($orig:expr, $($key:ident = $value:expr),* $(,)?) => {{
        $orig.parse::<$crate::misc::templating::FormatString>()
            .map(|parsed| parsed
                .replace_with(
                    vec![$((stringify!($key).to_string(), $value.to_string())),*]
                        .into_iter()
                        .collect::<std::collections::HashMap<String, String>>()
                )
            )
    }}
}

pub fn validate_format_string(original: &str) -> Result<(), TemplateError> {
    let mut last_bracket_idx = 0usize;

    for (i, c) in original.chars().enumerate() {
        match (c, last_bracket_idx) {
            ('{', lbi) if lbi != i.saturating_sub(1) => last_bracket_idx = i,
            ('{', lbi) if lbi == i.saturating_sub(1) => last_bracket_idx = 0,
            ('}', lbi) if lbi != 0 => last_bracket_idx = 0,

            (c, lbi) if i > 0 && lbi == i - 1 && !c.is_xid_start() => {
                return Err(TemplateError::InvalidxIdStart(c));
            },

            (c, lbi) if lbi != 0 && !c.is_xid_continue() => {
                return Err(TemplateError::InvalidxIdRest(c));
            },

            _ => {},
        }
    }

    Ok(())
}

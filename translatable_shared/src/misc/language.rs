//! [`Language`] declaration module.
//!
//! This module declares all the implementations
//! required for parsing and validating ISO-639-1
//! language strings from user input.

use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{ToTokens, TokenStreamExt, quote};
use strum::{Display, EnumIter, EnumString};
use syn::Ident;

/// This implementation converts the tagged union
/// to an equivalent call from the runtime context.
///
/// This is exclusively meant to be used from the
/// macro generation context.
impl ToTokens for Language {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let ident = Ident::new(&format!("{self:?}"), Span::call_site());

        tokens.append_all(quote! { translatable::shared::misc::language::Language::#ident })
    }
}

/// ISO 639-1 language code implementation with validation
///
/// Provides two-way mapping between language codes and names with:
/// - Case-insensitive parsing
/// - Strict validation
/// - Complete ISO 639-1 coverage
#[derive(Debug, Copy, Clone, EnumIter, Display, EnumString, Eq, Hash, PartialEq)]
#[strum(ascii_case_insensitive)]
pub enum Language {
    #[allow(missing_docs)]
    #[strum(serialize = "Abkhazian", serialize = "ab")]
    AB,
    #[allow(missing_docs)]
    #[strum(serialize = "Afar", serialize = "aa")]
    AA,
    #[allow(missing_docs)]
    #[strum(serialize = "Afrikaans", serialize = "af")]
    AF,
    #[allow(missing_docs)]
    #[strum(serialize = "Akan", serialize = "ak")]
    AK,
    #[allow(missing_docs)]
    #[strum(serialize = "Albanian", serialize = "sq")]
    SQ,
    #[allow(missing_docs)]
    #[strum(serialize = "Amharic", serialize = "am")]
    AM,
    #[allow(missing_docs)]
    #[strum(serialize = "Arabic", serialize = "ar")]
    AR,
    #[allow(missing_docs)]
    #[strum(serialize = "Aragonese", serialize = "an")]
    AN,
    #[allow(missing_docs)]
    #[strum(serialize = "Armenian", serialize = "hy")]
    HY,
    #[allow(missing_docs)]
    #[strum(serialize = "Assamese", serialize = "as")]
    AS,
    #[allow(missing_docs)]
    #[strum(serialize = "Avaric", serialize = "av")]
    AV,
    #[allow(missing_docs)]
    #[strum(serialize = "Avestan", serialize = "ae")]
    AE,
    #[allow(missing_docs)]
    #[strum(serialize = "Aymara", serialize = "ay")]
    AY,
    #[allow(missing_docs)]
    #[strum(serialize = "Azerbaijani", serialize = "az")]
    AZ,
    #[allow(missing_docs)]
    #[strum(serialize = "Bambara", serialize = "bm")]
    BM,
    #[allow(missing_docs)]
    #[strum(serialize = "Bashkir", serialize = "ba")]
    BA,
    #[allow(missing_docs)]
    #[strum(serialize = "Basque", serialize = "eu")]
    EU,
    #[allow(missing_docs)]
    #[strum(serialize = "Belarusian", serialize = "be")]
    BE,
    #[allow(missing_docs)]
    #[strum(serialize = "Bengali", serialize = "bn")]
    BN,
    #[allow(missing_docs)]
    #[strum(serialize = "Bislama", serialize = "bi")]
    BI,
    #[allow(missing_docs)]
    #[strum(serialize = "Bosnian", serialize = "bs")]
    BS,
    #[allow(missing_docs)]
    #[strum(serialize = "Breton", serialize = "br")]
    BR,
    #[allow(missing_docs)]
    #[strum(serialize = "Bulgarian", serialize = "bg")]
    BG,
    #[allow(missing_docs)]
    #[strum(serialize = "Burmese", serialize = "my")]
    MY,
    #[allow(missing_docs)]
    #[strum(serialize = "Catalan", serialize = "ca")]
    CA,
    #[allow(missing_docs)]
    #[strum(serialize = "Chamorro", serialize = "ch")]
    CH,
    #[allow(missing_docs)]
    #[strum(serialize = "Chechen", serialize = "ce")]
    CE,
    #[allow(missing_docs)]
    #[strum(serialize = "Chichewa", serialize = "ny")]
    NY,
    #[allow(missing_docs)]
    #[strum(serialize = "Chinese", serialize = "zh")]
    ZH,
    #[allow(missing_docs)]
    #[strum(serialize = "Church Slavonic", serialize = "cu")]
    CU,
    #[allow(missing_docs)]
    #[strum(serialize = "Chuvash", serialize = "cv")]
    CV,
    #[allow(missing_docs)]
    #[strum(serialize = "Cornish", serialize = "kw")]
    KW,
    #[allow(missing_docs)]
    #[strum(serialize = "Corsican", serialize = "co")]
    CO,
    #[allow(missing_docs)]
    #[strum(serialize = "Cree", serialize = "cr")]
    CR,
    #[allow(missing_docs)]
    #[strum(serialize = "Croatian", serialize = "hr")]
    HR,
    #[allow(missing_docs)]
    #[strum(serialize = "Czech", serialize = "cs")]
    CS,
    #[allow(missing_docs)]
    #[strum(serialize = "Danish", serialize = "da")]
    DA,
    #[allow(missing_docs)]
    #[strum(serialize = "Divehi", serialize = "dv")]
    DV,
    #[allow(missing_docs)]
    #[strum(serialize = "Dutch", serialize = "nl")]
    NL,
    #[allow(missing_docs)]
    #[strum(serialize = "Dzongkha", serialize = "dz")]
    DZ,
    #[allow(missing_docs)]
    #[strum(serialize = "English", serialize = "en")]
    EN,
    #[allow(missing_docs)]
    #[strum(serialize = "Esperanto", serialize = "eo")]
    EO,
    #[allow(missing_docs)]
    #[strum(serialize = "Estonian", serialize = "et")]
    ET,
    #[allow(missing_docs)]
    #[strum(serialize = "Ewe", serialize = "ee")]
    EE,
    #[allow(missing_docs)]
    #[strum(serialize = "Faroese", serialize = "fo")]
    FO,
    #[allow(missing_docs)]
    #[strum(serialize = "Fijian", serialize = "fj")]
    FJ,
    #[allow(missing_docs)]
    #[strum(serialize = "Finnish", serialize = "fi")]
    FI,
    #[allow(missing_docs)]
    #[strum(serialize = "French", serialize = "fr")]
    FR,
    #[allow(missing_docs)]
    #[strum(serialize = "Western Frisian", serialize = "fy")]
    FY,
    #[allow(missing_docs)]
    #[strum(serialize = "Fulah", serialize = "ff")]
    FF,
    #[allow(missing_docs)]
    #[strum(serialize = "Gaelic", serialize = "gd")]
    GD,
    #[allow(missing_docs)]
    #[strum(serialize = "Galician", serialize = "gl")]
    GL,
    #[allow(missing_docs)]
    #[strum(serialize = "Ganda", serialize = "lg")]
    LG,
    #[allow(missing_docs)]
    #[strum(serialize = "Georgian", serialize = "ka")]
    KA,
    #[allow(missing_docs)]
    #[strum(serialize = "German", serialize = "de")]
    DE,
    #[allow(missing_docs)]
    #[strum(serialize = "Greek", serialize = "el")]
    EL,
    #[allow(missing_docs)]
    #[strum(serialize = "Kalaallisut", serialize = "kl")]
    KL,
    #[allow(missing_docs)]
    #[strum(serialize = "Guarani", serialize = "gn")]
    GN,
    #[allow(missing_docs)]
    #[strum(serialize = "Gujarati", serialize = "gu")]
    GU,
    #[allow(missing_docs)]
    #[strum(serialize = "Haitian", serialize = "ht")]
    HT,
    #[allow(missing_docs)]
    #[strum(serialize = "Hausa", serialize = "ha")]
    HA,
    #[allow(missing_docs)]
    #[strum(serialize = "Hebrew", serialize = "he")]
    HE,
    #[allow(missing_docs)]
    #[strum(serialize = "Herero", serialize = "hz")]
    HZ,
    #[allow(missing_docs)]
    #[strum(serialize = "Hindi", serialize = "hi")]
    HI,
    #[allow(missing_docs)]
    #[strum(serialize = "Hiri Motu", serialize = "ho")]
    HO,
    #[allow(missing_docs)]
    #[strum(serialize = "Hungarian", serialize = "hu")]
    HU,
    #[allow(missing_docs)]
    #[strum(serialize = "Icelandic", serialize = "is")]
    IS,
    #[allow(missing_docs)]
    #[strum(serialize = "Ido", serialize = "io")]
    IO,
    #[allow(missing_docs)]
    #[strum(serialize = "Igbo", serialize = "ig")]
    IG,
    #[allow(missing_docs)]
    #[strum(serialize = "Indonesian", serialize = "id")]
    ID,
    #[allow(missing_docs)]
    #[strum(serialize = "Interlingua", serialize = "ia")]
    IA,
    #[allow(missing_docs)]
    #[strum(serialize = "Interlingue", serialize = "ie")]
    IE,
    #[allow(missing_docs)]
    #[strum(serialize = "Inuktitut", serialize = "iu")]
    IU,
    #[allow(missing_docs)]
    #[strum(serialize = "Inupiaq", serialize = "ik")]
    IK,
    #[allow(missing_docs)]
    #[strum(serialize = "Irish", serialize = "ga")]
    GA,
    #[allow(missing_docs)]
    #[strum(serialize = "Italian", serialize = "it")]
    IT,
    #[allow(missing_docs)]
    #[strum(serialize = "Japanese", serialize = "ja")]
    JA,
    #[allow(missing_docs)]
    #[strum(serialize = "Javanese", serialize = "jv")]
    JV,
    #[allow(missing_docs)]
    #[strum(serialize = "Kannada", serialize = "kn")]
    KN,
    #[allow(missing_docs)]
    #[strum(serialize = "Kanuri", serialize = "kr")]
    KR,
    #[allow(missing_docs)]
    #[strum(serialize = "Kashmiri", serialize = "ks")]
    KS,
    #[allow(missing_docs)]
    #[strum(serialize = "Kazakh", serialize = "kk")]
    KK,
    #[allow(missing_docs)]
    #[strum(serialize = "Central Khmer", serialize = "km")]
    KM,
    #[allow(missing_docs)]
    #[strum(serialize = "Kikuyu", serialize = "ki")]
    KI,
    #[allow(missing_docs)]
    #[strum(serialize = "Kinyarwanda", serialize = "rw")]
    RW,
    #[allow(missing_docs)]
    #[strum(serialize = "Kyrgyz", serialize = "ky")]
    KY,
    #[allow(missing_docs)]
    #[strum(serialize = "Komi", serialize = "kv")]
    KV,
    #[allow(missing_docs)]
    #[strum(serialize = "Kongo", serialize = "kg")]
    KG,
    #[allow(missing_docs)]
    #[strum(serialize = "Korean", serialize = "ko")]
    KO,
    #[allow(missing_docs)]
    #[strum(serialize = "Kuanyama", serialize = "kj")]
    KJ,
    #[allow(missing_docs)]
    #[strum(serialize = "Kurdish", serialize = "ku")]
    KU,
    #[allow(missing_docs)]
    #[strum(serialize = "Lao", serialize = "lo")]
    LO,
    #[allow(missing_docs)]
    #[strum(serialize = "Latin", serialize = "la")]
    LA,
    #[allow(missing_docs)]
    #[strum(serialize = "Latvian", serialize = "lv")]
    LV,
    #[allow(missing_docs)]
    #[strum(serialize = "Limburgan", serialize = "li")]
    LI,
    #[allow(missing_docs)]
    #[strum(serialize = "Lingala", serialize = "ln")]
    LN,
    #[allow(missing_docs)]
    #[strum(serialize = "Lithuanian", serialize = "lt")]
    LT,
    #[allow(missing_docs)]
    #[strum(serialize = "Luba-Katanga", serialize = "lu")]
    LU,
    #[allow(missing_docs)]
    #[strum(serialize = "Luxembourgish", serialize = "lb")]
    LB,
    #[allow(missing_docs)]
    #[strum(serialize = "Macedonian", serialize = "mk")]
    MK,
    #[allow(missing_docs)]
    #[strum(serialize = "Malagasy", serialize = "mg")]
    MG,
    #[allow(missing_docs)]
    #[strum(serialize = "Malay", serialize = "ms")]
    MS,
    #[allow(missing_docs)]
    #[strum(serialize = "Malayalam", serialize = "ml")]
    ML,
    #[allow(missing_docs)]
    #[strum(serialize = "Maltese", serialize = "mt")]
    MT,
    #[allow(missing_docs)]
    #[strum(serialize = "Manx", serialize = "gv")]
    GV,
    #[allow(missing_docs)]
    #[strum(serialize = "Maori", serialize = "mi")]
    MI,
    #[allow(missing_docs)]
    #[strum(serialize = "Marathi", serialize = "mr")]
    MR,
    #[allow(missing_docs)]
    #[strum(serialize = "Marshallese", serialize = "mh")]
    MH,
    #[allow(missing_docs)]
    #[strum(serialize = "Mongolian", serialize = "mn")]
    MN,
    #[allow(missing_docs)]
    #[strum(serialize = "Nauru", serialize = "na")]
    NA,
    #[allow(missing_docs)]
    #[strum(serialize = "Navajo", serialize = "nv")]
    NV,
    #[allow(missing_docs)]
    #[strum(serialize = "North Ndebele", serialize = "nd")]
    ND,
    #[allow(missing_docs)]
    #[strum(serialize = "South Ndebele", serialize = "nr")]
    NR,
    #[allow(missing_docs)]
    #[strum(serialize = "Nepali", serialize = "ng")]
    NG,
    #[allow(missing_docs)]
    #[strum(serialize = "Nepali", serialize = "ne")]
    NE,
    #[allow(missing_docs)]
    #[strum(serialize = "Norwegian", serialize = "no")]
    NO,
    #[allow(missing_docs)]
    #[strum(serialize = "Norwegian Bokmål", serialize = "nb")]
    NB,
    #[allow(missing_docs)]
    #[strum(serialize = "Norwegian Nynorsk", serialize = "nn")]
    NN,
    #[allow(missing_docs)]
    #[strum(serialize = "Occitan", serialize = "oc")]
    OC,
    #[allow(missing_docs)]
    #[strum(serialize = "Ojibwa", serialize = "oj")]
    OJ,
    #[allow(missing_docs)]
    #[strum(serialize = "Oriya", serialize = "or")]
    OR,
    #[allow(missing_docs)]
    #[strum(serialize = "Oromo", serialize = "om")]
    OM,
    #[allow(missing_docs)]
    #[strum(serialize = "Ossetian", serialize = "os")]
    OS,
    #[allow(missing_docs)]
    #[strum(serialize = "Pali", serialize = "pi")]
    PI,
    #[allow(missing_docs)]
    #[strum(serialize = "Pashto", serialize = "ps")]
    PS,
    #[allow(missing_docs)]
    #[strum(serialize = "Persian", serialize = "fa")]
    FA,
    #[allow(missing_docs)]
    #[strum(serialize = "Polish", serialize = "pl")]
    PL,
    #[allow(missing_docs)]
    #[strum(serialize = "Portuguese", serialize = "pt")]
    PT,
    #[allow(missing_docs)]
    #[strum(serialize = "Punjabi", serialize = "pa")]
    PA,
    #[allow(missing_docs)]
    #[strum(serialize = "Quechua", serialize = "qu")]
    QU,
    #[allow(missing_docs)]
    #[strum(serialize = "Romanian", serialize = "ro")]
    RO,
    #[allow(missing_docs)]
    #[strum(serialize = "Romansh", serialize = "rm")]
    RM,
    #[allow(missing_docs)]
    #[strum(serialize = "Rundi", serialize = "rn")]
    RN,
    #[allow(missing_docs)]
    #[strum(serialize = "Russian", serialize = "ru")]
    RU,
    #[allow(missing_docs)]
    #[strum(serialize = "North Sami", serialize = "se")]
    SE,
    #[allow(missing_docs)]
    #[strum(serialize = "Samoan", serialize = "sm")]
    SM,
    #[allow(missing_docs)]
    #[strum(serialize = "Sango", serialize = "sg")]
    SG,
    #[allow(missing_docs)]
    #[strum(serialize = "Sanskrit", serialize = "sa")]
    SA,
    #[allow(missing_docs)]
    #[strum(serialize = "Sardinian", serialize = "sc")]
    SC,
    #[allow(missing_docs)]
    #[strum(serialize = "Serbian", serialize = "sr")]
    SR,
    #[allow(missing_docs)]
    #[strum(serialize = "Shona", serialize = "sn")]
    SN,
    #[allow(missing_docs)]
    #[strum(serialize = "Sindhi", serialize = "sd")]
    SD,
    #[allow(missing_docs)]
    #[strum(serialize = "Sinhala", serialize = "si")]
    SI,
    #[allow(missing_docs)]
    #[strum(serialize = "Slovak", serialize = "sk")]
    SK,
    #[allow(missing_docs)]
    #[strum(serialize = "Slovenian", serialize = "sl")]
    SL,
    #[allow(missing_docs)]
    #[strum(serialize = "Somali", serialize = "so")]
    SO,
    #[allow(missing_docs)]
    #[strum(serialize = "Southern Sotho", serialize = "st")]
    ST,
    #[allow(missing_docs)]
    #[strum(serialize = "Spanish", serialize = "es")]
    ES,
    #[allow(missing_docs)]
    #[strum(serialize = "Sundanese", serialize = "su")]
    SU,
    #[allow(missing_docs)]
    #[strum(serialize = "Swahili", serialize = "sw")]
    SW,
    #[allow(missing_docs)]
    #[strum(serialize = "Swati", serialize = "ss")]
    SS,
    #[allow(missing_docs)]
    #[strum(serialize = "Swedish", serialize = "sv")]
    SV,
    #[allow(missing_docs)]
    #[strum(serialize = "Tagalog", serialize = "tl")]
    TL,
    #[allow(missing_docs)]
    #[strum(serialize = "Tahitian", serialize = "ty")]
    TY,
    #[allow(missing_docs)]
    #[strum(serialize = "Tajik", serialize = "tg")]
    TG,
    #[allow(missing_docs)]
    #[strum(serialize = "Tamil", serialize = "ta")]
    TA,
    #[allow(missing_docs)]
    #[strum(serialize = "Tatar", serialize = "tt")]
    TT,
    #[allow(missing_docs)]
    #[strum(serialize = "Telugu", serialize = "te")]
    TE,
    #[allow(missing_docs)]
    #[strum(serialize = "Thai", serialize = "th")]
    TH,
    #[allow(missing_docs)]
    #[strum(serialize = "Tibetan", serialize = "bo")]
    BO,
    #[allow(missing_docs)]
    #[strum(serialize = "Tigrinya", serialize = "ti")]
    TI,
    #[allow(missing_docs)]
    #[strum(serialize = "Tonga", serialize = "to")]
    TO,
    #[allow(missing_docs)]
    #[strum(serialize = "Tsonga", serialize = "ts")]
    TS,
    #[allow(missing_docs)]
    #[strum(serialize = "Tswana", serialize = "tn")]
    TN,
    #[allow(missing_docs)]
    #[strum(serialize = "Turkish", serialize = "tr")]
    TR,
    #[allow(missing_docs)]
    #[strum(serialize = "Turkmen", serialize = "tk")]
    TK,
    #[allow(missing_docs)]
    #[strum(serialize = "Twi", serialize = "tw")]
    TW,
    #[allow(missing_docs)]
    #[strum(serialize = "Uighur", serialize = "ug")]
    UG,
    #[allow(missing_docs)]
    #[strum(serialize = "Ukrainian", serialize = "uk")]
    UK,
    #[allow(missing_docs)]
    #[strum(serialize = "Urdu", serialize = "ur")]
    UR,
    #[allow(missing_docs)]
    #[strum(serialize = "Uzbek", serialize = "uz")]
    UZ,
    #[allow(missing_docs)]
    #[strum(serialize = "Venda", serialize = "ve")]
    VE,
    #[allow(missing_docs)]
    #[strum(serialize = "Vietnamese", serialize = "vi")]
    VI,
    #[allow(missing_docs)]
    #[strum(serialize = "Volapük", serialize = "vo")]
    VO,
    #[allow(missing_docs)]
    #[strum(serialize = "Walloon", serialize = "wa")]
    WA,
    #[allow(missing_docs)]
    #[strum(serialize = "Welsh", serialize = "cy")]
    CY,
    #[allow(missing_docs)]
    #[strum(serialize = "Wolof", serialize = "wo")]
    WO,
    #[allow(missing_docs)]
    #[strum(serialize = "Xhosa", serialize = "xh")]
    XH,
    #[allow(missing_docs)]
    #[strum(serialize = "Sichuan Yi", serialize = "ii")]
    II,
    #[allow(missing_docs)]
    #[strum(serialize = "Yiddish", serialize = "yi")]
    YI,
    #[allow(missing_docs)]
    #[strum(serialize = "Yoruba", serialize = "yo")]
    YO,
    #[allow(missing_docs)]
    #[strum(serialize = "Zhuang", serialize = "za")]
    ZA,
    #[allow(missing_docs)]
    #[strum(serialize = "Zulu", serialize = "zu")]
    ZU,
}

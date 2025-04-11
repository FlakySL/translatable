use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{ToTokens, TokenStreamExt, quote};
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};
use syn::Ident;

/// ISO 639-1 language code implementation with validation
///
/// Provides two-way mapping between language codes and names with:
/// - Case-insensitive parsing
/// - Strict validation
/// - Complete ISO 639-1 coverage
#[derive(Debug, Clone, EnumIter, Display, EnumString, Eq, Hash, PartialEq)]
#[strum(ascii_case_insensitive)]
pub enum Language {
    #[strum(serialize = "Abkhazian", serialize = "ab")]
    AB,
    #[strum(serialize = "Afar", serialize = "aa")]
    AA,
    #[strum(serialize = "Afrikaans", serialize = "af")]
    AF,
    #[strum(serialize = "Akan", serialize = "ak")]
    AK,
    #[strum(serialize = "Albanian", serialize = "sq")]
    SQ,
    #[strum(serialize = "Amharic", serialize = "am")]
    AM,
    #[strum(serialize = "Arabic", serialize = "ar")]
    AR,
    #[strum(serialize = "Aragonese", serialize = "an")]
    AN,
    #[strum(serialize = "Armenian", serialize = "hy")]
    HY,
    #[strum(serialize = "Assamese", serialize = "as")]
    AS,
    #[strum(serialize = "Avaric", serialize = "av")]
    AV,
    #[strum(serialize = "Avestan", serialize = "ae")]
    AE,
    #[strum(serialize = "Aymara", serialize = "ay")]
    AY,
    #[strum(serialize = "Azerbaijani", serialize = "az")]
    AZ,
    #[strum(serialize = "Bambara", serialize = "bm")]
    BM,
    #[strum(serialize = "Bashkir", serialize = "ba")]
    BA,
    #[strum(serialize = "Basque", serialize = "eu")]
    EU,
    #[strum(serialize = "Belarusian", serialize = "be")]
    BE,
    #[strum(serialize = "Bengali", serialize = "bn")]
    BN,
    #[strum(serialize = "Bislama", serialize = "bi")]
    BI,
    #[strum(serialize = "Bosnian", serialize = "bs")]
    BS,
    #[strum(serialize = "Breton", serialize = "br")]
    BR,
    #[strum(serialize = "Bulgarian", serialize = "bg")]
    BG,
    #[strum(serialize = "Burmese", serialize = "my")]
    MY,
    #[strum(serialize = "Catalan", serialize = "ca")]
    CA,
    #[strum(serialize = "Chamorro", serialize = "ch")]
    CH,
    #[strum(serialize = "Chechen", serialize = "ce")]
    CE,
    #[strum(serialize = "Chichewa", serialize = "ny")]
    NY,
    #[strum(serialize = "Chinese", serialize = "zh")]
    ZH,
    #[strum(serialize = "Church Slavonic", serialize = "cu")]
    CU,
    #[strum(serialize = "Chuvash", serialize = "cv")]
    CV,
    #[strum(serialize = "Cornish", serialize = "kw")]
    KW,
    #[strum(serialize = "Corsican", serialize = "co")]
    CO,
    #[strum(serialize = "Cree", serialize = "cr")]
    CR,
    #[strum(serialize = "Croatian", serialize = "hr")]
    HR,
    #[strum(serialize = "Czech", serialize = "cs")]
    CS,
    #[strum(serialize = "Danish", serialize = "da")]
    DA,
    #[strum(serialize = "Divehi", serialize = "dv")]
    DV,
    #[strum(serialize = "Dutch", serialize = "nl")]
    NL,
    #[strum(serialize = "Dzongkha", serialize = "dz")]
    DZ,
    #[strum(serialize = "English", serialize = "en")]
    EN,
    #[strum(serialize = "Esperanto", serialize = "eo")]
    EO,
    #[strum(serialize = "Estonian", serialize = "et")]
    ET,
    #[strum(serialize = "Ewe", serialize = "ee")]
    EE,
    #[strum(serialize = "Faroese", serialize = "fo")]
    FO,
    #[strum(serialize = "Fijian", serialize = "fj")]
    FJ,
    #[strum(serialize = "Finnish", serialize = "fi")]
    FI,
    #[strum(serialize = "French", serialize = "fr")]
    FR,
    #[strum(serialize = "Western Frisian", serialize = "fy")]
    FY,
    #[strum(serialize = "Fulah", serialize = "ff")]
    FF,
    #[strum(serialize = "Gaelic", serialize = "gd")]
    GD,
    #[strum(serialize = "Galician", serialize = "gl")]
    GL,
    #[strum(serialize = "Ganda", serialize = "lg")]
    LG,
    #[strum(serialize = "Georgian", serialize = "ka")]
    KA,
    #[strum(serialize = "German", serialize = "de")]
    DE,
    #[strum(serialize = "Greek", serialize = "el")]
    EL,
    #[strum(serialize = "Kalaallisut", serialize = "kl")]
    KL,
    #[strum(serialize = "Guarani", serialize = "gn")]
    GN,
    #[strum(serialize = "Gujarati", serialize = "gu")]
    GU,
    #[strum(serialize = "Haitian", serialize = "ht")]
    HT,
    #[strum(serialize = "Hausa", serialize = "ha")]
    HA,
    #[strum(serialize = "Hebrew", serialize = "he")]
    HE,
    #[strum(serialize = "Herero", serialize = "hz")]
    HZ,
    #[strum(serialize = "Hindi", serialize = "hi")]
    HI,
    #[strum(serialize = "Hiri Motu", serialize = "ho")]
    HO,
    #[strum(serialize = "Hungarian", serialize = "hu")]
    HU,
    #[strum(serialize = "Icelandic", serialize = "is")]
    IS,
    #[strum(serialize = "Ido", serialize = "io")]
    IO,
    #[strum(serialize = "Igbo", serialize = "ig")]
    IG,
    #[strum(serialize = "Indonesian", serialize = "id")]
    ID,
    #[strum(serialize = "Interlingua", serialize = "ia")]
    IA,
    #[strum(serialize = "Interlingue", serialize = "ie")]
    IE,
    #[strum(serialize = "Inuktitut", serialize = "iu")]
    IU,
    #[strum(serialize = "Inupiaq", serialize = "ik")]
    IK,
    #[strum(serialize = "Irish", serialize = "ga")]
    GA,
    #[strum(serialize = "Italian", serialize = "it")]
    IT,
    #[strum(serialize = "Japanese", serialize = "ja")]
    JA,
    #[strum(serialize = "Javanese", serialize = "jv")]
    JV,
    #[strum(serialize = "Kannada", serialize = "kn")]
    KN,
    #[strum(serialize = "Kanuri", serialize = "kr")]
    KR,
    #[strum(serialize = "Kashmiri", serialize = "ks")]
    KS,
    #[strum(serialize = "Kazakh", serialize = "kk")]
    KK,
    #[strum(serialize = "Central Khmer", serialize = "km")]
    KM,
    #[strum(serialize = "Kikuyu", serialize = "ki")]
    KI,
    #[strum(serialize = "Kinyarwanda", serialize = "rw")]
    RW,
    #[strum(serialize = "Kyrgyz", serialize = "ky")]
    KY,
    #[strum(serialize = "Komi", serialize = "kv")]
    KV,
    #[strum(serialize = "Kongo", serialize = "kg")]
    KG,
    #[strum(serialize = "Korean", serialize = "ko")]
    KO,
    #[strum(serialize = "Kuanyama", serialize = "kj")]
    KJ,
    #[strum(serialize = "Kurdish", serialize = "ku")]
    KU,
    #[strum(serialize = "Lao", serialize = "lo")]
    LO,
    #[strum(serialize = "Latin", serialize = "la")]
    LA,
    #[strum(serialize = "Latvian", serialize = "lv")]
    LV,
    #[strum(serialize = "Limburgan", serialize = "li")]
    LI,
    #[strum(serialize = "Lingala", serialize = "ln")]
    LN,
    #[strum(serialize = "Lithuanian", serialize = "lt")]
    LT,
    #[strum(serialize = "Luba-Katanga", serialize = "lu")]
    LU,
    #[strum(serialize = "Luxembourgish", serialize = "lb")]
    LB,
    #[strum(serialize = "Macedonian", serialize = "mk")]
    MK,
    #[strum(serialize = "Malagasy", serialize = "mg")]
    MG,
    #[strum(serialize = "Malay", serialize = "ms")]
    MS,
    #[strum(serialize = "Malayalam", serialize = "ml")]
    ML,
    #[strum(serialize = "Maltese", serialize = "mt")]
    MT,
    #[strum(serialize = "Manx", serialize = "gv")]
    GV,
    #[strum(serialize = "Maori", serialize = "mi")]
    MI,
    #[strum(serialize = "Marathi", serialize = "mr")]
    MR,
    #[strum(serialize = "Marshallese", serialize = "mh")]
    MH,
    #[strum(serialize = "Mongolian", serialize = "mn")]
    MN,
    #[strum(serialize = "Nauru", serialize = "na")]
    NA,
    #[strum(serialize = "Navajo", serialize = "nv")]
    NV,
    #[strum(serialize = "North Ndebele", serialize = "nd")]
    ND,
    #[strum(serialize = "South Ndebele", serialize = "nr")]
    NR,
    #[strum(serialize = "Nepali", serialize = "ng")]
    NG,
    #[strum(serialize = "Nepali", serialize = "ne")]
    NE,
    #[strum(serialize = "Norwegian", serialize = "no")]
    NO,
    #[strum(serialize = "Norwegian Bokmål", serialize = "nb")]
    NB,
    #[strum(serialize = "Norwegian Nynorsk", serialize = "nn")]
    NN,
    #[strum(serialize = "Occitan", serialize = "oc")]
    OC,
    #[strum(serialize = "Ojibwa", serialize = "oj")]
    OJ,
    #[strum(serialize = "Oriya", serialize = "or")]
    OR,
    #[strum(serialize = "Oromo", serialize = "om")]
    OM,
    #[strum(serialize = "Ossetian", serialize = "os")]
    OS,
    #[strum(serialize = "Pali", serialize = "pi")]
    PI,
    #[strum(serialize = "Pashto", serialize = "ps")]
    PS,
    #[strum(serialize = "Persian", serialize = "fa")]
    FA,
    #[strum(serialize = "Polish", serialize = "pl")]
    PL,
    #[strum(serialize = "Portuguese", serialize = "pt")]
    PT,
    #[strum(serialize = "Punjabi", serialize = "pa")]
    PA,
    #[strum(serialize = "Quechua", serialize = "qu")]
    QU,
    #[strum(serialize = "Romanian", serialize = "ro")]
    RO,
    #[strum(serialize = "Romansh", serialize = "rm")]
    RM,
    #[strum(serialize = "Rundi", serialize = "rn")]
    RN,
    #[strum(serialize = "Russian", serialize = "ru")]
    RU,
    #[strum(serialize = "North Sami", serialize = "se")]
    SE,
    #[strum(serialize = "Samoan", serialize = "sm")]
    SM,
    #[strum(serialize = "Sango", serialize = "sg")]
    SG,
    #[strum(serialize = "Sanskrit", serialize = "sa")]
    SA,
    #[strum(serialize = "Sardinian", serialize = "sc")]
    SC,
    #[strum(serialize = "Serbian", serialize = "sr")]
    SR,
    #[strum(serialize = "Shona", serialize = "sn")]
    SN,
    #[strum(serialize = "Sindhi", serialize = "sd")]
    SD,
    #[strum(serialize = "Sinhala", serialize = "si")]
    SI,
    #[strum(serialize = "Slovak", serialize = "sk")]
    SK,
    #[strum(serialize = "Slovenian", serialize = "sl")]
    SL,
    #[strum(serialize = "Somali", serialize = "so")]
    SO,
    #[strum(serialize = "Southern Sotho", serialize = "st")]
    ST,
    #[strum(serialize = "Spanish", serialize = "es")]
    ES,
    #[strum(serialize = "Sundanese", serialize = "su")]
    SU,
    #[strum(serialize = "Swahili", serialize = "sw")]
    SW,
    #[strum(serialize = "Swati", serialize = "ss")]
    SS,
    #[strum(serialize = "Swedish", serialize = "sv")]
    SV,
    #[strum(serialize = "Tagalog", serialize = "tl")]
    TL,
    #[strum(serialize = "Tahitian", serialize = "ty")]
    TY,
    #[strum(serialize = "Tajik", serialize = "tg")]
    TG,
    #[strum(serialize = "Tamil", serialize = "ta")]
    TA,
    #[strum(serialize = "Tatar", serialize = "tt")]
    TT,
    #[strum(serialize = "Telugu", serialize = "te")]
    TE,
    #[strum(serialize = "Thai", serialize = "th")]
    TH,
    #[strum(serialize = "Tibetan", serialize = "bo")]
    BO,
    #[strum(serialize = "Tigrinya", serialize = "ti")]
    TI,
    #[strum(serialize = "Tonga", serialize = "to")]
    TO,
    #[strum(serialize = "Tsonga", serialize = "ts")]
    TS,
    #[strum(serialize = "Tswana", serialize = "tn")]
    TN,
    #[strum(serialize = "Turkish", serialize = "tr")]
    TR,
    #[strum(serialize = "Turkmen", serialize = "tk")]
    TK,
    #[strum(serialize = "Twi", serialize = "tw")]
    TW,
    #[strum(serialize = "Uighur", serialize = "ug")]
    UG,
    #[strum(serialize = "Ukrainian", serialize = "uk")]
    UK,
    #[strum(serialize = "Urdu", serialize = "ur")]
    UR,
    #[strum(serialize = "Uzbek", serialize = "uz")]
    UZ,
    #[strum(serialize = "Venda", serialize = "ve")]
    VE,
    #[strum(serialize = "Vietnamese", serialize = "vi")]
    VI,
    #[strum(serialize = "Volapük", serialize = "vo")]
    VO,
    #[strum(serialize = "Walloon", serialize = "wa")]
    WA,
    #[strum(serialize = "Welsh", serialize = "cy")]
    CY,
    #[strum(serialize = "Wolof", serialize = "wo")]
    WO,
    #[strum(serialize = "Xhosa", serialize = "xh")]
    XH,
    #[strum(serialize = "Sichuan Yi", serialize = "ii")]
    II,
    #[strum(serialize = "Yiddish", serialize = "yi")]
    YI,
    #[strum(serialize = "Yoruba", serialize = "yo")]
    YO,
    #[strum(serialize = "Zhuang", serialize = "za")]
    ZA,
    #[strum(serialize = "Zulu", serialize = "zu")]
    ZU,
}

/// This struct represents a list of similar languages to the provided one.
pub struct Similarities<T: Sized> {
    /// Indicates how many languages are not included in the list.
    overflow_by: usize,
    /// List of similar languages.
    similarities: Vec<T>,
}

impl<T: Sized> Similarities<T> {
    pub fn overflow_by(&self) -> usize {
        self.overflow_by
    }

    pub fn similarities(&self) -> &[T] {
        &self.similarities
    }
}

impl Language {
    /// This method returns a list of similar languages to the provided one.
    pub fn get_similarities(lang: &str, max_amount: usize) -> Similarities<String> {
        let all_similarities = Self::iter()
            .map(|variant| format!("{variant:#} ({variant:?})"))
            .filter(|variant| variant.contains(lang))
            .collect::<Vec<_>>();

        let overflow_by = all_similarities.len() as i32 - max_amount as i32;

        if overflow_by > 0 {
            Similarities {
                similarities: all_similarities.into_iter().take(max_amount).collect(),
                overflow_by: overflow_by as usize,
            }
        } else {
            Similarities {
                similarities: all_similarities,
                overflow_by: 0,
            }
        }
    }
}

impl PartialEq<String> for Language {
    fn eq(&self, other: &String) -> bool {
        format!("{self:?}").to_lowercase() == other.to_lowercase()
    }
}

/// This implementation converts the tagged union
/// to an equivalent call from the runtime context.
///
/// This is exclusively meant to be used from the
/// macro generation context.
impl ToTokens for Language {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let ident = Ident::new(&format!("{self:?}"), Span::call_site());

        tokens.append_all(quote! { translatable::shared::Language::#ident })
    }
}

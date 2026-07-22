pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum GetPronunciationDictionaryWithRulesResponseModelRulesItem {
        #[serde(rename = "alias")]
        #[non_exhaustive]
        Alias {
            #[serde(default)]
            string_to_replace: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            case_sensitive: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            word_boundaries: Option<bool>,
            #[serde(default)]
            alias: String,
        },

        #[serde(rename = "phoneme")]
        #[non_exhaustive]
        Phoneme {
            #[serde(default)]
            string_to_replace: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            case_sensitive: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            word_boundaries: Option<bool>,
            #[serde(default)]
            phoneme: String,
            #[serde(default)]
            alphabet: String,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl GetPronunciationDictionaryWithRulesResponseModelRulesItem {
    pub fn alias(string_to_replace: String, alias: String) -> Self {
        Self::Alias { string_to_replace, case_sensitive: None, word_boundaries: None, alias }
    }

    pub fn phoneme(string_to_replace: String, phoneme: String, alphabet: String) -> Self {
        Self::Phoneme { string_to_replace, case_sensitive: None, word_boundaries: None, phoneme, alphabet }
    }

    pub fn alias_with_case_sensitive(string_to_replace: String, case_sensitive: bool, word_boundaries: Option<bool>, alias: String) -> Self {
        Self::Alias { string_to_replace, case_sensitive: Some(case_sensitive), word_boundaries, alias }
    }

    pub fn alias_with_word_boundaries(string_to_replace: String, case_sensitive: Option<bool>, word_boundaries: bool, alias: String) -> Self {
        Self::Alias { string_to_replace, case_sensitive, word_boundaries: Some(word_boundaries), alias }
    }

    pub fn phoneme_with_case_sensitive(string_to_replace: String, case_sensitive: bool, word_boundaries: Option<bool>, phoneme: String, alphabet: String) -> Self {
        Self::Phoneme { string_to_replace, case_sensitive: Some(case_sensitive), word_boundaries, phoneme, alphabet }
    }

    pub fn phoneme_with_word_boundaries(string_to_replace: String, case_sensitive: Option<bool>, word_boundaries: bool, phoneme: String, alphabet: String) -> Self {
        Self::Phoneme { string_to_replace, case_sensitive, word_boundaries: Some(word_boundaries), phoneme, alphabet }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}

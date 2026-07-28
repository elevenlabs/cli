pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind")]
#[non_exhaustive]
pub enum LanguagesResponse {
        #[serde(rename = "pair")]
        #[non_exhaustive]
        Pair {
            #[serde(default)]
            language_pairs: Vec<LanguagePairInfo>,
        },

        #[serde(rename = "single")]
        #[non_exhaustive]
        Single {
            #[serde(default)]
            languages: Vec<LanguageInfo>,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl LanguagesResponse {
    pub fn pair(language_pairs: Vec<LanguagePairInfo>) -> Self {
        Self::Pair { language_pairs }
    }

    pub fn single(languages: Vec<LanguageInfo>) -> Self {
        Self::Single { languages }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}

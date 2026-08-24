pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum BodySetRulesOnThePronunciationDictionaryV1PronunciationDictionariesPronunciationDictionaryIdSetRulesPostRulesItem {
        #[serde(rename = "alias")]
        #[non_exhaustive]
        Alias {
            #[serde(flatten)]
            data: PronunciationDictionaryAliasRuleRequestModel,
        },

        #[serde(rename = "phoneme")]
        #[non_exhaustive]
        Phoneme {
            #[serde(flatten)]
            data: PronunciationDictionaryPhonemeRuleRequestModel,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl BodySetRulesOnThePronunciationDictionaryV1PronunciationDictionariesPronunciationDictionaryIdSetRulesPostRulesItem {
    pub fn alias(data: PronunciationDictionaryAliasRuleRequestModel) -> Self {
        Self::Alias { data }
    }

    pub fn phoneme(data: PronunciationDictionaryPhonemeRuleRequestModel) -> Self {
        Self::Phoneme { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}

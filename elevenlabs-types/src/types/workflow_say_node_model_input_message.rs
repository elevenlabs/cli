pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum WorkflowSayNodeModelInputMessage {
        #[serde(rename = "literal")]
        #[non_exhaustive]
        Literal {
            #[serde(default)]
            text: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            text_translations: Option<HashMap<String, TranslatedString>>,
        },

        #[serde(rename = "prompt")]
        #[non_exhaustive]
        Prompt {
            #[serde(default)]
            prompt: String,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl WorkflowSayNodeModelInputMessage {
    pub fn literal(text: String) -> Self {
        Self::Literal { text, text_translations: None }
    }

    pub fn prompt(prompt: String) -> Self {
        Self::Prompt { prompt }
    }

    pub fn literal_with_text_translations(text: String, text_translations: HashMap<String, TranslatedString>) -> Self {
        Self::Literal { text, text_translations: Some(text_translations) }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}

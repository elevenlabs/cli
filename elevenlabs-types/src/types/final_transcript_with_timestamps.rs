pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FinalTranscriptWithTimestamps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<String>,
    #[serde(default)]
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub words: Option<Vec<FinalTranscriptWithTimestampsWordsItem>>,
}

impl FinalTranscriptWithTimestamps {
    pub fn builder() -> FinalTranscriptWithTimestampsBuilder {
        <FinalTranscriptWithTimestampsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FinalTranscriptWithTimestampsBuilder {
    message_type: Option<String>,
    text: Option<String>,
    language_code: Option<String>,
    words: Option<Vec<FinalTranscriptWithTimestampsWordsItem>>,
}

impl FinalTranscriptWithTimestampsBuilder {
    pub fn message_type(mut self, value: impl Into<String>) -> Self {
        self.message_type = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn language_code(mut self, value: impl Into<String>) -> Self {
        self.language_code = Some(value.into());
        self
    }

    pub fn words(mut self, value: Vec<FinalTranscriptWithTimestampsWordsItem>) -> Self {
        self.words = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FinalTranscriptWithTimestamps`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](FinalTranscriptWithTimestampsBuilder::text)
    pub fn build(self) -> Result<FinalTranscriptWithTimestamps, BuildError> {
        Ok(FinalTranscriptWithTimestamps {
            message_type: self.message_type,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            language_code: self.language_code,
            words: self.words,
        })
    }
}

pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AssetTranscriptionData {
    #[serde(default)]
    pub language_code: String,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub words: Vec<String>,
    #[serde(default)]
    pub word_start_times_ms: Vec<i64>,
    #[serde(default)]
    pub word_end_times_ms: Vec<i64>,
    #[serde(default)]
    pub word_speaker_ids: Vec<Option<String>>,
}

impl AssetTranscriptionData {
    pub fn builder() -> AssetTranscriptionDataBuilder {
        <AssetTranscriptionDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssetTranscriptionDataBuilder {
    language_code: Option<String>,
    text: Option<String>,
    words: Option<Vec<String>>,
    word_start_times_ms: Option<Vec<i64>>,
    word_end_times_ms: Option<Vec<i64>>,
    word_speaker_ids: Option<Vec<Option<String>>>,
}

impl AssetTranscriptionDataBuilder {
    pub fn language_code(mut self, value: impl Into<String>) -> Self {
        self.language_code = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn words(mut self, value: Vec<String>) -> Self {
        self.words = Some(value);
        self
    }

    pub fn word_start_times_ms(mut self, value: Vec<i64>) -> Self {
        self.word_start_times_ms = Some(value);
        self
    }

    pub fn word_end_times_ms(mut self, value: Vec<i64>) -> Self {
        self.word_end_times_ms = Some(value);
        self
    }

    pub fn word_speaker_ids(mut self, value: Vec<Option<String>>) -> Self {
        self.word_speaker_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AssetTranscriptionData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`language_code`](AssetTranscriptionDataBuilder::language_code)
    /// - [`text`](AssetTranscriptionDataBuilder::text)
    /// - [`words`](AssetTranscriptionDataBuilder::words)
    /// - [`word_start_times_ms`](AssetTranscriptionDataBuilder::word_start_times_ms)
    /// - [`word_end_times_ms`](AssetTranscriptionDataBuilder::word_end_times_ms)
    /// - [`word_speaker_ids`](AssetTranscriptionDataBuilder::word_speaker_ids)
    pub fn build(self) -> Result<AssetTranscriptionData, BuildError> {
        Ok(AssetTranscriptionData {
            language_code: self.language_code.ok_or_else(|| BuildError::missing_field("language_code"))?,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            words: self.words.ok_or_else(|| BuildError::missing_field("words"))?,
            word_start_times_ms: self.word_start_times_ms.ok_or_else(|| BuildError::missing_field("word_start_times_ms"))?,
            word_end_times_ms: self.word_end_times_ms.ok_or_else(|| BuildError::missing_field("word_end_times_ms"))?,
            word_speaker_ids: self.word_speaker_ids.ok_or_else(|| BuildError::missing_field("word_speaker_ids"))?,
        })
    }
}

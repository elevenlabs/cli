pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Word-level transcription data with timing information.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TranscriptionWord {
    /// The transcribed word.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Start time in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub start: Option<f64>,
    /// End time in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub end: Option<f64>,
    /// The type of word.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<TranscriptionWordType>,
    /// The ID of the speaker if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speaker_id: Option<String>,
    /// Confidence score for this word.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub logprob: Option<f64>,
    /// The characters in the word.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub characters: Option<Vec<String>>,
}

impl TranscriptionWord {
    pub fn builder() -> TranscriptionWordBuilder {
        <TranscriptionWordBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TranscriptionWordBuilder {
    text: Option<String>,
    start: Option<f64>,
    end: Option<f64>,
    r#type: Option<TranscriptionWordType>,
    speaker_id: Option<String>,
    logprob: Option<f64>,
    characters: Option<Vec<String>>,
}

impl TranscriptionWordBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn start(mut self, value: f64) -> Self {
        self.start = Some(value);
        self
    }

    pub fn end(mut self, value: f64) -> Self {
        self.end = Some(value);
        self
    }

    pub fn r#type(mut self, value: TranscriptionWordType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn speaker_id(mut self, value: impl Into<String>) -> Self {
        self.speaker_id = Some(value.into());
        self
    }

    pub fn logprob(mut self, value: f64) -> Self {
        self.logprob = Some(value);
        self
    }

    pub fn characters(mut self, value: Vec<String>) -> Self {
        self.characters = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TranscriptionWord`].
    pub fn build(self) -> Result<TranscriptionWord, BuildError> {
        Ok(TranscriptionWord {
            text: self.text,
            start: self.start,
            end: self.end,
            r#type: self.r#type,
            speaker_id: self.speaker_id,
            logprob: self.logprob,
            characters: self.characters,
        })
    }
}

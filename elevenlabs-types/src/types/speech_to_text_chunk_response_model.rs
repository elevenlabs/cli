pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Chunk-level detail of the transcription with timing information.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SpeechToTextChunkResponseModel {
    /// The detected language code (e.g. 'eng' for English).
    #[serde(default)]
    pub language_code: String,
    /// The confidence score of the language detection (0 to 1).
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub language_probability: f64,
    /// The raw text of the transcription.
    #[serde(default)]
    pub text: String,
    /// List of words with their timing information.
    #[serde(default)]
    pub words: Vec<SpeechToTextWordResponseModel>,
    /// The channel index this transcript belongs to (for multichannel audio).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_index: Option<i64>,
    /// Requested additional formats of the transcript.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_formats: Option<Vec<Option<AdditionalFormatResponseModel>>>,
    /// The transcription ID of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcription_id: Option<String>,
    /// List of detected entities with their text, type, and character positions in the transcript.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<DetectedEntity>>,
    /// The duration of the audio that was transcribed in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub audio_duration_secs: Option<f64>,
}

impl SpeechToTextChunkResponseModel {
    pub fn builder() -> SpeechToTextChunkResponseModelBuilder {
        <SpeechToTextChunkResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SpeechToTextChunkResponseModelBuilder {
    language_code: Option<String>,
    language_probability: Option<f64>,
    text: Option<String>,
    words: Option<Vec<SpeechToTextWordResponseModel>>,
    channel_index: Option<i64>,
    additional_formats: Option<Vec<Option<AdditionalFormatResponseModel>>>,
    transcription_id: Option<String>,
    entities: Option<Vec<DetectedEntity>>,
    audio_duration_secs: Option<f64>,
}

impl SpeechToTextChunkResponseModelBuilder {
    pub fn language_code(mut self, value: impl Into<String>) -> Self {
        self.language_code = Some(value.into());
        self
    }

    pub fn language_probability(mut self, value: f64) -> Self {
        self.language_probability = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn words(mut self, value: Vec<SpeechToTextWordResponseModel>) -> Self {
        self.words = Some(value);
        self
    }

    pub fn channel_index(mut self, value: i64) -> Self {
        self.channel_index = Some(value);
        self
    }

    pub fn additional_formats(mut self, value: Vec<Option<AdditionalFormatResponseModel>>) -> Self {
        self.additional_formats = Some(value);
        self
    }

    pub fn transcription_id(mut self, value: impl Into<String>) -> Self {
        self.transcription_id = Some(value.into());
        self
    }

    pub fn entities(mut self, value: Vec<DetectedEntity>) -> Self {
        self.entities = Some(value);
        self
    }

    pub fn audio_duration_secs(mut self, value: f64) -> Self {
        self.audio_duration_secs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SpeechToTextChunkResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`language_code`](SpeechToTextChunkResponseModelBuilder::language_code)
    /// - [`language_probability`](SpeechToTextChunkResponseModelBuilder::language_probability)
    /// - [`text`](SpeechToTextChunkResponseModelBuilder::text)
    /// - [`words`](SpeechToTextChunkResponseModelBuilder::words)
    pub fn build(self) -> Result<SpeechToTextChunkResponseModel, BuildError> {
        Ok(SpeechToTextChunkResponseModel {
            language_code: self.language_code.ok_or_else(|| BuildError::missing_field("language_code"))?,
            language_probability: self.language_probability.ok_or_else(|| BuildError::missing_field("language_probability"))?,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            words: self.words.ok_or_else(|| BuildError::missing_field("words"))?,
            channel_index: self.channel_index,
            additional_formats: self.additional_formats,
            transcription_id: self.transcription_id,
            entities: self.entities,
            audio_duration_secs: self.audio_duration_secs,
        })
    }
}

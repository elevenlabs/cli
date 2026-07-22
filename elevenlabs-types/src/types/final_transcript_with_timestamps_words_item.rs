pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Word-level detail of the transcription with timing information.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FinalTranscriptWithTimestampsWordsItem {
    /// The word or sound that was transcribed.
    #[serde(default)]
    pub text: String,
    /// The start time of the word or sound in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub start: Option<f64>,
    /// The end time of the word or sound in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub end: Option<f64>,
    /// The type of the word or sound. 'audio_event' is used for non-word sounds like laughter or footsteps.
    pub r#type: FinalTranscriptWithTimestampsWordsItemType,
    /// Unique identifier for the speaker of this word.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speaker_id: Option<String>,
    /// The log of the probability with which this word was predicted. Logprobs are in range [-infinity, 0], higher logprobs indicate a higher confidence the model has in its predictions.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub logprob: f64,
    /// The characters that make up the word and their timing information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub characters: Option<Vec<FinalTranscriptWithTimestampsWordsItemCharactersItem>>,
    /// The channel this word was spoken on (for multichannel audio). Null for single-channel transcriptions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_index: Option<i64>,
}

impl FinalTranscriptWithTimestampsWordsItem {
    pub fn builder() -> FinalTranscriptWithTimestampsWordsItemBuilder {
        <FinalTranscriptWithTimestampsWordsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FinalTranscriptWithTimestampsWordsItemBuilder {
    text: Option<String>,
    start: Option<f64>,
    end: Option<f64>,
    r#type: Option<FinalTranscriptWithTimestampsWordsItemType>,
    speaker_id: Option<String>,
    logprob: Option<f64>,
    characters: Option<Vec<FinalTranscriptWithTimestampsWordsItemCharactersItem>>,
    channel_index: Option<i64>,
}

impl FinalTranscriptWithTimestampsWordsItemBuilder {
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

    pub fn r#type(mut self, value: FinalTranscriptWithTimestampsWordsItemType) -> Self {
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

    pub fn characters(mut self, value: Vec<FinalTranscriptWithTimestampsWordsItemCharactersItem>) -> Self {
        self.characters = Some(value);
        self
    }

    pub fn channel_index(mut self, value: i64) -> Self {
        self.channel_index = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FinalTranscriptWithTimestampsWordsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](FinalTranscriptWithTimestampsWordsItemBuilder::text)
    /// - [`r#type`](FinalTranscriptWithTimestampsWordsItemBuilder::r#type)
    /// - [`logprob`](FinalTranscriptWithTimestampsWordsItemBuilder::logprob)
    pub fn build(self) -> Result<FinalTranscriptWithTimestampsWordsItem, BuildError> {
        Ok(FinalTranscriptWithTimestampsWordsItem {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            start: self.start,
            end: self.end,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            speaker_id: self.speaker_id,
            logprob: self.logprob.ok_or_else(|| BuildError::missing_field("logprob"))?,
            characters: self.characters,
            channel_index: self.channel_index,
        })
    }
}

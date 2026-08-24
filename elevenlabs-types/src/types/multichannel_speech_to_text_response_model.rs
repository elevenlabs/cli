pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Response model for multichannel speech-to-text transcription.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MultichannelSpeechToTextResponseModel {
    /// List of transcripts, one for each audio channel. Each transcript contains the text and word-level details for its respective channel.
    #[serde(default)]
    pub transcripts: Vec<SpeechToTextChunkResponseModel>,
    /// The transcription ID of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcription_id: Option<String>,
    /// The duration of the audio that was transcribed across all channels in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub audio_duration_secs: Option<f64>,
}

impl MultichannelSpeechToTextResponseModel {
    pub fn builder() -> MultichannelSpeechToTextResponseModelBuilder {
        <MultichannelSpeechToTextResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MultichannelSpeechToTextResponseModelBuilder {
    transcripts: Option<Vec<SpeechToTextChunkResponseModel>>,
    transcription_id: Option<String>,
    audio_duration_secs: Option<f64>,
}

impl MultichannelSpeechToTextResponseModelBuilder {
    pub fn transcripts(mut self, value: Vec<SpeechToTextChunkResponseModel>) -> Self {
        self.transcripts = Some(value);
        self
    }

    pub fn transcription_id(mut self, value: impl Into<String>) -> Self {
        self.transcription_id = Some(value.into());
        self
    }

    pub fn audio_duration_secs(mut self, value: f64) -> Self {
        self.audio_duration_secs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MultichannelSpeechToTextResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`transcripts`](MultichannelSpeechToTextResponseModelBuilder::transcripts)
    pub fn build(self) -> Result<MultichannelSpeechToTextResponseModel, BuildError> {
        Ok(MultichannelSpeechToTextResponseModel {
            transcripts: self.transcripts.ok_or_else(|| BuildError::missing_field("transcripts"))?,
            transcription_id: self.transcription_id,
            audio_duration_secs: self.audio_duration_secs,
        })
    }
}

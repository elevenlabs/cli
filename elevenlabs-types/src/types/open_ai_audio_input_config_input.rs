pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OpenAiAudioInputConfigInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<OpenAiAudioInputFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcription: Option<OpenAiAudioInputTranscription>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn_detection: Option<OpenAiTurnDetection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pacing: Option<AudioInputPacing>,
}

impl OpenAiAudioInputConfigInput {
    pub fn builder() -> OpenAiAudioInputConfigInputBuilder {
        <OpenAiAudioInputConfigInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OpenAiAudioInputConfigInputBuilder {
    format: Option<OpenAiAudioInputFormat>,
    transcription: Option<OpenAiAudioInputTranscription>,
    turn_detection: Option<OpenAiTurnDetection>,
    pacing: Option<AudioInputPacing>,
}

impl OpenAiAudioInputConfigInputBuilder {
    pub fn format(mut self, value: OpenAiAudioInputFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn transcription(mut self, value: OpenAiAudioInputTranscription) -> Self {
        self.transcription = Some(value);
        self
    }

    pub fn turn_detection(mut self, value: OpenAiTurnDetection) -> Self {
        self.turn_detection = Some(value);
        self
    }

    pub fn pacing(mut self, value: AudioInputPacing) -> Self {
        self.pacing = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OpenAiAudioInputConfigInput`].
    pub fn build(self) -> Result<OpenAiAudioInputConfigInput, BuildError> {
        Ok(OpenAiAudioInputConfigInput {
            format: self.format,
            transcription: self.transcription,
            turn_detection: self.turn_detection,
            pacing: self.pacing,
        })
    }
}

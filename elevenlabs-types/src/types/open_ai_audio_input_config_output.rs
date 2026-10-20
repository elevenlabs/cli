pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OpenAiAudioInputConfigOutput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<OpenAiAudioInputFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcription: Option<OpenAiAudioInputTranscription>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn_detection: Option<OpenAiTurnDetection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pacing: Option<AudioInputPacing>,
}

impl OpenAiAudioInputConfigOutput {
    pub fn builder() -> OpenAiAudioInputConfigOutputBuilder {
        <OpenAiAudioInputConfigOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OpenAiAudioInputConfigOutputBuilder {
    format: Option<OpenAiAudioInputFormat>,
    transcription: Option<OpenAiAudioInputTranscription>,
    turn_detection: Option<OpenAiTurnDetection>,
    pacing: Option<AudioInputPacing>,
}

impl OpenAiAudioInputConfigOutputBuilder {
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

    /// Consumes the builder and constructs a [`OpenAiAudioInputConfigOutput`].
    pub fn build(self) -> Result<OpenAiAudioInputConfigOutput, BuildError> {
        Ok(OpenAiAudioInputConfigOutput {
            format: self.format,
            transcription: self.transcription,
            turn_detection: self.turn_detection,
            pacing: self.pacing,
        })
    }
}

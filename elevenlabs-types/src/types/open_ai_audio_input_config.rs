pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OpenAiAudioInputConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<OpenAiAudioInputFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn_detection: Option<OpenAiTurnDetection>,
}

impl OpenAiAudioInputConfig {
    pub fn builder() -> OpenAiAudioInputConfigBuilder {
        <OpenAiAudioInputConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OpenAiAudioInputConfigBuilder {
    format: Option<OpenAiAudioInputFormat>,
    turn_detection: Option<OpenAiTurnDetection>,
}

impl OpenAiAudioInputConfigBuilder {
    pub fn format(mut self, value: OpenAiAudioInputFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn turn_detection(mut self, value: OpenAiTurnDetection) -> Self {
        self.turn_detection = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OpenAiAudioInputConfig`].
    pub fn build(self) -> Result<OpenAiAudioInputConfig, BuildError> {
        Ok(OpenAiAudioInputConfig {
            format: self.format,
            turn_detection: self.turn_detection,
        })
    }
}

pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AsrConversationalConfigWorkflowOverride {
    /// The quality of the transcription
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<AsrQuality>,
    /// The provider of the transcription service
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<AsrProvider>,
    /// The format of the audio to be transcribed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_input_audio_format: Option<AsrInputFormat>,
    /// Keywords to boost prediction probability for
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
}

impl AsrConversationalConfigWorkflowOverride {
    pub fn builder() -> AsrConversationalConfigWorkflowOverrideBuilder {
        <AsrConversationalConfigWorkflowOverrideBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AsrConversationalConfigWorkflowOverrideBuilder {
    quality: Option<AsrQuality>,
    provider: Option<AsrProvider>,
    user_input_audio_format: Option<AsrInputFormat>,
    keywords: Option<Vec<String>>,
}

impl AsrConversationalConfigWorkflowOverrideBuilder {
    pub fn quality(mut self, value: AsrQuality) -> Self {
        self.quality = Some(value);
        self
    }

    pub fn provider(mut self, value: AsrProvider) -> Self {
        self.provider = Some(value);
        self
    }

    pub fn user_input_audio_format(mut self, value: AsrInputFormat) -> Self {
        self.user_input_audio_format = Some(value);
        self
    }

    pub fn keywords(mut self, value: Vec<String>) -> Self {
        self.keywords = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AsrConversationalConfigWorkflowOverride`].
    pub fn build(self) -> Result<AsrConversationalConfigWorkflowOverride, BuildError> {
        Ok(AsrConversationalConfigWorkflowOverride {
            quality: self.quality,
            provider: self.provider,
            user_input_audio_format: self.user_input_audio_format,
            keywords: self.keywords,
        })
    }
}

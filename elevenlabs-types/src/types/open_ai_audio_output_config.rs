pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OpenAiAudioOutputConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<OpenAiAudioOutputFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<String>,
}

impl OpenAiAudioOutputConfig {
    pub fn builder() -> OpenAiAudioOutputConfigBuilder {
        <OpenAiAudioOutputConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OpenAiAudioOutputConfigBuilder {
    format: Option<OpenAiAudioOutputFormat>,
    voice: Option<String>,
}

impl OpenAiAudioOutputConfigBuilder {
    pub fn format(mut self, value: OpenAiAudioOutputFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn voice(mut self, value: impl Into<String>) -> Self {
        self.voice = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OpenAiAudioOutputConfig`].
    pub fn build(self) -> Result<OpenAiAudioOutputConfig, BuildError> {
        Ok(OpenAiAudioOutputConfig {
            format: self.format,
            voice: self.voice,
        })
    }
}

pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OpenAiAudioConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<OpenAiAudioInputConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<OpenAiAudioOutputConfig>,
}

impl OpenAiAudioConfig {
    pub fn builder() -> OpenAiAudioConfigBuilder {
        <OpenAiAudioConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OpenAiAudioConfigBuilder {
    input: Option<OpenAiAudioInputConfig>,
    output: Option<OpenAiAudioOutputConfig>,
}

impl OpenAiAudioConfigBuilder {
    pub fn input(mut self, value: OpenAiAudioInputConfig) -> Self {
        self.input = Some(value);
        self
    }

    pub fn output(mut self, value: OpenAiAudioOutputConfig) -> Self {
        self.output = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OpenAiAudioConfig`].
    pub fn build(self) -> Result<OpenAiAudioConfig, BuildError> {
        Ok(OpenAiAudioConfig {
            input: self.input,
            output: self.output,
        })
    }
}

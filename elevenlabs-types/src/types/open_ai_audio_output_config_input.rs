pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OpenAiAudioOutputConfigInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<OpenAiAudioOutputFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<String>,
}

impl OpenAiAudioOutputConfigInput {
    pub fn builder() -> OpenAiAudioOutputConfigInputBuilder {
        <OpenAiAudioOutputConfigInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OpenAiAudioOutputConfigInputBuilder {
    format: Option<OpenAiAudioOutputFormat>,
    voice: Option<String>,
}

impl OpenAiAudioOutputConfigInputBuilder {
    pub fn format(mut self, value: OpenAiAudioOutputFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn voice(mut self, value: impl Into<String>) -> Self {
        self.voice = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OpenAiAudioOutputConfigInput`].
    pub fn build(self) -> Result<OpenAiAudioOutputConfigInput, BuildError> {
        Ok(OpenAiAudioOutputConfigInput {
            format: self.format,
            voice: self.voice,
        })
    }
}

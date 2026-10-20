pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OpenAiAudioOutputConfigOutput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<OpenAiAudioOutputFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<String>,
}

impl OpenAiAudioOutputConfigOutput {
    pub fn builder() -> OpenAiAudioOutputConfigOutputBuilder {
        <OpenAiAudioOutputConfigOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OpenAiAudioOutputConfigOutputBuilder {
    format: Option<OpenAiAudioOutputFormat>,
    voice: Option<String>,
}

impl OpenAiAudioOutputConfigOutputBuilder {
    pub fn format(mut self, value: OpenAiAudioOutputFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn voice(mut self, value: impl Into<String>) -> Self {
        self.voice = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OpenAiAudioOutputConfigOutput`].
    pub fn build(self) -> Result<OpenAiAudioOutputConfigOutput, BuildError> {
        Ok(OpenAiAudioOutputConfigOutput {
            format: self.format,
            voice: self.voice,
        })
    }
}

pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OpenAiAudioConfigInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<OpenAiAudioInputConfigInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<OpenAiAudioOutputConfigInput>,
}

impl OpenAiAudioConfigInput {
    pub fn builder() -> OpenAiAudioConfigInputBuilder {
        <OpenAiAudioConfigInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OpenAiAudioConfigInputBuilder {
    input: Option<OpenAiAudioInputConfigInput>,
    output: Option<OpenAiAudioOutputConfigInput>,
}

impl OpenAiAudioConfigInputBuilder {
    pub fn input(mut self, value: OpenAiAudioInputConfigInput) -> Self {
        self.input = Some(value);
        self
    }

    pub fn output(mut self, value: OpenAiAudioOutputConfigInput) -> Self {
        self.output = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OpenAiAudioConfigInput`].
    pub fn build(self) -> Result<OpenAiAudioConfigInput, BuildError> {
        Ok(OpenAiAudioConfigInput {
            input: self.input,
            output: self.output,
        })
    }
}

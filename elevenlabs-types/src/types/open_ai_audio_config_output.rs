pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OpenAiAudioConfigOutput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<OpenAiAudioInputConfigOutput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<OpenAiAudioOutputConfigOutput>,
}

impl OpenAiAudioConfigOutput {
    pub fn builder() -> OpenAiAudioConfigOutputBuilder {
        <OpenAiAudioConfigOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OpenAiAudioConfigOutputBuilder {
    input: Option<OpenAiAudioInputConfigOutput>,
    output: Option<OpenAiAudioOutputConfigOutput>,
}

impl OpenAiAudioConfigOutputBuilder {
    pub fn input(mut self, value: OpenAiAudioInputConfigOutput) -> Self {
        self.input = Some(value);
        self
    }

    pub fn output(mut self, value: OpenAiAudioOutputConfigOutput) -> Self {
        self.output = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OpenAiAudioConfigOutput`].
    pub fn build(self) -> Result<OpenAiAudioConfigOutput, BuildError> {
        Ok(OpenAiAudioConfigOutput {
            input: self.input,
            output: self.output,
        })
    }
}

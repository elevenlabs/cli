pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OpenAiAudioInputFormat {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<OpenAiAudioFormat>,
    /// Sample rate in Hz of `audio/pcm` input. Only 24000 is supported; G.711 formats are always 8000 and ignore this field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<i64>,
}

impl OpenAiAudioInputFormat {
    pub fn builder() -> OpenAiAudioInputFormatBuilder {
        <OpenAiAudioInputFormatBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OpenAiAudioInputFormatBuilder {
    r#type: Option<OpenAiAudioFormat>,
    rate: Option<i64>,
}

impl OpenAiAudioInputFormatBuilder {
    pub fn r#type(mut self, value: OpenAiAudioFormat) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn rate(mut self, value: i64) -> Self {
        self.rate = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OpenAiAudioInputFormat`].
    pub fn build(self) -> Result<OpenAiAudioInputFormat, BuildError> {
        Ok(OpenAiAudioInputFormat {
            r#type: self.r#type,
            rate: self.rate,
        })
    }
}

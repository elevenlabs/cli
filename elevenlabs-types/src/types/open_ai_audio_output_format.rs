pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OpenAiAudioOutputFormat {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<OpenAiAudioFormat>,
}

impl OpenAiAudioOutputFormat {
    pub fn builder() -> OpenAiAudioOutputFormatBuilder {
        <OpenAiAudioOutputFormatBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OpenAiAudioOutputFormatBuilder {
    r#type: Option<OpenAiAudioFormat>,
}

impl OpenAiAudioOutputFormatBuilder {
    pub fn r#type(mut self, value: OpenAiAudioFormat) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OpenAiAudioOutputFormat`].
    pub fn build(self) -> Result<OpenAiAudioOutputFormat, BuildError> {
        Ok(OpenAiAudioOutputFormat {
            r#type: self.r#type,
        })
    }
}

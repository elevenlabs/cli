pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DubbingPartialTranscript {
    pub message_type: String,
    #[serde(default)]
    pub text: String,
}

impl DubbingPartialTranscript {
    pub fn builder() -> DubbingPartialTranscriptBuilder {
        <DubbingPartialTranscriptBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingPartialTranscriptBuilder {
    message_type: Option<String>,
    text: Option<String>,
}

impl DubbingPartialTranscriptBuilder {
    pub fn message_type(mut self, value: impl Into<String>) -> Self {
        self.message_type = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DubbingPartialTranscript`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_type`](DubbingPartialTranscriptBuilder::message_type)
    /// - [`text`](DubbingPartialTranscriptBuilder::text)
    pub fn build(self) -> Result<DubbingPartialTranscript, BuildError> {
        Ok(DubbingPartialTranscript {
            message_type: self.message_type.ok_or_else(|| BuildError::missing_field("message_type"))?,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}

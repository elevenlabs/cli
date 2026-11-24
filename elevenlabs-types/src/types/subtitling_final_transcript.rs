pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SubtitlingFinalTranscript {
    pub message_type: String,
    #[serde(default)]
    pub text: String,
}

impl SubtitlingFinalTranscript {
    pub fn builder() -> SubtitlingFinalTranscriptBuilder {
        <SubtitlingFinalTranscriptBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubtitlingFinalTranscriptBuilder {
    message_type: Option<String>,
    text: Option<String>,
}

impl SubtitlingFinalTranscriptBuilder {
    pub fn message_type(mut self, value: impl Into<String>) -> Self {
        self.message_type = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubtitlingFinalTranscript`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_type`](SubtitlingFinalTranscriptBuilder::message_type)
    /// - [`text`](SubtitlingFinalTranscriptBuilder::text)
    pub fn build(self) -> Result<SubtitlingFinalTranscript, BuildError> {
        Ok(SubtitlingFinalTranscript {
            message_type: self.message_type.ok_or_else(|| BuildError::missing_field("message_type"))?,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}

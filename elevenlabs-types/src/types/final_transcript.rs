pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FinalTranscript {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<String>,
    #[serde(default)]
    pub text: String,
}

impl FinalTranscript {
    pub fn builder() -> FinalTranscriptBuilder {
        <FinalTranscriptBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FinalTranscriptBuilder {
    message_type: Option<String>,
    text: Option<String>,
}

impl FinalTranscriptBuilder {
    pub fn message_type(mut self, value: impl Into<String>) -> Self {
        self.message_type = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FinalTranscript`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](FinalTranscriptBuilder::text)
    pub fn build(self) -> Result<FinalTranscript, BuildError> {
        Ok(FinalTranscript {
            message_type: self.message_type,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}

pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RedactedTranscript {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<String>,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub redacted_text: String,
}

impl RedactedTranscript {
    pub fn builder() -> RedactedTranscriptBuilder {
        <RedactedTranscriptBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RedactedTranscriptBuilder {
    message_type: Option<String>,
    text: Option<String>,
    redacted_text: Option<String>,
}

impl RedactedTranscriptBuilder {
    pub fn message_type(mut self, value: impl Into<String>) -> Self {
        self.message_type = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn redacted_text(mut self, value: impl Into<String>) -> Self {
        self.redacted_text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RedactedTranscript`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](RedactedTranscriptBuilder::text)
    /// - [`redacted_text`](RedactedTranscriptBuilder::redacted_text)
    pub fn build(self) -> Result<RedactedTranscript, BuildError> {
        Ok(RedactedTranscript {
            message_type: self.message_type,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            redacted_text: self.redacted_text.ok_or_else(|| BuildError::missing_field("redacted_text"))?,
        })
    }
}

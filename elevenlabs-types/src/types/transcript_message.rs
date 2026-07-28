pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A single turn in the conversation history.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TranscriptMessage {
    /// The speaker for this turn.
    pub role: TranscriptMessageRole,
    /// The transcript text for this turn.
    #[serde(default)]
    pub content: String,
}

impl TranscriptMessage {
    pub fn builder() -> TranscriptMessageBuilder {
        <TranscriptMessageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TranscriptMessageBuilder {
    role: Option<TranscriptMessageRole>,
    content: Option<String>,
}

impl TranscriptMessageBuilder {
    pub fn role(mut self, value: TranscriptMessageRole) -> Self {
        self.role = Some(value);
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TranscriptMessage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`role`](TranscriptMessageBuilder::role)
    /// - [`content`](TranscriptMessageBuilder::content)
    pub fn build(self) -> Result<TranscriptMessage, BuildError> {
        Ok(TranscriptMessage {
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
        })
    }
}

pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EditedTranscript {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<String>,
    /// The committed transcript text the edit instruction was applied to.
    #[serde(default)]
    pub text: String,
    /// The edited transcript text. If no edits were made it will be identical to the `text` field.
    #[serde(default)]
    pub edited_text: String,
}

impl EditedTranscript {
    pub fn builder() -> EditedTranscriptBuilder {
        <EditedTranscriptBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EditedTranscriptBuilder {
    message_type: Option<String>,
    text: Option<String>,
    edited_text: Option<String>,
}

impl EditedTranscriptBuilder {
    pub fn message_type(mut self, value: impl Into<String>) -> Self {
        self.message_type = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn edited_text(mut self, value: impl Into<String>) -> Self {
        self.edited_text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EditedTranscript`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](EditedTranscriptBuilder::text)
    /// - [`edited_text`](EditedTranscriptBuilder::edited_text)
    pub fn build(self) -> Result<EditedTranscript, BuildError> {
        Ok(EditedTranscript {
            message_type: self.message_type,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            edited_text: self.edited_text.ok_or_else(|| BuildError::missing_field("edited_text"))?,
        })
    }
}

pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationHistoryTranscriptToolCallClientDetails {
    #[serde(default)]
    pub parameters: String,
}

impl ConversationHistoryTranscriptToolCallClientDetails {
    pub fn builder() -> ConversationHistoryTranscriptToolCallClientDetailsBuilder {
        <ConversationHistoryTranscriptToolCallClientDetailsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationHistoryTranscriptToolCallClientDetailsBuilder {
    parameters: Option<String>,
}

impl ConversationHistoryTranscriptToolCallClientDetailsBuilder {
    pub fn parameters(mut self, value: impl Into<String>) -> Self {
        self.parameters = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConversationHistoryTranscriptToolCallClientDetails`].
    /// This method will fail if any of the following fields are not set:
    /// - [`parameters`](ConversationHistoryTranscriptToolCallClientDetailsBuilder::parameters)
    pub fn build(self) -> Result<ConversationHistoryTranscriptToolCallClientDetails, BuildError> {
        Ok(ConversationHistoryTranscriptToolCallClientDetails {
            parameters: self.parameters.ok_or_else(|| BuildError::missing_field("parameters"))?,
        })
    }
}

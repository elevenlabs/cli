pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConversationHistoryTranscriptToolCallIntegrationFunctionDetails {
    #[serde(default)]
    pub resolved_parameters: HashMap<String, String>,
}

impl ConversationHistoryTranscriptToolCallIntegrationFunctionDetails {
    pub fn builder() -> ConversationHistoryTranscriptToolCallIntegrationFunctionDetailsBuilder {
        <ConversationHistoryTranscriptToolCallIntegrationFunctionDetailsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationHistoryTranscriptToolCallIntegrationFunctionDetailsBuilder {
    resolved_parameters: Option<HashMap<String, String>>,
}

impl ConversationHistoryTranscriptToolCallIntegrationFunctionDetailsBuilder {
    pub fn resolved_parameters(mut self, value: HashMap<String, String>) -> Self {
        self.resolved_parameters = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationHistoryTranscriptToolCallIntegrationFunctionDetails`].
    /// This method will fail if any of the following fields are not set:
    /// - [`resolved_parameters`](ConversationHistoryTranscriptToolCallIntegrationFunctionDetailsBuilder::resolved_parameters)
    pub fn build(self) -> Result<ConversationHistoryTranscriptToolCallIntegrationFunctionDetails, BuildError> {
        Ok(ConversationHistoryTranscriptToolCallIntegrationFunctionDetails {
            resolved_parameters: self.resolved_parameters.ok_or_else(|| BuildError::missing_field("resolved_parameters"))?,
        })
    }
}

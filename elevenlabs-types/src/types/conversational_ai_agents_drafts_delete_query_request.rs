pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for delete
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationalAiAgentsDraftsDeleteQueryRequest {
    /// The ID of the agent branch to use
    #[serde(default)]
    pub branch_id: String,
}

impl ConversationalAiAgentsDraftsDeleteQueryRequest {
    pub fn builder() -> ConversationalAiAgentsDraftsDeleteQueryRequestBuilder {
        <ConversationalAiAgentsDraftsDeleteQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationalAiAgentsDraftsDeleteQueryRequestBuilder {
    branch_id: Option<String>,
}

impl ConversationalAiAgentsDraftsDeleteQueryRequestBuilder {
    pub fn branch_id(mut self, value: impl Into<String>) -> Self {
        self.branch_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConversationalAiAgentsDraftsDeleteQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`branch_id`](ConversationalAiAgentsDraftsDeleteQueryRequestBuilder::branch_id)
    pub fn build(self) -> Result<ConversationalAiAgentsDraftsDeleteQueryRequest, BuildError> {
        Ok(ConversationalAiAgentsDraftsDeleteQueryRequest {
            branch_id: self.branch_id.ok_or_else(|| BuildError::missing_field("branch_id"))?,
        })
    }
}


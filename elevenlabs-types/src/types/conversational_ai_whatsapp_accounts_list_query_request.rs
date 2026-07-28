pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationalAiWhatsappAccountsListQueryRequest {
    /// Filter by assigned agent ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
}

impl ConversationalAiWhatsappAccountsListQueryRequest {
    pub fn builder() -> ConversationalAiWhatsappAccountsListQueryRequestBuilder {
        <ConversationalAiWhatsappAccountsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationalAiWhatsappAccountsListQueryRequestBuilder {
    agent_id: Option<String>,
}

impl ConversationalAiWhatsappAccountsListQueryRequestBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConversationalAiWhatsappAccountsListQueryRequest`].
    pub fn build(self) -> Result<ConversationalAiWhatsappAccountsListQueryRequest, BuildError> {
        Ok(ConversationalAiWhatsappAccountsListQueryRequest {
            agent_id: self.agent_id,
        })
    }
}


pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsConversationsGetQueryRequest {
    /// Response format. Defaults to 'json'. Set to 'opentelemetry' for an OTLP-compatible trace payload using the same structure as the post-call webhook.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<ConversationsGetRequestFormat>,
}

impl AgentsConversationsGetQueryRequest {
    pub fn builder() -> AgentsConversationsGetQueryRequestBuilder {
        <AgentsConversationsGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsConversationsGetQueryRequestBuilder {
    format: Option<ConversationsGetRequestFormat>,
}

impl AgentsConversationsGetQueryRequestBuilder {
    pub fn format(mut self, value: ConversationsGetRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentsConversationsGetQueryRequest`].
    pub fn build(self) -> Result<AgentsConversationsGetQueryRequest, BuildError> {
        Ok(AgentsConversationsGetQueryRequest {
            format: self.format,
        })
    }
}


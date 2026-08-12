pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsWidgetGetQueryRequest {
    /// An expiring token that enables a websocket conversation to start. These can be generated for an agent using the /v1/convai/conversation/get_signed_url endpoint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_signature: Option<String>,
}

impl AgentsWidgetGetQueryRequest {
    pub fn builder() -> AgentsWidgetGetQueryRequestBuilder {
        <AgentsWidgetGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsWidgetGetQueryRequestBuilder {
    conversation_signature: Option<String>,
}

impl AgentsWidgetGetQueryRequestBuilder {
    pub fn conversation_signature(mut self, value: impl Into<String>) -> Self {
        self.conversation_signature = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentsWidgetGetQueryRequest`].
    pub fn build(self) -> Result<AgentsWidgetGetQueryRequest, BuildError> {
        Ok(AgentsWidgetGetQueryRequest {
            conversation_signature: self.conversation_signature,
        })
    }
}


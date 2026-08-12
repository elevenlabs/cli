pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationalAiAgentsWidgetGetQueryRequest {
    /// An expiring token that enables a websocket conversation to start. These can be generated for an agent using the /v1/convai/conversation/get_signed_url endpoint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_signature: Option<String>,
}

impl ConversationalAiAgentsWidgetGetQueryRequest {
    pub fn builder() -> ConversationalAiAgentsWidgetGetQueryRequestBuilder {
        <ConversationalAiAgentsWidgetGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationalAiAgentsWidgetGetQueryRequestBuilder {
    conversation_signature: Option<String>,
}

impl ConversationalAiAgentsWidgetGetQueryRequestBuilder {
    pub fn conversation_signature(mut self, value: impl Into<String>) -> Self {
        self.conversation_signature = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConversationalAiAgentsWidgetGetQueryRequest`].
    pub fn build(self) -> Result<ConversationalAiAgentsWidgetGetQueryRequest, BuildError> {
        Ok(ConversationalAiAgentsWidgetGetQueryRequest {
            conversation_signature: self.conversation_signature,
        })
    }
}


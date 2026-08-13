pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationalAiConversationsGetQueryRequest {
    /// Response format. Defaults to 'json'. Set to 'opentelemetry' for an OTLP-compatible trace payload using the same structure as the post-call webhook.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<ConversationsGetRequestFormat>,
}

impl ConversationalAiConversationsGetQueryRequest {
    pub fn builder() -> ConversationalAiConversationsGetQueryRequestBuilder {
        <ConversationalAiConversationsGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationalAiConversationsGetQueryRequestBuilder {
    format: Option<ConversationsGetRequestFormat>,
}

impl ConversationalAiConversationsGetQueryRequestBuilder {
    pub fn format(mut self, value: ConversationsGetRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationalAiConversationsGetQueryRequest`].
    pub fn build(self) -> Result<ConversationalAiConversationsGetQueryRequest, BuildError> {
        Ok(ConversationalAiConversationsGetQueryRequest {
            format: self.format,
        })
    }
}


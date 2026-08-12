pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationalAiKnowledgeBaseDocumentsGetQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
}

impl ConversationalAiKnowledgeBaseDocumentsGetQueryRequest {
    pub fn builder() -> ConversationalAiKnowledgeBaseDocumentsGetQueryRequestBuilder {
        <ConversationalAiKnowledgeBaseDocumentsGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationalAiKnowledgeBaseDocumentsGetQueryRequestBuilder {
    agent_id: Option<String>,
}

impl ConversationalAiKnowledgeBaseDocumentsGetQueryRequestBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConversationalAiKnowledgeBaseDocumentsGetQueryRequest`].
    pub fn build(self) -> Result<ConversationalAiKnowledgeBaseDocumentsGetQueryRequest, BuildError> {
        Ok(ConversationalAiKnowledgeBaseDocumentsGetQueryRequest {
            agent_id: self.agent_id,
        })
    }
}


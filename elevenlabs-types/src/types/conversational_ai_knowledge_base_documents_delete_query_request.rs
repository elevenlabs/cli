pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for delete
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationalAiKnowledgeBaseDocumentsDeleteQueryRequest {
    /// If set to true, the document or folder will be deleted regardless of whether it is used by any agents and it will be removed from the dependent agents. For non-empty folders, this will also delete all child documents and folders.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
}

impl ConversationalAiKnowledgeBaseDocumentsDeleteQueryRequest {
    pub fn builder() -> ConversationalAiKnowledgeBaseDocumentsDeleteQueryRequestBuilder {
        <ConversationalAiKnowledgeBaseDocumentsDeleteQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationalAiKnowledgeBaseDocumentsDeleteQueryRequestBuilder {
    force: Option<bool>,
}

impl ConversationalAiKnowledgeBaseDocumentsDeleteQueryRequestBuilder {
    pub fn force(mut self, value: bool) -> Self {
        self.force = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationalAiKnowledgeBaseDocumentsDeleteQueryRequest`].
    pub fn build(self) -> Result<ConversationalAiKnowledgeBaseDocumentsDeleteQueryRequest, BuildError> {
        Ok(ConversationalAiKnowledgeBaseDocumentsDeleteQueryRequest {
            force: self.force,
        })
    }
}


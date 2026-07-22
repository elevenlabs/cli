pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationalAiKnowledgeBaseDocumentsSummariesGetQueryRequest {
    /// The ids of knowledge base documents.
    #[serde(default)]
    pub document_ids: Vec<Option<String>>,
}

impl ConversationalAiKnowledgeBaseDocumentsSummariesGetQueryRequest {
    pub fn builder() -> ConversationalAiKnowledgeBaseDocumentsSummariesGetQueryRequestBuilder {
        <ConversationalAiKnowledgeBaseDocumentsSummariesGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationalAiKnowledgeBaseDocumentsSummariesGetQueryRequestBuilder {
    document_ids: Option<Vec<Option<String>>>,
}

impl ConversationalAiKnowledgeBaseDocumentsSummariesGetQueryRequestBuilder {
    pub fn document_ids(mut self, value: Vec<Option<String>>) -> Self {
        self.document_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationalAiKnowledgeBaseDocumentsSummariesGetQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`document_ids`](ConversationalAiKnowledgeBaseDocumentsSummariesGetQueryRequestBuilder::document_ids)
    pub fn build(self) -> Result<ConversationalAiKnowledgeBaseDocumentsSummariesGetQueryRequest, BuildError> {
        Ok(ConversationalAiKnowledgeBaseDocumentsSummariesGetQueryRequest {
            document_ids: self.document_ids.ok_or_else(|| BuildError::missing_field("document_ids"))?,
        })
    }
}


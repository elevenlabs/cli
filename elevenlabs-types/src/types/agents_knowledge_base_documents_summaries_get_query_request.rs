pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsKnowledgeBaseDocumentsSummariesGetQueryRequest {
    /// The ids of knowledge base documents.
    #[serde(default)]
    pub document_ids: Vec<Option<String>>,
}

impl AgentsKnowledgeBaseDocumentsSummariesGetQueryRequest {
    pub fn builder() -> AgentsKnowledgeBaseDocumentsSummariesGetQueryRequestBuilder {
        <AgentsKnowledgeBaseDocumentsSummariesGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsKnowledgeBaseDocumentsSummariesGetQueryRequestBuilder {
    document_ids: Option<Vec<Option<String>>>,
}

impl AgentsKnowledgeBaseDocumentsSummariesGetQueryRequestBuilder {
    pub fn document_ids(mut self, value: Vec<Option<String>>) -> Self {
        self.document_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentsKnowledgeBaseDocumentsSummariesGetQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`document_ids`](AgentsKnowledgeBaseDocumentsSummariesGetQueryRequestBuilder::document_ids)
    pub fn build(self) -> Result<AgentsKnowledgeBaseDocumentsSummariesGetQueryRequest, BuildError> {
        Ok(AgentsKnowledgeBaseDocumentsSummariesGetQueryRequest {
            document_ids: self.document_ids.ok_or_else(|| BuildError::missing_field("document_ids"))?,
        })
    }
}


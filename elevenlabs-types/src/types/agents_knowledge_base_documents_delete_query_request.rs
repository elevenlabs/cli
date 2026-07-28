pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for delete
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsKnowledgeBaseDocumentsDeleteQueryRequest {
    /// If set to true, the document or folder will be deleted regardless of whether it is used by any agents and it will be removed from the dependent agents. For non-empty folders, this will also delete all child documents and folders.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
}

impl AgentsKnowledgeBaseDocumentsDeleteQueryRequest {
    pub fn builder() -> AgentsKnowledgeBaseDocumentsDeleteQueryRequestBuilder {
        <AgentsKnowledgeBaseDocumentsDeleteQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsKnowledgeBaseDocumentsDeleteQueryRequestBuilder {
    force: Option<bool>,
}

impl AgentsKnowledgeBaseDocumentsDeleteQueryRequestBuilder {
    pub fn force(mut self, value: bool) -> Self {
        self.force = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentsKnowledgeBaseDocumentsDeleteQueryRequest`].
    pub fn build(self) -> Result<AgentsKnowledgeBaseDocumentsDeleteQueryRequest, BuildError> {
        Ok(AgentsKnowledgeBaseDocumentsDeleteQueryRequest {
            force: self.force,
        })
    }
}


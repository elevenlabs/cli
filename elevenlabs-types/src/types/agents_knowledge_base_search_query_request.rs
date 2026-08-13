pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for search
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsKnowledgeBaseSearchQueryRequest {
    /// The search query text
    #[serde(default)]
    pub query: String,
    /// How many documents to return at maximum. Can not exceed 100, defaults to 30.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// If present, the endpoint will return only documents of the given types.
    #[serde(default)]
    pub types: Vec<Option<KnowledgeBaseDocumentType>>,
    /// Used for fetching next page. Cursor is returned in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl AgentsKnowledgeBaseSearchQueryRequest {
    pub fn builder() -> AgentsKnowledgeBaseSearchQueryRequestBuilder {
        <AgentsKnowledgeBaseSearchQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsKnowledgeBaseSearchQueryRequestBuilder {
    query: Option<String>,
    page_size: Option<i64>,
    types: Option<Vec<Option<KnowledgeBaseDocumentType>>>,
    cursor: Option<String>,
}

impl AgentsKnowledgeBaseSearchQueryRequestBuilder {
    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn types(mut self, value: Vec<Option<KnowledgeBaseDocumentType>>) -> Self {
        self.types = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentsKnowledgeBaseSearchQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`query`](AgentsKnowledgeBaseSearchQueryRequestBuilder::query)
    /// - [`types`](AgentsKnowledgeBaseSearchQueryRequestBuilder::types)
    pub fn build(self) -> Result<AgentsKnowledgeBaseSearchQueryRequest, BuildError> {
        Ok(AgentsKnowledgeBaseSearchQueryRequest {
            query: self.query.ok_or_else(|| BuildError::missing_field("query"))?,
            page_size: self.page_size,
            types: self.types.ok_or_else(|| BuildError::missing_field("types"))?,
            cursor: self.cursor,
        })
    }
}


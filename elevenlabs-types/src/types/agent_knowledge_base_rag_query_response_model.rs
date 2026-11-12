pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AgentKnowledgeBaseRagQueryResponseModel {
    /// Retrieval strategy that ran.
    pub strategy: SearchStrategy,
    /// Raw query used for retrieval (echoes the request query), or null for the cat and ls strategies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieval_query: Option<String>,
    /// Ranked chunks the agent would retrieve with semantic or keyword search, after distance and length filtering. Empty for the cat and ls strategies.
    #[serde(default)]
    pub chunks: Vec<AgentKnowledgeBaseRagChunkResponseModel>,
    /// Document read by the cat strategy, or null when the reference matched no document or another strategy ran.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<AgentKnowledgeBaseRagDocumentResponseModel>,
    /// Entries listed by the ls strategy, or null when the reference matched nothing or another strategy ran.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listing: Option<AgentKnowledgeBaseRagListingResponseModel>,
}

impl AgentKnowledgeBaseRagQueryResponseModel {
    pub fn builder() -> AgentKnowledgeBaseRagQueryResponseModelBuilder {
        <AgentKnowledgeBaseRagQueryResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentKnowledgeBaseRagQueryResponseModelBuilder {
    strategy: Option<SearchStrategy>,
    retrieval_query: Option<String>,
    chunks: Option<Vec<AgentKnowledgeBaseRagChunkResponseModel>>,
    document: Option<AgentKnowledgeBaseRagDocumentResponseModel>,
    listing: Option<AgentKnowledgeBaseRagListingResponseModel>,
}

impl AgentKnowledgeBaseRagQueryResponseModelBuilder {
    pub fn strategy(mut self, value: SearchStrategy) -> Self {
        self.strategy = Some(value);
        self
    }

    pub fn retrieval_query(mut self, value: impl Into<String>) -> Self {
        self.retrieval_query = Some(value.into());
        self
    }

    pub fn chunks(mut self, value: Vec<AgentKnowledgeBaseRagChunkResponseModel>) -> Self {
        self.chunks = Some(value);
        self
    }

    pub fn document(mut self, value: AgentKnowledgeBaseRagDocumentResponseModel) -> Self {
        self.document = Some(value);
        self
    }

    pub fn listing(mut self, value: AgentKnowledgeBaseRagListingResponseModel) -> Self {
        self.listing = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentKnowledgeBaseRagQueryResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`strategy`](AgentKnowledgeBaseRagQueryResponseModelBuilder::strategy)
    /// - [`chunks`](AgentKnowledgeBaseRagQueryResponseModelBuilder::chunks)
    pub fn build(self) -> Result<AgentKnowledgeBaseRagQueryResponseModel, BuildError> {
        Ok(AgentKnowledgeBaseRagQueryResponseModel {
            strategy: self.strategy.ok_or_else(|| BuildError::missing_field("strategy"))?,
            retrieval_query: self.retrieval_query,
            chunks: self.chunks.ok_or_else(|| BuildError::missing_field("chunks"))?,
            document: self.document,
            listing: self.listing,
        })
    }
}

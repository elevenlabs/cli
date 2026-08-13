pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AgentKnowledgeBaseRagQueryResponseModel {
    /// Raw query used for retrieval (echoes the request query).
    #[serde(default)]
    pub retrieval_query: String,
    /// Ranked chunks the agent would retrieve, after distance and length filtering.
    #[serde(default)]
    pub chunks: Vec<AgentKnowledgeBaseRagChunkResponseModel>,
}

impl AgentKnowledgeBaseRagQueryResponseModel {
    pub fn builder() -> AgentKnowledgeBaseRagQueryResponseModelBuilder {
        <AgentKnowledgeBaseRagQueryResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentKnowledgeBaseRagQueryResponseModelBuilder {
    retrieval_query: Option<String>,
    chunks: Option<Vec<AgentKnowledgeBaseRagChunkResponseModel>>,
}

impl AgentKnowledgeBaseRagQueryResponseModelBuilder {
    pub fn retrieval_query(mut self, value: impl Into<String>) -> Self {
        self.retrieval_query = Some(value.into());
        self
    }

    pub fn chunks(mut self, value: Vec<AgentKnowledgeBaseRagChunkResponseModel>) -> Self {
        self.chunks = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentKnowledgeBaseRagQueryResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`retrieval_query`](AgentKnowledgeBaseRagQueryResponseModelBuilder::retrieval_query)
    /// - [`chunks`](AgentKnowledgeBaseRagQueryResponseModelBuilder::chunks)
    pub fn build(self) -> Result<AgentKnowledgeBaseRagQueryResponseModel, BuildError> {
        Ok(AgentKnowledgeBaseRagQueryResponseModel {
            retrieval_query: self.retrieval_query.ok_or_else(|| BuildError::missing_field("retrieval_query"))?,
            chunks: self.chunks.ok_or_else(|| BuildError::missing_field("chunks"))?,
        })
    }
}

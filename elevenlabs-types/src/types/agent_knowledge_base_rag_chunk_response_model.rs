pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AgentKnowledgeBaseRagChunkResponseModel {
    /// ID of the source knowledge base document.
    #[serde(default)]
    pub document_id: String,
    /// Name of the source knowledge base document.
    #[serde(default)]
    pub document_name: String,
    /// ID of the retrieved chunk.
    #[serde(default)]
    pub chunk_id: String,
    /// Text content of the retrieved chunk.
    #[serde(default)]
    pub text: String,
    /// Similarity distance when exposed by the retrieval strategy.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub vector_distance: Option<f64>,
    /// Format of the chunk text. Markdown chunks contain raw markdown; HTML chunks contain HTML.
    pub content_format: ContentFormat,
    /// Type of the source knowledge base document.
    pub document_type: KnowledgeBaseDocumentType,
}

impl AgentKnowledgeBaseRagChunkResponseModel {
    pub fn builder() -> AgentKnowledgeBaseRagChunkResponseModelBuilder {
        <AgentKnowledgeBaseRagChunkResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentKnowledgeBaseRagChunkResponseModelBuilder {
    document_id: Option<String>,
    document_name: Option<String>,
    chunk_id: Option<String>,
    text: Option<String>,
    vector_distance: Option<f64>,
    content_format: Option<ContentFormat>,
    document_type: Option<KnowledgeBaseDocumentType>,
}

impl AgentKnowledgeBaseRagChunkResponseModelBuilder {
    pub fn document_id(mut self, value: impl Into<String>) -> Self {
        self.document_id = Some(value.into());
        self
    }

    pub fn document_name(mut self, value: impl Into<String>) -> Self {
        self.document_name = Some(value.into());
        self
    }

    pub fn chunk_id(mut self, value: impl Into<String>) -> Self {
        self.chunk_id = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn vector_distance(mut self, value: f64) -> Self {
        self.vector_distance = Some(value);
        self
    }

    pub fn content_format(mut self, value: ContentFormat) -> Self {
        self.content_format = Some(value);
        self
    }

    pub fn document_type(mut self, value: KnowledgeBaseDocumentType) -> Self {
        self.document_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentKnowledgeBaseRagChunkResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`document_id`](AgentKnowledgeBaseRagChunkResponseModelBuilder::document_id)
    /// - [`document_name`](AgentKnowledgeBaseRagChunkResponseModelBuilder::document_name)
    /// - [`chunk_id`](AgentKnowledgeBaseRagChunkResponseModelBuilder::chunk_id)
    /// - [`text`](AgentKnowledgeBaseRagChunkResponseModelBuilder::text)
    /// - [`content_format`](AgentKnowledgeBaseRagChunkResponseModelBuilder::content_format)
    /// - [`document_type`](AgentKnowledgeBaseRagChunkResponseModelBuilder::document_type)
    pub fn build(self) -> Result<AgentKnowledgeBaseRagChunkResponseModel, BuildError> {
        Ok(AgentKnowledgeBaseRagChunkResponseModel {
            document_id: self.document_id.ok_or_else(|| BuildError::missing_field("document_id"))?,
            document_name: self.document_name.ok_or_else(|| BuildError::missing_field("document_name"))?,
            chunk_id: self.chunk_id.ok_or_else(|| BuildError::missing_field("chunk_id"))?,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            vector_distance: self.vector_distance,
            content_format: self.content_format.ok_or_else(|| BuildError::missing_field("content_format"))?,
            document_type: self.document_type.ok_or_else(|| BuildError::missing_field("document_type"))?,
        })
    }
}

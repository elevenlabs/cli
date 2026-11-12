pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AgentKnowledgeBaseRagDocumentResponseModel {
    /// ID of the knowledge base document.
    #[serde(default)]
    pub document_id: String,
    /// Name of the knowledge base document.
    #[serde(default)]
    pub document_name: String,
    /// Tracked source URL for URL documents, or null for other document types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
    /// Content the agent would read, cut to the maximum documents length. Empty when the document has no extractable content.
    #[serde(default)]
    pub content: String,
    /// Whether the content was cut to the maximum documents length.
    #[serde(default)]
    pub truncated: bool,
    /// Format of the document content.
    pub content_format: ContentFormat,
    /// Type of the knowledge base document.
    pub document_type: KnowledgeBaseDocumentType,
}

impl AgentKnowledgeBaseRagDocumentResponseModel {
    pub fn builder() -> AgentKnowledgeBaseRagDocumentResponseModelBuilder {
        <AgentKnowledgeBaseRagDocumentResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentKnowledgeBaseRagDocumentResponseModelBuilder {
    document_id: Option<String>,
    document_name: Option<String>,
    source_url: Option<String>,
    content: Option<String>,
    truncated: Option<bool>,
    content_format: Option<ContentFormat>,
    document_type: Option<KnowledgeBaseDocumentType>,
}

impl AgentKnowledgeBaseRagDocumentResponseModelBuilder {
    pub fn document_id(mut self, value: impl Into<String>) -> Self {
        self.document_id = Some(value.into());
        self
    }

    pub fn document_name(mut self, value: impl Into<String>) -> Self {
        self.document_name = Some(value.into());
        self
    }

    pub fn source_url(mut self, value: impl Into<String>) -> Self {
        self.source_url = Some(value.into());
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn truncated(mut self, value: bool) -> Self {
        self.truncated = Some(value);
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

    /// Consumes the builder and constructs a [`AgentKnowledgeBaseRagDocumentResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`document_id`](AgentKnowledgeBaseRagDocumentResponseModelBuilder::document_id)
    /// - [`document_name`](AgentKnowledgeBaseRagDocumentResponseModelBuilder::document_name)
    /// - [`content`](AgentKnowledgeBaseRagDocumentResponseModelBuilder::content)
    /// - [`truncated`](AgentKnowledgeBaseRagDocumentResponseModelBuilder::truncated)
    /// - [`content_format`](AgentKnowledgeBaseRagDocumentResponseModelBuilder::content_format)
    /// - [`document_type`](AgentKnowledgeBaseRagDocumentResponseModelBuilder::document_type)
    pub fn build(self) -> Result<AgentKnowledgeBaseRagDocumentResponseModel, BuildError> {
        Ok(AgentKnowledgeBaseRagDocumentResponseModel {
            document_id: self.document_id.ok_or_else(|| BuildError::missing_field("document_id"))?,
            document_name: self.document_name.ok_or_else(|| BuildError::missing_field("document_name"))?,
            source_url: self.source_url,
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
            truncated: self.truncated.ok_or_else(|| BuildError::missing_field("truncated"))?,
            content_format: self.content_format.ok_or_else(|| BuildError::missing_field("content_format"))?,
            document_type: self.document_type.ok_or_else(|| BuildError::missing_field("document_type"))?,
        })
    }
}

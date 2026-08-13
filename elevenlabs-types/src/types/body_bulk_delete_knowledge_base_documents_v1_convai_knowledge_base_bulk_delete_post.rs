pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BodyBulkDeleteKnowledgeBaseDocumentsV1ConvaiKnowledgeBaseBulkDeletePost {
    /// The ids of documents or folders from the knowledge base.
    #[serde(default)]
    pub document_ids: Vec<String>,
    /// If set to true, documents or folders will be deleted regardless of whether they are used by any agents and will be removed from the dependent agents. For non-empty folders, this will also delete all child documents and folders.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
}

impl BodyBulkDeleteKnowledgeBaseDocumentsV1ConvaiKnowledgeBaseBulkDeletePost {
    pub fn builder() -> BodyBulkDeleteKnowledgeBaseDocumentsV1ConvaiKnowledgeBaseBulkDeletePostBuilder {
        <BodyBulkDeleteKnowledgeBaseDocumentsV1ConvaiKnowledgeBaseBulkDeletePostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyBulkDeleteKnowledgeBaseDocumentsV1ConvaiKnowledgeBaseBulkDeletePostBuilder {
    document_ids: Option<Vec<String>>,
    force: Option<bool>,
}

impl BodyBulkDeleteKnowledgeBaseDocumentsV1ConvaiKnowledgeBaseBulkDeletePostBuilder {
    pub fn document_ids(mut self, value: Vec<String>) -> Self {
        self.document_ids = Some(value);
        self
    }

    pub fn force(mut self, value: bool) -> Self {
        self.force = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BodyBulkDeleteKnowledgeBaseDocumentsV1ConvaiKnowledgeBaseBulkDeletePost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`document_ids`](BodyBulkDeleteKnowledgeBaseDocumentsV1ConvaiKnowledgeBaseBulkDeletePostBuilder::document_ids)
    pub fn build(self) -> Result<BodyBulkDeleteKnowledgeBaseDocumentsV1ConvaiKnowledgeBaseBulkDeletePost, BuildError> {
        Ok(BodyBulkDeleteKnowledgeBaseDocumentsV1ConvaiKnowledgeBaseBulkDeletePost {
            document_ids: self.document_ids.ok_or_else(|| BuildError::missing_field("document_ids"))?,
            force: self.force,
        })
    }
}


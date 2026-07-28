pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BodyUpdateDocumentV1ConvaiKnowledgeBaseDocumentationIdPatch {
    /// A custom, human-readable name for the document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Updated content for the document. Only supported for text documents, URL documents with auto-sync disabled, and file documents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

impl BodyUpdateDocumentV1ConvaiKnowledgeBaseDocumentationIdPatch {
    pub fn builder() -> BodyUpdateDocumentV1ConvaiKnowledgeBaseDocumentationIdPatchBuilder {
        <BodyUpdateDocumentV1ConvaiKnowledgeBaseDocumentationIdPatchBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyUpdateDocumentV1ConvaiKnowledgeBaseDocumentationIdPatchBuilder {
    name: Option<String>,
    content: Option<String>,
}

impl BodyUpdateDocumentV1ConvaiKnowledgeBaseDocumentationIdPatchBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BodyUpdateDocumentV1ConvaiKnowledgeBaseDocumentationIdPatch`].
    pub fn build(self) -> Result<BodyUpdateDocumentV1ConvaiKnowledgeBaseDocumentationIdPatch, BuildError> {
        Ok(BodyUpdateDocumentV1ConvaiKnowledgeBaseDocumentationIdPatch {
            name: self.name,
            content: self.content,
        })
    }
}


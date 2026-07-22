pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BodyCreateTextDocumentV1ConvaiKnowledgeBaseTextPost {
    /// Text content to be added to the knowledge base.
    #[serde(default)]
    pub text: String,
    /// A custom, human-readable name for the document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// If set, the created document or folder will be placed inside the given folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
}

impl BodyCreateTextDocumentV1ConvaiKnowledgeBaseTextPost {
    pub fn builder() -> BodyCreateTextDocumentV1ConvaiKnowledgeBaseTextPostBuilder {
        <BodyCreateTextDocumentV1ConvaiKnowledgeBaseTextPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyCreateTextDocumentV1ConvaiKnowledgeBaseTextPostBuilder {
    text: Option<String>,
    name: Option<String>,
    parent_folder_id: Option<String>,
}

impl BodyCreateTextDocumentV1ConvaiKnowledgeBaseTextPostBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn parent_folder_id(mut self, value: impl Into<String>) -> Self {
        self.parent_folder_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BodyCreateTextDocumentV1ConvaiKnowledgeBaseTextPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](BodyCreateTextDocumentV1ConvaiKnowledgeBaseTextPostBuilder::text)
    pub fn build(self) -> Result<BodyCreateTextDocumentV1ConvaiKnowledgeBaseTextPost, BuildError> {
        Ok(BodyCreateTextDocumentV1ConvaiKnowledgeBaseTextPost {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            name: self.name,
            parent_folder_id: self.parent_folder_id,
        })
    }
}


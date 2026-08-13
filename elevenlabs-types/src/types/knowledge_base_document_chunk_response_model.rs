pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct KnowledgeBaseDocumentChunkResponseModel {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub content: String,
}

impl KnowledgeBaseDocumentChunkResponseModel {
    pub fn builder() -> KnowledgeBaseDocumentChunkResponseModelBuilder {
        <KnowledgeBaseDocumentChunkResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct KnowledgeBaseDocumentChunkResponseModelBuilder {
    id: Option<String>,
    name: Option<String>,
    content: Option<String>,
}

impl KnowledgeBaseDocumentChunkResponseModelBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`KnowledgeBaseDocumentChunkResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](KnowledgeBaseDocumentChunkResponseModelBuilder::id)
    /// - [`name`](KnowledgeBaseDocumentChunkResponseModelBuilder::name)
    /// - [`content`](KnowledgeBaseDocumentChunkResponseModelBuilder::content)
    pub fn build(self) -> Result<KnowledgeBaseDocumentChunkResponseModel, BuildError> {
        Ok(KnowledgeBaseDocumentChunkResponseModel {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
        })
    }
}

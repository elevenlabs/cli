pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct KnowledgeBaseDocumentMetadataResponseModel {
    #[serde(default)]
    pub created_at_unix_secs: i64,
    #[serde(default)]
    pub last_updated_at_unix_secs: i64,
    #[serde(default)]
    pub size_bytes: i64,
}

impl KnowledgeBaseDocumentMetadataResponseModel {
    pub fn builder() -> KnowledgeBaseDocumentMetadataResponseModelBuilder {
        <KnowledgeBaseDocumentMetadataResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct KnowledgeBaseDocumentMetadataResponseModelBuilder {
    created_at_unix_secs: Option<i64>,
    last_updated_at_unix_secs: Option<i64>,
    size_bytes: Option<i64>,
}

impl KnowledgeBaseDocumentMetadataResponseModelBuilder {
    pub fn created_at_unix_secs(mut self, value: i64) -> Self {
        self.created_at_unix_secs = Some(value);
        self
    }

    pub fn last_updated_at_unix_secs(mut self, value: i64) -> Self {
        self.last_updated_at_unix_secs = Some(value);
        self
    }

    pub fn size_bytes(mut self, value: i64) -> Self {
        self.size_bytes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`KnowledgeBaseDocumentMetadataResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at_unix_secs`](KnowledgeBaseDocumentMetadataResponseModelBuilder::created_at_unix_secs)
    /// - [`last_updated_at_unix_secs`](KnowledgeBaseDocumentMetadataResponseModelBuilder::last_updated_at_unix_secs)
    /// - [`size_bytes`](KnowledgeBaseDocumentMetadataResponseModelBuilder::size_bytes)
    pub fn build(self) -> Result<KnowledgeBaseDocumentMetadataResponseModel, BuildError> {
        Ok(KnowledgeBaseDocumentMetadataResponseModel {
            created_at_unix_secs: self.created_at_unix_secs.ok_or_else(|| BuildError::missing_field("created_at_unix_secs"))?,
            last_updated_at_unix_secs: self.last_updated_at_unix_secs.ok_or_else(|| BuildError::missing_field("last_updated_at_unix_secs"))?,
            size_bytes: self.size_bytes.ok_or_else(|| BuildError::missing_field("size_bytes"))?,
        })
    }
}

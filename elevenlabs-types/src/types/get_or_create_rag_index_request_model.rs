pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GetOrCreateRagIndexRequestModel {
    /// ID of the knowledgebase document for which to retrieve the index
    #[serde(default)]
    pub document_id: String,
    /// Whether to create the RAG index if it does not exist
    #[serde(default)]
    pub create_if_missing: bool,
    /// Embedding model to use for the RAG index
    pub model: EmbeddingModelEnum,
}

impl GetOrCreateRagIndexRequestModel {
    pub fn builder() -> GetOrCreateRagIndexRequestModelBuilder {
        <GetOrCreateRagIndexRequestModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetOrCreateRagIndexRequestModelBuilder {
    document_id: Option<String>,
    create_if_missing: Option<bool>,
    model: Option<EmbeddingModelEnum>,
}

impl GetOrCreateRagIndexRequestModelBuilder {
    pub fn document_id(mut self, value: impl Into<String>) -> Self {
        self.document_id = Some(value.into());
        self
    }

    pub fn create_if_missing(mut self, value: bool) -> Self {
        self.create_if_missing = Some(value);
        self
    }

    pub fn model(mut self, value: EmbeddingModelEnum) -> Self {
        self.model = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetOrCreateRagIndexRequestModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`document_id`](GetOrCreateRagIndexRequestModelBuilder::document_id)
    /// - [`create_if_missing`](GetOrCreateRagIndexRequestModelBuilder::create_if_missing)
    /// - [`model`](GetOrCreateRagIndexRequestModelBuilder::model)
    pub fn build(self) -> Result<GetOrCreateRagIndexRequestModel, BuildError> {
        Ok(GetOrCreateRagIndexRequestModel {
            document_id: self.document_id.ok_or_else(|| BuildError::missing_field("document_id"))?,
            create_if_missing: self.create_if_missing.ok_or_else(|| BuildError::missing_field("create_if_missing"))?,
            model: self.model.ok_or_else(|| BuildError::missing_field("model"))?,
        })
    }
}

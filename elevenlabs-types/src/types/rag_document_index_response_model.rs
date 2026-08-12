pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RagDocumentIndexResponseModel {
    #[serde(default)]
    pub id: String,
    pub model: EmbeddingModelEnum,
    pub status: RagIndexStatus,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub progress_percentage: f64,
    #[serde(default)]
    pub document_model_index_usage: RagDocumentIndexUsage,
}

impl RagDocumentIndexResponseModel {
    pub fn builder() -> RagDocumentIndexResponseModelBuilder {
        <RagDocumentIndexResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RagDocumentIndexResponseModelBuilder {
    id: Option<String>,
    model: Option<EmbeddingModelEnum>,
    status: Option<RagIndexStatus>,
    progress_percentage: Option<f64>,
    document_model_index_usage: Option<RagDocumentIndexUsage>,
}

impl RagDocumentIndexResponseModelBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn model(mut self, value: EmbeddingModelEnum) -> Self {
        self.model = Some(value);
        self
    }

    pub fn status(mut self, value: RagIndexStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn progress_percentage(mut self, value: f64) -> Self {
        self.progress_percentage = Some(value);
        self
    }

    pub fn document_model_index_usage(mut self, value: RagDocumentIndexUsage) -> Self {
        self.document_model_index_usage = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RagDocumentIndexResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](RagDocumentIndexResponseModelBuilder::id)
    /// - [`model`](RagDocumentIndexResponseModelBuilder::model)
    /// - [`status`](RagDocumentIndexResponseModelBuilder::status)
    /// - [`progress_percentage`](RagDocumentIndexResponseModelBuilder::progress_percentage)
    /// - [`document_model_index_usage`](RagDocumentIndexResponseModelBuilder::document_model_index_usage)
    pub fn build(self) -> Result<RagDocumentIndexResponseModel, BuildError> {
        Ok(RagDocumentIndexResponseModel {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            model: self.model.ok_or_else(|| BuildError::missing_field("model"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            progress_percentage: self.progress_percentage.ok_or_else(|| BuildError::missing_field("progress_percentage"))?,
            document_model_index_usage: self.document_model_index_usage.ok_or_else(|| BuildError::missing_field("document_model_index_usage"))?,
        })
    }
}

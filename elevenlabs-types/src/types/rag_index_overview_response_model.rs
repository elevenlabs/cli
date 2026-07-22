pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RagIndexOverviewResponseModel {
    #[serde(default)]
    pub total_used_bytes: i64,
    #[serde(default)]
    pub total_max_bytes: i64,
    #[serde(default)]
    pub models: Vec<RagIndexOverviewEmbeddingModelResponseModel>,
}

impl RagIndexOverviewResponseModel {
    pub fn builder() -> RagIndexOverviewResponseModelBuilder {
        <RagIndexOverviewResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RagIndexOverviewResponseModelBuilder {
    total_used_bytes: Option<i64>,
    total_max_bytes: Option<i64>,
    models: Option<Vec<RagIndexOverviewEmbeddingModelResponseModel>>,
}

impl RagIndexOverviewResponseModelBuilder {
    pub fn total_used_bytes(mut self, value: i64) -> Self {
        self.total_used_bytes = Some(value);
        self
    }

    pub fn total_max_bytes(mut self, value: i64) -> Self {
        self.total_max_bytes = Some(value);
        self
    }

    pub fn models(mut self, value: Vec<RagIndexOverviewEmbeddingModelResponseModel>) -> Self {
        self.models = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RagIndexOverviewResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`total_used_bytes`](RagIndexOverviewResponseModelBuilder::total_used_bytes)
    /// - [`total_max_bytes`](RagIndexOverviewResponseModelBuilder::total_max_bytes)
    /// - [`models`](RagIndexOverviewResponseModelBuilder::models)
    pub fn build(self) -> Result<RagIndexOverviewResponseModel, BuildError> {
        Ok(RagIndexOverviewResponseModel {
            total_used_bytes: self.total_used_bytes.ok_or_else(|| BuildError::missing_field("total_used_bytes"))?,
            total_max_bytes: self.total_max_bytes.ok_or_else(|| BuildError::missing_field("total_max_bytes"))?,
            models: self.models.ok_or_else(|| BuildError::missing_field("models"))?,
        })
    }
}

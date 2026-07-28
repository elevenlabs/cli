pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RagIndexOverviewEmbeddingModelResponseModel {
    pub model: EmbeddingModelEnum,
    #[serde(default)]
    pub used_bytes: i64,
}

impl RagIndexOverviewEmbeddingModelResponseModel {
    pub fn builder() -> RagIndexOverviewEmbeddingModelResponseModelBuilder {
        <RagIndexOverviewEmbeddingModelResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RagIndexOverviewEmbeddingModelResponseModelBuilder {
    model: Option<EmbeddingModelEnum>,
    used_bytes: Option<i64>,
}

impl RagIndexOverviewEmbeddingModelResponseModelBuilder {
    pub fn model(mut self, value: EmbeddingModelEnum) -> Self {
        self.model = Some(value);
        self
    }

    pub fn used_bytes(mut self, value: i64) -> Self {
        self.used_bytes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RagIndexOverviewEmbeddingModelResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`model`](RagIndexOverviewEmbeddingModelResponseModelBuilder::model)
    /// - [`used_bytes`](RagIndexOverviewEmbeddingModelResponseModelBuilder::used_bytes)
    pub fn build(self) -> Result<RagIndexOverviewEmbeddingModelResponseModel, BuildError> {
        Ok(RagIndexOverviewEmbeddingModelResponseModel {
            model: self.model.ok_or_else(|| BuildError::missing_field("model"))?,
            used_bytes: self.used_bytes.ok_or_else(|| BuildError::missing_field("used_bytes"))?,
        })
    }
}

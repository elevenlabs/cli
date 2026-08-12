pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RagIndexRequestModel {
    pub model: EmbeddingModelEnum,
}

impl RagIndexRequestModel {
    pub fn builder() -> RagIndexRequestModelBuilder {
        <RagIndexRequestModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RagIndexRequestModelBuilder {
    model: Option<EmbeddingModelEnum>,
}

impl RagIndexRequestModelBuilder {
    pub fn model(mut self, value: EmbeddingModelEnum) -> Self {
        self.model = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RagIndexRequestModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`model`](RagIndexRequestModelBuilder::model)
    pub fn build(self) -> Result<RagIndexRequestModel, BuildError> {
        Ok(RagIndexRequestModel {
            model: self.model.ok_or_else(|| BuildError::missing_field("model"))?,
        })
    }
}


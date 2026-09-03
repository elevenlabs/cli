pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct InternalSamplingConfigResponseModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub focus: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub similarity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub stability: Option<f64>,
}

impl InternalSamplingConfigResponseModel {
    pub fn builder() -> InternalSamplingConfigResponseModelBuilder {
        <InternalSamplingConfigResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InternalSamplingConfigResponseModelBuilder {
    focus: Option<f64>,
    similarity: Option<f64>,
    stability: Option<f64>,
}

impl InternalSamplingConfigResponseModelBuilder {
    pub fn focus(mut self, value: f64) -> Self {
        self.focus = Some(value);
        self
    }

    pub fn similarity(mut self, value: f64) -> Self {
        self.similarity = Some(value);
        self
    }

    pub fn stability(mut self, value: f64) -> Self {
        self.stability = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InternalSamplingConfigResponseModel`].
    pub fn build(self) -> Result<InternalSamplingConfigResponseModel, BuildError> {
        Ok(InternalSamplingConfigResponseModel {
            focus: self.focus,
            similarity: self.similarity,
            stability: self.stability,
        })
    }
}

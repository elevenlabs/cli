pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ModelSettingsResponseModel {
    /// Determines how stable the voice is and the randomness between each generation. Lower values introduce broader emotional range for the voice. Higher values can result in a monotonous voice with limited emotion.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub stability: Option<f64>,
}

impl ModelSettingsResponseModel {
    pub fn builder() -> ModelSettingsResponseModelBuilder {
        <ModelSettingsResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ModelSettingsResponseModelBuilder {
    stability: Option<f64>,
}

impl ModelSettingsResponseModelBuilder {
    pub fn stability(mut self, value: f64) -> Self {
        self.stability = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ModelSettingsResponseModel`].
    pub fn build(self) -> Result<ModelSettingsResponseModel, BuildError> {
        Ok(ModelSettingsResponseModel {
            stability: self.stability,
        })
    }
}

pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ToDialogueSettingsResponseModel {
    /// Determines how stable the voice is and the randomness between each generation. Lower values introduce broader emotional range for the voice. Higher values can result in a monotonous voice with limited emotion.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub stability: Option<f64>,
    /// Determines how strongly the model is guided while generating. Higher values make the model adhere more closely to the voice, at the cost of variation. Not supported by every model.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub similarity: Option<f64>,
}

impl ToDialogueSettingsResponseModel {
    pub fn builder() -> ToDialogueSettingsResponseModelBuilder {
        <ToDialogueSettingsResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToDialogueSettingsResponseModelBuilder {
    stability: Option<f64>,
    similarity: Option<f64>,
}

impl ToDialogueSettingsResponseModelBuilder {
    pub fn stability(mut self, value: f64) -> Self {
        self.stability = Some(value);
        self
    }

    pub fn similarity(mut self, value: f64) -> Self {
        self.similarity = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ToDialogueSettingsResponseModel`].
    pub fn build(self) -> Result<ToDialogueSettingsResponseModel, BuildError> {
        Ok(ToDialogueSettingsResponseModel {
            stability: self.stability,
            similarity: self.similarity,
        })
    }
}

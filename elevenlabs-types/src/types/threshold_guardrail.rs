pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ThresholdGuardrail {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub threshold: Option<f64>,
}

impl ThresholdGuardrail {
    pub fn builder() -> ThresholdGuardrailBuilder {
        <ThresholdGuardrailBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ThresholdGuardrailBuilder {
    is_enabled: Option<bool>,
    threshold: Option<f64>,
}

impl ThresholdGuardrailBuilder {
    pub fn is_enabled(mut self, value: bool) -> Self {
        self.is_enabled = Some(value);
        self
    }

    pub fn threshold(mut self, value: f64) -> Self {
        self.threshold = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ThresholdGuardrail`].
    pub fn build(self) -> Result<ThresholdGuardrail, BuildError> {
        Ok(ThresholdGuardrail {
            is_enabled: self.is_enabled,
            threshold: self.threshold,
        })
    }
}

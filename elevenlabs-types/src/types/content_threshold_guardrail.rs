pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ContentThresholdGuardrail {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<ContentThresholdGuardrailThreshold>,
}

impl ContentThresholdGuardrail {
    pub fn builder() -> ContentThresholdGuardrailBuilder {
        <ContentThresholdGuardrailBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ContentThresholdGuardrailBuilder {
    is_enabled: Option<bool>,
    threshold: Option<ContentThresholdGuardrailThreshold>,
}

impl ContentThresholdGuardrailBuilder {
    pub fn is_enabled(mut self, value: bool) -> Self {
        self.is_enabled = Some(value);
        self
    }

    pub fn threshold(mut self, value: ContentThresholdGuardrailThreshold) -> Self {
        self.threshold = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ContentThresholdGuardrail`].
    pub fn build(self) -> Result<ContentThresholdGuardrail, BuildError> {
        Ok(ContentThresholdGuardrail {
            is_enabled: self.is_enabled,
            threshold: self.threshold,
        })
    }
}

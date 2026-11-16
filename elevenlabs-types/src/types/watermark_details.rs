pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WatermarkDetails {
    /// Product that generated the audio, when known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    /// Generating model, when known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Generation time, when known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_at: Option<String>,
    /// Identifier of the matched generation record, when known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
}

impl WatermarkDetails {
    pub fn builder() -> WatermarkDetailsBuilder {
        <WatermarkDetailsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WatermarkDetailsBuilder {
    product: Option<String>,
    model: Option<String>,
    generated_at: Option<String>,
    unique_id: Option<String>,
}

impl WatermarkDetailsBuilder {
    pub fn product(mut self, value: impl Into<String>) -> Self {
        self.product = Some(value.into());
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn generated_at(mut self, value: impl Into<String>) -> Self {
        self.generated_at = Some(value.into());
        self
    }

    pub fn unique_id(mut self, value: impl Into<String>) -> Self {
        self.unique_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WatermarkDetails`].
    pub fn build(self) -> Result<WatermarkDetails, BuildError> {
        Ok(WatermarkDetails {
            product: self.product,
            model: self.model,
            generated_at: self.generated_at,
            unique_id: self.unique_id,
        })
    }
}
